//! Original connector/glue fixtures, independent of vendor assets.
use misterdiagram_model::{Cell, Connect, Row, Section, skeleton};
use misterdiagram_render::scene::{self, Options, SceneBudget};
use misterdiagram_vsdx::evaluate::Scope;
fn document() -> misterdiagram_model::Document {
    let mut d = skeleton::document(false);
    d.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 2., 1., "A").unwrap());
    d.pages[0].shapes.push(skeleton::rectangle(2, 5., 4., 2., 1., "B").unwrap());
    skeleton::glue_dynamic(&mut d.pages[0], 3, 2, [1, 2], [[3., 1.5], [5., 4.5]]).unwrap();
    d
}
fn model(d: &misterdiagram_model::Document) -> misterdiagram_export::model::Page {
    let p = scene::evaluated::page(d, 0, Options::default(), &mut SceneBudget::default()).unwrap();
    misterdiagram_export::model::Page::from_evaluated(p).unwrap()
}
fn slide(d: &misterdiagram_model::Document) -> misterdiagram_opc::xml::El {
    let out = misterdiagram_export::document(d, None, Options { page_canvas: true, ..Default::default() }).unwrap();
    misterdiagram_opc::Package::open(&out.bytes).unwrap().xml("ppt/slides/slide1.xml").unwrap().root
}
#[test]
fn dynamic_glue_uses_ordered_synthetic_sites_and_preset_connector() {
    let d = document();
    let m = model(&d);
    let route = &m.connections.routes[&Scope::PageShape { page: 0, shape: 3 }];
    assert_eq!(route.begin.as_ref().unwrap().index, 4);
    assert_eq!(route.end.as_ref().unwrap().index, 4);
    let root = slide(&d);
    let tree = root.path(&["cSld", "spTree"]).unwrap();
    let connector = tree.child("cxnSp").unwrap();
    assert!(connector.path(&["spPr", "custGeom"]).is_none());
    assert!(connector.path(&["spPr", "prstGeom"]).is_some());
    let start = connector.path(&["nvCxnSpPr", "cNvCxnSpPr", "stCxn"]).unwrap();
    assert_eq!(start.attr("idx"), Some("4"));
    assert_eq!(start.attr("id"), Some("2"));
    let sites = tree.child("sp").unwrap().path(&["spPr", "custGeom", "cxnLst"]).unwrap();
    assert_eq!(sites.children_named("cxn").count(), 5);
    let g = tree.child("sp").unwrap().path(&["spPr", "custGeom", "gdLst"]).unwrap();
    assert_eq!(g.children_named("gd").count(), 10);
    assert!(g.children_named("gd").all(|g| g.attr("fmla").unwrap().starts_with("*/ ")));
}
#[test]
fn explicit_rows_precede_synthetic_sites_in_source_row_order() {
    let mut d = document();
    let c = |name: &str, value: &str| Cell { name: name.into(), value: Some(value.into()), ..Default::default() };
    d.pages[0].shapes[0].sheet.sections.push(Section {
        name: "Connection".into(),
        rows: vec![Row { index: Some(7), cells: vec![c("X", "2"), c("Y", "0.5"), c("DirX", "1"), c("DirY", "0")], ..Default::default() }],
        ..Default::default()
    });
    d.pages[0].connects[0].to_cell = Some("Connections.X8".into());
    let m = model(&d);
    let a = &m.connections.routes[&Scope::PageShape { page: 0, shape: 3 }];
    assert_eq!(a.begin.as_ref().unwrap().index, 4);
    assert_eq!(m.connections.sites[&Scope::PageShape { page: 0, shape: 1 }].len(), 5);
}
#[test]
fn page_qualified_ids_do_not_collide_with_background_shapes() {
    let mut d = document();
    let mut bg = d.pages[0].clone();
    bg.id = 9;
    bg.attributes.insert("Background".into(), "1".into());
    d.pages[0].attributes.insert("BackPage".into(), "9".into());
    d.pages.push(bg);
    let m = model(&d);
    assert_eq!(m.connections.routes.len(), 2);
    assert_ne!(m.connections.ids[&Scope::PageShape { page: 0, shape: 1 }], m.connections.ids[&Scope::PageShape { page: 9, shape: 1 }]);
}
#[test]
fn hostile_glue_target_duplicate_endpoints_and_nonfinite_sites_refuse() {
    let mut d = document();
    d.pages[0].connects.push(Connect { from_sheet: 3, from_cell: Some("BeginX".into()), to_sheet: 999, ..Default::default() });
    assert!(misterdiagram_export::document(&d, None, Options::default()).is_err());
    let mut d = document();
    let duplicate = d.pages[0].connects[0].clone();
    d.pages[0].connects.push(duplicate);
    assert!(misterdiagram_export::document(&d, None, Options::default()).is_err());
    let d = document();
    let mut p = scene::evaluated::page(&d, 0, Options::default(), &mut SceneBudget::default()).unwrap();
    p.shapes[0].connections.push(scene::ConnectionSite {
        name: "hostile".into(),
        position: kurbo::Point::new(f64::NAN, 0.),
        normal: kurbo::Vec2::ZERO,
    });
    assert!(misterdiagram_export::model::Page::from_evaluated(p).is_err());
}
#[test]
fn straight_and_one_to_four_bends_choose_matching_presets() {
    let d = document();
    for vertical in [false, true] {
        for points in [
            vec![(1., 1.), (6., 5.)],
            vec![(1., 1.), (6., 1.), (6., 5.)],
            vec![(1., 1.), (3., 1.), (3., 5.), (6., 5.)],
            vec![(1., 1.), (3., 1.), (3., 3.), (6., 3.), (6., 5.)],
            vec![(1., 1.), (2., 1.), (2., 3.), (4., 3.), (4., 5.), (6., 5.)],
        ] {
            let mut page = scene::evaluated::page(&d, 0, Options::default(), &mut SceneBudget::default()).unwrap();
            page.connects.clear();
            let node = page.shapes.iter_mut().find(|s| s.one_d).unwrap();
            let mut path = kurbo::BezPath::new();
            for (i, (x, y)) in points.iter().enumerate() {
                let p = if vertical { (*y, *x) } else { (*x, *y) };
                if i == 0 {
                    path.move_to(p);
                } else {
                    path.line_to(p);
                }
            }
            node.sections =
                vec![misterdiagram_geom::GeometryPath { path, flags: misterdiagram_geom::GeometryFlags { no_fill: true, ..Default::default() } }];
            let out = misterdiagram_export::render(
                vec![page],
                &[misterdiagram_color::Rgba::BLACK; 12],
                Options { page_canvas: true, ..Default::default() },
            )
            .unwrap();
            let pkg = misterdiagram_opc::Package::open(&out.bytes).unwrap();
            let xml = pkg.xml("ppt/slides/slide1.xml").unwrap();
            let cxn = xml.root.find("cxnSp").unwrap();
            let geom = cxn.path(&["spPr", "prstGeom"]).unwrap();
            let expected = if points.len() == 2 { "straightConnector1".to_string() } else { format!("bentConnector{}", points.len() - 1) };
            assert_eq!(geom.attr("prst"), Some(expected.as_str()));
            assert_eq!(geom.child("avLst").unwrap().children_named("gd").count(), points.len().saturating_sub(3));
            let values: Vec<_> = geom.child("avLst").unwrap().children_named("gd").map(|g| g.attr("fmla").unwrap()).collect();
            let expected: &[&str] = match points.len() {
                4 => &["val 40000"],
                5 => &["val 40000", "val 50000"],
                6 => &["val 20000", "val 50000", "val 60000"],
                _ => &[],
            };
            assert_eq!(values, expected);

            if vertical && points.len() > 2 {
                assert_eq!(cxn.path(&["spPr", "xfrm"]).unwrap().attr("rot"), Some("5400000"));
            }
        }
    }
}
#[test]
fn too_many_bends_are_fitted_and_curves_keep_a_curved_preset() {
    let d = document();
    for curved in [false, true] {
        let mut page = scene::evaluated::page(&d, 0, Options::default(), &mut SceneBudget::default()).unwrap();
        page.connects.clear();
        let n = page.shapes.iter_mut().find(|s| s.one_d).unwrap();
        let mut path = kurbo::BezPath::new();
        path.move_to((1., 1.));
        if curved {
            path.curve_to((3., 1.), (3., 5.), (6., 5.));
        } else {
            for p in [(2., 1.), (2., 2.), (3., 2.), (3., 3.), (4., 3.), (4., 5.), (6., 5.)] {
                path.line_to(p);
            }
        }
        n.sections =
            vec![misterdiagram_geom::GeometryPath { path, flags: misterdiagram_geom::GeometryFlags { no_fill: true, ..Default::default() } }];
        let m = misterdiagram_export::model::Page::from_evaluated(page.clone()).unwrap();
        assert!(m.substitutions.contains(&misterdiagram_export::model::Substitution::Route));
        let out = misterdiagram_export::render(vec![page], &[misterdiagram_color::Rgba::BLACK; 12], Options::default()).unwrap();
        let pkg = misterdiagram_opc::Package::open(&out.bytes).unwrap();
        let xml = pkg.xml("ppt/slides/slide1.xml").unwrap();
        let g = xml.root.find("cxnSp").unwrap().path(&["spPr", "prstGeom"]).unwrap();
        assert_eq!(g.attr("prst"), Some(if curved { "curvedConnector2" } else { "bentConnector5" }));
    }
}

