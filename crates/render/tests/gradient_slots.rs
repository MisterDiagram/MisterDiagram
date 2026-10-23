use misterdiagram_color::Rgba;
use misterdiagram_model::{Cell, Row, Section, skeleton};
use misterdiagram_render::{raster, scene};

fn literal(name: &str, value: &str) -> Cell {
    Cell { name: name.into(), value: Some(value.into()), ..Default::default() }
}
fn stop(index: u32, position: &str, color: &str) -> Row {
    Row {
        index: Some(index),
        cells: vec![literal("GradientStopPosition", position), literal("GradientStopColor", color), literal("GradientStopColorTrans", "0")],
        ..Default::default()
    }
}

/// Blank THEMEVAL slots must not replace the first real colour when stops are sorted.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradient-section
#[test]
fn unused_theme_slots_do_not_whiten_an_authored_gradient() {
    let mut document = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
    shape.sheet.cells.push(literal("FillGradientEnabled", "1"));
    shape.sheet.cells.push(literal("FillGradientAngle", "0"));
    shape.sheet.sections.push(Section {
        name: "FillGradient".into(),
        rows: vec![stop(0, "0", "#801020"), stop(1, "1", "#102080")],
        ..Default::default()
    });
    document.pages[0].shapes.push(shape);
    let options = scene::Options { visio_export: true, ..Default::default() };
    let rendered = scene::page(&document, 0, options).unwrap();
    let stops = |scene: &scene::Scene| {
        scene
            .ops
            .iter()
            .find_map(|op| match op {
                scene::Op::Path { paint: scene::Paint::Pattern { gradient: Some(g), .. }, .. } => Some(g.stops.clone()),
                _ => None,
            })
            .unwrap()
    };
    assert_eq!(stops(&rendered), vec![(0., Rgba::rgb(128, 16, 32)), (1., Rgba::rgb(16, 32, 128))]);
    let image = raster::render(&rendered, options).unwrap();
    let pixel = image.get_pixel(image.width() / 2, image.height() / 2).0;
    assert!(pixel[..3].iter().all(|c| *c < 150), "unused white slots washed out {pixel:?}");

    // A literal white stop at zero is authored content, even when it follows a stop at one.
    document.pages[0].shapes[0].sheet.sections.last_mut().unwrap().rows.push(stop(2, "0", "1"));
    let rendered = scene::page(&document, 0, options).unwrap();
    let values = stops(&rendered);
    assert_eq!(values.len(), 3);
    assert!(values.contains(&(0., Rgba::WHITE)));
}

/// Theme provenance preserves real duplicate-position stops, including white at zero.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradient-section
#[test]
fn real_theme_white_stop_and_implicit_themed_slots_are_distinct() {
    use misterdiagram_model::Part;
    let mut document = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
    for (name, value) in [
        ("FillGradientEnabled", "1"),
        ("EffectSchemeIndex", "123"),
        ("ColorSchemeIndex", "123"),
        ("QuickStyleFillMatrix", "1"),
        ("QuickStyleFillColor", "0"),
        ("QuickStyleType", "2"),
    ] {
        shape.sheet.cells.retain(|cell| cell.name != name);
        shape.sheet.cells.push(literal(name, value));
    }
    // Contributor-original numerical theme. All three stops are intentional.
    let xml = br#"<theme><clrScheme><dk1><srgbClr val="14283C"/></dk1><accent1><srgbClr val="14283C"/></accent1><schemeID schemeEnum="123"/></clrScheme><fmtScheme><fillStyleLst><gradFill><gsLst><gs pos="0"><srgbClr val="801020"/></gs><gs pos="0"><srgbClr val="102080"/></gs><gs pos="0"><srgbClr val="FFFFFF"/></gs></gsLst></gradFill></fillStyleLst></fmtScheme><fmtSchemeEx><schemeID schemeEnum="123"/></fmtSchemeEx></theme>"#;
    document.parts.insert("visio/theme/theme1.xml".into(), Part { bytes: xml.to_vec().into(), ..Default::default() });
    shape.sheet.sections.push(Section {
        name: "FillGradient".into(),
        rows: (3..10)
            .map(|index| Row {
                index: Some(index),
                cells: ["GradientStopPosition", "GradientStopColor", "GradientStopColorTrans"].map(|name| literal(name, "Themed")).to_vec(),
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    });
    document.pages[0].shapes.push(shape);
    let rendered = scene::page(&document, 0, scene::Options::default()).unwrap();
    let gradient = rendered
        .ops
        .iter()
        .find_map(|op| match op {
            scene::Op::Path { paint: scene::Paint::Pattern { gradient: Some(g), .. }, .. } => Some(g),
            _ => None,
        })
        .unwrap();
    assert_eq!(gradient.stops, vec![(0., Rgba::rgb(128, 16, 32)), (0., Rgba::rgb(16, 32, 128)), (0., Rgba::WHITE)]);
}
