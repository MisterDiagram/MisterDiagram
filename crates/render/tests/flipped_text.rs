//! Selected synthetic flip regression from craft-apps review 3b, user-authorized MIT OR Apache-2.0.
//! Additional flip combinations follow W5.
use diagramcraft_model::{Cell, Document, Shape, skeleton};
use diagramcraft_render::{pdf, raster, scene, svg};
fn set(shape: &mut Shape, name: &str, value: &str) {
    if let Some(c) = shape.sheet.cells.iter_mut().find(|c| c.name == name) {
        c.value = Some(value.into());
        c.formula = None;
    } else {
        shape.sheet.cells.push(Cell { name: name.into(), value: Some(value.into()), ..Default::default() });
    }
}

fn doc_with(shapes: Vec<Shape>) -> Document {
    let mut d = skeleton::document(false);
    d.pages[0].shapes = shapes;
    d
}

fn opts() -> scene::Options {
    scene::Options { crop: false, ..Default::default() }
}

fn png(d: &Document) -> image::RgbaImage {
    let s = scene::page(d, 0, opts()).unwrap();
    raster::render(&s, opts()).unwrap()
}

fn svg_raster(d: &Document) -> image::RgbaImage {
    let s = scene::page(d, 0, opts()).unwrap();
    let (w, h, _) = s.canvas(opts()).unwrap();
    let xml = svg::render(&s, opts()).unwrap();
    let tree = resvg::usvg::Tree::from_str(&xml, &resvg::usvg::Options::default()).unwrap();
    let mut pixels = resvg::tiny_skia::Pixmap::new(u32::from(w), u32::from(h)).unwrap();
    resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pixels.as_mut());
    image::RgbaImage::from_fn(u32::from(w), u32::from(h), |x, y| {
        let p = pixels.pixel(x, y).unwrap().demultiply();
        image::Rgba([p.red(), p.green(), p.blue(), p.alpha()])
    })
}

fn mad(a: &image::RgbaImage, b: &image::RgbaImage) -> (f64, usize) {
    let mut sum = 0u64;
    let mut big = 0usize;
    for (p, q) in a.pixels().zip(b.pixels()) {
        let d: u32 = (0..3).map(|i| u32::from(p.0[i].abs_diff(q.0[i]))).sum();
        sum += u64::from(d);
        if d > 3 * 64 {
            big += 1;
        }
    }
    (sum as f64 / (f64::from(a.width()) * f64::from(a.height()) * 3.), big)
}

#[test]
fn flipx_shape_text_is_not_mirrored() {
    let make = |flip: &str| {
        let mut r = skeleton::rectangle(1, 1., 1., 3., 1., "Flipped label").unwrap();
        set(&mut r, "LinePattern", "0");
        set(&mut r, "FillPattern", "0");
        set(&mut r, "FlipX", flip);
        doc_with(vec![r])
    };
    let a = png(&make("0"));
    let b = png(&make("1"));
    let (m, big) = mad(&a, &b);
    println!("FlipX=1 vs FlipX=0: MAD {m:.3}, {big} strongly different pixels");
    assert!(big < 20, "FlipX text differs from unflipped text in {big} pixels (MAD {m:.3}); it is drawn mirrored");
}

fn flipped(x: bool, y: bool) -> Document {
    let mut r = skeleton::rectangle(1, 1., 1., 3., 1., "Readable").unwrap();
    for name in ["LinePattern", "FillPattern"] {
        set(&mut r, name, "0");
    }
    set(&mut r, "FlipX", if x { "1" } else { "0" });
    set(&mut r, "FlipY", if y { "1" } else { "0" });
    doc_with(vec![r])
}
#[test]
fn all_flips_match_readable_text_in_png_svg_and_shared_pdf_scene() {
    let normal = flipped(false, false);
    let a = png(&normal);
    let sa = svg_raster(&normal);
    let normal_scene = scene::page(&normal, 0, opts()).unwrap();
    let transforms = |scene: &scene::Scene| {
        scene
            .ops
            .iter()
            .filter_map(|o| match o {
                scene::Op::Text { transform, .. } => Some(transform.as_coeffs()),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let base = transforms(&normal_scene);
    assert!(!base.is_empty());
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/brief-a-flips").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("plain.pdf"), pdf::render(&[normal_scene], opts()).unwrap()).unwrap();
    for (x, y, name) in [(true, false, "x"), (false, true, "y"), (true, true, "xy")] {
        let d = flipped(x, y);
        for (format, expected, actual) in [("PNG", &a, png(&d)), ("SVG", &sa, svg_raster(&d))] {
            let (mean, big) = mad(expected, &actual);
            assert!(big < 20, "{name} {format}: {big} strongly different pixels, MAD={mean}");
        }
        let s = scene::page(&d, 0, opts()).unwrap();
        let actual = transforms(&s);
        assert_eq!(actual.len(), base.len());
        for (a, b) in actual.iter().flatten().zip(base.iter().flatten()) {
            assert!((a - b).abs() < 1e-12, "{name} shared text transform {actual:?} != {base:?}");
        }
        std::fs::write(dir.join(format!("{name}.pdf")), pdf::render(&[s], opts()).unwrap()).unwrap();
    }
    eprintln!("flip PDF artifacts: {}", dir.display());
}
