//! First measurements, intentionally not a performance gate.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use misterdiagram_model::{Cell, Document, Page, Shape, Sheet};
    use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
    use std::{path::Path, time::Instant};
    #[test]
    #[ignore = "measurement only; run explicitly when recording perf/budgets.toml"]
    fn measure_recalc() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        if !root.join("corpus").exists() {
            eprintln!("local corpus absent: recalc measurement skipped");
            return;
        }
        let path = root.join("corpus/ms-shipped/1033/WFDEP_M.vssx");
        assert!(path.exists(), "largest local stencil required");
        let start = Instant::now();
        let doc = misterdiagram_vsdx::read(path).unwrap();
        let read = start.elapsed();
        let start = Instant::now();
        let mut evaluator = Evaluator::new(&doc).unwrap();
        let index = start.elapsed();
        let cells: Vec<_> = evaluator.scopes().flat_map(|scope| evaluator.named_cells(scope).iter().map(move |c| (scope, c.name.clone()))).collect();
        let start = Instant::now();
        let mut errors = 0;
        for (scope, name) in &cells {
            if evaluator.evaluate(*scope, name).error().is_some() {
                errors += 1;
            }
        }
        println!(
            "largest_stencil=WFDEP_M.vssx uncompressed_bytes=7889047 cells={} errors={} read_ms={} index_ms={} evaluate_ms={}",
            cells.len(),
            errors,
            read.as_millis(),
            index.as_millis(),
            start.elapsed().as_millis()
        );
        let cell = |name: &str, formula: &str| Cell { name: name.into(), formula: Some(formula.into()), ..Default::default() };
        let doc = Document {
            document_sheet: Sheet { cells: vec![cell("Scale", "1")], ..Default::default() },
            pages: vec![Page {
                id: 0,
                shapes: (1..=500)
                    .map(|id| Shape {
                        id,
                        sheet: Sheet { cells: vec![cell("Width", "TheDoc!Scale*2"), cell("Height", "Width*0.5")], ..Default::default() },
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let start = Instant::now();
        let mut evaluator = Evaluator::new(&doc).unwrap();
        for shape in 1..=500 {
            assert_eq!(evaluator.evaluate(Scope::PageShape { page: 0, shape }, "Height").numeric().unwrap(), 1.);
        }
        let initial = start.elapsed();
        evaluator.set_formula(Scope::Document, "Scale", "3").unwrap();
        let start = Instant::now();
        let changed = evaluator.recalculate_dirty();
        assert_eq!(changed.len(), 1001);
        assert!(changed.iter().all(|c| c.2.error().is_none()));
        println!("synthetic_shapes=500 initial_ms={} dirty_cells={} recalc_ms={}", initial.as_millis(), changed.len(), start.elapsed().as_millis());
    }
}
