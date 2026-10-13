use diagramcraft_model::{skeleton, *};
use diagramcraft_vsdx::{SaveKind, import, write_as};
use std::path::Path;
fn cell(n: &str, v: &str, f: Option<&str>) -> Cell {
    Cell { name: n.into(), value: Some(v.into()), formula: f.map(str::to_owned), ..Default::default() }
}
fn put(s: &mut Sheet, c: Cell) {
    s.cells.retain(|v| v.name != c.name);
    s.cells.push(c);
}
fn save(root: &Path, name: &str, d: &Document, kind: SaveKind) -> Document {
    let bytes = write_as(d, kind).unwrap().bytes;
    let reread = import(&bytes).unwrap();
    assert!(!reread.parts.keys().any(|n| n.starts_with("docProps/thumbnail.")));
    assert!(!reread.root_relationships.iter().any(|r| r.type_uri.ends_with("/thumbnail")));
    if root.join("corpus").exists() {
        let out = root.join("corpus/_state/m3/scratch");
        std::fs::create_dir_all(&out).unwrap();
        std::fs::write(out.join(name), bytes).unwrap();
    }
    reread
}
/// Five P3 acceptance documents, constructed through typed model records.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/layer-section
#[test]
fn five_documents_build_and_reread() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut d = skeleton::document(false);
    d.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 2., 1., "Authored rectangle").unwrap());
    let a = save(&root, "scratch__rectangle-text.vsdx", &d, SaveKind::Drawing);
    assert_eq!(a.pages[0].shapes[0].text.as_ref().unwrap().text(), "Authored rectangle\n");
    assert!(!a.root_relationships.iter().any(|r| r.type_uri.ends_with("/thumbnail")));
    d.pages[0].shapes.push(skeleton::rectangle(2, 4., 1., 2., 1., "Destination").unwrap());
    // Right edge of the first rectangle, left edge of the destination, in page inches.
    skeleton::glue_dynamic(&mut d.pages[0], 3, 2, [1, 2], [[3., 1.5], [4., 1.5]]).unwrap();
    let a = save(&root, "scratch__dynamic-glue.vsdx", &d, SaveKind::Drawing);
    assert_eq!(a.pages[0].connects.len(), 2);
    for (a, b) in a.pages[0].connects.iter().zip(&d.pages[0].connects) {
        assert_eq!(
            (a.from_sheet, &a.from_cell, a.from_part, a.to_sheet, &a.to_cell, a.to_part),
            (b.from_sheet, &b.from_cell, b.from_part, b.to_sheet, &b.to_cell, b.to_part)
        );
    }
    assert_eq!(a.pages[0].shapes[2].master, Some(2));
    assert!(!a.parts.keys().any(|n| n.starts_with("docProps/thumbnail.")));
    let glue = &a.pages[0].shapes[2].sheet;
    for (name, formula, value) in [
        ("BeginX", "_WALKGLUE(BegTrigger,EndTrigger,WalkPreference)", "3"),
        ("BeginY", "_WALKGLUE(BegTrigger,EndTrigger,WalkPreference)", "1.5"),
        ("EndX", "_WALKGLUE(EndTrigger,BegTrigger,WalkPreference)", "4"),
        ("EndY", "_WALKGLUE(EndTrigger,BegTrigger,WalkPreference)", "1.5"),
        ("BegTrigger", "_XFTRIGGER(Sheet.1!EventXFMod)", "2"),
        ("EndTrigger", "_XFTRIGGER(Sheet.2!EventXFMod)", "2"),
    ] {
        let c = glue.cell(name).unwrap();
        assert_eq!(c.formula.as_deref(), Some(formula));
        assert_eq!(c.value.as_deref(), Some(value));
    }
    for target in &a.pages[0].shapes[..2] {
        assert_eq!(target.sheet.cell("ObjType").unwrap().value.as_deref(), Some("1"));
    }
    let mut d = skeleton::document(true);
    let mut back = skeleton::page(1, "Background", true);
    back.attributes.insert("Background".into(), "1".into());
    back.shapes.push(skeleton::rectangle(1, 0.5, 0.5, 5., 3., "Background").unwrap());
    d.pages[0].attributes.insert("BackPage".into(), "1".into());
    d.pages[0].sheet.sections.push(Section {
        name: "Layer".into(),
        rows: (0..2)
            .map(|i| Row {
                index: Some(i),
                cells: vec![
                    cell("Name", if i == 0 { "Visible" } else { "Hidden" }, None),
                    cell("Visible", if i == 0 { "1" } else { "0" }, None),
                    cell("Print", "1", None),
                    cell("Active", "0", None),
                    cell("Lock", "0", None),
                ],
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    });
    for i in 0..2 {
        let mut shape = skeleton::rectangle(i + 1, 1. + f64::from(i), 1., 1., 1., "Layer").unwrap();
        shape.sheet.cells.push(cell("LayerMember", &i.to_string(), None));
        d.pages[0].shapes.push(shape);
    }
    d.pages.push(back);
    let a = save(&root, "scratch__metric-background-layers.vsdx", &d, SaveKind::Drawing);
    assert_eq!(a.pages.len(), 2);
    assert_eq!(a.pages[0].attributes.get("BackPage").unwrap(), "1");
    assert_eq!(a.pages[0].sheet.sections.iter().find(|s| s.name == "Layer").unwrap().rows.len(), 2);
    for (i, shape) in a.pages[0].shapes.iter().enumerate() {
        assert_eq!(shape.sheet.cell("LayerMember").unwrap().value.as_deref(), Some(i.to_string().as_str()));
    }
    assert_eq!(
        a.pages[0].sheet.sections.iter().find(|s| s.name == "Layer").unwrap().rows[1]
            .cells
            .iter()
            .find(|c| c.name == "Visible")
            .unwrap()
            .value
            .as_deref(),
        Some("0")
    );
    let mut d = skeleton::document(false);
    d.masters.clear();
    d.parts.remove("visio/masters/master1.xml");
    for i in 0..3 {
        let mut m = skeleton::dynamic_connector();
        m.id = i + 1;
        m.name = format!("Authored {}", i + 1);
        m.name_u = m.name.clone();
        m.part = format!("visio/masters/master{}.xml", i + 1);
        m.shapes = vec![skeleton::rectangle(1, 0., 0., 1. + f64::from(i) * 0.5, 1., &m.name).unwrap()];
        d.masters.push(m);
    }
    let a = save(&root, "scratch__three-masters.vssx", &d, SaveKind::Stencil);
    assert_eq!(a.masters.len(), 3);
    assert!(a.masters.iter().all(|m| m.icon.is_none()));
    let witness = root.join("corpus/ms-shipped/1033/BASIC_U.vssx");
    if !root.join("corpus").exists() {
        eprintln!("skip: private corpus/ absent, BASIC_U dropped-master case");
        return;
    }
    let source = diagramcraft_vsdx::read(witness).unwrap();
    let mut d = skeleton::document(false);
    let mut m = source.masters.iter().find(|m| m.name_u == "Rectangle").unwrap().clone();
    m.id = 3;
    m.part = "visio/masters/master2.xml".into();
    let source_part = source.parts[&source.masters.iter().find(|m| m.name_u == "Rectangle").unwrap().part].clone();
    d.parts.insert(m.part.clone(), source_part);
    d.masters.push(m);
    let mut shape = Shape { id: 1, master: Some(3), shape_type: Some("Shape".into()), ..Default::default() };
    for (n, v) in [("PinX", "3"), ("PinY", "3"), ("Width", "2"), ("Height", "1")] {
        put(&mut shape.sheet, cell(n, v, None));
    }
    d.pages[0].shapes.push(shape);
    let a = save(&root, "scratch__dropped-master.vsdx", &d, SaveKind::Drawing);
    assert_eq!(a.pages[0].shapes[0].master, Some(3));
    assert_eq!(a.masters.iter().find(|m| m.id == 3).unwrap().name_u, "Rectangle");
}

#[test]
fn dynamic_glue_rejects_hostile_requests_without_mutation() {
    let mut d = skeleton::document(false);
    d.pages[0].shapes.push(skeleton::rectangle(1, 0., 0., 1., 1., "").unwrap());
    d.pages[0].shapes.push(skeleton::rectangle(2, 2., 0., 1., 1., "").unwrap());
    let before = d.pages[0].clone();
    for x in [f64::NAN, f64::INFINITY, -f64::INFINITY, 1e7] {
        assert!(skeleton::glue_dynamic(&mut d.pages[0], 3, 2, [1, 2], [[x, 0.5], [2., 0.5]]).is_err());
        assert_eq!(d.pages[0], before);
    }
    assert!(skeleton::glue_dynamic(&mut d.pages[0], 3, 2, [1, 99], [[1., 0.5], [2., 0.5]]).is_err());
    assert!(skeleton::glue_dynamic(&mut d.pages[0], 1, 2, [1, 2], [[1., 0.5], [2., 0.5]]).is_err());
    assert_eq!(d.pages[0], before);
}

#[test]
fn dynamic_glue_rejects_nested_ids_and_writer_boundaries() {
    let mut d = skeleton::document(false);
    for id in 1..=2 {
        d.pages[0].shapes.push(skeleton::rectangle(id, f64::from(id), 0., 1., 1., "").unwrap());
    }
    d.pages[0].shapes[0].shapes.push(Shape { id: 3, ..Default::default() });
    let before = d.pages[0].clone();
    assert!(skeleton::glue_dynamic(&mut d.pages[0], 3, 2, [1, 2], [[1., 0.5], [2., 0.5]]).is_err());
    assert_eq!(d.pages[0], before);
    d.pages[0].shapes[0].shapes = (3..=100_000).map(|id| Shape { id, ..Default::default() }).collect();
    assert!(skeleton::glue_dynamic(&mut d.pages[0], 100_001, 2, [1, 2], [[1., 0.5], [2., 0.5]]).is_err());
    assert_eq!(d.pages[0].shapes.len(), 2);
    assert_eq!(d.pages[0].shapes[0].shapes.len(), 99_998);
    assert!(d.pages[0].connects.is_empty());
    d.pages[0].shapes[0].shapes.clear();
    d.pages[0].connects = vec![Connect::default(); 99_999];
    assert!(skeleton::glue_dynamic(&mut d.pages[0], 3, 2, [1, 2], [[1., 0.5], [2., 0.5]]).is_err());
    assert_eq!(d.pages[0].connects.len(), 99_999);
    assert!(d.pages[0].shapes[0].sheet.cell("ObjType").is_none());
    d.pages[0].connects.pop();
    skeleton::glue_dynamic(&mut d.pages[0], 3, 2, [1, 2], [[1., 0.5], [2., 0.5]]).unwrap();
    assert_eq!(d.pages[0].connects.len(), 100_000);
}
