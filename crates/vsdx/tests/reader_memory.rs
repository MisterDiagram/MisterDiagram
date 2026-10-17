// Review 1a memory regressions, scaled and tightened with Charles Hood's permission.
// MIT OR Apache-2.0. Source recorded in ATTRIBUTION.md.
#[path = "support/hostile_package.rs"]
mod support;

#[test]
fn retained_dom_keeps_extension_attributes_and_order() {
    let bytes = support::package("<a b='1'/><a b='2'/>text", 0);
    let doc = misterdiagram_vsdx::import(&bytes).unwrap();
    let root = doc.parts["visio/pages/page1.xml"].xml.as_ref().unwrap();
    let extra: Vec<_> = root.elements().filter(|e| e.name == "a").map(|e| e.attr("b").unwrap()).collect();
    assert_eq!(extra, ["1", "2"]);
    assert_eq!(root.text(), "text");
    assert_eq!(root.elements().last().unwrap().namespace, support::NS);
    assert_eq!(doc.parts["visio/pages/page1.xml"].bytes, misterdiagram_opc::Package::open(&bytes).unwrap().get("visio/pages/page1.xml").unwrap());
}

#[test]
#[cfg(target_os = "macos")]
fn non_sheet_elements_in_a_page_do_not_exceed_1_gib() {
    if std::env::var_os("MISTERDIAGRAM_READER_MEMORY_CHILD").is_some() {
        let bytes = support::package(&"<a b='1'/>".repeat(100_000), 0);
        let doc = misterdiagram_vsdx::import(&bytes).unwrap();
        assert_eq!(doc.parts["visio/pages/page1.xml"].xml.as_ref().unwrap().children.len(), 100_001);
        return;
    }
    let rss = support::peak("non_sheet_elements_in_a_page_do_not_exceed_1_gib", "dom");
    eprintln!("100000-element page peak RSS: {rss} bytes (limit 90 MiB)");
    // One fifteenth of the full-size repro, with room for fixed process overhead.
    assert!(rss < 90 * 1024 * 1024, "peak RSS {rss} exceeds scaled DOM bound");
}

#[test]
#[cfg(target_os = "macos")]
fn import_peak_memory_tracks_decompressed_size_once() {
    if let Ok(mode) = std::env::var("MISTERDIAGRAM_READER_MEMORY_CHILD") {
        let bytes = support::package("", 32);
        if mode == "package" {
            let package = misterdiagram_opc::Package::open(&bytes).unwrap();
            assert_eq!(package.get("visio/media/blob2.bin").unwrap().len(), 32 * 1024 * 1024);
        } else {
            let doc = misterdiagram_vsdx::import(&bytes).unwrap();
            assert_eq!(doc.parts["visio/media/blob2.bin"].bytes.len(), 32 * 1024 * 1024);
            assert_eq!(doc.pages.len(), 1);
        }
        return;
    }
    let package = support::peak("import_peak_memory_tracks_decompressed_size_once", "package");
    let import = support::peak("import_peak_memory_tracks_decompressed_size_once", "import");
    eprintln!("96 MiB parts: package RSS {package}, import RSS {import}, ratio {}", import as f64 / package as f64);
    assert!(import * 10 <= package * 12, "import RSS {import} exceeds 1.2 times package RSS {package}");
}
