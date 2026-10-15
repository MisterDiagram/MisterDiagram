//! Compare recomputed route cells against private Visio witnesses.
//! https://learn.microsoft.com/en-us/office/vba/api/visio.shape.reroute
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use diagramcraft_engine::Session;
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use serde_json::json;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::var_os("DIAGRAMCRAFT_CORPUS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus"));
    let out = root.join("_state/m4");
    std::fs::create_dir_all(&out)?;
    let mut files = std::fs::read_dir(root.join("_vm/out/make-corpus/route"))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|v| v == "vsdx"))
        .collect::<Vec<_>>();
    files.extend(
        ["obstacle", "line-jumps", "dynamic-glue", "move-before", "move-after", "static-glue"]
            .map(|n| root.join(format!("generated/connector/{n}.vsdx"))),
    );
    files.sort();
    let mut failed = 0;
    for path in files {
        let name = path.file_stem().and_then(|n| n.to_str()).ok_or("witness filename")?;
        let original = diagramcraft_vsdx::read(&path)?;
        let mut oracle = Evaluator::new(&original).map_err(|e| e.to_string())?;
        let ids = original.pages.first().ok_or("page")?.connects.iter().map(|c| c.from_sheet).collect::<std::collections::BTreeSet<_>>();
        let mut s = Session::new();
        s.execute("file.open", &json!({"path":path}))?;
        let result = s.execute("route.reroute", &json!({}));
        let mut diff = String::new();
        if let Err(e) = result {
            diff.push_str(&format!("reroute error: {e}\n"));
        }
        let mut actual = Evaluator::new(s.document()).map_err(|e| e.to_string())?;
        for id in ids {
            let scope = Scope::PageShape { page: original.pages.first().ok_or("page")?.id, shape: id };
            let names = oracle
                .resolved_named_cells(scope)
                .into_iter()
                .chain(actual.resolved_named_cells(scope))
                .map(|n| n.name)
                .filter(|n| {
                    n.starts_with("Geometry")
                        || matches!(
                            n.as_str(),
                            "BeginX"
                                | "BeginY"
                                | "EndX"
                                | "EndY"
                                | "PinX"
                                | "PinY"
                                | "Width"
                                | "Height"
                                | "LocPinX"
                                | "LocPinY"
                                | "Angle"
                                | "FlipX"
                                | "FlipY"
                        )
                })
                .collect::<std::collections::BTreeSet<_>>();
            for cell in names {
                let a = actual.evaluate(scope, &cell);
                let b = oracle.evaluate(scope, &cell);
                let eq = match (a.numeric(), b.numeric()) {
                    (Ok(a), Ok(b)) => (a - b).abs() <= 1e-6,
                    _ => a == b,
                };
                if !eq {
                    diff.push_str(&format!("{id}\t{cell}\t{}\t{}\n", a.text(), b.text()));
                }
            }
        }
        std::fs::write(out.join(format!("route-{name}.diff")), &diff)?;
        println!("{name}: {} differences", diff.lines().count());
        if !diff.is_empty() {
            failed += 1;
        }
        s.execute("file.save", &json!({"path":out.join(format!("route-{name}.vsdx"))}))?;
    }
    if failed > 0 {
        return Err(format!("{failed} unmatched route witnesses").into());
    }
    Ok(())
}
