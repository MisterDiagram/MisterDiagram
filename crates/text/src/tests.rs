use super::*;
use diagramcraft_model::text::{Paragraph, Run, TextHost};
#[test]
fn finite_wrapping_bidi_and_autofit() {
    if FontDb::global().face("Calibri", "Regular").data().is_empty() {
        eprintln!("open font pack absent: layout metrics skipped");
        return;
    }
    let mut body = TextBody::default();
    body.paragraphs.push(Paragraph::new("A long paragraph with several words and العربية 123."));
    body.paragraphs[0].runs[0].props = RunProps { size: Some(12.), font: Some("Calibri".into()), ..Default::default() };
    let wide = Host.measure(&body, Some(400.)).unwrap();
    let narrow = Host.measure(&body, Some(80.)).unwrap();
    assert!(narrow.1 > wide.1);
    body.body.autofit = Some(AutoFit::Shrink { font_scale: 1., line_reduction: 0. });
    let l = layout(&body, &Opts { rect: Rect::new(0., 0., 80., 24.), fields: &NoFields, prompt_color: None, no_shrink: false }).unwrap();
    assert!(l.font_scale < 1.);
    assert!(l.runs.iter().flat_map(|r| &r.glyphs).all(|(_, x, y)| x.is_finite() && y.is_finite()));
}
#[test]
fn hostile_text_layout_is_bounded() {
    let mut body = TextBody::default();
    body.paragraphs.push(Paragraph::new("a"));
    let opts = Opts { rect: Rect::new(0., 0., 100., 100.), fields: &NoFields, prompt_color: None, no_shrink: false };
    body.paragraphs[0].runs[0].props.size = Some(f64::NAN);
    assert!(layout(&body, &opts).is_err());
    body.paragraphs[0].runs = vec![Run::new("x".repeat(65537))];
    assert!(layout(&body, &opts).is_err());
    body.paragraphs[0].runs.clear();
    body.paragraphs[0].props.bullet = Some(Bullet::AutoNum { scheme: "romanUcPeriod".into(), start_at: u32::MAX });
    assert!(layout(&body, &opts).is_err());
    assert!(Host.measure(&TextBody::default(), Some(f64::INFINITY)).is_err());
}

#[test]
fn report_default_font_substitutes() {
    for family in ["Calibri", "Calibri Light", "Arial", "Times New Roman", "Segoe UI", "Symbol", "Wingdings"] {
        let face = FontDb::global().face(family, "Regular");
        println!("{family} -> {} ({} font bytes)", face.family, face.data().len());
    }
}
