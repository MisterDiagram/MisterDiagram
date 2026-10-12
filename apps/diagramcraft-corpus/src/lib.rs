//! Read-only comparisons against original package caches and completed COM dumps.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
pub mod render;
use diagramcraft_sheet::Value;
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    fs::{self, File},
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};
const MAX_FILES: usize = 100_000;
const MAX_DEPTH: usize = 32;
const MAX_PACKAGE: u64 = 256 * 1024 * 1024;
const MAX_TSV: u64 = 512 * 1024 * 1024;
const MAX_LINE: usize = 1024 * 1024;
const MAX_ROWS: u64 = 10_000_000;
const TOLERANCE: f64 = 1e-6;
const HEADER: &str = "shape_id\tshape_name\tsection\trow\tcell\tname\tformula\tresult_iu\tunits";
#[derive(Debug, Default, Clone)]
pub struct Rate {
    pub matched: u64,
    pub total: u64,
    pub excluded: u64,
}
impl Rate {
    pub fn percent(&self) -> f64 {
        if self.total == 0 { 0. } else { 100. * self.matched as f64 / self.total as f64 }
    }
    fn record(&mut self, matched: bool) {
        self.total += 1;
        if matched {
            self.matched += 1;
        }
    }
}
#[derive(Debug, Clone)]
pub struct Witness {
    pub oracle: &'static str,
    pub category: String,
    pub file: String,
    pub scope: String,
    pub cell: String,
    pub formula: String,
    pub expected: String,
    pub actual: String,
}
#[derive(Debug)]
pub struct Report {
    pub render: Option<render::Report>,
    pub text_metric_floor: f64,
    pub text_promotion_sticky: bool,
    pub text_cached_floor: f64,
    pub text_com_floor: f64,
    pub text_metrics_included: bool,
    pub cached_hosts: [Rate; 2],
    pub com_hosts: [Rate; 2],
    pub cached_classes: [Rate; 3],
    pub com_classes: [Rate; 3],
    pub cached: Rate,
    pub com: Rate,
    pub shipped_cached: Rate,
    pub shipped_com: Rate,
    pub packages: usize,
    pub shipped_packages: usize,
    pub shipped_available: usize,
    pub dump_files: usize,
    pub shipped_dump_files: usize,
    pub cached_floor: f64,
    pub com_floor: f64,
    pub quick: bool,
    pub top_failures: Vec<(String, u64)>,
    pub text_cell_failures: Vec<(String, u64)>,
    pub text_delta_failures: Vec<(String, u64)>,
    pub function_failures: Vec<(String, u64)>,
    pub cached_cell_failures: Vec<(String, u64)>,
    pub cached_actual_failures: Vec<(String, u64)>,
    pub cached_formula_failures: Vec<(String, u64)>,
    pub witnesses: Vec<Witness>,
}
impl Report {
    pub fn passed(&self) -> bool {
        self.render.as_ref().is_none_or(render::Report::passed) && self.cells_passed()
    }
    /// Cell accuracy is reported separately; the complete gate also requires rendering.
    pub fn cells_passed(&self) -> bool {
        (!self.text_promotion_sticky || self.text_gate_passed())
            && self.shipped_cached.total > 0
            && self.com.total > 0
            && self.shipped_cached.percent() >= self.cached_floor
            && self.com.percent() >= self.com_floor
    }
}
impl Report {
    fn text_gate_passed(&self) -> bool {
        self.cached_hosts[1].total > 0
            && self.com_hosts[1].total > 0
            && self.cached_hosts[1].percent() >= self.text_metric_floor.max(self.text_cached_floor)
            && self.com_hosts[1].percent() >= self.text_metric_floor.max(self.text_com_floor)
    }
    fn include_text(&self) -> bool {
        self.text_promotion_sticky || text_promoted(&self.cached_hosts[1], &self.com_hosts[1], self.text_metric_floor)
    }
}
impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(render) = &self.render {
            writeln!(f, "{render}")?;
        }
        writeln!(
            f,
            "corpus mode={} packages={} shipped={}/{} completed_shipped_dumps={} dump_files={}",
            if self.quick { "quick" } else { "full" },
            self.packages,
            self.shipped_packages,
            self.shipped_available,
            self.shipped_dump_files,
            self.dump_files
        )?;
        for (name, rate, floor) in [
            ("cached overall informational", &self.cached, None),
            ("COM", &self.com, Some(self.com_floor)),
            ("shipped cached", &self.shipped_cached, Some(self.cached_floor)),
            ("shipped COM", &self.shipped_com, None),
        ] {
            writeln!(
                f,
                "{name}: {}/{} ({:.6}%) excluded={} floor={}",
                rate.matched,
                rate.total,
                rate.percent(),
                rate.excluded,
                floor.map(|n| format!("{n:.6}%")).unwrap_or_else(|| "reported separately".into())
            )?;
        }
        for (oracle, classes) in [("cached", &self.cached_classes), ("COM", &self.com_classes)] {
            for (class, rate) in ["literal-constant", "empty-formula", "non-literal"].into_iter().zip(classes) {
                writeln!(f, "{oracle} {class}: {}/{} ({:.6}%)", rate.matched, rate.total, rate.percent())?;
            }
        }
        for (oracle, hosts) in [("cached", &self.cached_hosts), ("COM", &self.com_hosts)] {
            for (kind, rate) in ["theme", "text-metric"].into_iter().zip(hosts) {
                writeln!(f, "{oracle} {kind}: {}/{} ({:.6}%)", rate.matched, rate.total, rate.percent())?;
            }
        }
        writeln!(
            f,
            "text-metric promotion: threshold={:.6}% main_denominators={} (both cached and COM must qualify)",
            self.text_metric_floor,
            if self.text_metrics_included { "included" } else { "deferred; M2.3 comparison mode" }
        )?;
        writeln!(
            f,
            "text-metric sticky={} cached_floor={:.6}% COM_floor={:.6}% text_gate={}",
            self.text_promotion_sticky,
            self.text_cached_floor,
            self.text_com_floor,
            if self.text_gate_passed() { "PASS" } else { "FAIL" }
        )?;
        writeln!(f, "diagnostics below use fully evaluated text, including deferred text rows and their dependents")?;
        writeln!(
            f,
            "result={}; tolerance={TOLERANCE}; snapshots isolate SETF writes; string COM values use numeric coercion",
            if self.passed() { "PASS" } else { "FAIL" }
        )?;
        for (kind, entries) in [
            ("cell", &self.top_failures),
            ("text-metric cell", &self.text_cell_failures),
            ("text-metric delta (ours minus Visio, IU)", &self.text_delta_failures),
            ("function", &self.function_failures),
            ("cached source cell", &self.cached_cell_failures),
            ("cached actual", &self.cached_actual_failures),
            ("cached formula kind", &self.cached_formula_failures),
        ] {
            for (name, count) in entries {
                writeln!(f, "failure {kind} {name}: {count}")?;
            }
        }
        for w in &self.witnesses {
            writeln!(
                f,
                "witness {} category={:?} {} {} {} formula={:?} expected={:?} actual={:?}",
                w.oracle, w.category, w.file, w.scope, w.cell, w.formula, w.expected, w.actual
            )?;
        }
        Ok(())
    }
}
fn record_hosts(rates: &mut [Rate; 2], calls: &BTreeSet<String>, matched: bool) {
    let [theme, text] = rates;
    if calls
        .iter()
        .any(|n| matches!(n.as_str(), "THEMEVAL" | "THEME" | "THEMEGUARD" | "THEMERESTORE" | "THEMECBV" | "THEMEPROP" | "ISTHEMED" | "CELLISTHEMED"))
    {
        theme.record(matched);
    }
    if calls.iter().any(|n| diagramcraft_sheet::is_text_metric(n)) {
        text.record(matched);
    }
}
fn walk(root: &Path) -> Result<Vec<PathBuf>, String> {
    if !root.exists() {
        return Err(format!("missing corpus directory {}", root.display()));
    }
    if fs::symlink_metadata(root).map_err(|e| e.to_string())?.file_type().is_symlink() {
        return Err("corpus root symlink rejected".into());
    }
    let mut paths = vec![];
    let mut pending = vec![(root.to_path_buf(), 0)];
    let mut visited = 0;
    while let Some((dir, depth)) = pending.pop() {
        if depth > MAX_DEPTH {
            return Err("corpus directory depth limit".into());
        }
        for entry in fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))? {
            let entry = entry.map_err(|e| e.to_string())?;
            visited += 1;
            if visited > MAX_FILES {
                return Err("corpus file count limit".into());
            }
            let kind = entry.file_type().map_err(|e| e.to_string())?;
            if kind.is_symlink() {
                return Err(format!("corpus symlink rejected: {}", entry.path().display()));
            }
            if kind.is_dir() {
                pending.push((entry.path(), depth + 1));
            } else if kind.is_file() {
                paths.push(entry.path());
            }
        }
    }
    paths.sort();
    Ok(paths)
}
fn package(p: &Path) -> bool {
    p.extension().and_then(|s| s.to_str()).is_some_and(|s| matches!(s.to_ascii_lowercase().as_str(), "vsdx" | "vssx" | "vstx" | "vsdm"))
}
fn read_package(path: &Path) -> Result<diagramcraft_model::Document, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() > MAX_PACKAGE {
        return Err("package size limit".into());
    }
    let mut bytes = vec![];
    file.take(MAX_PACKAGE + 1).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_PACKAGE {
        return Err("package size limit".into());
    }
    diagramcraft_vsdx::import(&bytes).map_err(|e| format!("{}: {e}", path.display()))
}
/// Scan calls independently of parser success, respecting strings, references, and numeric prime units.
pub fn function_calls(formula: &str) -> Result<BTreeSet<String>, String> {
    if formula.len() > MAX_LINE {
        return Err("formula length limit".into());
    }
    let bytes = formula.as_bytes();
    let mut i = 0;
    let mut names = BTreeSet::new();
    let mut quoted = false;
    let mut reference_quoted = false;
    let mut brackets = 0usize;
    let mut number_unit_pending = false;
    while let Some(&b) = bytes.get(i) {
        if quoted || reference_quoted {
            let delimiter = if quoted { b'"' } else { b'\'' };
            if b == delimiter {
                if bytes.get(i + 1) == Some(&delimiter) {
                    i += 2;
                    continue;
                }
                quoted = false;
                reference_quoted = false;
            }
            i += 1;
            continue;
        }
        if brackets > 0 {
            if b == b'[' {
                brackets += 1;
            }
            if b == b']' {
                brackets -= 1;
            }
            i += 1;
            continue;
        }
        if let Some(ch) = formula.get(i..).and_then(|tail| tail.chars().next()).filter(|ch| ch.is_whitespace()) {
            i += ch.len_utf8();
            continue;
        }
        if matches!(b, b'\'' | b'"') && brackets == 0 && number_unit_pending {
            number_unit_pending = false;
            i += 1;
            continue;
        }
        if b == b'"' {
            number_unit_pending = false;
            quoted = true;
            i += 1;
            continue;
        }
        if b == b'\'' {
            number_unit_pending = false;
            reference_quoted = true;
            i += 1;
            continue;
        }
        if b == b'[' {
            number_unit_pending = false;
            brackets += 1;
            i += 1;
            continue;
        }
        let number_start = b.is_ascii_digit() || b == b'.' && bytes.get(i + 1).is_some_and(u8::is_ascii_digit);
        let boundary = formula
            .get(..i)
            .and_then(|head| head.chars().next_back())
            .is_none_or(|c| c.is_whitespace() || matches!(c, '+' | '-' | '*' | '/' | '^' | '&' | '<' | '>' | '=' | '(' | ',' | ';'));
        if number_start && boundary {
            while bytes.get(i).is_some_and(u8::is_ascii_digit) {
                i += 1;
            }
            if bytes.get(i) == Some(&b'.') {
                i += 1;
                while bytes.get(i).is_some_and(u8::is_ascii_digit) {
                    i += 1;
                }
            }
            if matches!(bytes.get(i), Some(b'e' | b'E')) {
                let exponent = i;
                i += 1;
                if matches!(bytes.get(i), Some(b'+' | b'-')) {
                    i += 1;
                }
                let digits = i;
                while bytes.get(i).is_some_and(u8::is_ascii_digit) {
                    i += 1;
                }
                if digits == i {
                    i = exponent;
                }
            }
            number_unit_pending = true;
            continue;
        }
        number_unit_pending = false;
        if b.is_ascii_alphabetic() || b == b'_' {
            let start = i;
            i += 1;
            while bytes.get(i).is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_') {
                i += 1;
            }
            let end = i;
            while bytes.get(i).is_some_and(u8::is_ascii_whitespace) {
                i += 1;
            }
            if bytes.get(i) == Some(&b'(') {
                names.insert(formula.get(start..end).ok_or("invalid function boundary")?.to_ascii_uppercase());
            }
        } else {
            i += 1;
        }
    }
    Ok(names)
}
fn exclusions() -> BTreeSet<String> {
    [include_str!("../../../crates/sheet/tests/volatile.txt"), include_str!("../../../crates/sheet/tests/hooks.txt")]
        .into_iter()
        .flat_map(str::lines)
        .filter_map(|line| line.split('#').next())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect()
}
fn excluded(calls: &BTreeSet<String>, exclusions: &BTreeSet<String>) -> bool {
    calls.iter().any(|s| exclusions.contains(s) || exclusions.contains("_UCON_...") && s.starts_with("_UCON_"))
}
fn promoted(text: &str) -> Result<bool, String> {
    let mut value = None;
    for line in text.lines() {
        if let Some((key, raw)) = line.split('#').next().unwrap_or("").split_once('=')
            && key.trim() == "promoted"
        {
            if value.is_some() {
                return Err("duplicate promoted flag".into());
            }
            value = Some(raw.trim().parse::<bool>().map_err(|_| "invalid promoted flag")?);
        }
    }
    value.ok_or_else(|| "missing promoted flag".into())
}
fn floor(text: &str, key: &str) -> Result<f64, String> {
    let mut found = None;
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if let Some((k, v)) = line.split_once('=')
            && k.trim() == key
        {
            if found.is_some() {
                return Err(format!("duplicate floor {key}"));
            }
            let n = v.trim().parse::<f64>().map_err(|e| e.to_string())?;
            if !n.is_finite() || !(0.0..=1.0).contains(&n) {
                return Err(format!("invalid floor {key}"));
            }
            found = Some(n);
        }
    }
    found.ok_or_else(|| format!("missing floor {key}"))
}
fn equal(expected: &Value, actual: &Value) -> bool {
    match (expected, actual) {
        (Value::Point(ax, ay), Value::Point(bx, by)) => (ax - bx).abs() <= TOLERANCE && (ay - by).abs() <= TOLERANCE,
        (Value::Text(a), Value::Text(b)) => a == b,
        (Value::Text(_), _) | (_, Value::Text(_)) => false,
        _ => match (expected.numeric(), actual.numeric()) {
            (Ok(a), Ok(b)) => (a - b).abs() <= TOLERANCE,
            _ => false,
        },
    }
}
fn cached_excluded(source: &str, effective: &str, value: Option<&str>, calls: &BTreeSet<String>, exclude: &BTreeSet<String>) -> bool {
    source == "No Formula" || effective == "No Formula" || value == Some("Themed") || excluded(calls, exclude)
}
// An empty XML V serializes an empty string as well as a numeric zero. Do not
// extend this comparison rule to nonempty text or to error values.
fn cached_equal(raw: Option<&str>, expected: &Value, actual: &Value) -> bool {
    raw == Some("") && matches!(actual, Value::Text(s) if s.is_empty()) || equal(expected, actual)
}
fn formula_class(formula: &str) -> usize {
    use diagramcraft_sheet::Expr;
    fn literal(expr: &Expr) -> bool {
        match expr {
            Expr::Number(..) | Expr::Text(..) | Expr::Bool(..) => true,
            Expr::Unary(_, inner) => literal(inner),
            _ => false,
        }
    }
    if formula.trim().is_empty() {
        1
    } else if diagramcraft_sheet::parse(formula).is_ok_and(|expr| literal(&expr)) {
        0
    } else {
        2
    }
}
fn short(s: &str) -> String {
    s.chars().take(512).collect()
}
#[derive(Default)]
struct Histogram {
    counts: BTreeMap<String, u64>,
    examples: BTreeMap<String, Witness>,
}
impl Histogram {
    fn add(&mut self, name: &str, witness: &Witness) {
        let name = short(name);
        let key = if self.counts.len() < 4096 || self.counts.contains_key(&name) { name } else { "<additional categories>".into() };
        *self.counts.entry(key.clone()).or_default() += 1;
        self.examples.entry(key).or_insert_with(|| witness.clone());
    }
    fn finish(self, kind: &str, witnesses: &mut Vec<Witness>) -> Vec<(String, u64)> {
        let mut counts: Vec<_> = self.counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        // Keep all bounded text-cell buckets so promotion reports can account for
        // the complete remaining failure population, including small buckets.
        if kind != "text-metric cell" {
            counts.truncate(10);
        }
        for (name, _) in &counts {
            if let Some(example) = self.examples.get(name) {
                let mut example = example.clone();
                example.category = format!("{kind}: {name}");
                witnesses.push(example);
            }
        }
        counts
    }
}
// Geometry section and row indices do not define different cell kinds. Named User/Prop
// rows remain distinct; witnesses retain the complete original universal cell name.
fn normalized_cell(name: &str) -> String {
    let Some((section, cell)) = name.split_once('.') else { return name.into() };
    if section.strip_prefix("Geometry").is_some_and(|suffix| !suffix.is_empty() && suffix.bytes().all(|b| b.is_ascii_digit())) {
        let base = cell.trim_end_matches(|c: char| c.is_ascii_digit());
        return format!("Geometry#.{}{}", base, if base.len() < cell.len() { "#" } else { "" });
    }
    name.into()
}
#[derive(Default)]
struct Diagnostics {
    text_cells: Histogram,
    text_deltas: Histogram,
    cells: Histogram,
    functions: Histogram,
    cached_cells: Histogram,
    cached_actual: Histogram,
    cached_formula: Histogram,
}
impl Diagnostics {
    fn failure(&mut self, calls: &BTreeSet<String>, source_cell: &str, w: Witness) {
        self.cells.add(&normalized_cell(&w.cell), &w);
        if calls.iter().any(|n| diagramcraft_sheet::is_text_metric(n)) {
            self.text_cells.add(&format!("{} {}", w.oracle, normalized_cell(&w.cell)), &w);
            let number = |s: &str| {
                s.strip_prefix("Number(").or_else(|| s.strip_prefix("Some(")).and_then(|s| s.strip_suffix(')')).and_then(|s| s.parse::<f64>().ok())
            };
            let delta = match (number(&w.actual), number(&w.expected)) {
                (Some(a), Some(e)) => format!("{:.6}", a - e),
                _ => w.actual.clone(),
            };
            self.text_deltas.add(&format!("{} {} {}", w.oracle, normalized_cell(&w.cell), delta), &w);
        }
        let formula_kind = if w.formula == "Inh" {
            "Inh"
        } else if calls.is_empty() {
            "no function (other)"
        } else {
            "function calls"
        };
        if calls.is_empty() {
            self.functions.add(if formula_kind == "Inh" { "<Inh>" } else { "<no function other>" }, &w);
        } else {
            for name in calls {
                self.functions.add(name, &w);
            }
        }
        if w.oracle == "cached" {
            self.cached_cells.add(source_cell, &w);
            self.cached_actual.add(&w.actual, &w);
            self.cached_formula.add(formula_kind, &w);
        }
    }
    fn finish(self, report: &mut Report) {
        report.text_cell_failures = self.text_cells.finish("text-metric cell", &mut report.witnesses);
        report.text_delta_failures = self.text_deltas.finish("text-metric delta", &mut report.witnesses);
        report.top_failures = self.cells.finish("cell", &mut report.witnesses);
        report.function_failures = self.functions.finish("function", &mut report.witnesses);
        report.cached_cell_failures = self.cached_cells.finish("cached source cell", &mut report.witnesses);
        report.cached_actual_failures = self.cached_actual.finish("cached actual", &mut report.witnesses);
        report.cached_formula_failures = self.cached_formula.finish("cached formula kind", &mut report.witnesses);
    }
}
fn decode(s: &str) -> Result<String, String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            out.push(match chars.next() {
                Some('\\') => '\\',
                Some('t') => '\t',
                Some('r') => '\r',
                Some('n') => '\n',
                _ => return Err("invalid TSV escape".into()),
            });
        } else {
            out.push(c);
        }
    }
    Ok(out)
}
#[derive(Debug)]
struct DumpRow {
    id: u32,
    scope: String,
    name: String,
    formula: String,
    result: Option<f64>,
}
fn parse_row(line: &str) -> Result<DumpRow, String> {
    if line.len() > MAX_LINE {
        return Err("TSV line length limit".into());
    }
    let columns: Vec<_> = line.split('\t').collect();
    if columns.len() != 9 {
        return Err(format!("TSV expected 9 columns, got {}", columns.len()));
    }
    let get = |i| columns.get(i).copied().ok_or_else(|| "missing TSV column".to_string());
    let id = get(0)?.parse::<u32>().map_err(|e| e.to_string())?;
    for i in [2, 3, 4] {
        get(i)?.parse::<u32>().map_err(|e| e.to_string())?;
    }
    if !get(8)?.is_empty() {
        get(8)?.parse::<i32>().map_err(|e| e.to_string())?;
    }
    let result = get(7)?.parse::<f64>().ok().filter(|n| n.is_finite());
    Ok(DumpRow { id, scope: decode(get(1)?)?, name: decode(get(5)?)?, formula: decode(get(6)?)?, result })
}
fn map_scope(row: &DumpRow, doc: &diagramcraft_model::Document) -> Result<Scope, String> {
    let head = row.scope.split('/').next().ok_or("empty dump scope")?;
    if head == "document" {
        return Ok(Scope::Document);
    }
    if head == "style" {
        return doc.styles.iter().find(|s| s.id == row.id).map(|s| Scope::Style(s.id)).ok_or_else(|| "missing dump style".into());
    }
    if head == "page" {
        return doc.pages.first().filter(|_| doc.pages.len() == 1).map(|p| Scope::Page(p.id)).ok_or_else(|| "ambiguous blank page".into());
    }
    let mut parts = head.splitn(3, ':');
    let kind = parts.next().ok_or("scope kind")?;
    let id = parts.next().ok_or("scope ID")?.parse::<u32>().map_err(|e| e.to_string())?;
    let name = parts.next();
    match kind {
        "page" | "pagesheet" => {
            let p = doc
                .pages
                .iter()
                .find(|p| p.id == id)
                .or_else(|| name.and_then(|n| doc.pages.iter().find(|p| p.name_u == n || p.name == n)))
                // dump-blank.ps1 labels its sole rectangle page with collection index 1.
                .or_else(|| (name.is_none() && id == 1 && doc.pages.len() == 1).then(|| doc.pages.first()).flatten())
                .ok_or("missing dump page")?;
            Ok(if kind == "pagesheet" { Scope::Page(p.id) } else { Scope::PageShape { page: p.id, shape: row.id } })
        }
        "master" => {
            let m = doc
                .masters
                .iter()
                .find(|m| m.id == id)
                .or_else(|| name.and_then(|n| doc.masters.iter().find(|m| m.name_u == n || m.name == n)))
                .ok_or("missing dump master")?;
            Ok(Scope::MasterShape { master: m.id, shape: row.id })
        }
        _ => Err(format!("unknown dump scope {head}")),
    }
}
fn dump_rows(path: &Path, mut visit: impl FnMut(DumpRow) -> Result<(), String>) -> Result<(), String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() > MAX_TSV {
        return Err("TSV file size limit".into());
    }
    read_dump(BufReader::new(file.take(MAX_TSV + 1)), &path.display().to_string(), &mut visit)
}
// PowerShell Out-File emits BOM-marked UTF-16. Decode in memory without
// rewriting shared oracle files; reject malformed surrogates and partial units.
fn read_utf16_line(reader: &mut impl BufRead, little: bool, output: &mut Vec<u8>) -> Result<usize, String> {
    let mut units = Vec::new();
    let mut bytes = 0usize;
    loop {
        if reader.fill_buf().map_err(|e| e.to_string())?.is_empty() {
            break;
        }
        let mut pair = [0u8; 2];
        reader.read_exact(&mut pair).map_err(|e| e.to_string())?;
        bytes += 2;
        if bytes > MAX_LINE {
            return Err("TSV line limit".into());
        }
        let unit = if little { u16::from_le_bytes(pair) } else { u16::from_be_bytes(pair) };
        units.push(unit);
        if unit == 10 {
            break;
        }
    }
    let text = String::from_utf16(&units).map_err(|e| e.to_string())?;
    if text.len() > MAX_LINE {
        return Err("TSV decoded line limit".into());
    }
    output.extend_from_slice(text.as_bytes());
    Ok(bytes)
}
fn read_dump(mut reader: impl BufRead, label: &str, mut visit: impl FnMut(DumpRow) -> Result<(), String>) -> Result<(), String> {
    let prefix = reader.fill_buf().map_err(|e| format!("{label}: {e}"))?;
    let utf16 = match prefix.get(..2) {
        Some([0xff, 0xfe]) => Some(true),
        Some([0xfe, 0xff]) => Some(false),
        _ => None,
    };
    if utf16.is_some() {
        reader.consume(2);
    }
    let mut line = Vec::new();
    let mut rows = 0;
    let mut bytes = 0u64;
    loop {
        line.clear();
        let n = if let Some(little) = utf16 {
            read_utf16_line(&mut reader, little, &mut line).map_err(|e| format!("{label}:{}: {e}", rows + 1))?
        } else {
            reader.by_ref().take((MAX_LINE + 1) as u64).read_until(b'\n', &mut line).map_err(|e| format!("{label}: {e}"))?
        };
        if n == 0 {
            break;
        }
        bytes += n as u64;
        if n > MAX_LINE || bytes > MAX_TSV {
            return Err("TSV stream limit".into());
        }
        let text = std::str::from_utf8(&line).map_err(|e| format!("{label}:{}: {e}", rows + 1))?.trim_end_matches(['\r', '\n']);
        if rows == 0 {
            if text.trim_start_matches('\u{feff}') != HEADER {
                return Err("invalid TSV header".into());
            }
        } else {
            visit(parse_row(text).map_err(|e| format!("{label}:{}: {e}", rows + 1))?)?;
        }
        rows += 1;
        if rows > MAX_ROWS {
            return Err("TSV row count limit".into());
        }
    }
    if rows == 0 {
        return Err("empty TSV".into());
    }
    Ok(())
}
/// Root is the corpus directory. Quick mode includes all generated and blank packages,
/// then the first 50 sorted shipped packages and only their available completed dumps.
pub fn run(root: &Path, quick: bool) -> Result<Report, String> {
    let mut report = run_pass(root, quick, false)?;
    if report.include_text() {
        report.text_metrics_included = true;
    } else {
        let deferred = run_pass(root, quick, true)?;
        if (report.packages, report.dump_files) != (deferred.packages, deferred.dump_files) {
            return Err("corpus coverage changed during the text gate comparison; rerun to include the new input".into());
        }
        report.cached = deferred.cached;
        report.com = deferred.com;
        report.shipped_cached = deferred.shipped_cached;
        report.shipped_com = deferred.shipped_com;
        report.cached_classes = deferred.cached_classes;
        report.com_classes = deferred.com_classes;
    }
    report.render = Some(render::run(root)?);
    Ok(report)
}
fn text_promoted(cached: &Rate, com: &Rate, floor: f64) -> bool {
    cached.total > 0 && com.total > 0 && cached.percent() >= floor && com.percent() >= floor
}
fn run_pass(root: &Path, quick: bool, deferred: bool) -> Result<Report, String> {
    let frozen = include_str!("../../../crates/sheet/tests/floors.toml");
    let cached_floor = floor(frozen, "shipped")? * 100.;
    let com_floor = floor(frozen, "all")? * 100.;
    let generated = walk(&root.join("generated"))?;
    let blank = walk(&root.join("blank"))?;
    let shipped = walk(&root.join("ms-shipped"))?;
    let dumps = walk(&root.join("ms-shipped-dumps"))?;
    let shipped_all: Vec<_> = shipped.into_iter().filter(|p| package(p)).collect();
    let selected: Vec<_> = shipped_all.iter().take(if quick { 50 } else { usize::MAX }).cloned().collect();
    let mut files: Vec<_> = generated.iter().chain(&blank).filter(|p| package(p)).cloned().collect();
    files.extend(selected.iter().cloned());
    files.sort();
    let mut report = Report {
        render: None,
        text_metric_floor: floor(include_str!("../../../crates/sheet/tests/floors.toml"), "text-metric")? * 100.,
        text_promotion_sticky: promoted(frozen)?,
        text_cached_floor: floor(frozen, "text-cached-floor")? * 100.,
        text_com_floor: floor(frozen, "text-com-floor")? * 100.,
        text_metrics_included: false,
        cached_hosts: Default::default(),
        com_hosts: Default::default(),
        cached_classes: Default::default(),
        com_classes: Default::default(),
        cached: Rate::default(),
        com: Rate::default(),
        shipped_cached: Rate::default(),
        shipped_com: Rate::default(),
        packages: files.len(),
        shipped_packages: selected.len(),
        shipped_available: shipped_all.len(),
        dump_files: 0,
        shipped_dump_files: 0,
        cached_floor,
        com_floor,
        quick,
        top_failures: vec![],
        text_cell_failures: vec![],
        text_delta_failures: vec![],
        function_failures: vec![],
        cached_cell_failures: vec![],
        cached_actual_failures: vec![],
        cached_formula_failures: vec![],
        witnesses: vec![],
    };
    let mut associations: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    let mut by_lower = BTreeMap::new();
    for p in &files {
        let key = p.to_string_lossy().to_lowercase();
        if by_lower.insert(key, p.clone()).is_some() {
            return Err("case-ambiguous corpus package paths".into());
        }
    }
    for tsv in generated.iter().filter(|p| p.to_string_lossy().ends_with(".sheet.tsv")) {
        let stem = tsv.to_string_lossy().trim_end_matches(".sheet.tsv").to_owned();
        let key = format!("{stem}.vsdx").to_lowercase();
        let source = by_lower.get(&key).ok_or_else(|| format!("missing package for {}", tsv.display()))?;
        associations.entry(source.clone()).or_default().push(tsv.clone());
    }
    for tsv in blank.iter().filter(|p| p.extension().is_some_and(|e| e == "tsv")) {
        let file = if tsv.file_name().is_some_and(|n| n == "rectangle.tsv") { "blank-rect.vsdx" } else { "blank.vsdx" };
        let source = root.join("blank").join(file);
        if !source.is_file() {
            return Err(format!("missing blank package {}", source.display()));
        }
        associations.entry(source).or_default().push(tsv.clone());
    }
    for tsv in dumps.iter().filter(|p| p.extension().is_some_and(|e| e == "tsv")) {
        let relative = tsv.strip_prefix(root.join("ms-shipped-dumps")).map_err(|e| e.to_string())?;
        let package_path = root.join("ms-shipped").join(relative).with_extension("");
        let key = package_path.to_string_lossy().to_lowercase();
        if let Some(source) = by_lower.get(&key) {
            associations.entry(source.clone()).or_default().push(tsv.clone());
        } else if !quick {
            return Err(format!("unmatched shipped dump {}", tsv.display()));
        }
    }
    let mut exclude = exclusions();
    if deferred {
        exclude.extend(diagramcraft_sheet::TEXT_METRIC_FUNCTIONS.iter().map(|n| (*n).to_string()));
    }
    let mut diagnostics = Diagnostics::default();
    for path in files {
        let is_shipped = path.starts_with(root.join("ms-shipped"));
        let document = read_package(&path)?;
        let themes = diagramcraft_theme::Themes::from_document(&document).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut evaluator = Evaluator::new(&document)
            .map_err(|e| format!("{}: {e}", path.display()))?
            .with_theme_host(&themes)
            .with_text_host(&diagramcraft_text::Host);
        if deferred {
            evaluator = evaluator.with_deferred_text_metrics();
        }
        let scopes: Vec<_> = evaluator.scopes().collect();
        for scope in scopes {
            for named in evaluator.source_named_cells(scope).map_err(|e| e.to_string())? {
                let Some(formula) = named.cell.formula.as_deref() else {
                    continue;
                };
                let effective = evaluator.effective_formula(scope, &named.name);
                let effective = effective.as_deref().unwrap_or(formula);
                let calls = function_calls(effective)?;
                if cached_excluded(formula, effective, named.cell.value.as_deref(), &calls, &exclude) {
                    report.cached.excluded += 1;
                    if is_shipped {
                        report.shipped_cached.excluded += 1;
                    }
                    continue;
                }
                let mut original = named.cell.clone();
                original.error = None;
                let expected = evaluator.cached_cell(&original);
                let actual = evaluator.evaluate_snapshot(scope, &named.name);
                let matched = cached_equal(named.cell.value.as_deref(), &expected, &actual);
                record_hosts(&mut report.cached_hosts, &calls, matched);
                report.cached_classes[formula_class(effective)].record(matched);
                report.cached.record(matched);
                if is_shipped {
                    report.shipped_cached.record(matched);
                }
                if !matched {
                    diagnostics.failure(
                        &calls,
                        &named.cell.name,
                        Witness {
                            oracle: "cached",
                            category: String::new(),
                            file: path.display().to_string(),
                            scope: format!("{scope:?}"),
                            cell: named.name,
                            formula: short(formula),
                            expected: short(&format!("{expected:?}")),
                            actual: short(&format!("{actual:?}")),
                        },
                    );
                }
            }
        }
        if let Some(tsvs) = associations.get(&path) {
            for tsv in tsvs {
                report.dump_files += 1;
                if is_shipped {
                    report.shipped_dump_files += 1;
                }
                dump_rows(tsv, |row| {
                    let calls = function_calls(&row.formula)?;
                    if excluded(&calls, &exclude) {
                        report.com.excluded += 1;
                        if is_shipped {
                            report.shipped_com.excluded += 1;
                        }
                        return Ok(());
                    }
                    let actual = match map_scope(&row, &document) {
                        Ok(scope) => evaluator.evaluate_snapshot(scope, &row.name),
                        Err(_) => Value::Error(diagramcraft_sheet::Error::Ref),
                    };
                    // COM ResultIU is numeric even for strings; ShapeSheet numeric coercion maps nonnumeric text to zero.
                    let matched = match (row.result, actual.numeric()) {
                        (Some(a), Ok(b)) => (a - b).abs() <= TOLERANCE,
                        _ => false,
                    };
                    record_hosts(&mut report.com_hosts, &calls, matched);
                    report.com_classes[formula_class(&row.formula)].record(matched);
                    report.com.record(matched);
                    if is_shipped {
                        report.shipped_com.record(matched);
                    }
                    if !matched {
                        diagnostics.failure(
                            &calls,
                            &row.name.clone(),
                            Witness {
                                oracle: "COM",
                                category: String::new(),
                                file: tsv.display().to_string(),
                                scope: row.scope,
                                cell: row.name,
                                formula: short(&row.formula),
                                expected: format!("{:?}", row.result),
                                actual: short(&format!("{actual:?}")),
                            },
                        );
                    }
                    Ok(())
                })?;
            }
        }
    }
    diagnostics.finish(&mut report);
    Ok(report)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn utf16_dump_matches_utf8_and_rejects_hostile_encoding() {
        use std::io::Cursor;
        let text = format!("{HEADER}\n1\tpage:0/shape:1\t1\t0\t0\tWidth\t1\t1\t65\n");
        for little in [false, true] {
            let mut encoded = if little { vec![0xff, 0xfe] } else { vec![0xfe, 0xff] };
            for unit in text.encode_utf16() {
                encoded.extend_from_slice(&if little { unit.to_le_bytes() } else { unit.to_be_bytes() });
            }
            let mut count = 0;
            read_dump(Cursor::new(encoded), "utf16", |_| {
                count += 1;
                Ok(())
            })
            .unwrap();
            assert_eq!(count, 1);
        }
        for bytes in [vec![0xff, 0xfe, 0x00], vec![0xff, 0xfe, 0x00, 0xd8], vec![0xff, 0xfe, 0x00, 0xdc]] {
            assert!(read_dump(Cursor::new(bytes), "hostile", |_| Ok(())).is_err());
        }
        assert!(
            read_dump(Cursor::new(vec![0xff, 0xfe].into_iter().chain(std::iter::repeat_n(65, MAX_LINE + 2)).collect::<Vec<_>>()), "long", |_| Ok(()))
                .is_err()
        );
    }
    #[test]
    fn strict_denominator_classes_and_empty_serialized_string() {
        for formula in ["No Formula", "NOW()"] {
            assert!(cached_excluded("Inh", formula, Some("0"), &function_calls(formula).unwrap(), &exclusions()));
        }
        for formula in [
            "TEXTWIDTH(TheText)",
            "TEXTHEIGHT(TheText)",
            "SHAPETEXT(TheText)",
            "THEMEVAL()",
            "THEMEGUARD(1)",
            "THEMERESTORE(1)",
            "THEME(1)",
            "THEMECBV(1,1)",
            "THEMEPROP(1)",
            "ISTHEMED()",
            "CELLISTHEMED(FALSE)",
        ] {
            assert!(
                !cached_excluded("Inh", formula, Some("0"), &function_calls(formula).unwrap(), &exclusions()),
                "implemented theme cells must be measured: {formula}"
            );
        }
        assert!(cached_excluded("No Formula", "1", Some("1"), &BTreeSet::new(), &exclusions()));
        assert!(!cached_excluded("Inh", "Width*2", Some("4"), &BTreeSet::new(), &exclusions()));
        for f in ["-1 in", "TRUE", "\"hello\"", "25%"] {
            assert_eq!(formula_class(f), 0, "{f}");
        }
        assert_eq!(formula_class("  "), 1);
        for f in ["Width", "RGB(1,2,3)", "1+2", "broken("] {
            assert_eq!(formula_class(f), 2, "{f}");
        }
        assert!(cached_equal(Some(""), &Value::Number(0.), &Value::Text(String::new())));
        assert!(!cached_equal(Some("0"), &Value::Number(0.), &Value::Text(String::new())));
        assert!(!cached_equal(Some(""), &Value::Number(0.), &Value::Text("x".into())));
        assert!(!cached_equal(Some(""), &Value::Number(0.), &Value::Error(diagramcraft_sheet::Error::Ref)));
    }
    #[test]
    fn functions_ignore_strings_and_match_only_calls() {
        let names = function_calls(r#"IF(1,"NOW() "" RAND()",_UCON_X (2))+Sheet.1!NOW+SUM(2)"#).unwrap();
        assert_eq!(names, BTreeSet::from(["IF".into(), "SUM".into(), "_UCON_X".into()]));
        assert!(excluded(&names, &exclusions()));
        assert!(!excluded(&function_calls("_OTHER(1)+RAND").unwrap(), &exclusions()));
        assert!(function_calls(&"A".repeat(MAX_LINE + 1)).is_err());
        for formula in ["Pages[NOW()]!PageWidth", "'THEMEVAL()'!Width", "'a''NOW()'!Width", "Pages[Page[NOW()]]!Width"] {
            assert!(function_calls(formula).unwrap().is_empty(), "{formula}");
        }
        assert_eq!(function_calls("NOW()+Pages[RAND()]!Width+'THEMEVAL()'!Height").unwrap(), BTreeSet::from(["NOW".into()]));
    }
    #[test]
    fn escaped_rows_and_hostile_inputs() {
        let row = parse_row("1\tpage:1:x/Sheet.1\t1\t2\t3\tUser.X\t\"a\\tb\\\\c\"\t0\t32").unwrap();
        assert_eq!(row.formula, "\"a\tb\\c\"");
        assert_eq!(row.result, Some(0.));
        for bad in ["", HEADER, "1\tx\t1\t2\t3\tx\tbad\\q\t0\t32", "-1\tx\t1\t2\t3\tx\t0\t0\t32", "1\tx\t4294967296\t2\t3\tx\t0\t0\t32"] {
            assert!(parse_row(bad).is_err(), "{bad}");
        }
        assert!(parse_row(&"x".repeat(MAX_LINE + 1)).is_err());
        assert_eq!(parse_row("1\tx\t1\t2\t3\tx\t0\tNaN\t32").unwrap().result, None);
        assert_eq!(decode("x\\"), Err("invalid TSV escape".into()));
    }
    #[test]
    fn comparisons_never_pass_errors_and_require_exact_strings() {
        assert!(!equal(&Value::Error(diagramcraft_sheet::Error::Ref), &Value::Error(diagramcraft_sheet::Error::Ref)));
        assert!(!equal(&Value::Text("0".into()), &Value::Number(0.)));
        assert!(equal(&Value::Number(2.), &Value::Number(2.0000001)));
        assert!(!equal(&Value::Number(2.), &Value::Number(2.00001)));
        assert!(equal(&Value::Point(2., 3.), &Value::Point(2.0000001, 3.0000001)));
        assert!(!equal(&Value::Point(2., 3.), &Value::Point(2., 3.00001)));
        assert!(!equal(&Value::Point(f64::NAN, 3.), &Value::Point(f64::NAN, 3.)));
        assert!(!equal(&Value::Point(2., 3.), &Value::Number(2.)));
        assert!(floor("shipped=NaN", "shipped").is_err());
        assert!(floor("shipped=1\nshipped=1", "shipped").is_err());
    }
    #[test]
    fn scopes_are_scoped_not_global_shape_ids() {
        let doc = diagramcraft_model::Document {
            pages: vec![diagramcraft_model::Page { id: 7, name_u: "Named".into(), ..Default::default() }],
            masters: vec![diagramcraft_model::Master { id: 2, ..Default::default() }],
            ..Default::default()
        };
        let mut row = DumpRow { id: 5, scope: "page:7:Named/Group/Sheet.5".into(), name: "Width".into(), formula: "1".into(), result: Some(1.) };
        assert_eq!(map_scope(&row, &doc), Ok(Scope::PageShape { page: 7, shape: 5 }));
        row.scope = "master:2:Master/Sheet.5".into();
        assert_eq!(map_scope(&row, &doc), Ok(Scope::MasterShape { master: 2, shape: 5 }));
        row.scope = "page:1:Named/Sheet.5".into();
        assert_eq!(map_scope(&row, &doc), Ok(Scope::PageShape { page: 7, shape: 5 }));
        row.scope = "page:1/Sheet.5".into();
        assert_eq!(map_scope(&row, &doc), Ok(Scope::PageShape { page: 7, shape: 5 }));
    }
    #[test]
    fn stream_rejects_unbounded_or_incomplete_records() {
        use std::io::Cursor;
        let text = format!("{HEADER}\r\n1\tpage:1/Sheet.1\t1\t2\t3\tWidth\t1\t1\t32\r\n");
        let mut count = 0;
        read_dump(Cursor::new(text.as_bytes()), "fixture", |_| {
            count += 1;
            Ok(())
        })
        .unwrap();
        assert_eq!(count, 1);
        assert!(read_dump(Cursor::new(b""), "fixture", |_| Ok(())).is_err());
        assert!(read_dump(Cursor::new(b"wrong\n"), "fixture", |_| Ok(())).is_err());
        let huge = format!("{HEADER}\n{}", "x".repeat(MAX_LINE + 1));
        assert!(read_dump(Cursor::new(huge.as_bytes()), "fixture", |_| Ok(())).is_err());
        let bad = format!("{HEADER}\n1\tx");
        assert!(read_dump(Cursor::new(bad.as_bytes()), "fixture", |_| Ok(())).is_err());
    }
    #[test]
    fn shipped_floor_cannot_be_masked_by_overall_and_empty_oracles_fail() {
        let mut report = Report {
            render: None,
            text_metric_floor: 90.,
            text_promotion_sticky: false,
            text_cached_floor: 90.,
            text_com_floor: 90.,
            text_metrics_included: false,
            cached_hosts: Default::default(),
            com_hosts: Default::default(),
            cached_classes: Default::default(),
            com_classes: Default::default(),
            cached: Rate { matched: 999, total: 1000, excluded: 0 },
            shipped_cached: Rate { matched: 90, total: 100, excluded: 0 },
            com: Rate { matched: 90, total: 100, excluded: 0 },
            shipped_com: Rate::default(),
            packages: 0,
            shipped_packages: 0,
            shipped_available: 0,
            dump_files: 0,
            shipped_dump_files: 0,
            cached_floor: 97.,
            com_floor: 90.,
            quick: false,
            top_failures: vec![],
            text_cell_failures: vec![],
            text_delta_failures: vec![],
            function_failures: vec![],
            cached_cell_failures: vec![],
            cached_actual_failures: vec![],
            cached_formula_failures: vec![],
            witnesses: vec![],
        };
        assert!(!report.passed());
        assert!(!report.cells_passed());
        let display = report.to_string();
        assert!(display.lines().any(|line| line.starts_with("cached overall informational:") && line.ends_with("floor=reported separately")));
        assert!(display.lines().any(|line| line.starts_with("shipped cached:") && line.ends_with("floor=97.000000%")));
        report.shipped_cached.matched = 97;
        report.cached.matched = 800;
        assert!(report.passed());
        report.render = Some(render::Report::default());
        assert!(report.cells_passed(), "cell-only CI gate must still measure cell floors");
        assert!(!report.passed(), "CLI gate must reject failed render comparisons");
        report.render = None;
        // Promotion is permanent: poor text cannot select the old denominator.
        report.text_promotion_sticky = true;
        report.text_cached_floor = 91.;
        report.text_com_floor = 92.;
        report.cached_hosts[1] = Rate { matched: 91, total: 100, excluded: 0 };
        report.com_hosts[1] = Rate { matched: 92, total: 100, excluded: 0 };
        assert!(report.include_text());
        assert!(report.cells_passed());
        for (cached, com) in [(89, 100), (100, 89), (90, 100), (100, 91), (0, 0)] {
            report.cached_hosts[1].matched = cached;
            report.com_hosts[1].matched = com;
            assert!(report.include_text(), "a promoted denominator cannot shrink");
            assert!(!report.cells_passed(), "text floors must remain mandatory");
        }
        report.text_promotion_sticky = false;
        report.shipped_cached = Rate::default();
        assert!(!report.passed());
        report.shipped_cached = Rate { matched: 97, total: 100, excluded: 0 };
        report.com = Rate::default();
        assert!(!report.passed());
    }
    #[test]
    fn diagnostics_normalize_geometry_but_retain_named_user_rows_and_late_witnesses() {
        assert_eq!(normalized_cell("Geometry57.X234"), "Geometry#.X#");
        assert_eq!(normalized_cell("Geometry3.NoFill"), "Geometry#.NoFill");
        assert_eq!(normalized_cell("User.Named123"), "User.Named123");
        let mut histogram = Histogram::default();
        let mut witness = Witness {
            oracle: "cached",
            category: String::new(),
            file: "early".into(),
            scope: "Document".into(),
            cell: "Width".into(),
            formula: "Inh".into(),
            expected: "Number(1)".into(),
            actual: "Error(Ref)".into(),
        };
        for n in 0..100 {
            histogram.add(&format!("early{n}"), &witness);
        }
        witness.file = "late".into();
        for _ in 0..10 {
            histogram.add("dominant late", &witness);
        }
        let mut examples = vec![];
        let counts = histogram.finish("cell", &mut examples);
        assert_eq!(counts.first(), Some(&("dominant late".into(), 10)));
        assert_eq!(examples.first().unwrap().file, "late");
    }
    #[test]
    fn angular_prime_units_do_not_hide_following_function_calls() {
        for literal in ["5'", "5\"", "5 '", "5 \"", ".5'", "5. \"", "1e-3'", "1E+2 \"", "5 1/2 '", "5\t'", "5\u{a0}\""] {
            for name in ["NOW", "RAND"] {
                let formula = format!("{literal}+{name}()");
                assert!(diagramcraft_sheet::parse(&formula).is_ok(), "parser rejected {formula}");
                assert_eq!(function_calls(&formula).unwrap(), BTreeSet::from([name.to_string()]), "{formula}");
            }
        }
        for formula in ["'Page5 NOW()'!Width", "Pages[5\"NOW()]!Width", "\"5' NOW()\"", "Sheet.5!Prop.Row_5"] {
            assert!(function_calls(formula).unwrap().is_empty(), "{formula}");
        }
        let formula = "Pages[5\"]!Width+NOW()";
        assert!(diagramcraft_sheet::parse(formula).is_ok());
        assert_eq!(function_calls(formula).unwrap(), BTreeSet::from(["NOW".into()]));
        assert!(function_calls("5'+NOW(").unwrap().contains("NOW")); // Malformed formulas retain lexical call evidence.
    }
}

#[cfg(test)]
mod text_gate_tests {
    use super::*;
    #[test]
    fn both_text_oracles_must_reach_ninety_percent() {
        let low = Rate { matched: 89, total: 100, excluded: 0 };
        let ready = Rate { matched: 90, total: 100, excluded: 0 };
        assert!(!text_promoted(&low, &ready, 90.));
        assert!(!text_promoted(&ready, &low, 90.));
        assert!(!text_promoted(&Rate::default(), &ready, 90.));
        assert!(text_promoted(&ready, &ready, 90.));
        assert_eq!(diagramcraft_sheet::TEXT_METRIC_FUNCTIONS.len(), 3);
        assert_eq!(promoted(include_str!("../../../crates/sheet/tests/floors.toml")), Ok(true));
        for invalid in ["", "promoted=maybe", "promoted=true\npromoted=false"] {
            assert!(promoted(invalid).is_err());
        }
    }
}
