use diagramcraft_render::{pdf, raster, scene, svg};
#[test]
fn generated_pages_and_foreign_objects_produce_outputs() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let mut paths = vec![];
    for group in std::fs::read_dir(root.join("generated")).unwrap() {
        let group = group.unwrap().path();
        if !group.is_dir() {
            continue;
        }
        for file in std::fs::read_dir(group).unwrap() {
            let path = file.unwrap().path();
            if path.extension().is_some_and(|e| e == "vsdx") {
                paths.push(path);
            }
        }
    }
    paths.sort();
    let output = root.join("_state/m2-5-output");
    std::fs::create_dir_all(&output).unwrap();
    let mut errors = vec![];
    for path in paths {
        let result = (|| -> Result<(), String> {
            let doc = diagramcraft_vsdx::read(&path).map_err(|e| e.to_string())?;
            let scene = scene::page(&doc, scene::default_page_index(&doc)?, scene::Options::default())?;
            let stem = path.file_stem().unwrap().to_string_lossy();
            let output = output.join(path.parent().unwrap().file_name().unwrap());
            std::fs::create_dir_all(&output).map_err(|e| e.to_string())?;
            raster::render(&scene, scene::Options::default())?.save(output.join(format!("{stem}.png"))).map_err(|e| e.to_string())?;
            std::fs::write(output.join(format!("{stem}.svg")), svg::render(&scene, scene::Options::default())?).map_err(|e| e.to_string())?;
            std::fs::write(output.join(format!("{stem}.pdf")), pdf::render(&[scene], scene::Options::default())?).map_err(|e| e.to_string())?;
            Ok(())
        })();
        if let Err(error) = result {
            errors.push(format!("{}: {error}", path.display()));
        }
    }
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

#[test]
fn layers_visibility_print_and_background_pages() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let doc = diagramcraft_vsdx::read(root.join("generated/page/layers.vsdx")).unwrap();
    let screen = scene::page(&doc, 0, scene::Options::default()).unwrap();
    let printed = scene::page(&doc, 0, scene::Options { print: true, ..Default::default() }).unwrap();
    assert!(printed.ops.len() > screen.ops.len());
    let mut doc = diagramcraft_vsdx::read(root.join("generated/page/background.vsdx")).unwrap();
    let index = doc.pages.iter().position(|p| p.attributes.contains_key("BackPage")).unwrap();
    assert_eq!(scene::default_page_index(&doc).unwrap(), index);
    let full = scene::page(&doc, index, scene::Options::default()).unwrap();
    doc.pages.get_mut(index).unwrap().attributes.remove("BackPage");
    let foreground = scene::page(&doc, index, scene::Options::default()).unwrap();
    assert!(full.ops.len() > foreground.ops.len());
    let id = doc.pages.get(index).unwrap().id.to_string();
    doc.pages.get_mut(index).unwrap().attributes.insert("BackPage".into(), id);
    assert!(scene::page(&doc, index, scene::Options::default()).is_err());
}

#[test]
fn empty_glyph_outlines_do_not_expand_export_to_page_origin() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let doc = diagramcraft_vsdx::read(root.join("generated/basic/0001-probe.vsdx")).unwrap();
    let drawing = scene::page(&doc, 0, scene::Options::default()).unwrap();
    assert!(drawing.bounds.x0 > 0.9 && drawing.bounds.y0 > 3.9);
    assert_eq!(drawing.canvas(scene::Options::default()).unwrap().0, 577);
}

#[test]
fn arrow_and_cap_export_dimensions_match_all_generated_witnesses() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    for group in ["arrow", "line"] {
        for entry in std::fs::read_dir(root.join("generated").join(group)).unwrap() {
            let file = entry.unwrap().path();
            if !file.extension().is_some_and(|e| e == "vsdx") {
                continue;
            }
            let doc = diagramcraft_vsdx::read(&file).unwrap();
            let scene = scene::page(&doc, 0, scene::Options::default()).unwrap();
            let (w, h, _) = scene.canvas(scene::Options::default()).unwrap();
            let oracle = raster::decode(&std::fs::read(file.with_extension("png")).unwrap()).unwrap();
            assert_eq!((u32::from(w), u32::from(h)), oracle.dimensions(), "{}", file.display());
        }
    }
}

#[test]
fn observed_probe_and_hairline_edge_profiles() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    for file in ["basic/0001-probe", "master/basic-rect"] {
        let doc = diagramcraft_vsdx::read(root.join(format!("generated/{file}.vsdx"))).unwrap();
        let drawing = scene::page(&doc, 0, scene::Options::default()).unwrap();
        let ours = raster::render(&drawing, scene::Options::default()).unwrap();
        let visio = raster::decode(&std::fs::read(root.join(format!("generated/{file}.png"))).unwrap()).unwrap();
        let own: Vec<_> = (0..8).map(|x| ours.get_pixel(x, 30).0).collect();
        let expected: Vec<_> = (0..8).map(|x| visio.get_pixel(x, 30).0).collect();
        println!("{file} row30 Visio={expected:?} ours={own:?}");
        assert_eq!(own, expected);
    }
}

