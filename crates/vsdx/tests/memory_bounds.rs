//! Reader limits reject large ShapeSheets before DOM expansion.
use diagramcraft_vsdx::parse_xml;
const NS: &str = "http://schemas.microsoft.com/office/visio/2012/main";
fn huge_geometry() -> String {
    use std::fmt::Write;
    let mut xml = format!("<Shape xmlns=\"{NS}\"><Section N=\"Geometry\">");
    for i in 0..100_000 {
        write!(xml, "<Row IX=\"{i}\" T=\"LineTo\"><Cell N=\"X\" V=\"1\"/><Cell N=\"Y\" V=\"1\"/></Row>").unwrap();
    }
    xml.push_str("</Section></Shape>");
    xml
}
#[test]
fn sections_and_sheets_are_bounded_before_dom_allocation() {
    assert!(parse_xml(huge_geometry().as_bytes()).unwrap_err().to_string().contains("section cell or row limit"));
    let rows = format!("<Shape xmlns=\"{NS}\"><Section>{}</Section></Shape>", "<Row/>".repeat(16_385));
    assert!(parse_xml(rows.as_bytes()).unwrap_err().to_string().contains("section cell or row limit"));
    let bytes = format!("<Shape xmlns=\"{NS}\"><Text>{}</Text></Shape>", "x".repeat(16 * 1024 * 1024));
    assert!(parse_xml(bytes.as_bytes()).unwrap_err().to_string().contains("sheet byte limit"));
    let cells = format!("<Shape xmlns=\"{NS}\">{}</Shape>", "<Cell N=\"X\" V=\"0\"/>".repeat(65_537));
    assert!(parse_xml(cells.as_bytes()).unwrap_err().to_string().contains("sheet cell limit"));
}
#[test]
fn caller_built_sections_receive_the_same_bound() {
    use diagramcraft_model::{XmlElement, XmlNode};
    let cell = XmlElement { name: "Cell".into(), namespace: NS.into(), ..Default::default() };
    let section = XmlElement { name: "Section".into(), namespace: NS.into(), children: vec![XmlNode::Element(cell); 16_385], ..Default::default() };
    let shape = XmlElement { children: vec![XmlNode::Element(section)], ..Default::default() };
    assert!(diagramcraft_vsdx::parse_sheet(&shape).unwrap_err().to_string().contains("section cell or row limit"));
}
#[test]
#[cfg(target_os = "macos")]
fn geometry_section_peak_memory_is_bounded() {
    if std::env::var_os("DIAGRAMCRAFT_GEOMETRY_MEMORY_CHILD").is_some() {
        assert!(parse_xml(huge_geometry().as_bytes()).unwrap_err().to_string().contains("section cell or row limit"));
        return;
    }
    let output = std::process::Command::new("/usr/bin/time")
        .arg("-l")
        .arg(std::env::current_exe().unwrap())
        .args(["--exact", "geometry_section_peak_memory_is_bounded", "--nocapture"])
        .env("DIAGRAMCRAFT_GEOMETRY_MEMORY_CHILD", "1")
        .output()
        .unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(output.status.success(), "{stderr}");
    let rss: u64 = stderr.lines().find(|l| l.contains("maximum resident set size")).unwrap().split_whitespace().next().unwrap().parse().unwrap();
    eprintln!("100000-row geometry peak RSS: {rss} bytes (limit 200 MiB)");
    assert!(rss < 200 * 1024 * 1024, "{stderr}");
}
