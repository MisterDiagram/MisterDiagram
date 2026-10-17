// Review 1a attribute regression, scaled with Charles Hood's permission.
// MIT OR Apache-2.0. Source recorded in ATTRIBUTION.md.
use misterdiagram_opc::xml;
use std::time::{Duration, Instant};

fn attributes(n: usize) -> String {
    let mut xml = String::from("<a");
    for i in 0..n {
        xml.push_str(&format!(" a{i}=\"1\""));
    }
    xml.push_str("/>");
    xml
}

#[test]
fn many_attributes_on_one_element_parse_in_linear_time() {
    fn measure(n: usize) -> Duration {
        let input = attributes(n);
        let start = Instant::now();
        let parsed = xml::parse(input.as_bytes()).unwrap();
        let elapsed = start.elapsed();
        assert_eq!(parsed.root.attrs.len(), n);
        elapsed
    }
    let small = measure(20_000);
    let large = measure(100_000);
    eprintln!("20000 attributes: {small:?}; 100000 attributes: {large:?}");
    assert!(large <= small * 12 + Duration::from_millis(200), "attribute parse growth: {large:?} exceeds 12 times {small:?} plus 200 ms");
}

#[test]
fn duplicate_attributes_are_rejected_even_at_the_end() {
    for input in ["<a x='1' x='2'/>", "<a xmlns:x='urn:x' xmlns:x='urn:y'/>"] {
        assert!(xml::parse(input.as_bytes()).unwrap_err().to_string().contains("duplicate"));
    }
    let input = attributes(1000).replace("/>", " a0='2'/>");
    assert!(xml::parse(input.as_bytes()).unwrap_err().to_string().contains("duplicate"));
    assert!(xml::parse(b"<a x='1' X='2'/>").is_ok());
}
