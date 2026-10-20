//! Original regression for the readonly B25 Dell instance observation.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
use misterdiagram_model::{Cell, skeleton};
use misterdiagram_render::{raster, scene};
fn picture(formula: Option<&str>, cached: &str) -> (scene::Scene, image::RgbaImage) {
    let mut doc = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 1., 1., 1., 1., "").unwrap();
    let width = shape.sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap();
    width.value = Some("0".into());
    width.formula = None;
    shape.sheet.cells.retain(|c| c.name != "LineWeight");
    shape.sheet.cells.push(Cell { name: "LineWeight".into(), value: Some(cached.into()), formula: formula.map(str::to_owned), ..Default::default() });
    doc.pages[0].shapes = vec![shape].into();
    let options = scene::Options { scale: 4., ..Default::default() };
    let result = scene::page(&doc, 0, options).unwrap();
    let pixels = raster::render(&result, options).unwrap();
    (result, pixels)
}
#[test]
fn invalid_line_weight_with_zero_cache_keeps_the_witnessed_hairline_bounds() {
    let (literal, pixels) = picture(None, "0");
    let (invalid, actual) = picture(Some("REF()*1"), "0");
    assert_eq!(invalid.bounds, literal.bounds, "invalid line weight must retain witnessed zero-width bounds");
    assert_eq!(actual, pixels);
    let (valid, _) = picture(Some("0.03125"), "0");
    assert_ne!(valid.bounds, literal.bounds, "a valid formula takes precedence over its cache");
    for cache in ["NaN", "inf", "-1", "9999999999"] {
        assert!(picture(Some("REF()*1"), cache).0.bounds.width().is_finite());
    }
}
