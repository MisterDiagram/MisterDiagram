//! Compact query contracts use original synthetic drawings only.
use misterdiagram_engine::Session;
use serde_json::{Value, json};
fn run(s: &mut Session, command: &str, p: Value) -> Value {
    s.execute(command, &p).unwrap()
}
fn rect(s: &mut Session) -> Value {
    run(s, "shape.draw.rect", json!({"x":1,"y":2,"width":3,"height":1}))["id"].clone()
}
fn error(s: &mut Session, command: &str, p: Value, message: &str) {
    let before = s.document().clone();
    assert!(s.execute(command, &p).unwrap_err().to_string().contains(message), "{p}");
    assert_eq!(s.document(), &before);
}
#[test]
fn a1_command_summaries_and_ordered_schema_lookup() {
    let mut s = Session::new();
    let full = run(&mut s, "engine.commands", json!({}));
    assert_eq!(full, serde_json::to_value(s.commands()).unwrap());
    assert_eq!(full, run(&mut s, "engine.commands", json!({"detail":"full"})));
    let summary = run(&mut s, "engine.commands", json!({"detail":"summary"}));
    assert_eq!(summary.as_array().unwrap().len(), s.commands().len());
    for (a, b) in summary.as_array().unwrap().iter().zip(full.as_array().unwrap()) {
        assert!(a["params"].is_array());
        assert_eq!(a["id"], b["id"]);
        assert_eq!(a["required"], b["params"].get("required").cloned().unwrap_or(json!([])));
        assert!(a.get("menu").is_none());
        if b["enabled"] == false {
            assert_eq!(a["enabled"], false);
            assert_eq!(a["disabled_reason"], b["disabled_reason"]);
        } else {
            assert!(a.get("enabled").is_none());
            assert!(a.get("disabled_reason").is_none());
        }
    }
    let ids = json!(["format.line", "shape.text", "format.line"]);
    let selected = run(&mut s, "engine.commands", json!({"ids":ids}));
    assert_eq!(selected.as_array().unwrap().iter().map(|v| v["id"].clone()).collect::<Vec<_>>(), ids.as_array().unwrap().clone());
    assert_eq!(selected[0], selected[2]);
    assert!(selected[0]["params"].is_object());
    let one = run(&mut s, "engine.commands", json!({"ids":["format.line"],"detail":"summary"}));
    assert_eq!(one[0]["params"], json!(["page", "id", "color", "pattern", "units", "weight", "transparency", "begin_arrow", "end_arrow"]));
    error(&mut s, "engine.commands", json!({"ids":["missing.command"]}), "missing.command");
    for p in [json!({"ids":[]}), json!({"ids":vec!["shape.text";129]}), json!({"ids":[null]}), json!({"detail":"other"})] {
        assert!(s.execute("engine.commands", &p).is_err());
    }
}
#[test]
fn a2_outline_is_small_and_groups_connector_ends() {
    let mut s = Session::new();
    let mut commands = Vec::new();
    for i in 0..30 {
        commands.push(json!({"command":"shape.draw.rect","params":{"x":(i % 10)*4,"y":(i / 10)*3,"width":3,"height":1}}));
        if i < 10 {
            commands.push(json!({"command":"shape.text","params":{"id":i+1,"text":format!("Node {i}")}}));
        }
    }
    for i in 0..5 {
        commands.push(json!({"command":"connector.add","params":{"from":i+1,"to":i+2}}));
    }
    run(&mut s, "engine.batch", json!({"commands":commands}));
    let v = run(&mut s, "document.inspect", json!({"sections":["outline"]}));
    let page = &v["pages"][0];
    let shapes = page["shapes"].as_array().unwrap();
    assert_eq!(shapes.len(), 35);
    assert_eq!(shapes.iter().filter(|s| s.get("text").is_some()).count(), 10);
    assert_eq!(page["links"].as_array().unwrap().len(), 5);
    for (i, link) in page["links"].as_array().unwrap().iter().enumerate() {
        assert_eq!(*link, json!({"id":31+i,"begin":1+i,"end":2+i}));
    }
    assert!(v.to_string().len() <= 150 * 35 + 512, "{}", v.to_string().len());
    assert_eq!(page.as_object().unwrap().keys().map(String::as_str).collect::<Vec<_>>(), vec!["id", "links", "name", "page", "shapes"]);
    assert!(shapes[0]["bounds"].is_array());
    assert!(shapes[0].get("type").is_none());
    error(&mut s, "document.inspect", json!({"sections":["outline","text"]}), "outline cannot be combined with other sections");
    error(&mut s, "shape.inspect", json!({"id":1,"sections":["outline"]}), "invalid inspection section");
    error(&mut s, "document.inspect", json!({"sections":vec!["text";9]}), "at most eight");
    for p in [json!({"sections":[null]}), json!({"sections":["bogus"]})] {
        assert!(s.execute("document.inspect", &p).is_err());
    }
    // One-ended links and filtering a target without its connector.
    s.edit_document(|doc, _| {
        doc.pages[0].connects.retain(|c| {
            !(c.from_sheet == 31 && c.from_cell.as_deref() == Some("EndX")) && !(c.from_sheet == 32 && c.from_cell.as_deref() == Some("BeginX"))
        });
        Ok(())
    })
    .unwrap();
    let v = run(&mut s, "document.inspect", json!({"page":1,"ids":[31,32,1],"sections":["outline"]}));
    assert_eq!(v["pages"][0]["links"], json!([{"id":31,"begin":1},{"id":32,"end":3}]));
    assert_eq!(run(&mut s, "document.inspect", json!({"page":1,"ids":[1],"sections":["outline"]}))["pages"][0]["links"], json!([]));
}
#[test]
fn a3_find_folds_case_and_whitespace_before_expansion() {
    let mut s = Session::new();
    rect(&mut s);
    rect(&mut s);
    run(&mut s, "shape.text", json!({"id":1,"text":"Écho\u{2028}  BIG\u{2028}Oak"}));
    s.edit_document(|doc, _| {
        doc.pages[0].shapes[1].name_u = "Named\t  Birch".into();
        Ok(())
    })
    .unwrap();
    assert!(run(&mut s, "shape.inspect", json!({"id":1,"sections":["text"]}))["text"].as_str().unwrap().contains('\u{b}'));
    for sections in [json!([]), json!(["outline"]), json!(["text"]), json!(["style"]), json!(["cells"])] {
        for query in ["éCHO big oak", "ÉCHO\nBIG\t OAK"] {
            let v = run(&mut s, "document.inspect", json!({"find":query,"sections":sections}));
            assert_eq!(v["pages"][0]["shapes"].as_array().unwrap().len(), 1);
            assert_eq!(v["pages"][0]["shapes"][0]["id"], 1);
        }
    }
    assert_eq!(run(&mut s, "document.inspect", json!({"find":"named birch"}))["pages"][0]["shapes"][0]["id"], 2);
    run(&mut s, "page.add", json!({"name":"Empty"}));
    let all = run(&mut s, "document.inspect", json!({"find":"écho","sections":["outline"]}));
    assert_eq!(all["pages"].as_array().unwrap().len(), 2);
    assert_eq!(all["pages"][1]["shapes"], json!([]));
    assert_eq!(run(&mut s, "document.inspect", json!({"find":"absent","page":1}))["pages"][0]["shapes"], json!([]));
    for query in ["".to_owned(), "x".repeat(257)] {
        assert!(s.execute("document.inspect", &json!({"find":query})).is_err());
    }
    assert!(s.execute("document.inspect", &json!({"find":"é".repeat(256)})).is_ok());
    // A different shape is too large to inspect; only returned shapes consume the budget.
    run(&mut s, "shape.text", json!({"page":1,"id":2,"text":"z".repeat(65530)}));
    assert!(s.execute("document.inspect", &json!({"page":1})).is_err());
    assert_eq!(run(&mut s, "document.inspect", json!({"page":1,"find":"écho"}))["pages"][0]["shapes"][0]["id"], 1);
}
#[test]
fn a4_ids_preserve_order_and_validate_combinations() {
    let mut s = Session::new();
    rect(&mut s);
    rect(&mut s);
    for sections in [json!([]), json!(["outline"]), json!(["text"]), json!(["style"])] {
        let v = run(&mut s, "document.inspect", json!({"page":1,"ids":[2,1,2],"sections":sections}));
        assert_eq!(
            v["pages"][0]["shapes"].as_array().unwrap().iter().map(|v| v["id"].clone()).collect::<Vec<_>>(),
            vec![json!(2), json!(1), json!(2)]
        );
    }
    error(&mut s, "document.inspect", json!({"ids":[1]}), "ids requires page");
    error(&mut s, "document.inspect", json!({"page":1,"ids":[1],"find":"x"}), "ids and find cannot be combined");
    error(&mut s, "document.inspect", json!({"page":1,"ids":[999]}), "shape not found: 999");
    for ids in [json!([]), json!(vec![1; 257]), json!([null]), json!([-1]), json!([4294967296_u64]), json!([1.5])] {
        assert!(s.execute("document.inspect", &json!({"page":1,"ids":ids})).is_err());
    }
}
fn style(s: &mut Session, id: Value) -> Value {
    run(s, "shape.inspect", json!({"id":id,"sections":["style"]}))["style"].clone()
}
fn copy_style(s: &mut Session, p: Value, q: Value) {
    let source = style(s, p);
    for (section, keys) in [
        ("fill", vec!["color", "pattern", "transparency"]),
        ("line", vec!["begin_arrow", "color", "end_arrow", "pattern", "transparency", "units", "weight"]),
        ("text", vec!["align", "bold", "color", "font", "italic", "size", "underline"]),
    ] {
        assert_eq!(source[section].as_object().unwrap().keys().map(String::as_str).collect::<Vec<_>>(), keys);
        let mut params = source[section].clone();
        params["id"] = q.clone();
        run(s, &format!("format.{section}"), params);
    }
    assert_eq!(source, style(s, q));
}
#[test]
fn a5_style_roundtrip_explicit_and_inherited_theme_colors() {
    let mut s = Session::new();
    let p = rect(&mut s);
    let q = rect(&mut s);
    run(&mut s, "format.fill", json!({"id":p,"color":"#ABCDEF","pattern":1,"transparency":0.25}));
    run(
        &mut s,
        "format.line",
        json!({"id":p,"color":"#654321","weight":1.23456,"units":"PT","pattern":2,"begin_arrow":1,"end_arrow":4,"transparency":0.3}),
    );
    run(
        &mut s,
        "format.text",
        json!({"id":p,"font":"Arial","size":15.123456,"color":"#123ABC","bold":true,"italic":true,"underline":true,"align":2}),
    );
    let st = style(&mut s, p.clone());
    assert_eq!(st["fill"]["color"], "#abcdef");
    assert_eq!(st["line"]["weight"], 1.2346);
    assert_eq!(st["text"]["size"], 15.1235);
    copy_style(&mut s, p.clone(), q.clone());
    // Original custom palette and theme formula, inherited through our authored master.
    s.edit_document(|doc, _| {
        doc.colors.insert(24, "#3175b9".into());
        let xml = br#"<theme><clrScheme><accent2><srgbClr val="264E95"/></accent2><schemeID schemeEnum="123"/></clrScheme></theme>"#;
        doc.parts.insert(
            "visio/theme/theme1.xml".into(),
            misterdiagram_model::Part {
                bytes: xml.to_vec().into(),
                xml: Some(misterdiagram_vsdx::parse_xml(xml).unwrap().into()),
                content_type: Some("application/vnd.openxmlformats-officedocument.theme+xml".into()),
                ..Default::default()
            },
        );
        Ok(())
    })
    .unwrap();
    run(&mut s, "sheet.set", json!({"id":p,"name":"ColorSchemeIndex","value":123}));
    run(&mut s, "sheet.set", json!({"id":p,"name":"FillForegnd","formula":"THEMEVAL(\"AccentColor2\")"}));
    run(&mut s, "sheet.set", json!({"id":p,"name":"LineColor","value":24}));
    run(&mut s, "master.new_from_shape", json!({"id":p,"name":"Original style"}));
    let instance = run(&mut s, "master.drop", json!({"master":"Original style","x":6,"y":5}))["id"].clone();
    let inherited = style(&mut s, instance.clone());
    assert_eq!(inherited["line"]["color"], "#3175b9");
    assert_eq!(inherited["fill"]["color"], "#264e95");
    copy_style(&mut s, instance, q);
    let plain = rect(&mut s);
    assert!(!s.document().pages[0].shapes.last().unwrap().sheet.sections.iter().any(|section| section.name == "Character"));
    let plain_style = style(&mut s, plain);
    assert!(plain_style["text"]["font"].is_string());
    assert!(plain_style["text"]["size"].as_f64().unwrap() > 0.);
}
#[test]
fn a6_cells_are_returned_beside_sections() {
    let mut s = Session::new();
    rect(&mut s);
    for command in ["shape.inspect", "document.inspect"] {
        let mut p = json!({"sections":["xform"],"cells":["FillForegnd"]});
        if command == "shape.inspect" {
            p["id"] = json!(1);
        }
        let v = run(&mut s, command, p);
        let shape = if command == "shape.inspect" { &v } else { &v["pages"][0]["shapes"][0] };
        assert!(shape["xform"].is_object());
        assert!(shape["cells"]["FillForegnd"].is_object());
    }
    let v = run(&mut s, "document.inspect", json!({"sections":["outline"],"cells":["Width"]}));
    assert_eq!(v["pages"][0]["shapes"][0]["cells"]["Width"]["value"], "3");
}
#[test]
fn a7_text_cells_include_only_present_transforms() {
    let mut s = Session::new();
    rect(&mut s);
    assert!(run(&mut s, "shape.inspect", json!({"id":1,"sections":["text"]})).get("text_cells").is_none());
    // The legacy default remains unchanged, including absent-cell records.
    assert_eq!(run(&mut s, "shape.inspect", json!({"id":1}))["text_cells"].as_object().unwrap().len(), 7);
    for name in ["TxtPinX", "TxtPinY", "TxtLocPinX", "TxtLocPinY", "TxtWidth", "TxtHeight", "TxtAngle"] {
        run(&mut s, "sheet.set", json!({"id":1,"name":name,"value":1}));
    }
    for command in ["shape.inspect", "document.inspect"] {
        let mut p = json!({"sections":["text"]});
        if command == "shape.inspect" {
            p["id"] = json!(1);
        }
        let v = run(&mut s, command, p);
        let shape = if command == "shape.inspect" { &v } else { &v["pages"][0]["shapes"][0] };
        assert_eq!(shape["text_cells"].as_object().unwrap().len(), 7);
    }
}
#[test]
fn a8_budget_error_advertises_all_filters() {
    let mut s = Session::new();
    rect(&mut s);
    run(&mut s, "shape.text", json!({"id":1,"text":"x".repeat(65530)}));
    error(&mut s, "document.inspect", json!({}), "inspection exceeds 65536 byte budget; use page, find, ids, cells or sections filters");
}