#[test]
fn basic_drop_theme_extension_uses_its_zero_style_slot() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let doc = diagramcraft_vsdx::read(root.join("ms-shipped/1033/BASIC_U.vssx")).unwrap();
    let themes = diagramcraft_theme::Themes::from_document(&doc).unwrap();
    let mut e = diagramcraft_vsdx::evaluate::Evaluator::new(&doc).unwrap().with_theme_host(&themes);
    assert_eq!(e.evaluate(diagramcraft_vsdx::evaluate::Scope::MasterShape { master: 56, shape: 7 }, "LinePattern").numeric().unwrap(), 1.);
}

#[test]
fn contact_cells_fit_large_masters_and_skip_empty_text_blocks() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    for name in ["CISCONETWORKSHAPES_U", "AZURECOMPUTE_U", "NETLOC_U", "NETSME_M", "NETSME_U"] {
        let doc = diagramcraft_vsdx::read(root.join(format!("ms-shipped/1033/{name}.vssx"))).unwrap();
        for index in 0..if name.starts_with("CISCO") { 1 } else { doc.masters.len() } {
            let drawing = scene::master(&doc, index, scene::Options::default()).unwrap();
            let scale = (216. / (drawing.bounds.width() * 96.).max(1.)).min(160. / (drawing.bounds.height() * 96.).max(1.)).min(2.);
            let (w, h, _) = drawing.canvas(scene::Options { scale, ..Default::default() }).unwrap();
            assert!(w <= 216 && h <= 160, "{name} master {index}: {w}x{h}");
        }
    }
}

#[test]
fn scaled_page_dimensions_are_physical_inches() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let doc = diagramcraft_vsdx::read(root.join("generated/page/metric-scale.vsdx")).unwrap();
    let drawing = scene::page(&doc, 0, scene::Options { crop: false, ..Default::default() }).unwrap();
    assert!((drawing.page_size.0 - 0.085).abs() < 1e-9);
    assert!((drawing.page_size.1 - 0.11).abs() < 1e-9);
}

#[test]
fn composed_text_height_slack_matches_visio_and_preserves_empty_blocks() {
    use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    for (file, scope, expected) in [
        ("generated/basic/0001-probe.vsdx", Scope::PageShape { page: 0, shape: 5 }, 0.2444939358181424),
        ("generated/basic/0001-probe.vsdx", Scope::MasterShape { master: 2, shape: 5 }, 0.2444444444444444),
        ("ms-shipped/1033/AWSANALYTICS_U.vssx", Scope::MasterShape { master: 2, shape: 5 }, 0.15005),
    ] {
        let doc = diagramcraft_vsdx::read(root.join(file)).unwrap();
        let themes = diagramcraft_theme::Themes::from_document(&doc).unwrap();
        let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes).with_text_host(&diagramcraft_text::Host);
        let actual = e.evaluate(scope, "TxtHeight").numeric().unwrap();
        assert!((actual - expected).abs() < 1e-6, "{file} {scope:?}: {actual} != {expected}");
    }
}

#[test]
fn text_metrics_return_scaled_drawing_units() {
    use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    for (name, master, shape, expected) in [("APPL_M", 2, 6, 11.11111111111111), ("APPL_U", 25, 5, 10.66666666666667)] {
        let doc = diagramcraft_vsdx::read(root.join(format!("ms-shipped/1033/{name}.vssx"))).unwrap();
        let themes = diagramcraft_theme::Themes::from_document(&doc).unwrap();
        let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes).with_text_host(&diagramcraft_text::Host);
        let actual = e.evaluate(Scope::MasterShape { master, shape }, "TxtWidth").numeric().unwrap();
        assert!((actual - expected).abs() < 1e-6, "{name}: {actual} != {expected}");
    }
}

#[test]
fn empty_flowchart_text_does_not_recurse_through_wrapping_width() {
    use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let doc = diagramcraft_vsdx::read(root.join("ms-shipped/1033/BASFLO_U.vssx")).unwrap();
    for id in [9, 10, 11, 13] {
        let index = doc.masters.iter().position(|m| m.id == id).unwrap();
        assert!(scene::master(&doc, index, scene::Options::default()).is_ok(), "master{id}");
    }
    let themes = diagramcraft_theme::Themes::from_document(&doc).unwrap();
    let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes).with_text_host(&diagramcraft_text::Host);
    let scope = Scope::MasterShape { master: 11, shape: 4 };
    // Fresh COM observation: _vm/out/dump-shipped/1033/BASFLO_U.VSSX.tsv.
    for (cell, expected) in [("Height", 0.75), ("User.ResizeTxtHeight", 0.75), ("Scratch.Y1", 0.25), ("Scratch.X1", 0.25)] {
        assert_eq!(e.evaluate(scope, cell).numeric().unwrap(), expected, "{cell}");
    }
}

