//! The two generator shell scripts are executable CI acceptance, using the built CLI.
use std::{path::PathBuf, process::Command};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn run(script: &str, out: &std::path::Path) {
    let result = Command::new("bash")
        .arg(root().join("tools/examples").join(script))
        .arg(out)
        .env("DIAGRAMCRAFT_CLI", env!("CARGO_BIN_EXE_diagramcraft-cli"))
        .output()
        .unwrap();
    assert!(result.status.success(), "{script}: {} {}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
}
#[test]
fn probe_script_matches_all_eligible_oracle_cells() {
    if !root().join("corpus").exists() {
        eprintln!("skip: private corpus absent");
        return;
    }
    let out = root().join("target/example-tests/probe with spaces");
    run("probe-rebuild.sh", &out);
    assert!(std::fs::read(out.join("probe-rebuild.diff")).unwrap().is_empty());
}
#[test]
fn static_script_matches_sheet_and_connects() {
    if !root().join("corpus").exists() {
        eprintln!("skip: private corpus absent");
        return;
    }
    let out = root().join("target/example-tests/static");
    run("static-glue-rebuild.sh", &out);
    assert!(std::fs::read(out.join("static-glue.diff")).unwrap().is_empty());
}
#[test]
fn two_boxes_script_has_ink_between_boxes_with_visio_extent() {
    let out = root().join("target/example-tests/two boxes");
    run("two-boxes-glued.sh", &out);
    let doc = diagramcraft_vsdx::read(out.join("two-boxes-glued.vsdx")).unwrap();
    let options = diagramcraft_render::scene::Options { visio_export: true, ..Default::default() };
    let scene = diagramcraft_render::scene::page(&doc, 0, options).unwrap();
    let (width, height, transform) = scene.canvas(options).unwrap();
    let png = diagramcraft_render::raster::decode(&std::fs::read(out.join("two-boxes-glued.png")).unwrap()).unwrap();
    assert_eq!(png.dimensions(), (u32::from(width), u32::from(height)));
    let point = transform * diagramcraft_geom::Point::new(3.75, 6.75);
    let x = point.x.round() as u32;
    let y = point.y.round() as u32;
    let ink = (x.saturating_sub(20)..x + 20)
        .flat_map(|x| (y.saturating_sub(2)..y + 3).map(move |y| (x, y)))
        .filter(|(x, y)| {
            let p = png.get_pixel(*x, *y).0;
            p[3] > 0 && p[0] < 128 && p[1] < 128 && p[2] < 128
        })
        .count();
    assert!(ink >= 30, "connector ink missing between boxes: {ink}");
}
#[test]
fn acceptance_checker_rejects_corrupt_package_without_panic() {
    let out = root().join("target/example-tests/hostile");
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("static-glue.vsdx"), b"not a ZIP").unwrap();
    let result = Command::new("python3")
        .arg("-c")
        .arg("import runpy,sys,zipfile\nm=runpy.run_path(sys.argv[1])\ntry: m['connections'](sys.argv[2])\nexcept zipfile.BadZipFile: sys.exit(1)\nsys.exit(0)")
        .arg(root().join("tools/examples/check.py"))
        .arg(out.join("static-glue.vsdx"))
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(!String::from_utf8_lossy(&result.stderr).contains("Traceback"));
}
