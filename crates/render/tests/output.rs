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
            let scene = scene::page(&doc, 0, scene::Options::default())?;
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
    let full = scene::page(&doc, index, scene::Options::default()).unwrap();
    doc.pages.get_mut(index).unwrap().attributes.remove("BackPage");
    let foreground = scene::page(&doc, index, scene::Options::default()).unwrap();
    assert!(full.ops.len() > foreground.ops.len());
    let id = doc.pages.get(index).unwrap().id.to_string();
    doc.pages.get_mut(index).unwrap().attributes.insert("BackPage".into(), id);
    assert!(scene::page(&doc, index, scene::Options::default()).is_err());
}
