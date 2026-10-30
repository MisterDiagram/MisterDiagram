//! Contributor-original labelled connector uses the existing theme background default.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/textbkgnd-cell-text-block-format-section
use misterdiagram_engine::Session;
use misterdiagram_render::{raster, scene};
use serde_json::json;

#[test]
fn authored_label_occludes_its_connector_with_the_theme_default() {
    let mut s = Session::new();
    s.execute("shape.draw.line", &json!({"x1":1.,"y1":2.,"x2":4.,"y2":2.})).unwrap();
    s.execute("shape.text", &json!({"id":1,"text":"Alpha beta"})).unwrap();
    let shape = &s.document().pages[0].shapes[0];
    assert_eq!(
        shape.sheet.cells.iter().find(|c| c.name == "TextBkgnd").unwrap().formula.as_deref(),
        Some("THEMEGUARD(THEMEVAL(\"BackgroundColor\")+1)")
    );
    let options = scene::Options { scale: 2., crop: false, ..Default::default() };
    let page = scene::page(s.document(), 0, options).unwrap();
    let (rect, transform, color) = page
        .ops
        .iter()
        .find_map(|op| if let scene::Op::TextBackground { rect, transform, color } = op { Some((*rect, *transform, *color)) } else { None })
        .unwrap();
    assert_eq!((color.r, color.g, color.b, color.a), (255, 255, 255, 255));
    let image = raster::render(&page, options).unwrap();
    // A point near the top of the background is white, and its full shared
    // rectangle surrounds the label's connector crossing at page y=2.
    let bounds = transform.transform_rect_bbox(rect);
    assert!(bounds.y0 < 2. && bounds.y1 > 2.);
    let point = page.canvas(options).unwrap().2 * transform * misterdiagram_geom::Point::new(rect.x0 + 1., rect.y0 + 1.);
    assert_eq!(image.get_pixel(point.x.round() as u32, point.y.round() as u32).0, [255, 255, 255, 255]);
}
