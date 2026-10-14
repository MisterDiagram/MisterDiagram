use diagramcraft_model::{Cell, Sheet, skeleton};
use std::collections::BTreeMap;
#[test]
fn skeleton_parts_styles_and_fresh_connector_identity() {
    let a = skeleton::document(false);
    let b = skeleton::document(true);
    assert_eq!(a.styles.iter().map(|s| s.id).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 6, 7]);
    assert_eq!(
        a.parts.keys().map(String::as_str).collect::<Vec<_>>(),
        vec![
            "docProps/app.xml",
            "docProps/core.xml",
            "docProps/custom.xml",
            "visio/document.xml",
            "visio/masters/master1.xml",
            "visio/masters/masters.xml",
            "visio/pages/page1.xml",
            "visio/pages/pages.xml",
            "visio/windows.xml"
        ]
    );
    assert_eq!(a.root_relationships.len(), 4);
    assert_eq!(a.parts["visio/document.xml"].relationships.len(), 3);
    assert_eq!(a.parts["visio/masters/masters.xml"].relationships.len(), 1);
    assert_eq!(a.parts["visio/pages/pages.xml"].relationships.len(), 1);
    let m = &a.masters[0];
    assert_ne!(m.unique_id, b.masters[0].unique_id);
    assert_ne!(m.unique_id, m.base_id);
    assert_eq!(uuid::Uuid::parse_str(m.unique_id.as_ref().unwrap()).unwrap().get_version_num(), 4);
    assert!(m.icon.is_none());
    assert!(m.shapes[0].sheet.cell("Copyright").is_none());
    assert!(m.shapes[0].sheet.cell("HelpTopic").is_none());
    assert_eq!(m.shapes[0].sheet.cell("PinX").unwrap().formula.as_deref(), Some("GUARD((BeginX+EndX)/2)"));
    assert_eq!(a.pages[0].sheet.cell("PageWidth").unwrap().value.as_deref(), Some("8.5"));
    let metric = b.pages[0].sheet.cell("PageWidth").unwrap().value.as_ref().unwrap().parse::<f64>().unwrap();
    assert!((metric - 210. / 25.4).abs() < 1e-12);
    let custom = a.parts["docProps/custom.xml"].xml.as_ref().unwrap();
    assert!(custom.elements().any(|e| e.attr("name") == Some("BuildNumberCreated") && e.text() == "1074417614"));
    assert!(custom.elements().any(|e| e.attr("name") == Some("IsMetric") && e.text() == "false"));
    assert_eq!(a.parts["docProps/app.xml"].xml.as_ref().unwrap().elements().find(|e| e.name == "Application").unwrap().text(), "Diagramcraft");
}
type Values = BTreeMap<String, (Option<String>, Option<String>, Option<String>)>;
fn insert(out: &mut Values, path: &str, c: &Cell) {
    out.insert(format!("{path}/{}", c.name), (c.value.clone(), c.formula.clone(), c.units.clone()));
}
fn model_cells(sheet: &Sheet) -> Values {
    let mut out = Values::new();
    for c in &sheet.cells {
        insert(&mut out, "", c);
    }
    for s in &sheet.sections {
        let path = format!("/{}/{}", s.name, s.index.map_or(String::new(), |n| n.to_string()));
        for c in &s.cells {
            insert(&mut out, &path, c);
        }
        for r in &s.rows {
            let row = format!("{path}/{}/{}", r.name.as_deref().unwrap_or(""), r.index.map_or(String::new(), |n| n.to_string()));
            for c in &r.cells {
                insert(&mut out, &row, c);
            }
        }
    }
    out
}
fn observed_cells(sheet: &diagramcraft_opc::xml::El) -> Values {
    fn visit(e: &diagramcraft_opc::xml::El, path: &str, out: &mut Values) {
        for child in e.elements() {
            match child.local() {
                "Cell" => {
                    if !matches!(child.attr("N"), Some("Copyright" | "HelpTopic")) {
                        out.insert(
                            format!("{path}/{}", child.attr("N").unwrap()),
                            (child.attr("V").map(str::to_owned), child.attr("F").map(str::to_owned), child.attr("U").map(str::to_owned)),
                        );
                    }
                }
                "Section" | "Row" => visit(child, &format!("{path}/{}/{}", child.attr("N").unwrap_or(""), child.attr("IX").unwrap_or("")), out),
                _ => {}
            }
        }
    }
    let mut out = Values::new();
    visit(sheet, "", &mut out);
    out
}
#[test]
fn skeleton_cell_set_matches_blank_and_probe_observations() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let d = skeleton::document(false);
    let p = diagramcraft_opc::Package::open(&std::fs::read(root.join("blank/blank.vsdx")).unwrap()).unwrap();
    let xml = p.xml("visio/document.xml").unwrap();
    assert_eq!(model_cells(&d.document_sheet), observed_cells(xml.root.child("DocumentSheet").unwrap()));
    let styles = xml.root.child("StyleSheets").unwrap();
    assert_eq!(styles.elements().map(|s| s.u32("ID").unwrap()).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 6]);
    for s in &d.styles {
        if s.id == 7 {
            continue;
        }
        let original = styles.elements().find(|e| e.u32("ID") == Some(s.id)).unwrap();
        let mut expected = observed_cells(original);
        if s.id == 0 {
            for name in ["Copyright", "HelpTopic"] {
                expected.insert(format!("/{name}"), (Some("".into()), None, None));
            }
        }
        assert_eq!(model_cells(&s.sheet), expected, "style {}", s.id);
    }
    let pages = p.xml("visio/pages/pages.xml").unwrap();
    assert_eq!(model_cells(&d.pages[0].sheet), observed_cells(pages.root.child("Page").unwrap().child("PageSheet").unwrap()));
    let p = diagramcraft_opc::Package::open(&std::fs::read(root.join("generated/basic/0001-probe.vsdx")).unwrap()).unwrap();
    let master = p.xml("visio/masters/master1.xml").unwrap();
    let mut expected = observed_cells(master.root.child("Shapes").unwrap().child("Shape").unwrap());
    for name in ["Comment", "ShapeKeywords"] {
        expected.insert(format!("/{name}"), (Some("".into()), Some("Inh".into()), None));
    }
    assert_eq!(model_cells(&d.masters[0].shapes[0].sheet), expected);
    let xml = p.xml("visio/document.xml").unwrap();
    assert_eq!(
        model_cells(&d.styles.iter().find(|s| s.id == 7).unwrap().sheet),
        observed_cells(xml.root.child("StyleSheets").unwrap().elements().find(|s| s.u32("ID") == Some(7)).unwrap())
    );
    let p = diagramcraft_opc::Package::open(&std::fs::read(root.join("blank/blank-rect.vsdx")).unwrap()).unwrap();
    let xml = p.xml("visio/pages/page1.xml").unwrap();
    assert_eq!(
        model_cells(&skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap().sheet),
        observed_cells(xml.root.child("Shapes").unwrap().child("Shape").unwrap())
    );
}
#[test]
fn rectangle_rejects_hostile_coordinates_and_text() {
    for x in [f64::NAN, f64::INFINITY, -f64::INFINITY, 1e7] {
        assert!(skeleton::rectangle(1, x, 0., 1., 1., "").is_err());
    }
    for w in [-1., 0., f64::NAN] {
        assert!(skeleton::rectangle(1, 0., 0., w, 1., "").is_err());
    }
    assert!(skeleton::rectangle(1, 0., 0., 1., 1., &"x".repeat(1024 * 1024 + 1)).is_err());
}