#[test]
fn text_blocks_and_unkerned_lines_match_visio_svg() {
    use diagramcraft_model::{XmlElement, XmlNode};
    fn collect<'a>(e: &'a XmlElement, name: &str, out: &mut Vec<&'a XmlElement>) {
        if e.name == name {
            out.push(e);
        }
        for node in &e.children {
            if let XmlNode::Element(child) = node {
                collect(child, name, out);
            }
        }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let mut count = 0;
    for entry in std::fs::read_dir(root.join("generated/text")).unwrap() {
        let file = entry.unwrap().path();
        if !file.extension().is_some_and(|s| s == "vsdx") {
            continue;
        }
        let doc = diagramcraft_vsdx::read(&file).unwrap();
        // The trusted local SVG fixture declares an external SVG DTD; do not load it.
        let svg = std::fs::read_to_string(file.with_extension("svg")).unwrap();
        let without_dtd = svg.lines().filter(|line| !line.starts_with("<!DOCTYPE svg PUBLIC")).collect::<Vec<_>>().join("\n");
        let xml = diagramcraft_vsdx::parse_xml(without_dtd.as_bytes()).unwrap();
        let mut rects = vec![];
        collect(&xml, "textRect", &mut rects);
        let themes = diagramcraft_theme::Themes::from_document(&doc).unwrap();
        let mut e = diagramcraft_vsdx::evaluate::Evaluator::new(&doc).unwrap().with_theme_host(&themes).with_text_host(&diagramcraft_text::Host);
        let page = doc.pages.first().unwrap();
        assert_eq!(rects.len(), page.shapes.len(), "{}", file.display());
        for (shape, rect) in page.shapes.iter().zip(rects) {
            let block = e.text_block(diagramcraft_vsdx::evaluate::Scope::PageShape { page: page.id, shape: shape.id }).unwrap();
            let w = rect.attr("width").unwrap().parse::<f64>().unwrap();
            let h = rect.attr("height").unwrap().parse::<f64>().unwrap();
            assert!((block.width - w).abs() < 0.01 && (block.height - h).abs() < 0.01, "{} {:?}", file.display(), (block.width, block.height, w, h));
        }
        if file.file_stem().unwrap() == "multiline-wrap" {
            let drawing = scene::page(&doc, 0, scene::Options::default()).unwrap();
            let layout = drawing
                .ops
                .iter()
                .find_map(|op| match op {
                    scene::Op::Text { layout, .. } => Some(layout),
                    _ => None,
                })
                .unwrap();
            let xs = [9.32, 26.84, 20.87, 23.73, 5.32, 5.87, 34.37, 43.40];
            assert_eq!(layout.lines.len(), xs.len());
            for (i, (line, x)) in layout.lines.iter().zip(xs).enumerate() {
                assert!((line.extent.0 - x).abs() < 0.005, "line {i}: {} != {x}", line.extent.0);
                assert!((line.baseline - (73.8 + 21.6 * i as f64)).abs() < 1e-6);
            }
        }
        count += 1;
    }
    assert_eq!(count, 16);
}

#[test]
fn transformed_strokes_preserve_physical_pen_and_fit_export_viewport() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let doc = diagramcraft_vsdx::read(root.join("generated/text/align-1-0.vsdx")).unwrap();
    let drawing = scene::page(&doc, 0, scene::Options::default()).unwrap();
    let actual = raster::render(&drawing, scene::Options::default()).unwrap();
    let expected = raster::decode(&std::fs::read(root.join("generated/text/align-1-0.png")).unwrap()).unwrap();
    for x in (0..4).chain(actual.width() - 5..actual.width()) {
        println!("edge x={x} Visio={:?} ours={:?}", expected.get_pixel(x, 30), actual.get_pixel(x, 30));
        assert_eq!(actual.get_pixel(x, 30), expected.get_pixel(x, 30));
    }
    let doc = diagramcraft_vsdx::read(root.join("generated/group/two-rects-rotated.vsdx")).unwrap();
    let drawing = scene::page(&doc, 0, scene::Options::default()).unwrap();
    let pens: Vec<_> = drawing
        .ops
        .iter()
        .filter_map(|op| match op {
            scene::Op::Stroke { stroke, .. } => Some(stroke.width),
            _ => None,
        })
        .collect();
    assert_eq!(pens.len(), 2);
    assert!(pens.iter().all(|w| (*w - 0.75 / 72.).abs() < 1e-10));
}

#[test]
fn text_metric_grid_and_empty_fields_match_private_com_witnesses() {
    use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    for (file, master, shape, expected) in [
        ("ADO_M.vssx", 2, 45, 0.1417151407877604),
        ("ANNOT_U.vssx", 16, 11, 0.3444962246365018),
        ("BASICCIRCUIT_DIAGRAM_M.vstx", 8, 5, 0.1944949530707465),
        ("HVACCE_M.vssx", 12, 11, 0.2333851135253906),
    ] {
        let doc = diagramcraft_vsdx::read(root.join("ms-shipped/1033").join(file)).unwrap();
        let themes = diagramcraft_theme::Themes::from_document(&doc).unwrap();
        let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes).with_text_host(&diagramcraft_text::Host);
        let actual = e.evaluate(Scope::MasterShape { master, shape }, "TxtHeight").numeric().unwrap();
        assert!((actual - expected).abs() < 1e-12, "{file} master{master} shape{shape}: {actual} != {expected}");
    }
}
