use misterdiagram_model::{Cell, Row, Section, skeleton};
use misterdiagram_render::scene::{self, Op, Paint};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};

fn cell(name: &str, value: &str, formula: Option<&str>) -> Cell {
    Cell { name: name.into(), value: Some(value.into()), formula: formula.map(str::to_owned), ..Default::default() }
}
fn document(legacy: bool) -> misterdiagram_model::Document {
    let mut doc = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 2., 2., 3., 1., "").unwrap();
    shape.sheet.cells.retain(|c| !matches!(c.name.as_str(), "FillForegnd" | "FillBkgnd" | "FillPattern"));
    shape.sheet.cells.extend([
        cell("QuickStyleFillColor", "100", None),
        cell("QuickStyleFillMatrix", "100", None),
        cell("FillForegnd", "#aabbcc", Some(if legacy { "THEME(\"FillColor2\")" } else { "THEMEGUARD(MSOTINT(THEME(\"FillColor\"),60))" })),
        cell(
            "FillBkgnd",
            "#aabbcc",
            Some(if legacy { "THEME(\"FillColor\")" } else { "THEMEGUARD(SHADE(FillForegnd,LUMDIFF(THEME(\"FillColor\"),THEME(\"FillColor2\"))))" }),
        ),
        cell("FillPattern", "1", Some("THEME(\"FillPattern\")")),
        cell("FillGradientEnabled", "0", Some("THEME(\"FillGradientEnabled\")")),
    ]);
    if legacy {
        shape.sheet.sections.push(Section {
            name: "User".into(),
            rows: vec![
                Row { name: Some("msvThemeColors".into()), cells: vec![cell("Value", "36", None)], ..Default::default() },
                Row { name: Some("msvThemeEffects".into()), cells: vec![cell("Value", "16", None)], ..Default::default() },
            ],
            ..Default::default()
        });
        shape.sheet.sections.push(Section {
            name: "FillGradient".into(),
            rows: (0..2)
                .map(|i| Row {
                    index: Some(i),
                    cells: vec![
                        cell("GradientStopColor", "#aabbcc", Some(if i == 0 { "THEME(\"FillColor2\")" } else { "THEME(\"FillColor\")" })),
                        cell(
                            "GradientStopPosition",
                            "0",
                            Some(if i == 0 { "THEME(\"FillGradStop1Position\")" } else { "THEME(\"FillGradStop2Position\")" }),
                        ),
                        cell("GradientStopColorTrans", "0", None),
                    ],
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        });
    }
    doc.pages[0].shapes.push(shape);
    doc
}
/// Native scalar controls distinguish legacy selectors from modern quick styles.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/theme-function
#[test]
fn legacy_fill_selectors_and_no_theme_nested_formulas_match_native() {
    for legacy in [false, true] {
        let doc = document(legacy);
        let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
        let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes);
        let s = Scope::PageShape { page: 0, shape: 1 };
        assert_eq!(e.color_rgb(s, "FillForegnd"), Some(if legacy { [240, 240, 240] } else { [213, 223, 235] }));
        assert_eq!(e.color_rgb(s, "FillBkgnd"), Some(if legacy { [255, 255, 255] } else { [225, 232, 240] }));
        assert_eq!(e.evaluate(s, "FillPattern").numeric().unwrap(), if legacy { 30. } else { 1. });
        assert_eq!(e.evaluate(s, "FillGradientEnabled").numeric().unwrap(), if legacy { 1. } else { 0. });
        let rendered = scene::page(&doc, 0, scene::Options::default()).unwrap();
        if legacy {
            let g = rendered
                .ops
                .iter()
                .find_map(|o| match o {
                    Op::Path { paint: Paint::Pattern { gradient: Some(g), .. }, .. } => Some(g),
                    _ => None,
                })
                .unwrap();
            assert_eq!(g.stops.len(), 2);
            assert_eq!(g.stops[0].1.r, 240);
            assert_eq!(g.stops[1].1.r, 255);
            assert_eq!(g.stops[0].0, 0.);
            assert_eq!(g.stops[1].0, 1.);
        }
    }
}
/// An unmeasured inner property cannot be replaced by its enclosing cell's value.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
#[test]
fn unknown_legacy_property_preserves_whole_cached_expression_without_warning() {
    let mut doc = document(false);
    let c = doc.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "FillForegnd").unwrap();
    c.formula = Some("MSOTINT(THEME(\"UnmeasuredProperty\"),60)".into());
    let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
    let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes);
    assert_eq!(e.color_rgb(Scope::PageShape { page: 0, shape: 1 }, "FillForegnd"), Some([170, 187, 204]));
    assert_eq!(e.take_fallback_warnings(), (vec![], 0));
}
/// Invalid palette identities are hostile input, never an absent fill.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
#[test]
fn unresolved_palette_uses_saved_rgb_and_dynamic_palette_remains_visible() {
    for formula in ["999999", "-1", "1.5", "1e100"] {
        let mut doc = document(false);
        let c = doc.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "FillForegnd").unwrap();
        c.formula = Some(formula.into());
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.color_rgb(Scope::PageShape { page: 0, shape: 1 }, "FillForegnd"), Some([170, 187, 204]));
        assert!(e.take_fallback_warnings().0.iter().any(|w| w.contains("unresolved colour palette")));
    }
    let mut doc = document(false);
    let c = doc.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "FillForegnd").unwrap();
    c.formula = Some("RGB(12,34,56)+0".into());
    let mut e = Evaluator::new(&doc).unwrap();
    assert_eq!(e.color_rgb(Scope::PageShape { page: 0, shape: 1 }, "FillForegnd"), Some([12, 34, 56]));
    assert_eq!(e.take_fallback_warnings(), (vec![], 0));
}

