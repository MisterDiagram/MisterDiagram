//! Contributor-original command sequences and Microsoft reference examples.
use diagramcraft_engine::Session;
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
fn run(s: &mut Session, id: &str, p: Value) -> Value {
    s.execute(id, &p).unwrap_or_else(|e| panic!("{id} {p}: {e}"))
}
fn number(s: &Session, id: u32, name: &str) -> f64 {
    let mut e = Evaluator::new(s.document()).unwrap();
    e.evaluate(Scope::PageShape { page: s.document().pages[0].id, shape: id }, name).numeric().unwrap()
}
#[test]
fn pages_background_scale_and_layers_are_transactional() {
    let mut s = Session::new();
    run(&mut s, "page.add", json!({"name":"Background"}));
    run(&mut s, "page.background", json!({"page":1,"background":2}));
    assert_eq!(s.document().pages[0].attributes.get("BackPage").unwrap(), "1");
    let written = diagramcraft_vsdx::write(s.document()).unwrap();
    let round = diagramcraft_vsdx::import(&written.bytes).unwrap();
    assert!(round.parts.values().flat_map(|p| &p.relationships).any(|r| r.type_uri.ends_with("/page") && r.target.ends_with("page2.xml")));
    run(&mut s, "page.size", json!({"page":1,"width":20,"height":10}));
    run(&mut s, "page.scale", json!({"page":1,"page_scale":1,"drawing_scale":10}));
    run(&mut s, "page.rename", json!({"page":1,"name":"Network"}));
    run(&mut s, "page.add", json!({"name":"Details"}));
    run(&mut s, "page.reorder", json!({"page":3,"index":1}));
    assert_eq!(s.document().pages[0].name, "Details");
    run(&mut s, "page.delete", json!({"page":3}));
    assert!(!s.document().pages[1].attributes.contains_key("BackPage"));
    run(&mut s, "page.background", json!({"page":1,"background":null}));
    run(&mut s, "shape.draw.rect", json!({"page":1,"x":0,"y":0,"width":2,"height":1}));
    run(&mut s, "layer.add", json!({"page":1,"name":"First"}));
    run(&mut s, "layer.add", json!({"page":1,"name":"Second"}));
    run(&mut s, "layer.assign", json!({"page":1,"id":1,"layers":[0,1]}));
    run(&mut s, "layer.rename", json!({"page":1,"layer":1,"name":"Fabric"}));
    run(&mut s, "layer.delete", json!({"page":1,"layer":0}));
    assert_eq!(s.document().pages[0].shapes[0].sheet.cell("LayerMember").unwrap().value.as_deref(), Some("0"));
    assert_eq!(s.document().pages[0].sheet.sections.iter().find(|s| s.name == "Layer").unwrap().rows.len(), 1);
}
#[test]
fn draw_transform_order_delete_and_undo() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":2,"width":3,"height":4}));
    run(&mut s, "shape.draw.ellipse", json!({"x":5,"y":2,"width":2,"height":2}));
    run(&mut s, "shape.draw.line", json!({"x1":0,"y1":0,"x2":3,"y2":4}));
    assert!((number(&s, 3, "Width") - 5.).abs() < 1e-10);
    assert_eq!(s.document().pages[0].shapes[1].sheet.sections[0].rows[0].row_type.as_deref(), Some("Ellipse"));
    run(&mut s, "shape.move", json!({"id":1,"x":8,"y":9}));
    run(&mut s, "shape.resize", json!({"id":1,"width":6,"height":3}));
    run(&mut s, "shape.rotate", json!({"id":1,"angle":0.5}));
    run(&mut s, "shape.flip", json!({"id":1,"axis":"x"}));
    assert_eq!(number(&s, 1, "PinX"), 8.);
    assert_eq!(number(&s, 1, "LocPinX"), 3.);
    assert_eq!(number(&s, 1, "FlipX"), 1.);
    assert_eq!(number(&s, 1, "Angle"), 0.5);
    run(&mut s, "shape.move", json!({"id":3,"x":6.5,"y":7}));
    assert_eq!(number(&s, 3, "BeginX"), 5.);
    assert_eq!(number(&s, 3, "BeginY"), 5.);
    for (direction, expected) in [("front", vec![2, 3, 1]), ("back", vec![1, 2, 3]), ("forward", vec![2, 1, 3]), ("backward", vec![1, 2, 3])] {
        run(&mut s, "shape.order", json!({"id":1,"order":direction}));
        assert_eq!(s.document().pages[0].shapes.iter().map(|s| s.id).collect::<Vec<_>>(), expected);
    }
    run(&mut s, "shape.delete", json!({"id":2}));
    assert_eq!(s.document().pages[0].shapes.len(), 2);
    run(&mut s, "edit.undo", json!({}));
    assert_eq!(s.document().pages[0].shapes.len(), 3);
}
#[test]
fn formula_u_reference_example_and_hostile_formulas() {
    // FormulaU example: Scratch.X1 = MIN(Width,Height)/5.
    // https://learn.microsoft.com/en-us/office/vba/api/visio.cell.formulau
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":4,"height":4}));
    run(&mut s, "sheet.section.add", json!({"id":1,"section":"Scratch"}));
    run(&mut s, "sheet.row.add", json!({"id":1,"section":"Scratch"}));
    run(&mut s, "sheet.set", json!({"id":1,"name":"Scratch.X1","formula":"=MIN(Width,Height)/5"}));
    assert!((number(&s, 1, "Scratch.X1") - 0.8).abs() < 1e-10);
    run(&mut s, "sheet.section.add", json!({"id":1,"section":"Geometry","section_index":1}));
    run(&mut s, "sheet.row.add", json!({"id":1,"section":"Geometry","section_index":1,"type":"MoveTo"}));
    run(&mut s, "sheet.set", json!({"id":1,"name":"Geometry2.X1","formula":"GUARD(Width*0+Scratch.X1)"}));
    let before = s.document().clone();
    for params in [
        json!({"id":1,"name":"Geometry2.X1","value":3}),
        json!({"id":1,"name":"Width","formula":"1+"}),
        json!({"id":1,"name":"Width","formula":"(".repeat(300)}),
        json!({"id":1,"name":"InventedCell","value":2}),
    ] {
        assert!(s.execute("sheet.set", &params).is_err());
        assert_eq!(s.document(), &before);
    }
    run(&mut s, "sheet.set", json!({"id":1,"name":"Geometry2.X1","value":3,"force":true}));
    assert_eq!(run(&mut s, "sheet.get", json!({"id":1,"name":"Geometry2.X1"}))["value"], "3");
    run(&mut s, "sheet.section.add", json!({"id":1,"section":"User"}));
    run(&mut s, "sheet.row.add", json!({"id":1,"section":"User","row_name":"Counter"}));
    run(&mut s, "sheet.set", json!({"id":1,"name":"User.Counter","formula":"Width*2"}));
    assert_eq!(number(&s, 1, "User.Counter"), 8.);
}

