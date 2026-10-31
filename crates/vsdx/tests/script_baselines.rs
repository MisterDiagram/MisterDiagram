//! Script offsets are evaluator outputs, independent of installed font binaries.
use misterdiagram_model::{Shape, skeleton};
use misterdiagram_vsdx::{
    evaluate::{Evaluator, Scope},
    parse_sheet, parse_xml,
};

/// Native offsets: field/subsup-2026-10-30. Pos determines the script's baseline.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pos-cell-character-section
#[test]
fn evaluated_script_runs_keep_all_six_native_baseline_shifts() {
    for (family, upper, lower) in
        [("Arial", 0.477, -0.14), ("Calibri", 0.3, -0.14), ("Times New Roman", 0.3, -0.3), ("aRiAl", 0.477, -0.14), ("cAlIbRi", 0.3, -0.14)]
    {
        let mut d = skeleton::document(false);
        d.face_names.clear();
        d.face_names.insert(0, family.into());
        let xml = format!(
            "<Shape xmlns='{}'><Section N='Character'><Row IX='0'><Cell N='Font' V='0'/><Cell N='Size' V='0.25'/><Cell N='Pos' V='0'/></Row><Row IX='1'><Cell N='Font' V='0'/><Cell N='Size' V='0.25'/><Cell N='Pos' V='1'/></Row><Row IX='2'><Cell N='Font' V='0'/><Cell N='Size' V='0.25'/><Cell N='Pos' V='2'/></Row></Section><Text><cp IX='0'/>N<cp IX='1'/>U<cp IX='2'/>L</Text></Shape>",
            skeleton::VISIO_NS
        );
        let element = parse_xml(xml.as_bytes()).unwrap();
        d.pages[0].shapes.push(Shape { id: 7, sheet: parse_sheet(&element).unwrap(), text: element.child("Text").cloned(), ..Default::default() });
        let mut evaluator = Evaluator::new(&d).unwrap();
        let scope = Scope::PageShape { page: 0, shape: 7 };
        let body = evaluator.text_body(scope).unwrap();
        let runs = &body.paragraphs[0].runs;
        assert_eq!(runs.len(), 3);
        for (i, expected) in [0., upper, lower].into_iter().enumerate() {
            // The evaluator may canonicalize the spelling of Calibri.
            assert!(runs[i].props.font.as_deref().is_some_and(|name| name.eq_ignore_ascii_case(family)));
            assert_eq!(runs[i].props.size, Some(18.));
            assert_eq!(runs[i].props.baseline, Some(expected), "{family} Pos {i}");
            let cell = if i == 0 { "Char.Pos".into() } else { format!("Char.Pos[{}]", i + 1) };
            assert_eq!(evaluator.evaluate(scope, &cell).numeric().unwrap(), i as f64);
        }
    }
}
