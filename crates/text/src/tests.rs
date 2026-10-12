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

#[test]
fn visio_font_percentage_empty_line_and_aligned_tabs() {
    use diagramcraft_model::text::TabStop;
    let mut body = TextBody {
        body: BodyProps {
            inset_l: Some(0.),
            inset_r: Some(0.),
            inset_t: Some(4.),
            inset_b: Some(4.),
            anchor: Some(Anchor::Top),
            ..Default::default()
        },
        ..Default::default()
    };
    body.paragraphs.push(Paragraph {
        props: ParaProps { line_spacing: Some(Spacing::FontPct(1.2)), ..Default::default() },
        end_props: RunProps { font: Some("Calibri".into()), size: Some(8.), ..Default::default() },
        ..Default::default()
    });
    let (_, height) = Host.measure(&body, Some(100.)).unwrap();
    assert!((height - 17.6).abs() < 1e-9);
    body.paragraphs[0].runs = vec![Run::with("x\t123.45", RunProps { font: Some("Calibri".into()), size: Some(12.), ..Default::default() })];
    for alignment in ["l", "ctr", "r", "dec"] {
        body.paragraphs[0].props.tabs = Some(vec![TabStop { pos: 80., align: alignment.into() }]);
        let l = layout(&body, &Opts { rect: Rect::new(0., 0., 200., 100.), fields: &NoFields, prompt_color: None, no_shrink: true }).unwrap();
        let line = &l.lines[0];
        let begin = line.edges[2].0;
        let end = line.edges[7].1;
        let at = match alignment {
            "ctr" => (begin + end) / 2.,
            "r" => end,
            "dec" => line.edges[5].0,
            _ => begin,
        };
        assert!((at - 80.).abs() < 1e-8, "{alignment}: {at}");
    }
}

#[test]
fn percentage_line_baseline_matches_generated_alignment_svg() {
    let body = TextBody {
        body: BodyProps { inset_t: Some(4.), anchor: Some(Anchor::Top), ..Default::default() },
        paragraphs: vec![Paragraph {
            props: ParaProps { line_spacing: Some(Spacing::FontPct(1.2)), ..Default::default() },
            runs: vec![Run::with("Alignment", RunProps { font: Some("Calibri".into()), size: Some(12.), ..Default::default() })],
            ..Default::default()
        }],
        ..Default::default()
    };
    let l = layout(&body, &Opts { rect: Rect::new(0., 0., 288., 216.), fields: &NoFields, prompt_color: None, no_shrink: true }).unwrap();
    assert!((l.lines[0].baseline - 14.8).abs() < 1e-9);
}

#[test]
fn metric_font_size_grid_matches_com_height_witnesses() {
    // COM: ADO_M Domain (6pt), ANNOT_U Info. line (8pt), UML Member (10pt), Class (12pt).
    for (size, expected) in [(6., 0.10004847412109375), (8., 0.1333828247070313), (10., 0.16671717529296873), (12., 0.20005152587890625)] {
        let body = TextBody {
            paragraphs: vec![Paragraph {
                props: ParaProps { line_spacing: Some(Spacing::FontPct(1.2)), ..Default::default() },
                runs: vec![Run::with("X", RunProps { size: Some(size), font: Some("Calibri".into()), ..Default::default() })],
                ..Default::default()
            }],
            ..Default::default()
        };
        assert!((Host.measure(&body, Some(100.)).unwrap().1 / 72. - expected).abs() < 1e-12, "{size}pt");
    }
    let body = TextBody {
        paragraphs: vec![Paragraph {
            props: ParaProps { line_spacing: Some(Spacing::FontPct(1.2)), ..Default::default() },
            end_props: RunProps { size: Some(10.), font: Some("Calibri".into()), ..Default::default() },
            runs: vec![Run {
                text: String::new(),
                props: RunProps { size: Some(10.), font: Some("Calibri".into()), ..Default::default() },
                kind: RunKind::Field { field: "0".into() },
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    assert!((Host.measure(&body, Some(100.)).unwrap().1 / 72. - 0.16671717529296873).abs() < 1e-12);
}
