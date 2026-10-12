//! The quick local corpus gate keeps the same exclusions and floors as the full CLI run.
use std::path::Path;
#[test]
fn quick_corpus_meets_frozen_floors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let report = diagramcraft_corpus::run(&root.join("corpus"), true).unwrap();
    println!("{report}");
    assert_eq!(report.shipped_packages, 50, "the CI sample must include exactly 50 shipped packages");
    assert!(report.passed(), "{report}");
}
