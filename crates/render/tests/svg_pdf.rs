use misterdiagram_render::{pdf, raster, scene, svg};
#[test]
fn generated_svg_resvg_and_pdf_text() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = repo.join("corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let output = root.join("_state/svg-pdf");
    std::fs::create_dir_all(&output).unwrap();
    let mut failures = vec![];
    let mut count = 0usize;
    let mut worst = 0f64;
    for group in std::fs::read_dir(root.join("generated")).unwrap() {
        let group = group.unwrap().path();
        if !group.is_dir() {
            continue;
        }
        let dest = output.join(group.file_name().unwrap());
        std::fs::create_dir_all(&dest).unwrap();
        for file in std::fs::read_dir(&group).unwrap() {
            let file = file.unwrap().path();
            if !file.extension().is_some_and(|e| e == "vsdx") {
                continue;
            }
            let doc = misterdiagram_vsdx::read(&file).unwrap();
            let options = scene::Options::default();
            let scene = scene::page(&doc, scene::default_page_index(&doc).unwrap(), options).unwrap();
            let png = raster::render(&scene, options).unwrap();
            let xml = svg::render(&scene, options).unwrap();
            let tree = resvg::usvg::Tree::from_str(&xml, &resvg::usvg::Options::default()).unwrap();
            let mut pixels = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).unwrap();
            // The SVG page is transparent like Visio's export; the raster paints a white page.
            pixels.fill(resvg::tiny_skia::Color::WHITE);
            resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pixels.as_mut());
            let sum: u64 = png
                .pixels()
                .zip(pixels.pixels())
                .map(|(a, b)| {
                    let [r, g, blue, _] = a.0;
                    u64::from(r.abs_diff(b.red())) + u64::from(g.abs_diff(b.green())) + u64::from(blue.abs_diff(b.blue()))
                })
                .sum();
            let mad = sum as f64 / (f64::from(png.width()) * f64::from(png.height()) * 3.);
            worst = worst.max(mad);
            if mad > 2. {
                failures.push(format!("{} resvg MAD {mad}", file.display()));
            }
            let name = file.file_stem().unwrap().to_string_lossy();
            std::fs::write(dest.join(format!("{name}.svg")), xml).unwrap();
            std::fs::write(dest.join(format!("{name}.pdf")), pdf::render(&[scene], options).unwrap()).unwrap();
            count += 1;
        }
    }
    let script = r#"
import sys
from pathlib import Path
from pypdf import PdfReader
count = 0
text_count = 0
for file in Path(sys.argv[1]).glob('*/*.pdf'):
    reader = PdfReader(file)
    assert len(reader.pages) == 1, str(file)
    if file.parent.name == 'text':
        assert (reader.pages[0].extract_text() or '').strip(), str(file)
        text_count += 1
    count += 1
assert count == int(sys.argv[2]) and text_count == 16, (count, text_count)
print(f'pypdf: {count} one-page PDFs, {text_count} extractable text drawings')
"#;
    let result = std::process::Command::new(repo.join(".venv/bin/python"))
        .args(["-c", script])
        .arg(&output)
        .arg(count.to_string())
        .output()
        .expect("project .venv with pypdf is required when private corpus exists");
    println!("{}", String::from_utf8_lossy(&result.stdout));
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    println!("resvg: {count} files, worst MAD={worst:.6}");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn svg_thin_closed_stroke_matches_the_raster_pen() {
    use kurbo::Shape as _;
    use misterdiagram_render::scene::{Op, Scene};
    let scene = Scene {
        warnings: vec![],
        ops: vec![Op::Stroke {
            path: kurbo::Rect::new(0.01, 0.16, 0.78, 0.17).to_path(1e-5),
            stroke: kurbo::Stroke::new(1. / 96.),
            color: misterdiagram_color::Rgba::BLACK,
        }],
        bounds: kurbo::Rect::new(0., 0., 0.8, 0.33),
        page_size: (8.5, 11.),
    };
    let options = scene::Options::default();
    let png = raster::render(&scene, options).unwrap();
    let xml = svg::render(&scene, options).unwrap();
    let tree = resvg::usvg::Tree::from_str(&xml, &resvg::usvg::Options::default()).unwrap();
    let mut pixels = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).unwrap();
    pixels.fill(resvg::tiny_skia::Color::WHITE);
    resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pixels.as_mut());
    let error: u64 = png.pixels().zip(pixels.pixels()).map(|(a, b)| u64::from(a.0[0].abs_diff(b.red()))).sum();
    let mad = error as f64 / (f64::from(png.width()) * f64::from(png.height()));
    assert!(mad < 1., "thin closed stroke backend MAD={mad}");
}
