//! P4a cells/geometry acceptance; per-pair pixels are informational under
//! Charles's 2026-10-15 ruling until the connector stroke rendering gap closes.
//! Run: cargo run -p misterdiagram-corpus --example authoring_routes
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use misterdiagram_engine::Session;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::json;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    let out = root.join("_state/m4");
    std::fs::create_dir_all(&out)?;
    let mut over_ceiling = Vec::new();
    for name in ["dynamic-glue", "move-before", "move-after", "static-glue"] {
        let path = root.join(format!("generated/connector/{name}.vsdx"));
        let original = misterdiagram_vsdx::read(&path)?;
        let mut session = Session::new();
        session.execute("file.open", &json!({"path":path}))?;
        session.execute("route.reroute", &json!({"id":3}))?;
        let scope = Scope::PageShape { page: original.pages.first().ok_or("original page missing")?.id, shape: 3 };
        let mut oracle = Evaluator::new(&original).map_err(|e| e.to_string())?;
        let mut actual = Evaluator::new(session.document()).map_err(|e| e.to_string())?;
        let mut diff = String::new();
        let names = oracle
            .resolved_named_cells(scope)
            .into_iter()
            .filter(|n| {
                n.name.starts_with("Geometry")
                    || matches!(
                        n.name.as_str(),
                        "PinX"
                            | "PinY"
                            | "Width"
                            | "Height"
                            | "LocPinX"
                            | "LocPinY"
                            | "Angle"
                            | "FlipX"
                            | "FlipY"
                            | "BeginX"
                            | "BeginY"
                            | "EndX"
                            | "EndY"
                    )
            })
            .map(|n| n.name)
            .collect::<Vec<_>>();
        for cell in names {
            let a = actual.evaluate(scope, &cell);
            let b = oracle.evaluate(scope, &cell);
            let matches = match (a.numeric(), b.numeric()) {
                (Ok(a), Ok(b)) => (a - b).abs() <= 1e-6,
                _ => a == b,
            };
            if !matches {
                diff.push_str(&format!("{cell}\t{}\t{}\n", a.text(), b.text()));
            }
        }
        std::fs::write(out.join(format!("route-{name}.diff")), &diff)?;
        if !diff.is_empty() {
            return Err(format!("{name}: {diff}").into());
        }
        session.execute("file.save", &json!({"path":out.join(format!("route-{name}.vsdx"))}))?;
        let options = misterdiagram_render::scene::Options::default();
        let scene = misterdiagram_render::scene::page(session.document(), 0, options)?;
        let rendered = misterdiagram_render::raster::render(&scene, options)?;
        rendered.save(out.join(format!("route-{name}.png")))?;
        let expected = misterdiagram_render::raster::decode(&std::fs::read(path.with_extension("png"))?)?;
        let (mad, fraction) = misterdiagram_corpus::render::compare(&expected, &rendered)?;
        let result = misterdiagram_corpus::render::FileResult {
            file: name.into(),
            expected: expected.dimensions(),
            actual: Some(rendered.dimensions()),
            mad: Some(mad),
            differing: Some(fraction),
            error: None,
        };
        println!(
            "P4a route {name}: size={}x{} expected={}x{} MAD={mad:.6} over16={:.6}% informational=true",
            rendered.width(),
            rendered.height(),
            expected.width(),
            expected.height(),
            fraction * 100.
        );
        if result.over_ceiling(false) {
            over_ceiling.push(name);
        }
    }
    if !over_ceiling.is_empty() {
        println!(
            "P4a informational per-pair connector ceilings exceeded: {over_ceiling:?}; cells/geometry agree, corpus group ceiling remains required"
        );
    }
    Ok(())
}
