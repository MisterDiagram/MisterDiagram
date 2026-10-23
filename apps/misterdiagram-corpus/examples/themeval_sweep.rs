//! Strict comparison of private theme sweeps with Visio's resolved cell values.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use misterdiagram_sheet::Value;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    path::Path,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let root = std::env::args_os()
        .nth(1)
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../diagramcraft-corpus/field/themeval-2026-10-22"));
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(&root)? {
        let path = entry?.path();
        if path.join("sweep.tsv").is_file() {
            paths.push(path.join("sweep.tsv"));
        }
        if path.join("variants/sweep.tsv").is_file() {
            paths.push(path.join("variants/sweep.tsv"));
        }
    }
    paths.sort();
    if paths.len() != 5 {
        return Err(format!("expected five sweep TSVs, found {}", paths.len()).into());
    }
    let mut failed = false;
    let mut populations = BTreeSet::new();
    for path in paths {
        let (equal, total, population) = compare(&path)?;
        if !populations.insert(population) {
            return Err("duplicate theme sweep population".into());
        }
        println!("{}: rows compared {total}, rows equal {equal}, differences {}", path.display(), total - equal);
        failed |= equal != total;
    }
    if populations != BTreeSet::from([(34, false), (43, false), (45, false), (43, true), (45, true)]) {
        return Err("incorrect five-theme population".into());
    }
    if failed { Err("theme sweep contains unequal rows".into()) } else { Ok(()) }
}

fn cell_name(name: &str) -> Result<String> {
    let Some(rest) = name.strip_prefix("FillGradientStops[") else { return Ok(name.to_owned()) };
    let (row, column) = rest.split_once("].").ok_or("invalid gradient address")?;
    let row: u32 = row.parse()?;
    if row >= 10 {
        return Err("gradient row exceeds ten slots".into());
    }
    let column = match column {
        "0" => "Color",
        "1" => "ColorTrans",
        "2" => "Position",
        _ => return Err("invalid gradient column".into()),
    };
    Ok(if row == 0 { format!("FillGradientStops.GradientStop{column}") } else { format!("FillGradientStops.GradientStop{column}[{}]", row + 1) })
}

fn rgb(text: &str) -> Option<[u8; 3]> {
    if let Some(hex) = text.strip_prefix('#') {
        if hex.len() != 6 || !hex.is_ascii() {
            return None;
        }
        return Some([
            u8::from_str_radix(hex.get(0..2)?, 16).ok()?,
            u8::from_str_radix(hex.get(2..4)?, 16).ok()?,
            u8::from_str_radix(hex.get(4..6)?, 16).ok()?,
        ]);
    }
    let fields = text.strip_prefix("RGB(")?.strip_suffix(')')?.split(',').map(str::trim).collect::<Vec<_>>();
    let [r, g, b] = fields.as_slice() else { return None };
    Some([r.parse().ok()?, g.parse().ok()?, b.parse().ok()?])
}

fn equivalent(expected: &str, numeric: &str, actual: &Value, actual_rgb: Option<[u8; 3]>, color_cell: bool, background: bool) -> bool {
    if matches!(actual, Value::Error(_)) {
        return false;
    }
    if color_cell {
        // TextBkgnd zero is transparent, and its other palette indexes are one-based.
        if background && expected == "0" {
            return actual.numeric().is_ok_and(|n| n == 0.);
        }
        let expected = rgb(expected).or_else(|| {
            let index: u32 = expected.parse().ok()?;
            misterdiagram_sheet::functions::default_color(if background { index.checked_sub(1)? } else { index })
        });
        return expected.is_some() && expected == actual_rgb;
    }
    if let Value::Text(text) = actual {
        return text == expected;
    }
    numeric.parse::<f64>().ok().filter(|n| n.is_finite()).zip(actual.numeric().ok()).is_some_and(|(a, b)| (a - b).abs() <= 1e-6)
}

fn selector(value: Value) -> Result<u32> {
    let number = value.numeric().map_err(|e| e.to_string())?;
    if !number.is_finite() || !(0. ..=f64::from(u32::MAX)).contains(&number) || number.fract() != 0. {
        return Err("invalid integral theme selector".into());
    }
    Ok(number as u32)
}

