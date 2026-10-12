//! Release measurements, not self-assigned performance acceptance limits.
#[test]
#[ignore = "explicit release measurement at the P2 human gate"]
fn largest_stencil_and_500_shape_page() {
    use diagramcraft_render::{contact, raster, scene};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let start = std::time::Instant::now();
    let doc = diagramcraft_vsdx::read(root.join("ms-shipped/1033/WFDEP_M.vssx")).unwrap();
    let (image, report) = contact::render(&doc).unwrap();
    let largest_ms = start.elapsed().as_millis();
    image.save(root.join("_state/contact-sheets/WFDEP_M-ours.png")).unwrap();
    println!(
        "largest stencil: masters={} dimensions={}x{} errors={} elapsed_ms={largest_ms}",
        doc.masters.len(),
        image.width(),
        image.height(),
        report.failed
    );
    for e in &report.warnings {
        eprintln!("{e}");
    }
    let mut doc = diagramcraft_vsdx::read(root.join("generated/fill/pattern-01.vsdx")).unwrap();
    let mut base = doc.pages.first().unwrap().shapes.first().unwrap().clone();
    for c in &mut base.sheet.cells {
        if c.name == "Width" || c.name == "Height" {
            c.formula = None;
            c.value = Some("0.25".into());
        }
    }
    let page = doc.pages.first_mut().unwrap();
    page.shapes.clear();
    for i in 0..500u32 {
        let mut s = base.clone();
        s.id = i + 1;
        for c in &mut s.sheet.cells {
            match c.name.as_str() {
                "PinX" => {
                    c.formula = None;
                    c.value = Some((f64::from(i % 20) * 0.3).to_string());
                }
                "PinY" => {
                    c.formula = None;
                    c.value = Some((f64::from(i / 20) * 0.3).to_string());
                }
                _ => {}
            }
        }
        page.shapes.push(s);
    }
    let start = std::time::Instant::now();
    let scene = scene::page(&doc, 0, scene::Options::default()).unwrap();
    let image = raster::render(&scene, scene::Options::default()).unwrap();
    let page_ms = start.elapsed().as_millis();
    image.save(root.join("_state/render/500-shapes.png")).unwrap();
    println!("500-shape page: dimensions={}x{} elapsed_ms={page_ms}", image.width(), image.height());
    std::fs::write(
        root.join("_state/render-perf.txt"),
        format!("largest_ms={largest_ms}\nlargest_errors={}\npage_500_ms={page_ms}\n", report.failed),
    )
    .unwrap();
}
