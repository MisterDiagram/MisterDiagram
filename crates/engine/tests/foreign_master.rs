//! Contributor-original Foreign master bitmaps, built with the model writer.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml
//! https://learn.microsoft.com/en-us/office/client-developer/visio/rel-element-foreigndata_type-complextypevisio-xml
use diagramcraft_engine::Session;
use diagramcraft_geom::{Point, Rect};
use diagramcraft_model::{Cell, Document, Part, Relationship, XmlElement, XmlNode, skeleton};
use diagramcraft_render::{pdf, raster, scene, svg};
use hayro_syntax::{Pdf, content::ops::TypedInstruction, object::Name};
use image::{ImageFormat, Rgb, RgbImage};
use serde_json::json;
use std::{collections::BTreeMap, io::Cursor, path::PathBuf};

const VIS: &str = "http://schemas.microsoft.com/office/visio/2012/main";
const REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const RED: [u8; 3] = [255, 0, 0];
const GREEN: [u8; 3] = [0, 255, 0];

fn foreign() -> XmlElement {
    XmlElement {
        name: "ForeignData".into(),
        namespace: VIS.into(),
        attributes: BTreeMap::from([("ForeignType".into(), "Bitmap".into())]),
        children: vec![XmlNode::Element(XmlElement {
            name: "Rel".into(),
            namespace: VIS.into(),
            attributes: BTreeMap::from([("r:id".into(), "rImage".into()), ("xmlns:r".into(), REL.into())]),
            attribute_namespaces: BTreeMap::from([("r:id".into(), REL.into())]),
            ..Default::default()
        })],
        ..Default::default()
    }
}
fn media(doc: &mut Document, owner: &str, target: &str, format: ImageFormat, color: [u8; 3]) {
    let mut bytes = Cursor::new(Vec::new());
    RgbImage::from_pixel(8, 4, Rgb(color)).write_to(&mut bytes, format).unwrap();
    doc.parts.insert(
        target.into(),
        Part {
            bytes: bytes.into_inner(),
            content_type: Some(format!("image/{}", if format == ImageFormat::Bmp { "bmp" } else { "png" })),
            ..Default::default()
        },
    );
    doc.parts.entry(owner.into()).or_default().relationships.push(Relationship {
        id: "rImage".into(),
        type_uri: format!("{REL}/image"),
        target: target.into(),
        external: false,
    });
}
fn fixture(format: ImageFormat) -> PathBuf {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let fixture_id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let mut doc = skeleton::document(false);
    let master = &mut doc.masters[0];
    master.name = "Original bitmap".into();
    master.name_u = master.name.clone();
    let mut shape = skeleton::rectangle(5, 0., 0., 2., 1., "").unwrap();
    shape.shape_type = Some("Foreign".into());
    shape.sheet.sections.clear();
    shape.foreign_data = Some(foreign());
    for (name, value, formula) in [("ImgWidth", "2", "Width"), ("ImgHeight", "1", "Height")] {
        shape.sheet.cells.push(Cell { name: name.into(), value: Some(value.into()), formula: Some(formula.into()), ..Default::default() });
    }
    master.shapes = vec![shape];
    let part = master.part.clone();
    media(&mut doc, &part, &format!("visio/media/original.{format:?}"), format, RED);
    let bytes = diagramcraft_vsdx::write_as(&doc, diagramcraft_vsdx::SaveKind::Stencil).unwrap().bytes;
    let reopened = diagramcraft_vsdx::import(&bytes).unwrap();
    assert_eq!(reopened.masters.len(), 1);
    let scene = scene::master(&reopened, 0, scene::Options::default()).unwrap();
    assert_eq!(scene.ops.iter().filter(|op| matches!(op, scene::Op::Image { .. })).count(), 1, "master contact-sheet path works");
    let directory =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../target/engine-tests/foreign-{}-{fixture_id}-{format:?}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let path = directory.join("original.vssx");
    std::fs::write(&path, bytes).unwrap();
    path
}
fn instances(format: ImageFormat) -> (Session, PathBuf) {
    let path = fixture(format);
    let mut s = Session::new();
    s.execute("page.size", &json!({"width":8,"height":6})).unwrap();
    s.execute("stencil.open", &json!({"path":path})).unwrap();
    s.execute("master.drop", &json!({"stencil":1,"master":"Original bitmap","x":2,"y":1.5})).unwrap();
    s.execute("master.drop", &json!({"stencil":1,"master":"Original bitmap","x":5.5,"y":3.5,"width":1,"height":3})).unwrap();
    assert!(s.document().pages[0].shapes.iter().all(|s| s.foreign_data.is_none()));
    assert!(s.document().parts.keys().any(|p| p.starts_with("visio/imported/master3/visio/media/")));
    (s, path.with_extension("vsdx"))
}
fn outputs(doc: &Document, colors: [[u8; 3]; 2]) -> image::RgbaImage {
    let options = scene::Options { crop: false, ..Default::default() };
    let scene = scene::page(doc, 0, options).unwrap();
    assert!(scene.warnings.is_empty(), "{:?}", scene.warnings);
    let (_, _, view) = scene.canvas(options).unwrap();
    let boxes = [Rect::new(1., 1., 3., 2.), Rect::new(5., 2., 6., 5.)].map(|b| view.transform_rect_bbox(b));
    let pixels = raster::render(&scene, options).unwrap();
    let mut png = Cursor::new(Vec::new());
    pixels.write_to(&mut png, ImageFormat::Png).unwrap();
    let pixels = image::load_from_memory_with_format(png.get_ref(), ImageFormat::Png).unwrap().into_rgba8();
    let mut colored = [0usize; 2];
    for (x, y, pixel) in pixels.enumerate_pixels() {
        let p = Point::new(f64::from(x) + 0.5, f64::from(y) + 0.5);
        for (i, b) in boxes.iter().enumerate() {
            if b.inflate(-1., -1.).contains(p) {
                assert!(
                    pixel.0[..3].iter().zip(colors[i]).all(|(actual, expected)| if expected == 255 { *actual == 255 } else { *actual < 255 }),
                    "PNG instance {i} lacks source color at interior pixel ({x},{y}): {:?}",
                    pixel.0
                );
                if pixel.0[..3] == colors[i] {
                    colored[i] += 1;
                }
            }
        }
        if !boxes.iter().any(|b| b.inflate(1., 1.).contains(p)) {
            assert_eq!(pixel.0, [255; 4], "PNG ink outside instance boxes at ({x},{y})");
        }
    }
    assert!(colored.iter().all(|n| *n > 100), "PNG must contain both instance interiors");
    let svg = svg::render(&scene, options).unwrap();
    assert_eq!(svg.matches("<image ").count(), 2, "SVG must draw each instance");
    let pdf = Pdf::new(pdf::render(&[scene], options).unwrap()).unwrap();
    assert_eq!(pdf.pages().len(), 1);
    let page = &pdf.pages()[0];
    let mut operations = page.typed_operations();
    let mut drawn = 0;
    while let Some(op) = operations.next() {
        if let TypedInstruction::XObject(op) = op {
            let image = page.resources().get_x_object(op.0).unwrap();
            let subtype: Name<'_> = image.dict().get(b"Subtype").unwrap();
            assert_eq!(subtype.as_ref(), b"Image", "PDF Do must draw an image XObject");
            assert_eq!(image.decoded().unwrap().get(..3), Some(colors[drawn].as_slice()), "PDF instance color");
            drawn += 1;
        }
    }
    assert_eq!(drawn, 2, "PDF must draw an image XObject for each instance");
    pixels
}
#[test]
fn foreign_master_instances_render_bmp_and_png_in_all_outputs_and_roundtrip() {
    for format in [ImageFormat::Bmp, ImageFormat::Png] {
        let (mut s, saved) = instances(format);
        let before = outputs(s.document(), [RED, RED]);
        s.execute("file.save", &json!({"path":saved})).unwrap();
        s.execute("file.open", &json!({"path":saved})).unwrap();
        assert_eq!(before, outputs(s.document(), [RED, RED]), "saved VSDX must re-render identically");
    }
}
#[test]
fn local_foreign_data_wins_and_uses_the_page_relationship() {
    let (mut s, _) = instances(ImageFormat::Bmp);
    s.edit_document(|doc, _| {
        doc.pages[0].shapes[0].foreign_data = Some(foreign());
        let part = doc.pages[0].part.clone();
        // Same r:id as the master, but a different page-local image.
        media(doc, &part, "visio/media/local.png", ImageFormat::Png, GREEN);
        Ok(())
    })
    .unwrap();
    outputs(s.document(), [GREEN, RED]);
}
