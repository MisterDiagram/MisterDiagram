#[test]
fn generated_render_groups_meet_frozen_ceilings() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let report = diagramcraft_corpus::render::run(&root).unwrap();
    println!("{report}");
    if std::env::var_os("DIAGRAMCRAFT_P2_RENDER_ADVISORY").is_some() {
        println!("P2 human gate pending: render comparison advisory in CI; corpus check remains strict");
    } else {
        assert!(report.passed(), "generated render groups exceed frozen ceilings");
    }
}
