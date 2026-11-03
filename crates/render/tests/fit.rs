//! Original controls for the native PNG drawing crop, with explicit physical padding.
//! Witness: field/brief66-2026-11-03/fit-native.
//! https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
use hayro_syntax::Pdf;
use misterdiagram_model::{Cell, skeleton};
use misterdiagram_render::{
    pdf, raster,
    scene::{self, Options},
    svg,
};
fn drawing() -> misterdiagram_model::Document {
    let mut d = skeleton::document(false);
    let mut a = skeleton::rectangle(1, -1., 3., 2., 1., "").unwrap();
    a.sheet.cells.push(Cell { name: "LineWeight".into(), value: Some((12. / 72.).to_string()), ..Default::default() });
    d.pages[0].shapes.push(a);
    d
}
#[test]
fn fitted_thick_offpage_rectangle_and_margin_share_all_three_canvases() {
    for scale in [1., 2.] {
        let d = drawing();
        let base = Options { scale, page_canvas: true, ..Default::default() }.with_fit(true, None).unwrap();
        let a = scene::page(&d, 0, base).unwrap();
        assert!((a.bounds.width() - 13. / 6.).abs() < 1e-9);
        let (_, _, original) = a.fit_canvas(base).unwrap();
        let unit_view = a.fit_canvas(Options { scale: 1., ..base }).unwrap().2;
        assert_eq!(original, kurbo::Affine::scale(scale) * unit_view);
        for margin in [None, Some("0.25 in"), Some("6.35 mm"), Some("18 pt")] {
            let options = base.with_fit(true, margin).unwrap();
            let s = scene::page(&d, 0, options).unwrap();
            let pad = if margin.is_some() { 24. * scale } else { 0. };
            let expected = (208. * scale + 2. * pad, 112. * scale + 2. * pad);
            let (w, h, view) = s.fit_canvas(options).unwrap();
            assert_eq!((w, h), expected);
            assert_eq!(view, kurbo::Affine::translate((pad, pad)) * original);
            assert_eq!(raster::render(&s, options).unwrap().dimensions(), (w as u32, h as u32));
            let xml = svg::render(&s, options).unwrap();
            let xml = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
            assert_eq!(xml.root_element().attribute("viewBox"), Some(format!("0 0 {} {}", w * 0.75 / scale, h * 0.75 / scale).as_str()));
            let bytes = pdf::render(&[s], options).unwrap();
            let parsed = Pdf::new(bytes).unwrap();
            let page = parsed.pages()[0].media_box();
            assert!((page.width() - w * 0.75 / scale).abs() < 1e-4);
            assert!((page.height() - h * 0.75 / scale).abs() < 1e-4);
        }
    }
}
#[test]
fn background_does_not_enlarge_fit_and_empty_page_keeps_its_page() {
    let mut d = drawing();
    let options = Options::default().with_fit(true, Some("0.25 in")).unwrap();
    let original = scene::page(&d, 0, options).unwrap().bounds;
    let mut bg = d.pages[0].clone();
    bg.id = 9;
    bg.attributes.insert("Background".into(), "1".into());
    bg.shapes = vec![skeleton::rectangle(1, -20., -20., 40., 40., "").unwrap()].into();
    d.pages.push(bg);
    d.pages[0].attributes.insert("BackPage".into(), "9".into());
    assert_eq!(scene::page(&d, 0, options).unwrap().bounds, original);
    d.pages[0].shapes.clear();
    let s = scene::page(&d, 0, options).unwrap();
    assert!(s.fit_empty);
    assert!(s.warnings.iter().any(|s| s.contains("empty foreground")));
    let (w, h, _) = s.fit_canvas(options).unwrap();
    assert_eq!((w, h), (s.page_size.0 * 96., s.page_size.1 * 96.));
}
#[test]
fn hostile_fit_options_and_scene_limits_remain_errors() {
    for text in ["-1 in", "NaN", "inf", "1e309 in", "1e100 in", "1 deg", "10%", "Width", "1/0 in", "1 in + 1 in", "", "1 mm^2"] {
        assert!(Options::default().with_fit(true, Some(text)).is_err(), "{text}");
    }
    assert!(Options::default().with_fit(false, Some("0 in")).is_err());
    assert!(Options::default().with_fit(true, Some(&"1".repeat(65537))).is_err());
    for margin in [f64::NAN, f64::INFINITY, -1., 1e6] {
        let o = Options { fit: true, margin, ..Default::default() };
        assert!(scene::page(&drawing(), 0, o).is_err());
    }
    let o = Options::default().with_fit(true, None).unwrap();
    let mut s = scene::page(&drawing(), 0, o).unwrap();
    s.ops.push(scene::Op::Pop);
    assert!(s.fit_canvas(o).is_err());
}
