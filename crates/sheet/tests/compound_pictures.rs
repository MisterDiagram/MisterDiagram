//! Native numeric pictures and coercion; source-only, contributor-original inputs.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/format-function
//! https://learn.microsoft.com/en-us/office/client-developer/visio/formatex-function
use misterdiagram_sheet::{Context, Error, Value, formula};
#[path = "support/compound_cases.rs"]
mod cases;
struct Host;
impl Context for Host {
    fn resolve(&mut self, _: &str) -> Value {
        Value::Error(Error::Ref)
    }
}
#[test]
fn native_compound_pictures_and_nonzero_resultiu() {
    let mut failures = Vec::new();
    for c in cases::CASES {
        let result = formula(c.formula, &mut Host);
        if result.text() != c.expected {
            failures.push(format!("{} {}: {:?}, expected {:?}", c.id, c.formula, result.text(), c.expected));
        }
        if c.iu != 0. && !result.numeric().is_ok_and(|n| (n - c.iu).abs() <= 1e-6) {
            failures.push(format!("{} {}: IU {:?}, expected {}", c.id, c.formula, result.numeric(), c.iu));
        }
    }
    assert!(failures.is_empty(), "{} failures / {} cases:\n{}", failures.len(), cases::CASES.len(), failures.join("\n"));
}
#[test]
fn hostile_compound_pictures_and_finite_arithmetic() {
    for picture in [
        "<,FEET/INCH".to_owned(),
        "<,FEET/INCH><,FEET/INCH>0 u".into(),
        format!("<,FEET/INCH>{}", "0".repeat(4097)),
        "<,FEET/INCH># #/65537".into(),
        "<,FEET/INCH># #/999999999999999999999999".into(),
        "<,FEET/INCH># #/#####".into(),
        "<,FEET/INCH>0e+999".into(),
    ] {
        assert!(matches!(formula(&format!("FORMAT(72in,\"{picture}\")"), &mut Host), Value::Error(_)), "{picture}");
    }
    for value in ["1e999 in", "1.7976931348623157e308 in"] {
        for picture in ["<,FEET/INCH># #/#", "<,PICAPOINTS>0"] {
            assert!(matches!(formula(&format!("FORMAT({value},\"{picture}\")"), &mut Host), Value::Error(_)));
        }
    }
    assert!(matches!(formula(r#"FORMAT(1e100 in,"<,FEET/INCH># #/#")"#, &mut Host), Value::Error(_)));
    let large = formula(r#"FORMAT(1e100 in,"<,PICAPOINTS>0")"#, &mut Host);
    assert!(large.error().is_none() && large.text().len() < 1024);
    // Overflow in major * 12 must return an error, never an infinity-derived cast.
    assert!(matches!(formula(r#"FORMAT(1.7976931348623157e308 in,"<,FEET/INCH>0 u")"#, &mut Host), Value::Error(_)));
    for input in ["NaN", "inf", "1 p 2 p 3 p 4 p 5 p"] {
        assert!(misterdiagram_sheet::units::numeric_text(input).is_err());
    }
    assert_eq!(formula(r#"FORMAT(72in,"<,FEET/INCH>0 u 'é λ'")"#, &mut Host).text(), "6 ft. 0 in. é λ");
    // Unterminated quoted literals and trailing escapes are deliberately harmless.
    assert!(formula(r#"FORMAT(72in,"<,FEET/INCH>0 u 'open")"#, &mut Host).error().is_none());
    assert!(formula(r#"FORMAT(72in,"<,FEET/INCH>0 u \")"#, &mut Host).error().is_none());
}