/// Repeated endpoint rows carry no extra colour segment in the native renderer.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradient-section
#[test]
fn duplicate_legacy_endpoint_keeps_the_native_two_stop_blend() {
    let mut doc = document(true);
    let section = doc.pages[0].shapes[0].sheet.sections.iter_mut().find(|s| s.name == "FillGradient").unwrap();
    let mut third = section.rows[0].clone();
    third.index = Some(2);
    third.cells.iter_mut().find(|c| c.name == "GradientStopPosition").unwrap().formula = Some("THEME(\"FillGradStop3Position\")".into());
    section.rows.push(third);
    section.rows.extend((3..10).map(|index| Row { index: Some(index), deleted: true, ..Default::default() }));
    let rendered = scene::page(&doc, 0, scene::Options::default()).unwrap();
    let g = rendered
        .ops
        .iter()
        .find_map(|o| match o {
            Op::Path { paint: Paint::Pattern { gradient: Some(g), .. }, .. } => Some(g),
            _ => None,
        })
        .unwrap();
    assert_eq!(g.stops.len(), 2);
    assert!(g.smooth);
    assert!(g.sample(0.5, 0.1).r < g.sample(0.5, 0.9).r);
}

/// Canonical fallback follows executed calls and keeps selector dependencies.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/theme-function
#[test]
fn legacy_fallback_does_not_freeze_branches_or_lose_dependencies() {
    let doc = document(true);
    let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
    let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes);
    let s = Scope::PageShape { page: 0, shape: 1 };
    e.set_formula(s, "User.msvThemeColors", "99").unwrap();
    assert_eq!(e.color_rgb(s, "FillForegnd"), Some([170, 187, 204]));
    e.set_formula(s, "User.msvThemeColors", "36").unwrap();
    assert_eq!(e.color_rgb(s, "FillForegnd"), Some([240, 240, 240]));
    e.set_formula(s, "FillForegnd", "IF(1,RGB(12,34,56),THEME(\"Unknown\"))").unwrap();
    assert_eq!(e.color_rgb(s, "FillForegnd"), Some([12, 34, 56]));
    e.set_formula(s, "FillForegnd", "UNSUPPORTEDFUNCTION(1)+THEME(\"Unknown\")").unwrap();
    assert!(e.evaluate(s, "FillForegnd").error().is_some());
}

/// Text backgrounds use one-based palette values and must retain their saved RGB.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/textbkgnd-cell-text-block-format-section
#[test]
fn missing_text_background_palette_warns_and_preserves_saved_colour() {
    let mut doc = document(false);
    doc.pages[0].shapes[0].sheet.cells.push(cell("TextBkgnd", "#345678", Some("999999")));
    let mut e = Evaluator::new(&doc).unwrap();
    let s = Scope::PageShape { page: 0, shape: 1 };
    assert_eq!(e.color_rgb(s, "TextBkgnd"), Some([52, 86, 120]));
    assert!(e.take_fallback_warnings().0.iter().any(|w| w.contains("unresolved colour palette")));
    e.set_formula(s, "TextBkgnd", "RGB(12,34,56)+1").unwrap();
    assert_eq!(e.color_rgb(s, "TextBkgnd"), Some([12, 34, 56]));
    e.set_formula(s, "TextBkgnd", "0").unwrap();
    assert_eq!(e.color_rgb(s, "TextBkgnd"), None);
}

