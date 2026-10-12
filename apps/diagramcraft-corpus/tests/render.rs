#[test]
fn generated_render_groups_meet_frozen_ceilings() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let report = diagramcraft_corpus::render::run(&root).unwrap();
    println!("{report}");
    assert!(report.passed(), "generated render groups exceed frozen ceilings");
}
