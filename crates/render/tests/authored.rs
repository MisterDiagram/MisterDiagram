use misterdiagram_model::{Cell, skeleton};
use misterdiagram_render::{raster, scene};
use misterdiagram_sheet::Value;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};

/// Observed no-theme palette and white fill in scratch__rectangle-text/saved.vsdx.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
#[test]
fn authored_palette_and_white_render_follow_saved_witness() {
    let mut d = skeleton::document(false);
    assert_eq!(
        d.colors.iter().map(|(i, v)| (*i, v.as_str())).collect::<Vec<_>>(),
        vec![(24, "#7F7F7F"), (25, "#000000"), (26, "#FFFFFF"), (27, "#0000FF")]
    );
    d.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 2., 1., "Authored rectangle").unwrap());
    let written = misterdiagram_vsdx::write(&d).unwrap();
    let d = misterdiagram_vsdx::import(&written.bytes).unwrap();
    let options = scene::Options { visio_export: true, ..Default::default() };
    let image = raster::render(&scene::page(&d, 0, options).unwrap(), options).unwrap();
    assert_eq!(image.dimensions(), (193, 97));
    assert_eq!(image.get_pixel(3, 3).0, [255, 255, 255, 255]);
}

/// A literal RGB cache remains a colour when the optional palette has no entry.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
#[test]
fn undeclared_literal_rgb_is_a_colour_and_is_bounded() {
    let d = misterdiagram_model::Document::default();
    let e = Evaluator::new(&d).unwrap();
    let c = Cell { name: "FillForegnd".into(), value: Some("#ffffff".into()), ..Default::default() };
    assert!(matches!(e.cached_cell(&c), Value::Color { rgb: [255, 255, 255], .. }));
    assert_eq!(e.cached_cell(&c), e.cached_cell(&c));
    let mut c = c;
    c.value = Some(format!("#{}", "f".repeat(65_536)));
    assert!(matches!(e.cached_cell(&c), Value::Error(misterdiagram_sheet::Error::Limit)));
}

/// Metric omitted-U length cells use the owning DrawingScale unit.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
#[test]
fn metric_default_units_come_from_evaluator() {
    let mut d = skeleton::document(true);
    d.pages[0].shapes.push(skeleton::rectangle(1, 0., 0., 1., 1., "").unwrap());
    let e = Evaluator::new(&d).unwrap();
    let c = Cell::default();
    for scope in [Scope::Document, Scope::Style(0), Scope::Page(0), Scope::PageShape { page: 0, shape: 1 }] {
        for name in ["Para.IndFirst", "Para.IndLeft", "Para.IndRight", "Para.TextPosAfterBullet", "Width", "Geometry1.X2"] {
            assert_eq!(e.default_unit_code(scope, name, &c, 73), 70, "{scope:?}/{name}");
        }
        assert_eq!(e.default_unit_code(scope, "Angle", &c, 81), 81);
        assert_eq!(e.default_unit_code(scope, "Width", &Cell { units: Some("DL".into()), ..Default::default() }, 73), 73);
        assert_eq!(e.default_unit_code(scope, "Width", &Cell { units: Some("IN".into()), ..Default::default() }, 65), 65);
    }
}

/// Native open drops later duplicates, including an entire duplicate group subtree.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
#[test]
fn duplicate_shapes_render_as_the_first_occurrence_before_and_after_save() {
    let mut d = skeleton::document(false);
    d.pages[0].shapes.push(skeleton::rectangle(5, 1., 1., 1., 1., "").unwrap());
    let options = scene::Options { visio_export: true, ..Default::default() };
    let expected = raster::render(&scene::page(&d, 0, options).unwrap(), options).unwrap();
    let mut loser = skeleton::rectangle(5, 5., 5., 2., 2., "").unwrap();
    loser.shape_type = Some("Group".into());
    loser.shapes.push(skeleton::rectangle(6, 3., 3., 1., 1., "").unwrap());
    d.pages[0].shapes.push(loser);
    let actual = raster::render(&scene::page(&d, 0, options).unwrap(), options).unwrap();
    assert_eq!(actual, expected);
    let written = misterdiagram_vsdx::write(&d).unwrap();
    assert!(written.warnings.iter().any(|w| w.contains("page 0") && w.contains("shape ID 5")));
    let clean = misterdiagram_vsdx::import(&written.bytes).unwrap();
    assert_eq!(raster::render(&scene::page(&clean, 0, options).unwrap(), options).unwrap(), expected);
}
#[test]
fn deleted_members_and_their_descendants_never_draw_for_either_boolean_spelling() {
    let mut d = skeleton::document(false);
    d.pages[0].shapes.push(skeleton::rectangle(5, 1., 1., 1., 1., "").unwrap());
    let options = scene::Options { visio_export: true, ..Default::default() };
    let expected = raster::render(&scene::page(&d, 0, options).unwrap(), options).unwrap();
    for value in ["1", "true"] {
        let mut deleted = skeleton::rectangle(6, 5., 5., 2., 2., "").unwrap();
        deleted.shape_type = Some("Group".into());
        deleted.sheet.attributes.insert("Del".into(), value.into());
        deleted.shapes.push(skeleton::rectangle(7, 3., 3., 1., 1., "").unwrap());
        d.pages[0].shapes.push(deleted);
        assert_eq!(raster::render(&scene::page(&d, 0, options).unwrap(), options).unwrap(), expected);
        d.pages[0].shapes.pop();
    }
}
/// field/duplicate-ids-2026-10-31/native-deleted-duplicates.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
#[test]
fn deleted_duplicate_ownership_renders_consistently_before_and_after_save() {
    // Fixed page bounds isolate visible content from empty-export crop policy.
    let options = scene::Options::default();
    for parent in [false, true] {
        for deleted_first in [false, true] {
            let mut d = skeleton::document(false);
            let live = skeleton::rectangle(5, 6., 1., 2., 1., "").unwrap();
            let mut expected = d.clone();
            if parent || !deleted_first {
                expected.pages[0].shapes.push(live.clone());
            }
            let expected = raster::render(&scene::page(&expected, 0, options).unwrap(), options).unwrap();
            let mut dead = skeleton::rectangle(if parent { 20 } else { 5 }, 1., 1., 1., 1., "").unwrap();
            dead.sheet.attributes.insert("Del".into(), "true".into());
            if parent {
                dead.shape_type = Some("Group".into());
                dead.shapes.push(skeleton::rectangle(5, 1., 1., 1., 1., "").unwrap());
            }
            d.pages[0].shapes = if deleted_first { vec![dead, live] } else { vec![live, dead] }.into();
            let actual = raster::render(&scene::page(&d, 0, options).unwrap(), options).unwrap();
            assert!(actual == expected, "parent={parent} first={deleted_first}");
            let clean = misterdiagram_vsdx::import(&misterdiagram_vsdx::write(&d).unwrap().bytes).unwrap();
            assert!(raster::render(&scene::page(&clean, 0, options).unwrap(), options).unwrap() == expected);
        }
    }
}
