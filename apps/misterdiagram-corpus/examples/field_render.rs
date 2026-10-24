//! Private field corpus render gate. No sample names or bytes belong in this source.
//! Usage: field_render FOLDER BASELINE.tsv OUTPUT.tsv [IMAGE_DIRECTORY]
//! Use '-' for BASELINE only for the initial measurement; that run still fails on render errors.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use image::RgbImage;
use misterdiagram_render::{raster, scene};
use std::{collections::BTreeMap, fs, io::Write, path::Path};
const HEADER: &str = "stem\tpage\texit\terror\twarnings\tfirst_five_warnings\tour_size\tvisio_size\tdistance\tvisio_open";
#[derive(Debug)]
struct Row {
    stem: String,
    page: usize,
    error: String,
    warnings: Vec<String>,
    warning_count: usize,
    ours: String,
    visio: String,
    distance: Option<f64>,
    opened: bool,
}
fn clean(s: &str) -> String {
    s.replace(['\t', '\r', '\n'], " ")
}
impl Row {
    fn line(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            self.stem,
            self.page,
            u8::from(!self.error.is_empty()),
            clean(&self.error),
            self.warning_count,
            clean(&self.warnings.iter().take(5).cloned().collect::<Vec<_>>().join(" | ")),
            self.ours,
            self.visio,
            self.distance.map(|n| format!("{n:.9}")).unwrap_or_default(),
            self.opened
        )
    }
}
fn baseline(path: &Path) -> Result<BTreeMap<(String, usize), Row>, String> {
    if fs::metadata(path).map_err(|e| e.to_string())?.len() > 16 * 1024 * 1024 {
        return Err("baseline size limit".into());
    }
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let text = std::str::from_utf8(&bytes).map_err(|e| e.to_string())?;
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err("baseline header".into());
    }
    let mut rows = BTreeMap::new();
    for line in lines.filter(|l| !l.starts_with('#')) {
        let fields = line.split('\t').collect::<Vec<_>>();
        let [stem, page, exit, error, warnings, _, ours, visio, distance, opened] = fields.as_slice() else {
            return Err("baseline columns".into());
        };
        if stem.is_empty() || !matches!(*exit, "0" | "1") || (*exit == "0") != error.is_empty() {
            return Err("baseline status".into());
        }
        let distance = if distance.is_empty() {
            None
        } else {
            let n = distance.parse::<f64>().map_err(|e| e.to_string())?;
            if !n.is_finite() || !(0. ..=255.).contains(&n) {
                return Err("baseline distance".into());
            }
            Some(n)
        };
        let row = Row {
            stem: stem.to_string(),
            page: page.parse::<usize>().map_err(|e| e.to_string())?,
            error: error.to_string(),
            warnings: vec![],
            warning_count: warnings.parse::<usize>().map_err(|e| e.to_string())?,
            ours: ours.to_string(),
            visio: visio.to_string(),
            distance,
            opened: opened.parse::<bool>().map_err(|e| e.to_string())?,
        };
        if row.error.is_empty() && !row.visio.is_empty() && row.distance.is_none() {
            return Err("baseline missing distance".into());
        }
        if rows.insert((row.stem.clone(), row.page), row).is_some() {
            return Err("duplicate baseline row".into());
        }
    }
    if rows.is_empty() {
        return Err("empty baseline".into());
    }
    Ok(rows)
}
/// Exact area/box resampling, including fractional edge coverage. RGB channels are
/// rounded only after both axes, not between passes. Original witnesses remain untouched.
fn distance(ours: &RgbImage, witness: &RgbImage) -> Result<f64, String> {
    let (w, h) = ours.dimensions();
    let (sw, sh) = witness.dimensions();
    if w == 0 || h == 0 || sw == 0 || sh == 0 {
        return Err("empty comparison image".into());
    }
    let sx = f64::from(sw) / f64::from(w);
    let sy = f64::from(sh) / f64::from(h);
    let mut sum = 0f64;
    for y in 0..h {
        let y0 = f64::from(y) * sy;
        let y1 = f64::from(y + 1) * sy;
        for x in 0..w {
            let x0 = f64::from(x) * sx;
            let x1 = f64::from(x + 1) * sx;
            let mut rgb = [0f64; 3];
            for iy in y0.floor() as u32..(y1.ceil() as u32).min(sh) {
                let wy = y1.min(f64::from(iy + 1)) - y0.max(f64::from(iy));
                for ix in x0.floor() as u32..(x1.ceil() as u32).min(sw) {
                    let wx = x1.min(f64::from(ix + 1)) - x0.max(f64::from(ix));
                    for (out, c) in rgb.iter_mut().zip(witness.get_pixel(ix, iy).0) {
                        *out += f64::from(c) * wx * wy;
                    }
                }
            }
            for (actual, expected) in ours.get_pixel(x, y).0.into_iter().zip(rgb) {
                sum += (f64::from(actual) - (expected / (sx * sy)).round()).abs();
            }
        }
    }
    Ok(sum / (f64::from(w) * f64::from(h) * 3.))
}
fn named_refusal(error: &str) -> bool {
    let lower = error.to_ascii_lowercase();
    !lower.contains("panic") && (lower.contains("zip") || lower.contains("xml"))
}
// Resolve existing ancestors too, so a not-yet-created leaf cannot bypass protection.
fn resolved(path: &Path) -> Result<std::path::PathBuf, String> {
    let absolute = std::path::absolute(path).map_err(|e| e.to_string())?;
    let mut parent = absolute.as_path();
    let mut suffix = Vec::new();
    for _ in 0..256 {
        if parent.exists() {
            let mut out = parent.canonicalize().map_err(|e| e.to_string())?;
            for component in suffix.iter().rev() {
                out.push(component);
            }
            return Ok(out);
        }
        suffix.push(parent.file_name().ok_or("output path component")?);
        parent = parent.parent().ok_or("output path parent")?;
    }
    Err("output path depth limit".into())
}
fn protect_inputs(root: &Path, output: &Path) -> Result<(), String> {
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let output = resolved(output)?;
    if output.starts_with(&root) || root.starts_with(&output) {
        return Err("outputs must be outside the immutable field input directory".into());
    }
    Ok(())
}
fn run() -> Result<bool, String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if !(3..=4).contains(&args.len()) {
        return Err("usage: field_render FOLDER BASELINE.tsv|- OUTPUT.tsv [IMAGE_DIRECTORY]".into());
    }
    let root = Path::new(args.first().ok_or("folder")?);
    let base = args.get(1).ok_or("baseline")?;
    let before = if base == "-" { None } else { Some(baseline(Path::new(base))?) };
    let output = Path::new(args.get(2).ok_or("output")?);
    if base != "-" && Path::new(base).canonicalize().ok() == output.canonicalize().ok() {
        return Err("output must not overwrite baseline".into());
    }
    let images = args.get(3).map(Path::new);
    let mut paths = fs::read_dir(root.join("files"))
        .map_err(|e| e.to_string())?
        .map(|r| r.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    paths.retain(|p| p.extension().is_some_and(|s| s == "vsdx"));
    paths.sort();
    if paths.len() != 95 {
        return Err(format!("field population: {} files, expected 95", paths.len()));
    }
    protect_inputs(root, output)?;
    if let Some(images) = images {
        protect_inputs(root, images)?;
    }
    let mut output = fs::OpenOptions::new().write(true).create_new(true).open(output).map_err(|e| e.to_string())?;
    writeln!(output, "{HEADER}").map_err(|e| e.to_string())?;
    let mut classes: [Vec<String>; 4] = Default::default();
    let mut keys = std::collections::BTreeSet::new();
    let mut passed = true;
    for path in paths {
        let stem = path.file_stem().and_then(|s| s.to_str()).ok_or("filename")?;
        let witness_dir = root.join("visio").join(stem);
        if !witness_dir.is_dir() {
            return Err(format!("missing witness directory: {stem}"));
        }
        let opened = witness_dir.join("resaved.vsdx").is_file();
        let result = std::panic::catch_unwind(|| misterdiagram_vsdx::read(&path));
        let result = match result {
            Ok(result) => result.map_err(|e| e.to_string()),
            Err(_) => Err("panic reading file".into()),
        };
        let mut rows = Vec::new();
        let new_row = |page| Row {
            stem: stem.into(),
            page,
            error: String::new(),
            warnings: vec![],
            warning_count: 0,
            ours: String::new(),
            visio: String::new(),
            distance: None,
            opened,
        };
        match result {
            Err(error) => {
                let mut row = new_row(0);
                row.error = error;
                rows.push(row);
            }
            Ok(document) => {
                if document.pages.is_empty() {
                    let mut row = new_row(0);
                    row.error = "no pages".into();
                    rows.push(row);
                }
                for index in 0..document.pages.len() {
                    let mut row = new_row(index + 1);
                    let options = scene::Options { cached_fallback: true, ..Default::default() };
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let scene = scene::page(&document, index, options)?;
                        row.warning_count = scene
                            .warnings
                            .iter()
                            .map(|w| {
                                if w.ends_with(" more cells used the value saved in the file") || w.ends_with(" more drawing recovery warnings") {
                                    w.split_whitespace().next().and_then(|n| n.parse::<usize>().ok()).unwrap_or(1)
                                } else {
                                    1
                                }
                            })
                            .sum();
                        row.warnings = scene.warnings.iter().take(5).cloned().collect();
                        raster::render(&scene, options)
                    }));
                    let result = match result {
                        Ok(result) => result,
                        Err(_) => Err("panic rendering page".into()),
                    };
                    let witness = witness_dir.join(format!("page{}.png", index + 1));
                    let witness = if witness.is_file() {
                        let pixels = raster::decode(&fs::read(witness).map_err(|e| e.to_string())?)?;
                        row.visio = format!("{}x{}", pixels.width(), pixels.height());
                        Some(image::DynamicImage::ImageRgba8(pixels).into_rgb8())
                    } else {
                        None
                    };
                    match result {
                        Err(error) => row.error = error,
                        Ok(image) => {
                            row.ours = format!("{}x{}", image.width(), image.height());
                            if let Some(dir) = images {
                                fs::create_dir_all(dir.join(stem)).map_err(|e| e.to_string())?;
                                let png = dir.join(stem).join(format!("page{}.png", index + 1));
                                protect_inputs(root, &png)?;
                                image.save(png).map_err(|e| e.to_string())?;
                            }
                            if let Some(witness) = witness {
                                row.distance = Some(distance(&image::DynamicImage::ImageRgba8(image).into_rgb8(), &witness)?);
                            }
                        }
                    }
                    rows.push(row);
                }
                // A witness page absent from our document is a failure, not a skipped page.
                for entry in fs::read_dir(&witness_dir).map_err(|e| e.to_string())? {
                    let name = entry.map_err(|e| e.to_string())?.file_name();
                    let name = name.to_string_lossy();
                    if let Some(n) = name.strip_prefix("page").and_then(|s| s.strip_suffix(".png")) {
                        let n = n.parse::<usize>().map_err(|e| e.to_string())?;
                        if n == 0 || n > document.pages.len() {
                            return Err(format!("{stem}: missing page {n}"));
                        }
                    }
                }
            }
        }
        let success = rows.iter().all(|r| r.error.is_empty());
        let class = match (opened, success) {
            (true, true) => 0,
            (true, false) => 1,
            (false, false) => 2,
            (false, true) => 3,
        };
        let errors = rows.iter().filter(|r| !r.error.is_empty()).map(|r| format!("page {}: {}", r.page, r.error)).collect::<Vec<_>>().join("; ");
        classes.get_mut(class).ok_or("class")?.push(if errors.is_empty() { stem.to_string() } else { format!("{stem} ({})", clean(&errors)) });
        if opened != success || (!opened && rows.iter().any(|r| !named_refusal(&r.error))) {
            passed = false;
        }
        for row in rows {
            let key = (row.stem.clone(), row.page);
            if let Some(before) = &before {
                let old = before.get(&key).ok_or_else(|| format!("new page not in baseline: {key:?}"))?;
                if old.opened != row.opened || old.visio != row.visio {
                    return Err(format!("witness changed: {key:?}"));
                }
                if let Some(old_distance) = old.distance
                    && row.distance.is_none_or(|n| n > old_distance + 0.05)
                {
                    eprintln!("regression {key:?}: {:?} -> {:?}", old.distance, row.distance);
                    passed = false;
                }
            }
            keys.insert(key);
            output.write_all(row.line().as_bytes()).map_err(|e| e.to_string())?;
        }
        output.flush().map_err(|e| e.to_string())?;
        eprintln!("{stem}: {}", if success { "rendered" } else { &errors });
    }
    if before.as_ref().is_some_and(|b| b.keys().any(|k| !keys.contains(k))) {
        return Err("missing baseline page".into());
    }
    if classes.first().map_or(0, Vec::len) + classes.get(1).map_or(0, Vec::len) != 85 {
        return Err("Visio population differs from 85 opened / 10 refused".into());
    }
    for (label, names) in
        ["Visio opened / rendered", "Visio opened / failed", "Visio refused / refused", "Visio refused / opened"].into_iter().zip(classes)
    {
        let summary = format!("# {label}: {}: {}", names.len(), names.join("; "));
        writeln!(output, "{summary}").map_err(|e| e.to_string())?;
        println!("{summary}");
    }
    Ok(passed)
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(true) => std::process::ExitCode::SUCCESS,
        Ok(false) => std::process::ExitCode::FAILURE,
        Err(error) => {
            eprintln!("field_render: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn output_protection_preserves_existing_baselines_and_witnesses() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug").join(format!("field-gate-test-{}", std::process::id()));
        fs::create_dir_all(root.join("inputs/visio")).unwrap();
        fs::write(root.join("inputs/visio/page1.png"), b"witness").unwrap();
        fs::write(root.join("baseline.tsv"), b"baseline").unwrap();
        assert!(protect_inputs(&root.join("inputs"), &root.join("inputs/visio")).is_err());
        assert!(protect_inputs(&root.join("inputs"), &root.join("inputs/visio/new")).is_err());
        assert!(fs::OpenOptions::new().write(true).create_new(true).open(root.join("baseline.tsv")).is_err());
        assert_eq!(fs::read(root.join("inputs/visio/page1.png")).unwrap(), b"witness");
        assert_eq!(fs::read(root.join("baseline.tsv")).unwrap(), b"baseline");
        for row in ["s\t1\t0\t\t0\t\t1x1\t1x1\tNaN\ttrue", "s\t1\t0\t\t0\t\t1x1\t1x1\t\ttrue", "wrong columns"] {
            fs::write(root.join("malformed.tsv"), format!("{HEADER}\n{row}\n")).unwrap();
            assert!(baseline(&root.join("malformed.tsv")).is_err());
        }
        #[cfg(unix)]
        {
            fs::create_dir_all(root.join("images")).unwrap();
            std::os::unix::fs::symlink(root.join("inputs/visio").canonicalize().unwrap(), root.join("images/sample")).unwrap();
            assert!(protect_inputs(&root.join("inputs"), &root.join("images/sample/page1.png")).is_err());
            assert!(protect_inputs(&root.join("inputs"), &root.join("images/sample/page2.png")).is_err());
        }
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn box_filter_averages_all_channels_and_refusals_name_a_cause() {
        let witness = RgbImage::from_fn(2, 2, |x, y| image::Rgb([((x + y) * 40) as u8, 20, 100]));
        let ours = RgbImage::from_pixel(1, 1, image::Rgb([40, 20, 100]));
        assert_eq!(distance(&ours, &witness).unwrap(), 0.);
        assert!(!named_refusal("#VALUE!"));
        assert!(!named_refusal("i/o error"));
        assert!(!named_refusal("panic: zip"));
        assert!(named_refusal("zip: invalid archive"));
        assert!(named_refusal("XML: invalid element"));
    }
}
