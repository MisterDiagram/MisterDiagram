//! CLI hostile-input and supported-package tests.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#[cfg(test)]
mod tests {
    use diagramcraft_opc::{MAX_PART, PackageOut, RelsOut};
    use std::path::PathBuf;
    use std::process::Command;
    fn package(kind: &str) -> Vec<u8> {
        let mut p = PackageOut::default();
        p.xml_part("visio/document.xml", kind, b"<VisioDocument xmlns=\"http://schemas.microsoft.com/office/visio/2012/main\"/>".to_vec());
        let mut r = RelsOut::default();
        r.add("http://schemas.microsoft.com/visio/2010/relationships/document", "visio/document.xml", false);
        p.rels("", &r);
        p.finish().unwrap()
    }
    fn invoke(bytes: &[u8], ext: &str) -> std::process::Output {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/info-tests");
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join(format!("{}-{}.{}", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed), ext));
        std::fs::write(&p, bytes).unwrap();
        let o = Command::new(env!("CARGO_BIN_EXE_diagramcraft-cli")).args(["info"]).arg(&p).output().unwrap();
        std::fs::remove_file(p).unwrap();
        o
    }
    #[test]
    fn supported_package_types_and_root_relationships() {
        for (ext, kind) in [("vsdx", "drawing"), ("vssx", "stencil"), ("vstx", "template"), ("vsdm", "drawing.macroEnabled")] {
            let o = invoke(&package(&format!("application/vnd.ms-visio.{kind}.main+xml")), ext);
            assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
            let s = String::from_utf8(o.stdout).unwrap();
            assert!(s.contains("visio/document.xml\tapplication/vnd.ms-visio."));
            assert!(s.contains("relationship\trId1\tdocument\tvisio/document.xml"));
        }
    }
    #[test]
    fn rejects_unrelated_namespaces_and_malformed_document() {
        for (uri, xml) in [
            ("https://example.invalid/document", "<VisioDocument xmlns='http://schemas.microsoft.com/office/visio/2012/main'/>"),
            ("http://schemas.microsoft.com/visio/2010/relationships/document", "<VisioDocument xmlns='urn:not-visio'/>"),
            (
                "http://schemas.microsoft.com/visio/2010/relationships/document",
                "<VisioDocument xmlns='http://schemas.microsoft.com/office/visio/2012/main' bad=unquoted/>",
            ),
        ] {
            let mut p = PackageOut::default();
            p.xml_part("visio/document.xml", "application/vnd.ms-visio.drawing.main+xml", xml.as_bytes().to_vec());
            let mut r = RelsOut::default();
            r.add(uri, "visio/document.xml", false);
            p.rels("", &r);
            let o = invoke(&p.finish().unwrap(), "vsdx");
            assert!(!o.status.success());
            assert_eq!(String::from_utf8(o.stderr).unwrap().lines().count(), 1);
        }
    }
    #[test]
    fn truncated_arbitrary_and_zip_bomb_fail_cleanly() {
        let valid = package("application/vnd.ms-visio.drawing.main+xml");
        let mut bomb = valid.clone();
        // A ZIP bomb's declared expanded size must be rejected before allocating or inflating it.
        let at = bomb.windows(4).position(|w| w == b"PK\x01\x02").unwrap() + 24;
        bomb[at..at + 4].copy_from_slice(&((MAX_PART + 1) as u32).to_le_bytes());
        for data in [b"not a zip".as_slice(), &valid[..valid.len() / 2], &bomb, &package("application/xml")] {
            let o = invoke(data, "vsdx");
            assert!(!o.status.success());
            let e = String::from_utf8(o.stderr).unwrap();
            assert_eq!(e.lines().count(), 1, "{e}");
            assert!(e.starts_with("error: "));
            assert!(!e.contains("panicked"));
        }
        assert!(!invoke(&valid, "zip").status.success());
    }
}
