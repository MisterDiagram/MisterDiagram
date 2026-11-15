//! Contributor-original controls for physical vector pages and raster limits (#120).
//! https://learn.microsoft.com/en-us/office/client-developer/visio/pagewidth-cell-page-properties-section
use misterdiagram_geom::{BezPath, Rect};
use misterdiagram_model::{Cell, Document, Part, Relationship, Sheet, XmlElement, XmlNode, skeleton};
use misterdiagram_render::{
    pdf, raster,
    scene::{self, Op, Options, Paint, Scene},
    svg,
};
use std::io::Cursor;

fn vector_options() -> Options {
    Options { page_canvas: true, ..Default::default() }
}

fn drawing(width: f64, height: f64, full: bool) -> Scene {
    let bounds = if full { Rect::new(0., 0., width, height) } else { Rect::new(1., 1., 2., 2.) };
    let mut path = BezPath::new();
    path.move_to((bounds.x0, bounds.y0));
    path.line_to((bounds.x1, bounds.y0));
    path.line_to((bounds.x1, bounds.y1));
    path.line_to((bounds.x0, bounds.y1));
    path.close_path();
    Scene {
        fit_empty: false,
        warnings: vec![],
        pdf_strokes: Default::default(),
        page_size: (width, height),
        bounds,
        ops: vec![Op::Path { even_odd: false, path, paint: Paint::Solid(misterdiagram_color::Rgba::BLACK) }],
    }
}

#[test]
fn large_vector_page_keeps_exact_physical_canvas_and_small_png() {
    for full in [false, true] {
        let s = drawing(60., 62., full);
        let xml = svg::render(&s, vector_options()).unwrap();
        assert!(xml.contains("width=\"60in\" height=\"62in\" viewBox=\"0 0 4320 4464\""));
        let bytes = pdf::render(&[s], vector_options()).unwrap();
        assert!(bytes.windows(b"/MediaBox[0 0 4320 4464]".len()).any(|w| w == b"/MediaBox[0 0 4320 4464]"));
    }
    let small = raster::render(&drawing(6., 6., false), Options::default()).unwrap();
    let large = raster::render(&drawing(60., 62., false), Options::default()).unwrap();
    assert_eq!(large, small);
}

#[test]
fn large_vector_page_preserves_raster_and_fit_product_limits() {
    let s = drawing(60., 62., true);
    for page_canvas in [false, true] {
        let options = Options { page_canvas, ..Default::default() };
        assert_eq!(s.canvas(options).unwrap_err(), "render pixel limit");
        assert_eq!(raster::render(&s, options).unwrap_err(), "render pixel limit");
    }
    let options = Options { fit: true, ..vector_options() };
    for empty in [false, true] {
        let mut s = s.clone();
        s.fit_empty = empty;
        if empty {
            s.ops.clear();
        }
        assert_eq!(svg::render(&s, options).unwrap_err(), "render pixel limit");
        assert_eq!(pdf::render(&[s], options).unwrap_err(), "render pixel limit");
    }
}

#[test]
fn large_vector_page_preserves_each_side_and_invalid_page_limits() {
    for (width, height) in [(700., 1.), (1., 700.)] {
        let s = drawing(width, height, false);
        assert_eq!(svg::render(&s, vector_options()).unwrap_err(), "render pixel limit");
        assert_eq!(pdf::render(&[s], vector_options()).unwrap_err(), "render pixel limit");
    }
    for width in [0., -1., f64::NAN, f64::INFINITY, 1_000_001.] {
        let s = drawing(width, 62., false);
        assert_eq!(svg::render(&s, vector_options()).unwrap_err(), "vector page size limit");
        assert_eq!(pdf::render(&[s], vector_options()).unwrap_err(), "vector page size limit");
    }
}