fn compare(path: &Path) -> Result<(usize, usize, (u32, bool))> {
    if std::fs::metadata(path)?.len() > 64 * 1024 * 1024 {
        return Err("sweep TSV exceeds 64 MiB".into());
    }
    let mut data = String::new();
    std::fs::File::open(path)?.take(64 * 1024 * 1024 + 1).read_to_string(&mut data)?;
    if data.len() > 64 * 1024 * 1024 {
        return Err("sweep TSV exceeds 64 MiB".into());
    }
    let mut lines = data.lines();
    let header_line = lines.next().ok_or("empty TSV")?;
    let variant = parse_header(header_line)?;
    let header = header_line.split('\t').collect::<Vec<_>>();
    let directory = path.parent().ok_or("TSV parent")?;
    let document = misterdiagram_vsdx::read(directory.join(if variant { "variants.vsdx" } else { "sweep.vsdx" }))?;
    let themes = misterdiagram_theme::Themes::from_document(&document).map_err(|e| e.to_string())?;
    // Evaluator::new is strict. Never enable cached fallback or use cached_cell here.
    let mut evaluator = Evaluator::new(&document).map_err(|e| e.to_string())?.with_theme_host(&themes);
    let theme_id =
        evaluator.evaluate(Scope::Page(document.pages.first().ok_or("missing first page")?.id), "ThemeIndex").numeric().map_err(|e| e.to_string())?;
    if ![34., 43., 45.].contains(&theme_id) {
        return Err("unexpected sweep theme".into());
    }
    let mut keys = BTreeSet::new();
    let mut cohorts = BTreeMap::new();
    let mut identities = BTreeMap::new();
    let mut page_rows: BTreeMap<u32, BTreeSet<String>> = BTreeMap::new();
    let mut shape_rows: BTreeMap<(u32, u32), BTreeSet<String>> = BTreeMap::new();
    let shape_inventory = expected_cells(variant);
    let mut total = 0;
    let mut equal = 0;
    for line in lines {
        if line.len() > 65536 || total >= 1_000_000 {
            return Err("sweep row limit".into());
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != header.len() {
            return Err(format!("invalid row {}", total + 2).into());
        }
        let row = header.iter().copied().zip(fields).collect::<BTreeMap<_, _>>();
        let get = |key: &str| row.get(key).copied().ok_or_else(|| format!("missing column {key}"));
        let doc_directory = if variant { directory.parent().ok_or("document directory")? } else { directory };
        if doc_directory.file_name().and_then(|n| n.to_str()) != Some(get("doc")?) {
            return Err("TSV document identity mismatch".into());
        }
        let kind = get("kind")?;
        if variant {
            for key in ["vc", "vs"] {
                if get(key)?.parse::<u32>()? > 3 {
                    return Err("variant index outside sweep".into());
                }
            }
        }
        if matches!(kind, "rect" | "conn") {
            let matrices: &[u32] = if variant { &[1, 3, 5, 100, 101, 102] } else { &[1, 2, 3, 4, 5, 6, 100, 101, 102, 103, 104, 105, 106] };
            let colours: &[u32] = if variant {
                &[0, 100, 101, 102, 200, 201, 202, 206]
            } else {
                &[0, 1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 105, 106, 107, 200, 201, 202, 203, 204, 205, 206, 207]
            };
            if !matrices.contains(&get("matrix")?.parse()?) || !colours.contains(&get("colour")?.parse()?) {
                return Err("coordinate outside sweep".into());
            }
        } else if kind != (if variant { "pagesheet" } else { "page1sheet" })
            || get("id")? != "0"
            || !get("matrix")?.is_empty()
            || !get("colour")?.is_empty()
        {
            return Err("invalid page selector row".into());
        }
        let page_name = if variant { format!("var-c{}-s{}", get("vc")?, get("vs")?) } else { format!("sweep-{kind}") };
        let page = if kind == "page1sheet" { document.pages.first() } else { document.pages.iter().find(|p| p.name_u == page_name) }
            .ok_or_else(|| format!("missing page {page_name}"))?;
        let scope = match kind {
            "page1sheet" | "pagesheet" => Scope::Page(page.id),
            "rect" | "conn" => Scope::PageShape { page: page.id, shape: get("id")?.parse()? },
            _ => return Err(format!("unknown kind {kind}").into()),
        };
        let name = cell_name(get("cell")?)?;
        if matches!(name.as_str(), "QuickStyleFillMatrix" | "QuickStyleFillColor") {
            let coordinate = get(if name == "QuickStyleFillMatrix" { "matrix" } else { "colour" })?.parse::<u32>()?;
            if get("resultiu")?.parse::<f64>()? != f64::from(coordinate) {
                return Err("selector row does not match cohort coordinate".into());
            }
        }
        if !keys.insert((page.id, kind.to_owned(), get("id")?.to_owned(), name.clone())) {
            return Err(format!("duplicate row {}", total + 2).into());
        }
        if let Scope::PageShape { page, shape } = scope {
            if !shape_inventory.contains(&name) {
                return Err("incorrect shape cell inventory".into());
            }
            let coordinate = (page, kind.to_owned(), get("matrix")?.parse::<u32>()?, get("colour")?.parse::<u32>()?);
            if cohorts.insert(coordinate.clone(), shape).is_some_and(|old| old != shape)
                || identities.insert((page, shape), coordinate.clone()).is_some_and(|old| old != coordinate)
                || (evaluator.cell(scope, "BeginX").is_some() != (kind == "conn"))
            {
                return Err("invalid shape cohort identity".into());
            }
            if evaluator.shape(scope).is_none() {
                return Err("missing swept shape".into());
            }
            shape_rows.entry((page, shape)).or_default().insert(name.clone());
        } else {
            page_rows.entry(page.id).or_default().insert(name.clone());
        }
        let actual = evaluator.evaluate(scope, &name);
        let color_cell = Evaluator::baseline_metadata(scope, &name).is_some_and(|m| m.3 == 251);
        let actual_rgb = match &actual {
            Value::Color { rgb, .. } => Some(*rgb),
            _ if color_cell => actual.numeric().ok().and_then(|n| {
                let cell = evaluator.cell(scope, &name)?;
                evaluator.serialized_number(scope, &name, cell, n).ok().and_then(|s| rgb(&s))
            }),
            _ => None,
        };
        total += 1;
        let expected = get("resultstr")?;
        let font_pair = if name == "Char.Font" {
            let expected = expected_font(
                &document,
                get("matrix")?.parse()?,
                if variant {
                    get("vs")?.parse()?
                } else {
                    usize::try_from(selector(evaluator.evaluate(Scope::Page(page.id), "VariationStyleIndex"))?)?
                },
                kind == "conn",
                selector(evaluator.evaluate(Scope::Page(page.id), "FontSchemeIndex"))?,
            )?;
            let ours = selector(actual.clone()).ok().and_then(|id| evaluator.font_name(id)).map(str::to_owned).unwrap_or_else(|| actual.text());
            Some((expected, ours))
        } else {
            None
        };
        let expected_rgb = if color_cell && !(name == "TextBkgnd" && expected == "0") {
            expected.parse::<u32>().ok().and_then(|index| {
                let index = if name == "TextBkgnd" { index.checked_sub(1)? } else { index };
                document.colors.get(&index).and_then(|s| rgb(s))
            })
        } else {
            None
        };
        let expected_color = expected_rgb.map(|[r, g, b]| format!("RGB({r},{g},{b})"));
        let result_iu = get("resultiu")?;
        let matches = font_pair.as_ref().map_or_else(
            || equivalent(expected_color.as_deref().unwrap_or(expected), result_iu, &actual, actual_rgb, color_cell, name == "TextBkgnd"),
            |(expected, ours)| expected == ours,
        );
        if matches {
            equal += 1;
        } else {
            let ours = actual_rgb.map(|[r, g, b]| format!("RGB({r}, {g}, {b})")).unwrap_or_else(|| actual.text());
            let font_diagnostic = font_pair.map(|(expected, ours)| format!("\tVisio-face={expected}\tours-face={ours}")).unwrap_or_default();
            println!(
                "{}\tpage={}\tkind={kind}\tmatrix={}\tcolour={}\tid={}\t{}\tformula={}\tVisio={}\tours={ours}{font_diagnostic}",
                path.display(),
                page.name_u,
                get("matrix")?,
                get("colour")?,
                get("id")?,
                get("cell")?,
                get("formula")?,
                get("resultstr")?
            );
        }
    }
    let expected_total = if variant { 72288 } else { 39943 };
    let expected_shapes = if variant { 1536 } else { 624 };
    let cell_count = if variant { 47 } else { 64 };
    if total != expected_total
        || cohorts.len() != expected_shapes
        || shape_rows.len() != expected_shapes
        || shape_rows.values().any(|rows| rows.len() != cell_count)
    {
        return Err(format!("incomplete sweep: {total}/{expected_total} rows, {}/{expected_shapes} shapes", shape_rows.len()).into());
    }
    let expected_shape = expected_cells(variant);
    if shape_rows.values().any(|cells| cells != &expected_shape) {
        return Err("incorrect shape cell inventory".into());
    }
    let mut expected_page = BTreeSet::from(
        ["ThemeIndex", "VariationColorIndex", "VariationStyleIndex", "ColorSchemeIndex", "ConnectorSchemeIndex", "FontSchemeIndex"]
            .map(str::to_owned),
    );
    if !variant {
        expected_page.insert("EffectSchemeIndex".into());
    }
    if page_rows.len() != (if variant { 16 } else { 1 }) || page_rows.values().any(|cells| cells != &expected_page) {
        return Err("incorrect page cell inventory".into());
    }
    Ok((equal, total, (theme_id as u32, variant)))
}

/// Font IDs belong to the capturing installation. Compare the requested face instead.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.font.id
fn expected_font(document: &misterdiagram_model::Document, matrix: usize, variant: usize, connector: bool, scheme: u32) -> Result<String> {
    use misterdiagram_model::XmlElement;
    fn child<'a>(e: &'a XmlElement, name: &str) -> Option<&'a XmlElement> {
        e.elements().find(|e| e.name.as_str() == name)
    }
    fn find<'a>(e: &'a XmlElement, name: &str, depth: usize) -> Option<&'a XmlElement> {
        if depth > 128 {
            return None;
        }
        if e.name.as_str() == name {
            return Some(e);
        }
        e.elements().find_map(|e| find(e, name, depth + 1))
    }
    let (root, font) = document
        .parts
        .iter()
        .filter(|(n, _)| n.starts_with("visio/theme/"))
        .find_map(|(_, part)| {
            let root = part.xml.as_deref()?;
            let font = find(root, "fontScheme", 0)?;
            (find(font, "schemeID", 0)?.attr("schemeEnum")?.parse::<u32>().ok()? == scheme).then_some((root, font))
        })
        .ok_or("missing oracle font scheme")?;
    let selected = if (100..=103).contains(&matrix) {
        find(root, "variationStyleSchemeLst", 0)
            .and_then(|e| e.elements().nth(variant))
            .and_then(|e| e.elements().nth(matrix - 100))
            .and_then(|e| e.attr("fontIdx"))
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or("missing oracle font matrix")?
    } else if (104..=106).contains(&matrix) {
        1
    } else {
        matrix
    };
    let props =
        find(root, if connector { "connectorFontStyles" } else { "fontStyles" }, 0).and_then(|e| e.elements().nth(selected.saturating_sub(1)));
    let explicit = props.and_then(|e| find(e, "latin", 0)).and_then(|e| e.attr("typeface"));
    let face = match explicit {
        Some("+mj-lt") => child(font, "majorFont").and_then(|e| child(e, "latin")).and_then(|e| e.attr("typeface")),
        Some("+mn-lt") | None => child(font, "minorFont").and_then(|e| child(e, "latin")).and_then(|e| e.attr("typeface")),
        face => face,
    };
    face.filter(|s| !s.is_empty()).map(str::to_owned).ok_or_else(|| "missing oracle Latin face".into())
}

