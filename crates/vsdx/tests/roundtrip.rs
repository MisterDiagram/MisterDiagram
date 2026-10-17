use std::path::{Path, PathBuf};
fn walk(path: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let ty = entry.file_type().unwrap();
        assert!(!ty.is_symlink());
        let p = entry.path();
        if ty.is_dir() {
            walk(&p, out);
        } else if p.extension().and_then(|s| s.to_str()).is_some_and(|s| matches!(s.to_ascii_lowercase().as_str(), "vsdx" | "vssx" | "vstx" | "vsdm"))
        {
            out.push(p);
        }
    }
}
#[test]
fn every_corpus_package_roundtrips_every_strict_literal_cell() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let mut files = vec![];
    for dir in ["generated", "blank", "ms-shipped"] {
        walk(&root.join(dir), &mut files);
    }
    files.sort();
    assert!(!files.is_empty());
    let mut total = misterdiagram_vsdx::roundtrip::Report::default();
    for (i, file) in files.iter().enumerate() {
        let before = misterdiagram_vsdx::read(file).unwrap();
        let theme = misterdiagram_theme::Themes::from_document(&before).unwrap();
        let written = misterdiagram_vsdx::write_with_hosts(&before, Some(&theme), Some(&misterdiagram_text::Host))
            .unwrap_or_else(|e| panic!("{}: {e}", file.display()));
        let after = misterdiagram_vsdx::import(&written.bytes).unwrap();
        let theme_after = misterdiagram_theme::Themes::from_document(&after).unwrap();
        total.merge(
            misterdiagram_vsdx::roundtrip::compare(&before, &after, Some(&theme), Some(&theme_after), Some(&misterdiagram_text::Host))
                .unwrap_or_else(|e| panic!("{}: {e}", file.display())),
        );
        if (i + 1) % 100 == 0 {
            eprintln!("roundtrip progress: {}/{}", i + 1, files.len());
        }
    }
    eprintln!(
        "roundtrip cells: packages={} passed={}/{} rate={:.6}% floor={:.6}%",
        files.len(),
        total.matched,
        total.total,
        total.percent(),
        misterdiagram_vsdx::roundtrip::FLOOR
    );
    eprintln!(
        "roundtrip evaluation-only diagnostic: passed={}/{} rate={:.6}%",
        total.evaluation_only_matched,
        total.total,
        100. * total.evaluation_only_matched as f64 / total.total.max(1) as f64
    );
    let mut failures: Vec<_> = total.failures.iter().collect();
    failures.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (name, count) in failures {
        eprintln!("roundtrip mismatch {name}: {count}");
    }
    assert!(total.passed());
}

#[test]
fn passive_actions_and_last_valid_error_references() {
    use misterdiagram_model::{Cell, Row, Section};
    use misterdiagram_sheet::Value;
    use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
    let mut d = misterdiagram_model::skeleton::document(false);
    d.document_sheet.sections.push(Section {
        name: "Actions".into(),
        rows: vec![Row {
            name: Some("Toggle".into()),
            cells: vec![Cell {
                name: "Action".into(),
                formula: Some("NOT(Actions.Toggle.Action)".into()),
                value: Some("0".into()),
                ..Cell::default()
            }],
            ..Row::default()
        }],
        ..Section::default()
    });
    for (name, value, formula, error) in
        [("Bad", "1.3336", "1/0", Some("#DIV/0!")), ("Reference", "1.3336", "Bad", None), ("IsBad", "1", "ISERR(Bad)", None)]
    {
        d.document_sheet.cells.push(Cell {
            name: name.into(),
            value: Some(value.into()),
            formula: Some(formula.into()),
            error: error.map(Into::into),
            ..Cell::default()
        });
    }
    let mut e = Evaluator::new(&d).unwrap();
    assert_eq!(e.evaluate(Scope::Document, "Actions.Toggle.Action"), Value::Number(0.));
    assert_eq!(e.evaluate(Scope::Document, "Reference"), Value::Number(1.3336));
    assert_eq!(e.evaluate(Scope::Document, "IsBad"), Value::Bool(true));
    assert!(e.evaluate(Scope::Document, "Bad").error().is_some());
}
