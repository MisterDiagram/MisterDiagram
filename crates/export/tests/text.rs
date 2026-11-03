//! Original text serialization, placement and saved-text fallback witnesses.
use misterdiagram_model::{Cell, Document, Row, Section, Shape, skeleton};
use misterdiagram_opc::Package;
use misterdiagram_render::scene::{self, Options, SceneBudget};
fn cell(n: &str, v: &str) -> Cell {
    Cell { name: n.into(), value: Some(v.into()), ..Default::default() }
}
fn set(s: &mut Shape, n: &str, v: &str) {
    s.sheet.cells.retain(|c| c.name != n);
    s.sheet.cells.push(cell(n, v));
}
fn section(s: &mut Shape, n: &str, rows: Vec<Row>) {
    s.sheet.sections.retain(|x| x.name != n);
    s.sheet.sections.push(Section { name: n.into(), rows, ..Default::default() });
}
fn row(index: u32, cells: Vec<Cell>) -> Row {
    Row { index: Some(index), cells, ..Default::default() }
}
fn document(text: &str) -> Document {
    let mut d = skeleton::document(false);
    d.pages[0].shapes.push(skeleton::rectangle(1, 3., 4., 2., 1., text).unwrap());
    d
}
fn export(d: &Document) -> (misterdiagram_export::Export, Package) {
    let out = misterdiagram_export::document(d, None, Options { page_canvas: true, cached_fallback: true, ..Default::default() }).unwrap();
    let pkg = Package::open(&out.bytes).unwrap();
    (out, pkg)
}
#[test]
fn native_paragraphs_runs_fields_and_soft_breaks_keep_evaluator_boundaries() {
    let mut d = document("");
    d.face_names.insert(1, "Arial".into());
    let s = &mut d.pages[0].shapes[0];
    s.text = Some(misterdiagram_vsdx::parse_xml("<Text><cp IX='0'/>A<cp IX='1'/>B\n\nC\u{2028}D<fld IX='0'>saved</fld></Text>".as_bytes()).unwrap());
    section(
        s,
        "Character",
        vec![
            row(0, vec![cell("Font", "1"), cell("Style", "1"), cell("Size", "0.16666666666666666")]),
            row(1, vec![cell("Font", "1"), cell("Style", "2"), cell("Size", "0.25")]),
        ],
    );
    section(
        s,
        "Field",
        vec![row(
            0,
            vec![
                Cell { name: "Value".into(), value: Some("2".into()), formula: Some("1+1".into()), ..Default::default() },
                Cell { name: "Format".into(), value: Some("esc(0)".into()), units: Some("STR".into()), ..Default::default() },
            ],
        )],
    );
    let (_, p) = export(&d);
    let slide = p.xml("ppt/slides/slide1.xml").unwrap();
    let body = slide.root.find("txBody").unwrap();
    let paras: Vec<_> = body.children_named("p").collect();
    assert_eq!(paras.len(), 3);
    assert_eq!(paras[0].children_named("r").count(), 2);
    let runs: Vec<_> = paras[0].children_named("r").collect();
    assert_eq!(runs[0].child("rPr").unwrap().attr("b"), Some("1"));
    assert_eq!(runs[0].child("rPr").unwrap().attr("i"), Some("0"));
    assert_eq!(runs[1].child("rPr").unwrap().attr("b"), Some("0"));
    assert_eq!(runs[1].child("rPr").unwrap().attr("i"), Some("1"));
    assert_eq!(runs[0].path(&["rPr", "latin"]).unwrap().attr("typeface"), Some("Arial"));
    assert_eq!(paras[2].children_named("br").count(), 1);
    assert_eq!(paras[2].children_named("r").last().unwrap().child("t").unwrap().text(), "2");
    assert!(body.find("fld").is_none());
    assert!(slide.root.find("schemeClr").is_none());
    assert!(slide.root.find("style").is_none());
}
#[test]
fn explicit_margins_alignment_tabs_bullets_and_character_properties() {
    let mut d = document("Marked text");
    let s = &mut d.pages[0].shapes[0];
    for (n, v) in [("LeftMargin", "0.1"), ("RightMargin", "0.2"), ("TopMargin", "0.05"), ("BottomMargin", "0.07"), ("VerticalAlign", "2")] {
        set(s, n, v);
    }
    section(
        s,
        "Character",
        vec![row(
            0,
            vec![
                cell("Style", "7"),
                cell("Size", "0.25"),
                cell("Strikethru", "1"),
                cell("Pos", "1"),
                cell("Letterspace", "0.01"),
                cell("Color", "#123456"),
            ],
        )],
    );
    section(
        s,
        "Paragraph",
        vec![row(
            0,
            vec![
                cell("HorzAlign", "2"),
                cell("IndLeft", "0.15"),
                cell("IndFirst", "-0.05"),
                cell("SpLine", "-1.2"),
                cell("Bullet", "1"),
                cell("TextPosAfterBullet", "0.25"),
            ],
        )],
    );
    section(s, "Tabs", vec![row(0, vec![cell("Position0", "0.5"), cell("Alignment0", "1")])]);
    let (_, pkg) = export(&d);
    let xml = pkg.xml("ppt/slides/slide1.xml").unwrap();
    let body = xml.root.find("txBody").unwrap();
    let bp = body.child("bodyPr").unwrap();
    assert_eq!(bp.attr("lIns"), Some("91440"));
    assert_eq!(bp.attr("rIns"), Some("182880"));
    assert_eq!(bp.attr("anchor"), Some("b"));
    assert_eq!(bp.attr("wrap"), Some("square"));
    assert!(bp.child("noAutofit").is_some());
    let pp = body.find("pPr").unwrap();
    assert_eq!(pp.attr("algn"), Some("r"));
    assert_eq!(pp.attr("marL"), Some("137160"));
    assert_eq!(pp.attr("indent"), Some("-45720"));
    assert_eq!(pp.find("spcPts").unwrap().attr("val"), Some("2160"));
    assert_eq!(pp.child("buChar").unwrap().attr("char"), Some("•"));
    assert!(pp.find("tab").is_some());
    let r = body.find("rPr").unwrap();
    assert_eq!(r.attr("sz"), Some("1800"));
    assert_eq!(r.attr("u"), Some("sng"));
    assert_eq!(r.attr("strike"), Some("sngStrike"));
    assert_eq!(r.attr("baseline"), Some("30000"));
    assert_eq!(r.attr("spc"), Some("72"));
    assert_eq!(r.find("srgbClr").unwrap().attr("val"), Some("123456"));
}
#[test]
fn saved_text_rectangle_is_not_expanded_for_overflow_and_rotation_is_independent() {
    let mut d = document("Repeated words to overflow the very small text rectangle without changing its saved size");
    let s = &mut d.pages[0].shapes[0];
    for (n, v) in [
        ("TxtWidth", "0.5"),
        ("TxtHeight", "0.1"),
        ("TxtPinX", "0.75"),
        ("TxtPinY", "0.4"),
        ("TxtLocPinX", "0.25"),
        ("TxtLocPinY", "0.05"),
        ("TxtAngle", "1.5707963267948966"),
    ] {
        set(s, n, v);
    }
    let opts = Options { page_canvas: true, ..Default::default() };
    let page = scene::evaluated::page(&d, 0, opts, &mut SceneBudget::default()).unwrap();
    let b = page.shapes[0].text_block.unwrap();
    assert_eq!(b.rect.width(), 36.);
    assert!((b.rect.height() - 7.2).abs() < 1e-9);
    assert!(b.layout_rect.height() > b.rect.height());
    let (_, pkg) = export(&d);
    let xml = pkg.xml("ppt/slides/slide1.xml").unwrap();
    let tree = xml.root.path(&["cSld", "spTree"]).unwrap();
    let text = tree.children_named("sp").find(|s| s.child("txBody").is_some()).unwrap();
    assert_eq!(text.path(&["nvSpPr", "cNvPr"]).unwrap().attr("name"), Some("Text 1"));
    let xf = text.path(&["spPr", "xfrm"]).unwrap();
    assert!(xf.attr("rot").is_none());
    assert_eq!(xf.child("ext").unwrap().attr("cx"), Some("457200"));
    assert_eq!(xf.child("ext").unwrap().attr("cy"), Some("91440"));
    assert_eq!(text.path(&["txBody", "bodyPr"]).unwrap().attr("rot"), Some("-5400000"));
}
#[test]
fn coincident_text_uses_original_shape_and_page_scaling_scales_type_and_margins() {
    let d = document("Inline");
    let (_, pkg) = export(&d);
    let xml = pkg.xml("ppt/slides/slide1.xml").unwrap();
    let sp = xml.root.find("sp").unwrap();
    assert!(sp.child("txBody").is_some());
    assert_eq!(xml.root.path(&["cSld", "spTree"]).unwrap().children_named("sp").count(), 1);
    let mut d = d;
    set(&mut d.pages[0].shapes[0], "LeftMargin", "0.1");
    for c in &mut d.pages[0].sheet.cells {
        if c.name == "PageWidth" || c.name == "PageHeight" {
            c.value = Some("0.5".into());
        }
    }
    let (_, pkg) = export(&d);
    let xml = pkg.xml("ppt/slides/slide1.xml").unwrap();
    assert_eq!(xml.root.find("rPr").unwrap().attr("sz"), Some("2400"));
    assert_eq!(xml.root.find("bodyPr").unwrap().attr("lIns"), Some("182880"));
}
#[test]
fn retained_export_keeps_saved_text_when_layout_rectangle_is_unusable() {
    let mut d = document("Keep this saved label");
    set(&mut d.pages[0].shapes[0], "TxtWidth", "50000");
    let opts = Options { page_canvas: true, cached_fallback: true, ..Default::default() };
    assert!(scene::page(&d, 0, opts).is_err());
    let (out, pkg) = export(&d);
    assert!(out.warnings.iter().any(|w| w.contains("text layout failed; saved text retained")));
    let xml = pkg.xml("ppt/slides/slide1.xml").unwrap();
    assert_eq!(xml.root.find("t").unwrap().text(), "Keep this saved label");
}
#[test]
fn hostile_character_properties_refuse_without_panicking() {
    let d = document("Bounded");
    let opts = Options { page_canvas: true, ..Default::default() };
    let colors = misterdiagram_export::colors(&d, 0).unwrap();
    for value in [f64::NAN, f64::INFINITY, 1e100, -1e100] {
        let mut page = scene::evaluated::page(&d, 0, opts, &mut SceneBudget::default()).unwrap();
        page.shapes[0].text_body.as_mut().unwrap().paragraphs[0].runs[0].props.size = Some(value);
        assert!(std::panic::catch_unwind(|| misterdiagram_export::render(vec![page], &colors, opts)).unwrap().is_err());
    }
}
#[test]
fn cached_field_fallback_is_the_saved_text_in_native_runs() {
    let mut d = document("");
    let s = &mut d.pages[0].shapes[0];
    s.text = Some(misterdiagram_vsdx::parse_xml(b"<Text>Label <fld IX='0'>saved value</fld></Text>").unwrap());
    section(
        s,
        "Field",
        vec![row(
            0,
            vec![
                Cell { name: "Value".into(), value: Some("2".into()), formula: Some("Width".into()), ..Default::default() },
                Cell { name: "Format".into(), value: Some("<,FEET/INCH><,FEET/INCH>0 u".into()), units: Some("STR".into()), ..Default::default() },
            ],
        )],
    );
    let (out, pkg) = export(&d);
    let xml = pkg.xml("ppt/slides/slide1.xml").unwrap();
    let p = xml.root.find("txBody").unwrap().child("p").unwrap();
    let text: String = p.children_named("r").map(|r| r.child("t").unwrap().text()).collect();
    assert_eq!(text, "Label saved value");
    assert!(out.warnings.iter().any(|w| w.contains("used the field text saved in the file")));
}
#[test]
fn run_alpha_and_explicit_false_properties_survive_native_serialization() {
    let d = document("Alpha");
    let opts = Options { page_canvas: true, ..Default::default() };
    let mut page = scene::evaluated::page(&d, 0, opts, &mut SceneBudget::default()).unwrap();
    let r = &mut page.shapes[0].text_body.as_mut().unwrap().paragraphs[0].runs[0].props;
    r.fill = Some(misterdiagram_color::Rgba::rgba(26, 51, 77, 128));
    r.bold = Some(false);
    r.italic = Some(false);
    r.baseline = Some(-0.2);
    let out = misterdiagram_export::render(vec![page], &misterdiagram_export::colors(&d, 0).unwrap(), opts).unwrap();
    let pkg = Package::open(&out.bytes).unwrap();
    let xml = pkg.xml("ppt/slides/slide1.xml").unwrap();
    let r = xml.root.find("rPr").unwrap();
    assert_eq!(r.attr("b"), Some("0"));
    assert_eq!(r.attr("i"), Some("0"));
    assert_eq!(r.attr("baseline"), Some("-20000"));
    assert_eq!(r.find("alpha").unwrap().attr("val"), Some("50196"));
}
#[test]
fn hostile_text_strings_are_refused_before_package_serialization() {
    let d = document("Bounded");
    let opts = Options { page_canvas: true, ..Default::default() };
    let colors = misterdiagram_export::colors(&d, 0).unwrap();
    for text in ["bad\u{1}control".into(), "&".repeat(65537)] {
        let mut page = scene::evaluated::page(&d, 0, opts, &mut SceneBudget::default()).unwrap();
        page.shapes[0].text_body.as_mut().unwrap().paragraphs[0].runs[0].text = text;
        assert!(std::panic::catch_unwind(|| misterdiagram_export::render(vec![page], &colors, opts)).unwrap().is_err());
    }
}
#[test]
fn drawingml_paragraph_metric_limits_refuse_invalid_packages() {
    // ECMA-376 Part 1: ST_TextMargin 0..51206400, ST_TextIndent +/-51206400 EMU.
    let d = document("Bounded paragraph");
    let opts = Options { page_canvas: true, ..Default::default() };
    let colors = misterdiagram_export::colors(&d, 0).unwrap();
    for (left, right, indent) in [(-1., 0., 0.), (0., -1., 0.), (5000., 0., 0.), (0., 5000., 0.), (0., 0., 5000.), (0., 0., -5000.)] {
        let mut page = scene::evaluated::page(&d, 0, opts, &mut SceneBudget::default()).unwrap();
        let props = &mut page.shapes[0].text_body.as_mut().unwrap().paragraphs[0].props;
        props.margin_left = Some(left);
        props.margin_right = Some(right);
        props.indent = Some(indent);
        assert!(misterdiagram_export::render(vec![page], &colors, opts).is_err(), "{left}, {right}, {indent}");
    }
}
