//! Exact raster self-consistency of the export intent. No native oracle edits.
use misterdiagram_render::scene::{Options, SceneBudget, evaluated};
use std::{fmt, path::Path};

/// Amendment 2, frozen on 2026-11-03: the prior 41 pages plus six
/// connector pages with dropped jumps or fitted routes. This ceiling may only fall.
pub const MAX_SUBSTITUTED_PAGES: usize = 47;

#[derive(Default, Debug)]
pub struct Report {
    pub files: usize,
    pub pages: usize,
    pub identical: usize,
    pub labelled_shapes: usize,
    pub grouped_labels: usize,
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
            "PPTX model: {} files, {} pages, {} identical, {} substitutions excluded (ceiling {MAX_SUBSTITUTED_PAGES}), {} failures",
            self.files,
            self.pages,
            self.identical,
            self.excluded.len(),
            self.failures.len()
        )?;
        writeln!(f, "PPTX labels: {} labelled 2-D shapes, {} grouped separate labels", self.labelled_shapes, self.grouped_labels)?;
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
        let (labelled, grouped) = check_labels(&model, options)?;
        report.labelled_shapes += labelled;
        report.grouped_labels += grouped;
        let connector_only = model.substitutions.iter().all(|s| matches!(s, crate::model::Substitution::Jumps | crate::model::Substitution::Route));
        if !model.substitutions.is_empty() && !connector_only {
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
        } else if !model.substitutions.is_empty() && connector_only {
            report.excluded.push(format!(
                "{} page {}: {}",
                path.display(),
                index + 1,
                model.substitutions.iter().map(|s| s.name()).collect::<Vec<_>>().join(", ")
            ));
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
                report.labelled_shapes += r.labelled_shapes;
                report.grouped_labels += r.grouped_labels;
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

/// Check serialized membership, not merely the intent count. This makes a
/// writer-only sibling defect fail the same corpus command as the raster gate.
pub fn check_labels(page: &crate::model::Page, options: Options) -> Result<(usize, usize), String> {
    use misterdiagram_opc::{Package, xml::El};
    use std::collections::{BTreeMap, BTreeSet};
    fn collect<'a>(node: &'a El, parent: Option<u32>, depth: usize, out: &mut BTreeMap<u32, (Option<u32>, &'a El)>) -> Result<(), String> {
        if depth > 129 || out.len() > 200_000 {
            return Err("PPTX label readback limit".into());
        }
        for child in node.elements() {
            let property = match child.local() {
                "sp" => "nvSpPr",
                "cxnSp" => "nvCxnSpPr",
                "grpSp" => "nvGrpSpPr",
                "pic" => "nvPicPr",
                _ => continue,
            };
            let id = child.path(&[property, "cNvPr"]).and_then(|c| c.u32("id")).ok_or("PPTX label readback id")?;
            if out.insert(id, (parent, child)).is_some() {
                return Err("PPTX duplicate slide id".into());
            }
            if child.is("grpSp") {
                let xf = child.path(&["grpSpPr", "xfrm"]).ok_or("PPTX group transform missing")?;
                for (a, b, names) in [("off", "chOff", ["x", "y"]), ("ext", "chExt", ["cx", "cy"])] {
                    let a = xf.child(a).ok_or("PPTX group coordinate missing")?;
                    let b = xf.child(b).ok_or("PPTX group coordinate missing")?;
                    for name in names {
                        if a.attr(name).is_none() || a.attr(name) != b.attr(name) {
                            return Err("PPTX group changes child coordinates".into());
                        }
                    }
                }
                collect(child, Some(id), depth + 1, out)?;
            }
        }
        Ok(())
    }
    let export = crate::write::render_limited(std::slice::from_ref(page), &[misterdiagram_color::Rgba::BLACK; 12], options, 256 * 1024 * 1024)?;
    let package = Package::open(&export.bytes).map_err(|e| e.to_string())?;
    let xml = package.xml("ppt/slides/slide1.xml").ok_or("PPTX label slide XML missing")?;
    let tree = xml.root.path(&["cSld", "spTree"]).ok_or("PPTX shape tree missing")?;
    let mut nodes = BTreeMap::new();
    collect(tree, None, 0, &mut nodes)?;
    let actual: BTreeSet<_> = nodes.iter().filter(|(_, (_, n))| n.child("txBody").is_some()).map(|(id, _)| *id).collect();
    let expected: BTreeSet<_> = page.labels.nodes.values().filter_map(|n| n.text_id).collect();
    if actual != expected {
        return Err("PPTX label coverage differs from intent".into());
    }
    let mut labelled = 0;
    let mut grouped = 0;
    for label in page.labels.nodes.values() {
        let Some(text_id) = label.text_id else {
            continue;
        };
        let (shape_parent, _) = nodes.get(&label.shape_id).ok_or("PPTX labelled shape missing")?;
        let (text_parent, _) = nodes.get(&text_id).ok_or("PPTX label missing")?;
        if *shape_parent != label.parent_id || shape_parent != text_parent {
            return Err("PPTX shape and label parent differs from intent".into());
        }
        if !label.one_d {
            labelled += 1;
            if text_id != label.shape_id {
                if label.group_id.is_none() {
                    return Err("PPTX separate label has no group".into());
                }
                grouped += 1;
            }
        }
    }
    Ok((labelled, grouped))
}
