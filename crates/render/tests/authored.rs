use diagramcraft_model::{Cell, skeleton};
use diagramcraft_render::{raster, scene};
use diagramcraft_sheet::Value;
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};

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
    let written = diagramcraft_vsdx::write(&d).unwrap();
    let d = diagramcraft_vsdx::import(&written.bytes).unwrap();
    let options = scene::Options { visio_export: true, ..Default::default() };
    let image = raster::render(&scene::page(&d, 0, options).unwrap(), options).unwrap();
    assert_eq!(image.dimensions(), (193, 97));
    assert_eq!(image.get_pixel(3, 3).0, [255, 255, 255, 255]);
}

/// A literal RGB cache remains a colour when the optional palette has no entry.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
#[test]
fn undeclared_literal_rgb_is_a_colour_and_is_bounded() {
    let d = diagramcraft_model::Document::default();
    let e = Evaluator::new(&d).unwrap();
    let c = Cell { name: "FillForegnd".into(), value: Some("#ffffff".into()), ..Default::default() };
    assert!(matches!(e.cached_cell(&c), Value::Color { rgb: [255, 255, 255], .. }));
    assert_eq!(e.cached_cell(&c), e.cached_cell(&c));
    let mut c = c;
    c.value = Some(format!("#{}", "f".repeat(65_536)));
    assert!(matches!(e.cached_cell(&c), Value::Error(diagramcraft_sheet::Error::Limit)));
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