#[test]
fn style_uses_first_rows_instead_of_text_markers_and_resolves_palette() {
    use misterdiagram_model::{Cell, Row, XmlElement, XmlNode};
    let mut s = Session::new();
    rect(&mut s);
    run(
        &mut s,
        "shape.text",
        json!({"id":1,"paragraphs":[{"runs":[{"text":"first","font":"Arial","color":"#010203","size":14}],"align":2},{"runs":[{"text":"second","font":"Courier New","color":"#AABBCC","size":20}],"align":1}]}),
    );
    s.edit_document(|doc, _| {
        doc.colors.insert(24, "#ba7135".into());
        let shape = &mut doc.pages[0].shapes[0];
        let section = shape.sheet.sections.iter_mut().find(|section| section.name == "Character").unwrap();
        section.rows[0].cells.retain(|c| c.name != "Color");
        section.rows[0].cells.push(Cell { name: "Color".into(), value: Some("24".into()), ..Default::default() });
        // First stored text run selects the second row; inspection still reports row zero.
        shape.text = Some(XmlElement {
            name: "Text".into(),
            children: vec![
                XmlNode::Element(XmlElement { name: "cp".into(), attributes: [("IX".into(), "1".into())].into(), ..Default::default() }),
                XmlNode::Element(XmlElement { name: "pp".into(), attributes: [("IX".into(), "1".into())].into(), ..Default::default() }),
                XmlNode::Text("selected later row".into()),
            ],
            ..Default::default()
        });
        // A later row must not affect the first-row properties.
        section.rows.push(Row { index: Some(8), ..Default::default() });
        Ok(())
    })
    .unwrap();
    let st = style(&mut s, json!(1));
    assert_eq!(st["text"]["font"], "Arial");
    assert_eq!(st["text"]["size"], 14.);
    assert_eq!(st["text"]["color"], "#ba7135");
    assert_eq!(st["text"]["align"], 2);
}

