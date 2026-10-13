//! P3 round-trip cells and original-versus-rewritten rendering. Oracle inputs are read-only.
use crate::render::{FileResult, Group};
use diagramcraft_render::scene::{self, Options};
use diagramcraft_vsdx::evaluate::Evaluator;
use std::{collections::BTreeMap, fmt, path::Path};
#[derive(Debug, Default)]
pub struct Report {
    pub packages: usize,
    pub cells: diagramcraft_vsdx::roundtrip::Report,
    pub groups: BTreeMap<String, Group>,
    pub equal_errors: BTreeMap<String, usize>,
    pub equal_error_witnesses: BTreeMap<String, Vec<String>>,
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
            && self.groups.iter().all(|(n, g)| self.group_passed(n, g))
    }
    fn group_passed(&self, name: &str, group: &Group) -> bool {
        if group.files.is_empty() { self.equal_errors.get(name).is_some_and(|n| *n > 0) } else { group.passed(name == "text") }
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
        writeln!(
            f,
            "roundtrip evaluation-only diagnostic: passed={}/{} rate={:.6}%",
            self.cells.evaluation_only_matched,
            self.cells.total,
            100. * self.cells.evaluation_only_matched as f64 / self.cells.total.max(1) as f64
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
                "roundtrip render {name}: images={} equal_errors={} errors={} size_mismatches={} mean_MAD={:?} fraction_over16={:?} within_ceiling={}",
                g.files.len().saturating_add(self.equal_errors.get(name).copied().unwrap_or(0)),
                self.equal_errors.get(name).copied().unwrap_or(0),
                g.files.iter().filter(|f| f.error.is_some()).count(),
                g.files.iter().filter(|f| f.actual.is_some_and(|a| a != f.expected)).count(),
                g.mean_mad(),
                g.fraction(),
                self.group_passed(name, g)
            )?;
            for file in worst.into_iter().take(3) {
                writeln!(
                    f,
                    "roundtrip worst {name}: {} original={:?} rewritten={:?} MAD={:?} fraction={:?} error={:?}",
                    file.file, file.expected, file.actual, file.mad, file.differing, file.error
                )?;
            }
        }
        for (group, witnesses) in &self.equal_error_witnesses {
            for witness in witnesses {
                writeln!(f, "roundtrip equal-error witness {group}: {witness}")?;
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
enum Pair<T> {
    Values(T, T),
    EqualError(String),
}
fn pair_results<T>(a: Result<T, String>, b: Result<T, String>, stage: &str) -> Result<Pair<T>, String> {
    match (a, b) {
        (Ok(a), Ok(b)) => Ok(Pair::Values(a, b)),
        (Err(a), Err(b)) if a == b => Ok(Pair::EqualError(format!("{stage}: {a}"))),
        (Err(a), Err(b)) => Err(format!("different {stage} errors: original={a}; rewritten={b}")),
        (Err(a), Ok(_)) => Err(format!("original-only {stage} error: {a}")),
        (Ok(_), Err(b)) => Err(format!("rewritten-only {stage} error: {b}")),
    }
}
fn image_pair(report: &mut Report, before: &mut Evaluator<'_>, after: &mut Evaluator<'_>, name: &str, group: &str, index: usize, master: bool) {
    let mut result = FileResult {
        file: format!("{name}#{}{}", if master { "master" } else { "page" }, index + 1),
        expected: (0, 0),
        actual: None,
        mad: None,
        differing: None,
        error: None,
    };
    let mut equal_error = None;
    let compare = (|| -> Result<(), String> {
        // Both sides are evaluated before classification, even if the original fails.
        macro_rules! pair {
            ($a:expr, $b:expr, $stage:literal) => {
                match pair_results($a, $b, $stage)? {
                    Pair::Values(a, b) => (a, b),
                    Pair::EqualError(e) => {
                        equal_error = Some(e);
                        return Ok(());
                    }
                }
            };
        }
        let options = Options::default();
        let (a, mut b) = pair!(
            if master { scene::master_prepared(before, index, options) } else { scene::page_prepared(before, index, options) },
            if master { scene::master_prepared(after, index, options) } else { scene::page_prepared(after, index, options) },
            "scene"
        );
        report.scene_warnings[0] += a.warnings.len();
        report.scene_warnings[1] += b.warnings.len();
        // Fit masters to the original viewport and retain its scale for the rewrite.
        let options = if master {
            Options { scale: (216. / (a.bounds.width() * 96.).max(1.)).min(160. / (a.bounds.height() * 96.).max(1.)).min(2.), ..options }
        } else {
            options
        };
        let ((aw, ah, _), (bw, bh, _)) = pair!(a.canvas(options), b.canvas(options), "canvas");
        if aw.abs_diff(bw) > 1 || ah.abs_diff(bh) > 1 {
            return Err(format!("native size mismatch: {aw}x{ah} versus {bw}x{bh}"));
        }
        if (a.bounds.x0 - b.bounds.x0).abs() * 96. * options.scale > 1. || (a.bounds.y1 - b.bounds.y1).abs() * 96. * options.scale > 1. {
            return Err(format!("viewport origin changed: {:?} versus {:?}", a.bounds, b.bounds));
        }
        b.bounds = a.bounds;
        let (a, b) = pair!(diagramcraft_render::raster::render(&a, options), diagramcraft_render::raster::render(&b, options), "raster");
        result.expected = a.dimensions();
        result.actual = Some(b.dimensions());
        let (mad, fraction) = crate::render::compare(&a, &b)?;
        result.mad = Some(mad);
        result.differing = Some(fraction);
        Ok(())
    })();
    if let Some(error) = equal_error {
        let count = report.equal_errors.entry(group.into()).or_default();
        *count = count.saturating_add(1);
        let witnesses = report.equal_error_witnesses.entry(group.into()).or_default();
        if witnesses.len() < 3 {
            witnesses.push(format!("{}: {error}", result.file));
        }
        report.groups.entry(group.into()).or_default();
        return;
    }
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
            let mut before_eval =
                Evaluator::new(&before).map_err(|e| e.to_string())?.with_theme_host(&theme).with_text_host(&diagramcraft_text::Host);
            let mut after_eval =
                Evaluator::new(&after).map_err(|e| e.to_string())?.with_theme_host(&after_theme).with_text_host(&diagramcraft_text::Host);
            for (i, page) in before.pages.iter().enumerate() {
                if !page.part.is_empty() {
                    image_pair(&mut report, &mut before_eval, &mut after_eval, &name, group, i, false);
                }
            }
            for i in 0..before.masters.len() {
                image_pair(&mut report, &mut before_eval, &mut after_eval, &name, "masters", i, true);
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
    fn image_pair(
        report: &mut Report,
        before: &diagramcraft_model::Document,
        after: &diagramcraft_model::Document,
        name: &str,
        group: &str,
        index: usize,
        master: bool,
    ) {
        let a = diagramcraft_theme::Themes::from_document(before).unwrap();
        let b = diagramcraft_theme::Themes::from_document(after).unwrap();
        let mut ea = Evaluator::new(before).unwrap().with_theme_host(&a).with_text_host(&diagramcraft_text::Host);
        let mut eb = Evaluator::new(after).unwrap().with_theme_host(&b).with_text_host(&diagramcraft_text::Host);
        super::image_pair(report, &mut ea, &mut eb, name, group, index, master);
    }
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
    #[test]
    fn identical_errors_are_equal_but_one_sided_or_changed_errors_fail() {
        let mut good = diagramcraft_model::skeleton::document(false);
        good.pages[0].shapes.push(diagramcraft_model::skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap());
        let mut bad = good.clone();
        bad.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "PinX").unwrap().formula = Some("SQRT(-1)".into());
        let mut different = good.clone();
        different.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "PinX").unwrap().formula = Some("1/0".into());
        let mut r = Report::default();
        image_pair(&mut r, &bad, &bad, "same error", "basic", 0, false);
        assert_eq!(r.equal_errors["basic"], 1);
        assert!(r.groups["basic"].files.is_empty());
        assert!(r.group_passed("basic", &r.groups["basic"]));
        assert!(r.to_string().contains("images=1 equal_errors=1 errors=0"));
        for (a, b) in [(&good, &bad), (&bad, &good), (&bad, &different)] {
            let mut r = Report::default();
            image_pair(&mut r, a, b, "not equal", "basic", 0, false);
            assert!(r.equal_errors.is_empty());
            assert!(!r.group_passed("basic", &r.groups["basic"]));
            assert!(r.groups["basic"].files[0].error.is_some());
        }
    }
}
