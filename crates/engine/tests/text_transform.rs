//! Optional Text Transform cells are top-level cells on every shape (#61).
//! https://learn.microsoft.com/en-us/office/client-developer/visio/text-transform-section
use misterdiagram_engine::Session;
use serde_json::json;
#[test]
fn text_transform_cells_are_settable_on_plain_shapes_and_master_instances() {
    let mut s = Session::new();
    s.execute("shape.draw.rect", &json!({"x":0,"y":0,"width":1,"height":1})).unwrap();
    s.execute("master.new_from_shape", &json!({"id":1,"name":"Plain"})).unwrap();
    s.execute("master.drop", &json!({"master":"Plain","x":0,"y":0})).unwrap();
    for id in [1, 2] {
        for (n, v) in [
            ("TxtPinX", 0.5),
            ("TxtPinY", -0.25),
            ("TxtWidth", 1.5),
            ("TxtHeight", 0.25),
            ("TxtLocPinX", 0.75),
            ("TxtLocPinY", 0.125),
            ("TxtAngle", 0.),
        ] {
            s.execute("sheet.set", &json!({"id":id,"name":n,"value":v})).unwrap();
            assert_eq!(s.execute("sheet.get", &json!({"id":id,"name":n})).unwrap()["value"].as_str().unwrap().parse::<f64>().unwrap(), v);
        }
    }
    let before = s.document().clone();
    for params in
        [json!({"id":999,"name":"TxtPinX","value":1}), json!({"id":1,"name":"TxtInvalid","value":1}), json!({"id":1,"name":"TxtAngle","formula":"("})]
    {
        assert!(s.execute("sheet.set", &params).is_err());
        assert_eq!(s.document(), &before);
    }
    let bytes = misterdiagram_vsdx::write(s.document()).unwrap().bytes;
    let read = misterdiagram_vsdx::import(&bytes).unwrap();
    assert_eq!(read.pages[0].shapes.len(), 2);
}

#[test]
fn first_text_transform_edit_materializes_untouched_companion_defaults() {
    let mut s = Session::new();
    s.execute("shape.draw.rect", &json!({"x":0,"y":0,"width":2,"height":1})).unwrap();
    s.execute("shape.text", &json!({"id":1,"text":"Label"})).unwrap();
    let before = s.document().clone();
    assert_eq!(s.execute("sheet.get", &json!({"id":1,"name":"TxtWidth"})).unwrap()["value"], "2");
    assert_eq!(s.document(), &before, "reading optional defaults must not mutate");
    s.execute("sheet.set", &json!({"id":1,"name":"TxtLocPinX","formula":"TxtWidth*0.5"})).unwrap();
    assert_eq!(s.execute("sheet.get", &json!({"id":1,"name":"TxtLocPinX"})).unwrap()["value"], "1");
    let options = misterdiagram_render::scene::Options::default();
    let scene = misterdiagram_render::scene::page(s.document(), 0, options).unwrap();
    assert!(!misterdiagram_render::svg::render(&scene, options).unwrap().is_empty());
    s.execute("shape.resize", &json!({"id":1,"width":4,"height":2})).unwrap();
    assert_eq!(s.execute("sheet.get", &json!({"id":1,"name":"TxtLocPinX"})).unwrap()["value"], "2");
}
