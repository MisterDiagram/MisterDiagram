//! Frozen P2 pixel oracle. A dimension mismatch is always a file failure.
use std::{
    collections::BTreeMap,
    fmt,
    path::{Path, PathBuf},
};
#[derive(Debug)]
pub struct FileResult {
    pub file: String,
    pub expected: (u32, u32),
    pub actual: Option<(u32, u32)>,
    pub mad: Option<f64>,
    pub differing: Option<f64>,
    pub error: Option<String>,
}
#[derive(Debug, Default)]
pub struct Group {
    pub files: Vec<FileResult>,
}
#[derive(Debug, Default)]
pub struct Report {
    pub groups: BTreeMap<String, Group>,
}
impl Group {
    pub fn mean_mad(&self) -> Option<f64> {
        let values: Vec<_> = self.files.iter().filter_map(|f| f.mad).collect();
        (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
    }
    pub fn fraction(&self) -> Option<f64> {
        let mut pixels = 0.;
        let mut different = 0.;
        for file in &self.files {
            if let Some(fraction) = file.differing {
                let n = f64::from(file.expected.0) * f64::from(file.expected.1);
                pixels += n;
                different += n * fraction;
            }
        }
        (pixels > 0.).then(|| different / pixels)
    }
    pub fn passed(&self, text: bool) -> bool {
        let (mad, fraction) = if text { (8., 0.05) } else { (4., 0.02) };
        !self.files.is_empty()
            && self.files.iter().all(|f| f.error.is_none() && f.actual == Some(f.expected) && f.mad.is_some() && f.differing.is_some())
            && self.mean_mad().is_some_and(|v| v <= mad)
            && self.fraction().is_some_and(|v| v <= fraction)
    }
}
impl Report {
    pub fn passed(&self) -> bool {
        !self.groups.is_empty() && self.groups.iter().all(|(name, group)| group.passed(name == "text"))
    }
}
impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "render oracle: RGB MAD in 0..255; differing means any RGB channel exceeds 16; frozen non-text ceilings=4/255,2%; text=8/255,5%"
        )?;
        for (name, g) in &self.groups {
            let mismatches = g.files.iter().filter(|r| r.actual.is_some_and(|a| a != r.expected)).count();
            let errors = g.files.iter().filter(|r| r.error.is_some()).count();
            let compared = g.files.iter().filter(|r| r.mad.is_some()).count();
            let mean = g.mean_mad();
            let fraction = g.fraction();
            let mut worst: Vec<_> = g.files.iter().collect();
            worst.sort_by(|a, b| b.mad.unwrap_or(f64::INFINITY).total_cmp(&a.mad.unwrap_or(f64::INFINITY)).then(a.file.cmp(&b.file)));
            writeln!(
                f,
                "render {name}: files={} size_mismatches={mismatches} errors={errors} compared={compared} mean_MAD={} fraction_over_16={} within_ceiling={}",
                g.files.len(),
                mean.map_or("n/a".into(), |v| format!("{v:.6}")),
                fraction.map_or("n/a".into(), |v| format!("{:.6}%", v * 100.)),
                g.passed(name == "text")
            )?;
            for r in worst.iter().take(3) {
                writeln!(
                    f,
                    "render worst {name}: {} Visio={:?} ours={:?} MAD={:?} fraction={:?} error={:?}",
                    r.file, r.expected, r.actual, r.mad, r.differing, r.error
                )?;
            }
            for r in &g.files {
                if r.actual.is_some_and(|a| a != r.expected) {
                    writeln!(f, "render size mismatch {} Visio={:?} ours={:?}", r.file, r.expected, r.actual)?;
                }
            }
        }
        Ok(())
    }
}
pub fn compare(a: &image::RgbaImage, b: &image::RgbaImage) -> Result<(f64, f64), String> {
    if a.dimensions() != b.dimensions() || a.width() == 0 || a.height() == 0 {
        return Err("pixel dimensions differ or empty image".into());
    }
    let mut sum = 0u64;
    let mut different = 0u64;
    for (a, b) in a.pixels().zip(b.pixels()) {
        let [ar, ag, ab, aa] = a.0;
        let [br, bg, bb, ba] = b.0;
        let white = |c: u8, alpha: u8| ((u32::from(c) * u32::from(alpha) + 255 * (255 - u32::from(alpha)) + 127) / 255) as u8;
        let d = [white(ar, aa).abs_diff(white(br, ba)), white(ag, aa).abs_diff(white(bg, ba)), white(ab, aa).abs_diff(white(bb, ba))];
        sum += d.iter().map(|d| u64::from(*d)).sum::<u64>();
        different += u64::from(d.iter().any(|d| *d > 16));
    }
    let n = f64::from(a.width()) * f64::from(a.height());
    Ok((sum as f64 / (n * 3.), different as f64 / n))
}
pub fn run(root: &Path) -> Result<Report, String> {
    let mut report = Report::default();
    let mut files: Vec<PathBuf> = super::walk(&root.join("generated"))?.into_iter().filter(|p| p.extension().is_some_and(|s| s == "vsdx")).collect();
    files.sort();
    for file in files {
        let name = file.parent().and_then(|p| p.file_name()).and_then(|s| s.to_str()).ok_or("invalid render group")?.to_string();
        let expected = diagramcraft_render::raster::decode(&std::fs::read(file.with_extension("png")).map_err(|e| e.to_string())?)?;
        let mut r = FileResult {
            file: file.strip_prefix(root).unwrap_or(&file).display().to_string(),
            expected: expected.dimensions(),
            actual: None,
            mad: None,
            differing: None,
            error: None,
        };
        let result = (|| -> Result<(), String> {
            let doc = diagramcraft_vsdx::read(&file).map_err(|e| e.to_string())?;
            let opts = diagramcraft_render::scene::Options::default();
            let scene = diagramcraft_render::scene::page(&doc, diagramcraft_render::scene::default_page_index(&doc)?, opts)?;
            let actual = diagramcraft_render::raster::render(&scene, opts)?;
            let directory = root.join("_state/render").join(&name);
            std::fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
            actual.save(directory.join(file.file_name().ok_or("missing render name")?).with_extension("png")).map_err(|e| e.to_string())?;
            r.actual = Some(actual.dimensions());
            if actual.dimensions() == expected.dimensions() {
                let (mad, fraction) = compare(&expected, &actual)?;
                r.mad = Some(mad);
                r.differing = Some(fraction);
            }
            Ok(())
        })();
        if let Err(e) = result {
            r.error = Some(e);
        }
        report.groups.entry(name).or_default().files.push(r);
    }
    Ok(report)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strict_dimensions_rgb_distance_and_frozen_ceilings() {
        let a = image::RgbaImage::from_pixel(2, 1, image::Rgba([0, 0, 0, 255]));
        let b = image::RgbaImage::from_pixel(2, 1, image::Rgba([17, 0, 0, 255]));
        assert_eq!(compare(&a, &b).unwrap(), (17. / 3., 1.));
        assert!(compare(&a, &image::RgbaImage::new(1, 2)).is_err());
        let group = Group {
            files: vec![FileResult {
                file: "fixture".into(),
                expected: (2, 1),
                actual: Some((2, 1)),
                mad: Some(4.01),
                differing: Some(0.01),
                error: None,
            }],
        };
        assert!(!group.passed(false));
        assert!(group.passed(true));
    }
}