#[test]
fn text_runs_properties_hyperlinks_and_formatting() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":4,"height":2}));
    run(&mut s, "shape.text", json!({"id":1,"text":"Rectangle Shape"}));
    {
        let mut e = Evaluator::new(s.document()).unwrap();
        let body = e.text_body(Scope::PageShape { page: 0, shape: 1 }).unwrap();
        assert!(body.paragraphs.iter().flat_map(|p| &p.runs).any(|r| r.text.contains("Rectangle Shape")));
    }
    run(
        &mut s,
        "shape.text",
        json!({"id":1,"paragraphs":[
            {"align":0,"runs":[{"text":"Left ","bold":true,"font":"Arimo","size":14},{"text":"tail","italic":true,"color":"#cc0000"}]},
            {"align":2,"runs":[{"text":"Right","underline":true}]}
        ]}),
    );
    {
        let mut e = Evaluator::new(s.document()).unwrap();
        let body = e.text_body(Scope::PageShape { page: 0, shape: 1 }).unwrap();
        assert_eq!(body.paragraphs.len(), 2);
        assert_eq!(e.evaluate(Scope::PageShape { page: 0, shape: 1 }, "Char.Style").numeric().unwrap(), 1.);
        assert_eq!(e.evaluate(Scope::PageShape { page: 0, shape: 1 }, "Char.Style[2]").numeric().unwrap(), 2.);
        assert_eq!(e.evaluate(Scope::PageShape { page: 0, shape: 1 }, "Para.HorzAlign[2]").numeric().unwrap(), 2.);
        diagramcraft_model::text::TextHost::measure(&diagramcraft_text::Host, &body, Some(4.)).unwrap();
    }
    run(&mut s, "format.fill", json!({"id":1,"color":"#cc0000","pattern":1,"transparency":0.25}));
    run(&mut s, "format.line", json!({"id":1,"color":"#0000cc","weight":0.02,"pattern":2,"end_arrow":4}));
    run(&mut s, "format.text", json!({"id":1,"size":18,"color":"#112233","bold":true,"align":1}));
    assert_eq!(number(&s, 1, "Char.Size"), 0.25);
    assert_eq!(number(&s, 1, "Char.Style[2]"), 3.);
    assert_eq!(number(&s, 1, "Char.Style[3]"), 5.);
    assert_eq!(number(&s, 1, "LineWeight"), 0.02);
    assert_eq!(number(&s, 1, "FillForegndTrans"), 0.25);
    for (name, kind, value) in [("Label", 0, json!("rack a")), ("Enabled", 3, json!(true)), ("Ports", 2, json!(48))] {
        run(&mut s, "shape.data.define", json!({"id":1,"name":name,"type":kind,"label":name,"sort_key":"01","invisible":false}));
        run(&mut s, "shape.data.set", json!({"id":1,"name":name,"value":value}));
    }
    assert_eq!(number(&s, 1, "Prop.Ports"), 48.);
    run(&mut s, "shape.data.define", json!({"id":1,"name":"Fixed","type":1,"format":"A;B"}));
    run(&mut s, "shape.data.set", json!({"id":1,"name":"Fixed","value":"B"}));
    let before = s.document().clone();
    assert!(s.execute("shape.data.set", &json!({"id":1,"name":"Fixed","value":"C"})).is_err());
    assert_eq!(s.document(), &before);
    run(&mut s, "shape.data.define", json!({"id":1,"name":"Variable","type":4,"format":"A;B"}));
    run(&mut s, "shape.data.set", json!({"id":1,"name":"Variable","value":"C"}));
    assert_eq!(run(&mut s, "sheet.get", json!({"id":1,"name":"Prop.Variable.Format"}))["value"], "A;B;C");
    run(&mut s, "shape.hyperlink", json!({"id":1,"name":"Reference","address":"https://example.org/diagram","description":"Original diagram"}));
    assert_eq!(run(&mut s, "sheet.get", json!({"id":1,"name":"Hyperlink.Reference.Address"}))["value"], "https://example.org/diagram");
}

