use misterdiagram_model::{Cell, Row, Section, Shape, skeleton};
use misterdiagram_opc::Package;
use misterdiagram_render::scene::Options;
fn set(s: &mut Shape, n: &str, v: &str) {
    s.sheet.cells.retain(|c| c.name != n);
    s.sheet.cells.push(Cell { name: n.into(), value: Some(v.into()), ..Default::default() });
}
fn package(doc: &misterdiagram_model::Document) -> Package {
    let output = misterdiagram_export::document(doc, None, Options::default()).unwrap();
    Package::open(&output.bytes).unwrap()
}
#[test]
fn independent_geometry_flags_and_group_paint_order_are_native() {
    for mode in [0, 1, 2] {
        let mut doc = skeleton::document(false);
        let mut group = skeleton::rectangle(1, 1., 1., 4., 2., "").unwrap();
        group.shape_type = Some("Group".into());
        set(&mut group, "DisplayMode", &mode.to_string());
        let mut child = skeleton::rectangle(2, 1., 1., 2., 1., "").unwrap();
        let mut other = child.sheet.sections.iter().find(|s| s.name == "Geometry").unwrap().clone();
        other.index = Some(1);
        other.cells.push(Cell { name: "NoLine".into(), value: Some("1".into()), ..Default::default() });
        child.sheet.sections.iter_mut().find(|s| s.name == "Geometry").unwrap().cells.push(Cell {
            name: "NoFill".into(),
            value: Some("1".into()),
            ..Default::default()
        });
        child.sheet.sections.push(other);
        group.shapes.push(child);
        doc.pages[0].shapes.push(group);
        let pkg = package(&doc);
        let slide = pkg.xml("ppt/slides/slide1.xml").unwrap();
        let group = slide.root.find("grpSp").unwrap();
        let xfrm = group.path(&["grpSpPr", "xfrm"]).unwrap();
        assert_eq!(xfrm.child("off").unwrap().attr("x"), xfrm.child("chOff").unwrap().attr("x"));
        let shapes: Vec<_> = group.children_named("sp").collect();
        let names: Vec<_> = shapes.iter().map(|s| s.path(&["nvSpPr", "cNvPr"]).unwrap().attr("name").unwrap()).collect();
        assert_eq!(
            names,
            match mode {
                0 => vec!["Shape 2"],
                1 => vec!["Shape 1", "Shape 2"],
                _ => vec!["Shape 2", "Shape 1"],
            }
        );
        let child = shapes.iter().find(|s| s.path(&["nvSpPr", "cNvPr"]).unwrap().attr("name") == Some("Shape 2")).unwrap();
        let paths: Vec<_> = child.path(&["spPr", "custGeom", "pathLst"]).unwrap().children_named("path").collect();
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0].attr("fill"), Some("none"));
        assert_eq!(paths[1].attr("stroke"), Some("0"));
        assert!(slide.root.find("style").is_none());
        assert!(slide.root.find("schemeClr").is_none());
        assert!(child.path(&["spPr", "xfrm"]).unwrap().attr("rot").is_none());
    }
}
#[test]
fn all_dash_patterns_and_explicit_alpha_survive_serialization() {
    for pattern in 1..=23 {
        let mut doc = skeleton::document(false);
        let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
        set(&mut shape, "LinePattern", &pattern.to_string());
        set(&mut shape, "LineWeight", "0.013888888888888888");
        set(&mut shape, "FillForegndTrans", "0.5");
        doc.pages[0].shapes.push(shape);
        let pkg = package(&doc);
        let slide = pkg.xml("ppt/slides/slide1.xml").unwrap();
        let sp = slide.root.find("spPr").unwrap();
        assert_eq!(sp.path(&["solidFill", "srgbClr", "alpha"]).unwrap().attr("val"), Some("50196"));
        let line = sp.child("ln").unwrap();
        assert_eq!(line.attr("w"), Some("12700"));
        if pattern > 1 {
            assert!(line.find("custDash").is_some());
        } else {
            assert!(line.find("prstDash").is_some());
        }
        if pattern == 2 {
            let ds = line.find("ds").unwrap();
            assert_eq!(ds.attr("d"), Some("700000"));
            assert_eq!(ds.attr("sp"), Some("500000"));
        }
        if pattern == 23 {
            let ds = line.find("ds").unwrap();
            assert_eq!(ds.attr("d"), Some("100000"));
            assert_eq!(ds.attr("sp"), Some("200000"));
        }
    }
}
#[test]
fn gradient_stops_and_radial_shape_are_written_without_theme_styles() {
    for radial in [false, true] {
        let mut doc = skeleton::document(false);
        let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
        set(&mut shape, "FillGradientEnabled", "1");
        set(&mut shape, "FillGradientAngle", "0");
        set(&mut shape, "FillGradientDir", if radial { "1" } else { "0" });
        shape.sheet.sections.push(Section {
            name: "FillGradient".into(),
            rows: [(0, "0", "#801020"), (1, "1", "#102080")]
                .into_iter()
                .map(|(i, p, c)| Row {
                    index: Some(i),
                    cells: [("GradientStopPosition", p), ("GradientStopColor", c), ("GradientStopColorTrans", "0")]
                        .map(|(n, v)| Cell { name: n.into(), value: Some(v.into()), ..Default::default() })
                        .into(),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        });
        doc.pages[0].shapes.push(shape);
        let pkg = package(&doc);
        let slide = pkg.xml("ppt/slides/slide1.xml").unwrap();
        let gradient = slide.root.find("gradFill").unwrap();
        let stops: Vec<_> = gradient.child("gsLst").unwrap().children_named("gs").collect();
        assert_eq!(stops.len(), 2);
        assert_eq!(stops[0].child("srgbClr").unwrap().attr("val"), Some("801020"));
        if radial {
            assert_eq!(gradient.child("path").unwrap().attr("path"), Some("circle"));
            let focus = gradient.path(&["path", "fillToRect"]).unwrap();
            assert_eq!([focus.attr("l"), focus.attr("t"), focus.attr("r"), focus.attr("b")], [Some("100000"), Some("100000"), Some("0"), Some("0")]);
        } else {
            assert_eq!(gradient.child("lin").unwrap().attr("ang"), Some("0"));
        }
    }
}

#[test]
fn nested_contours_are_named_substitution_with_unfilled_hole() {
    let mut doc = skeleton::document(false);
    doc.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 3., 3., "").unwrap());
    let mut page = misterdiagram_render::scene::evaluated::page(&doc, 0, Options::default(), &mut Default::default()).unwrap();
    let outer = page.shapes[0].sections[0].path.clone();
    use kurbo::Shape as _;
    let bounds = outer.bounding_box();
    let hole = bounds.inset(-bounds.width() / 4.).to_path(1e-5);
    page.shapes[0].sections[0].path.extend(hole.elements().iter().copied());
    let model = misterdiagram_export::model::Page::from_evaluated(page).unwrap();
    assert!(model.substitutions.contains(&misterdiagram_export::model::Substitution::Alternate));
    assert_eq!(model.shapes[0].sections.len(), 2);
    assert!(!model.shapes[0].sections[0].flags.no_fill);
    assert!(model.shapes[0].sections[1].flags.no_fill);
}

