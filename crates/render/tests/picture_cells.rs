//! Contributor-original panels observed by the frozen B25 picture-cell checker (#54).
//! https://learn.microsoft.com/en-us/office/client-developer/visio/foreign-image-info-section
use misterdiagram_model::{Cell, Document, Part, Relationship, XmlElement, XmlNode, skeleton};
use misterdiagram_render::{raster, scene};
use std::io::Cursor;
fn emf() -> Vec<u8> {
    let mut records = Vec::new();
    fn record(out: &mut Vec<u8>, kind: u32, values: &[u32]) {
        for n in [kind, 8 + 4 * values.len() as u32].into_iter().chain(values.iter().copied()) {
            out.extend_from_slice(&n.to_le_bytes());
        }
    }
    record(&mut records, 37, &[0x80000008]);
    for (i, c) in [0x2828dc, 0x46b41e, 0xdc461e].into_iter().enumerate() {
        record(&mut records, 39, &[i as u32 + 1, 0, c, 0]);
        record(&mut records, 37, &[i as u32 + 1]);
        record(&mut records, 43, &[i as u32 * 100, 0, (i as u32 + 1) * 100, 200]);
    }
    record(&mut records, 14, &[0, 0, 20]);
    let mut bytes = Vec::new();
    for n in [1, 88, 0, 0, 300, 200, 0, 0, 7620, 5080, 0x464d4520, 0x10000, 88 + records.len() as u32, 12, 4, 0, 0, 0, 300, 200, 76, 51] {
        bytes.extend_from_slice(&n.to_le_bytes());
    }
    assert_eq!(bytes.len(), 88);
    bytes.extend(records);
    bytes
}
fn document(kind: &str, w: f64, h: f64, x: f64, y: f64) -> Document {
    let mut doc = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 0., 0., 3., 2., "").unwrap();
    shape.shape_type = Some("Foreign".into());
    shape.sheet.sections.clear();
    for (name, value) in [("ImgWidth", w), ("ImgHeight", h), ("ImgOffsetX", x), ("ImgOffsetY", y)] {
        shape.sheet.cells.push(Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() });
    }
    let ns = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    shape.foreign_data = Some(XmlElement {
        name: "ForeignData".into(),
        namespace: skeleton::VISIO_NS.into(),
        attributes: [("ForeignType".into(), "EnhMetaFile".into())].into(),
        children: vec![XmlNode::Element(XmlElement {
            name: "Rel".into(),
            namespace: skeleton::VISIO_NS.into(),
            attributes: [("r:id".into(), "picture".into())].into(),
            attribute_namespaces: [("r:id".into(), ns.into())].into(),
            ..Default::default()
        })],
        ..Default::default()
    });
    doc.pages[0].shapes = vec![shape].into();
    doc.pages[0].sheet.cells.iter_mut().filter(|c| c.name == "PageWidth" || c.name == "PageHeight").for_each(|c| {
        c.formula = None;
        c.value = Some("4".into());
    });
    let bytes = if kind == "emf" {
        emf()
    } else {
        let pixels = image::RgbImage::from_fn(300, 200, |x, _| image::Rgb([[220, 40, 40], [30, 180, 70], [30, 70, 220]][x as usize / 100]));
        let mut buffer = Cursor::new(Vec::new());
        pixels.write_to(&mut buffer, image::ImageFormat::Png).unwrap();
        buffer.into_inner()
    };
    let target = "visio/media/panels";
    doc.parts.insert(target.into(), Part { bytes: bytes.into(), ..Default::default() });
    doc.parts.entry(doc.pages[0].part.clone()).or_default().relationships.push(Relationship {
        id: "picture".into(),
        type_uri: format!("{ns}/image"),
        target: target.into(),
        external: false,
    });
    doc
}
fn dominant(p: &image::Rgba<u8>) -> Option<usize> {
    let rgb = &p.0[..3];
    if p.0[3] == 0 || rgb.iter().max().unwrap() - rgb.iter().min().unwrap() < 70 {
        None
    } else {
        rgb.iter().enumerate().max_by_key(|(_, v)| *v).map(|(i, _)| i)
    }
}
#[test]
fn emf_and_png_picture_size_offsets_and_clip_match_the_preobserved_fixture() {
    for kind in ["png", "emf"] {
        for (w, h, x, y) in [
            (0., 2., 0., 0.),
            (3., 0., 0., 0.),
            (3., 2., 0., 0.),
            (1.5, 1., 0., 0.),
            (6., 4., 0., 0.),
            (3., 2., -0.5, 0.),
            (3., 2., 0.5, 0.),
            (3., 2., 0., -0.5),
            (3., 2., 0., 0.5),
            (1.5, 1., 0.5, 0.5),
            (6., 4., -1., -1.),
        ] {
            let doc = document(kind, w, h, x, y);
            let options = scene::Options { scale: 2., crop: false, ..Default::default() };
            let s = scene::page(&doc, 0, options).unwrap();
            let im = raster::render(&s, options).unwrap();
            let (_, _, m) = s.canvas(options).unwrap();
            if w == 0. || h == 0. {
                assert!(im.pixels().all(|p| dominant(p).is_none()), "{kind} zero");
                continue;
            }
            for iy in 1..24 {
                for ix in 1..34 {
                    let px = ix as f64 / 10.;
                    let py = iy as f64 / 10.;
                    let u = (px - x) / w;
                    let v = (py - y) / h;
                    if (0..4).any(|i| (u - i as f64 / 3.).abs() < 0.025) || v.abs() < 0.025 || (v - 1.).abs() < 0.025 || px == 3. || py == 2. {
                        continue;
                    }
                    let expected = if px < 3. && py < 2. && u > 0. && u < 1. && v > 0. && v < 1. { Some((u * 3.) as usize) } else { None };
                    let p = m * misterdiagram_geom::Point::new(px, py);
                    assert_eq!(dominant(im.get_pixel(p.x.round() as u32, p.y.round() as u32)), expected, "{kind} {w} {h} {x} {y}: {px},{py}");
                }
            }
        }
    }
}
#[test]
fn zero_picture_size_does_not_decode_hostile_media() {
    let mut doc = document("emf", 0., 2., 0., 0.);
    doc.parts.get_mut("visio/media/panels").unwrap().bytes = vec![0xff; 32].into();
    assert!(scene::page(&doc, 0, scene::Options::default()).is_ok());
    doc.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "ImgWidth").unwrap().value = Some("3".into());
    assert!(scene::page(&doc, 0, scene::Options::default()).is_err());
}

