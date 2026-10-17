use misterdiagram_render::{
    raster,
    scene::{self, Options},
};
use misterdiagram_vsdx::evaluate::Evaluator;
#[test]
fn prepared_master_index_matches_independent_scenes_in_any_order() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ absent");
        return;
    }
    let doc = misterdiagram_vsdx::read(root.join("ms-shipped/1033/BASIC_U.vssx")).unwrap();
    let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
    let mut evaluator = Evaluator::new(&doc).unwrap().with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    for i in [0, 10, 2, 0] {
        let independent = scene::master(&doc, i, Options::default()).unwrap();
        let reused = scene::master_prepared(&mut evaluator, i, Options::default()).unwrap();
        assert_eq!(independent.bounds, reused.bounds);
        assert_eq!(independent.warnings, reused.warnings);
        let options = Options { scale: 0.5, ..Default::default() };
        assert_eq!(raster::render(&independent, options).unwrap(), raster::render(&reused, options).unwrap());
    }
}

#[test]
fn prepared_render_rejects_deferred_text_metrics() {
    let doc = misterdiagram_model::skeleton::document(false);
    let mut e = Evaluator::new(&doc).unwrap().with_deferred_text_metrics();
    assert!(scene::page_prepared(&mut e, 0, Options::default()).is_err());
    assert!(scene::master_prepared(&mut e, 0, Options::default()).is_err());
}
