//! Contributor-original master scale regressions from the Brief 31b COM matrix.
use crate::{Session, edit::*};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::json;
use std::sync::Arc;

fn session(scale: f64, page: f64, kind: f64, line: bool) -> Session {
    let mut s = Session::new();
    let doc = Arc::make_mut(&mut s.document);
    let master = &mut doc.masters[0];
    for (n, v) in [("PageScale", 1.), ("DrawingScale", scale), ("DrawingScaleType", 3.)] {
        set_number(&mut master.sheet, n, v).unwrap();
    }
    let mut shape = misterdiagram_model::skeleton::rectangle(5, 0., 0., 10., 4., "").unwrap();
    if line {
        for (n, v) in [("BeginX", 0.), ("BeginY", 0.), ("EndX", 10.), ("EndY", 0.), ("PinY", 0.), ("Height", 0.)] {
            set_number(&mut shape.sheet, n, v).unwrap();
        }
        set(&mut shape.sheet, "Width", "10", Some("SQRT((EndX-BeginX)^2+(EndY-BeginY)^2)")).unwrap();
    }
    set_number(&mut shape.sheet, "LineWeight", 1.25 / 72.).unwrap();
    master.shapes = vec![shape].into();
    for (n, v) in [("PageScale", 1.), ("DrawingScale", page), ("DrawingScaleType", kind)] {
        set_number(&mut doc.pages[0].sheet, n, v).unwrap();
    }
    s
}
fn value(s: &Session, name: &str) -> f64 {
    Evaluator::new(s.document()).unwrap().evaluate(Scope::PageShape { page: 0, shape: 1 }, name).numeric().unwrap()
}
fn near(s: &Session, name: &str, expected: f64) {
    let actual = value(s, name);
    assert!((actual - expected).abs() <= 1e-6, "{name}: actual {actual}, expected {expected}");
}
#[test]
fn scale_matrix_and_strict_eightfold_boundary() {
    // Native Visio Width results on the NoScale destination. The other four
    // destinations keep Width=10, including Custom 1:1 and Custom 1:100.
    let witnessed = [
        (0.1, 100.),
        (0.125, 10.),
        (0.2, 10.),
        (0.25, 10.),
        (0.5, 10.),
        (1., 10.),
        (2., 10.),
        (4., 10.),
        (5., 10.),
        (8., 10.),
        (10., 1.),
        (16., 0.625),
        (0.124999, 80.00064000512004),
        (0.125001, 10.),
        (7.999999, 10.),
        (8.000001, 1.2499998437500195),
        (9., 10. / 9.),
    ];
    for (scale, unscaled_width) in witnessed {
        for (page, kind) in [(1., 0.), (1., 3.), (4., 3.), (10., 3.), (100., 3.)] {
            for line in [false, true] {
                let mut s = session(scale, page, kind, line);
                s.execute("master.drop", &json!({"master":2,"x":5,"y":5})).unwrap();
                let width = if kind == 0. { unscaled_width } else { 10. };
                near(&s, "Width", width);
                near(&s, "Height", if line { 0. } else { width * 0.4 });
                near(&s, "LocPinX", width * 0.5);
                near(&s, "LocPinY", if line { 0. } else { width * 0.2 });
                near(&s, "LineWeight", 1.25 / 72.);
                near(&s, "Geometry1.X2", width);
                near(&s, "DropOnPageScale", if width == 10. { 1. } else { width / 10. });
                if line {
                    near(&s, "BeginX", 5. - width / 2.);
                    near(&s, "EndX", 5. + width / 2.);
                    near(&s, "BeginY", 5.);
                    near(&s, "EndY", 5.);
                }
            }
        }
    }
}
#[test]
fn explicit_dimensions_win_and_metric_scale_matches_inches() {
    for line in [false, true] {
        let mut s = session(10., 1., 0., line);
        let sheet = &mut Arc::make_mut(&mut s.document).masters[0].sheet;
        set_number(sheet, "PageScale", 1. / 2.54).unwrap();
        set_number(sheet, "DrawingScale", 10. / 2.54).unwrap();
        s.execute("master.drop", &json!({"master":2,"x":5,"y":5,"width":3,"height":2})).unwrap();
        near(&s, "Width", 3.);
        near(&s, "Height", 2.);
        near(&s, "DropOnPageScale", 0.1);
        if line {
            near(&s, "BeginX", 3.5);
            near(&s, "EndX", 6.5);
        }
    }
}
#[test]
fn unchanged_master_keeps_exact_instance_bytes() {
    let mut s = session(1., 1., 0., false);
    // Expected original drop representation: only pins override the master.
    let mut expected = s.document().clone();
    let mut shape = misterdiagram_model::Shape {
        id: 1,
        master: Some(2),
        shape_type: Some("Shape".into()),
        name: expected.masters[0].name.clone(),
        name_u: expected.masters[0].name_u.clone(),
        ..Default::default()
    };
    set_number(&mut shape.sheet, "PinX", 5.).unwrap();
    set_number(&mut shape.sheet, "PinY", 5.).unwrap();
    expected.pages[0].shapes.push(shape);
    s.execute("master.drop", &json!({"master":2,"x":5,"y":5})).unwrap();
    let expected = misterdiagram_vsdx::write(&expected).unwrap();
    let actual = misterdiagram_vsdx::write(s.document()).unwrap();
    assert!(actual.bytes == expected.bytes, "1:1 master package bytes changed");
}
#[test]
fn discovery_reports_page_dependent_sizes_and_hostile_scale_is_atomic() {
    let mut s = session(10., 1., 0., false);
    for command in ["stencil.masters", "stencil.search"] {
        let params = if command == "stencil.search" { json!({"query":"Dynamic"}) } else { json!({}) };
        let reply = s.execute(command, &params).unwrap();
        assert_eq!(reply["masters"][0]["scale"]["drawing_scale"], 10.);
        assert_eq!(reply["masters"][0]["drop_size"]["width"], 1.);
    }
    Arc::make_mut(&mut s.document).pages.clear();
    let reply = s.execute("stencil.masters", &json!({})).unwrap();
    assert_eq!(reply["masters"][0]["drop_size"], "page-dependent");
    for invalid in ["0", "-1", "NaN", "1e309", "1e-300"] {
        let mut s = session(10., 1., 0., false);
        set(&mut Arc::make_mut(&mut s.document).masters[0].sheet, "DrawingScale", invalid, None).unwrap();
        let before = s.document.clone();
        assert!(s.execute("master.drop", &json!({"master":2})).is_err(), "{invalid}");
        assert_eq!(s.document().pages[0].shapes.len(), 0);
        assert_eq!(s.document().masters, before.masters);
    }
}

