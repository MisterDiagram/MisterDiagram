//! Document bindings for the model-independent ShapeSheet evaluator.
use diagramcraft_model::{Cell, Document, Row, Section, Shape, Sheet};
use diagramcraft_sheet::{Context, Error, Value};
use std::collections::{BTreeMap, BTreeSet};
mod path;
mod transform;
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
/// Local document evaluation with cached hooks, bounded dependencies and explicit error values.
/// Master/style/default inheritance is layered on these scopes separately.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
pub struct Evaluator<'a> {
    pub document: &'a Document,
    sheets: BTreeMap<Scope, &'a Sheet>,
    shapes: BTreeMap<Scope, &'a Shape>,
    parents: BTreeMap<Scope, Scope>,
    cells: BTreeMap<Key, &'a Cell>,
    canonical: BTreeMap<Key, Key>,
    names: BTreeMap<Scope, Vec<NamedCell<'a>>>,
    values: BTreeMap<Key, Value>,
    active: BTreeSet<Key>,
    work: usize,
    eval_depth: usize,
    eval_work: usize,
    indexed_name_bytes: usize,
    palette: BTreeMap<u32, [u8; 3]>,
    overrides: BTreeMap<Key, String>,
    override_bytes: usize,
    arguments: Vec<BTreeMap<String, Value>>,
    argument_bytes: usize,
    assignments: usize,
    revision: usize,
    dependencies: BTreeMap<Key, BTreeSet<Key>>,
    dependency_count: usize,
    dependency_bytes: usize,
    graph_limited: bool,
    stack: Vec<Key>,
}
impl<'a> Evaluator<'a> {
    pub fn new(document: &'a Document) -> Result<Self, Error> {
        if document.colors.len() > 65536 {
            return Err(Error::Limit);
        }
        let mut result = Self {
            document,
            sheets: BTreeMap::new(),
            shapes: BTreeMap::new(),
            parents: BTreeMap::new(),
            cells: BTreeMap::new(),
            canonical: BTreeMap::new(),
            names: BTreeMap::new(),
            values: BTreeMap::new(),
            active: BTreeSet::new(),
            work: 0,
            eval_depth: 0,
            eval_work: 0,
            indexed_name_bytes: 0,
            overrides: BTreeMap::new(),
            override_bytes: 0,
            arguments: Vec::new(),
            argument_bytes: 0,
            assignments: 0,
            revision: 0,
            dependencies: BTreeMap::new(),
            dependency_count: 0,
            dependency_bytes: 0,
            graph_limited: false,
            stack: Vec::new(),
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
        Ok(result)
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
                    // First indexed formatting rows also accept the explicit [1] syntax.
                    if matches!(section.name.as_str(), "Character" | "Paragraph" | "Layer" | "FillGradient" | "LineGradient")
                        && row.index.unwrap_or(position as u32) == 0
                    {
                        self.alias(scope, format!("{name}[1]"), c, &name)?;
                    }
                    if matches!(section.name.as_str(), "User" | "Property") && c.name == "Value" {
                        self.alias(scope, format!("{name}.Value"), c, &name)?;
                    }
                    if matches!(section.name.as_str(), "Control" | "Controls") && c.name == "X" {
                        self.alias(scope, format!("{name}.X"), c, &name)?;
                    }
                }
            }
        }
        self.names.insert(scope, names);
        Ok(())
    }
    fn alias(&mut self, scope: Scope, name: String, cell: &'a Cell, canonical: &str) -> Result<(), Error> {
        if self.cells.len() >= MAX_CELLS || name.len() > 65536 {
            return Err(Error::Limit);
        }
        self.indexed_name_bytes = self.indexed_name_bytes.checked_add(name.len()).ok_or(Error::Limit)?;
        if self.indexed_name_bytes > 64 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        let key = Key { scope, name: name.to_ascii_lowercase() };
        self.canonical.insert(key.clone(), Key { scope, name: canonical.to_ascii_lowercase() });
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
        self.alias(scope, name.clone(), cell, &name)?;
        names.push(NamedCell { name, cell, section, row });
        Ok(())
    }
    pub fn scopes(&self) -> impl Iterator<Item = Scope> + '_ {
        self.sheets.keys().copied()
    }
    pub fn sheet(&self, scope: Scope) -> Option<&'a Sheet> {
        self.sheets.get(&scope).copied()
    }
    pub fn shape(&self, scope: Scope) -> Option<&'a Shape> {
        self.shapes.get(&scope).copied()
    }
    pub fn named_cells(&self, scope: Scope) -> &[NamedCell<'a>] {
        self.names.get(&scope).map(Vec::as_slice).unwrap_or(&[])
    }
    pub fn cell(&self, scope: Scope, name: &str) -> Option<&'a Cell> {
        self.cells.get(&Key { scope, name: name.to_ascii_lowercase() }).copied()
    }
    pub fn clear_cache(&mut self) {
        self.values.clear();
    }
    pub fn evaluate(&mut self, scope: Scope, name: &str) -> Value {
        if self.eval_depth == 0 {
            self.work = 0;
            self.eval_work = 0;
        }
        self.evaluate_inner(scope, name)
    }
    fn evaluate_inner(&mut self, scope: Scope, name: &str) -> Value {
        if self.graph_limited || name.len() > 65536 || self.active.len() >= 128 || self.work >= 100_000 {
            return Value::Error(Error::Limit);
        }
        self.work += 1;
        let Some(key) = self.canonical_key(scope, name) else { return Value::Error(Error::Ref) };
        if self.arguments.is_empty()
            && let Some(value) = self.values.get(&key)
        {
            return value.clone();
        }
        if !self.active.insert(key.clone()) {
            return Value::Error(Error::Cycle);
        }
        let revision = self.revision;
        self.stack.push(key.clone());
        let value = if let Some(cell) = self.cells.get(&key).copied() {
            let cached = self.cached_cell(cell);
            let formula = self.overrides.get(&key).cloned().or_else(|| cell.formula.clone());
            match formula.as_deref() {
                None | Some("No Formula") => cached,
                // Step 4 must resolve inherited formulas before corpus Oracle 1 is
                // enabled. This temporary Step 2 path is not formula-evaluation proof.
                Some("Inh") => cached,
                Some(formula) => {
                    let mut context = BoundContext { evaluator: self, scope, cached };
                    diagramcraft_sheet::formula(formula, &mut context)
                }
            }
        } else {
            Value::Error(Error::Ref)
        };
        let value = self.cells.get(&key).map_or(value.clone(), |cell| project_cell_point(cell, value));
        self.stack.pop();
        self.active.remove(&key);
        let value = if self.graph_limited { Value::Error(Error::Limit) } else { value };
        if self.arguments.is_empty() && revision == self.revision {
            self.values.insert(key, value.clone());
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
        self.override_bytes = bytes;
        self.overrides.insert(key, formula.into());
        self.assignments += 1;
        self.revision = self.revision.checked_add(1).ok_or(Error::Limit)?;
        self.values.clear();
        Ok(())
    }
    fn canonical_key(&self, scope: Scope, name: &str) -> Option<Key> {
        if name.len() > 65536 {
            return None;
        }
        self.canonical.get(&Key { scope, name: name.to_ascii_lowercase() }).cloned()
    }
    /// Custom function arguments live only during the invocation, and bypass ordinary memoization.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/evalcell-function
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/arg-function
    pub fn eval_cell(&mut self, scope: Scope, reference: &str, args: &[(String, Value)]) -> Value {
        if self.eval_depth == 0 {
            self.work = 0;
            self.eval_work = 0;
        }
        if args.len() > 256 || self.arguments.len() >= 64 || self.work >= 100000 {
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
        let formula = self.overrides.get(&key).cloned().or_else(|| cell.formula.clone());
        let cached = self.cached_cell(cell);
        let mut bytes = 0usize;
        for (name, value) in args {
            if name.len() > 65536 {
                return Value::Error(Error::Limit);
            }
            bytes = bytes.saturating_add(name.len()).saturating_add(match value {
                Value::Text(s) => s.len(),
                _ => 64,
            });
        }
        if self.argument_bytes.saturating_add(bytes) > 8 * 1024 * 1024 {
            return Value::Error(Error::Limit);
        }
        self.argument_bytes += bytes;
        self.arguments.push(args.iter().map(|(n, v)| (n.to_ascii_lowercase(), v.clone())).collect());
        let result = match formula.as_deref() {
            None | Some("No Formula") => cached,
            Some(f) => {
                let mut context = BoundContext { evaluator: self, scope: target, cached };
                diagramcraft_sheet::formula(f, &mut context)
            }
        };
        self.arguments.pop();
        self.argument_bytes -= bytes;
        result
    }
    fn add_dependency(&mut self, scope: Scope, reference: &str) {
        if let Ok((scope, name)) = self.reference_target(scope, reference)
            && let Some(target) = self.canonical_key(scope, &name)
            && let Some(caller) = self.stack.last().cloned()
        {
            let edges = self.dependencies.entry(caller.clone()).or_default();
            if !edges.contains(&target) {
                let bytes = caller.name.len().saturating_add(target.name.len());
                if self.dependency_count >= 2_000_000 || self.dependency_bytes.saturating_add(bytes) > 256 * 1024 * 1024 {
                    self.graph_limited = true;
                    return;
                }
                edges.insert(target);
                self.dependency_count += 1;
                self.dependency_bytes += bytes;
            }
        }
    }
    /// The V attribute is already in internal units. Character font names resolve to FaceNames indices.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/font-cell-character-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
    pub fn cached_cell(&self, cell: &Cell) -> Value {
        if cell.value.as_ref().is_some_and(|s| s.len() > 65536) {
            return Value::Error(Error::Limit);
        }
        if cell.units.as_deref() != Some("STR")
            && cell.error.is_none()
            && let Some(rgb) = cell.value.as_deref().and_then(parse_color)
            && let Some(index) = self.document.colors.iter().find_map(|(i, v)| (parse_color(v) == Some(rgb)).then_some(*i))
        {
            return Value::Number(f64::from(index));
        }
        if cell.name == "Font"
            && cell.error.is_none()
            && let Some(name) = cell.value.as_deref()
            && let Some(index) = self.document.face_names.iter().find_map(|(i, n)| n.eq_ignore_ascii_case(name).then_some(*i))
        {
            return Value::Number(f64::from(index));
        }
        diagramcraft_sheet::units::cached(cell.value.as_deref(), cell.units.as_deref(), cell.error.as_deref())
    }
    /// Resolve local, cross-shape, document, named page/master and style-qualified references.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
    pub fn resolve(&mut self, scope: Scope, reference: &str) -> Value {
        self.add_dependency(scope, reference);
        match self.reference_target(scope, reference) {
            Ok((scope, name)) => self.evaluate_inner(scope, &name),
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
fn page_scope(scope: Scope) -> Option<Scope> {
    match scope {
        Scope::Page(p) | Scope::PageShape { page: p, .. } => Some(Scope::Page(p)),
        Scope::Master(m) | Scope::MasterShape { master: m, .. } => Some(Scope::Master(m)),
        _ => None,
    }
}
fn section_prefix(section: &Section) -> Result<String, Error> {
    Ok(match section.name.as_str() {
        "Geometry" => format!("Geometry{}", section.index.unwrap_or(0).checked_add(1).ok_or(Error::Limit)?),
        "Character" => "Char".into(),
        "Paragraph" => "Para".into(),
        "Property" => "Prop".into(),
        "Control" => "Controls".into(),
        "Connection" => "Connections".into(),
        "Layer" => "Layers".into(),
        "FillGradient" => "FillGradientStops".into(),
        "LineGradient" => "LineGradientStops".into(),
        other => other.into(),
    })
}
/// Convert XML zero-based formatting row indices and named rows to universal formula names.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
/// https://learn.microsoft.com/en-us/office/client-developer/visio/size-cell-character-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/value-cell-user-defined-cells-section
pub fn cell_name(section: &Section, row: &Row, position: usize, cell: &Cell) -> Result<String, Error> {
    let prefix = section_prefix(section)?;
    let index = row.index.unwrap_or(u32::try_from(position).map_err(|_| Error::Limit)?);
    let one = index.checked_add(1).ok_or(Error::Limit)?;
    Ok(match section.name.as_str() {
        "Geometry" => format!("{prefix}.{}{index}", cell.name),
        "Scratch" | "Connection" | "Tabs" if row.name.is_none() => format!("{prefix}.{}{one}", cell.name),
        "Character" | "Paragraph" | "Layer" | "FillGradient" | "LineGradient" => {
            if index == 0 {
                format!("{prefix}.{}", cell.name)
            } else {
                format!("{prefix}.{}[{one}]", cell.name)
            }
        }
        _ => {
            let row_name = row.name.clone().unwrap_or_else(|| format!("Row_{one}"));
            let omit = (matches!(section.name.as_str(), "User" | "Property") && cell.name == "Value")
                || (matches!(section.name.as_str(), "Control" | "Controls") && cell.name == "X");
            if omit { format!("{prefix}.{row_name}") } else { format!("{prefix}.{row_name}.{}", cell.name) }
        }
    })
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
struct BoundContext<'a, 'd> {
    evaluator: &'a mut Evaluator<'d>,
    scope: Scope,
    cached: Value,
}
impl Context for BoundContext<'_, '_> {
    fn enter_eval(&mut self) -> bool {
        if self.evaluator.eval_depth >= 128 || self.evaluator.eval_work >= 100000 {
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
        self.evaluator.resolve(self.scope, reference)
    }
    fn path_point(&mut self, reference: &str, travel: f64, offset: f64, segment: Option<usize>) -> Value {
        match self.evaluator.path_sample(self.scope, reference, travel, segment) {
            Ok(((x, y), (dx, dy))) if offset.is_finite() => {
                let (x, y) = (x - dy * offset, y + dx * offset);
                if x.is_finite() && y.is_finite() { Value::Point(x, y) } else { Value::Error(Error::Num) }
            }
            Ok(_) => Value::Error(Error::Num),
            Err(e) => Value::Error(e),
        }
    }
    fn path_angle(&mut self, reference: &str, travel: f64, segment: Option<usize>) -> Value {
        match self.evaluator.path_sample(self.scope, reference, travel, segment) {
            Ok((_, (dx, dy))) => Value::number(dy.atan2(dx)),
            Err(e) => Value::Error(e),
        }
    }
    fn transform_point(
        &mut self,
        point: (f64, f64),
        source: Option<&str>,
        source_parent: bool,
        destination: Option<&str>,
        destination_parent: bool,
    ) -> Value {
        match self.evaluator.transform_point(self.scope, point, source, source_parent, destination, destination_parent) {
            Ok((x, y)) => Value::Point(x, y),
            Err(e) => Value::Error(e),
        }
    }
    fn set_formula(&mut self, reference: &str, formula: &str) -> Result<(), Error> {
        self.evaluator.set_formula(self.scope, reference, formula)
    }
    fn eval_cell(&mut self, reference: &str, args: &[(String, Value)]) -> Value {
        self.evaluator.eval_cell(self.scope, reference, args)
    }
    fn argument(&self, name: &str) -> Option<Value> {
        self.evaluator.arguments.last()?.get(&name.to_ascii_lowercase()).cloned()
    }
    fn dependency(&mut self, reference: &str) {
        if self.evaluator.stack.last().is_some_and(|k| k.name.starts_with("event") || k.name.starts_with("actions.") && k.name.ends_with(".action")) {
            return;
        }
        self.evaluator.add_dependency(self.scope, reference)
    }
    fn shape_text(&mut self, reference: &str) -> Value {
        let (scope, name) = match self.evaluator.reference_target(self.scope, reference) {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        };
        if !name.eq_ignore_ascii_case("TheText") {
            return Value::Error(Error::Ref);
        }
        let Some(shape) = self.evaluator.shape(scope) else { return Value::Error(Error::Ref) };
        match shape.text.as_ref().map(limited_text).transpose() {
            Ok(s) => Value::Text(s.unwrap_or_default()),
            Err(e) => Value::Error(e),
        }
    }
    fn cached(&self) -> Value {
        self.cached.clone()
    }
    fn unit(&self, reference: &str) -> Option<String> {
        let (scope, name) = self.evaluator.reference_target(self.scope, reference).ok()?;
        self.evaluator.cell(scope, &name)?.units.clone()
    }
    fn color(&self, index: u32) -> Option<[u8; 3]> {
        self.evaluator.palette.get(&index).copied().or_else(|| diagramcraft_sheet::functions::default_color(index))
    }
    fn color_index(&mut self, rgb: [u8; 3]) -> Option<u32> {
        if let Some(index) = self.evaluator.palette.iter().find_map(|(index, value)| (*value == rgb).then_some(*index)) {
            return Some(index);
        }
        if self.evaluator.palette.len() >= 65536 {
            return None;
        }
        let index = self.evaluator.palette.last_key_value().map_or(Some(24), |(i, _)| i.checked_add(1).map(|i| i.max(24)))?;
        self.evaluator.palette.insert(index, rgb);
        Some(index)
    }
    fn font(&self, index: u32) -> Option<String> {
        self.evaluator.document.face_names.get(&index).cloned()
    }
    fn font_index(&self, name: &str) -> Option<u32> {
        self.evaluator.document.face_names.iter().find_map(|(index, value)| value.eq_ignore_ascii_case(name).then_some(*index))
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
fn parse_color(s: &str) -> Option<[u8; 3]> {
    let s = s.strip_prefix('#')?;
    if s.len() != 6 {
        return None;
    }
    Some([u8::from_str_radix(s.get(..2)?, 16).ok()?, u8::from_str_radix(s.get(2..4)?, 16).ok()?, u8::from_str_radix(s.get(4..)?, 16).ok()?])
}
fn limited_text(element: &diagramcraft_model::XmlElement) -> Result<String, Error> {
    let mut out = String::new();
    let mut stack = vec![element.children.iter()];
    let mut nodes = 0usize;
    while let Some(iter) = stack.last_mut() {
        nodes += 1;
        if nodes > 1000000 {
            return Err(Error::Limit);
        }
        match iter.next() {
            Some(diagramcraft_model::XmlNode::Text(s)) => {
                if out.len().saturating_add(s.len()) > 65536 {
                    return Err(Error::Limit);
                }
                out.push_str(s)
            }
            Some(diagramcraft_model::XmlNode::Element(e)) => {
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
    diagramcraft_sheet::parse(formula).map(|_| ()).map_err(|e| Error::Parse(e.to_string()))
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
        use diagramcraft_model::{Master, Page, Style};
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
    fn deleted_master_child_markers_do_not_collide() {
        let mut doc = fixture();
        let deleted = Shape {
            id: u32::MAX,
            master_shape: Some(7),
            sheet: Sheet { attributes: BTreeMap::from([("Del".into(), "1".into())]), ..Default::default() },
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
        for name in ["User.x", "User.x.Value", "Prop.x.Format", "Prop.y", "Geometry1.X1", "Scratch.A1", "Char.Size", "Char.Size[1]", "Char.Size[2]"] {
            assert_eq!(e.evaluate(scope, name), Value::Number(12.0), "{name}");
        }
        assert_eq!(e.named_cells(scope).len(), 9);
    }
    #[test]
    fn cycles_missing_references_and_cached_hooks() {
        let mut doc = fixture();
        let s = &mut doc.pages[0].shapes[0].sheet;
        s.cells.push(cell("CycleA", "0", Some("CycleB")));
        s.cells.push(cell("CycleB", "0", Some("CycleA")));
        s.cells.push(cell("Hook", "42", Some("THEMEVAL()")));
        s.cells.push(cell("Inherited", "7", Some("Inh")));
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 7, shape: 1 };
        assert_eq!(e.evaluate(scope, "CycleA"), Value::Error(Error::Cycle));
        assert_eq!(e.evaluate(scope, "missing"), Value::Error(Error::Ref));
        assert_eq!(e.evaluate(scope, "Hook"), Value::Number(42.));
        assert_eq!(e.evaluate(scope, "Inherited"), Value::Number(7.));
        e.clear_cache();
        assert_eq!(e.evaluate(scope, "CycleB"), Value::Error(Error::Cycle));
    }
    #[test]
    fn palette_font_tables_and_cached_color_font_values() {
        let doc = fixture();
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.cached_cell(&cell("FillForegnd", "#CC0000", None)), Value::Number(26.));
        assert_eq!(e.cached_cell(&cell("Font", "Example", None)), Value::Number(0.));
        let mut ctx = BoundContext { evaluator: &mut e, scope: Scope::Document, cached: Value::Number(0.) };
        assert_eq!(ctx.color(26), Some([204, 0, 0]));
        assert_eq!(ctx.color_index([204, 0, 0]), Some(26));
        assert_eq!(ctx.font(0), Some("Example".into()));
        assert_eq!(ctx.font_index("example"), Some(0));
        assert_eq!(ctx.color(99), None);
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
        assert_eq!(e.evaluate(scope, "EventXFMod"), Value::Bool(false));
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