fn world(s: &Session, id: u32) -> diagramcraft_geom::Affine {
    let mut e = Evaluator::new(s.document()).unwrap();
    e.shape_to_page(Scope::PageShape { page: 0, shape: id }).unwrap()
}
fn assert_affine(a: diagramcraft_geom::Affine, b: diagramcraft_geom::Affine) {
    for (a, b) in a.as_coeffs().into_iter().zip(b.as_coeffs()) {
        assert!((a - b).abs() < 1e-9, "{a} != {b}");
    }
}
#[test]
fn grouping_ungrouping_preserve_rotated_flipped_child_positions() {
    let mut s = Session::new();
    for x in [1, 5] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":2,"width":2,"height":1}));
    }
    run(&mut s, "shape.rotate", json!({"id":1,"angle":0.3}));
    run(&mut s, "shape.flip", json!({"id":2,"axis":"x"}));
    let before = [world(&s, 1), world(&s, 2)];
    let group = run(&mut s, "shape.group", json!({"ids":[1,2]}))["id"].as_u64().unwrap() as u32;
    assert_eq!(s.document().pages[0].shapes.len(), 1);
    for (i, id) in [1, 2].into_iter().enumerate() {
        assert_affine(before[i], world(&s, id));
    }
    let old_width = number(&s, 1, "Width");
    let group_width = number(&s, group, "Width");
    let group_height = number(&s, group, "Height");
    run(&mut s, "shape.resize", json!({"id":group,"width":group_width*2.,"height":group_height*2.}));
    assert!((number(&s, 1, "Width") - old_width * 2.).abs() < 1e-9);
    run(&mut s, "shape.rotate", json!({"id":group,"angle":0.7}));
    run(&mut s, "shape.flip", json!({"id":group,"axis":"y"}));
    let transformed = [world(&s, 1), world(&s, 2)];
    run(&mut s, "shape.ungroup", json!({"id":group}));
    for (i, id) in [1, 2].into_iter().enumerate() {
        assert_affine(transformed[i], world(&s, id));
    }
}
#[test]
fn duplicate_remaps_internal_formulas_and_arrangement_has_expected_positions() {
    let mut s = Session::new();
    for x in [0, 2, 10] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":x,"width":2,"height":1}));
    }
    run(&mut s, "sheet.section.add", json!({"id":1,"section":"User"}));
    run(&mut s, "sheet.row.add", json!({"id":1,"section":"User","row_name":"OwnWidth"}));
    run(&mut s, "sheet.set", json!({"id":1,"name":"User.OwnWidth","formula":"Sheet.1!Width"}));
    let duplicate = run(&mut s, "shape.duplicate", json!({"id":1,"dx":1,"dy":1}))["id"].as_u64().unwrap() as u32;
    run(&mut s, "shape.resize", json!({"id":duplicate,"width":4,"height":3}));
    assert_eq!(number(&s, duplicate, "User.OwnWidth"), 4.);
    assert_eq!(number(&s, 1, "User.OwnWidth"), 2.);
    run(&mut s, "arrange.align", json!({"ids":[1,2,3],"mode":"middle"}));
    assert_eq!(number(&s, 1, "PinY"), number(&s, 2, "PinY"));
    assert_eq!(number(&s, 1, "PinY"), number(&s, 3, "PinY"));
    run(&mut s, "arrange.distribute", json!({"ids":[3,1,2],"axis":"x"}));
    assert_eq!(number(&s, 2, "PinX"), 6.);
    run(&mut s, "arrange.same_size", json!({"ids":[duplicate,1,2,3]}));
    for id in [1, 2, 3] {
        assert_eq!(number(&s, id, "Width"), 4.);
        assert_eq!(number(&s, id, "Height"), 3.);
    }
}

