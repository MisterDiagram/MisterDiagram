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

#[test]
fn fitted_pages_have_independent_media_boxes_and_shared_limits() {
    let mut d = drawing();
    let mut second = d.pages[0].clone();
    second.id = 2;
    second.part = "visio/pages/page2.xml".into();
    second.shapes = vec![skeleton::rectangle(2, 8., -3., 1., 3., "").unwrap()].into();
    d.pages.push(second);
    let options = Options::default().with_fit(true, Some("0.25 in")).unwrap();
    let scenes = scene::foreground_pages_budgeted(&d, options, &mut scene::SceneBudget::default()).unwrap();
    assert_eq!(scenes.len(), 2);
    let expected: Vec<_> = scenes.iter().map(|s| s.fit_canvas(options).unwrap()).collect();
    assert_ne!((expected[0].0, expected[0].1), (expected[1].0, expected[1].1));
    let pdf = Pdf::new(pdf::render(&scenes, options).unwrap()).unwrap();
    assert_eq!(pdf.pages().len(), 2);
    for (page, (w, h, _)) in pdf.pages().iter().zip(expected) {
        assert!((page.media_box().width() - w * 0.75).abs() < 1e-4);
        assert!((page.media_box().height() - h * 0.75).abs() < 1e-4);
    }
    let empty = skeleton::document(false);
    // One empty scene fits, but two must share the same retained allocation cap.
    let retained = scene::page(&empty, 0, options).unwrap();
    let capacity = std::mem::size_of::<scene::Scene>()
        + retained.warnings.capacity() * std::mem::size_of::<String>()
        + retained.warnings.iter().map(String::capacity).sum::<usize>();
    assert!(scene::page_budgeted(&empty, 0, options, &mut scene::SceneBudget::new(std::mem::size_of::<scene::Scene>())).is_err());
    scene::page_budgeted(&empty, 0, options, &mut scene::SceneBudget::new(capacity)).unwrap();
    let mut two = empty.clone();
    let mut page = two.pages[0].clone();
    page.id = 2;
    page.part = "visio/pages/page2.xml".into();
    two.pages.push(page);
    assert!(scene::foreground_pages_budgeted(&two, options, &mut scene::SceneBudget::new(capacity)).unwrap_err().contains("memory limit"));
    for id in 3..=4097 {
        let mut page = empty.pages[0].clone();
        page.id = id;
        two.pages.push(page);
    }
    assert_eq!(scene::foreground_pages_budgeted(&two, options, &mut scene::SceneBudget::default()).unwrap_err(), "export page limit");
}

#[test]
fn content_empty_pages_keep_dimensions_and_exactly_one_fit_warning() {
    for kind in 0..3 {
        let mut d = drawing();
        match kind {
            0 => d.pages[0].shapes.clear(),
            1 => {
                d.pages[0].shapes[0].shape_type = Some("Group".into());
                d.pages[0].shapes[0].sheet.sections.clear();
            }
            _ => {
                let geometry = d.pages[0].shapes[0].sheet.sections.iter_mut().find(|s| s.name == "Geometry").unwrap();
                geometry.cells.push(Cell { name: "NoShow".into(), value: Some("1".into()), ..Default::default() });
            }
        }
        let o = Options::default().with_fit(true, Some("5 mm")).unwrap();
        let s = scene::page(&d, 0, o).unwrap();
        assert!(s.fit_empty, "kind {kind}");
        assert_eq!(s.warnings.iter().filter(|s| s.contains("empty foreground")).count(), 1);
        let (w, h, _) = s.fit_canvas(o).unwrap();
        assert_eq!((w, h), (s.page_size.0 * 96., s.page_size.1 * 96.));
    }
}

#[test]
fn margins_translate_every_pixel_and_pattern_without_rescaling() {
    for pattern in [1., 14.] {
        let mut d = drawing();
        d.pages[0].shapes[0].sheet.cells.push(Cell { name: "FillPattern".into(), value: Some(pattern.to_string()), ..Default::default() });
        for scale in [1., 2.] {
            let o = Options { scale, ..Default::default() }.with_fit(true, None).unwrap();
            let s = scene::page(&d, 0, o).unwrap();
            let plain = raster::render(&s, o).unwrap();
            let padded = raster::render(&s, o.with_fit(true, Some("0.25 in")).unwrap()).unwrap();
            let pad = (24. * scale) as u32;
            assert_eq!(padded.dimensions(), (plain.width() + 2 * pad, plain.height() + 2 * pad));
            for (x, y, pixel) in padded.enumerate_pixels() {
                if x >= pad && y >= pad && x < pad + plain.width() && y < pad + plain.height() {
                    assert_eq!(pixel, plain.get_pixel(x - pad, y - pad));
                } else {
                    assert_eq!(pixel.0, [255; 4]);
                }
            }
        }
    }
}
