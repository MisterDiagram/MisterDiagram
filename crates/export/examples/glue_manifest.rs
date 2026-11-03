//! Test-build intent manifest for independent connector and picture readback.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use misterdiagram_render::scene::{EvaluatedShape, Op, Options, SceneBudget, evaluated};
use std::io::{BufWriter, Write};
fn pictures(nodes: &[EvaluatedShape], depth: usize) -> Result<usize, String> {
    if depth > 128 {
        return Err("picture manifest depth limit".into());
    }
    let mut count = 0usize;
    for n in nodes {
        if !n.hidden_own {
            count = count
                .checked_add(n.drawing.ops.iter().filter(|op| matches!(op, Op::Image { .. } | Op::RasterImage { .. })).count())
                .ok_or("picture count limit")?;
        }
        count = count.checked_add(pictures(&n.children, depth + 1)?).ok_or("picture count limit")?;
    }
    Ok(count)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let [input, output] = args.as_slice() else {
        return Err("usage: glue_manifest IN.vsdx OUT.tsv".into());
    };
    let doc = misterdiagram_vsdx::read(input)?;
    let options = Options { visio_export: true, cached_fallback: true, ..Default::default() };
    let mut budget = SceneBudget::default();
    let pages = evaluated::foreground_pages(&doc, options, &mut budget)?;
    let mut w = BufWriter::new(std::fs::File::create(output)?);
    writeln!(w, "S\t{}", pages.len())?;
    for (i, page) in pages.into_iter().enumerate() {
        let expected = page.connects.len();
        let model = misterdiagram_export::model::Page::with_options(page, options, &mut budget)?;
        writeln!(w, "P\t{}\t{}\t{}", i + 1, pictures(&model.shapes, 0)?, expected)?;
        let mut count = 0;
        for (scope, route) in &model.connections.routes {
            let id = model.connections.ids.get(scope).ok_or("connector id")?;
            for (end, a) in [(false, &route.begin), (true, &route.end)] {
                if let Some(a) = a {
                    let target = model.connections.targets.get(&a.target).ok_or("target id")?;
                    writeln!(w, "C\t{}\t{id}\t{}\t{target}\t{}", i + 1, u8::from(end), a.index)?;
                    count += 1;
                }
            }
        }
        if count != expected {
            return Err("Connects-table attachment count differs".into());
        }
    }
    w.flush()?;
    Ok(())
}