#[test]
fn one_d_glue_targets_get_a_grouped_transparent_site_carrier() {
    let mut d = document();
    // The original target now also has the shared renderer's 1-D discriminator.
    d.pages[0].shapes[0].sheet.cells.push(Cell { name: "BeginX".into(), value: Some("1".into()), ..Default::default() });
    let m = model(&d);
    let scope = Scope::PageShape { page: 0, shape: 1 };
    assert_ne!(m.connections.ids[&scope], m.connections.targets[&scope]);
    let root = slide(&d);
    let tree = root.path(&["cSld", "spTree"]).unwrap();
    let group = tree.child("grpSp").unwrap();
    assert!(group.child("cxnSp").is_some());
    let carrier = group.child("sp").unwrap();
    assert!(carrier.path(&["spPr", "noFill"]).is_some());
    assert!(carrier.path(&["spPr", "ln", "noFill"]).is_some());
    assert!(carrier.path(&["spPr", "custGeom", "cxnLst"]).is_some());
}
#[test]
fn output_route_points_drive_connector_raster() {
    let d = document();
    let mut m = model(&d);
    let options = Options::default();
    let before = misterdiagram_render::raster::render(&m.raster_scene().unwrap(), options).unwrap();
    let route = m.connections.routes.get_mut(&Scope::PageShape { page: 0, shape: 3 }).unwrap();
    route.points.insert(1, kurbo::Point::new(4., 6.));
    let after = misterdiagram_render::raster::render(&m.raster_scene().unwrap(), options).unwrap();
    assert_ne!(before, after);
}