fn set(sheet: &mut Sheet, name: &str, value: &str) {
    if let Some(cell) = sheet.cells.iter_mut().find(|c| c.name == name) {
        cell.formula = None;
        cell.value = Some(value.into());
    } else {
        sheet.cells.push(Cell { name: name.into(), value: Some(value.into()), ..Default::default() });
    }
}

fn mixed_document() -> Document {
    let mut doc = skeleton::document(false);
    set(&mut doc.pages[0].sheet, "PageWidth", "60");
    set(&mut doc.pages[0].sheet, "PageHeight", "62");
    let mut shadow = skeleton::rectangle(1, 2., 2., 1., 1., "").unwrap();
    for (name, value) in [
        ("ShdwPattern", "1"),
        ("ShapeShdwType", "1"),
        ("ShapeShdwOffsetX", "0.2"),
        ("ShapeShdwOffsetY", "0.2"),
        ("ShapeShdwBlur", "0.08"),
        ("ShdwForegnd", "#501e6e"),
        ("ShdwForegndTrans", "0"),
    ] {
        set(&mut shadow.sheet, name, value);
    }
    let mut picture = skeleton::rectangle(2, 5., 2., 1., 1., "").unwrap();
    picture.shape_type = Some("Foreign".into());
    picture.sheet.sections.clear();
    for (name, value) in [("ImgWidth", "1"), ("ImgHeight", "1"), ("ImgOffsetX", "0"), ("ImgOffsetY", "0")] {
        set(&mut picture.sheet, name, value);
    }
    let ns = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    picture.foreign_data = Some(XmlElement {
        name: "ForeignData".into(),
        namespace: skeleton::VISIO_NS.into(),
        attributes: [("ForeignType".into(), "Bitmap".into()), ("CompressionType".into(), "PNG".into())].into(),
        children: vec![XmlNode::Element(XmlElement {
            name: "Rel".into(),
            namespace: skeleton::VISIO_NS.into(),
            attributes: [("r:id".into(), "picture".into())].into(),
            attribute_namespaces: [("r:id".into(), ns.into())].into(),
            ..Default::default()
        })],
        ..Default::default()
    });
    let pixels = image::RgbaImage::from_fn(4, 4, |x, y| image::Rgba([20 + x as u8 * 30, 40 + y as u8 * 30, 180, 255]));
    let mut encoded = Cursor::new(Vec::new());
    pixels.write_to(&mut encoded, image::ImageFormat::Png).unwrap();
    let target = "visio/media/original.png";
    doc.parts.insert(target.into(), Part { bytes: encoded.into_inner().into(), ..Default::default() });
    doc.parts.entry(doc.pages[0].part.clone()).or_default().relationships.push(Relationship {
        id: "picture".into(),
        type_uri: format!("{ns}/image"),
        target: target.into(),
        external: false,
    });
    doc.pages[0].shapes = vec![shadow, picture].into();
    doc
}

#[test]
fn large_vector_page_renders_real_shadow_and_embedded_picture() {
    let doc = mixed_document();
    let s = scene::page(&doc, 0, vector_options()).unwrap();
    assert!(s.ops.iter().any(|op| matches!(op, Op::Image { pixels, .. } if pixels.dimensions() != (4, 4) && pixels.pixels().any(|p| p.0[..3] == [80, 30, 110] && p.0[3] > 0))), "shadow raster survives");
    assert!(s.ops.iter().any(|op| matches!(op, Op::Image { pixels, .. } if pixels.dimensions() == (4, 4))), "embedded picture survives");
    let xml = svg::render(&s, vector_options()).unwrap();
    assert!(xml.contains("width=\"60in\" height=\"62in\" viewBox=\"0 0 4320 4464\""));
    let parsed = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
    assert!(parsed.descendants().filter(|node| node.has_tag_name("image")).count() >= 2);
    let bytes = pdf::render(&[s], vector_options()).unwrap();
    assert!(bytes.windows(b"/MediaBox[0 0 4320 4464]".len()).any(|w| w == b"/MediaBox[0 0 4320 4464]"));
}
