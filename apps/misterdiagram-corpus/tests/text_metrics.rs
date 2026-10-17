//! Exact corpus witness for the empty text line omitted by the initial text adapter.
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
#[test]
fn probe_connector_empty_text_retains_minimum_line_height() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("local corpus absent: text metric witness skipped");
        return;
    }
    let doc = misterdiagram_vsdx::read(root.join("generated/basic/0001-probe.vsdx")).unwrap();
    let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
    let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    let actual = e.evaluate(Scope::MasterShape { master: 2, shape: 5 }, "TxtHeight").numeric().unwrap();
    assert!((actual - 0.2444444444444444).abs() < 1e-6, "{actual}");
}
