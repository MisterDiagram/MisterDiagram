use super::*;
use misterdiagram_model::text::{Paragraph, Run, TextHost};
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
    body.paragraphs[0].runs = vec![Run::new("a")];
    body.paragraphs[0].props.bullet = Some(Bullet::Char { char: "•".into() });
    for tab in [f64::INFINITY, f64::NAN, f64::MAX] {
        body.paragraphs[0].props.bullet_tab = Some(tab);
        assert!(layout(&body, &opts).is_err(), "bullet_tab {tab}");
    }
    body.paragraphs[0].props.bullet_tab = Some(36.);
    assert!(layout(&body, &opts).is_ok());
    assert!(Host.measure(&TextBody::default(), Some(f64::INFINITY)).is_err());
}
#[test]
fn a_wrap_width_past_the_layout_ceiling_means_no_wrapping() {
    // MisterDiagram#78: CFF Container headings measure TEXTHEIGHT(TheText,65535).
    let mut body = TextBody::default();
    body.paragraphs.push(Paragraph::new("E-Commerce Shopping Process Flow Diagram"));
    let unwrapped = Host.measure(&body, None).unwrap();
    let wide = Host.measure(&body, Some(65535. * 72.)).unwrap();
    assert!((wide.0 - unwrapped.0).abs() < 1e-6 && (wide.1 - unwrapped.1).abs() < 1e-6, "{wide:?} {unwrapped:?}");
    assert!(Host.measure(&body, Some(f64::NAN)).is_err());
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
    use misterdiagram_model::text::TabStop;
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

#[test]
fn composed_width_terminal_space_and_advance_grid() {
    // ADO_M/U, ADS_M/U and ANNOT_M: twenty cached TxtWidth witnesses.
    // https://learn.microsoft.com/en-us/office/client-developer/visio/textwidth-function
    for (text, size, margin, expected) in [
        ("Domain", 6., 3., 0.3668267852610678),
        ("Computer", 6., 3., 0.443158877546224),
        ("User", 6., 3., 0.2587983733958333),
        ("Group", 6., 3., 0.3153550750559896),
        ("Site", 8., 0., 0.1942180918203125),
        ("Site Link Bridge", 8., 0., 0.7161831186757811),
        ("Domain", 8., 0., 0.3779758738027344),
        ("Domain Controller", 8., 2., 0.9144312401493055),
        ("Label:", 8., 0., 0.2939914988027344),
        ("A", 12., 0., 0.1341633127675781),
    ] {
        let body = TextBody {
            body: BodyProps { inset_l: Some(margin), inset_r: Some(margin), ..Default::default() },
            paragraphs: vec![Paragraph {
                runs: vec![Run::with(text, RunProps { size: Some(size), font: Some("Calibri".into()), ..Default::default() })],
                ..Default::default()
            }],
            ..Default::default()
        };
        let width = Host.measure(&body, None).unwrap().0 / 72.;
        assert!((width - expected).abs() < 2e-9, "{text}: {width} vs {expected}");
    }
}

#[test]
fn symbol_glyph_keeps_the_text_face_line_and_a_trailing_no_break_space_hangs() {
    // #106: U+2714 set in "Segoe UI Symbol" falls back to the bundled symbol face. The line keeps
    // the text face's metrics, so the block measures the same height as a letter; and a glyph
    // followed by a no-break space in a block narrower than the pair, with no earlier break
    // opportunity, stays on one line, as Visio lays it out, instead of opening a second line
    // for the space (a line with an earlier opportunity still wraps: multiline-wrap's ninth line).
    let mut sym = TextBody::default();
    sym.paragraphs.push(Paragraph::new("\u{2714}\u{a0}"));
    sym.paragraphs[0].runs[0].props = RunProps { size: Some(90.), font: Some("Segoe UI Symbol".into()), ..Default::default() };
    let mut letter = sym.clone();
    letter.paragraphs[0].runs[0].text = "A\u{a0}".into();
    let (sym_w, sym_h) = Host.measure(&sym, Some(81.)).unwrap();
    let (_, letter_h) = Host.measure(&letter, Some(81.)).unwrap();
    assert!((sym_h - letter_h).abs() < 1e-6, "symbol line {sym_h} letter line {letter_h}");
    assert!(sym_w > 81., "the pair overflows the block rather than wrapping: {sym_w}");
    let opts = Opts { rect: Rect::new(0., 0., 81., 400.), fields: &NoFields, prompt_color: None, no_shrink: true };
    let l = layout(&sym, &opts).unwrap();
    let baselines: std::collections::BTreeSet<i64> =
        l.runs.iter().flat_map(|r| r.glyphs.iter().map(|(_, _, y)| (y * 1000.).round() as i64)).collect();
    assert_eq!(baselines.len(), 1, "one line: {baselines:?}");
    assert!(l.runs.iter().any(|r| r.face.family == "Noto Sans Symbols 2"), "the mark is drawn by the symbol face");
}

#[test]
fn a_dash_before_the_terminal_space_keeps_one_line() {
    // MisterDiagram#87: an op-amp's "-" in a 0.1743 in box with 4 pt margins,
    // Calibri 12 pt, middle-aligned. UAX #14 allows a break between a dash and the composed
    // no-break space, which opened a second line holding only the space and lifted the dash
    // half a line. Visio 2021 keeps hyphen, en dash and U+2010 on the line of "+" and "x".
    if FontDb::global().face("Calibri", "Regular").data().is_empty() {
        eprintln!("open font pack absent: layout metrics skipped");
        return;
    }
    let glyphs = |text: &str, width: f64| {
        let mut body = TextBody {
            body: BodyProps {
                inset_l: Some(4.),
                inset_r: Some(4.),
                inset_t: Some(4.),
                inset_b: Some(4.),
                anchor: Some(misterdiagram_model::text::Anchor::Middle),
                ..Default::default()
            },
            ..Default::default()
        };
        body.paragraphs.push(Paragraph::new(text));
        body.paragraphs[0].runs[0].props = RunProps { size: Some(12.), font: Some("Calibri".into()), ..Default::default() };
        body.paragraphs[0].props.align = Some(misterdiagram_model::text::Align::Center);
        let opts = Opts { rect: Rect::new(0., 0., width * 72., 0.2778 * 72.), fields: &NoFields, prompt_color: None, no_shrink: true };
        let l = layout_for_export(&body, &opts).unwrap();
        l.runs.iter().flat_map(|r| r.glyphs.iter().map(|(_, x, y)| ((x * 1000.).round() as i64, (y * 1000.).round() as i64))).collect::<Vec<_>>()
    };
    let block = |text: &str, width: f64| glyphs(text, width).into_iter().map(|(_, y)| y).collect::<std::collections::BTreeSet<i64>>();
    let plus = block("+", 0.1743);
    assert_eq!(plus.len(), 1, "{plus:?}");
    // U+2010 is not in the Calibri substitute; its fallback face moves the baseline by about
    // 0.05 pt. The defect was half a line (7.2 pt), so the check is one line within 0.5 pt.
    for dash in ["-", "\u{2013}", "\u{2010}"] {
        let lines = block(dash, 0.1743);
        assert_eq!(lines.len(), 1, "{dash:?}: {lines:?}");
        let (y, p) = (*lines.first().unwrap(), *plus.first().unwrap());
        assert!((y - p).abs() < 500, "{dash:?} sits on the line of \"+\": {y} vs {p}");
    }
    // The same drawing's "50 ohm    " (four trailing spaces) in a 0.75 in block: the terminal
    // space hangs with the trailing spaces, so the label keeps one line on the baseline of
    // "50 ohm"; and the trailing spaces do not count when the line is centred.
    let trimmed = block("50 ohm", 0.75);
    assert_eq!(trimmed.len(), 1, "{trimmed:?}");
    assert_eq!(block("50 ohm    ", 0.75), trimmed);
    for width in [0.75, 2.0] {
        let x = |text: &str| glyphs(text, width).first().map(|g| g.0);
        assert_eq!(x("50 ohm    "), x("50 ohm"), "centred at {width} in");
    }
}

#[test]
fn visio_bullets_start_the_text_at_a_tab_stop() {
    // MisterDiagram#110: Visio 2021 starts the text after a bullet at a tab stop, measured on a
    // synthetic witness page (Calibri, 0.5 in default stop, positions from the text area's
    // left edge). The DrawingML rule (text right after the bullet) stays the default.
    if FontDb::global().face("Calibri", "Regular").data().is_empty() {
        eprintln!("open font pack absent: layout metrics skipped");
        return;
    }
    let runs = |bullet: &str, tab: Option<f64>, margin: f64, indent: f64, align: misterdiagram_model::text::Align| {
        let mut body = TextBody {
            body: BodyProps { inset_l: Some(0.), inset_r: Some(0.), inset_t: Some(0.), inset_b: Some(0.), ..Default::default() },
            ..Default::default()
        };
        body.paragraphs.push(Paragraph::new("Abc"));
        body.paragraphs[0].runs[0].props = RunProps { size: Some(12.), font: Some("Calibri".into()), ..Default::default() };
        let props = &mut body.paragraphs[0].props;
        props.bullet = Some(Bullet::Char { char: bullet.into() });
        props.bullet_tab = tab;
        props.default_tab = Some(36.);
        props.margin_left = Some(margin);
        props.indent = Some(indent);
        props.align = Some(align);
        let opts = Opts { rect: Rect::new(0., 0., 230.4, 30.), fields: &NoFields, prompt_color: None, no_shrink: true };
        let l = layout_for_export(&body, &opts).unwrap();
        let first = |i: usize| l.runs.get(i).and_then(|r| r.glyphs.first()).map(|g| g.1).unwrap();
        // The text run's end: its last glyph ("c") plus that glyph's advance.
        let end = l.runs.get(1).map(|r| r.glyphs.last().map(|g| g.1).unwrap() + measure_str(&r.face, r.size, "c").0).unwrap();
        (first(0), first(1), end, l.runs[0].glyphs.iter().all(|g| g.0 != 0))
    };
    use misterdiagram_model::text::Align::{Center, Left, Right};
    let near = |a: f64, b: f64| (a - b).abs() < 0.01;
    // TextPosAfterBullet 0: the next default stop, whatever the size.
    let (b, t, ..) = runs("•", Some(0.), 0., 0., Left);
    assert!(near(b, 0.) && near(t, 36.), "{b} {t}");
    // TextPosAfterBullet past the bullet wins over a nearer default stop.
    assert!(near(runs("•", Some(72.), 0., 0., Left).1, 72.));
    assert!(near(runs("•", Some(7.2), 0., 0., Left).1, 7.2));
    // Behind the bullet it is ignored: IndFirst 0.25 in with 0.1 in starts at 0.5 in.
    assert!(near(runs("•", Some(7.2), 0., 18., Left).1, 36.));
    // A bullet at a stop moves on to the next one; the hanging indent is a stop too.
    assert!(near(runs("•", Some(0.), 36., 0., Left).1, 72.));
    assert!(near(runs("•", Some(0.), 28.8, -28.8, Left).1, 28.8));
    // A bullet reaching past the stop pushes the text to its end.
    let (b, t, ..) = runs("•", Some(0.), 0., 34.56, Left);
    assert!(near(b, 34.56) && t > 36.5, "{b} {t}");
    // Centred and right-aligned: bullet, gap and text are one unit.
    let (b, t, end, _) = runs("•", Some(0.), 0., 0., Center);
    assert!(near(t - b, 36.) && ((b + end) / 2. - 115.2).abs() < 0.5, "{b} {t} {end}");
    let (b, t, end, _) = runs("•", Some(0.), 0., 0., Right);
    assert!(near(t - b, 36.) && (end - 230.4).abs() < 0.5, "{b} {t} {end}");
    // Without bullet_tab the text follows the bullet glyph.
    assert!(runs("•", None, 0., 0., Left).1 < 10.);
    // Wingdings bullets are drawn by the bundled symbol face, never as a missing glyph.
    for bullet in ["▪", "❑", "❖", "➢", "✓"] {
        assert!(runs(bullet, Some(0.), 0., 0., Left).3, "{bullet}");
    }
}

#[test]
fn the_terminal_space_hangs_and_never_wraps_the_last_word() {
    // MisterDiagram#87: Visio 2021's final build keeps an 8 pt Arial "CDR Lock Time" on one line
    // in a 0.906 in block with 4 pt margins (57.25 pt of measure); the words take 56.45 pt and the
    // composed terminal no-break space would overflow by 1.4 pt. Build 20430 wrapped "Time".
    if FontDb::global().face("Arial", "Regular").data().is_empty() {
        eprintln!("open font pack absent: layout metrics skipped");
        return;
    }
    let lines = |text: &str, width: f64| {
        let mut body = TextBody {
            body: BodyProps { inset_l: Some(4.), inset_r: Some(4.), inset_t: Some(4.), inset_b: Some(4.), ..Default::default() },
            ..Default::default()
        };
        body.paragraphs.push(Paragraph::new(text));
        body.paragraphs[0].runs[0].props = RunProps { size: Some(8.), font: Some("Arial".into()), ..Default::default() };
        body.paragraphs[0].props.align = Some(misterdiagram_model::text::Align::Center);
        let opts = Opts { rect: Rect::new(0., 0., width * 72., 20.), fields: &NoFields, prompt_color: None, no_shrink: true };
        layout_for_export(&body, &opts).unwrap().lines.len()
    };
    assert_eq!(lines("CDR Lock Time", 0.90625), 1);
    // Without room for the words themselves it still wraps.
    assert_eq!(lines("CDR Lock Time", 0.85), 2);
    assert_eq!(lines("Unlock Data", 0.625), 2);
}
