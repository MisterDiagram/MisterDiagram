//! Document bindings for the model-independent ShapeSheet evaluator.
use misterdiagram_model::{Cell, Document, Row, Section, Shape, Sheet};
use misterdiagram_sheet::{Context, Error, Value};
use std::collections::{BTreeMap, BTreeSet};
mod defaults;
mod geometry;
mod inherit;
mod interval;
mod path;
mod relationships;
mod string_cells;
mod text;
mod transform;
pub use text::TextBlock;
const MAX_CELLS: usize = 1_000_000;
const MAX_SCOPES: usize = 100_000;

/// IDs are document/page/master IDs, never vector positions. Shape IDs are scoped to their page.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scope {
    Document,
    Page(u32),
    PageShape { page: u32, shape: u32 },
    Master(u32),
    MasterShape { master: u32, shape: u32 },
    Style(u32),
}
/// Canonical universal cell name and its original row/section metadata.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
#[derive(Clone, Debug)]
pub struct NamedCell<'a> {
    pub name: String,
    pub cell: &'a Cell,
    pub section: Option<&'a Section>,
    pub row: Option<&'a Row>,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    scope: Scope,
    name: String,
}
struct CachedValue {
    value: Value,
    shared: bool,
}
#[derive(Default)]
struct Snapshot {
    formulas: BTreeMap<Key, Option<String>>,
    edges: Vec<(Key, Key)>,
}
/// Local document evaluation with cached hooks, bounded dependencies and explicit error values.
/// Master/style/default inheritance is layered on these scopes separately.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
pub struct Evaluator<'a> {
    defer_text_metrics: bool,
    internal_cache_units: bool,
    glue_host: Option<&'a mut dyn misterdiagram_sheet::HostHooks>,
    text_host: Option<&'a dyn misterdiagram_model::text::TextHost>,
    theme_host: Option<&'a dyn misterdiagram_sheet::host::ThemeHost>,
    inheritance: inherit::Inheritance<'a>,
    pub document: &'a Document,
    sheets: BTreeMap<Scope, &'a Sheet>,
    shapes: BTreeMap<Scope, &'a Shape>,
    parents: BTreeMap<Scope, Scope>,
    cells: BTreeMap<Key, &'a Cell>,
    names: BTreeMap<Scope, Vec<NamedCell<'a>>>,
    values: BTreeMap<Key, CachedValue>,
    active: BTreeSet<Key>,
    resolving_cycle: bool,
    work: usize,
    eval_depth: usize,
    eval_work: usize,
    indexed_name_bytes: usize,
    palette: BTreeMap<u32, [u8; 3]>,
    fonts: BTreeMap<u32, String>,
    overrides: BTreeMap<Key, String>,
    override_bytes: usize,
    arguments: Vec<BTreeMap<String, Value>>,
    argument_bytes: usize,
    context_epoch: std::cell::Cell<u64>,
    assignments: usize,
    revision: usize,
    dependencies: BTreeMap<Key, BTreeSet<Key>>,
    dependents: BTreeMap<Key, BTreeSet<Key>>,
    dirty: BTreeSet<Key>,
    dependency_count: usize,
    dependency_bytes: usize,
    graph_limited: bool,
    stack: Vec<Key>,
    snapshot: Option<Snapshot>,
}
impl<'a> Evaluator<'a> {
    /// Register literal colours without requiring a serialized palette entry.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
    fn intern_color(&mut self, rgb: [u8; 3]) -> Option<u32> {
        if let Some(index) = self.palette.iter().find_map(|(index, value)| (*value == rgb).then_some(*index)) {
            return Some(index);
        }
        if self.palette.len() >= 65536 {
            return None;
        }
        let index = self.palette.last_key_value().map_or(Some(24), |(i, _)| i.checked_add(1).map(|i| i.max(24)))?;
        self.palette.insert(index, rgb);
        Some(index)
    }

