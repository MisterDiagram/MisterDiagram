use misterdiagram_sheet::{Context, Error, Value, formula};

struct Input(Value);
impl Context for Input {
    fn resolve(&mut self, name: &str) -> Value {
        if name == "Input" { self.0.clone() } else { Value::Error(Error::Ref) }
    }
}
fn result(source: &str) -> Value {
    formula(source, &mut Input(Value::Error(Error::Ref)))
}

#[test]
fn trim_public_example_and_reference_coercion() {
    assert_eq!(result("TRIM(\"January 1, 2003 \" )"), Value::Text("January 1, 2003".into()));
    assert_eq!(result("TRIM(\"  alpha   beta  \" )"), Value::Text("alpha beta".into()));
    assert_eq!(result("TRIM(42)"), Value::Text("42".into()));
    assert_eq!(result("TRIM(TRUE)"), Value::Text("TRUE".into()));
    assert_eq!(result("TRIM(\"\")"), Value::Text(String::new()));
    assert_eq!(result("TRIM(\"    \")"), Value::Text(String::new()));
}

#[test]
fn trim_uses_the_published_scalar_set_not_host_unicode_whitespace() {
    let points = [
        9, 10, 11, 12, 13, 32, 160, 0x1680, 0x180e, 0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007, 0x2008, 0x2009, 0x200a, 0x202f,
        0x205f, 0x3000,
    ];
    for point in points {
        let ch = char::from_u32(point).unwrap();
        let mut input = Input(Value::Text(format!("{ch}{ch}é{ch}{ch}λ{ch}{ch}")));
        assert_eq!(formula("TRIM(Input)", &mut input), Value::Text("é λ".into()), "U+{point:04X}");
    }
    for ch in ['\u{0085}', '\u{2028}', '\u{2029}', '\u{200b}'] {
        let original = format!("{ch}é{ch}λ{ch}");
        assert_eq!(formula("TRIM(Input)", &mut Input(Value::Text(original.clone()))), Value::Text(original));
    }
}

#[test]
fn trim_propagates_errors_and_obeys_text_limits() {
    for source in ["TRIM()", "TRIM(1,2)"] {
        assert_eq!(result(source), Value::Error(Error::Value));
    }
    assert_eq!(result("TRIM(1/0)"), Value::Error(Error::Div0));
    assert_eq!(result("TRIM(Missing)"), Value::Error(Error::Ref));
    assert_eq!(formula("TRIM(Input)", &mut Input(Value::Text("a".repeat(65536)))), Value::Text("a".repeat(65536)));
    assert_eq!(formula("TRIM(Input)", &mut Input(Value::Text("a".repeat(65537)))), Value::Error(Error::Limit));
}
