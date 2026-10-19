//! Typed line weights match the COM and XML observation (#62).
//! https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
use misterdiagram_engine::Session;
use serde_json::json;
#[test]
fn typed_points_serialize_like_visio_and_inches_remain_default() {
    let mut s = Session::new();
    s.execute("shape.draw.rect", &json!({"x":0,"y":0,"width":1,"height":1})).unwrap();
    s.execute("format.line", &json!({"id":1,"weight":1.5,"units":"PT"})).unwrap();
    let c = s.execute("sheet.get", &json!({"id":1,"name":"LineWeight"})).unwrap();
    assert!((c["value"].as_str().unwrap().parse::<f64>().unwrap() - 1.5 / 72.).abs() < 1e-14);
    assert_eq!(c["units"], "PT");
    assert!(c["formula"].is_null());
    let bytes = misterdiagram_vsdx::write(s.document()).unwrap().bytes;
    let read = misterdiagram_vsdx::import(&bytes).unwrap();
    let c = read.pages[0].shapes[0].sheet.cells.iter().find(|c| c.name == "LineWeight").unwrap();
    assert_eq!(c.units.as_deref(), Some("PT"));
    assert!(c.formula.is_none());
    s.execute("format.line", &json!({"id":1,"weight":1.5})).unwrap();
    assert_eq!(s.execute("sheet.get", &json!({"id":1,"name":"LineWeight"})).unwrap()["value"], "1.5");
    let before = s.document().clone();
    for p in [
        json!({"id":1,"weight":1.5,"units":"DEG"}),
        json!({"id":1,"weight":-1,"units":"PT"}),
        json!({"id":1,"units":"PT"}),
        json!({"id":1,"weight":1e6,"units":"FT"}),
    ] {
        assert!(s.execute("format.line", &p).is_err());
        assert_eq!(s.document(), &before);
    }
}