    pub fn new(document: &'a Document) -> Result<Self, Error> {
        if document.colors.len() > 65536 {
            return Err(Error::Limit);
        }
        let mut result = Self {
            theme_host: None,
            text_host: None,
            glue_host: None,
            defer_text_metrics: false,
            internal_cache_units: false,
            document,
            inheritance: Default::default(),
            sheets: BTreeMap::new(),
            shapes: BTreeMap::new(),
            parents: BTreeMap::new(),
            cells: BTreeMap::new(),
            names: BTreeMap::new(),
            values: BTreeMap::new(),
            active: BTreeSet::new(),
            resolving_cycle: false,
            work: 0,
            eval_depth: 0,
            eval_work: 0,
            indexed_name_bytes: 0,
            overrides: BTreeMap::new(),
            override_bytes: 0,
            arguments: Vec::new(),
            argument_bytes: 0,
            context_epoch: std::cell::Cell::new(0),
            assignments: 0,
            revision: 0,
            dependencies: BTreeMap::new(),
            dependents: BTreeMap::new(),
            dirty: BTreeSet::new(),
            dependency_count: 0,
            dependency_bytes: 0,
            graph_limited: false,
            stack: Vec::new(),
            snapshot: None,
            fonts: font_ids(document)?,
            palette: document.colors.iter().filter_map(|(i, c)| parse_color(c).map(|c| (*i, c))).collect(),
        };
        result.add_sheet(Scope::Document, &document.document_sheet)?;
        for style in &document.styles {
            result.add_sheet(Scope::Style(style.id), &style.sheet)?;
        }
        for page in &document.pages {
            result.add_sheet(Scope::Page(page.id), &page.sheet)?;
            result.add_shapes(&page.shapes, Scope::Page(page.id))?;
        }
        for master in &document.masters {
            result.add_sheet(Scope::Master(master.id), &master.sheet)?;
            result.add_shapes(&master.shapes, Scope::Master(master.id))?;
        }
        result.install_inheritance()?;
        Ok(result)
    }
    /// Writer caches use physical internal inches, independently of displayed drawing units.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub(crate) fn with_internal_cache_units(mut self) -> Self {
        self.internal_cache_units = true;
        self
    }
    /// Convert evaluator colour/font/language identities into serialized cell values.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/font-cell-character-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/textbkgnd-cell-text-block-format-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/langid-cell-character-section
    pub fn serialized_number(&self, scope: Scope, name: &str, cell: &Cell, number: f64) -> Result<String, Error> {
        if !number.is_finite() {
            return Err(Error::Num);
        }
        let index = || -> Result<u32, Error> {
            if !(0. ..=f64::from(u32::MAX)).contains(&number) || number.fract() != 0. {
                return Err(Error::Value);
            }
            Ok(number as u32)
        };
        if matches!(cell.name.as_str(), "LangID" | "DocLangID") {
            let id = u16::try_from(index()?).map_err(|_| Error::Value)?;
            return misterdiagram_sheet::language::tag(id).map(str::to_string).ok_or(Error::Value);
        }
        if cell.name == "Font" {
            let face = self.fonts.get(&index()?).ok_or(Error::Value)?;
            let ordinal = self.document.face_names.values().position(|f| f.eq_ignore_ascii_case(face)).ok_or(Error::Value)?;
            return Ok(ordinal.to_string());
        }
        if Self::baseline_metadata(scope, name).is_some_and(|m| m.3 == 251) {
            let mut index = index()?;
            if cell.name == "TextBkgnd" {
                if index == 0 {
                    return Ok("0".into());
                }
                index -= 1;
            }
            let rgb = self.palette.get(&index).copied().or_else(|| misterdiagram_sheet::functions::default_color(index)).ok_or(Error::Value)?;
            return Ok(format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]));
        }
        misterdiagram_model::skeleton::number_text(number).ok_or(Error::Num)
    }
    /// Resolve omitted length units from the owning metric drawing or master.
    /// Witnesses: scratch metric Para.IndFirst and SOCIAL_DISTANCING_M PageSheet.
    /// Explicit physical U wins. Master DL resolves through its PageSheet as COM does.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub fn default_unit_code(&self, scope: Scope, name: &str, cell: &Cell, code: u16) -> u16 {
        if code != 73
            || cell.units.is_some() && !(cell.units.as_deref() == Some("DL") && matches!(scope, Scope::MasterShape { .. }))
            || matches!(name, "DrawingScale" | "PageScale")
        {
            return code;
        }
        let unit = |scope| self.sheet(scope).and_then(|s| s.cell("DrawingScale")).and_then(|c| c.units.as_deref());
        let metric = self
            .document
            .parts
            .get("docProps/custom.xml")
            .and_then(|p| p.xml.as_ref())
            .is_some_and(|root| root.elements().any(|p| p.attr("name") == Some("IsMetric") && matches!(p.text().as_str(), "true" | "1")));
        let (unit, master) = match scope {
            Scope::Master(id) | Scope::MasterShape { master: id, .. } => (unit(Scope::Master(id)), true),
            Scope::Page(id) | Scope::PageShape { page: id, .. } => (unit(Scope::Page(id)), false),
            Scope::Document | Scope::Style(_) if metric => (self.document.pages.first().and_then(|p| unit(Scope::Page(p.id))).or(Some("MM")), false),
            _ => (None, false),
        };
        match unit {
            Some("MM") => 70,
            Some("CM") => 69,
            Some("M") => 71,
            Some("IN") if master => 65,
            Some("FT") if master => 66,
            _ => code,
        }
    }
    /// Whether the comparison-only mode has deferred real text metrics.
    pub fn text_metrics_deferred(&self) -> bool {
        self.defer_text_metrics
    }
    /// Discard all evaluation effects while retaining the immutable source/inheritance index.
    /// Each independently rendered page/master starts with fresh caches, SETF overrides,
    /// dependency budgets and dynamic palette indices, exactly as a new evaluator does.
    pub fn reset_evaluation(&mut self) {
        self.values.clear();
        self.active.clear();
        self.resolving_cycle = false;
        self.work = 0;
        self.eval_depth = 0;
        self.eval_work = 0;
        self.palette = self.document.colors.iter().filter_map(|(i, c)| parse_color(c).map(|c| (*i, c))).collect();
        self.overrides.clear();
        self.override_bytes = 0;
        self.arguments.clear();
        self.argument_bytes = 0;
        self.assignments = 0;
        self.revision = 0;
        self.dependencies.clear();
        self.dependents.clear();
        self.dirty.clear();
        self.dependency_count = 0;
        self.dependency_bytes = 0;
        self.graph_limited = false;
        self.stack.clear();
        self.snapshot = None;
    }
    /// Inject a theme resolver from the application composition layer.
    pub fn with_theme_host(mut self, host: &'a dyn misterdiagram_sheet::host::ThemeHost) -> Self {
        self.theme_host = Some(host);
        self.values.clear();
        self
    }
    pub fn with_text_host(mut self, host: &'a dyn misterdiagram_model::text::TextHost) -> Self {
        self.text_host = Some(host);
        self.values.clear();
        self
    }
    /// Inject runtime glue hooks while unimplemented hooks retain their source caches.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/begtrigger-cell-glue-info-section
    pub fn with_glue_host(mut self, host: &'a mut dyn misterdiagram_sheet::HostHooks) -> Self {
        self.glue_host = Some(host);
        self.values.clear();
        self
    }
    /// M2.3 comparison mode for the P2-followup promotion gate only. Text rows are
    /// excluded by the corpus harness; their dependents retain the old cached inputs.
    /// Rendering and text-only measurement must never select this mode.
    pub fn with_deferred_text_metrics(mut self) -> Self {
        self.defer_text_metrics = true;
        self.values.clear();
        self
    }
    fn add_shapes(&mut self, shapes: &'a [Shape], parent: Scope) -> Result<(), Error> {
        if shapes.len() > MAX_SCOPES {
            return Err(Error::Limit);
        }
        let mut pending: Vec<_> = shapes.iter().map(|s| (s, 0usize, parent)).collect();
        while let Some((shape, depth, shape_parent)) = pending.pop() {
            // Deleted master-child markers have no live sheet identity. Visio may repeat
            // ID=4294967295 for several such markers in a single page.
            if shape.sheet.attributes.get("Del").is_some_and(|v| v == "1" || v == "true") {
                continue;
            }
            if depth > 128 || self.shapes.len() >= MAX_SCOPES {
                return Err(Error::Limit);
            }
            let scope = match parent {
                Scope::Page(page) => Scope::PageShape { page, shape: shape.id },
                Scope::Master(master) => Scope::MasterShape { master, shape: shape.id },
                _ => return Err(Error::Ref),
            };
            self.shapes.insert(scope, shape);
            self.parents.insert(scope, shape_parent);
            self.add_sheet(scope, &shape.sheet)?;
            for s in &shape.shapes {
                if pending.len() >= MAX_SCOPES {
                    return Err(Error::Limit);
                }
                pending.push((s, depth + 1, scope));
            }
        }
        Ok(())
    }
    fn add_sheet(&mut self, scope: Scope, sheet: &'a Sheet) -> Result<(), Error> {
        if self.sheets.len() >= MAX_SCOPES {
            return Err(Error::Limit);
        }
        if self.sheets.insert(scope, sheet).is_some() {
            return Err(Error::Value);
        }
        let mut names = vec![];
        for c in &sheet.cells {
            self.add_cell(scope, c.name.clone(), c, None, None, &mut names)?;
        }
        for section in &sheet.sections {
            if section.deleted {
                continue;
            }
            let prefix = section_prefix(section)?;
            for c in &section.cells {
                self.add_cell(scope, format!("{prefix}.{}", c.name), c, Some(section), None, &mut names)?;
            }
            for (position, row) in section.rows.iter().enumerate() {
                if row.deleted {
                    continue;
                }
                for c in &row.cells {
                    let name = cell_name(section, row, position, c)?;
                    self.add_cell(scope, name.clone(), c, Some(section), Some(row), &mut names)?;
                }
            }
        }
        self.names.insert(scope, names);
        Ok(())
    }
    fn index_cell(&mut self, scope: Scope, name: String, cell: &'a Cell) -> Result<(), Error> {
        if self.cells.len() >= MAX_CELLS || name.len() > 65536 {
            return Err(Error::Limit);
        }
        self.indexed_name_bytes = self.indexed_name_bytes.checked_add(name.len()).ok_or(Error::Limit)?;
        if self.indexed_name_bytes > 64 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        let key = Key { scope, name: name.to_ascii_lowercase() };
        self.cells.insert(key, cell);
        Ok(())
    }
    fn add_cell(
        &mut self,
        scope: Scope,
        name: String,
        cell: &'a Cell,
        section: Option<&'a Section>,
        row: Option<&'a Row>,
        names: &mut Vec<NamedCell<'a>>,
    ) -> Result<(), Error> {
        self.index_cell(scope, name.clone(), cell)?;
        names.push(NamedCell { name, cell, section, row });
        Ok(())
    }
    pub fn scopes(&self) -> impl Iterator<Item = Scope> + '_ {
        self.sheets.keys().copied()
    }
    pub fn sheet(&self, scope: Scope) -> Option<&'a Sheet> {
        self.sheets.get(&scope).copied()
    }
    pub(crate) fn source_parent(&self, scope: Scope) -> Option<Scope> {
        self.parents.get(&scope).copied()
    }
    pub fn shape(&self, scope: Scope) -> Option<&'a Shape> {
        self.shapes.get(&scope).copied()
    }
    pub fn named_cells(&self, scope: Scope) -> &[NamedCell<'a>] {
        self.names.get(&scope).map(Vec::as_slice).unwrap_or(&[])
    }
    /// Original source cells, including masked rows, for the cached-value oracle.
    /// Inherited entries are deliberately absent from this enumeration.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub fn source_named_cells(&self, scope: Scope) -> Result<Vec<NamedCell<'a>>, Error> {
        let Some(sheet) = self.sheet(scope) else { return Err(Error::Ref) };
        let mut cells = Vec::new();
        for cell in &sheet.cells {
            cells.push(NamedCell { name: cell.name.clone(), cell, section: None, row: None });
        }
        for section in &sheet.sections {
            let prefix = section_prefix(section)?;
            for cell in &section.cells {
                cells.push(NamedCell { name: format!("{prefix}.{}", cell.name), cell, section: Some(section), row: None });
            }
            for (position, row) in section.rows.iter().enumerate() {
                for cell in &row.cells {
                    cells.push(NamedCell { name: cell_name(section, row, position, cell)?, cell, section: Some(section), row: Some(row) });
                }
            }
        }
        Ok(cells)
    }
    pub fn cell(&self, scope: Scope, name: &str) -> Option<&'a Cell> {
        indexed_key(&self.cells, scope, name).and_then(|key| self.cells.get(&key).copied()).or_else(|| self.baseline_cell(scope, name))
    }
    fn baseline_cell(&self, scope: Scope, name: &str) -> Option<&'a Cell> {
        if !self.sheets.contains_key(&scope) || !self.defaults_allowed(scope, name) {
            return None;
        }
        let baseline = defaults::cell(scope, name).or_else(|| self.reserved_default(scope, name)).or_else(|| self.virtual_path(scope, name))?;
        self.baseline_provider(scope, name)
            .and_then(|p| self.cells.get(&Key { scope: p, name: name.to_ascii_lowercase() }).copied())
            .or(Some(baseline))
    }
    /// Geometry.Path is a reference-valued virtual slot; COM ResultIU is numeric zero.
    /// Presence follows effective geometry, including master inheritance and tombstones.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.viscellindices
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/pointalongpath-function
    fn virtual_path(&self, scope: Scope, name: &str) -> Option<&'static Cell> {
        let index = path_index(name)?;
        self.geometry_indices(scope).any(|i| i == index).then_some(&*PATH_CELL)
    }
    /// Reserved formatting columns exist in every actual row. Values and COM metadata
    /// are taken from the authorized blank baseline, not from the document's oracle dump.
    /// Witness: ADS_M.vssx master part2 shape10, Character IX1, Char.Q2 through Char.BE2.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.rowscellcount
    fn reserved_default(&self, scope: Scope, name: &str) -> Option<&'static Cell> {
        let (prototype, row) = reserved_prototype(name)?;
        let cell = defaults::cell(scope, &prototype)?;
        if !reserved_cell(cell) {
            return None;
        }
        let section = cell.attributes.get("ComSection")?.parse::<u16>().ok()?;
        self.formatting_rows(scope).contains_key(&(section, row)).then_some(cell)
    }
    fn formatting_rows(&self, scope: Scope) -> BTreeMap<(u16, u32), (&'a Section, &'a Row)> {
        let mut rows = BTreeMap::new();
        for named in self.named_cells(scope) {
            let (Some(section), Some(row)) = (named.section, named.row) else { continue };
            let section_id = match section.name.as_str() {
                "Character" => 3,
                "Paragraph" => 4,
                _ => continue,
            };
            let index = row.index.or_else(|| section.rows.iter().position(|r| std::ptr::eq(r, row)).and_then(|i| u32::try_from(i).ok()));
            if let Some(index) = index {
                rows.entry((section_id, index)).or_insert((section, row));
            }
        }
        rows
    }
    fn baseline_provider(&self, scope: Scope, name: &str) -> Option<Scope> {
        defaults::cell(scope, name)?;
        let key = |scope| Key { scope, name: name.to_ascii_lowercase() };
        if scope != Scope::Document && self.cells.contains_key(&key(Scope::Document)) {
            return Some(Scope::Document);
        }
        if !matches!(scope, Scope::Document | Scope::Style(0)) && self.cells.contains_key(&key(Scope::Style(0))) {
            return Some(Scope::Style(0));
        }
        None
    }
    /// Enumerate effective cells plus known scoped baseline slots, without storing the
    /// baseline hundreds of times in the per-instance index.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
    pub fn resolved_named_cells(&self, scope: Scope) -> Vec<NamedCell<'a>> {
        let mut cells: BTreeMap<String, NamedCell<'a>> = self.named_cells(scope).iter().map(|n| (n.name.to_ascii_lowercase(), n.clone())).collect();
        if self.sheets.contains_key(&scope) {
            for default in defaults::CELLS.iter().filter(|c| c.kind == defaults::kind(scope)) {
                let name = &default.cell.name;
                if let Some(cell) = self.baseline_cell(scope, name) {
                    cells.entry(name.to_ascii_lowercase()).or_insert_with(|| NamedCell { name: name.clone(), cell, section: None, row: None });
                }
            }
        }
        'reserved: for ((section_id, row_index), (section, row)) in self.formatting_rows(scope) {
            if row_index == 0 {
                continue;
            }
            let Some(one) = row_index.checked_add(1) else { continue };
            for default in defaults::CELLS.iter().filter(|c| c.kind == defaults::kind(scope) && reserved_cell(&c.cell)) {
                if default.cell.attributes.get("ComSection").and_then(|n| n.parse::<u16>().ok()) != Some(section_id) {
                    continue;
                }
                let base = &default.cell.name;
                let name = if base.to_ascii_lowercase().contains("._unused") {
                    format!("{base}[{one}]")
                } else {
                    format!("{}{one}", base.strip_suffix('1').unwrap_or(base))
                };
                if cells.len() >= MAX_CELLS {
                    break 'reserved;
                }
                if self.defaults_allowed(scope, &name) {
                    cells.entry(name.to_ascii_lowercase()).or_insert_with(|| NamedCell {
                        name,
                        cell: &default.cell,
                        section: Some(section),
                        row: Some(row),
                    });
                }
            }
        }
        for index in self.geometry_indices(scope) {
            if cells.len() >= MAX_CELLS {
                break;
            }
            let Some(one) = index.checked_add(1) else { continue };
            let name = format!("Geometry{one}.Path");
            cells.entry(name.to_ascii_lowercase()).or_insert_with(|| NamedCell { name, cell: &PATH_CELL, section: None, row: None });
        }
        cells.into_values().collect()
    }
    /// Recorded baseline SRC indices and numeric COM units, independent of cell values.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.cellssrc
    pub fn baseline_metadata(scope: Scope, name: &str) -> Option<(u16, u16, u16, u16)> {
        let reserved = reserved_prototype(name);
        let c = defaults::cell(scope, name)
            .or_else(|| reserved.as_ref().and_then(|(prototype, _)| defaults::cell(scope, prototype)).filter(|c| reserved_cell(c)))?;
        let n = |key| c.attributes.get(key)?.parse().ok();
        Some((n("ComSection")?, reserved.and_then(|(_, row)| u16::try_from(row).ok()).unwrap_or(n("ComRow")?), n("ComCell")?, n("ComUnits")?))
    }
    pub fn dump_defaults(scope: Scope) -> Vec<NamedCell<'static>> {
        defaults::CELLS
            .iter()
            .filter(|c| c.kind == defaults::kind(scope))
            .map(|c| NamedCell { name: c.cell.name.clone(), cell: &c.cell, section: None, row: None })
            .collect()
    }
    /// Formula provider text, with source bytes left unchanged.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub fn effective_formula(&self, scope: Scope, name: &str) -> Option<String> {
        let key = self.canonical_key(scope, name)?;
        let origin = self.formula_origin(scope, &key.name).or_else(|| self.baseline_provider(scope, &key.name)).unwrap_or(scope);
        self.overrides.get(&key).or_else(|| self.overrides.get(&Key { scope: origin, name: key.name.clone() })).cloned().or_else(|| {
            let cell = self.cell(scope, name)?;
            cell.formula.clone().or_else(|| {
                (cell.value.as_deref() == Some("Themed") && cell.units.as_deref() != Some("STR") && !string_cells::is_string(&key.name, cell))
                    .then(|| "THEMEVAL()".into())
            })
        })
    }
    /// A passive SETF Action reads zero, without evaluating its arguments or side effect.
    /// Original M5.10 and M5.13e nested-action witnesses cover stored zero and one.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/action-cell-actions-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/setf-function
    pub(crate) fn passive_setf_action(&self, scope: Scope, name: &str) -> bool {
        name.get(..8).is_some_and(|prefix| prefix.eq_ignore_ascii_case("Actions."))
            && name.rsplit_once('.').is_some_and(|(_, column)| column.eq_ignore_ascii_case("Action"))
            && self.effective_formula(scope, name).is_some_and(|formula| {
                use misterdiagram_sheet::{Expr, parser::BinaryOp};
                fn root_setf(expr: &Expr) -> bool {
                    matches!(expr, Expr::Call { name, .. } if name.eq_ignore_ascii_case("SETF"))
                }
                misterdiagram_sheet::parse(&formula).is_ok_and(|expr| {
                    root_setf(&expr)
                        || match &expr {
                            Expr::Call { name, args } if name.eq_ignore_ascii_case("GUARD") && args.len() == 1 => args.first().is_some_and(root_setf),
                            Expr::Binary(BinaryOp::Add, left, right) if matches!(left.as_ref(), Expr::Number(1., None)) => root_setf(right),
                            _ => false,
                        }
                })
            })
    }
    pub fn display_reference(&self, scope: Scope, name: &str, reference: &str) -> Result<String, Error> {
        let origin = self.formula_origin(scope, name).unwrap_or(scope);
        self.remap_master_reference(scope, origin, reference)
    }
    /// Presence is independent of evaluation success. XML constants omit F but still
    /// have a COM formula; the explicit No Formula sentinel does not.
    /// Witnesses: BPMN_U master8 User.msvSDTargetIntersection and SOCIAL_DISTANCING_M
    /// page1 shape1835 Prop._VisDM_Whole_Name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/formulaexists-function
    pub fn formula_exists(&mut self, scope: Scope, reference: &str) -> Result<bool, Error> {
        self.add_dependency(scope, reference);
        let (scope, name) = match self.reference_target(scope, reference) {
            Ok(target) => target,
            Err(Error::Ref) => return Ok(false),
            Err(e) => return Err(e),
        };
        let Some(cell) = self.cell(scope, &name) else { return Ok(false) };
        if let Some(formula) = self.effective_formula(scope, &name) {
            return Ok(!formula.is_empty() && formula != "No Formula" && formula != "Inh");
        }
        if cell.attributes.contains_key("ComSection") || cell.attributes.contains_key("VirtualPath") {
            return Ok(false);
        }
        Ok(cell.value.is_some())
    }
    pub fn clear_cache(&mut self) {
        self.values.clear();
    }
    pub fn evaluate(&mut self, scope: Scope, name: &str) -> Value {
        if self.internal_cache_units {
            self.add_dependency_key(Key { scope, name: name.to_ascii_lowercase() });
        }
        if self.eval_depth == 0 {
            self.work = 0;
            self.eval_work = 0;
        }
        self.evaluate_inner(scope, name)
    }
    /// Evaluate against the original overlay snapshot. SETF runs normally within the
    /// query, then its changes are rolled back; caches touched by a write are discarded.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/setf-function
    pub fn evaluate_snapshot(&mut self, scope: Scope, name: &str) -> Value {
        if self.snapshot.is_some() || self.eval_depth != 0 {
            return Value::Error(Error::Value);
        }
        let before = (self.override_bytes, self.assignments, self.revision, self.graph_limited);
        let dirty = self.dirty.clone();
        self.snapshot = Some(Snapshot::default());
        let result = self.evaluate(scope, name);
        if let Some(snapshot) = self.snapshot.take()
            && !snapshot.formulas.is_empty()
        {
            for (key, old) in snapshot.formulas {
                if let Some(formula) = old {
                    self.overrides.insert(key, formula);
                } else {
                    self.overrides.remove(&key);
                }
            }
            for (caller, target) in snapshot.edges {
                if let Some(edges) = self.dependencies.get_mut(&caller) {
                    edges.remove(&target);
                }
                if let Some(users) = self.dependents.get_mut(&target) {
                    users.remove(&caller);
                }
                self.dependency_count = self.dependency_count.saturating_sub(1);
                self.dependency_bytes = self.dependency_bytes.saturating_sub(caller.name.len().saturating_add(target.name.len()));
            }
            self.dependencies.retain(|_, edges| !edges.is_empty());
            self.dependents.retain(|_, edges| !edges.is_empty());
            self.values.clear();
            self.dirty = dirty;
            (self.override_bytes, self.assignments, self.revision, self.graph_limited) = before;
        }
        result
    }
    /// Whether the evaluated dependency closure intersects changed serialized inputs.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
    pub(crate) fn depends_on_changed(&self, scope: Scope, name: &str, changed: &BTreeSet<(Scope, String)>, contexts: &BTreeSet<Scope>) -> bool {
        if self.graph_limited {
            return true;
        }
        let mut pending = vec![Key { scope, name: name.to_ascii_lowercase() }];
        let mut seen = BTreeSet::new();
        while let Some(key) = pending.pop() {
            if contexts.contains(&key.scope) || changed.contains(&(key.scope, key.name.clone())) {
                return true;
            }
            if !seen.insert(key.clone()) {
                continue;
            }
            if seen.len() > 100_000 {
                return true;
            }
            if let Some(edges) = self.dependencies.get(&key) {
                pending.extend(edges.iter().cloned());
            }
        }
        false
    }

    fn mark_context_dependent(&self) {
        self.context_epoch.set(self.context_epoch.get().saturating_add(1));
    }
    fn evaluate_inner(&mut self, scope: Scope, name: &str) -> Value {
        if self.graph_limited || name.len() > 65536 || self.active.len() >= 128 || self.work >= 100_000 {
            self.mark_context_dependent();
            return Value::Error(Error::Limit);
        }
        self.work += 1;
        if let Some(e) = self.inheritance_error(scope, name) {
            return Value::Error(e);
        }
        let Some(key) = self.canonical_key(scope, name) else { return Value::Error(Error::Ref) };
        if let Some(cached) = self.values.get(&key)
            && (cached.shared || self.arguments.is_empty())
        {
            if !cached.shared {
                self.mark_context_dependent();
            }
            return cached.value.clone();
        }
        // Action formulas run on user invocation, not passive cell recalculation.
        // ANNOT_U master26 uses a toggle referencing its own previous Action result.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/action-cell-actions-section
        // https://learn.microsoft.com/en-us/office/client-developer/visio/eventdblclick-cell-events-section
        if key.name.starts_with("actions.") && key.name.ends_with(".action")
            || matches!(key.name.as_str(), "eventdblclick" | "eventdrop" | "eventmultidrop" | "eventxfmod" | "eventthetext")
        {
            if self.passive_setf_action(scope, &key.name) {
                return Value::Number(0.);
            }
            return self
                .local_inh_cell(scope, &key.name)
                .or_else(|| self.cell(scope, &key.name))
                .map_or(Value::Error(Error::Ref), |c| self.cached_named_cell(scope, &key.name, c));
        }
        if !self.active.insert(key.clone()) {
            let value = self.prove_clamped_cycle(&key);
            if value == Value::Error(Error::Cycle) {
                self.mark_context_dependent();
            }
            return value;
        }
        let revision = self.revision;
        let context_epoch = self.context_epoch.get();
        self.stack.push(key.clone());
        let value = if let Some(cell) = self.cell(scope, &key.name) {
            let cached = self.cached_named_cell(scope, &key.name, self.local_inh_cell(scope, &key.name).unwrap_or(cell));
            let origin = if self.overrides.contains_key(&key) {
                scope
            } else {
                self.formula_origin(scope, &key.name).or_else(|| self.baseline_provider(scope, &key.name)).unwrap_or(scope)
            };
            let provider = Key { scope: origin, name: key.name.clone() };
            if origin != scope {
                self.add_dependency_key(provider.clone());
            }
            let formula = self.overrides.get(&key).or_else(|| self.overrides.get(&provider)).cloned().or_else(|| cell.formula.clone());
            match formula.as_deref() {
                None if cell.value.as_deref() == Some("Themed")
                    && cell.units.as_deref() != Some("STR")
                    && !string_cells::is_string(&key.name, cell) =>
                {
                    self.compute_formula(scope, origin, cached, "THEMEVAL()")
                }
                None | Some("No Formula") => self.cached_named_cell(scope, &key.name, cell),
                Some("Inh") => Value::Error(Error::Ref),
                Some(formula) => self.compute_formula(scope, origin, cached, formula),
            }
        } else {
            Value::Error(Error::Ref)
        };
        let value = self.cell(scope, &key.name).map_or(value.clone(), |cell| project_cell_point(cell, value));
        let value = self.coerce_numeric_cell(scope, &key.name, value);
        self.stack.pop();
        self.active.remove(&key);
        let value = if self.graph_limited { Value::Error(Error::Limit) } else { value };
        // ARG reads and transient depth/work refusals taint enclosing evaluations,
        // including IFERROR fallbacks. Preserve their historical outside-frame cache,
        // but share only context-independent entries with EVALCELL frames.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/evalcell-function
        // https://learn.microsoft.com/en-us/office/client-developer/visio/arg-function
        let shared = context_epoch != u64::MAX && context_epoch == self.context_epoch.get();
        if (shared || self.arguments.is_empty()) && revision == self.revision {
            self.dirty.remove(&key);
            self.values.insert(key, CachedValue { value: value.clone(), shared });
        }
        value
    }
    /// Assign a parsed formula to an in-memory overlay; source parts remain immutable.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/setf-function
    pub fn set_formula(&mut self, scope: Scope, reference: &str, formula: &str) -> Result<(), Error> {
        if formula.len() > 65536 || self.assignments >= 10000 {
            return Err(Error::Limit);
        }
        crate::evaluate::validate_formula(formula)?;
        let (scope, name) = self.reference_target(scope, reference)?;
        let key = self.canonical_key(scope, &name).ok_or(Error::Ref)?;
        let old = self.overrides.get(&key).map_or(0, String::len);
        let bytes = self.override_bytes.saturating_sub(old).saturating_add(formula.len());
        if bytes > 64 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        if let Some(snapshot) = self.snapshot.as_mut() {
            snapshot.formulas.entry(key.clone()).or_insert_with(|| self.overrides.get(&key).cloned());
        }
        self.override_bytes = bytes;
        self.overrides.insert(key.clone(), formula.into());
        self.assignments += 1;
        self.revision = self.revision.checked_add(1).ok_or(Error::Limit)?;
        self.invalidate(&key);
        Ok(())
    }
    fn invalidate(&mut self, key: &Key) {
        let mut pending = vec![key.clone()];
        let mut visited = BTreeSet::new();
        while let Some(key) = pending.pop() {
            if !visited.insert(key.clone()) {
                continue;
            }
            self.values.remove(&key);
            self.dirty.insert(key.clone());
            if let Some(users) = self.dependents.get(&key) {
                pending.extend(users.iter().filter(|k| !visited.contains(*k)).cloned());
            }
        }
    }
    /// Recalculate the dirty dependency closure. DFS postorder visits dependencies first;
    /// normal evaluation handles cycles as values and discovers changed formula edges.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/dependson-function
    pub fn recalculate_dirty(&mut self) -> Vec<(Scope, String, Value)> {
        let mut order = Vec::new();
        let mut visited = BTreeSet::new();
        for root in &self.dirty {
            let mut stack = vec![(root.clone(), false)];
            while let Some((key, finish)) = stack.pop() {
                if finish {
                    order.push(key);
                    continue;
                }
                if !visited.insert(key.clone()) {
                    continue;
                }
                stack.push((key.clone(), true));
                if let Some(edges) = self.dependencies.get(&key) {
                    stack.extend(edges.iter().filter(|k| self.dirty.contains(*k) && !visited.contains(*k)).map(|k| (k.clone(), false)));
                }
            }
        }
        order
            .into_iter()
            .map(|key| {
                let value = self.evaluate(key.scope, &key.name);
                (key.scope, key.name, value)
            })
            .collect()
    }
    pub fn dirty_count(&self) -> usize {
        self.dirty.len()
    }
    fn canonical_key(&self, scope: Scope, name: &str) -> Option<Key> {
        if name.len() > 65536 {
            return None;
        }
        indexed_key(&self.cells, scope, name).or_else(|| self.baseline_cell(scope, name).map(|_| Key { scope, name: name.to_ascii_lowercase() }))
    }
    /// Custom arguments live only during invocation; argument-free dependencies share the cell cache.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/evalcell-function
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/arg-function
    pub fn eval_cell(&mut self, scope: Scope, reference: &str, args: &[(String, Value)]) -> Value {
        if self.eval_depth == 0 {
            self.work = 0;
            self.eval_work = 0;
        }
        if args.len() > 256 || self.arguments.len() >= 64 || self.work >= 100000 {
            self.mark_context_dependent();
            return Value::Error(Error::Limit);
        }
        self.work += 1;
        self.add_dependency(scope, reference);
        let (target, name) = match self.reference_target(scope, reference) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        };
        let Some(key) = self.canonical_key(target, &name) else { return Value::Error(Error::Ref) };
        let Some(cell) = self.cell(target, &name) else { return Value::Error(Error::Ref) };
        let origin = if self.overrides.contains_key(&key) {
            target
        } else {
            self.formula_origin(target, &key.name).or_else(|| self.baseline_provider(target, &key.name)).unwrap_or(target)
        };
        let provider = Key { scope: origin, name: key.name.clone() };
        if origin != target {
            self.add_dependency_key(provider.clone());
        }
        let formula = self.overrides.get(&key).or_else(|| self.overrides.get(&provider)).cloned().or_else(|| cell.formula.clone());
        let cached = self.cached_named_cell(target, &key.name, self.local_inh_cell(target, &key.name).unwrap_or(cell));
        let mut bytes = 0usize;
        for (name, value) in args {
            if name.len() > 65536 {
                self.mark_context_dependent();
                return Value::Error(Error::Limit);
            }
            bytes = bytes.saturating_add(name.len()).saturating_add(match value {
                Value::Text(s) => s.len(),
                _ => 64,
            });
        }
        if self.argument_bytes.saturating_add(bytes) > 8 * 1024 * 1024 {
            self.mark_context_dependent();
            return Value::Error(Error::Limit);
        }
        self.argument_bytes += bytes;
        self.arguments.push(args.iter().map(|(n, v)| (n.to_ascii_lowercase(), v.clone())).collect());
        let result = match formula.as_deref() {
            None | Some("No Formula") => self.cached_named_cell(target, &key.name, cell),
            Some(f) => self.compute_formula(target, origin, cached, f),
        };
        self.arguments.pop();
        self.argument_bytes -= bytes;
        self.coerce_numeric_cell(target, &key.name, result)
    }
    /// Apply the cell's declared string type before interpreting its cached spelling.
    /// Shape Data Value remains a variant governed by Type and the stored unit.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/type-cell-shape-data-section
    pub(crate) fn cached_named_cell(&self, scope: Scope, name: &str, cell: &Cell) -> Value {
        if string_cells::is_string(name, cell) {
            // Preserve the observed COM baseline's synthetic empty-string convention.
            if self.internal_cache_units && defaults::cell(scope, name).is_some_and(|c| std::ptr::eq(c, cell)) {
                return self.cached_cell(cell);
            }
            return misterdiagram_sheet::units::cached(cell.value.as_deref(), Some("STR"), cell.error.as_deref());
        }
        self.cached_cell(cell)
    }
    /// Numeric cell types coerce numeric text returned by INDEX and other string functions.
    /// The scoped baseline's COM type supplies classification; User/Property variant cells remain variants.
    /// Scratch X/Y cells are drawing-unit numbers; Scratch A-D remain untyped variants.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/quickstylefillcolor-cell-quick-style-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/about-operators
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/scratch-section
    fn coerce_numeric_cell(&self, scope: Scope, name: &str, value: Value) -> Value {
        let scratch_number = name.get(..9).is_some_and(|prefix| prefix.eq_ignore_ascii_case("Scratch.X") || prefix.eq_ignore_ascii_case("Scratch.Y"))
            && name
                .get(9..)
                .is_some_and(|row| row.len() <= 10 && row.bytes().all(|b| b.is_ascii_digit()) && row.parse::<u32>().is_ok_and(|row| row > 0));
        let baseline_number = defaults::cell(scope, name)
            .is_some_and(|default| default.units.as_deref() != Some("STR") && default.attributes.get("ComUnits").is_some_and(|u| u != "231"));
        if let Value::Text(text) = &value
            && (scratch_number || baseline_number)
        {
            return match misterdiagram_sheet::units::numeric_text(text) {
                Ok(number) => Value::number(number),
                Err(Error::Unsupported(_)) => Value::Error(Error::Value),
                Err(error) => Value::Error(error),
            };
        }
        value
    }
    /// A cached V belongs to the entire cell, not an inner host-function invocation.
    /// Frozen host formulas therefore retain that final value until their host is implemented.
    /// Ordinary unsupported functions never take this path. References still register dependencies.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    fn compute_formula(&mut self, scope: Scope, origin: Scope, cached: Value, formula: &str) -> Value {
        let expr = match misterdiagram_sheet::parse(formula) {
            Ok(expr) => expr,
            Err(e) => return Value::Error(Error::Parse(e.to_string())),
        };
        match hook_references_with_host(&expr, self.defer_text_metrics, self.glue_host.as_deref()) {
            Ok(Some(references)) => {
                for reference in references {
                    if let Ok(reference) = self.remap_master_reference(scope, origin, reference) {
                        self.add_dependency(scope, &reference);
                    }
                }
                cached
            }
            Ok(None) => {
                let mut context = BoundContext { evaluator: self, scope, origin, cached };
                misterdiagram_sheet::evaluate(&expr, &mut context)
            }
            Err(e) => Value::Error(e),
        }
    }
    fn add_dependency(&mut self, scope: Scope, reference: &str) {
        if let Ok((scope, name)) = self.reference_target(scope, reference)
            && let Some(target) = self.canonical_key(scope, &name)
        {
            self.add_dependency_key(target);
        }
    }
    fn add_dependency_key(&mut self, target: Key) {
        if let Some(caller) = self.stack.last().cloned() {
            let edges = self.dependencies.entry(caller.clone()).or_default();
            if !edges.contains(&target) {
                let bytes = caller.name.len().saturating_add(target.name.len());
                if self.dependency_count >= 2_000_000 || self.dependency_bytes.saturating_add(bytes) > 256 * 1024 * 1024 {
                    self.graph_limited = true;
                    return;
                }
                edges.insert(target.clone());
                if let Some(snapshot) = self.snapshot.as_mut() {
                    snapshot.edges.push((caller.clone(), target.clone()));
                }
                self.dependents.entry(target).or_default().insert(caller);
                self.dependency_count += 1;
                self.dependency_bytes += bytes;
            }
        }
    }
    /// The V attribute is already in internal units. Character font names resolve to FaceNames indices.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/font-cell-character-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/360407c5-34d8-417d-b08d-4044fd858a5e
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/c5e4da4b-c1d0-4cdd-a418-04fccbc098ae
    pub fn cached_cell(&self, cell: &Cell) -> Value {
        if self.internal_cache_units
            && cell.error.is_none()
            && matches!(cell.name.as_str(), "BulletStr" | "Comment" | "ShapeKeywords" | "Copyright" | "HelpTopic")
        {
            // Synthetic COM defaults store numeric ResultIU=0 for an empty string.
            // An actual source/authored string "0" must retain its spelling.
            if cell.value.as_deref() == Some("0") && cell.formula.is_none() && defaults::CELLS.iter().any(|d| std::ptr::eq(&d.cell, cell)) {
                return Value::Text(String::new());
            }
            return Value::Text(cell.value.clone().unwrap_or_default());
        }
        if matches!(cell.name.as_str(), "LangID" | "DocLangID") && cell.error.is_none() {
            return match misterdiagram_sheet::language::lcid(cell.value.as_deref().unwrap_or("0")) {
                Ok(id) => Value::Number(f64::from(id)),
                Err(e) => Value::Error(e),
            };
        }
        if cell.value.as_ref().is_some_and(|s| s.len() > 65536) {
            return Value::Error(Error::Limit);
        }
        if cell.units.as_deref() != Some("STR")
            && cell.error.is_none()
            && let Some(rgb) = cell.value.as_deref().and_then(parse_color)
            && let Some(index) = self.document.colors.iter().find_map(|(i, v)| (parse_color(v) == Some(rgb)).then_some(*i))
        {
            return Value::Number(f64::from(index) + if cell.name == "TextBkgnd" { 1. } else { 0. });
        }
        if cell.units.as_deref() != Some("STR")
            && cell.error.is_none()
            && let Some(rgb) = cell.value.as_deref().and_then(parse_color)
        {
            // Reading a speculative formula cache must not allocate a COM palette ID.
            // RGB-bearing values carry their colour independently of an optional index.
            let index = self
                .palette
                .iter()
                .find_map(|(i, c)| (*c == rgb).then_some(*i))
                .or_else(|| (0..24).find(|i| misterdiagram_sheet::functions::default_color(*i) == Some(rgb)))
                .or_else(|| self.palette.last_key_value().map_or(Some(24), |(i, _)| i.checked_add(1)))
                .unwrap_or(0);
            return if cell.name == "TextBkgnd" {
                index.checked_add(1).map_or(Value::Error(Error::Limit), |n| Value::Number(f64::from(n)))
            } else {
                Value::Color { index, rgb, hsl: None }
            };
        }
        if cell.name == "Font"
            && cell.error.is_none()
            && let Some(name) = cell.value.as_deref()
            && let Some(index) = self.fonts.iter().find_map(|(i, n)| n.eq_ignore_ascii_case(name).then_some(*i))
        {
            return Value::Number(f64::from(index));
        }
        misterdiagram_sheet::units::cached(cell.value.as_deref(), cell.units.as_deref(), cell.error.as_deref())
    }
    /// Resolve local, cross-shape, document, named page/master and style-qualified references.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
    pub fn resolve(&mut self, scope: Scope, reference: &str) -> Value {
        self.add_dependency(scope, reference);
        match self.reference_target(scope, reference) {
            Ok((scope, name)) => {
                let value = self.evaluate_inner(scope, &name);
                // E retains the current error while V is the last valid numeric value.
                // ARROWS_M master2 Scratch.A1 exposes V through numeric references.
                // https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
                if value.error().is_some()
                    && let Some(c) = self.cell(scope, &name).filter(|c| c.error.is_some())
                    && !string_cells::is_string(&name.to_ascii_lowercase(), c)
                {
                    let saved = misterdiagram_sheet::units::cached(c.value.as_deref(), c.units.as_deref(), None);
                    if matches!(saved, Value::Number(_)) {
                        return saved;
                    }
                }
                value
            }
            Err(e) => Value::Error(e),
        }
    }
    pub fn reference_target(&self, mut scope: Scope, reference: &str) -> Result<(Scope, String), Error> {
        if reference.len() > 65536 {
            return Err(Error::Limit);
        }
        let segments = split_reference(reference)?;
        let Some((name, prefixes)) = segments.split_last() else { return Err(Error::Ref) };
        for prefix in prefixes {
            let p = unquote(prefix);
            if p.eq_ignore_ascii_case("TheDoc") {
                scope = Scope::Document
            } else if p.eq_ignore_ascii_case("ThePage") {
                scope = page_scope(scope).ok_or(Error::Ref)?
            } else if let Some(name) = bracket_name(&p, "Pages") {
                let page = self
                    .document
                    .pages
                    .iter()
                    .find(|p| p.name_u.eq_ignore_ascii_case(&name) || p.name.eq_ignore_ascii_case(&name))
                    .ok_or(Error::Ref)?;
                scope = Scope::Page(page.id)
            } else if let Some(name) = bracket_name(&p, "Masters") {
                let master = self
                    .document
                    .masters
                    .iter()
                    .find(|p| p.name_u.eq_ignore_ascii_case(&name) || p.name.eq_ignore_ascii_case(&name))
                    .ok_or(Error::Ref)?;
                scope = Scope::Master(master.id)
            } else if p.eq_ignore_ascii_case("Styles") {
                scope = Scope::Style(u32::MAX)
            } else {
                scope = self.shape_scope(scope, &p).ok_or(Error::Ref)?
            }
        }
        Ok((scope, unquote(name)))
    }
    fn shape_scope(&self, current: Scope, name: &str) -> Option<Scope> {
        let id = name.split_once('.').filter(|(prefix, _)| prefix.eq_ignore_ascii_case("Sheet")).and_then(|(_, s)| s.parse::<u32>().ok());
        match page_scope(current) {
            Some(Scope::Page(page)) => self
                .shapes
                .iter()
                .find(|(scope, shape)| {
                    matches!(scope,Scope::PageShape{page:p,..}if *p==page)
                        && (id == Some(shape.id) || shape.name.eq_ignore_ascii_case(name) || shape.name_u.eq_ignore_ascii_case(name))
                })
                .map(|(s, _)| *s),
            Some(Scope::Master(master)) => self
                .shapes
                .iter()
                .find(|(scope, shape)| {
                    matches!(scope,Scope::MasterShape{master:m,..}if *m==master)
                        && (id == Some(shape.id) || shape.name.eq_ignore_ascii_case(name) || shape.name_u.eq_ignore_ascii_case(name))
                })
                .map(|(s, _)| *s),
            _ => {
                if matches!(current, Scope::Style(_)) {
                    self.document
                        .styles
                        .iter()
                        .find(|s| id == Some(s.id) || s.name.eq_ignore_ascii_case(name) || s.name_u.eq_ignore_ascii_case(name))
                        .map(|s| Scope::Style(s.id))
                } else {
                    None
                }
            }
        }
    }
}
static PATH_CELL: std::sync::LazyLock<Cell> = std::sync::LazyLock::new(|| Cell {
    name: "Path".into(),
    value: Some("0".into()),
    units: Some("NUM".into()),
    attributes: misterdiagram_model::Attributes::from([("VirtualPath".into(), "1".into())]),
    ..Default::default()
});
fn path_index(name: &str) -> Option<u32> {
    if name.len() > 64 {
        return None;
    }
    let lower = name.to_ascii_lowercase();
    lower.strip_prefix("geometry")?.strip_suffix(".path")?.parse::<u32>().ok()?.checked_sub(1).filter(|i| *i < 230)
}
fn reserved_cell(cell: &Cell) -> bool {
    let lower = cell.name.to_ascii_lowercase();
    if !(lower.starts_with("char.") || lower.starts_with("para.")) {
        return false;
    }
    lower.contains("._unused") || (cell.formula.is_none() && lower.ends_with('1'))
}
fn reserved_prototype(name: &str) -> Option<(String, u32)> {
    if name.len() > 65536 {
        return None;
    }
    let lower = name.to_ascii_lowercase();
    let (prefix, tail) = lower.split_once('.')?;
    if !matches!(prefix, "char" | "para") {
        return None;
    }
    if let Some((base, index)) = tail.split_once('[') {
        if !base.starts_with("_unused") {
            return None;
        }
        let index = index.strip_suffix(']')?.parse::<u32>().ok()?.checked_sub(1)?;
        return Some((format!("{prefix}.{base}"), index));
    }
    let start = tail.find(|c: char| c.is_ascii_digit())?;
    let letters = tail.get(..start)?;
    if letters.is_empty() || !letters.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let index = tail.get(start..)?.parse::<u32>().ok()?.checked_sub(1)?;
    Some((format!("{prefix}.{letters}1"), index))
}
fn page_scope(scope: Scope) -> Option<Scope> {
    match scope {
        Scope::Page(p) | Scope::PageShape { page: p, .. } => Some(Scope::Page(p)),
        Scope::Master(m) | Scope::MasterShape { master: m, .. } => Some(Scope::Master(m)),
        _ => None,
    }
}
pub(crate) fn section_prefix(section: &Section) -> Result<String, Error> {
    Ok(match section.name.as_str() {
        "Geometry" => format!("Geometry{}", section.index.unwrap_or(0).checked_add(1).ok_or(Error::Limit)?),
        "Character" => "Char".into(),
        "Paragraph" => "Para".into(),
        "Field" => "Fields".into(),
        "Property" => "Prop".into(),
        "Control" => "Controls".into(),
        "Connection" => "Connections".into(),
        "Action" => "Actions".into(),
        "Layer" => "Layers".into(),
        "FillGradient" => "FillGradientStops".into(),
        "LineGradient" => "LineGradientStops".into(),
        other => other.into(),
    })
}
/// Convert XML zero-based formatting row indices and named rows to universal formula names.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
/// https://learn.microsoft.com/en-us/office/client-developer/visio/size-cell-character-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/value-cell-text-fields-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/format-cell-text-fields-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/value-cell-user-defined-cells-section
pub fn cell_name(section: &Section, row: &Row, position: usize, cell: &Cell) -> Result<String, Error> {
    let prefix = section_prefix(section)?;
    let index = row.index.unwrap_or(u32::try_from(position).map_err(|_| Error::Limit)?);
    let one = index.checked_add(1).ok_or(Error::Limit)?;
    Ok(match section.name.as_str() {
        "Geometry" => format!("{prefix}.{}{index}", cell.name),
        "Connection" if row.name.is_none() => connection_name(index, &cell.name)?,
        "Scratch" | "Tabs" if row.name.is_none() => format!("{prefix}.{}{one}", cell.name),
        "Character" | "Paragraph" | "Field" | "Layer" | "FillGradient" | "LineGradient" => {
            if index == 0 {
                format!("{prefix}.{}", cell.name)
            } else {
                format!("{prefix}.{}[{one}]", cell.name)
            }
        }
        _ => {
            let row_name = row.name.clone().unwrap_or_else(|| format!("Row_{one}"));
            let omit = (matches!(section.name.as_str(), "Actions" | "Action") && cell.name == "Menu")
                || (matches!(section.name.as_str(), "User" | "Property") && cell.name == "Value")
                || (matches!(section.name.as_str(), "Control" | "Controls") && cell.name == "X");
            if omit { format!("{prefix}.{row_name}") } else { format!("{prefix}.{row_name}.{}", cell.name) }
        }
    })
}
/// Universal names observed in generated/master/basic-rect.sheet.tsv.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-row-connection-points-section
pub(crate) fn connection_name(index: u32, column: &str) -> Result<String, Error> {
    let one = index.checked_add(1).ok_or(Error::Limit)?;
    Ok(if matches!(column, "X" | "Y" | "F" | "G" | "H" | "I" | "J" | "K" | "L") {
        format!("Connections.{column}{one}")
    } else if index == 0 {
        format!("Connections.{column}")
    } else {
        format!("Connections.{column}[{one}]")
    })
}
/// Formatting [1], explicit User/Prop.Value and Controls.X are alternate
/// spellings of an existing cell, not additional retained cell entries.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
fn indexed_key(cells: &BTreeMap<Key, &Cell>, scope: Scope, name: &str) -> Option<Key> {
    let mut key = Key { scope, name: name.to_ascii_lowercase() };
    if cells.contains_key(&key) {
        return Some(key);
    }
    let (section, _) = key.name.split_once('.')?;
    let section = section.to_owned();
    let formatting = matches!(section.as_str(), "char" | "para" | "fields" | "layers" | "fillgradientstops" | "linegradientstops");
    let base = if formatting {
        key.name.strip_suffix("[1]")
    } else if matches!(section.as_str(), "user" | "prop") {
        key.name.strip_suffix(".value")
    } else if section == "controls" {
        key.name.strip_suffix(".x")
    } else {
        None
    }?;
    key.name = base.to_owned();
    let cell = cells.get(&key)?;
    if formatting && key.name != format!("{section}.{}", cell.name.to_ascii_lowercase()) {
        return None;
    }
    if (matches!(section.as_str(), "user" | "prop") && cell.name != "Value") || (section == "controls" && cell.name != "X") {
        return None;
    }
    Some(key)
}
fn unquote(s: &str) -> String {
    let s = s.trim();
    if s.len() >= 2 && s.starts_with('\'') && s.ends_with('\'') { s.get(1..s.len() - 1).unwrap_or("").replace("''", "'") } else { s.into() }
}
fn bracket_name(s: &str, prefix: &str) -> Option<String> {
    let head = s.get(..prefix.len())?;
    if !head.eq_ignore_ascii_case(prefix) {
        return None;
    }
    Some(unquote(s.get(prefix.len()..)?.strip_prefix('[')?.strip_suffix(']')?))
}
fn split_reference(s: &str) -> Result<Vec<&str>, Error> {
    let mut bracket = 0usize;
    let mut quoted = false;
    let mut start = 0;
    let mut out = vec![];
    let mut chars = s.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        match c {
            '\'' => {
                if quoted && chars.peek().is_some_and(|(_, c)| *c == '\'') {
                    chars.next();
                } else {
                    quoted = !quoted
                }
            }
            '[' if !quoted => {
                bracket = bracket.checked_add(1).ok_or(Error::Limit)?;
                if bracket > 128 {
                    return Err(Error::Limit);
                }
            }
            ']' if !quoted => bracket = bracket.checked_sub(1).ok_or(Error::Ref)?,
            '!' if !quoted && bracket == 0 => {
                if out.len() >= 8 {
                    return Err(Error::Limit);
                }
                out.push(s.get(start..i).ok_or(Error::Ref)?);
                start = i + 1;
            }
            _ => {}
        }
    }
    if quoted || bracket != 0 {
        return Err(Error::Ref);
    }
    out.push(s.get(start..).ok_or(Error::Ref)?);
    if out.iter().any(|s| s.is_empty()) {
        return Err(Error::Ref);
    }
    Ok(out)
}
// Only parsed call nodes select the frozen host fallback. Quoted strings and reference names do not.
fn hook_references(expr: &misterdiagram_sheet::Expr, defer_text: bool) -> Result<Option<Vec<&str>>, Error> {
    hook_references_with_host(expr, defer_text, None)
}
fn hook_references_with_host<'e>(
    expr: &'e misterdiagram_sheet::Expr,
    defer_text: bool,
    host: Option<&dyn misterdiagram_sheet::HostHooks>,
) -> Result<Option<Vec<&'e str>>, Error> {
    use misterdiagram_sheet::Expr;
    let mut pending = vec![(expr, 0usize)];
    let mut references = Vec::new();
    let mut nodes = 0usize;
    let mut has_hook = false;
    let mut unsupported = None;
    while let Some((node, depth)) = pending.pop() {
        nodes += 1;
        if nodes > 8192 || depth > 128 {
            return Err(Error::Limit);
        }
        match node {
            Expr::Reference(r) => references.push(r.as_str()),
            Expr::Unary(_, e) | Expr::Qualified(e, _) => pending.push((e, depth + 1)),
            Expr::Binary(_, a, b) => {
                pending.push((a, depth + 1));
                pending.push((b, depth + 1));
            }
            Expr::Call { name, args } => {
                let hook = misterdiagram_sheet::is_hook(name) || defer_text && misterdiagram_sheet::is_text_metric(name);
                has_hook |= hook && !host.is_some_and(|host| host.supports(name));
                if !hook && !misterdiagram_sheet::functions::IMPLEMENTED.contains(&name.as_str()) {
                    unsupported.get_or_insert_with(|| name.clone());
                }
                pending.extend(args.iter().map(|arg| (arg, depth + 1)));
            }
            _ => {}
        }
    }
    if has_hook && let Some(name) = unsupported {
        return Err(Error::Unsupported(name));
    }
    Ok(has_hook.then_some(references))
}
struct BoundContext<'a, 'd> {
    origin: Scope,
    evaluator: &'a mut Evaluator<'d>,
    scope: Scope,
    cached: Value,
}
impl BoundContext<'_, '_> {
    fn reference(&self, reference: &str) -> Result<String, Error> {
        self.evaluator.remap_master_reference(self.scope, self.origin, reference)
    }
}
impl Context for BoundContext<'_, '_> {
    fn hook(&mut self, name: &str, args: &[Value]) -> Value {
        let cell = self.host_cell();
        let (page, shape) = match self.scope {
            Scope::PageShape { page, shape } => (Some(page), Some(shape)),
            _ => (None, None),
        };
        match self.evaluator.glue_host.as_deref_mut() {
            Some(host) => host.hook_at(page, shape, &cell, name, args, &self.cached),
            None => self.cached.clone(),
        }
    }

