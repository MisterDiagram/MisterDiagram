//! P3 round-trip cells and original-versus-rewritten rendering. Oracle inputs are read-only.
use crate::render::{FileResult, Group};
use diagramcraft_model::Document;
use diagramcraft_render::scene::{self, Options};
use std::{collections::BTreeMap, fmt, path::Path};
#[derive(Debug, Default)]
pub struct Report {
    pub packages: usize,
    pub cells: diagramcraft_vsdx::roundtrip::Report,
    pub groups: BTreeMap<String, Group>,
    pub errors: Vec<String>,
    pub error_count: usize,
    pub writer_warnings: usize,
    pub scene_warnings: [usize; 2],
}
impl Report {
    pub fn passed(&self) -> bool {
        self.packages > 0
            && self.error_count == 0
            && self.cells.passed()
            && !self.groups.is_empty()
            && self.groups.iter().all(|(n, g)| g.passed(n == "text"))
    }
    fn error(&mut self, message: String) {
        self.error_count += 1;
        if self.errors.len() < 100 {
            self.errors.push(message);
        }
    }
}
impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "roundtrip cells: packages={} passed={}/{} rate={:.6}% floor={:.6}% errors={} writer_warnings={} scene_warnings={:?}",
            self.packages,
            self.cells.matched,
            self.cells.total,
            self.cells.percent(),
            diagramcraft_vsdx::roundtrip::FLOOR,
            self.error_count,
            self.writer_warnings,
            self.scene_warnings
        )?;
        let mut failures: Vec<_> = self.cells.failures.iter().collect();
        failures.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        for (name, count) in failures {
            writeln!(f, "roundtrip cell mismatch {name}: {count}")?;
        }
        for (name, g) in &self.groups {
            let mut worst: Vec<_> = g.files.iter().collect();
            worst.sort_by(|a, b| b.mad.unwrap_or(f64::INFINITY).total_cmp(&a.mad.unwrap_or(f64::INFINITY)).then(a.file.cmp(&b.file)));
            writeln!(
                f,
                "roundtrip render {name}: images={} errors={} size_mismatches={} mean_MAD={:?} fraction_over16={:?} within_ceiling={}",
                g.files.len(),
                g.files.iter().filter(|f| f.error.is_some()).count(),
                g.files.iter().filter(|f| f.actual.is_some_and(|a| a != f.expected)).count(),
                g.mean_mad(),
                g.fraction(),
                g.passed(name == "text")
            )?;
            for file in worst.into_iter().take(3) {
                writeln!(
                    f,
                    "roundtrip worst {name}: {} original={:?} rewritten={:?} MAD={:?} fraction={:?} error={:?}",
                    file.file, file.expected, file.actual, file.mad, file.differing, file.error
                )?;
            }
        }
        for group in self.groups.values() {
            for file in &group.files {
                if let Some(error) = &file.error {
                    writeln!(f, "roundtrip render error {}: {error}", file.file)?;
                }
            }
        }
        for e in &self.errors {
            writeln!(f, "roundtrip error: {e}")?;
        }
        writeln!(f, "roundtrip result={}", if self.passed() { "PASS" } else { "FAIL" })
    }
}
fn image_pair(report: &mut Report, before: &Document, after: &Document, name: &str, group: &str, index: usize, master: bool) {
    let mut result = FileResult {
        file: format!("{name}#{}{}", if master { "master" } else { "page" }, index + 1),
        expected: (0, 0),
        actual: None,
        mad: None,
        differing: None,
        error: None,
    };
    let compare = (|| -> Result<(), String> {
        let options = Options::default();
        let a = if master { scene::master(before, index, options) } else { scene::page(before, index, options) }?;
        let mut b = if master { scene::master(after, index, options) } else { scene::page(after, index, options) }?;
        report.scene_warnings[0] += a.warnings.len();
        report.scene_warnings[1] += b.warnings.len();
        // Master comparisons fit the original drawing into a fixed bounded cell. The same
        // scale is used for its rewrite, so bounds changes cannot be hidden by independent fits.
        let options = if master {
            Options { scale: (216. / (a.bounds.width() * 96.).max(1.)).min(160. / (a.bounds.height() * 96.).max(1.)).min(2.), ..options }
        } else {
            options
        };
        let (aw, ah, _) = a.canvas(options)?;
        let (bw, bh, _) = b.canvas(options)?;
        if aw.abs_diff(bw) > 1 || ah.abs_diff(bh) > 1 {
            return Err(format!("native size mismatch: {aw}x{ah} versus {bw}x{bh}"));
        }
        // Keep the original pixel denominator. A distant rewrite must not dilute
        // changed ink with a large white union viewport.
        if (a.bounds.x0 - b.bounds.x0).abs() * 96. * options.scale > 1. || (a.bounds.y1 - b.bounds.y1).abs() * 96. * options.scale > 1. {
            return Err(format!("viewport origin changed: {:?} versus {:?}", a.bounds, b.bounds));
        }
        b.bounds = a.bounds;
        let a = diagramcraft_render::raster::render(&a, options)?;
        result.expected = a.dimensions();
        let b = diagramcraft_render::raster::render(&b, options)?;
        result.actual = Some(b.dimensions());
        let (mad, fraction) = crate::render::compare(&a, &b)?;
        result.mad = Some(mad);
        result.differing = Some(fraction);
        Ok(())
    })();
    if let Err(e) = compare {
        result.error = Some(e);
    }
    report.groups.entry(group.into()).or_default().files.push(result);
}
pub fn run(root: &Path, quick: bool) -> Result<Report, String> {
    let mut files = vec![];
    for dir in ["generated", "blank"] {
        files.extend(super::walk(&root.join(dir))?.into_iter().filter(|p| super::package(p)));
    }
    files.extend(super::walk(&root.join("ms-shipped"))?.into_iter().filter(|p| super::package(p)).take(if quick { 50 } else { usize::MAX }));
    files.sort();
    let mut report = Report::default();
    let total = files.len();
    for file in files {
        report.packages += 1;
        if report.packages % 100 == 0 {
            eprintln!("roundtrip package progress: {}/{total}", report.packages);
        }
        let name = file.strip_prefix(root).unwrap_or(&file).display().to_string();
        let result = (|| -> Result<(), String> {
            let before = diagramcraft_vsdx::read(&file).map_err(|e| e.to_string())?;
            let theme = diagramcraft_theme::Themes::from_document(&before).map_err(|e| e.to_string())?;
            let written = diagramcraft_vsdx::write_with_hosts(&before, Some(&theme), Some(&diagramcraft_text::Host)).map_err(|e| e.to_string())?;
            report.writer_warnings += written.warning_count;
            let after = diagramcraft_vsdx::import(&written.bytes).map_err(|e| e.to_string())?;
            let after_theme = diagramcraft_theme::Themes::from_document(&after).map_err(|e| e.to_string())?;
            report.cells.merge(diagramcraft_vsdx::roundtrip::compare(
                &before,
                &after,
                Some(&theme),
                Some(&after_theme),
                Some(&diagramcraft_text::Host),
            )?);
            if before.pages.len() != after.pages.len() || before.masters.len() != after.masters.len() {
                return Err("page/master count changed".into());
            }
            let group = if file.starts_with(root.join("generated")) {
                file.parent().and_then(|p| p.file_name()).and_then(|s| s.to_str()).unwrap_or("generated")
            } else if file.starts_with(root.join("blank")) {
                "blank"
            } else {
                "shipped-pages"
            };
            for (i, page) in before.pages.iter().enumerate() {
                if !page.part.is_empty() {
                    image_pair(&mut report, &before, &after, &name, group, i, false);
                }
            }
            for i in 0..before.masters.len() {
                image_pair(&mut report, &before, &after, &name, "masters", i, true);
            }
            Ok(())
        })();
        if let Err(e) = result {
            report.error(format!("{name}: {e}"));
        }
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rewritten_geometry_changes_are_visible_to_the_render_gate() {
        let mut before = diagramcraft_model::skeleton::document(false);
        before.pages[0].shapes.push(diagramcraft_model::skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap());
        let mut after = before.clone();
        after.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap().value = Some("4".into());
        let mut r = Report::default();
        image_pair(&mut r, &before, &before, "same", "basic", 0, false);
        assert!(r.groups["basic"].passed(false));
        image_pair(&mut r, &before, &after, "changed", "basic", 0, false);
        assert!(!r.groups["basic"].passed(false));
        assert_eq!(r.groups["basic"].files.len(), 2);
        let mut translated = before.clone();
        translated.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "PinX").unwrap().value = Some("5".into());
        let mut r = Report::default();
        image_pair(&mut r, &before, &translated, "translated", "basic", 0, false);
        assert!(!r.groups["basic"].passed(false));
        translated.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "PinX").unwrap().value = Some("301".into());
        let mut r = Report::default();
        image_pair(&mut r, &before, &translated, "distant", "basic", 0, false);
        assert!(!r.groups["basic"].passed(false));
    }
}
