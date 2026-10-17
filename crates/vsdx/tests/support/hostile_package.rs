// Review 1a synthetic package recipe, adapted with Charles Hood's permission.
// MIT OR Apache-2.0. Source recorded in ATTRIBUTION.md.
use std::io::Write;
pub const NS: &str = "http://schemas.microsoft.com/office/visio/2012/main";
const R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const P: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const V: &str = "http://schemas.microsoft.com/visio/2010/relationships/";
pub fn package(extra: &str, media_mib: usize) -> Vec<u8> {
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let parts = [
        ("[Content_Types].xml", "<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"><Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/><Default Extension=\"xml\" ContentType=\"application/xml\"/><Override PartName=\"/visio/document.xml\" ContentType=\"application/vnd.ms-visio.drawing.main+xml\"/></Types>".into()),
        ("_rels/.rels", format!("<Relationships xmlns='{P}'><Relationship Id='rId1' Type='{V}document' Target='visio/document.xml'/></Relationships>")),
        ("visio/document.xml", format!("<VisioDocument xmlns='{NS}'/>")),
        ("visio/_rels/document.xml.rels", format!("<Relationships xmlns='{P}'><Relationship Id='rId1' Type='{V}pages' Target='pages/pages.xml'/></Relationships>")),
        ("visio/pages/pages.xml", format!("<Pages xmlns='{NS}' xmlns:r='{R}'><Page ID='0' Name='P'><Rel r:id='rId1'/></Page></Pages>")),
        ("visio/pages/_rels/pages.xml.rels", format!("<Relationships xmlns='{P}'><Relationship Id='rId1' Type='{V}page' Target='page1.xml'/></Relationships>")),
        ("visio/pages/page1.xml", format!("<PageContents xmlns='{NS}'><Shapes/>{extra}</PageContents>")),
    ];
    for (name, bytes) in parts {
        zip.start_file(name, options).unwrap();
        zip.write_all(bytes.as_bytes()).unwrap();
    }
    let zeros = vec![0; 1024 * 1024];
    for i in 0..3 {
        if media_mib == 0 {
            break;
        }
        zip.start_file(format!("visio/media/blob{i}.bin"), options).unwrap();
        for _ in 0..media_mib {
            zip.write_all(&zeros).unwrap();
        }
    }
    zip.finish().unwrap().into_inner()
}
#[cfg(target_os = "macos")]
pub fn peak(test: &str, mode: &str) -> u64 {
    let output = std::process::Command::new("/usr/bin/time")
        .arg("-l")
        .arg(std::env::current_exe().unwrap())
        .args(["--exact", test, "--nocapture"])
        .env("MISTERDIAGRAM_READER_MEMORY_CHILD", mode)
        .output()
        .unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(output.status.success(), "{stderr}");
    stderr.lines().find(|l| l.contains("maximum resident set size")).unwrap().split_whitespace().next().unwrap().parse().unwrap()
}
