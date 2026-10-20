//! Stencil contact sheets use the same master renderer and embedded font outlines.
use crate::scene::{self, Op, Options, Paint, Scene};
use kurbo::Shape as _;
use misterdiagram_color::Rgba;
use misterdiagram_geom::{Affine, Rect};
use misterdiagram_model::{Document, text::*};
use misterdiagram_text::{NoFields, Opts};
use misterdiagram_vsdx::evaluate::Evaluator;
const COLUMNS: usize = 6;
const CELL_WIDTH: usize = 240;
const CELL_HEIGHT: usize = 200;
/// One labelled cell per master, in document order. Errors remain visible and are returned.
#[derive(Debug, Default)]
pub struct Report {
    pub rendered: usize,
    pub blank: usize,
    pub failed: usize,
    pub warnings: Vec<String>,
}
/// Test only drawing pixels, before names and failure markers are overlaid.
pub fn has_ink(image: &image::RgbaImage) -> bool {
    image.pixels().any(|p| p.0[3] > 0 && p.0.iter().take(3).any(|c| *c < 255))
}
fn dimensions(doc: &Document) -> Result<(usize, usize), String> {
    if doc.masters.is_empty() || doc.masters.len() > 4096 {
        return Err("stencil master count limit".into());
    }
    let rows = doc.masters.len().div_ceil(COLUMNS);
    let width = COLUMNS * CELL_WIDTH;
    let height = rows.checked_mul(CELL_HEIGHT).ok_or("contact height overflow")?;
    if width.checked_mul(height).is_none_or(|n| n > scene::MAX_PIXELS) {
        return Err("contact sheet pixel limit".into());
    }
    Ok((width, height))
}
pub fn render(doc: &Document) -> Result<(image::RgbaImage, Report), String> {
    let size = dimensions(doc)?;
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(|e| e.to_string());
    let host = misterdiagram_text::Host;
    render_with(doc, size, || {
        let themes = themes.as_ref().map_err(Clone::clone)?;
        Ok(Evaluator::new(doc).map_err(|e| e.to_string())?.with_theme_host(themes).with_text_host(&host))
    })
}
fn render_with<'a>(
    doc: &'a Document,
    (width, height): (usize, usize),
    mut prepare: impl FnMut() -> Result<Evaluator<'a>, String>,
) -> Result<(image::RgbaImage, Report), String> {
    let (columns, cell_width, cell_height) = (COLUMNS, CELL_WIDTH, CELL_HEIGHT);
    let mut evaluator = prepare();
    let mut pixels = image::RgbaImage::from_pixel(width as u32, height as u32, image::Rgba([255, 255, 255, 255]));
    let mut report = Report::default();
    for (index, master) in doc.masters.iter().enumerate() {
        let x = (index % columns) * cell_width;
        let y = (index / columns) * cell_height;
        let mut failed = false;
        match evaluator.as_mut().map_err(|e| e.clone()).and_then(|e| scene::master_prepared(e, index, Options::default())).and_then(|scene| {
            report.warnings.extend(scene.warnings.iter().map(|w| format!("master {} {}: {w}", master.id, master.name_u)));
            let scale = ((cell_width - 24) as f64 / (scene.bounds.width() * 96.).max(1.))
                .min((cell_height - 40) as f64 / (scene.bounds.height() * 96.).max(1.))
                .min(2.);
            crate::raster::render(&scene, Options { scale, ..Default::default() })
        }) {
            Ok(img) => {
                if has_ink(&img) {
                    report.rendered += 1;
                } else {
                    report.blank += 1;
                    report.warnings.push(format!("master {} {}: blank drawing area", master.id, master.name_u));
                }
                image::imageops::overlay(&mut pixels, &img, (x + (cell_width.saturating_sub(img.width() as usize)) / 2) as i64, (y + 8) as i64)
            }
            Err(e) => {
                failed = true;
                report.failed += 1;
                report.warnings.push(format!("master {} {}: {e}", master.id, master.name_u));
            }
        }
        let body = TextBody {
            paragraphs: vec![Paragraph {
                runs: vec![Run {
                    text: master.name_u.clone(),
                    kind: RunKind::Text,
                    props: RunProps { font: Some("Carlito".into()), size: Some(8.), ..Default::default() },
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let layout = misterdiagram_text::layout(
            &body,
            &Opts { rect: Rect::new(0., 0., (cell_width - 12) as f64 * 0.75, 22.), fields: &NoFields, prompt_color: None, no_shrink: false },
        )
        .map_err(|e| e.to_string())?;
        let label = Scene {
            ops: vec![Op::Text { layout, transform: Affine::translate((0., 24. / 96.)) * Affine::scale_non_uniform(1. / 72., -1. / 72.) }],
            bounds: Rect::new(0., 0., (cell_width - 12) as f64 / 96., 24. / 96.),
            page_size: (1., 1.),
            warnings: vec![],
        };
        let img = crate::raster::render(&label, Options::default())?;
        image::imageops::overlay(&mut pixels, &img, (x + 6) as i64, (y + cell_height - 28) as i64);
        if failed {
            let error = Scene {
                ops: vec![Op::Path { path: Rect::new(0., 0., 1., 0.05).to_path(0.001), paint: Paint::Solid(Rgba::rgb(200, 0, 0)) }],
                bounds: Rect::new(0., 0., 1., 0.05),
                page_size: (1., 1.),
                warnings: vec![],
            };
            image::imageops::overlay(&mut pixels, &crate::raster::render(&error, Options::default())?, (x + 12) as i64, (y + 12) as i64);
        }
    }
    Ok((pixels, report))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn labels_do_not_turn_blank_masters_into_rendered_masters() {
        let doc = Document {
            masters: vec![misterdiagram_model::Master { id: 1, name_u: "Empty master".into(), ..Default::default() }],
            ..Default::default()
        };
        let (image, report) = render(&doc).unwrap();
        assert!(has_ink(&image), "the label is visible");
        assert_eq!((report.rendered, report.blank, report.failed), (0, 1, 0));
        assert!(report.warnings.iter().any(|s| s.contains("blank drawing area")));
    }
    #[test]
    fn contact_sheet_builds_one_evaluator_for_all_masters() {
        let doc = Document {
            masters: (1..=8)
                .map(|id| misterdiagram_model::Master {
                    id,
                    name_u: format!("Master {id}"),
                    shapes: vec![misterdiagram_model::skeleton::rectangle(1, 1., 1., 1., 1., "").unwrap()].into(),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        };
        let mut builds = 0;
        let (pixels, report) = render_with(&doc, dimensions(&doc).unwrap(), || {
            builds += 1;
            Evaluator::new(&doc).map_err(|e| e.to_string())
        })
        .unwrap();
        assert_eq!(builds, 1, "one evaluator must serve the entire contact sheet");
        assert_eq!((report.rendered, report.blank, report.failed), (8, 0, 0));
        assert_eq!(pixels.dimensions(), (1440, 400));
    }
    #[test]
    fn preparation_failure_is_reported_in_every_contact_cell() {
        let doc = Document {
            masters: (1..=2).map(|id| misterdiagram_model::Master { id, name_u: format!("M{id}"), ..Default::default() }).collect(),
            ..Default::default()
        };
        let mut attempts = 0;
        let (pixels, report) = render_with(&doc, dimensions(&doc).unwrap(), || {
            attempts += 1;
            Err("synthetic preparation limit".into())
        })
        .unwrap();
        assert_eq!(attempts, 1);
        assert_eq!((report.rendered, report.blank, report.failed), (0, 0, 2));
        assert_eq!(report.warnings, ["master 1 M1: synthetic preparation limit", "master 2 M2: synthetic preparation limit"]);
        assert!(pixels.pixels().any(|p| p.0 == [200, 0, 0, 255]), "failure markers remain visible");
    }
    #[test]
    fn invalid_theme_and_evaluator_limit_keep_per_master_failure_markers() {
        let mut doc = Document {
            masters: (1..=2).map(|id| misterdiagram_model::Master { id, name_u: format!("M{id}"), ..Default::default() }).collect(),
            ..Default::default()
        };
        doc.parts.insert("visio/theme/theme1.xml".into(), misterdiagram_model::Part { bytes: b"<notTheme/>".to_vec().into(), ..Default::default() });
        for error in ["#VALUE!", "#LIMIT!"] {
            let (pixels, report) = render(&doc).unwrap();
            assert_eq!((report.rendered, report.blank, report.failed), (0, 0, 2));
            assert_eq!(report.warnings, [format!("master 1 M1: {error}"), format!("master 2 M2: {error}")]);
            assert!(pixels.pixels().any(|p| p.0 == [200, 0, 0, 255]));
            doc.parts.clear();
            doc.colors = (0..=65536).map(|index| (index, "#000000".into())).collect();
        }
    }
}