/// Missing caches cannot invent zero, and call order cannot hide unknown functions.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
#[test]
fn canonical_legacy_fallback_requires_saved_value_and_known_surrounding_functions() {
    for formula in ["THEME(\"Unknown\")+UNSUPPORTEDFUNCTION(1)", "UNSUPPORTEDFUNCTION(1)+THEME(\"Unknown\")"] {
        let mut doc = document(false);
        let cell = doc.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "FillForegnd").unwrap();
        cell.formula = Some(formula.into());
        let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
        let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes);
        assert!(e.evaluate(Scope::PageShape { page: 0, shape: 1 }, "FillForegnd").error().is_some());
    }
    let mut doc = document(false);
    let cell = doc.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "FillForegnd").unwrap();
    cell.formula = Some("THEME(\"Unknown\")".into());
    cell.value = None;
    let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
    let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes);
    assert!(e.evaluate(Scope::PageShape { page: 0, shape: 1 }, "FillForegnd").error().is_some());
    assert_eq!(e.take_fallback_warnings(), (vec![], 0));
}

/// Final paint keeps saved RGB, opacity and dynamic identities across fresh evaluators.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillforegndtrans-cell-fill-format-section
#[test]
fn palette_recovery_preserves_final_paint_and_evaluation_order() {
    let mut doc = document(false);
    let cells = &mut doc.pages[0].shapes[0].sheet.cells;
    for c in cells.iter_mut() {
        if c.name == "FillForegnd" {
            c.formula = Some("999999".into());
        }
        if c.name == "FillGradientEnabled" {
            c.formula = None;
            c.value = Some("0".into());
        }
    }
    cells.push(cell("FillForegndTrans", "0.25", None));
    for _ in 0..2 {
        let page = scene::page(&doc, 0, scene::Options::default()).unwrap();
        assert!(page.ops.iter().any(|op| matches!(op,Op::Path {paint:Paint::Solid(c),..} if (c.r,c.g,c.b,c.a)==(170,187,204,191))));
        assert!(page.warnings.iter().any(|w| w.contains("unresolved colour palette")));
    }
    let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
    let s = Scope::PageShape { page: 0, shape: 1 };
    for first in ["FillForegnd", "FillBkgnd"] {
        let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes);
        let _ = e.evaluate(s, first);
        assert_eq!(e.color_rgb(s, "FillForegnd"), Some([170, 187, 204]));
    }
}
/// Custom palette entries and speculative cached identities keep their RGB payload.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
#[test]
fn custom_palette_and_speculative_cache_collision_keep_paint_rgb() {
    let mut doc = document(false);
    doc.colors.insert(88, "#123456".into());
    let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
    let s = Scope::PageShape { page: 0, shape: 1 };
    let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes);
    e.set_formula(s, "FillForegnd", "88").unwrap();
    assert_eq!(e.color_rgb(s, "FillForegnd"), Some([18, 52, 86]));
    assert_eq!(e.take_fallback_warnings(), (vec![], 0));
    for c in &mut doc.pages[0].shapes[0].sheet.cells {
        if c.name == "FillForegnd" {
            c.formula = Some("THEME(\"Unknown\")".into());
        } else if c.name == "FillBkgnd" {
            c.formula = Some("RGB(12,34,56)".into());
        }
    }
    let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
    for first in ["FillForegnd", "FillBkgnd"] {
        let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes);
        let _ = e.evaluate(s, first);
        assert_eq!(e.color_rgb(s, "FillBkgnd"), Some([12, 34, 56]));
        assert_eq!(e.color_rgb(s, "FillForegnd"), Some([170, 187, 204]));
        assert_eq!(e.take_fallback_warnings(), (vec![], 0));
    }
}