#[test]
fn offset_picture_clip_limits_export_bounds_and_painted_pixels() {
    for kind in ["png", "emf"] {
        let doc = document(kind, 3., 2., 1., 0.5);
        let options = scene::Options { scale: 2., ..Default::default() };
        let cropped = scene::page(&doc, 0, options).unwrap();
        assert_eq!(cropped.bounds, misterdiagram_geom::Rect::new(1., 0.5, 3., 2.), "{kind} export bounds");
        let image = raster::render(&cropped, options).unwrap();
        assert_eq!(image.dimensions(), (384, 288), "{kind} cropped dimensions");

        // Retain surrounding page pixels so a missing clip cannot hide outside
        // the export canvas. Every coloured pixel must lie inside the shape.
        let options = scene::Options { crop: false, ..options };
        let page = scene::page(&doc, 0, options).unwrap();
        let image = raster::render(&page, options).unwrap();
        let inverse = page.canvas(options).unwrap().2.inverse();
        let mut painted = 0;
        for (x, y, pixel) in image.enumerate_pixels() {
            if dominant(pixel).is_some() {
                painted += 1;
                let p = inverse * misterdiagram_geom::Point::new(f64::from(x) + 0.5, f64::from(y) + 0.5);
                assert!(p.x >= 0.99 && p.x <= 3.01 && p.y >= 0.49 && p.y <= 2.01, "{kind} unclipped pixel at {p:?}");
            }
        }
        assert!(painted > 100_000, "{kind} picture must remain visible");
    }
}

/// Geometry2 ellipse controls are clipped in native PNG even with NoShow=1.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/clippingpath-cell-foreign-image-info-section
#[test]
fn clipping_path_uses_local_hidden_geometry_for_raster_and_emf() {
    use misterdiagram_model::{Row, Section};
    use misterdiagram_render::{pdf, svg};
    for kind in ["png", "emf"] {
        let mut doc = document(kind, 3., 2., 0., 0.);
        let shape = &mut doc.pages[0].shapes[0];
        shape.sheet.cells.push(Cell { name: "ClippingPath".into(), formula: Some("\"Geometry1.Path\"".into()), ..Default::default() });
        shape.sheet.sections.push(Section {
            name: "Geometry".into(),
            index: Some(0),
            cells: vec![Cell { name: "NoShow".into(), value: Some("1".into()), ..Default::default() }],
            rows: vec![Row {
                index: Some(1),
                row_type: Some("Ellipse".into()),
                cells: [("X", "1.5"), ("Y", "1"), ("A", "3"), ("B", "1"), ("C", "1.5"), ("D", "2")]
                    .into_iter()
                    .map(|(name, value)| Cell { name: name.into(), value: Some(value.into()), ..Default::default() })
                    .collect(),
                ..Default::default()
            }],
            ..Default::default()
        });
        let options = scene::Options { scale: 2., crop: false, ..Default::default() };
        let s = scene::page(&doc, 0, options).unwrap();
        let png = raster::render(&s, options).unwrap();
        let xml = svg::render(&s, options).unwrap();
        let tree = resvg::usvg::Tree::from_str(&xml, &resvg::usvg::Options::default()).unwrap();
        let mut svg_pixels = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).unwrap();
        svg_pixels.fill(resvg::tiny_skia::Color::WHITE);
        resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut svg_pixels.as_mut());
        let m = s.canvas(options).unwrap().2;
        for (x, y, painted) in [(0.1, 0.1, false), (2.9, 1.9, false), (1.5, 1., true), (0.2, 1., true)] {
            let p = m * misterdiagram_geom::Point::new(x, y);
            let (x, y) = (p.x.round() as u32, p.y.round() as u32);
            assert_eq!(dominant(png.get_pixel(x, y)).is_some(), painted, "{kind} PNG {x} {y}");
            let p = svg_pixels.pixel(x, y).unwrap();
            let p = image::Rgba([p.red(), p.green(), p.blue(), p.alpha()]);
            assert_eq!(dominant(&p).is_some(), painted, "{kind} SVG {x} {y}");
        }
        let bytes = pdf::render(&[s], options).unwrap();
        let parsed = hayro_syntax::Pdf::new(bytes).unwrap();
        let mut ops = parsed.pages()[0].typed_operations();
        let mut curves = 0;
        let mut curved_clip = false;
        while let Some(op) = ops.next() {
            use hayro_syntax::content::ops::TypedInstruction as T;
            match op {
                T::CubicTo(_) | T::CubicStartTo(_) | T::CubicEndTo(_) => curves += 1,
                T::ClipNonZero(_) | T::ClipEvenOdd(_) => curved_clip |= curves >= 4,
                T::EndPath(_) => curves = 0,
                _ => {}
            }
        }
        assert!(curved_clip, "{kind} PDF must encode the ellipse as a clipping path");
    }
}