#[test]
fn group_transform_formulas_match_the_private_visio_witness() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/generated/group/two-rects-rotated.vsdx");
    if !path.exists() {
        eprintln!("skip: private group witness absent");
        return;
    }
    let oracle = diagramcraft_vsdx::read(path).unwrap();
    let mut s = Session::new();
    for x in [1, 3] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":4,"width":2,"height":2}));
    }
    run(&mut s, "shape.flip", json!({"id":1,"axis":"x"}));
    run(&mut s, "shape.group", json!({"ids":[1,2]}));
    run(&mut s, "shape.rotate", json!({"id":3,"angle":std::f64::consts::PI/6.}));
    let mut expected = Evaluator::new(&oracle).unwrap();
    let mut actual = Evaluator::new(s.document()).unwrap();
    for id in [1, 2, 3] {
        for name in ["PinX", "PinY", "Width", "Height", "LocPinX", "LocPinY", "Angle", "FlipX", "FlipY"] {
            let scope = Scope::PageShape { page: 0, shape: id };
            let want = expected.evaluate(scope, name).numeric().unwrap();
            let got = actual.evaluate(scope, name).numeric().unwrap();
            assert!((want - got).abs() <= 1e-6, "shape {id} {name}: {want} vs {got}");
            if id != 3 && matches!(name, "PinX" | "PinY" | "Width" | "Height") {
                assert_eq!(actual.cell(scope, name).unwrap().formula, expected.cell(scope, name).unwrap().formula);
            }
        }
    }
}

#[test]
fn line_resize_and_rotation_keep_endpoint_and_transform_cells_consistent() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.line", json!({"x1":0,"y1":0,"x2":4,"y2":0}));
    run(&mut s, "shape.resize", json!({"id":1,"width":6,"height":0}));
    assert_eq!(number(&s, 1, "BeginX"), -1.);
    assert_eq!(number(&s, 1, "EndX"), 5.);
    run(&mut s, "shape.rotate", json!({"id":1,"angle":std::f64::consts::FRAC_PI_2}));
    assert!((number(&s, 1, "BeginX") - 2.).abs() < 1e-9);
    assert!((number(&s, 1, "EndX") - 2.).abs() < 1e-9);
    assert_eq!(number(&s, 1, "BeginY"), -3.);
    assert_eq!(number(&s, 1, "EndY"), 3.);
    assert!((number(&s, 1, "Width") - 6.).abs() < 1e-9);
}