/// Master and page paint use the same recovery with independent evaluator lifetimes.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillforegndtrans-cell-fill-format-section
#[test]
fn inherited_master_palette_recovery_keeps_opacity_across_repeated_renders() {
    let mut doc = document(false);
    for c in &mut doc.pages[0].shapes[0].sheet.cells {
        if c.name == "FillForegnd" {
            c.formula = Some("999999".into());
        }
        if c.name == "FillGradientEnabled" {
            c.formula = None;
            c.value = Some("0".into());
        }
    }
    doc.pages[0].shapes[0].sheet.cells.push(cell("FillForegndTrans", "0.25", None));
    doc.masters[0].shapes = vec![doc.pages[0].shapes[0].clone()].into();
    for _ in 0..2 {
        for rendered in [scene::page(&doc, 0, scene::Options::default()).unwrap(), scene::master(&doc, 0, scene::Options::default()).unwrap()] {
            assert!(rendered.ops.iter().any(|op| matches!(op, Op::Path { paint: Paint::Solid(c), .. } if (c.r,c.g,c.b,c.a)==(170,187,204,191))));
            assert!(rendered.warnings.iter().any(|w| w.contains("unresolved colour palette")));
        }
    }
}
/// Inherited modern defaults do not reverse a two-row legacy colour ramp.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradient-section
#[test]
fn two_stored_legacy_stops_ignore_unused_modern_slots_for_blending() {
    let mut doc = document(true);
    for style in &mut doc.styles {
        style.sheet.sections.retain(|section| section.name != "FillGradient");
    }
    let e = Evaluator::new(&doc).unwrap();
    assert!(e.section_row_indices(Scope::PageShape { page: 0, shape: 1 }, "FillGradient", 0).len() > 2);
    let rendered = scene::page(&doc, 0, scene::Options::default()).unwrap();
    let gradient = |scene: &scene::Scene| {
        scene
            .ops
            .iter()
            .find_map(|op| match op {
                Op::Path { paint: Paint::Pattern { gradient: Some(g), .. }, .. } => Some(g.clone()),
                _ => None,
            })
            .unwrap()
    };
    assert!(gradient(&rendered).smooth);
    // An actual authored third row still selects the multirow path.
    let section = doc.pages[0].shapes[0].sheet.sections.iter_mut().find(|s| s.name == "FillGradient").unwrap();
    section.rows.push(Row {
        index: Some(2),
        cells: vec![cell("GradientStopColor", "#123456", None), cell("GradientStopPosition", "0.5", None)],
        ..Default::default()
    });
    assert!(!gradient(&scene::page(&doc, 0, scene::Options::default()).unwrap()).smooth);
}

/// Missing numeric caches and unexecuted calls cannot become canonical saved results.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
#[test]
fn empty_legacy_cache_and_unexecuted_legacy_calls_do_not_hide_failures() {
    for saved in [None, Some(""), Some("Themed"), Some("#VALUE!")] {
        let mut doc = document(false);
        let cell = doc.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "FillForegnd").unwrap();
        cell.value = saved.map(str::to_owned);
        cell.formula = Some("MSOTINT(THEME(\"Unknown\"),60)".into());
        let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
        let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes);
        assert!(e.evaluate(Scope::PageShape { page: 0, shape: 1 }, "FillForegnd").error().is_some());
    }
    let doc = document(false);
    let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
    let mut e = Evaluator::new(&doc).unwrap().with_theme_host(&themes);
    let s = Scope::PageShape { page: 0, shape: 1 };
    e.set_formula(s, "FillForegnd", "IF(0,THEME(\"Unknown\"),1/0)").unwrap();
    assert!(e.evaluate(s, "FillForegnd").error().is_some());
}

/// Errors and logical values are not unregistered numeric palette identities.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillforegnd-cell-fill-format-section
#[test]
fn nonnumeric_colours_do_not_claim_missing_palette_recovery() {
    for formula in ["SQRT(-1)", "1/0", "TRUE", "FALSE"] {
        let mut doc = document(false);
        let cell = doc.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "FillForegnd").unwrap();
        cell.formula = Some(formula.into());
        let mut evaluator = Evaluator::new(&doc).unwrap();
        assert_eq!(evaluator.color_rgb(Scope::PageShape { page: 0, shape: 1 }, "FillForegnd"), None);
        assert_eq!(evaluator.take_fallback_warnings(), (vec![], 0));
    }
    let mut doc = document(false);
    doc.pages[0].shapes[0].sheet.cells.push(cell("TextBkgnd", "#345678", Some("TRUE")));
    let mut evaluator = Evaluator::new(&doc).unwrap();
    let scope = Scope::PageShape { page: 0, shape: 1 };
    assert_eq!(evaluator.color_rgb(scope, "TextBkgnd"), Some([0, 0, 0]));
    assert_eq!(evaluator.take_fallback_warnings(), (vec![], 0));
    evaluator.set_formula(scope, "TextBkgnd", "FALSE").unwrap();
    assert_eq!(evaluator.color_rgb(scope, "TextBkgnd"), None);
    assert_eq!(evaluator.take_fallback_warnings(), (vec![], 0));
}
