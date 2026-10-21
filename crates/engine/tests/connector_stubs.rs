//! Native right-angle endpoint and line-jump witnesses for Brief26b (#66).
//! https://learn.microsoft.com/en-us/office/client-developer/visio/shaperoutestyle-cell-shape-layout-section
use misterdiagram_engine::Session;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
fn run(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd} {p}: {e}"))
}
fn close(s: &Session, id: u32, expected: &[(&str, f64)]) {
    let themes = misterdiagram_theme::Themes::from_document(s.document()).unwrap();
    let mut e = Evaluator::new(s.document()).unwrap().with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    let scope = Scope::PageShape { page: s.document().pages[0].id, shape: id };
    for (n, want) in expected {
        let got = e.evaluate(scope, n).numeric().unwrap();
        assert!((got - want).abs() <= 1e-6, "connector {id} {n}: ours={got} Visio={want}");
    }
}
fn drawing(spacing: Option<&str>) -> Session {
    let mut s = Session::new();
    if let Some(n) = spacing {
        run(&mut s, "sheet.set", json!({"scope":"page","name":n,"value":0.4}));
    }
    for (x, y, w, h) in [(0.5, 6., 1., 1.), (4., 6., 1., 1.), (1., 2., 3., 2.)] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":w,"height":h}));
    }
    for from in [1, 2] {
        let c = run(&mut s, "connector.add", json!({"from":from,"to":3}))["id"].as_u64().unwrap();
        run(&mut s, "sheet.set", json!({"id":c,"name":"ShapeRouteStyle","value":1}));
    }
    run(&mut s, "route.reroute", json!({}));
    s
}
#[test]
fn right_angle_enters_top_edge_with_constant_stub_and_native_jump() {
    for spacing in [None, Some("LineToNodeX"), Some("LineToNodeY"), Some("LineToLineX"), Some("LineToLineY")] {
        let s = drawing(spacing);
        close(
            &s,
            4,
            &[
                ("BeginX", 1.),
                ("BeginY", 6.),
                ("EndX", 2.5),
                ("EndY", 4.),
                ("Geometry1.X1", 0.),
                ("Geometry1.Y1", 0.),
                ("Geometry1.X2", 0.),
                ("Geometry1.Y2", -0.1875),
                ("Geometry1.X3", 1.5),
                ("Geometry1.Y3", -0.1875),
                ("Geometry1.X4", 1.5),
                ("Geometry1.Y4", -2.),
            ],
        );
        let radius = if spacing == Some("LineToLineX") { 2. / 15. } else { 1. / 24. };
        close(
            &s,
            5,
            &[
                ("BeginX", 4.5),
                ("BeginY", 6.),
                ("EndX", 1.75),
                ("EndY", 4.),
                ("Geometry1.X1", 0.),
                ("Geometry1.Y1", 0.),
                ("Geometry1.X2", 0.),
                ("Geometry1.Y2", -1.8125),
                ("Geometry1.X3", -2. + radius),
                ("Geometry1.Y3", -1.8125),
                ("Geometry1.X4", -2. - radius),
                ("Geometry1.Y4", -1.8125),
                ("Geometry1.A4", radius),
                ("Geometry1.X5", -2.75),
                ("Geometry1.Y5", -1.8125),
                ("Geometry1.X6", -2.75),
                ("Geometry1.Y6", -2.),
            ],
        );
    }
}

#[test]
fn targeted_reroute_keeps_neighbor_clearance_and_jump() {
    let mut s = drawing(None);
    for _ in 0..3 {
        run(&mut s, "route.reroute", json!({"id":5}));
        close(&s, 5, &[("Geometry1.Y2", -1.8125), ("Geometry1.Y3", -1.8125), ("Geometry1.A4", 1. / 24.)]);
        run(&mut s, "route.reroute", json!({}));
    }
}

