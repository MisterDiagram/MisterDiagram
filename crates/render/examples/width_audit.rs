//! Read-only text-width diagnostics against private cached/COM observations.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use diagramcraft_fonts::{FontDb, feature, shape};
use diagramcraft_model::text::TextHost;
use diagramcraft_vsdx::evaluate::Evaluator;
fn main() -> Result<(), String> {
    let cell = std::env::var("AUDIT_CELL").unwrap_or_else(|_| "TxtWidth".into());
    let limit = std::env::var("AUDIT_LIMIT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(4).min(100);
    for file in std::env::args().skip(1).take(32) {
        let doc = diagramcraft_vsdx::read(&file).map_err(|e| e.to_string())?;
        let themes = diagramcraft_theme::Themes::from_document(&doc).map_err(|e| e.to_string())?;
        let mut e = Evaluator::new(&doc).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&diagramcraft_text::Host);
        let scopes: Vec<_> = e.scopes().collect();
        let mut count = 0;
        for scope in scopes {
            let Some(named) = e.source_named_cells(scope).map_err(|e| e.to_string())?.into_iter().find(|n| n.name.eq_ignore_ascii_case(&cell)) else {
                continue;
            };
            if cell == "TxtWidth" && !e.effective_formula(scope, &cell).is_some_and(|f| f.contains("TEXTWIDTH")) {
                continue;
            }
            let expected = e.cached_cell(named.cell);
            let actual = e.evaluate_snapshot(scope, &cell);
            if let (Ok(a), Ok(b)) = (actual.numeric(), expected.numeric())
                && (a - b).abs() <= 1e-6
            {
                continue;
            }
            let Ok(body) = e.text_body(scope) else { continue };
            println!("CELL\t{file}\t{scope:?}\t{cell}\t{actual:?}\t{expected:?}\ttext={:?}", body.text());
            for name in ["LeftMargin", "RightMargin", "TxtPinX", "TxtWidth", "Width", "Height", "Char.Size", "Char.Font", "Para.SpLine"] {
                println!("  {name}\t{:?}\t{:?}", e.effective_formula(scope, name), e.evaluate_snapshot(scope, name));
            }
            println!("  HOST {:?} BODY {:?}", diagramcraft_text::Host.measure(&body, None), body.body);
            for p in &body.paragraphs {
                for r in &p.runs {
                    let props = &r.props;
                    let family = props.font.as_deref().unwrap_or("Calibri");
                    let style = match (props.bold.unwrap_or(false), props.italic.unwrap_or(false)) {
                        (true, true) => "Bold Italic",
                        (true, false) => "Bold",
                        (false, true) => "Italic",
                        _ => "Regular",
                    };
                    let face = FontDb::global().face(family, style);
                    let size = props.size.unwrap_or(12.);
                    let features: Vec<_> = feature("-kern").into_iter().collect();
                    let advances: Vec<_> =
                        shape(&face, &r.text, &features, |c| c).iter().map(|g| f64::from(g.x_advance) * size / face.upem).collect();
                    let kern: f64 = shape(&face, &r.text, &[], |c| c).iter().map(|g| f64::from(g.x_advance) * size / face.upem).sum();
                    let space: f64 = shape(&face, " ", &features, |c| c).iter().map(|g| f64::from(g.x_advance) * size / face.upem).sum();
                    println!(
                        "  RUN text={:?} font={family:?} face={:?} size_pt={size} advances_pt={advances:?} kern_pt={kern} space_pt={space}",
                        r.text, face.family
                    );
                }
            }
            count += 1;
            if count >= limit {
                break;
            }
        }
        println!("COUNT {file} {count}");
    }
    Ok(())
}