/// Full local cells, including LangID, must survive save exactly as the original.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-data-section
#[test]
fn property_rows_match_visio_original_local_cells() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let witness = root.join("corpus/generated/data/property-rows.vsdx");
    if !witness.is_file() {
        eprintln!("skip: private Property witness absent");
        return;
    }
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":6,"width":1.5,"height":1.5}));
    for (name, kind, format, value) in [
        ("Name", 0, "", json!("Switch A")),
        ("DomainID", 2, "", json!(42)),
        ("Fabric", 1, "A;B", json!("A")),
        ("Installed", 5, "yyyy-MM-dd", json!(45000)),
    ] {
        run(&mut s, "shape.data.define", json!({"id":1,"name":name,"type":kind,"format":format}));
        run(&mut s, "shape.data.set", json!({"id":1,"name":name,"value":value}));
    }
    let bytes = diagramcraft_vsdx::write(s.document()).unwrap().bytes;
    let actual = diagramcraft_vsdx::import(&bytes).unwrap();
    let expected = diagramcraft_vsdx::read(witness).unwrap();
    let cells = |doc: &diagramcraft_model::Document| {
        doc.pages[0].shapes[0]
            .sheet
            .sections
            .iter()
            .find(|s| s.name == "Property")
            .unwrap()
            .rows
            .iter()
            .flat_map(|row| {
                row.cells
                    .iter()
                    .map(move |c| ((row.name.clone(), c.name.clone()), (c.value.clone(), c.formula.clone(), c.units.clone(), c.error.clone())))
            })
            .collect::<std::collections::BTreeMap<_, _>>()
    };
    assert_eq!(cells(&actual), cells(&expected));
}

/// Renderer fallback is not a document font declaration.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/facenames-element-visiodocument_type-complextypevisio-xml
#[test]
fn authoring_and_rendering_text_add_no_substitute_face() {
    let mut s = Session::new();
    let original = s.document().face_names.clone();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}));
    run(&mut s, "shape.text", json!({"id":1,"text":"Inherited document face"}));
    run(&mut s, "format.text", json!({"id":1,"size":16,"bold":true,"align":1}));
    let scene = diagramcraft_render::scene::page(s.document(), 0, Default::default()).unwrap();
    assert!(!scene.ops.is_empty());
    let bytes = diagramcraft_vsdx::write(s.document()).unwrap().bytes;
    let round = diagramcraft_vsdx::import(&bytes).unwrap();
    assert_eq!(round.face_names, original);
    assert_eq!(s.document().face_names, original);
    run(&mut s, "format.text", json!({"id":1,"font":"Example Explicit Face"}));
    assert_eq!(s.document().face_names.len(), original.len() + 1, "explicit font remains supported");
    assert!(s.document().face_names.values().any(|name| name == "Example Explicit Face"));
}

/// Inherited paragraph formatting needs no authored pp marker, including in a new master.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pp-element-text_type-complextypevisio-xml
#[test]
fn authored_inherited_paragraph_has_no_redundant_pp() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}));
    run(&mut s, "shape.text", json!({"id":1,"text":"Switch\nOriginal"}));
    let align = number(&s, 1, "Para.HorzAlign") as u32;
    run(&mut s, "format.text", json!({"id":1,"align":align}));
    run(&mut s, "master.new_from_shape", json!({"id":1,"name":"Switch"}));
    let bytes = diagramcraft_vsdx::write(s.document()).unwrap().bytes;
    let round = diagramcraft_vsdx::import(&bytes).unwrap();
    let master = round.masters.iter().find(|m| m.name == "Switch").unwrap();
    for shape in [&round.pages[0].shapes[0], &master.shapes[0]] {
        assert!(!shape.sheet.sections.iter().any(|section| section.name == "Paragraph"));
        assert!(!shape.text.as_ref().unwrap().elements().any(|e| e.is_visio("pp")), "inherited-only pp must be absent");
    }
    run(&mut s, "format.text", json!({"id":1,"align":if align == 0 {1} else {0}}));
    let round = diagramcraft_vsdx::import(&diagramcraft_vsdx::write(s.document()).unwrap().bytes).unwrap();
    assert!(round.pages[0].shapes[0].text.as_ref().unwrap().elements().any(|e| e.is_visio("pp")), "local paragraph formatting retains its marker");
}

#[test]
fn sheet_set_names_a_missing_shape() {
    let mut s = Session::new();
    assert!(
        matches!(s.execute("sheet.set", &json!({"id":999,"name":"Width","value":2})), Err(diagramcraft_engine::EngineError::Operation(message)) if message == "shape not found")
    );
}
