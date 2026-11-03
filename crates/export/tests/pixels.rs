use misterdiagram_model::skeleton;
use misterdiagram_render::{
    raster,
    scene::{self, Options, SceneBudget, evaluated},
};

#[test]
fn output_sections_drive_raster_instead_of_replaying_source_operations() {
    let mut doc = skeleton::document(false);
    doc.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 2., 1., "Original text").unwrap());
    // A closed path suppresses saved arrows. Losing its close changes the picture.
    doc.pages[0].shapes[0].sheet.cells.push(misterdiagram_model::Cell { name: "EndArrow".into(), value: Some("2".into()), ..Default::default() });
    let options = Options::default();
    let source = scene::page(&doc, 0, options).unwrap();
    let evaluated = evaluated::page(&doc, 0, options, &mut SceneBudget::default()).unwrap();
    let model = misterdiagram_export::model::Page::from_evaluated(evaluated).unwrap();
    assert!(model.substitutions.is_empty());
    assert_eq!(raster::render(&source, options).unwrap(), raster::render(&model.raster_scene().unwrap(), options).unwrap());
}
#[test]
fn every_generated_page_has_exact_model_pixels_or_named_substitutions() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/generated");
    if !root.exists() {
        eprintln!("private generated corpus is absent");
        return;
    }
    let report = misterdiagram_export::qa::generated(&root).unwrap();
    println!("{report}");
    assert_eq!(report.files, 230);
    assert!(report.passed(), "{report}");
}

#[test]
fn foreign_pixels_are_serialized_and_deduplicated() {
    use kurbo::{Affine, Rect, Shape as _};
    use scene::{Op, Scene};
    use std::sync::Arc;
    let mut doc = skeleton::document(false);
    doc.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap());
    let mut page = evaluated::page(&doc, 0, Options::default(), &mut SceneBudget::default()).unwrap();
    let pixels = Arc::new(image::RgbaImage::from_pixel(8, 8, image::Rgba([12, 34, 56, 255])));
    for (index, offset) in [1., 4.].into_iter().enumerate() {
        let mut node = page.shapes[0].clone();
        node.id = index as u32 + 2;
        node.scope = misterdiagram_vsdx::evaluate::Scope::PageShape { page: 0, shape: node.id };
        node.sections.clear();
        node.paint = None;
        node.line = None;
        node.stroke = None;
        node.drawing = Scene {
            ops: vec![
                Op::Clip { path: Rect::new(offset + 0.1, 2.3, offset + 0.7, 2.9).to_path(1e-5), even_odd: false },
                Op::Image { pixels: pixels.clone(), transform: Affine::new([0.1, 0., 0., -0.1, offset, 3.]) },
                Op::Pop,
            ],
            warnings: vec![],
            pdf_strokes: Default::default(),
            bounds: Rect::ZERO,
            page_size: page.scene.page_size,
            fit_empty: false,
        };
        // Already sampled image, no foreign projection left to flatten.
        node.content_start = 0;
        node.foreign_end = 0;
        page.shapes.push(node);
    }
    page.shapes.remove(0);
    let output = misterdiagram_export::render(vec![page], &[misterdiagram_color::Rgba::BLACK; 12], Options::default()).unwrap();
    let pkg = misterdiagram_opc::Package::open(&output.bytes).unwrap();
    let slide = pkg.xml("ppt/slides/slide1.xml").unwrap();
    let tree = slide.root.path(&["cSld", "spTree"]).unwrap();
    let pictures: Vec<_> = tree.children_named("pic").collect();
    assert_eq!(pictures.len(), 2);
    for picture in &pictures {
        let clip = picture.path(&["spPr", "custGeom", "pathLst", "path"]).unwrap();
        assert_eq!(clip.attr("stroke"), Some("0"));
        assert_eq!(clip.attr("fill"), Some("norm"));
        assert!(clip.child("close").is_some());
    }
    let rels = pkg.xml("ppt/slides/_rels/slide1.xml.rels").unwrap();
    let targets: Vec<_> = rels
        .root
        .children_named("Relationship")
        .filter(|r| r.attr("Type").is_some_and(|t| t.ends_with("/image")))
        .map(|r| r.attr("Target").unwrap())
        .collect();
    assert_eq!(targets.len(), 1);
    assert_eq!(pictures[0].path(&["blipFill", "blip"]).unwrap().attr("embed"), pictures[1].path(&["blipFill", "blip"]).unwrap().attr("embed"));
    let decoded = misterdiagram_render::raster::decode(pkg.get("ppt/media/image1.png").unwrap()).unwrap();
    assert_eq!(&decoded, pixels.as_ref());
    assert!(pkg.get("ppt/media/image2.png").is_none());
}

#[test]
fn mcp_drawing_models_match_the_renderer() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/_state/brief64");
    for relative in ["mcp-office/office-network.vsdx", "mcp-san/p5a-san-topology.vsdx", "mcp-fabric/fabric-page.vsdx"] {
        let path = root.join(relative);
        if !path.exists() {
            eprintln!("private MCP witness absent: {}", path.display());
            continue;
        }
        let report = misterdiagram_export::qa::file(&path).unwrap();
        println!("{report}");
        assert!(report.passed(), "{report}");
    }
}

#[test]
fn cached_metafile_frames_keep_the_original_clip_compositing() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../diagramcraft-corpus/field/oss-2026-10-22/files");
    for file in [
        "asposejava__Examples__src__main__resources__VisioActiveXControls__Drawing1.vsdx",
        "asposejava__Examples__src__main__resources__VisioActiveXControls__RetrieveActiveXControl_out.vsdx",
    ] {
        let path = root.join(file);
        if !path.exists() {
            eprintln!("private cached metafile witness absent");
            continue;
        }
        let report = misterdiagram_export::qa::file(&path).unwrap();
        assert!(report.passed(), "{report}");
        assert_eq!(report.identical, report.pages);
        assert!(report.excluded.is_empty());
    }
}