/// Invalid and cross-sheet entries retain the documented rectangular fallback.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/clippingpath-cell-foreign-image-info-section
#[test]
fn clipping_path_reference_parser_is_bounded_and_local() {
    let options = scene::Options { crop: false, ..Default::default() };
    let reference = raster::render(&scene::page(&document("png", 6., 4., -1., -1.), 0, options).unwrap(), options).unwrap();
    for value in [
        String::new(),
        "Geometry0.Path".into(),
        "Geometry999.Path".into(),
        "Sheet.2!Geometry1.Path".into(),
        "Geometry+1.Path".into(),
        "Geometry4294967296.Path".into(),
        "Geometry1.X1".into(),
        "x".repeat(65536),
    ] {
        let mut d = document("png", 6., 4., -1., -1.);
        d.pages[0].shapes[0].sheet.cells.push(Cell {
            name: "ClippingPath".into(),
            value: Some(value),
            units: Some("STR".into()),
            ..Default::default()
        });
        let actual = raster::render(&scene::page(&d, 0, options).unwrap(), options).unwrap();
        assert_eq!(reference, actual);
    }
}

/// The original #54 crop case: a 4 inch image offset -1 inside a 2 by 1 inch border.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/imgwidth-cell-foreign-image-info-section
#[test]
fn two_by_one_crop_has_no_outside_pixels_in_png_svg_or_pdf_clip() {
    use misterdiagram_render::{pdf, svg};
    for kind in ["png", "emf"] {
        let mut d = document(kind, 4., 1., -1., 0.);
        for (name, value) in [("Width", "2"), ("Height", "1"), ("PinX", "2"), ("PinY", "1.5"), ("LocPinX", "1"), ("LocPinY", "0.5")] {
            let c = d.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == name).unwrap();
            c.formula = None;
            c.value = Some(value.into());
        }
        let options = scene::Options { scale: 2., crop: false, ..Default::default() };
        let s = scene::page(&d, 0, options).unwrap();
        let png = raster::render(&s, options).unwrap();
        let xml = svg::render(&s, options).unwrap();
        let tree = resvg::usvg::Tree::from_str(&xml, &resvg::usvg::Options::default()).unwrap();
        let mut pixels = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).unwrap();
        pixels.fill(resvg::tiny_skia::Color::WHITE);
        resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pixels.as_mut());
        let svg = image::RgbaImage::from_raw(png.width(), png.height(), pixels.data().to_vec()).unwrap();
        let inv = s.canvas(options).unwrap().2.inverse();
        for image in [&png, &svg] {
            let mut painted = 0;
            for (x, y, p) in image.enumerate_pixels() {
                if dominant(p).is_some() {
                    painted += 1;
                    let p = inv * misterdiagram_geom::Point::new(f64::from(x) + 0.5, f64::from(y) + 0.5);
                    assert!(p.x >= 0.99 && p.x <= 3.01 && p.y >= 0.99 && p.y <= 2.01, "{kind} outside crop at {p:?}");
                }
            }
            assert!(painted > 60_000);
        }
        let parsed = hayro_syntax::Pdf::new(pdf::render(&[s], options).unwrap()).unwrap();
        let mut ops = parsed.pages()[0].typed_operations();
        let mut clips = 0;
        while let Some(op) = ops.next() {
            if matches!(
                op,
                hayro_syntax::content::ops::TypedInstruction::ClipNonZero(_) | hayro_syntax::content::ops::TypedInstruction::ClipEvenOdd(_)
            ) {
                clips += 1;
            }
        }
        assert!(clips > 0, "{kind} PDF clip missing");
    }
}
