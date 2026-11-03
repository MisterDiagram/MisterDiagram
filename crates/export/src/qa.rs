//! Exact raster self-consistency of the export intent. No native oracle edits.
use misterdiagram_render::scene::{Options, SceneBudget, evaluated};
use std::{fmt, path::Path};

/// Amendment 1 A2: first measured on 2026-11-03, 39 pattern pages, one
/// compound page and one shadow page. This ceiling may only fall.
pub const MAX_SUBSTITUTED_PAGES: usize = 41;

#[derive(Default, Debug)]
pub struct Report {
    pub files: usize,
    pub pages: usize,
    pub identical: usize,
    pub excluded: Vec<String>,
    pub failures: Vec<String>,
}
impl Report {
    pub fn passed(&self) -> bool {
        self.files > 0 && self.pages > 0 && self.identical + self.excluded.len() == self.pages && self.failures.is_empty()
    }
}
impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "PPTX model: {} files, {} pages, {} identical, {} substitutions excluded (ceiling 41), {} failures",
            self.files,
            self.pages,
            self.identical,
            self.excluded.len(),
            self.failures.len()
        )?;
        for name in &self.excluded {
            writeln!(f, "PPTX excluded: {name}")?;
        }
        for failure in &self.failures {
            writeln!(f, "PPTX failure: {failure}")?;
        }
        Ok(())
    }
}
pub fn file(path: &Path) -> Result<Report, String> {
    if std::fs::metadata(path).map_err(|e| e.to_string())?.len() > 256 * 1024 * 1024 {
        return Err("PPTX QA input limit".into());
    }
    let doc = misterdiagram_vsdx::read(path).map_err(|e| e.to_string())?;
    let options = Options { visio_export: true, cached_fallback: true, ..Default::default() };
    let mut report = Report { files: 1, ..Default::default() };
    let mut budget = SceneBudget::default();
    for (index, p) in doc.pages.iter().enumerate() {
        if p.attributes.get("Background").is_some_and(|s| s == "1" || s == "true") {
            continue;
        }
        report.pages += 1;
        let page = evaluated::page(&doc, index, options, &mut budget)?;
        let source = misterdiagram_render::raster::render(&page.scene, options)?;
        let model = crate::model::Page::with_options(page, options, &mut budget)?;
        if !model.substitutions.is_empty() {
            report.excluded.push(format!(
                "{} page {}: {}",
                path.display(),
                index + 1,
                model.substitutions.iter().map(|s| s.name()).collect::<Vec<_>>().join(", ")
            ));
            continue;
        }
        let actual = misterdiagram_render::raster::render(&model.raster_scene()?, options)?;
        if source == actual {
            report.identical += 1;
        } else {
            let count = source.as_raw().iter().zip(actual.as_raw()).filter(|(a, b)| a != b).count();
            report.failures.push(format!(
                "{} page {}: {} differing channels; {:?} versus {:?}",
                path.display(),
                index + 1,
                count,
                source.dimensions(),
                actual.dimensions()
            ));
        }
    }
    Ok(report)
}
pub fn generated(root: &Path) -> Result<Report, String> {
    let mut paths = Vec::new();
    for family in std::fs::read_dir(root).map_err(|e| e.to_string())? {
        let family = family.map_err(|e| e.to_string())?;
        if !family.file_type().map_err(|e| e.to_string())?.is_dir() {
            continue;
        }
        for entry in std::fs::read_dir(family.path()).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.path().extension().is_some_and(|s| s == "vsdx") {
                if paths.len() >= 4096 {
                    return Err("PPTX QA file limit".into());
                }
                paths.push(entry.path());
            }
        }
    }
    paths.sort();
    let mut report = Report::default();
    for path in paths {
        match file(&path) {
            Ok(r) => {
                report.files += r.files;
                report.pages += r.pages;
                report.identical += r.identical;
                report.excluded.extend(r.excluded);
                report.failures.extend(r.failures);
            }
            Err(e) => report.failures.push(format!("{}: {e}", path.display())),
        }
    }
    if report.files != 230 {
        report.failures.push(format!("generated PPTX witness count: {} instead of 230", report.files));
    }
    if report.excluded.len() > MAX_SUBSTITUTED_PAGES {
        report.failures.push(format!("PPTX substitution count {} exceeds frozen {}", report.excluded.len(), MAX_SUBSTITUTED_PAGES));
    }
    Ok(report)
}
