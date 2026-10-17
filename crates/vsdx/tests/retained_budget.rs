//! Package admission across parts, before retaining XML trees.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
use misterdiagram_opc::{PackageOut, RelsOut};
use misterdiagram_vsdx::{MAX_RETAINED_XML_NODES, import_with_open_nodes, parse_xml};
const NS: &str = "http://schemas.microsoft.com/office/visio/2012/main";
const REL: &str = "http://schemas.microsoft.com/visio/2010/relationships/";

fn package(extra: &str, pages: usize) -> Vec<u8> {
    let mut p = PackageOut::default();
    let mut root = RelsOut::default();
    root.add(&format!("{REL}document"), "visio/document.xml", false);
    p.rels("", &root);
    p.xml_part("visio/document.xml", "application/vnd.ms-visio.drawing.main+xml", format!("<VisioDocument xmlns='{NS}'/>").into_bytes());
    let mut rels = RelsOut::default();
    rels.add(&format!("{REL}pages"), "pages/pages.xml", false);
    p.rels("visio/document.xml", &rels);
    let mut index = format!("<Pages xmlns='{NS}' xmlns:r='http://schemas.openxmlformats.org/officeDocument/2006/relationships'>");
    let mut rels = RelsOut::default();
    for i in 0..pages {
        let rid = rels.add(&format!("{REL}page"), &format!("page{i}.xml"), false);
        index.push_str(&format!("<Page ID='{i}'><Rel r:id='{rid}'/></Page>"));
        p.xml_part(
            &format!("visio/pages/page{i}.xml"),
            "application/vnd.ms-visio.page+xml",
            format!("<PageContents xmlns='{NS}'>{extra}</PageContents>").into_bytes(),
        );
    }
    index.push_str("</Pages>");
    p.xml_part("visio/pages/pages.xml", "application/vnd.ms-visio.pages+xml", index.into_bytes());
    p.rels("visio/pages/pages.xml", &rels);
    p.finish().unwrap()
}
#[test]
fn several_small_parts_share_one_retained_node_budget() {
    let bytes = package(&"<a b='1'/>".repeat(20), 3);
    let package = misterdiagram_opc::Package::open(&bytes).unwrap();
    let mut required = 0;
    for (_, xml) in package.parts() {
        misterdiagram_opc::xml::charge_retained_nodes(xml, &mut required, MAX_RETAINED_XML_NODES, 0).unwrap();
    }
    // Reserve the existing session population so the package has a small remaining
    // budget. Each part independently fits that budget; their combined cost does not.
    let available = required - 1;
    for (name, xml) in package.parts() {
        let mut count = 0;
        misterdiagram_opc::xml::charge_retained_nodes(xml, &mut count, available, 0).unwrap_or_else(|e| panic!("{name}: {e}"));
    }
    let already = MAX_RETAINED_XML_NODES - available;
    let Err(error) = import_with_open_nodes(&bytes, already) else {
        panic!("combined XML parts must share the retained node budget");
    };
    let error = error.to_string();
    assert!(error.contains("retained XML node limit: found 4000001, limit 4000000"), "{error}");
    assert!(error.contains(&format!("already open {already}")), "{error}");
    let accepted = import_with_open_nodes(&bytes, already - 1).unwrap();
    assert_eq!(accepted.pages.len(), 3);
    assert_eq!(accepted.retained_xml_nodes, required);
}
#[test]
fn text_capacity_matches_coalesced_nodes_and_namespaces_restore() {
    let input = format!("<a xmlns:p='old'>{}<p:b xmlns:p='new' p:x='1'/><p:c p:y='2'/>z</a>", "&amp;<![CDATA[x]]><!--c-->".repeat(1000));
    let root = parse_xml(input.as_bytes()).unwrap();
    assert_eq!(root.children.len(), 4);
    assert_eq!(root.children.capacity(), 4, "text event count must not become XML child capacity");
    assert_eq!(root.text().len(), 2001);
    let children: Vec<_> = root.elements().collect();
    assert_eq!(children[0].namespace, "new");
    assert_eq!(children[1].namespace, "old");
    assert_eq!(children[0].attribute_namespaces.get("p:x").unwrap(), "new");
    assert_eq!(children[1].attribute_namespaces.get("p:y").unwrap(), "old");
    let mut used = 0;
    misterdiagram_opc::xml::charge_retained_nodes(input.as_bytes(), &mut used, 10, 0).unwrap();
    assert_eq!(used, 9);
    let mut bytes = vec![0xff, 0xfe];
    for u in input.encode_utf16() {
        bytes.extend(u.to_le_bytes());
    }
    assert_eq!(parse_xml(&bytes).unwrap(), root);
}
#[test]
fn duplicate_expanded_attributes_and_hostile_preflight_are_rejected() {
    assert!(parse_xml(b"<a xmlns:p='same' xmlns:q='same' p:x='1' q:x='2'/>").is_err());
    assert!(parse_xml(format!("{}{}", "<x>".repeat(257), "</x>".repeat(257)).as_bytes()).is_err());
    let mut used = 3;
    let e = misterdiagram_opc::xml::charge_retained_nodes(b"<a x='1'/>", &mut used, 4, 3).unwrap_err();
    assert!(e.to_string().contains("found 5, limit 4, already open 3"));
}
#[test]
#[cfg(target_os = "macos")]
fn retained_package_at_node_budget_edge_stays_under_1_gib() {
    if std::env::var_os("MISTERDIAGRAM_RETAINED_EDGE_CHILD").is_some() {
        let overhead = misterdiagram_vsdx::import(&package("", 1)).unwrap().retained_xml_nodes;
        let bytes = package(&"<a/>".repeat(MAX_RETAINED_XML_NODES - overhead - 1), 1);
        let d = misterdiagram_vsdx::import(&bytes).unwrap();
        assert_eq!(d.retained_xml_nodes, MAX_RETAINED_XML_NODES - 1);
        let evaluator = misterdiagram_vsdx::evaluate::Evaluator::new(&d).unwrap();
        assert!(evaluator.scopes().count() > 0);
        return;
    }
    let output = std::process::Command::new("/usr/bin/time")
        .arg("-l")
        .arg(std::env::current_exe().unwrap())
        .args(["--exact", "retained_package_at_node_budget_edge_stays_under_1_gib", "--nocapture"])
        .env("MISTERDIAGRAM_RETAINED_EDGE_CHILD", "1")
        .output()
        .unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(output.status.success(), "{stderr}");
    let rss: u64 = stderr.lines().find(|l| l.contains("maximum resident set size")).unwrap().split_whitespace().next().unwrap().parse().unwrap();
    eprintln!("retained XML budget edge: {rss} bytes peak RSS");
    assert!(rss < 1024 * 1024 * 1024, "budget-edge RSS {rss} exceeds 1 GiB");
}