#[test]
fn page_scale_command_selects_measured_destination_and_back() {
    let mut s = session(10., 1., 0., false);
    s.execute("page.scale", &json!({"page_scale":1,"drawing_scale":10})).unwrap();
    s.execute("master.drop", &json!({"master":2})).unwrap();
    near(&s, "Width", 10.);
    s.execute("page.scale", &json!({"page_scale":1,"drawing_scale":1})).unwrap();
    let reply = s.execute("stencil.masters", &json!({})).unwrap();
    assert_eq!(reply["masters"][0]["drop_size"]["width"], 1.);
}

#[test]
fn literal_line_width_scales_and_explicit_rectangle_width_skips_bad_formula() {
    let mut s = session(10., 1., 0., true);
    set_number(&mut Arc::make_mut(&mut s.document).masters[0].shapes[0].sheet, "Width", 10.).unwrap();
    s.execute("master.drop", &json!({"master":2,"x":5,"y":5})).unwrap();
    near(&s, "Width", 1.);
    near(&s, "BeginX", 4.5);
    near(&s, "EndX", 5.5);
    let mut s = session(10., 1., 0., false);
    set(&mut Arc::make_mut(&mut s.document).masters[0].shapes[0].sheet, "Width", "10", Some("UNSUPPORTED()")).unwrap();
    s.execute("master.drop", &json!({"master":2,"width":3,"height":2})).unwrap();
    near(&s, "Width", 3.);
    near(&s, "Height", 2.);
}
#[test]
fn text_dimensions_use_same_hosts_as_discovery() {
    let mut s = session(10., 1., 0., false);
    let master = &mut Arc::make_mut(&mut s.document).masters[0];
    master.shapes[0] = misterdiagram_model::skeleton::rectangle(5, 0., 0., 10., 4., "Original text").unwrap();
    set(&mut master.shapes[0].sheet, "Width", "10", Some("TEXTWIDTH(TheText)")).unwrap();
    let discovered = s.execute("stencil.masters", &json!({})).unwrap()["masters"][0]["drop_size"]["width"].as_f64().unwrap();
    s.execute("master.drop", &json!({"master":2})).unwrap();
    near(&s, "Width", discovered);
}
#[test]
fn noscale_source_control_matrix() {
    // Native control: source NoScale, destinations NoScale 1:1 and Custom
    // 1:1, 1:4, 1:10, 1:100. These are measured outputs, not computed expectations.
    for (ratio, widths) in [(0.1, [10., 100., 400., 1000., 10000.]), (1., [10., 10., 10., 100., 1000.]), (10., [10., 1., 10., 10., 100.])] {
        for ((page, kind), width) in [(1., 0.), (1., 3.), (4., 3.), (10., 3.), (100., 3.)].into_iter().zip(widths) {
            for line in [false, true] {
                let mut s = session(ratio, page, kind, line);
                set_number(&mut Arc::make_mut(&mut s.document).masters[0].sheet, "DrawingScaleType", 0.).unwrap();
                s.execute("master.drop", &json!({"master":2,"x":5,"y":5})).unwrap();
                near(&s, "Width", width);
                near(&s, "Height", if line { 0. } else { width * 0.4 });
                near(&s, "DropOnPageScale", width / 10.);
                if line {
                    near(&s, "BeginX", 5. - width / 2.);
                    near(&s, "EndX", 5. + width / 2.);
                }
            }
        }
    }
}

#[test]
fn tenfold_master_drop_witness() {
    for line in [false, true] {
        let mut s = session(10., 1., 0., line);
        s.execute("master.drop", &json!({"master":2,"x":5,"y":5})).unwrap();
        near(&s, "Width", 1.);
        near(&s, "DropOnPageScale", 0.1);
    }
}

#[test]
fn omitted_master_pagesheet_keeps_default_drop_and_discovery() {
    let mut s = session(10., 1., 0., false);
    Arc::make_mut(&mut s.document).masters[0].sheet = Default::default();
    let reply = s.execute("stencil.masters", &json!({})).unwrap();
    assert_eq!(reply["masters"][0]["scale"], json!({"page_scale":1.,"drawing_scale":1.,"drawing_scale_type":0.}));
    assert_eq!(reply["masters"][0]["drop_size"]["width"], 10.);
    s.execute("master.drop", &json!({"master":2,"x":5,"y":5})).unwrap();
    near(&s, "Width", 10.);
    near(&s, "Height", 4.);
    near(&s, "DropOnPageScale", 1.);
}