fn parse_header(header: &str) -> Result<bool> {
    match header {
        "doc\tkind\tmatrix\tcolour\tid\tcell\tformula\tresultstr\tresultiu" => Ok(false),
        "doc\tvc\tvs\tkind\tmatrix\tcolour\tid\tcell\tformula\tresultstr\tresultiu" => Ok(true),
        _ => Err("invalid sweep TSV header".into()),
    }
}

fn expected_cells(variant: bool) -> BTreeSet<String> {
    let mut cells = BTreeSet::from(
        [
            "LineWeight",
            "LineColor",
            "LinePattern",
            "FillForegnd",
            "FillBkgnd",
            "FillPattern",
            "TextBkgnd",
            "FillGradientAngle",
            "FillGradientEnabled",
            "QuickStyleFillColor",
            "QuickStyleFillMatrix",
            "ThemeIndex",
            "VariationColorIndex",
            "VariationStyleIndex",
            "Char.Font",
            "Char.Color",
            "Char.Size",
        ]
        .map(str::to_owned),
    );
    if !variant {
        cells.extend(
            [
                "LineColorTrans",
                "FillForegndTrans",
                "FillBkgndTrans",
                "TextBkgndTrans",
                "FillGradientDir",
                "LineGradientEnabled",
                "QuickStyleLineColor",
                "QuickStyleFontColor",
                "QuickStyleLineMatrix",
                "QuickStyleFontMatrix",
                "QuickStyleType",
                "QuickStyleVariation",
                "ColorSchemeIndex",
                "EffectSchemeIndex",
                "ConnectorSchemeIndex",
                "FontSchemeIndex",
                "Char.Style",
            ]
            .map(str::to_owned),
        );
    }
    for row in 0..10 {
        for column in ["Color", "ColorTrans", "Position"] {
            cells.insert(if row == 0 {
                format!("FillGradientStops.GradientStop{column}")
            } else {
                format!("FillGradientStops.GradientStop{column}[{}]", row + 1)
            });
        }
    }
    cells
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn font_names_follow_style_and_variant_without_host_ids() {
        use misterdiagram_model::{Document, Part, XmlElement, XmlNode};
        fn element(name: &str, attrs: &[(&str, &str)], children: Vec<XmlElement>) -> XmlElement {
            XmlElement {
                name: name.into(),
                attributes: attrs.iter().map(|(k, v)| ((*k).into(), (*v).into())).collect(),
                children: children.into_iter().map(XmlNode::Element).collect(),
                ..Default::default()
            }
        }
        let latin = |face: &str| element("latin", &[("typeface", face)], vec![]);
        let root = element(
            "theme",
            &[],
            vec![
                element(
                    "fontScheme",
                    &[],
                    vec![
                        element("schemeID", &[("schemeEnum", "19")], vec![]),
                        element("majorFont", &[], vec![latin("Authored Major")]),
                        element("minorFont", &[], vec![latin("Authored Minor")]),
                    ],
                ),
                element(
                    "fontStyles",
                    &[],
                    vec![element("fontProps", &[], vec![latin("+mj-lt")]), element("fontProps", &[], vec![latin("Authored Explicit")])],
                ),
                element("connectorFontStyles", &[], vec![element("fontProps", &[], vec![latin("+mn-lt")])]),
                element(
                    "variationStyleSchemeLst",
                    &[],
                    vec![element("variationStyleScheme", &[], vec![element("variationStyle", &[("fontIdx", "2")], vec![])])],
                ),
            ],
        );
        let mut doc = Document::default();
        doc.parts.insert("visio/theme/theme1.xml".into(), Part { xml: Some(root.into()), ..Default::default() });
        assert_eq!(expected_font(&doc, 1, 0, false, 19).unwrap(), "Authored Major");
        assert_eq!(expected_font(&doc, 100, 0, false, 19).unwrap(), "Authored Explicit");
        assert_eq!(expected_font(&doc, 1, 0, true, 19).unwrap(), "Authored Minor");
        assert_eq!(expected_font(&doc, 6, 0, true, 19).unwrap(), "Authored Minor");
        assert!(expected_font(&doc, 100, 3, false, 19).is_err());
        assert!(expected_font(&doc, 1, 0, false, 20).is_err());
    }
    #[test]
    fn hostile_rows_and_exact_comparison() {
        for number in [f64::NAN, f64::INFINITY, -1., 0.5, f64::from(u32::MAX) + 1.] {
            assert!(selector(Value::Number(number)).is_err());
        }
        assert!(!parse_header("doc\tkind\tmatrix\tcolour\tid\tcell\tformula\tresultstr\tresultiu").unwrap());
        for header in ["", "doc\tdoc", "doc\tkind\tcell\tresultstr\tresultiu", "doc\tvc\tvs", &"\t".repeat(65537)] {
            assert!(parse_header(header).is_err());
        }
        assert_eq!(expected_cells(false).len(), 64);
        assert_eq!(expected_cells(true).len(), 47);
        assert_eq!(cell_name("FillGradientStops[0].0").unwrap(), "FillGradientStops.GradientStopColor");
        for bad in ["FillGradientStops[10].0", "FillGradientStops[0].3", "FillGradientStops[4294967295].0", "FillGradientStops[no].0"] {
            assert!(cell_name(bad).is_err());
        }
        assert_eq!(rgb("RGB(3, 5, 7)"), Some([3, 5, 7]));
        for bad in ["RGB(256,0,0)", "RGB(1,2)", "#日本語", "#ffffff00"] {
            assert!(rgb(bad).is_none());
        }
        assert!(equivalent("2", "2", &Value::Number(2.0000005), None, false, false));
        assert!(!equivalent("2", "2", &Value::Number(2.000002), None, false, false));
        assert!(!equivalent("#REF!", "0", &Value::Error(misterdiagram_sheet::Error::Ref), None, false, false));
        assert!(!equivalent("word", "0", &Value::Text("Word".into()), None, false, false));
        assert!(equivalent("RGB(3,5,7)", "999", &Value::Number(24.), Some([3, 5, 7]), true, false));
    }
    #[test]
    fn parser_rejects_incomplete_duplicate_substituted_and_mislabeled_rows() {
        use misterdiagram_model::{Cell, skeleton};
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../target/debug/themeval-parser-test-{}", std::process::id()));
        let directory = root.join("authored");
        std::fs::create_dir_all(&directory).unwrap();
        let mut doc = skeleton::document(false);
        doc.pages.clear();
        let mut page = skeleton::page(0, "sweep-rect", false);
        page.sheet.cells.retain(|c| c.name != "ThemeIndex");
        page.sheet.cells.push(Cell { name: "ThemeIndex".into(), value: Some("34".into()), ..Default::default() });
        page.shapes.push(skeleton::rectangle(1, 1., 1., 2., 1., "Original test").unwrap());
        doc.pages.push(page);
        let package = misterdiagram_vsdx::write(&doc).unwrap();
        std::fs::write(directory.join("sweep.vsdx"), package.bytes).unwrap();
        let path = directory.join("sweep.tsv");
        let header = "doc\tkind\tmatrix\tcolour\tid\tcell\tformula\tresultstr\tresultiu\n";
        let row = "authored\trect\t1\t0\t1\tLinePattern\t1\t1\t1\n";
        for (body, message) in [
            (row.to_string(), "incomplete sweep"),
            (row.repeat(2), "duplicate row"),
            (row.replace("LinePattern", "Width"), "incorrect shape cell inventory"),
            (row.replace("LinePattern", "QuickStyleFillMatrix").replace("rect\t1", "rect\t3"), "selector row does not match cohort coordinate"),
            (row.replace("rect\t1\t0\t1", "page1sheet\t\t\t1"), "invalid page selector row"),
        ] {
            std::fs::write(&path, format!("{header}{body}")).unwrap();
            let error = compare(&path).unwrap_err().to_string();
            assert!(error.contains(message), "{error} must contain {message}");
        }
        std::fs::remove_dir_all(root).unwrap();
    }
}