#[test]
fn authored_numbers_empty_style_and_layer_defaults_follow_saved_witness() {
    assert_eq!(skeleton::number_text(297. / 25.4).as_deref(), Some("11.69291338582677"));
    assert_eq!(skeleton::number_text(210. / 25.4).as_deref(), Some("8.267716535433072"));
    assert_eq!(skeleton::number_text(-0.).as_deref(), Some("0"));
    for n in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(skeleton::number_text(n).is_none());
    }
    let d = skeleton::document(false);
    for name in ["Copyright", "HelpTopic"] {
        assert_eq!(d.styles[0].sheet.cell(name).unwrap().value.as_deref(), Some(""));
    }
    for name in ["Comment", "ShapeKeywords"] {
        let c = d.masters[0].shapes[0].sheet.cell(name).unwrap();
        assert_eq!(c.formula.as_deref(), Some("Inh"));
        assert_eq!(c.value.as_deref(), Some(""));
    }
    let layer = skeleton::layer(0, "Visible", true).unwrap();
    assert_eq!(
        layer.cells.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(),
        vec!["Name", "Color", "Status", "Visible", "Print", "Active", "Lock", "Snap", "Glue", "NameUniv", "ColorTrans"]
    );
    assert!(skeleton::layer(16_384, "bad", true).is_err());
    assert!(skeleton::layer(0, &"x".repeat(65_537), true).is_err());
}
