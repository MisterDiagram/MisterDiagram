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

#[test]
fn native_mixed_points_retain_source_walk_and_split_target_intervals() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":3,"y":5.5,"width":2,"height":1}));
    for x in [0.25, 0.5, 0.75] {
        run(&mut s, "shape.connection.add", json!({"id":1,"x":format!("Width*{x}"),"y":"Height*0"}));
    }
    let mut targets = vec![(1, 4., 6.)];
    for (index, (x, y)) in [(1., 2.), (5., 2.), (3., 0.2), (5., 0.2)].into_iter().enumerate() {
        let b = run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":2,"height":1}))["id"].as_u64().unwrap();
        targets.push((b, x + 1., y + 0.5));
        let c = run(&mut s, "master.drop", json!({"master":"Dynamic connector"}))["id"].as_u64().unwrap();
        run(&mut s, "connect.static", json!({"id":c,"target":1,"point":format!("Connections.X{}",[2,1,3,1][index]),"end":"begin"}));
        run(&mut s, "connect.dynamic", json!({"id":c,"target":b,"end":"end"}));
    }
    close(&s, 3, &[("EndY", 2.5)]);
    close(&s, 5, &[("EndY", 2.75)]);
    for (id, x, y) in targets {
        run(&mut s, "shape.move", json!({"id":id,"x":x+0.01,"y":y}));
        run(&mut s, "shape.move", json!({"id":id,"x":x,"y":y}));
    }
    close(
        &s,
        3,
        &[
            ("BeginX", 4.0),
            ("BeginY", 5.5),
            ("EndX", 3.0),
            ("EndY", 2.375),
            ("Geometry1.X1", 0.0),
            ("Geometry1.Y1", 0.0),
            ("Geometry1.X2", 0.0),
            ("Geometry1.Y2", -3.125),
            ("Geometry1.X3", -0.45833333333333304),
            ("Geometry1.Y3", -3.1250000000000004),
            ("Geometry1.X4", -0.541666666666667),
            ("Geometry1.Y4", -3.125),
            ("Geometry1.A4", 0.04166666666666696),
            ("Geometry1.X5", -1.0),
            ("Geometry1.Y5", -3.125),
        ],
    );
    close(
        &s,
        5,
        &[
            ("BeginX", 3.5),
            ("BeginY", 5.5),
            ("EndX", 5.0),
            ("EndY", 2.6875),
            ("Geometry1.X1", 0.0),
            ("Geometry1.Y1", 0.0),
            ("Geometry1.X2", 0.0),
            ("Geometry1.Y2", -2.8125),
            ("Geometry1.X3", 0.45833333333333304),
            ("Geometry1.Y3", -2.8125000000000004),
            ("Geometry1.X4", 0.541666666666667),
            ("Geometry1.Y4", -2.8125),
            ("Geometry1.A4", -0.04166666666666696),
            ("Geometry1.X5", 0.958333333333333),
            ("Geometry1.Y5", -2.8125000000000004),
            ("Geometry1.X6", 1.041666666666667),
            ("Geometry1.Y6", -2.8125),
            ("Geometry1.A6", -0.04166666666666696),
            ("Geometry1.X7", 1.5),
            ("Geometry1.Y7", -2.8125),
        ],
    );
    close(
        &s,
        7,
        &[
            ("BeginX", 4.5),
            ("BeginY", 5.5),
            ("EndX", 4.5),
            ("EndY", 1.2),
            ("Geometry1.X1", 0.125),
            ("Geometry1.Y1", 0.0),
            ("Geometry1.X2", 0.125),
            ("Geometry1.Y2", -4.3),
        ],
    );
    close(
        &s,
        9,
        &[
            ("BeginX", 3.5),
            ("BeginY", 5.5),
            ("EndX", 6.0),
            ("EndY", 1.2),
            ("Geometry1.X1", 0.0),
            ("Geometry1.Y1", 0.0),
            ("Geometry1.X2", 0.0),
            ("Geometry1.Y2", -4.1125),
            ("Geometry1.X3", 0.958333333333333),
            ("Geometry1.Y3", -4.112500000000001),
            ("Geometry1.X4", 1.041666666666667),
            ("Geometry1.Y4", -4.1125),
            ("Geometry1.A4", -0.04166666666666696),
            ("Geometry1.X5", 2.5),
            ("Geometry1.Y5", -4.1125),
            ("Geometry1.X6", 2.5),
            ("Geometry1.Y6", -4.3),
        ],
    );
}
