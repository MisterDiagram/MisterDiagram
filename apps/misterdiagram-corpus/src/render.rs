//! P2 pixel oracle frozen by Charles on 2026-10-12 at 16:45.
//! At most one pixel per dimension is compared over the top-left common area.
use std::{
    collections::BTreeMap,
    fmt,
    path::{Path, PathBuf},
};
const MAX_SIZE_DELTA: u32 = 1;
const NON_TEXT_MAD: f64 = 4.;
const NON_TEXT_FRACTION: f64 = 0.02;
const TEXT_MAD: f64 = 8.;
const TEXT_FRACTION: f64 = 0.05;
// Charles's P3/M3.0 disclosure ratchet. Pixel ceilings and aggregation stay frozen.
const MAX_FILE_OUTLIERS: usize = 10;
fn compatible_size(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 > 0 && a.1 > 0 && b.0 > 0 && b.1 > 0 && a.0.abs_diff(b.0) <= MAX_SIZE_DELTA && a.1.abs_diff(b.1) <= MAX_SIZE_DELTA
}
#[derive(Debug)]
pub struct FileResult {
    pub file: String,
    pub expected: (u32, u32),
    pub actual: Option<(u32, u32)>,
    pub mad: Option<f64>,
    pub differing: Option<f64>,
    pub error: Option<String>,
}
impl FileResult {
    /// Individual pixel exceedance, counted by the P3 disclosure ratchet.
    pub fn over_ceiling(&self, text: bool) -> bool {
        let (mad, fraction) = if text { (TEXT_MAD, TEXT_FRACTION) } else { (NON_TEXT_MAD, NON_TEXT_FRACTION) };
        self.mad.is_some_and(|v| v > mad) || self.differing.is_some_and(|v| v > fraction)
    }
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
                let Some(actual) = file.actual else { continue };
                let n = f64::from(file.expected.0.min(actual.0)) * f64::from(file.expected.1.min(actual.1));
                pixels += n;
                different += n * fraction;
            }
        }
        (pixels > 0.).then(|| different / pixels)
    }
    pub fn passed(&self, text: bool) -> bool {
        let (mad, fraction) = if text { (TEXT_MAD, TEXT_FRACTION) } else { (NON_TEXT_MAD, NON_TEXT_FRACTION) };
        !self.files.is_empty()
            && self
                .files
                .iter()
                .all(|f| f.error.is_none() && f.actual.is_some_and(|a| compatible_size(f.expected, a)) && f.mad.is_some() && f.differing.is_some())
            && self.mean_mad().is_some_and(|v| v <= mad)
            && self.fraction().is_some_and(|v| v <= fraction)
    }
}
impl Report {
    pub fn outlier_count(&self) -> usize {
        self.count_outliers(true)
    }
    pub fn witness_outlier_count(&self) -> usize {
        self.count_outliers(false)
    }
    fn count_outliers(&self, pinned: bool) -> usize {
        self.groups
            .iter()
            .map(|(name, group)| {
                group
                    .files
                    .iter()
                    .filter(|file| {
                        crate::render_population::MEMBERS.lines().any(|member| member == file.file) == pinned && file.over_ceiling(name == "text")
                    })
                    .count()
            })
            .sum()
    }
    pub fn passed(&self) -> bool {
        let Ok(population) = crate::render_population::frozen() else { return false };
        let files: Vec<_> = self.groups.values().flat_map(|g| &g.files).collect();
        population.iter().all(|member| files.iter().filter(|f| f.file == **member).count() == 1)
            && self.outlier_count() <= MAX_FILE_OUTLIERS
            && self.groups.iter().all(|(name, group)| {
                if name == "data" {
                    !group.files.is_empty()
                        && group.files.iter().all(|f| {
                            f.error.is_none() && f.actual.is_some_and(|a| compatible_size(f.expected, a)) && f.mad.is_some() && f.differing.is_some()
                        })
                } else {
                    group.passed(name == "text")
                }
            })
    }
}
impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "render oracle: RGB MAD in 0..255; differing means any RGB channel exceeds 16; frozen 2026-10-12 16:45: non-text ceilings=4/255,2%; text=8/255,5%; size tolerance=1px per axis; common-area comparison; group MAD=unweighted per-file mean, fraction=pixel-weighted common area"
        )?;
        writeln!(
            f,
            "render per-file outliers: count={} maximum={MAX_FILE_OUTLIERS} ratchet={}",
            self.outlier_count(),
            if self.outlier_count() <= MAX_FILE_OUTLIERS { "PASS" } else { "FAIL" }
        )?;
        writeln!(f, "Phase 4 witness outliers: count={} informational=true", self.witness_outlier_count())?;
        for (name, g) in &self.groups {
            let mismatches = g.files.iter().filter(|r| r.actual.is_some_and(|a| a != r.expected)).count();
            let tolerated = g.files.iter().filter(|r| r.actual.is_some_and(|a| a != r.expected && compatible_size(r.expected, a))).count();
            let hard = mismatches - tolerated;
            let errors = g.files.iter().filter(|r| r.error.is_some()).count();
            let compared = g.files.iter().filter(|r| r.mad.is_some()).count();
            let individually_over = g.files.iter().filter(|r| r.over_ceiling(name == "text")).count();
            let mean = g.mean_mad();
            let fraction = g.fraction();
            let mut worst: Vec<_> = g.files.iter().collect();
            worst.sort_by(|a, b| b.mad.unwrap_or(f64::INFINITY).total_cmp(&a.mad.unwrap_or(f64::INFINITY)).then(a.file.cmp(&b.file)));
            writeln!(
                f,
                "render {name}: files={} size_mismatches={mismatches} tolerated_sizes={tolerated} hard_size_failures={hard} errors={errors} compared={compared} individually_over_ceiling={individually_over} mean_MAD={} fraction_over_16={} within_ceiling={} informational={}",
                g.files.len(),
                mean.map_or("n/a".into(), |v| format!("{v:.6}")),
                fraction.map_or("n/a".into(), |v| format!("{:.6}%", v * 100.)),
                g.passed(name == "text"),
                name == "data"
            )?;
            writeln!(f, "render per-file {name}: file | Visio | ours | RGB MAD | pixels over16 | over pixel ceiling | size accepted | error")?;
            for r in &g.files {
                writeln!(
                    f,
                    "render file: {} | {}x{} | {} | {} | {} | {} | {} | {}",
                    r.file,
                    r.expected.0,
                    r.expected.1,
                    r.actual.map_or("n/a".into(), |(w, h)| format!("{w}x{h}")),
                    r.mad.map_or("n/a".into(), |v| format!("{v:.6}")),
                    r.differing.map_or("n/a".into(), |v| format!("{:.6}%", v * 100.)),
                    r.over_ceiling(name == "text"),
                    r.actual.is_some_and(|a| compatible_size(r.expected, a)),
                    r.error.as_deref().unwrap_or("none")
                )?;
            }
            for r in worst.iter().take(3) {
                writeln!(
                    f,
                    "render worst {name}: {} Visio={:?} ours={:?} MAD={:?} fraction={:?} error={:?}",
                    r.file, r.expected, r.actual, r.mad, r.differing, r.error
                )?;
            }
            for r in &g.files {
                if r.actual.is_some_and(|a| a != r.expected) {
                    writeln!(
                        f,
                        "render size mismatch {} Visio={:?} ours={:?} tolerated={}",
                        r.file,
                        r.expected,
                        r.actual,
                        r.actual.is_some_and(|a| compatible_size(r.expected, a))
                    )?;
                }
            }
        }
        Ok(())
    }
}
pub fn compare(a: &image::RgbaImage, b: &image::RgbaImage) -> Result<(f64, f64), String> {
    if !compatible_size(a.dimensions(), b.dimensions()) {
        return Err("pixel dimensions differ by more than one pixel or empty image".into());
    }
    let width = a.width().min(b.width());
    let height = a.height().min(b.height());
    let pixels = u64::from(width) * u64::from(height);
    if pixels > misterdiagram_render::scene::MAX_PIXELS as u64 {
        return Err("pixel comparison limit".into());
    }
    let mut sum = 0u64;
    let mut different = 0u64;
    // Zip rows, then crop each row: flat pixel zipping would shift later rows
    // when the widths differ. No resampling, alignment search or input mutation.
    for (a, b) in a.rows().zip(b.rows()).take(height as usize).flat_map(|(a, b)| a.zip(b).take(width as usize)) {
        let [ar, ag, ab, aa] = a.0;
        let [br, bg, bb, ba] = b.0;
        let white = |c: u8, alpha: u8| ((u32::from(c) * u32::from(alpha) + 255 * (255 - u32::from(alpha)) + 127) / 255) as u8;
        let d = [white(ar, aa).abs_diff(white(br, ba)), white(ag, aa).abs_diff(white(bg, ba)), white(ab, aa).abs_diff(white(bb, ba))];
        sum += d.iter().map(|d| u64::from(*d)).sum::<u64>();
        different += u64::from(d.iter().any(|d| *d > 16));
    }
    let n = pixels as f64;
    Ok((sum as f64 / (n * 3.), different as f64 / n))
}
pub fn run(root: &Path) -> Result<Report, String> {
    let population = crate::render_population::frozen()?;
    for member in &population {
        if !root.join(member).is_file() {
            return Err(format!("missing pinned render input: {member}"));
        }
    }
    let mut report = Report::default();
    let mut files: Vec<PathBuf> = super::walk(&root.join("generated"))?.into_iter().filter(|p| p.extension().is_some_and(|s| s == "vsdx")).collect();
    files.sort();
    for file in files {
        let name = file.parent().and_then(|p| p.file_name()).and_then(|s| s.to_str()).ok_or("invalid render group")?.to_string();
        let expected = misterdiagram_render::raster::decode(&std::fs::read(file.with_extension("png")).map_err(|e| e.to_string())?)?;
        let mut r = FileResult {
            file: file.strip_prefix(root).unwrap_or(&file).display().to_string().replace('\\', "/"),
            expected: expected.dimensions(),
            actual: None,
            mad: None,
            differing: None,
            error: None,
        };
        let result = (|| -> Result<(), String> {
            let doc = misterdiagram_vsdx::read(&file).map_err(|e| e.to_string())?;
            let opts = misterdiagram_render::scene::Options::default();
            let scene = misterdiagram_render::scene::page(&doc, misterdiagram_render::scene::default_page_index(&doc)?, opts)?;
            let actual = misterdiagram_render::raster::render(&scene, opts)?;
            let directory = root.join("_state/render").join(&name);
            std::fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
            actual.save(directory.join(file.file_name().ok_or("missing render name")?).with_extension("png")).map_err(|e| e.to_string())?;
            r.actual = Some(actual.dimensions());
            if compatible_size(expected.dimensions(), actual.dimensions()) {
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
    fn per_file_outlier_ratchet_rejects_eleven_even_when_groups_pass() {
        let files = crate::render_population::MEMBERS
            .lines()
            .enumerate()
            .map(|(i, name)| FileResult {
                file: name.into(),
                expected: (100, 100),
                actual: Some((100, 100)),
                mad: Some(if i < 10 { 5. } else { 0. }),
                differing: Some(0.),
                error: None,
            })
            .collect();
        let mut report = Report { groups: BTreeMap::from([("fixture".into(), Group { files })]) };
        assert_eq!(report.outlier_count(), 10);
        assert!(report.passed());
        report.groups.get_mut("fixture").unwrap().files[10].mad = Some(5.);
        assert!(report.groups["fixture"].passed(false));
        assert_eq!(report.outlier_count(), 11);
        assert!(!report.passed());
        assert!(report.to_string().contains("count=11 maximum=10 ratchet=FAIL"));
        report.groups.get_mut("fixture").unwrap().files[10].mad = Some(0.);
        report.groups.insert(
            "data".into(),
            Group {
                files: vec![FileResult {
                    file: "generated/data/property-rows.vsdx".into(),
                    expected: (529, 145),
                    actual: Some((529, 145)),
                    mad: Some(6.),
                    differing: Some(0.04),
                    error: None,
                }],
            },
        );
        assert!(report.passed());
        assert_eq!(report.witness_outlier_count(), 1);
        assert!(report.to_string().contains("Phase 4 witness outliers: count=1 informational=true"));
        report.groups.insert(
            "route".into(),
            Group {
                files: vec![FileResult {
                    file: "generated/route/offset-left.vsdx".into(),
                    expected: (625, 145),
                    actual: Some((625, 145)),
                    mad: Some(6.),
                    differing: Some(0.04),
                    error: None,
                }],
            },
        );
        assert!(!report.passed(), "route group remains required");
        report.groups.remove("route");
        report.groups.get_mut("fixture").unwrap().files.pop();
        assert!(!report.passed(), "missing pinned members fail");
    }
    #[test]
    fn per_file_disclosure_does_not_change_group_aggregation() {
        let group = Group {
            files: vec![
                FileResult {
                    file: "small outlier".into(),
                    expected: (1, 1),
                    actual: Some((1, 1)),
                    mad: Some(5.),
                    differing: Some(0.03),
                    error: None,
                },
                FileResult {
                    file: "large match".into(),
                    expected: (100, 100),
                    actual: Some((100, 100)),
                    mad: Some(0.),
                    differing: Some(0.),
                    error: None,
                },
            ],
        };
        assert_eq!(group.mean_mad(), Some(2.5));
        assert_eq!(group.fraction(), Some(0.03 / 10001.));
        assert!(group.passed(false), "the frozen group gate remains unchanged");
        assert_eq!(group.files.iter().filter(|f| f.over_ceiling(false)).count(), 1);
        let report = Report { groups: BTreeMap::from([("fixture".into(), group)]) }.to_string();
        assert!(report.contains("individually_over_ceiling=1"));
        assert_eq!(report.lines().filter(|line| line.starts_with("render file:")).count(), 2);
        assert!(report.contains("small outlier | 1x1 | 1x1 | 5.000000 | 3.000000% | true"));
    }
    #[test]
    fn rgb_distance_and_frozen_ceilings() {
        let a = image::RgbaImage::from_pixel(2, 1, image::Rgba([0, 0, 0, 255]));
        let b = image::RgbaImage::from_pixel(2, 1, image::Rgba([17, 0, 0, 255]));
        assert_eq!(compare(&a, &b).unwrap(), (17. / 3., 1.));
        assert!(compare(&a, &image::RgbaImage::new(4, 1)).is_err());
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
    #[test]
    fn one_pixel_common_area_preserves_rows_and_rejects_two_pixels() {
        let a = image::RgbaImage::from_fn(2, 2, |x, y| image::Rgba([(x + y * 2) as u8 * 40, 0, 0, 255]));
        for dimensions in [(3, 2), (2, 3), (3, 3)] {
            let b = image::RgbaImage::from_fn(dimensions.0, dimensions.1, |x, y| {
                if x < 2 && y < 2 { *a.get_pixel(x, y) } else { image::Rgba([255, 255, 255, 255]) }
            });
            assert_eq!(compare(&a, &b).unwrap(), (0., 0.));
            assert_eq!(compare(&b, &a).unwrap(), (0., 0.));
        }
        for dimensions in [(4, 2), (2, 4), (0, 2), (2, 0)] {
            assert!(compare(&a, &image::RgbaImage::new(dimensions.0, dimensions.1)).is_err());
        }
        let mut b = a.clone();
        b.put_pixel(0, 0, image::Rgba([255, 255, 255, 0]));
        assert_eq!(compare(&a, &b).unwrap(), (255. / 4., 0.25));
    }
    #[test]
    fn group_weights_common_pixels_and_keeps_size_failures_mandatory() {
        let mut group = Group {
            files: vec![
                FileResult { file: "cropped".into(), expected: (2, 3), actual: Some((3, 2)), mad: Some(0.), differing: Some(0.01), error: None },
                FileResult { file: "equal".into(), expected: (2, 2), actual: Some((2, 2)), mad: Some(0.), differing: Some(0.), error: None },
            ],
        };
        assert_eq!(group.fraction(), Some(0.005));
        assert!(group.passed(false));
        group.files[0].actual = Some((4, 2));
        assert!(!group.passed(false));
    }
    #[test]
    fn ceilings_match_charles_frozen_record() {
        let record = include_str!("../../../crates/sheet/tests/floors.toml");
        for (key, expected) in [
            ("non_text_mad_rgb", NON_TEXT_MAD),
            ("non_text_fraction", NON_TEXT_FRACTION),
            ("text_mad_rgb", TEXT_MAD),
            ("text_fraction", TEXT_FRACTION),
            ("max_size_delta_px", f64::from(MAX_SIZE_DELTA)),
        ] {
            let values: Vec<f64> = record
                .lines()
                .filter_map(|line| {
                    let (name, value) = line.split_once('=')?;
                    (name.trim() == key).then(|| value.trim().parse().unwrap())
                })
                .collect();
            assert_eq!(values, vec![expected], "{key}");
        }
    }
}