#[test]
fn targeted_queries_exclude_unrelated_page_layers_and_connections() {
    let mut s = Session::new();
    rect(&mut s);
    rect(&mut s);
    rect(&mut s);
    run(&mut s, "shape.text", json!({"id":1,"text":"Selected"}));
    run(&mut s, "connector.add", json!({"from":2,"to":3}));
    run(&mut s, "layer.add", json!({"name":"x".repeat(40000)}));
    assert!(s.execute("document.inspect", &json!({})).unwrap_err().to_string().contains("budget"));
    for p in [json!({"page":1,"ids":[1]}), json!({"find":"selected"})] {
        let v = run(&mut s, "document.inspect", p);
        assert!(v["pages"][0].get("layers").is_none());
        assert_eq!(v["pages"][0]["connects"], json!([]));
        assert_eq!(v["pages"][0]["shapes"].as_array().unwrap().len(), 1);
        assert!(v.to_string().len() < 1000);
    }
}

#[test]
fn find_collapses_whitespace_without_erasing_boundaries() {
    let mut s = Session::new();
    for text in ["Oak", " Oak ", "Two\tTrees", ""] {
        let id = rect(&mut s);
        run(&mut s, "shape.text", json!({"id":id,"text":text}));
    }
    let ids = |value: Value| value["pages"][0]["shapes"].as_array().unwrap().iter().map(|shape| shape["id"].clone()).collect::<Vec<_>>();
    assert_eq!(ids(run(&mut s, "document.inspect", json!({"find":" \t\u{b}\u{2028}","sections":["outline"]}))), vec![json!(2), json!(3)]);
    assert_eq!(ids(run(&mut s, "document.inspect", json!({"find":"\t OAK\n","sections":["outline"]}))), vec![json!(2)]);
    assert_eq!(ids(run(&mut s, "document.inspect", json!({"find":"Oak","sections":["outline"]}))), vec![json!(1), json!(2)]);
}

#[test]
fn outline_includes_y_cell_endpoint_connections() {
    let mut s = Session::new();
    rect(&mut s);
    rect(&mut s);
    let connector = run(&mut s, "connector.add", json!({"from":1,"to":2}))["id"].clone();
    s.edit_document(|doc, _| {
        for connection in &mut doc.pages[0].connects {
            connection.from_cell = match connection.from_cell.as_deref() {
                Some("BeginX") => Some("BeginY".into()),
                Some("EndX") => Some("EndY".into()),
                _ => connection.from_cell.clone(),
            };
        }
        Ok(())
    })
    .unwrap();
    let value = run(&mut s, "document.inspect", json!({"sections":["outline"]}));
    assert_eq!(value["pages"][0]["links"], json!([{"id":connector,"begin":1,"end":2}]));
}