    fn host_cell(&self) -> String {
        self.evaluator.stack.last().map(|k| k.name.clone()).unwrap_or_default()
    }
    /// Page inheritance for the theme component sentinel.
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/1e7e9b7e-d116-41c0-9c53-5e57c26042a4
    fn theme_selector(&mut self, name: &str) -> Value {
        let value = self.resolve(name);
        if value.numeric() != Ok(65534.) {
            return value;
        }
        let page = match self.scope {
            Scope::PageShape { page, .. } => Scope::Page(page),
            Scope::MasterShape { master, .. } => Scope::Master(master),
            _ => Scope::Document,
        };
        self.evaluator.add_dependency_key(Key { scope: page, name: name.to_ascii_lowercase() });
        let value = self.evaluator.evaluate_inner(page, name);
        if value.numeric() == Ok(65534.) && page != Scope::Document {
            self.evaluator.add_dependency_key(Key { scope: Scope::Document, name: name.to_ascii_lowercase() });
            return self.evaluator.evaluate_inner(Scope::Document, name);
        }
        value
    }
    /// Arrow styles are theme-capable even though blank shape baselines are literal.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function
    fn theme_default(&mut self, name: &str) -> Value {
        let prototype = if name.starts_with("char.") { name.split('[').next().unwrap_or(name) } else { name };
        if !matches!(prototype, "beginarrow" | "endarrow" | "beginarrowsize" | "endarrowsize")
            && !defaults::cell(Scope::PageShape { page: 0, shape: 0 }, prototype).is_some_and(|c| c.formula.as_deref() == Some("THEMEVAL()"))
        {
            return Value::Error(Error::Value);
        }
        if self.scope != Scope::Style(0) && self.evaluator.cell(Scope::Style(0), name).is_some() {
            self.evaluator.add_dependency_key(Key { scope: Scope::Style(0), name: name.to_ascii_lowercase() });
            return self.evaluator.evaluate_inner(Scope::Style(0), name);
        }
        defaults::cell(self.scope, name).map(|cell| self.evaluator.cached_cell(cell)).unwrap_or(Value::Error(Error::Value))
    }
    fn theme(&mut self, name: &str, args: &[Value]) -> Value {
        match self.evaluator.theme_host {
            Some(host) => host.theme(name, args, self),
            None => misterdiagram_sheet::host::no_theme(name, args, self),
        }
    }
    fn enter_eval(&mut self) -> bool {
        if self.evaluator.eval_depth >= 128 || self.evaluator.eval_work >= 100000 {
            self.evaluator.mark_context_dependent();
            return false;
        }
        self.evaluator.eval_depth += 1;
        self.evaluator.eval_work += 1;
        true
    }
    fn leave_eval(&mut self) {
        self.evaluator.eval_depth = self.evaluator.eval_depth.saturating_sub(1);
    }
    fn resolve(&mut self, reference: &str) -> Value {
        match self.reference(reference) {
            Ok(r) => self.evaluator.resolve(self.scope, &r),
            Err(e) => Value::Error(e),
        }
    }
    fn reference_error(&mut self, reference: &str) -> Option<Error> {
        let reference = self.reference(reference).ok()?;
        let (scope, name) = self.evaluator.reference_target(self.scope, &reference).ok()?;
        self.evaluator.evaluate_inner(scope, &name).error()
    }
    fn list_sheet_reference(&mut self) -> Value {
        match self.evaluator.list_sheet_reference(self.scope) {
            Ok(reference) => Value::Text(reference),
            Err(error) => Value::Error(error),
        }
    }
    fn formula_exists(&mut self, reference: &str) -> Value {
        let reference = match self.reference(reference) {
            Ok(r) => r,
            Err(e) => return Value::Error(e),
        };
        match self.evaluator.formula_exists(self.scope, &reference) {
            Ok(exists) => Value::Number(if exists { 1. } else { 0. }),
            Err(e) => Value::Error(e),
        }
    }
    fn path_length(&mut self, reference: &str, segment: Option<usize>) -> Value {
        let reference = match self.reference(reference) {
            Ok(r) => r,
            Err(e) => return Value::Error(e),
        };
        Value::from(self.evaluator.path_length(self.scope, &reference, segment))
    }
    fn nearest_point_on_path(&mut self, reference: &str, x: f64, y: f64) -> Value {
        let reference = match self.reference(reference) {
            Ok(r) => r,
            Err(e) => return Value::Error(e),
        };
        Value::from(self.evaluator.path_nearest(self.scope, &reference, x, y))
    }
    fn path_point(&mut self, reference: &str, travel: f64, offset: f64, segment: Option<usize>) -> Value {
        let reference = match self.reference(reference) {
            Ok(r) => r,
            Err(e) => return Value::Error(e),
        };
        match self.evaluator.path_sample(self.scope, &reference, travel, segment) {
            Ok(((x, y), (dx, dy))) if offset.is_finite() => {
                let (x, y) = (x - dy * offset, y + dx * offset);
                if x.is_finite() && y.is_finite() { Value::Point(x, y) } else { Value::Error(Error::Num) }
            }
            Ok(_) => Value::Error(Error::Num),
            Err(e) => Value::Error(e),
        }
    }
    fn path_angle(&mut self, reference: &str, travel: f64, segment: Option<usize>) -> Value {
        let reference = match self.reference(reference) {
            Ok(r) => r,
            Err(e) => return Value::Error(e),
        };
        match self.evaluator.path_sample(self.scope, &reference, travel, segment) {
            Ok((_, (dx, dy))) => Value::number(dy.atan2(dx)),
            Err(e) => Value::Error(e),
        }
    }
    fn transform_angle(&mut self, angle: f64, source: &str, destination: &str, parent: bool) -> Value {
        let source = match self.reference(source) {
            Ok(r) => r,
            Err(e) => return Value::Error(e),
        };
        let destination = match self.reference(destination) {
            Ok(r) => r,
            Err(e) => return Value::Error(e),
        };
        self.evaluator.transform_angle(self.scope, angle, &source, &destination, parent).into()
    }
    fn transform_point(
        &mut self,
        point: (f64, f64),
        source: Option<&str>,
        source_parent: bool,
        destination: Option<&str>,
        destination_parent: bool,
    ) -> Value {
        let source = match source.map(|r| self.reference(r)).transpose() {
            Ok(r) => r,
            Err(e) => return Value::Error(e),
        };
        let destination = match destination.map(|r| self.reference(r)).transpose() {
            Ok(r) => r,
            Err(e) => return Value::Error(e),
        };
        match self.evaluator.transform_point(self.scope, point, source.as_deref(), source_parent, destination.as_deref(), destination_parent) {
            Ok((x, y)) => Value::Point(x, y),
            Err(e) => Value::Error(e),
        }
    }
    fn set_formula(&mut self, reference: &str, formula: &str) -> Result<(), Error> {
        self.evaluator.set_formula(self.scope, &self.reference(reference)?, formula)
    }
    fn eval_cell(&mut self, reference: &str, args: &[(String, Value)]) -> Value {
        match self.reference(reference) {
            Ok(r) => self.evaluator.eval_cell(self.scope, &r, args),
            Err(e) => Value::Error(e),
        }
    }
    fn argument(&self, name: &str) -> Option<Value> {
        self.evaluator.mark_context_dependent();
        self.evaluator.arguments.last()?.get(&name.to_ascii_lowercase()).cloned()
    }
    fn dependency(&mut self, reference: &str) {
        if self.evaluator.stack.last().is_some_and(|k| k.name.starts_with("event") || k.name.starts_with("actions.") && k.name.ends_with(".action")) {
            return;
        }
        if let Ok(r) = self.reference(reference) {
            self.evaluator.add_dependency(self.scope, &r)
        }
    }
    fn shape_text(&mut self, reference: &str) -> Value {
        let (scope, name) = match self.evaluator.reference_target(self.scope, &self.reference(reference).unwrap_or_default()) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        };
        if !name.eq_ignore_ascii_case("TheText") {
            return Value::Error(Error::Ref);
        }
        self.evaluator.text_string(scope, 0).map(Value::Text).unwrap_or_else(Value::Error)
    }
    fn shape_text_flags(&mut self, reference: &str, flags: u32) -> Value {
        let result = (|| {
            let reference = self.reference(reference)?;
            let (scope, name) = self.evaluator.reference_target(self.scope, &reference)?;
            if !name.eq_ignore_ascii_case("TheText") {
                return Err(Error::Ref);
            }
            self.evaluator.text_string(scope, flags)
        })();
        result.map(Value::Text).unwrap_or_else(Value::Error)
    }
    fn text_metric(&mut self, reference: &str, width: Option<f64>, height: bool) -> Value {
        let result = (|| {
            let reference = self.reference(reference)?;
            let (scope, name) = self.evaluator.reference_target(self.scope, &reference)?;
            if !name.eq_ignore_ascii_case("TheText") {
                return Err(Error::Ref);
            }
            let host = self.evaluator.text_host.ok_or_else(|| Error::Unsupported("text host".into()))?;
            let body = self.evaluator.text_body(scope)?;
            // TEXTWIDTH/TEXTHEIGHT return drawing inches; fonts and margins are physical points.
            // https://learn.microsoft.com/en-us/office/client-developer/visio/drawingscale-cell-page-properties-section
            // APPL_M master2 shape6 uses 1:100, APPL_U master25 shape5 uses 1:96.
            let page = match scope {
                Scope::PageShape { page, .. } => Scope::Page(page),
                Scope::MasterShape { master, .. } => Scope::Master(master),
                other => other,
            };
            let page_scale = self.evaluator.evaluate(page, "PageScale").numeric()?;
            let drawing_scale = self.evaluator.evaluate(page, "DrawingScale").numeric()?;
            if page_scale <= 0. || drawing_scale <= 0. {
                return Err(Error::Value);
            }
            let ratio = if self.evaluator.internal_cache_units { 1. } else { drawing_scale / page_scale };
            if !ratio.is_finite() || !(1e-9..=1e9).contains(&ratio) {
                return Err(Error::Limit);
            }
            let (w, h) = host.measure(&body, width.map(|v| v * 72. / ratio)).map_err(|_| Error::Value)?;
            Ok(Value::number((if height { h } else { w }) * ratio / 72.))
        })();
        result.unwrap_or_else(Value::Error)
    }
    fn cached(&self) -> Value {
        self.cached.clone()
    }
    fn unit(&self, reference: &str) -> Option<String> {
        let (scope, name) = self.evaluator.reference_target(self.scope, &self.reference(reference).unwrap_or_default()).ok()?;
        // F="Inh" inherits the expression, but a local U still controls display units.
        // CRIME_SCENE_INVESTIGATION_M page0 shape8 overrides Fields.Value to MM.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/cell-element-shapesheet_type-complextypevisio-xml
        self.evaluator.local_inh_cell(scope, &name).and_then(|c| c.units.clone()).or_else(|| self.evaluator.cell(scope, &name)?.units.clone())
    }
    fn color(&self, index: u32) -> Option<[u8; 3]> {
        self.evaluator.palette.get(&index).copied().or_else(|| misterdiagram_sheet::functions::default_color(index))
    }
    fn color_index(&mut self, rgb: [u8; 3]) -> Option<u32> {
        self.evaluator.intern_color(rgb)
    }
    fn font(&self, index: u32) -> Option<String> {
        self.evaluator.fonts.get(&index).cloned()
    }
    fn font_index(&self, name: &str) -> Option<u32> {
        self.evaluator.fonts.iter().find_map(|(index, value)| value.eq_ignore_ascii_case(name).then_some(*index))
    }
}
/// Destination coordinate cells select a point component; point-typed cells retain both.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pnt-function
fn project_cell_point(cell: &Cell, value: Value) -> Value {
    if cell.units.as_deref() == Some("PNT") {
        return value;
    }
    if let Value::Point(x, y) = value {
        match cell.name.as_str() {
            "X" | "A" | "PinX" | "LocPinX" | "BeginX" | "EndX" | "TxtPinX" | "TxtLocPinX" => Value::number(x),
            "Y" | "B" | "PinY" | "LocPinY" | "BeginY" | "EndY" | "TxtPinY" | "TxtLocPinY" => Value::number(y),
            _ => Value::Point(x, y),
        }
    } else {
        value
    }
}
/// Font IDs are host-specific. The observed blank document and Microsoft's example both
/// identify Calibri as 4. Other FaceNames retain collision-free document-local IDs; those
/// IDs are not yet a complete Windows font-profile observation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/font-function
fn font_ids(document: &Document) -> Result<BTreeMap<u32, String>, Error> {
    if document.face_names.len() > 65536 {
        return Err(Error::Limit);
    }
    let mut fonts = BTreeMap::new();
    if let Some(name) = document.face_names.values().find(|n| n.eq_ignore_ascii_case("Calibri")) {
        fonts.insert(4, name.clone());
    }
    for (preferred, name) in &document.face_names {
        if name.eq_ignore_ascii_case("Calibri") {
            continue;
        }
        let mut id = *preferred;
        while fonts.contains_key(&id) {
            id = id.checked_add(1).ok_or(Error::Limit)?;
        }
        fonts.insert(id, name.clone());
    }
    Ok(fonts)
}
fn parse_color(s: &str) -> Option<[u8; 3]> {
    let s = s.strip_prefix('#')?;
    if s.len() != 6 {
        return None;
    }
    Some([u8::from_str_radix(s.get(..2)?, 16).ok()?, u8::from_str_radix(s.get(2..4)?, 16).ok()?, u8::from_str_radix(s.get(4..)?, 16).ok()?])
}
fn limited_text(element: &misterdiagram_model::XmlElement) -> Result<String, Error> {
    let mut out = String::new();
    let mut stack = vec![element.children.iter()];
    let mut nodes = 0usize;
    while let Some(iter) = stack.last_mut() {
        nodes += 1;
        if nodes > 1000000 {
            return Err(Error::Limit);
        }
        match iter.next() {
            Some(misterdiagram_model::XmlNode::Text(s)) => {
                if out.len().saturating_add(s.len()) > 65536 {
                    return Err(Error::Limit);
                }
                out.push_str(s)
            }
            Some(misterdiagram_model::XmlNode::Element(e)) => {
                if stack.len() >= 128 {
                    return Err(Error::Limit);
                }
                stack.push(e.children.iter())
            }
            None => {
                stack.pop();
            }
        }
    }
    Ok(out)
}
fn validate_formula(formula: &str) -> Result<(), Error> {
    misterdiagram_sheet::parse(formula).map(|_| ()).map_err(|e| Error::Parse(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn cell(name: &str, value: &str, formula: Option<&str>) -> Cell {
        Cell { name: name.into(), value: Some(value.into()), formula: formula.map(str::to_string), ..Default::default() }
    }
    fn sheet(cells: Vec<Cell>) -> Sheet {
        Sheet { cells, ..Default::default() }
    }
    fn fixture() -> Document {
        use misterdiagram_model::{Master, Page, Style};
        Document {
            document_sheet: sheet(vec![cell("DocLangID", "1033", None)]),
            pages: vec![
                Page {
                    id: 7,
                    name: "A Page".into(),
                    name_u: "A Page".into(),
                    sheet: sheet(vec![cell("PageWidth", "10", None)]),
                    shapes: vec![
                        Shape {
                            id: 1,
                            name: "A".into(),
                            sheet: sheet(vec![
                                cell("Width", "2", None),
                                cell("Height", "0", Some("Sheet.2!Width+ThePage!PageWidth+TheDoc!DocLangID")),
                            ]),
                            ..Default::default()
                        },
                        Shape { id: 2, name: "B".into(), sheet: sheet(vec![cell("Width", "4", None)]), ..Default::default() },
                    ],
                    ..Default::default()
                },
                Page {
                    id: 9,
                    name: "Other".into(),
                    sheet: sheet(vec![cell("PageWidth", "20", None)]),
                    shapes: vec![Shape { id: 2, name: "Named".into(), sheet: sheet(vec![cell("Width", "6", None)]), ..Default::default() }],
                    ..Default::default()
                },
            ],
            masters: vec![Master {
                id: 5,
                name: "Gear".into(),
                shapes: vec![Shape { id: 3, name: "Shaft".into(), sheet: sheet(vec![cell("Width", "8", None)]), ..Default::default() }],
                ..Default::default()
            }],
            styles: vec![Style { id: 4, name: "Manager".into(), sheet: sheet(vec![cell("LineColor", "3", None)]), ..Default::default() }],
            colors: BTreeMap::from([(26, "#CC0000".into())]),
            face_names: BTreeMap::from([(0, "Example".into())]),
            ..Default::default()
        }
    }
    #[test]
    fn alternate_cell_spellings_share_one_retained_index_entry() {
        let mut doc = Document::default();
        for (section, row, column, value) in [
            ("Character", None, "Synthetic", "2"),
            ("Paragraph", None, "Synthetic", "3"),
            ("User", Some("Flag"), "Value", "4"),
            ("Property", Some("Flag"), "Value", "5"),
            ("Controls", Some("Handle"), "X", "6"),
        ] {
            doc.document_sheet.sections.push(Section {
                name: section.into(),
                rows: vec![Row { name: row.map(str::to_owned), index: Some(0), cells: vec![cell(column, value, None)], ..Default::default() }],
                ..Default::default()
            });
        }
        for section in doc.document_sheet.sections.iter_mut().take(2) {
            section.rows.push(Row { index: Some(1), cells: vec![cell("Synthetic", "7", None)], ..Default::default() });
        }
        let mut evaluator = Evaluator::new(&doc).unwrap();
        assert_eq!(evaluator.cells.len(), 7, "alternate spellings must not consume extra retained cell entries");
        for (name, expected) in
            [("Char.Synthetic[1]", 2.), ("Para.Synthetic[1]", 3.), ("User.Flag.Value", 4.), ("Prop.Flag.Value", 5.), ("Controls.Handle.X", 6.)]
        {
            assert_eq!(evaluator.evaluate(Scope::Document, name), Value::Number(expected));
            assert!(evaluator.cell(Scope::Document, name).is_some());
        }
        assert_eq!(evaluator.evaluate(Scope::Document, "Char.Synthetic[2]"), Value::Number(7.));
        for invalid in ["Char.Synthetic[2][1]", "Para.Synthetic[2][1]", "Char.Synthetic[1][1]", "Char.Synthetic[3]"] {
            assert_eq!(evaluator.evaluate(Scope::Document, invalid), Value::Error(Error::Ref), "{invalid}");
            assert!(evaluator.cell(Scope::Document, invalid).is_none(), "{invalid}");
        }
        assert_eq!(evaluator.evaluate(Scope::Document, "User.Missing.Value"), Value::Error(Error::Ref));
    }
    #[test]
    fn numeric_cell_coerces_index_text_but_user_variant_keeps_string() {
        // BELTORGCHART_M: QuickStyleLineColor references QuickStyleFillColor's INDEX result.
        let doc = Document {
            document_sheet: sheet(vec![
                cell("QuickStyleFillColor", "103", Some("INDEX(1,\"100;103;102\")")),
                cell("QuickStyleLineColor", "103", Some("GUARD(QuickStyleFillColor)")),
                cell("User.Variant", "103", Some("INDEX(1,\"100;103;102\")")),
            ]),
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(Scope::Document, "QuickStyleFillColor"), Value::Number(103.));
        assert_eq!(e.evaluate(Scope::Document, "QuickStyleLineColor"), Value::Number(103.));
        assert_eq!(e.evaluate(Scope::Document, "User.Variant"), Value::Text("103".into()));
    }
    #[test]
    fn scratch_coordinates_coerce_text_without_reusing_error_cache() {
        // ANNOT_M master12 shape6 has Scratch.X2 F='"Geo. Line Spacing"',
        // E=#VALUE!, V=0. Its numeric cache must not conceal the typed error.
        let doc = Document {
            document_sheet: sheet(vec![
                cell("Scratch.X2", "0", Some("\"Geo. Line Spacing\"")),
                cell("Scratch.Y2", "0", Some("\"SpLine\"")),
                cell("Scratch.X3", "999", Some("\"2.5\"")),
                cell("Scratch.Y3", "999", Some("\"-3\"")),
                cell("Scratch.A2", "0", Some("\"Geo. Line Spacing\"")),
                cell("Scratch.D2", "0", Some("\"3\"")),
            ]),
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(Scope::Document, "Scratch.X2"), Value::Error(Error::Value));
        assert_eq!(e.evaluate(Scope::Document, "Scratch.Y2"), Value::Error(Error::Value));
        assert_eq!(e.evaluate(Scope::Document, "Scratch.X3"), Value::Number(2.5));
        assert_eq!(e.evaluate(Scope::Document, "Scratch.Y3"), Value::Number(-3.));
        assert_eq!(e.evaluate(Scope::Document, "Scratch.A2"), Value::Text("Geo. Line Spacing".into()));
        assert_eq!(e.evaluate(Scope::Document, "Scratch.D2"), Value::Text("3".into()));
        for name in ["Scratch.X0", "Scratch.X-1", "Scratch.X+1", "Scratch.X4294967296", "Scratch.X1tail", "Scratch.Y", "Scratch.文1"] {
            assert_eq!(e.coerce_numeric_cell(Scope::Document, name, Value::Text("2".into())), Value::Text("2".into()), "{name}");
        }
        assert_eq!(e.coerce_numeric_cell(Scope::Document, "sCrAtCh.y4294967295", Value::Text("2".into())), Value::Number(2.));
        let oversized = format!("Scratch.X{}", "9".repeat(65_536));
        assert_eq!(e.coerce_numeric_cell(Scope::Document, &oversized, Value::Text("2".into())), Value::Text("2".into()));
    }
    #[test]
    fn unit_fraction_property_string_divides_in_internal_units() {
        // CRIME_SCENE_INVESTIGATION_M master2 shape12, User.InchPrec.
        let mut precision = cell("Prop.LInchPrec", "1/16 in.", None);
        precision.units = Some("STR".into());
        let doc = Document { document_sheet: sheet(vec![precision, cell("User.InchPrec", "999", Some("1IN/Prop.LInchPrec"))]), ..Default::default() };
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(Scope::Document, "User.InchPrec"), Value::Number(16.));
    }
    #[test]
    fn no_theme_gradient_defaults_and_text_background_color_index() {
        let mut doc = fixture();
        doc.colors.insert(25, "#ffffff".into());
        doc.face_names.insert(4, "Calibri".into());
        doc.styles.push(misterdiagram_model::Style { id: 0, ..Default::default() });
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 7, shape: 1 };
        assert_eq!(e.evaluate(scope, "LineGradientStops.GradientStopColor[10]"), Value::Number(1.));
        assert_eq!(e.evaluate(scope, "FillGradientStops.GradientStopColorTrans[2]"), Value::Number(0.));
        assert_eq!(e.cached_cell(&cell("TextBkgnd", "#ffffff", None)), Value::Number(26.));
        assert_eq!(e.cached_cell(&cell("FillForegnd", "#ffffff", None)), Value::Number(25.));
        e.set_formula(scope, "Char.Font", "IF(ISTHEMED(),THEMEVAL(),FONT(\"Calibri\"))").unwrap();
        assert_eq!(e.evaluate(scope, "Char.Font"), Value::Number(4.));
        e.set_formula(scope, "Width", "THEMEVAL()").unwrap();
        assert_eq!(e.evaluate(scope, "Width"), Value::Error(Error::Value));
    }
    #[test]
    fn deferred_text_comparison_preserves_cached_dependencies_only() {
        let doc = Document {
            document_sheet: sheet(vec![
                cell("Metric", "3", Some("TEXTWIDTH(TheText)")),
                cell("Dependent", "6", Some("Metric*2")),
                cell("Bad", "8", Some("TEXTWIDTH(TheText)+UNIMPLEMENTEDFUNCTION(1)")),
            ]),
            ..Default::default()
        };
        let mut live = Evaluator::new(&doc).unwrap();
        assert!(matches!(live.evaluate(Scope::Document, "Metric"), Value::Error(_)));
        let mut deferred = Evaluator::new(&doc).unwrap().with_deferred_text_metrics();
        assert_eq!(deferred.evaluate(Scope::Document, "Dependent"), Value::Number(6.));
        assert!(matches!(deferred.evaluate(Scope::Document, "Bad"), Value::Error(Error::Unsupported(_))));
    }
    #[test]
    fn cached_host_result_belongs_to_whole_cell_not_inner_call() {
        // ALARM_M master2 User.HasText and its dependent Controls.Row_1.YCon.
        let doc = Document {
            document_sheet: sheet(vec![
                cell("User.HasText", "0", Some("LEN(SHAPETEXT(TheText))>0")),
                cell("Result", "7", Some("2+5*NOT(User.HasText)")),
                cell("Bad", "42", Some("UNIMPLEMENTEDFUNCTION(1)")),
                cell("MixedBad", "42", Some("UNIMPLEMENTEDFUNCTION(1)+THEMEVAL()")),
                cell("Quoted", "42", Some("LEN(\"SHAPETEXT()\")")),
                cell("Width", "2", None),
                cell("HostDependent", "3", Some("NOW()+Width")),
            ]),
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(Scope::Document, "User.HasText"), Value::Bool(false));
        assert_eq!(e.evaluate(Scope::Document, "Result"), Value::Number(7.));
        assert!(matches!(e.evaluate(Scope::Document, "Bad"), Value::Error(Error::Unsupported(_))));
        assert!(matches!(e.evaluate(Scope::Document, "MixedBad"), Value::Error(Error::Unsupported(_))));
        assert_eq!(e.evaluate(Scope::Document, "Quoted"), Value::Number(11.));
        assert_eq!(e.evaluate(Scope::Document, "HostDependent"), Value::Number(3.));
        e.set_formula(Scope::Document, "Width", "4").unwrap();
        assert!(e.dirty.iter().any(|k| k.name == "hostdependent"));
        for formula in ["Pages[NOW()]!Width", "'THEMEVAL()'!Width", "UNIMPLEMENTEDFUNCTION(1)"] {
            assert!(hook_references(&misterdiagram_sheet::parse(formula).unwrap(), false).unwrap().is_none());
        }
    }
    #[test]
    fn speculative_rgb_cache_does_not_allocate_palette_ids() {
        let doc = Document::default();
        let mut e = Evaluator::new(&doc).unwrap();
        assert!(matches!(e.cached_cell(&cell("FillForegnd", "#123456", Some("RGB(1,2,3)"))), Value::Color { rgb: [18, 52, 86], .. }));
        assert!(e.palette.is_empty());
        assert_eq!(e.intern_color([1, 2, 3]), Some(24));
    }
    #[test]
    fn language_cells_use_lcid_not_text_zero() {
        let doc = Document::default();
        let evaluator = Evaluator::new(&doc).unwrap();
        assert_eq!(evaluator.cached_cell(&cell("LangID", "en-US", None)), Value::Number(1033.));
        assert_eq!(evaluator.cached_cell(&cell("DocLangID", "de-DE", None)), Value::Number(1031.));
        assert_eq!(evaluator.cached_cell(&cell("Text", "en-US", None)), Value::Text("en-US".into()));
    }
    #[test]
    fn oracle_snapshot_executes_then_rolls_back_setf() {
        let mut doc = fixture();
        doc.pages[0].shapes[0].sheet.cells.extend([cell("Query", "0", Some("SETF(GETREF(Width),9)+Width")), cell("Read", "0", Some("Width*2"))]);
        let scope = Scope::PageShape { page: 7, shape: 1 };
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate_snapshot(scope, "Query"), Value::Number(9.));
        assert_eq!(e.evaluate_snapshot(scope, "Read"), Value::Number(4.));
        assert!(e.overrides.is_empty());
        assert_eq!(e.assignments, 0);
        e.set_formula(scope, "Width", "3").unwrap();
        assert_eq!(e.evaluate_snapshot(scope, "Query"), Value::Number(9.));
        assert_eq!(e.evaluate(scope, "Width"), Value::Number(3.));
    }
    #[test]
    fn blank_host_font_id_is_distinct_from_facename_position() {
        let mut doc = fixture();
        doc.face_names = BTreeMap::from([(0, "Calibri".into()), (4, "Other".into())]);
        let e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.cached_cell(&cell("Font", "Calibri", None)), Value::Number(4.));
        assert_eq!(e.fonts.get(&5), Some(&"Other".to_string()));
    }
    #[test]
    fn inherited_constants_are_computed_from_provider_not_instance_cache() {
        let mut doc = fixture();
        doc.pages[0].shapes[0].master = Some(5);
        doc.pages[0].shapes[0].sheet.cells[0] = cell("Width", "999", Some("Inh"));
        let scope = Scope::PageShape { page: 7, shape: 1 };
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(scope, "Width"), Value::Number(8.));
        e.set_formula(Scope::MasterShape { master: 5, shape: 3 }, "Width", "9").unwrap();
        assert_eq!(e.evaluate(scope, "Width"), Value::Number(9.));
    }
    #[test]
    fn dirty_recalc_orders_dependencies_and_retains_unrelated_cache() {
        let mut doc = fixture();
        doc.pages[0].shapes[0].sheet.cells.extend([
            cell("Dependent", "0", Some("Width*2")),
            cell("Downstream", "0", Some("Dependent+1")),
            cell("Other", "0", Some("10+1")),
        ]);
        let scope = Scope::PageShape { page: 7, shape: 1 };
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(scope, "Downstream"), Value::Number(5.));
        assert_eq!(e.evaluate(scope, "Other"), Value::Number(11.));
        e.set_formula(scope, "Width", "7").unwrap();
        assert_eq!(e.dirty_count(), 3);
        assert_eq!(e.values.get(&Key { scope, name: "other".into() }).map(|cached| &cached.value), Some(&Value::Number(11.)));
        let changed = e.recalculate_dirty();
        assert_eq!(changed.iter().map(|r| r.1.as_str()).collect::<Vec<_>>(), vec!["width", "dependent", "downstream"]);
        assert_eq!(changed.last().unwrap().2, Value::Number(15.));
        assert_eq!(e.dirty_count(), 0);
        e.set_formula(scope, "Width", "Downstream").unwrap();
        assert!(e.recalculate_dirty().iter().all(|r| r.2 == Value::Error(Error::Cycle)));
    }
    #[test]
    fn deleted_master_child_markers_do_not_collide() {
        let mut doc = fixture();
        let deleted = Shape {
            id: u32::MAX,
            master_shape: Some(7),
            sheet: Sheet { attributes: misterdiagram_model::Attributes::from([("Del".into(), "1".into())]), ..Default::default() },
            ..Default::default()
        };
        doc.pages[0].shapes.extend([deleted.clone(), deleted]);
        let e = Evaluator::new(&doc).unwrap();
        assert!(e.shape(Scope::PageShape { page: 7, shape: u32::MAX }).is_none());
    }
    #[test]
    fn point_cell_projection_preserves_point_typed_values() {
        for (name, n) in [("BeginX", 2.), ("BeginY", 3.), ("A", 2.), ("B", 3.)] {
            assert_eq!(project_cell_point(&cell(name, "", None), Value::Point(2., 3.)), Value::Number(n));
        }
        let mut c = cell("Value", "", None);
        c.units = Some("PNT".into());
        assert_eq!(project_cell_point(&c, Value::Point(2., 3.)), Value::Point(2., 3.));
    }
    #[test]
    fn composed_formula_recursion_is_bounded_on_small_stack() {
        std::thread::Builder::new()
            .stack_size(2 * 1024 * 1024)
            .spawn(|| {
                let mut doc = fixture();
                let cells = &mut doc.pages[0].shapes[0].sheet.cells;
                cells.push(cell("Custom", "0", Some(&format!("{}EVALCELL(Custom)", "-".repeat(100)))));
                for i in 0..100 {
                    cells.push(cell(&format!("Deep{i}"), "0", Some(&format!("{}Deep{}", "-".repeat(100), i + 1))));
                }
                let scope = Scope::PageShape { page: 7, shape: 1 };
                let mut e = Evaluator::new(&doc).unwrap();
                assert_eq!(e.eval_cell(scope, "Custom", &[]), Value::Error(Error::Limit));
                assert_eq!(e.evaluate(scope, "Deep0"), Value::Error(Error::Limit));
                assert_eq!(e.eval_depth, 0);
                assert_eq!(e.evaluate(scope, "Width"), Value::Number(2.));
            })
            .unwrap()
            .join()
            .unwrap();
    }
    #[test]
    fn invalid_setf_target_cannot_execute_nested_assignment() {
        let mut doc = fixture();
        doc.pages[0].shapes[0].sheet.cells.push(cell("Invalid", "0", Some("SETF(SETF(GETREF(Width),9),\"1+\")")));
        let scope = Scope::PageShape { page: 7, shape: 1 };
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(scope, "Invalid"), Value::Error(Error::Value));
        assert_eq!(e.evaluate(scope, "Width"), Value::Number(2.));
    }
    #[test]
    fn references_across_pages_masters_styles_and_document() {
        let doc = fixture();
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 7, shape: 1 };
        assert_eq!(e.evaluate(scope, "Height"), Value::Number(1047.0));
        for (reference, value) in [
            ("Width", 2.),
            ("B!Width", 4.),
            ("Pages[Other]!Named!Width", 6.),
            ("Pages['A Page']!Sheet.2!Width", 4.),
            ("Masters[Gear]!Shaft!Width", 8.),
            ("Styles!Manager!LineColor", 3.),
            ("TheDoc!DocLangID", 1033.),
        ] {
            assert_eq!(e.resolve(scope, reference), Value::Number(value), "{reference}");
        }
        assert_eq!(e.resolve(scope, "Pages[missing]!Width"), Value::Error(Error::Ref));
    }
    #[test]
    fn formula_exists_inspects_formulas_without_evaluating_errors() {
        let doc = Document {
            document_sheet: Sheet {
                cells: vec![
                    Cell { name: "Constant".into(), value: Some("sample".into()), units: Some("STR".into()), ..Default::default() },
                    Cell { name: "Disabled".into(), value: Some("0".into()), formula: Some("No Formula".into()), ..Default::default() },
                    Cell { name: "Invalid".into(), value: Some("0".into()), formula: Some("1/0".into()), ..Default::default() },
                    Cell { name: "Query".into(), value: Some("0".into()), formula: Some("FORMULAEXISTS(Disabled)".into()), ..Default::default() },
                ],
                ..Default::default()
            },
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.formula_exists(Scope::Document, "Constant"), Ok(true));
        assert_eq!(e.formula_exists(Scope::Document, "Disabled"), Ok(false));
        assert_eq!(e.formula_exists(Scope::Document, "Missing"), Ok(false));
        assert_eq!(e.formula_exists(Scope::Document, "Invalid"), Ok(true));
        assert_eq!(e.evaluate(Scope::Document, "Query"), Value::Number(0.));
        e.set_formula(Scope::Document, "Disabled", "1/0").unwrap();
        assert_eq!(e.evaluate(Scope::Document, "Query"), Value::Number(1.));
    }
    #[test]
    fn geometry_path_virtual_values_follow_inheritance_and_deletions() {
        let geometry = Section {
            name: "Geometry".into(),
            index: Some(0),
            rows: vec![Row {
                index: Some(1),
                row_type: Some("MoveTo".into()),
                cells: vec![
                    Cell { name: "X".into(), value: Some("0".into()), ..Default::default() },
                    Cell { name: "Y".into(), value: Some("0".into()), ..Default::default() },
                ],
                ..Default::default()
            }],
            ..Default::default()
        };
        let doc = Document {
            masters: vec![misterdiagram_model::Master {
                id: 2,
                shapes: vec![Shape { id: 5, sheet: Sheet { sections: vec![geometry], ..Default::default() }, ..Default::default() }],
                ..Default::default()
            }],
            pages: vec![misterdiagram_model::Page {
                id: 0,
                shapes: vec![
                    Shape { id: 1, master: Some(2), ..Default::default() },
                    Shape {
                        id: 2,
                        master: Some(2),
                        sheet: Sheet {
                            sections: vec![Section { name: "Geometry".into(), index: Some(0), deleted: true, ..Default::default() }],
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 0, shape: 1 };
        assert_eq!(e.evaluate(scope, "Geometry1.Path"), Value::Number(0.));
        assert_eq!(e.evaluate(scope, "Geometry2.Path"), Value::Error(Error::Ref));
        assert_eq!(e.evaluate(Scope::PageShape { page: 0, shape: 2 }, "Geometry1.Path"), Value::Error(Error::Ref));
        assert!(e.resolved_named_cells(scope).iter().any(|n| n.name == "Geometry1.Path"));
        assert_eq!(e.evaluate(scope, "Geometry4294967296.Path"), Value::Error(Error::Ref));
    }
    #[test]
    fn text_fields_use_documented_indexed_universal_names() {
        let section = Section {
            name: "Field".into(),
            rows: vec![
                Row {
                    index: Some(0),
                    cells: vec![Cell { name: "Value".into(), value: Some("2".into()), ..Default::default() }],
                    ..Default::default()
                },
                Row {
                    index: Some(1),
                    cells: vec![Cell { name: "Format".into(), value: Some("date".into()), units: Some("STR".into()), ..Default::default() }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let doc = Document {
            pages: vec![misterdiagram_model::Page {
                id: 0,
                shapes: vec![Shape { id: 1, sheet: Sheet { sections: vec![section], ..Default::default() }, ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 0, shape: 1 };
        assert_eq!(e.resolve(scope, "Fields.Value"), Value::Number(2.));
        assert_eq!(e.resolve(scope, "Fields.Value[1]"), Value::Number(2.));
        assert_eq!(e.resolve(scope, "Fields.Format[2]"), Value::Text("date".into()));
        assert!(e.named_cells(scope).iter().any(|c| c.name == "Fields.Format[2]"));
    }
    #[test]
    fn actual_formatting_rows_expose_only_baseline_reserved_slots() {
        let section = Section {
            name: "Character".into(),
            rows: vec![
                Row {
                    index: Some(1),
                    cells: vec![Cell { name: "Size".into(), value: Some("0.1".into()), ..Default::default() }],
                    ..Default::default()
                },
                Row { index: Some(2), deleted: true, ..Default::default() },
            ],
            ..Default::default()
        };
        let doc = Document {
            pages: vec![misterdiagram_model::Page {
                id: 0,
                shapes: vec![Shape { id: 1, sheet: Sheet { sections: vec![section], ..Default::default() }, ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 0, shape: 1 };
        for name in ["Char.Q2", "Char.BE2", "Char._Unused6[2]"] {
            assert_eq!(e.evaluate(scope, name), Value::Number(0.));
        }
        assert_eq!(Evaluator::baseline_metadata(scope, "Char.BE2"), Some((3, 1, 56, 141)));
        assert!(e.cell(scope, "Char.Q3").is_none());
        assert!(e.cell(scope, "Char.Q999").is_none());
        assert!(e.cell(scope, "Char.Q4294967296").is_none());
        assert!(e.cell(scope, "Char.Size[2]").is_some());
        assert!(e.cell(scope, "Char.Case[2]").is_none());
        let names = e.resolved_named_cells(scope);
        assert!(names.iter().any(|c| c.name == "Char.Q2"));
        assert!(!names.iter().any(|c| c.name == "Char.Q3"));
    }
    #[test]
    fn named_and_indexed_sections_bind_to_universal_names() {
        let mut doc = fixture();
        let shape = &mut doc.pages[0].shapes[0];
        for (name, index, rowname, cname) in [
            ("User", 0, Some("x"), "Value"),
            ("Property", 0, Some("x"), "Format"),
            ("Property", 1, Some("y"), "Value"),
            ("Geometry", 1, None, "X"),
            ("Scratch", 0, None, "A"),
            ("Character", 0, None, "Size"),
            ("Character", 1, None, "Size"),
        ] {
            shape.sheet.sections.push(Section {
                name: name.into(),
                index: Some(0),
                rows: vec![Row { name: rowname.map(str::to_string), index: Some(index), cells: vec![cell(cname, "12", None)], ..Default::default() }],
                ..Default::default()
            });
        }
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 7, shape: 1 };
        for name in ["User.x", "User.x.Value", "Prop.y", "Geometry1.X1", "Scratch.A1", "Char.Size", "Char.Size[1]", "Char.Size[2]"] {
            assert_eq!(e.evaluate(scope, name), Value::Number(12.0), "{name}");
        }
        assert_eq!(e.evaluate(scope, "Prop.x.Format"), Value::Text("12".into()));
        assert_eq!(e.named_cells(scope).len(), 9);
    }
    #[test]
    fn clamped_cycles_are_proven_without_cached_formula_values() {
        let doc = Document {
            document_sheet: sheet(vec![
                cell("Height", "999", Some("Resize")),
                cell("Resize", "999", Some("MAX(.75,CEILING(.24+Offset,.25))")),
                cell("Offset", "999", Some("MIN(Height/2,.25)")),
                cell("Ambiguous", "999", Some("MAX(1,Ambiguous)")),
                cell("Divergent", "999", Some("MAX(1,Divergent+1)")),
                cell("Missing", "999", Some("MAX(.75,MIN(Missing+NoSuchCell,.25))")),
            ]),
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(Scope::Document, "Height"), Value::Number(0.75));
        assert_eq!(e.evaluate(Scope::Document, "Offset"), Value::Number(0.25));
        for name in ["Ambiguous", "Divergent", "Missing"] {
            assert!(matches!(e.evaluate(Scope::Document, name), Value::Error(_)), "{name}");
        }
        e.set_formula(Scope::Document, "Resize", "MAX(1,CEILING(.24+Offset,.25))").unwrap();
        assert_eq!(e.evaluate(Scope::Document, "Height"), Value::Number(1.));
    }
    #[test]
    fn cycles_missing_references_and_cached_hooks() {
        let mut doc = fixture();
        let s = &mut doc.pages[0].shapes[0].sheet;
        s.cells.push(cell("CycleA", "0", Some("CycleB")));
        s.cells.push(cell("CycleB", "0", Some("CycleA")));
        s.cells.push(cell("Hook", "42", Some("NOW()")));
        s.cells.push(cell("Inherited", "7", Some("Inh")));
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 7, shape: 1 };
        assert_eq!(e.evaluate(scope, "CycleA"), Value::Error(Error::Cycle));
        assert_eq!(e.evaluate(scope, "missing"), Value::Error(Error::Ref));
        assert_eq!(e.evaluate(scope, "Hook"), Value::Number(42.));
        // Step 4 removes the temporary Inh cached-value path: no provider is #REF.
        assert_eq!(e.evaluate(scope, "Inherited"), Value::Error(Error::Ref));
        e.clear_cache();
        assert_eq!(e.evaluate(scope, "CycleB"), Value::Error(Error::Cycle));
    }
    #[test]
    fn palette_font_tables_and_cached_color_font_values() {
        let doc = fixture();
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.cached_cell(&cell("FillForegnd", "#CC0000", None)), Value::Number(26.));
        assert_eq!(e.cached_cell(&cell("Font", "Example", None)), Value::Number(0.));
        let mut ctx = BoundContext { evaluator: &mut e, scope: Scope::Document, origin: Scope::Document, cached: Value::Number(0.) };
        assert_eq!(ctx.color(26), Some([204, 0, 0]));
        assert_eq!(ctx.color_index([204, 0, 0]), Some(26));
        assert_eq!(ctx.font(0), Some("Example".into()));
        assert_eq!(ctx.font_index("example"), Some(0));
        assert_eq!(ctx.color(99), None);
    }
    #[test]
    fn reset_isolates_assignments_palette_and_budget_state() {
        let mut doc = fixture();
        let scope = Scope::PageShape { page: 7, shape: 1 };
        doc.pages[0].shapes[0].sheet.cells.push(cell("Assign", "99", Some("SETF(GETREF(Width),9)")));
        let mut e = Evaluator::new(&doc).unwrap();
        e.evaluate(scope, "Assign");
        assert_eq!(e.evaluate(scope, "Width"), Value::Number(9.));
        e.palette.insert(300, [1, 2, 3]);
        e.graph_limited = true;
        e.reset_evaluation();
        let mut fresh = Evaluator::new(&doc).unwrap();
        assert_eq!(e.palette, fresh.palette);
        assert!(e.overrides.is_empty() && e.dependencies.is_empty() && e.dirty.is_empty());
        assert!(!e.graph_limited);
        assert_eq!(e.evaluate(scope, "Width"), fresh.evaluate(scope, "Width"));
        assert_eq!(e.evaluate(scope, "Height"), fresh.evaluate(scope, "Height"));
    }
    #[test]
    fn assignments_are_real_atomic_and_keep_source_immutable() {
        let mut doc = fixture();
        let scope = Scope::PageShape { page: 7, shape: 1 };
        doc.pages[0].shapes[0].sheet.cells.push(cell("Assign", "99", Some("SETF(GETREF(Width),\"7+2\")")));
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(scope, "Width"), Value::Number(2.));
        assert_eq!(e.evaluate(scope, "Assign"), Value::Number(0.));
        assert_eq!(e.evaluate(scope, "Width"), Value::Number(9.));
        assert!(e.set_formula(scope, "Width", "1+").is_err());
        assert_eq!(e.evaluate(scope, "Width"), Value::Number(9.));
        assert_eq!(doc.pages[0].shapes[0].sheet.cells[0].value.as_deref(), Some("2"));
    }
    #[test]
    fn argument_frames_bypass_cache_and_use_target_scope() {
        let mut doc = fixture();
        let target = Scope::PageShape { page: 7, shape: 2 };
        let caller = Scope::PageShape { page: 7, shape: 1 };
        doc.pages[0].shapes[1].sheet.cells.extend([cell("Helper", "0", Some("ARG(\"A\")")), cell("Custom", "0", Some("Helper+Width"))]);
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(target, "Helper"), Value::Error(Error::Value));
        assert_eq!(e.eval_cell(caller, "Sheet.2!Custom", &[("A".into(), Value::Number(3.))]), Value::Number(7.));
        assert_eq!(e.eval_cell(caller, "Sheet.2!Custom", &[("A".into(), Value::Number(9.))]), Value::Number(13.));
        assert_eq!(e.evaluate(target, "Helper"), Value::Error(Error::Value));
        e.set_formula(target, "Custom", "EVALCELL(Custom)").unwrap();
        assert_eq!(e.eval_cell(caller, "Sheet.2!Custom", &[]), Value::Error(Error::Limit));
    }
    #[test]
    fn text_formula_and_dependency_edges_are_bound() {
        let mut doc = fixture();
        let scope = Scope::PageShape { page: 7, shape: 1 };
        doc.pages[0].shapes[0].text = Some(crate::parse_xml(b"<Text>4 ft + 0.5 ft</Text>").unwrap());
        doc.pages[0].shapes[0].sheet.cells.extend([
            cell("TextValue", "0", Some("EVALTEXT(TheText)")),
            cell("D", "0", Some("DEPENDSON(Width)")),
            cell("R", "0", Some("GETREF(Width)")),
            cell("EventXFMod", "0", Some("DEPENDSON(Width)")),
        ]);
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(scope, "TextValue"), Value::Number(54.));
        assert_eq!(e.evaluate(scope, "D"), Value::Bool(false));
        assert_eq!(e.evaluate(scope, "R"), Value::Text("Width".into()));
        // Passive reads retain the saved event result; event execution is separate.
        assert_eq!(e.evaluate(scope, "EventXFMod"), Value::Number(0.));
        assert!(e.dependencies[&Key { scope, name: "d".into() }].contains(&Key { scope, name: "width".into() }));
        assert!(!e.dependencies.contains_key(&Key { scope, name: "r".into() }));
        assert!(!e.dependencies.contains_key(&Key { scope, name: "eventxfmod".into() }));
    }
    #[test]
    fn hostile_reference_and_model_limits() {
        let doc = fixture();
        let e = Evaluator::new(&doc).unwrap();
        for s in ["Pages[unclosed!Width", "Sheet.1!!Width", "'unterminated!Width", "Pages[x]]!Width"] {
            assert!(e.reference_target(Scope::Document, s).is_err(), "{s}")
        }
        assert!(e.reference_target(Scope::Document, &"a".repeat(65537)).is_err());
        let mut row = Row { index: Some(u32::MAX), ..Default::default() };
        assert!(cell_name(&Section { name: "Scratch".into(), ..Default::default() }, &row, 0, &cell("A", "1", None)).is_err());
        row.index = Some(0);
        assert_eq!(
            cell_name(&Section { name: "Geometry".into(), index: Some(u32::MAX), ..Default::default() }, &row, 0, &cell("X", "1", None)),
            Err(Error::Limit)
        );
    }
}
