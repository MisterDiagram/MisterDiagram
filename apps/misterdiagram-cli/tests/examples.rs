//! The two generator shell scripts are executable CI acceptance, using the built CLI.
use std::{path::PathBuf, process::Command};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn check_layout(path: &std::path::Path, tree: bool) {
    let mut command = Command::new(if cfg!(windows) { "python" } else { "python3" });
    command.arg(root().join("tools/check-layout.py")).arg(path).arg("--cli").arg(env!("CARGO_BIN_EXE_misterdiagram"));
    if tree {
        command.args(["--root", "1", "--fit-page"]);
    }
    let checked = command.output().unwrap();
    assert!(checked.status.success(), "layout checker: {} {}", String::from_utf8_lossy(&checked.stdout), String::from_utf8_lossy(&checked.stderr));
}
/// Find Bash in the installed Git tree rather than the Windows WSL launcher.
/// Git reports its installation's helper directory via --exec-path.
/// https://git-scm.com/docs/git#Documentation/git.txt---exec-pathltpathgt
fn bash() -> PathBuf {
    #[cfg(windows)]
    {
        let result = Command::new("git").arg("--exec-path").output().unwrap();
        assert!(result.status.success(), "git --exec-path failed");
        let directory = PathBuf::from(String::from_utf8(result.stdout).unwrap().trim());
        directory.ancestors().map(|dir| dir.join("bin/bash.exe")).find(|path| path.is_file()).expect("Git for Windows Bash is required")
    }
    #[cfg(not(windows))]
    {
        PathBuf::from("bash")
    }
}
fn shell_path(path: &std::path::Path) -> std::ffi::OsString {
    #[cfg(windows)]
    {
        path.to_str().unwrap().replace('\\', "/").into()
    }
    #[cfg(not(windows))]
    {
        path.as_os_str().to_owned()
    }
}
fn run_output(script: &str, out: &std::path::Path) -> std::process::Output {
    Command::new(bash())
        .arg(shell_path(&root().join("tools/examples").join(script)))
        .arg(shell_path(out))
        .env("MISTERDIAGRAM_CLI", shell_path(std::path::Path::new(env!("CARGO_BIN_EXE_misterdiagram"))))
        .output()
        .unwrap()
}
fn run(script: &str, out: &std::path::Path) {
    let result = run_output(script, out);
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
    let doc = misterdiagram_vsdx::read(out.join("two-boxes-glued.vsdx")).unwrap();
    let options = misterdiagram_render::scene::Options { visio_export: true, ..Default::default() };
    let scene = misterdiagram_render::scene::page(&doc, 0, options).unwrap();
    let (width, height, transform) = scene.canvas(options).unwrap();
    let png = misterdiagram_render::raster::decode(&std::fs::read(out.join("two-boxes-glued.png")).unwrap()).unwrap();
    assert_eq!(png.dimensions(), (u32::from(width), u32::from(height)));
    let point = transform * misterdiagram_geom::Point::new(3.75, 6.75);
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

/// A complete SAN has two glue rows and visible path ink for every ISL.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.connects
#[test]
fn san_script_has_eighty_glues_and_ink_on_every_isl() {
    let out = root().join("target/example-tests/san topology");
    let result = run_output("san-topology.sh", &out);
    check_layout(&out.join("san-topology.vsdx"), false);
    let doc = misterdiagram_vsdx::read(out.join("san-topology.vsdx")).unwrap();
    let page = &doc.pages[0];
    assert_eq!(page.connects.len(), 80, "every ISL requires both glued ends");
    assert!(result.status.success(), "SAN script: {}", String::from_utf8_lossy(&result.stderr));
    let ids = page.connects.iter().map(|c| c.from_sheet).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(ids.len(), 40);
    for id in &ids {
        let cells =
            page.connects.iter().filter(|c| c.from_sheet == *id).map(|c| c.from_cell.as_deref().unwrap()).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(cells, ["BeginX", "EndX"].into_iter().collect());
    }
    let mut e = misterdiagram_vsdx::evaluate::Evaluator::new(&doc).unwrap();
    let ps = misterdiagram_vsdx::evaluate::Scope::Page(page.id);
    assert_eq!(e.evaluate(ps, "PageWidth").numeric().unwrap(), 17.);
    assert_eq!(e.evaluate(ps, "PageHeight").numeric().unwrap(), 11.);
    let switch = doc.masters.iter().find(|m| m.name == "storage.switch").unwrap();
    let switches = page.shapes.iter().filter(|s| s.master == Some(switch.id)).collect::<Vec<_>>();
    assert_eq!(switches.len(), 24);
    let mut switch_bounds = Vec::new();
    for shape in switches {
        let scope = misterdiagram_vsdx::evaluate::Scope::PageShape { page: page.id, shape: shape.id };
        let w = e.evaluate(scope, "Width").numeric().unwrap();
        let h = e.evaluate(scope, "Height").numeric().unwrap();
        let bounds = e.shape_to_page(scope).unwrap().transform_rect_bbox(misterdiagram_geom::Rect::new(0., 0., w, h));
        switch_bounds.push((shape.id, bounds));
        for prop in ["Name", "Fabric", "DomainID", "Model"] {
            assert!(!e.evaluate(scope, &format!("Prop.{prop}")).text().is_empty(), "missing {prop}");
        }
    }
    let options = misterdiagram_render::scene::Options { visio_export: true, ..Default::default() };
    let scene = misterdiagram_render::scene::page(&doc, 0, options).unwrap();
    let (_, _, transform) = scene.canvas(options).unwrap();
    let png = misterdiagram_render::raster::decode(&std::fs::read(out.join("san-topology.png")).unwrap()).unwrap();
    for id in ids {
        let scope = misterdiagram_vsdx::evaluate::Scope::PageShape { page: page.id, shape: id };
        let layers = e.evaluate(scope, "LayerMember").text();
        let fabric = if id < 47 { "0" } else { "1" };
        assert!(layers.split(';').any(|s| s == fabric), "ISL {id} lost fabric layer: {layers}");
        let local = e.shape_to_page(scope).unwrap();
        let mut rows = e
            .resolved_named_cells(scope)
            .into_iter()
            .filter(|n| n.section.is_some_and(|s| s.name == "Geometry") && n.cell.name == "X")
            .filter_map(|n| n.row.filter(|r| !r.deleted).map(|r| r.index.unwrap_or(0)))
            .collect::<Vec<_>>();
        rows.sort();
        rows.dedup();
        let mut points = Vec::new();
        for row in rows {
            let x = e.evaluate(scope, &format!("Geometry1.X{row}")).numeric().unwrap();
            let y = e.evaluate(scope, &format!("Geometry1.Y{row}")).numeric().unwrap();
            points.push(transform * (local * misterdiagram_geom::Point::new(x, y)));
        }
        let mut tested = 0;
        for segment in points.windows(2) {
            let a = segment[0];
            let b = segment[1];
            for (switch, bounds) in &switch_bounds {
                let bounds = transform.transform_rect_bbox(*bounds);
                let mut low = 0f64;
                let mut high = 1f64;
                for (start, end, min, max) in [(a.x, b.x, bounds.x0 + 1e-6, bounds.x1 - 1e-6), (a.y, b.y, bounds.y0 + 1e-6, bounds.y1 - 1e-6)] {
                    if (start - end).abs() < 1e-10 {
                        if start <= min || start >= max {
                            high = -1.;
                        }
                    } else {
                        let u = (min - start) / (end - start);
                        let v = (max - start) / (end - start);
                        low = low.max(u.min(v));
                        high = high.min(u.max(v));
                    }
                }
                assert!(low >= high || high <= 0. || low >= 1., "ISL {id} passes through switch {switch}: {a:?} to {b:?}");
            }
            if (a.x - b.x).hypot(a.y - b.y) < 12. {
                continue;
            }
            let p = misterdiagram_geom::Point::new((a.x + b.x) / 2., (a.y + b.y) / 2.);
            let ink = (-2..=2).any(|dx| {
                (-2..=2).any(|dy| {
                    let x = p.x.round() as i64 + dx;
                    let y = p.y.round() as i64 + dy;
                    if x < 0 || y < 0 || x >= i64::from(png.width()) || y >= i64::from(png.height()) {
                        return false;
                    }
                    let v = png.get_pixel(x as u32, y as u32).0;
                    v[3] > 0 && v[0] < 180 && v[1] < 180 && v[2] < 180
                })
            });
            assert!(ink, "ISL {id} has no ink at segment midpoint {p:?}");
            tested += 1;
        }
        assert!(tested > 0, "ISL {id} has no testable path");
    }
    assert!(std::fs::metadata(out.join("san-topology.svg")).unwrap().len() > 100);
}

/// Rebuild all four Property types through public session commands.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-data-section
#[test]
fn property_rows_script_matches_original_dump() {
    if !root().join("corpus/generated/data/property-rows.vsdx").is_file() {
        eprintln!("skip: private Property witness absent");
        return;
    }
    let out = root().join("target/example-tests/property rows");
    run("property-rows.sh", &out);
    assert_eq!(std::fs::metadata(out.join("property-rows.diff")).unwrap().len(), 0);
}

/// Original Masters.Drop cells and authored text follow the dated canonical-form ruling.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.masters.drop
#[test]
fn from_shape_script_matches_original_dump_and_master_type() {
    let witness = root().join("corpus/generated/master/from-shape.vsdx");
    if !witness.is_file() {
        eprintln!("skip: private master witness absent");
        return;
    }
    let out = root().join("target/example-tests/from shape");
    run("from-shape.sh", &out);
    assert_eq!(std::fs::metadata(out.join("from-shape.diff")).unwrap().len(), 0);
    let actual = misterdiagram_vsdx::read(out.join("from-shape.vsdx")).unwrap();
    let expected = misterdiagram_vsdx::read(witness).unwrap();
    let (a, b) = (&actual.masters[0], &expected.masters[0]);
    for name in ["MasterType", "Prompt", "MatchByName", "IconSize", "IconUpdate", "AlignName", "PatternFlags", "Hidden"] {
        assert_eq!(a.attributes.get(name), b.attributes.get(name), "master attribute {name}");
    }
    assert!(a.icon.is_none(), "generated vendor Icon must not be copied");
    let save = root().join("corpus/_vm/out/accept/p4bc__from-shape/saved.vsdx");
    if save.is_file() {
        let canonical = misterdiagram_vsdx::read(save).unwrap();
        assert_eq!(a.shapes[0].text, canonical.masters[0].shapes[0].text, "authored master text run form");
        assert_eq!(actual.parts["docProps/app.xml"].bytes, canonical.parts["docProps/app.xml"].bytes, "authored app.xml canonical bytes");
    }
    // An unchanged original keeps its original pp and opaque metadata under M3.12.
    let round = misterdiagram_vsdx::import(&misterdiagram_vsdx::write(&expected).unwrap().bytes).unwrap();
    assert_eq!(round.masters[0].shapes[0].text, b.shapes[0].text);
    assert_eq!(round.parts["docProps/app.xml"].bytes, expected.parts["docProps/app.xml"].bytes);
}

/// The published CLI recipe is exact text and its decoded pixels are portable.
#[test]
fn readme_office_network_matches_script_and_pixels() {
    let script = std::fs::read_to_string(root().join("tools/examples/office-network.sh")).unwrap().replace("\r\n", "\n");
    let readme = std::fs::read_to_string(root().join("README.md")).unwrap().replace("\r\n", "\n");
    let block = readme.split("<!-- office-network:start -->\n```bash\n").nth(1).unwrap().split("```\n<!-- office-network:end -->").next().unwrap();
    assert_eq!(block, script, "README office-network block must equal the script");
    let out = root().join("target/example-tests/office network");
    let output = Command::new(bash())
        .current_dir(root())
        .arg(shell_path(&root().join("tools/examples/office-network.sh")))
        .arg(shell_path(&out))
        .arg(shell_path(&out.join("office-network.png")))
        .env("MISTERDIAGRAM_CLI", shell_path(std::path::Path::new(env!("CARGO_BIN_EXE_misterdiagram"))))
        .output()
        .unwrap();
    assert!(output.status.success(), "office network: {} {}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    check_layout(&out.join("office-network.vsdx"), true);
    let actual = misterdiagram_render::raster::decode(&std::fs::read(out.join("office-network.png")).unwrap()).unwrap();
    let expected = misterdiagram_render::raster::decode(&std::fs::read(root().join("docs/images/office-network.png")).unwrap()).unwrap();
    assert_eq!(actual.dimensions(), expected.dimensions(), "office PNG dimensions on {}", std::env::consts::OS);
    let differences = actual.pixels().zip(expected.pixels()).filter(|(a, b)| a != b).count();
    assert_eq!(differences, 0, "office PNG differing pixels on {} {}", std::env::consts::OS, std::env::consts::ARCH);
    let doc = misterdiagram_vsdx::read(out.join("office-network.vsdx")).unwrap();
    assert_eq!(doc.pages[0].shapes.len(), 19);
    assert_eq!(doc.pages[0].connects.len(), 18);
}