#[test]
fn native_other_sides_and_target_smaller_than_stub() {
    for (variant, angle) in
        [("bottom", std::f64::consts::PI), ("left", std::f64::consts::FRAC_PI_2), ("right", 3. * std::f64::consts::FRAC_PI_2), ("tiny", 0.)]
    {
        let mut s = Session::new();
        for (x, y, w, h) in [(0.5, 6., 1., 1.), (4., 6., 1., 1.), (1., 2., 3., 2.)] {
            run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":w,"height":h}));
        }
        for (id, x, y) in [(1, "Width*0.5", "0"), (2, "Width*0.5", "0"), (3, "Width*0.5", "Height"), (3, "Width*0.25", "Height")] {
            run(&mut s, "shape.connection.add", json!({"id":id,"x":x,"y":y}));
        }
        if variant == "tiny" {
            for name in ["Width", "Height"] {
                run(&mut s, "sheet.set", json!({"id":3,"name":name,"value":0.1}));
            }
        } else {
            for (id, x, y) in [(1, 1., 6.5), (2, 4.5, 6.5), (3, 2.5, 3.)] {
                let (x, y) = (x - 3., y - 4.);
                run(&mut s, "shape.move", json!({"id":id,"x":3.+x*angle.cos()-y*angle.sin(),"y":4.+x*angle.sin()+y*angle.cos()}));
                run(&mut s, "shape.rotate", json!({"id":id,"angle":angle}));
            }
        }
        for (from, point) in [(1, "Connections.X1"), (2, "Connections.X2")] {
            let id = run(&mut s, "connector.add", json!({"from":from,"to":3,"from_point":"Connections.X1","to_point":point}))["id"].as_u64().unwrap();
            run(&mut s, "sheet.set", json!({"id":id,"name":"ShapeRouteStyle","value":1}));
        }
        run(&mut s, "route.reroute", json!({}));
        match variant {
            "bottom" => {
                close(
                    &s,
                    4,
                    &[
                        ("BeginX", 4.9999999999999964),
                        ("BeginY", 1.9999999999999996),
                        ("EndX", 3.500000000000007),
                        ("EndY", 4.0),
                        ("Geometry1.X1", 0.0),
                        ("Geometry1.Y1", 0.0),
                        ("Geometry1.X2", 0.0),
                        ("Geometry1.Y2", 0.1875),
                        ("Geometry1.X3", -1.4999999999999893),
                        ("Geometry1.Y3", 0.1875),
                        ("Geometry1.X4", -1.4999999999999893),
                        ("Geometry1.Y4", 2.0000000000000004),
                    ],
                );
                close(
                    &s,
                    5,
                    &[
                        ("BeginX", 1.4999999999999962),
                        ("BeginY", 2.0),
                        ("EndX", 4.250000000000007),
                        ("EndY", 4.000000000000005),
                        ("Geometry1.X1", 0.0),
                        ("Geometry1.Y1", 0.0),
                        ("Geometry1.X2", 0.0),
                        ("Geometry1.Y2", 1.8125),
                        ("Geometry1.X3", 1.958333333333344),
                        ("Geometry1.Y3", 1.8124999999999996),
                        ("Geometry1.X4", 2.0416666666666776),
                        ("Geometry1.Y4", 1.8125),
                        ("Geometry1.A4", -0.04166666666666685),
                        ("Geometry1.X5", 2.7500000000000107),
                        ("Geometry1.Y5", 1.8125),
                        ("Geometry1.X6", 2.7500000000000107),
                        ("Geometry1.Y6", 2.0000000000000053),
                    ],
                );
            }
            "left" => {
                close(
                    &s,
                    4,
                    &[
                        ("BeginX", 1.0),
                        ("BeginY", 2.0000000000000018),
                        ("EndX", 3.0),
                        ("EndY", 3.4999999999999964),
                        ("Geometry1.X1", 0.0),
                        ("Geometry1.Y1", 0.0),
                        ("Geometry1.X2", 0.1875),
                        ("Geometry1.Y2", 0.0),
                        ("Geometry1.X3", 0.1875),
                        ("Geometry1.Y3", 1.4999999999999947),
                        ("Geometry1.X4", 2.0),
                        ("Geometry1.Y4", 1.4999999999999947),
                    ],
                );
                close(
                    &s,
                    5,
                    &[
                        ("BeginX", 1.0),
                        ("BeginY", 5.500000000000002),
                        ("EndX", 3.0000000000000027),
                        ("EndY", 2.7499999999999964),
                        ("Geometry1.X1", 0.0),
                        ("Geometry1.Y1", 0.0),
                        ("Geometry1.X2", 0.1875),
                        ("Geometry1.Y2", 0.0),
                        ("Geometry1.X3", 0.1875),
                        ("Geometry1.Y3", -2.7500000000000053),
                        ("Geometry1.X4", 2.0000000000000027),
                        ("Geometry1.Y4", -2.7500000000000053),
                    ],
                );
            }
            "right" => {
                close(
                    &s,
                    4,
                    &[
                        ("BeginX", 5.0),
                        ("BeginY", 5.999999999999994),
                        ("EndX", 3.0),
                        ("EndY", 4.500000000000011),
                        ("Geometry1.X1", 0.0),
                        ("Geometry1.Y1", 0.0),
                        ("Geometry1.X2", -0.1875),
                        ("Geometry1.Y2", 0.0),
                        ("Geometry1.X3", -0.1875),
                        ("Geometry1.Y3", -1.4999999999999831),
                        ("Geometry1.X4", -2.0),
                        ("Geometry1.Y4", -1.4999999999999831),
                    ],
                );
                close(
                    &s,
                    5,
                    &[
                        ("BeginX", 5.0),
                        ("BeginY", 2.4999999999999942),
                        ("EndX", 2.999999999999992),
                        ("EndY", 5.250000000000011),
                        ("Geometry1.X1", 0.0),
                        ("Geometry1.Y1", 0.0),
                        ("Geometry1.X2", -0.1875),
                        ("Geometry1.Y2", 0.0),
                        ("Geometry1.X3", -0.1875),
                        ("Geometry1.Y3", 2.7500000000000164),
                        ("Geometry1.X4", -2.000000000000008),
                        ("Geometry1.Y4", 2.7500000000000164),
                    ],
                );
            }
            "tiny" => {
                close(
                    &s,
                    4,
                    &[
                        ("BeginX", 1.0),
                        ("BeginY", 6.0),
                        ("EndX", 2.5),
                        ("EndY", 3.0500000000000003),
                        ("Geometry1.X1", 0.0),
                        ("Geometry1.Y1", 0.0),
                        ("Geometry1.X2", 0.0),
                        ("Geometry1.Y2", -0.1875),
                        ("Geometry1.X3", 1.5),
                        ("Geometry1.Y3", -0.1875),
                        ("Geometry1.X4", 1.5),
                        ("Geometry1.Y4", -2.9499999999999997),
                    ],
                );
                close(
                    &s,
                    5,
                    &[
                        ("BeginX", 4.5),
                        ("BeginY", 6.0),
                        ("EndX", 2.475),
                        ("EndY", 3.0500000000000003),
                        ("Geometry1.X1", 0.0),
                        ("Geometry1.Y1", 0.0),
                        ("Geometry1.X2", 0.0),
                        ("Geometry1.Y2", -0.1875),
                        ("Geometry1.X3", -2.025),
                        ("Geometry1.Y3", -0.1875),
                        ("Geometry1.X4", -2.025),
                        ("Geometry1.Y4", -2.9499999999999997),
                    ],
                );
            }
            _ => panic!("unexpected test variant"),
        }
    }
}