#[test]
fn non_square_gradients_keep_direction_and_rotated_stop_domain() {
    // A local 45-degree ramp in a 2x1 box has scalar normal (1/2,1).
    for (angle, rotation, flip, expected, domain) in [
        (std::f64::consts::FRAC_PI_4, 0., false, 296.565051177078, false),
        (std::f64::consts::FRAC_PI_4, 0., true, 243.434948822922, false),
        (0., std::f64::consts::FRAC_PI_4, false, 315., true),
    ] {
        let mut doc = skeleton::document(false);
        let mut shape = skeleton::rectangle(1, 3., 3., 2., 1., "").unwrap();
        for (n, v) in [
            ("FillGradientEnabled", "1".into()),
            ("FillGradientAngle", angle.to_string()),
            ("Angle", rotation.to_string()),
            ("FlipX", if flip { "1" } else { "0" }.into()),
        ] {
            set(&mut shape, n, &v);
        }
        shape.sheet.sections.push(Section {
            name: "FillGradient".into(),
            rows: [(0, "0", "#000000"), (1, "1", "#FFFFFF")]
                .into_iter()
                .map(|(i, t, c)| Row {
                    index: Some(i),
                    cells: [("GradientStopPosition", t), ("GradientStopColor", c), ("GradientStopColorTrans", "0")]
                        .map(|(n, v)| Cell { name: n.into(), value: Some(v.into()), ..Default::default() })
                        .into(),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        });
        doc.pages[0].shapes.push(shape);
        let pkg = package(&doc);
        let slide = pkg.xml("ppt/slides/slide1.xml").unwrap();
        let gradient = slide.root.find("gradFill").unwrap();
        let linear = gradient.child("lin").unwrap();
        let actual: f64 = linear.attr("ang").unwrap().parse::<f64>().unwrap() / 60000.;
        assert!((actual - expected).abs() < 0.00002, "{actual} versus {expected}");
        assert_eq!(linear.attr("scaled"), Some("0"));
        let stops: Vec<_> = gradient.child("gsLst").unwrap().children_named("gs").map(|s| s.attr("pos").unwrap().parse::<i64>().unwrap()).collect();
        assert_eq!(stops, if domain { vec![0, 16667, 83333, 100000] } else { vec![0, 100000] });
    }
}

#[test]
fn radial_focus_follows_flips_and_baked_rotation() {
    for (rotation, flip, expected) in [(0., true, [0, 100000, 100000, 0]), (std::f64::consts::FRAC_PI_2, false, [100000, 0, 0, 100000])] {
        let mut doc = skeleton::document(false);
        let mut shape = skeleton::rectangle(1, 3., 3., 2., 1., "").unwrap();
        for (name, value) in [
            ("FillGradientEnabled", "1".into()),
            ("FillGradientDir", "1".into()),
            ("Angle", rotation.to_string()),
            ("FlipX", if flip { "1" } else { "0" }.into()),
        ] {
            set(&mut shape, name, &value);
        }
        doc.pages[0].shapes.push(shape);
        let pkg = package(&doc);
        let slide = pkg.xml("ppt/slides/slide1.xml").unwrap();
        let focus = slide.root.find("fillToRect").unwrap();
        let actual: [i64; 4] = ["l", "t", "r", "b"].map(|name| focus.attr(name).unwrap().parse().unwrap());
        assert_eq!(actual, expected);
    }
}
