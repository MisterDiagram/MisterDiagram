//! Original synthetic presentation package and hostile-coordinate coverage.
use misterdiagram_export::{colors, render};
use misterdiagram_model::{Document, skeleton};
use misterdiagram_opc::Package;
use misterdiagram_render::scene::{self, Options, SceneBudget};

fn original() -> Document {
    let mut doc = skeleton::document(false);
    let mut back = skeleton::page(11, "Original background", false);
    back.attributes.insert("Background".into(), "1".into());
    back.shapes.push(skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap());
    let mut front = skeleton::page(12, "Original foreground", false);
    front.attributes.insert("BackPage".into(), "11".into());
    front.shapes.push(skeleton::rectangle(1, 4., 4., 2., 1., "").unwrap());
    let mut guide = skeleton::rectangle(2, 6., 6., 1., 1., "").unwrap();
    guide.shape_type = Some("Guide".into());
    front.shapes.push(guide);
    let mut deleted = skeleton::rectangle(3, 7., 7., 1., 1., "").unwrap();
    deleted.sheet.attributes.insert("Del".into(), "1".into());
    front.shapes.push(deleted);
    let cell = |name: &str, value: &str| misterdiagram_model::Cell { name: name.into(), value: Some(value.into()), ..Default::default() };
    front.sheet.sections.push(misterdiagram_model::Section {
        name: "Layer".into(),
        rows: vec![
            misterdiagram_model::Row { index: Some(0), cells: vec![cell("Visible", "0"), cell("Print", "1")], ..Default::default() },
            misterdiagram_model::Row { index: Some(1), cells: vec![cell("Visible", "1"), cell("Print", "0")], ..Default::default() },
        ],
        ..Default::default()
    });
    let mut hidden = skeleton::rectangle(4, 2., 2., 1., 1., "").unwrap();
    hidden.sheet.cells.push(cell("LayerMember", "0"));
    front.shapes.push(hidden);
    let mut nonprinting = skeleton::rectangle(5, 6., 4., 1., 1., "").unwrap();
    nonprinting.sheet.cells.push(cell("LayerMember", "1"));
    front.shapes.push(nonprinting);
    let mut other = skeleton::page(13, "Second foreground", false);
    for cell in &mut other.sheet.cells {
        if cell.name == "PageWidth" {
            cell.value = Some("17".into());
        }
        if cell.name == "PageHeight" {
            cell.value = Some("22".into());
        }
    }
    doc.pages = vec![back, front, other];
    doc
}

#[test]
fn foreground_order_background_content_and_package_relationships() {
    let doc = original();
    let opts = Options { page_canvas: true, ..Default::default() };
    let scenes = scene::foreground_pages_budgeted(&doc, opts, &mut SceneBudget::default()).unwrap();
    assert_eq!(scenes.len(), 2);
    let output = render(&scenes, &colors(&doc, 1).unwrap(), opts).unwrap();
    assert!(output.warnings.iter().any(|s| s.contains("page 2: scaled by 0.500000000")));
    let pkg = Package::open(&output.bytes).unwrap();
    let pres = pkg.xml("ppt/presentation.xml").unwrap();
    assert_eq!(pres.root.child("sldIdLst").unwrap().elements().count(), 2);
    let size = pres.root.child("sldSz").unwrap();
    assert_eq!(size.attr("cx"), Some("7772400"));
    assert_eq!(size.attr("cy"), Some("10058400"));
    for n in 1..=2 {
        let part = format!("ppt/slides/slide{n}.xml");
        let rels = pkg.rels(&part);
        let layout = rels.first_of("slideLayout").unwrap();
        assert!(pkg.get(&layout.target).is_some());
        let layout_rels = pkg.rels(&layout.target);
        let master = layout_rels.first_of("slideMaster").unwrap();
        assert!(pkg.get(&master.target).is_some());
        let master_rels = pkg.rels(&master.target);
        assert!(pkg.get(&master_rels.first_of("theme").unwrap().target).is_some());
    }
    let slide = pkg.xml("ppt/slides/slide1.xml").unwrap();
    let tree = slide.root.path(&["cSld", "spTree"]).unwrap();
    let paths: Vec<_> = tree.children_named("sp").collect();
    // Each rectangle's fill then stroke, background before foreground. The
    // repeated page-local shape ID does not become a duplicate slide ID.
    assert_eq!(paths.len(), 6, "visible Print=0 content remains under print:false; hidden, guides and tombstones do not");
    let left = |i: usize| paths[i].path(&["spPr", "xfrm", "off"]).unwrap().i64("x").unwrap();
    assert!(left(0) < left(2));
    let ids: std::collections::BTreeSet<_> = paths.iter().map(|s| s.path(&["nvSpPr", "cNvPr"]).unwrap().attr("id").unwrap()).collect();
    assert_eq!(ids.len(), paths.len());
    assert!(slide.root.find("style").is_none());
    assert!(slide.root.find("schemeClr").is_none());
    let app = pkg.xml("docProps/app.xml").unwrap();
    assert_eq!(app.root.child("Application").unwrap().text(), format!("MisterDiagram {}", env!("CARGO_PKG_VERSION")));
    assert!(pkg.xml("ppt/slideLayouts/slideLayout1.xml").unwrap().root.find("ph").is_none());
    assert!(pkg.xml("ppt/slideMasters/slideMaster1.xml").unwrap().root.find("ph").is_none());
}

#[test]
fn malformed_sizes_paths_and_page_counts_refuse_without_panicking() {
    let doc = original();
    let opts = Options::default();
    let colors = colors(&doc, 1).unwrap();
    let mut page = scene::page(&doc, 1, opts).unwrap();
    assert!(render(&[], &colors, opts).is_err());
    for size in [(0., 8.), (f64::NAN, 8.), (f64::INFINITY, 8.), (57., 8.)] {
        page.page_size = size;
        assert!(render(std::slice::from_ref(&page), &colors, opts).is_err());
    }
    page.page_size = (8.5, 11.);
    let mut normal_path = kurbo::BezPath::new();
    normal_path.move_to((0., 0.));
    normal_path.line_to((1., 1.));
    page.ops = vec![scene::Op::Stroke { path: normal_path, stroke: kurbo::Stroke::new(23.), color: misterdiagram_color::Rgba::BLACK }];
    assert!(render(std::slice::from_ref(&page), &colors, opts).err().unwrap().contains("PPTX line width limit"));
    let mut path = kurbo::BezPath::new();
    path.move_to((0., 0.));
    path.line_to((1e100, 0.));
    page.ops = vec![scene::Op::Path { even_odd: false, path, paint: scene::Paint::Solid(misterdiagram_color::Rgba::BLACK) }];
    assert!(render(&[page], &colors, opts).is_err());
}
