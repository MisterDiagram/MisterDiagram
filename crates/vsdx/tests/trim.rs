use misterdiagram_opc::{PackageOut, RelsOut};
use misterdiagram_sheet::Value;
use misterdiagram_vsdx::{
    evaluate::{Evaluator, Scope},
    import,
};

#[test]
fn imported_trim_formula_recalculates_instead_of_using_saved_text() {
    let mut package = PackageOut::default();
    let mut rels = RelsOut::default();
    rels.add("http://schemas.microsoft.com/visio/2010/relationships/document", "visio/document.xml", false);
    package.rels("", &rels);
    package.xml_part(
        "visio/document.xml",
        "application/vnd.ms-visio.drawing.main+xml",
        "<VisioDocument xmlns='http://schemas.microsoft.com/office/visio/2012/main'><DocumentSheet><Cell N='Input' U='STR' V='  é&#x180E;&#x3000;λ  '/><Cell N='Label' U='STR' V='stale text' F='TRIM(Input)'/></DocumentSheet></VisioDocument>".as_bytes().to_vec(),
    );
    let doc = import(&package.finish().unwrap()).unwrap();
    let mut evaluator = Evaluator::new(&doc).unwrap();
    assert_eq!(evaluator.evaluate(Scope::Document, "Label"), Value::Text("é λ".into()));
}
