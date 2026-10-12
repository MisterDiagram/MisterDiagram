//! Effective ShapeSheets retain borrowed source cells and explicit formula provenance.
use super::{Cell, Error, Evaluator, Key, MAX_CELLS, MAX_SCOPES, NamedCell, Row, Scope, Section};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub(super) struct Inheritance<'a> {
    local_cells: BTreeMap<Key, &'a Cell>,
    origins: BTreeMap<Key, Scope>,
    errors: BTreeMap<Scope, Error>,
    masters: BTreeMap<Scope, Scope>,
    instances: BTreeMap<Scope, (Scope, u32)>,
    remapping: BTreeMap<(Scope, u32), Scope>,
    geometry: BTreeMap<(Scope, u32), Vec<GeometryRow<'a>>>,
    pub(super) omitted_master_children: usize,
}
/// A merged geometry row contains borrowed cells and the inherited row type.
/// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/74428617-9833-4d73-aa7f-f3a6f043a12d
#[derive(Clone, Debug)]
pub(super) struct GeometryRow<'a> {
    pub index: u32,
    pub row_type: Option<&'a str>,
    pub cells: Vec<&'a Cell>,
}
#[derive(Clone)]
struct Entry<'a> {
    named: NamedCell<'a>,
    origin: Scope,
}
#[derive(Clone, Default)]
struct Effective<'a> {
    cells: BTreeMap<String, Entry<'a>>,
    geometry: BTreeMap<u32, BTreeMap<u32, GeometryRow<'a>>>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Category {
    Line,
    Fill,
    Text,
}
/// Theme selectors follow formatting style providers, not an unconditional No Style fallback.
/// Corpus witness: basic/0001-probe, Theme style 6 has 65534 while No Style 0 has 0.
/// Separate-chain routing follows each selector's purpose; conflicting chains remain unverified.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/themeindex-cell-theme-properties-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colorschemeindex-cell-theme-properties-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/effectschemeindex-cell-theme-properties-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fontschemeindex-cell-theme-properties-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connectorschemeindex-cell-theme-properties-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/variationcolorindex-cell-theme-properties-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/variationstyleindex-cell-theme-properties-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/embellishmentindex-cell-theme-properties-section
fn category(name: &str) -> Option<Category> {
    let name = name.to_ascii_lowercase();
    if name.starts_with("char.")
        || name.starts_with("para.")
        || name.starts_with("tabs.")
        || matches!(
            name.as_str(),
            "leftmargin"
                | "rightmargin"
                | "topmargin"
                | "bottommargin"
                | "verticalalign"
                | "textbkgnd"
                | "textbkgndtrans"
                | "textdirection"
                | "defaulttabstop"
                | "textmaxdepth"
                | "textgeometry"
                | "textcontainer"
                | "quickstylefontcolor"
                | "quickstylefontmatrix"
                | "fontschemeindex"
        )
    {
        Some(Category::Text)
    } else if name.starts_with("line")
        || name.starts_with("begarrow")
        || name.starts_with("beginarrow")
        || name.starts_with("endarrow")
        || name.starts_with("quickstyleline")
        || matches!(name.as_str(), "rounding" | "compoundtype" | "themeindex" | "colorschemeindex" | "connectorschemeindex" | "variationcolorindex")
    {
        Some(Category::Line)
    } else if name.starts_with("fill")
        || name.starts_with("shdw")
        || name.starts_with("shapeshdw")
        || name.starts_with("quickstylefill")
        || name.starts_with("quickstyleshadow")
        || name.starts_with("bevel")
        || name.starts_with("reflection")
        || name.starts_with("glow")
        || name.starts_with("rotation")
        || name.starts_with("sketch")
        || matches!(
            name.as_str(),
            "softedgessize" | "quickstyleeffectsmatrix" | "perspective" | "effectschemeindex" | "variationstyleindex" | "embellishmentindex"
        )
    {
        Some(Category::Fill)
    } else {
        None
    }
}
impl<'a> Evaluator<'a> {
    /// Resolve master shapes, category-specific style chains and local tombstones.
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/74428617-9833-4d73-aa7f-f3a6f043a12d
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/59214758-549c-4f99-a4de-de2bf5fb08d3
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/64509af3-e393-4ac1-9474-949c50d4b06f
    pub(super) fn install_inheritance(&mut self) -> Result<(), Error> {
        self.inheritance.local_cells = self.cells.clone();
        self.master_links()?;
        let local_names = self.names.clone();
        let mut resolved = BTreeMap::new();
        let mut visiting = BTreeSet::new();
        let scopes: Vec<_> = self.sheets.keys().copied().collect();
        let mut count = 0usize;
        for scope in &scopes {
            self.effective(*scope, &local_names, &mut resolved, &mut visiting, &mut count, 0)?;
        }
        self.cells.clear();
        self.canonical.clear();
        self.names.clear();
        self.indexed_name_bytes = 0;
        for scope in scopes {
            let Some(effective) = resolved.remove(&scope) else { continue };
            let mut names = Vec::new();
            for entry in effective.cells.into_values() {
                let n = entry.named;
                self.add_cell(scope, n.name.clone(), n.cell, n.section, n.row, &mut names)?;
                self.inheritance.origins.insert(Key { scope, name: n.name.to_ascii_lowercase() }, entry.origin);
                if let Some(section) = n.section
                    && n.row.is_some()
                {
                    if matches!(section.name.as_str(), "Character" | "Paragraph" | "Layer" | "FillGradient" | "LineGradient")
                        && !n.name.ends_with(']')
                    {
                        self.alias(scope, format!("{}[1]", n.name), n.cell, &n.name)?;
                    }
                    if matches!(section.name.as_str(), "User" | "Property") && n.cell.name == "Value" {
                        self.alias(scope, format!("{}.Value", n.name), n.cell, &n.name)?;
                    }
                    if matches!(section.name.as_str(), "Control" | "Controls") && n.cell.name == "X" {
                        self.alias(scope, format!("{}.X", n.name), n.cell, &n.name)?;
                    }
                }
            }
            self.names.insert(scope, names);
            for (index, rows) in effective.geometry {
                self.inheritance.geometry.insert((scope, index), rows.into_values().collect());
            }
        }
        Ok(())
    }
    fn master_links(&mut self) -> Result<(), Error> {
        for (&scope, shape) in &self.shapes {
            let mut current = scope;
            let mut instance = None;
            for _ in 0..128 {
                if let Some(master) = self.shapes.get(&current).and_then(|s| s.master) {
                    instance = Some((current, master));
                    break;
                }
                let Some(parent) = self.parents.get(&current).copied() else { break };
                current = parent;
            }
            let Some((root, master_id)) = instance else { continue };
            self.inheritance.instances.insert(scope, (root, master_id));
            let Some(master) = self.document.masters.iter().find(|m| m.id == master_id) else {
                self.inheritance.errors.insert(scope, Error::Ref);
                continue;
            };
            let roots: Vec<_> = master.shapes.iter().filter(|s| !s.sheet.attributes.get("Del").is_some_and(|v| v == "1" || v == "true")).collect();
            let master_shape = shape.master_shape.or_else(|| (root == scope && roots.len() == 1).then(|| roots.first().map(|s| s.id)).flatten());
            if let Some(id) = master_shape {
                let source = Scope::MasterShape { master: master_id, shape: id };
                if self.shapes.contains_key(&source) {
                    self.inheritance.masters.insert(scope, source);
                    if self.inheritance.remapping.insert((root, id), scope).is_some() {
                        return Err(Error::Value);
                    }
                } else {
                    self.inheritance.errors.insert(scope, Error::Ref);
                }
            }
        }
        let roots: BTreeSet<_> = self.inheritance.instances.values().copied().collect();
        for (root, master_id) in roots {
            let Some(master) = self.document.masters.iter().find(|m| m.id == master_id) else { continue };
            let mut pending: Vec<_> = master.shapes.iter().collect();
            while let Some(shape) = pending.pop() {
                if pending.len() >= MAX_SCOPES {
                    return Err(Error::Limit);
                }
                if !self.inheritance.remapping.contains_key(&(root, shape.id)) {
                    let deleted = self.shapes.get(&root).is_some_and(|instance| {
                        let mut stack: Vec<_> = instance.shapes.iter().collect();
                        let mut count = 0;
                        while let Some(s) = stack.pop() {
                            count += 1;
                            if count > MAX_SCOPES {
                                return false;
                            }
                            if s.master_shape == Some(shape.id) && s.sheet.attributes.get("Del").is_some_and(|v| v == "1" || v == "true") {
                                return true;
                            }
                            stack.extend(s.shapes.iter());
                        }
                        false
                    });
                    if !deleted {
                        self.inheritance.omitted_master_children = self.inheritance.omitted_master_children.checked_add(1).ok_or(Error::Limit)?;
                    }
                }
                pending.extend(shape.shapes.iter());
            }
        }
        Ok(())
    }
    fn effective(
        &mut self,
        scope: Scope,
        locals: &BTreeMap<Scope, Vec<NamedCell<'a>>>,
        memo: &mut BTreeMap<Scope, Effective<'a>>,
        visiting: &mut BTreeSet<Scope>,
        count: &mut usize,
        depth: usize,
    ) -> Result<Effective<'a>, Error> {
        if let Some(sheet) = memo.get(&scope) {
            return Ok(sheet.clone());
        }
        if depth > 128 || *count > MAX_CELLS {
            return Err(Error::Limit);
        }
        if !visiting.insert(scope) {
            self.inheritance.errors.insert(scope, Error::Cycle);
            return Ok(Effective::default());
        }
        let mut effective = if let Some(master) = self.inheritance.masters.get(&scope).copied() {
            let e = self.effective(master, locals, memo, visiting, count, depth + 1)?;
            if let Some(error) = self.inheritance.errors.get(&master).cloned() {
                self.inheritance.errors.insert(scope, error);
            }
            e
        } else {
            Effective::default()
        };
        for cat in [Category::Line, Category::Fill, Category::Text] {
            if let Some(style) = self.style_link(scope, cat) {
                let parent = Scope::Style(style);
                let style_sheet = self.effective(parent, locals, memo, visiting, count, depth + 1)?;
                if let Some(error) = self.inheritance.errors.get(&parent).cloned() {
                    self.inheritance.errors.insert(scope, error);
                }
                if !style_enabled(&style_sheet, cat) {
                    continue;
                }
                for (name, entry) in style_sheet.cells {
                    if category(&name) == Some(cat)
                        && self.defaults_allowed(scope, &name)
                        && effective.cells.get(&name).is_none_or(|old| matches!(old.origin, Scope::Style(_) | Scope::Document))
                    {
                        effective.cells.insert(name, entry);
                    }
                }
            }
        }
        if let Some(sheet) = self.sheets.get(&scope).copied() {
            for section in &sheet.sections {
                let section_index = section.index.unwrap_or(0);
                if section.deleted {
                    effective.cells.retain(|_, entry| !same_section(entry.named.section, section));
                    if section.name == "Geometry" {
                        effective.geometry.remove(&section_index);
                    }
                    continue;
                }
                for (position, row) in section.rows.iter().enumerate() {
                    if row.deleted {
                        effective.cells.retain(|_, entry| {
                            !(same_section(entry.named.section, section)
                                && super::section_prefix(section).ok().is_some_and(|prefix| {
                                    entry
                                        .named
                                        .name
                                        .to_ascii_lowercase()
                                        .strip_prefix(&format!("{}.", prefix.to_ascii_lowercase()))
                                        .is_some_and(|tail| row_matches(&section.name, row, position, tail))
                                }))
                        });
                    }
                }
                if section.name == "Geometry" {
                    merge_geometry(&mut effective.geometry, section)?;
                }
            }
        }
        if let Some(names) = locals.get(&scope) {
            for named in names {
                let key = named.name.to_ascii_lowercase();
                if named.cell.formula.as_deref() == Some("Inh") {
                    if let std::collections::btree_map::Entry::Vacant(slot) = effective.cells.entry(key) {
                        if let Some(cell) = super::defaults::cell(scope, &named.name)
                            && !self.inheritance.errors.contains_key(&scope)
                            && self.defaults_allowed(scope, &named.name)
                        {
                            let mut inherited = named.clone();
                            inherited.cell = cell;
                            slot.insert(Entry { named: inherited, origin: Scope::Document });
                        } else {
                            slot.insert(Entry { named: named.clone(), origin: scope });
                        }
                    }
                } else {
                    effective.cells.insert(key, Entry { named: named.clone(), origin: scope });
                }
            }
        }
        *count = count.checked_add(effective.cells.len()).ok_or(Error::Limit)?;
        if *count > MAX_CELLS {
            return Err(Error::Limit);
        }
        visiting.remove(&scope);
        memo.insert(scope, effective.clone());
        Ok(effective)
    }
    /// Tombstones also suppress lazy defaults, including inherited tombstones.
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/735b599d-1359-4767-8593-1c508a885779
    pub(super) fn defaults_allowed(&self, scope: Scope, name: &str) -> bool {
        if self.inheritance.errors.get(&scope) == Some(&Error::Cycle) {
            return false;
        }
        let mut pending = vec![scope];
        let mut seen = BTreeSet::new();
        let lower = name.to_ascii_lowercase();
        while let Some(current) = pending.pop() {
            if !seen.insert(current) {
                continue;
            }
            if seen.len() > 128 {
                return false;
            }
            if let Some(sheet) = self.sheets.get(&current) {
                for section in &sheet.sections {
                    let Ok(prefix) = super::section_prefix(section) else {
                        return false;
                    };
                    let prefix = format!("{}.", prefix.to_ascii_lowercase());
                    let Some(tail) = lower.strip_prefix(&prefix) else {
                        continue;
                    };
                    if section.deleted {
                        return false;
                    }
                    for (position, row) in section.rows.iter().enumerate() {
                        if row_matches(&section.name, row, position, tail) {
                            return !row.deleted;
                        }
                    }
                }
            }
            if let Some(cat) = category(name)
                && let Some(style) = self.style_link(current, cat)
            {
                pending.push(Scope::Style(style));
            }
            if let Some(master) = self.inheritance.masters.get(&current) {
                pending.push(*master);
            }
        }
        true
    }
    fn style_link(&self, scope: Scope, category: Category) -> Option<u32> {
        let name = match category {
            Category::Line => "LineStyle",
            Category::Fill => "FillStyle",
            Category::Text => "TextStyle",
        };
        if let Some(shape) = self.shapes.get(&scope) {
            return match category {
                Category::Line => shape.line_style,
                Category::Fill => shape.fill_style,
                Category::Text => shape.text_style,
            };
        }
        if let Scope::Style(id) = scope {
            let style = self.document.styles.iter().find(|s| s.id == id)?;
            return match category {
                Category::Line => style.line_style,
                Category::Fill => style.fill_style,
                Category::Text => style.text_style,
            };
        }
        self.sheets.get(&scope)?.attributes.get(name)?.parse().ok()
    }
    pub(super) fn formula_origin(&self, scope: Scope, name: &str) -> Option<Scope> {
        let key = self.canonical_key(scope, name)?;
        self.inheritance.origins.get(&key).copied()
    }
    pub(super) fn local_inh_cell(&self, scope: Scope, name: &str) -> Option<&'a Cell> {
        let key = self.canonical_key(scope, name)?;
        self.inheritance.local_cells.get(&key).copied().filter(|c| c.formula.as_deref() == Some("Inh"))
    }
    pub(super) fn inheritance_error(&self, scope: Scope, name: &str) -> Option<Error> {
        if self.cell(scope, name).is_some_and(|c| c.formula.as_deref() != Some("Inh")) {
            return None;
        }
        self.inheritance
            .errors
            .get(&scope)
            .cloned()
            .or_else(|| self.cell(scope, name).filter(|c| c.formula.as_deref() == Some("Inh")).map(|_| Error::Ref))
    }
    pub(super) fn effective_geometry(&self, scope: Scope, index: u32) -> Result<Vec<GeometryRow<'a>>, Error> {
        self.inheritance.geometry.get(&(scope, index)).cloned().ok_or(Error::Ref)
    }
    /// Rebind master-local Sheet IDs only while executing a formula supplied by that master.
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/74428617-9833-4d73-aa7f-f3a6f043a12d
    pub(super) fn remap_master_reference(&self, current: Scope, source: Scope, reference: &str) -> Result<String, Error> {
        if reference.len() > 65536 {
            return Err(Error::Limit);
        }
        let Scope::MasterShape { master, .. } = source else { return Ok(reference.into()) };
        if current == source {
            return Ok(reference.into());
        }
        let Some(&(root, instance_master)) = self.inheritance.instances.get(&current) else { return Ok(reference.into()) };
        if master != instance_master {
            return Ok(reference.into());
        }
        let parts = super::split_reference(reference)?;
        let Some(first) = parts.first() else { return Err(Error::Ref) };
        let first = super::unquote(first);
        let Some((prefix, id)) = first.split_once('.') else { return Ok(reference.into()) };
        if !prefix.eq_ignore_ascii_case("Sheet") {
            return Ok(reference.into());
        }
        let Ok(id) = id.parse::<u32>() else { return Ok(reference.into()) };
        if let Some(target) = self.inheritance.remapping.get(&(root, id)) {
            let number = match target {
                Scope::PageShape { shape, .. } | Scope::MasterShape { shape, .. } => *shape,
                _ => return Err(Error::Ref),
            };
            let tail = parts.get(1..).ok_or(Error::Ref)?.join("!");
            Ok(format!("Sheet.{number}!{tail}"))
        } else if self.shapes.contains_key(&Scope::MasterShape { master, shape: id }) {
            Err(Error::Ref)
        } else {
            Ok(reference.into())
        }
    }
}
fn same_section(original: Option<&Section>, local: &Section) -> bool {
    original.is_some_and(|s| s.name.eq_ignore_ascii_case(&local.name) && s.index.unwrap_or(0) == local.index.unwrap_or(0))
}

fn merge_geometry<'a>(geometry: &mut BTreeMap<u32, BTreeMap<u32, GeometryRow<'a>>>, section: &'a Section) -> Result<(), Error> {
    let rows = geometry.entry(section.index.unwrap_or(0)).or_default();
    for (position, row) in section.rows.iter().enumerate() {
        let index = row.index.unwrap_or(u32::try_from(position).map_err(|_| Error::Limit)?);
        if row.deleted {
            rows.remove(&index);
            continue;
        }
        let effective = rows.entry(index).or_insert(GeometryRow { index, row_type: None, cells: Vec::new() });
        if let Some(kind) = row.row_type.as_deref() {
            effective.row_type = Some(kind);
        }
        for cell in &row.cells {
            if cell.formula.as_deref() == Some("Inh") && effective.cells.iter().any(|c| c.name.eq_ignore_ascii_case(&cell.name)) {
                continue;
            }
            effective.cells.retain(|c| !c.name.eq_ignore_ascii_case(&cell.name));
            effective.cells.push(cell);
        }
        if rows.len() > MAX_CELLS {
            return Err(Error::Limit);
        }
    }
    Ok(())
}

fn row_matches(section: &str, row: &Row, position: usize, tail: &str) -> bool {
    if let Some(name) = &row.name {
        let name = name.to_ascii_lowercase();
        return tail == name || tail.strip_prefix(&name).is_some_and(|s| s.starts_with('.'));
    }
    let index = row.index.unwrap_or(u32::try_from(position).unwrap_or(u32::MAX));
    if section == "Geometry" {
        return tail.trim_start_matches(|c: char| c.is_ascii_alphabetic()).parse::<u32>().ok() == Some(index);
    }
    let Some(one) = index.checked_add(1) else {
        return false;
    };
    if matches!(section, "Character" | "Paragraph" | "Layer" | "FillGradient" | "LineGradient") {
        return if let Some((_, suffix)) = tail.rsplit_once('[') {
            suffix.strip_suffix(']').and_then(|s| s.parse::<u32>().ok()) == Some(one)
        } else {
            index == 0
        };
    }
    tail.trim_start_matches(|c: char| c.is_ascii_alphabetic()).parse::<u32>().ok() == Some(one)
}

#[cfg(test)]
mod tests {
    use super::*;
    use diagramcraft_model::{Document, Master, Page, Shape, Sheet, Style};
    fn c(name: &str, value: &str, formula: Option<&str>) -> Cell {
        Cell { name: name.into(), value: Some(value.into()), formula: formula.map(str::to_owned), ..Default::default() }
    }
    fn sheet(cells: Vec<Cell>) -> Sheet {
        Sheet { cells, ..Default::default() }
    }
    #[test]
    fn instance_formula_origin_and_master_child_rebinding() {
        let mut master_root = Shape { id: 8, sheet: sheet(vec![c("Width", "10", None)]), ..Default::default() };
        master_root.shapes.push(Shape { id: 5, sheet: sheet(vec![c("PinX", "5", Some("Sheet.8!Width*0.5"))]), ..Default::default() });
        let mut root = Shape { id: 349, master: Some(43), sheet: sheet(vec![c("Width", "4", None)]), ..Default::default() };
        root.shapes.push(Shape { id: 351, master_shape: Some(5), sheet: sheet(vec![c("PinX", "2", Some("Inh"))]), ..Default::default() });
        let doc = Document {
            masters: vec![Master { id: 43, shapes: vec![master_root], ..Default::default() }],
            pages: vec![Page { id: 0, shapes: vec![root], ..Default::default() }],
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        let current = Scope::PageShape { page: 0, shape: 351 };
        let source = Scope::MasterShape { master: 43, shape: 5 };
        assert_eq!(e.cell(current, "PinX").unwrap().formula.as_deref(), Some("Sheet.8!Width*0.5"));
        assert_eq!(e.formula_origin(current, "PinX"), Some(source));
        assert_eq!(e.local_inh_cell(current, "PinX").unwrap().value.as_deref(), Some("2"));
        assert_eq!(e.remap_master_reference(current, source, "Sheet.8!Width").unwrap(), "Sheet.349!Width");
        assert_eq!(e.remap_master_reference(current, current, "Sheet.349!Width").unwrap(), "Sheet.349!Width");
        assert_eq!(e.evaluate(current, "PinX"), super::super::Value::Number(2.));
        e.set_formula(Scope::PageShape { page: 0, shape: 349 }, "Width", "8").unwrap();
        assert_eq!(e.evaluate(current, "PinX"), super::super::Value::Number(4.));
        e.set_formula(source, "PinX", "Sheet.8!Width*0.25").unwrap();
        assert_eq!(e.evaluate(current, "PinX"), super::super::Value::Number(2.));
    }
    #[test]
    fn geometry_rows_merge_cells_and_types_and_respect_tombstones() {
        let master_geometry = Section {
            name: "Geometry".into(),
            index: Some(0),
            rows: vec![
                Row { index: Some(1), row_type: Some("MoveTo".into()), cells: vec![c("X", "0", None), c("Y", "0", None)], ..Default::default() },
                Row { index: Some(2), row_type: Some("LineTo".into()), cells: vec![c("X", "2", None), c("Y", "3", None)], ..Default::default() },
                Row { index: Some(3), row_type: Some("LineTo".into()), cells: vec![c("X", "4", None), c("Y", "5", None)], ..Default::default() },
            ],
            ..Default::default()
        };
        let local_geometry = Section {
            name: "Geometry".into(),
            index: Some(0),
            rows: vec![
                Row { index: Some(2), cells: vec![c("X", "7", None)], ..Default::default() },
                Row { index: Some(3), deleted: true, ..Default::default() },
            ],
            ..Default::default()
        };
        let doc = Document {
            masters: vec![Master {
                id: 2,
                shapes: vec![Shape { id: 5, sheet: Sheet { sections: vec![master_geometry], ..Default::default() }, ..Default::default() }],
                ..Default::default()
            }],
            pages: vec![Page {
                id: 0,
                shapes: vec![Shape {
                    id: 1,
                    master: Some(2),
                    sheet: Sheet { sections: vec![local_geometry], ..Default::default() },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 0, shape: 1 };
        let rows = e.effective_geometry(scope, 0).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows.get(1).unwrap().index, 2);
        assert_eq!(rows.get(1).unwrap().row_type, Some("LineTo"));
        assert_eq!(e.cell(scope, "Geometry1.X2").unwrap().value.as_deref(), Some("7"));
        assert_eq!(e.cell(scope, "Geometry1.Y2").unwrap().value.as_deref(), Some("3"));
        assert!(e.cell(scope, "Geometry1.X3").is_none());
        assert!(!e.defaults_allowed(scope, "Geometry1.X3"));
    }
    #[test]
    fn three_style_chains_and_local_master_format_precedence() {
        let styles = vec![
            Style { id: 1, sheet: sheet(vec![c("LineWeight", "1", None), c("FillPattern", "99", None)]), ..Default::default() },
            Style { id: 2, sheet: sheet(vec![c("FillPattern", "2", None), c("LineWeight", "99", None)]), ..Default::default() },
            Style { id: 3, sheet: sheet(vec![c("VerticalAlign", "3", None)]), ..Default::default() },
            Style { id: 4, line_style: Some(1), sheet: sheet(vec![c("LineWeight", "1", Some("Inh"))]), ..Default::default() },
        ];
        let doc = Document {
            styles,
            pages: vec![Page {
                id: 0,
                shapes: vec![Shape { id: 1, line_style: Some(4), fill_style: Some(2), text_style: Some(3), ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 0, shape: 1 };
        for (name, expected) in [("LineWeight", "1"), ("FillPattern", "2"), ("VerticalAlign", "3")] {
            assert_eq!(e.cell(scope, name).unwrap().value.as_deref(), Some(expected));
        }
    }
    #[test]
    fn theme_selectors_follow_actual_style_ancestor() {
        let selectors = [
            "ThemeIndex",
            "ColorSchemeIndex",
            "FontSchemeIndex",
            "EffectSchemeIndex",
            "ConnectorSchemeIndex",
            "VariationColorIndex",
            "VariationStyleIndex",
            "EmbellishmentIndex",
        ];
        let doc = Document {
            styles: vec![
                Style { id: 0, sheet: sheet(selectors.iter().map(|n| c(n, "0", None)).collect()), ..Default::default() },
                Style {
                    id: 6,
                    line_style: Some(0),
                    fill_style: Some(0),
                    text_style: Some(0),
                    sheet: sheet(selectors.iter().map(|n| c(n, "65534", None)).collect()),
                    ..Default::default()
                },
                Style { id: 3, line_style: Some(6), fill_style: Some(6), text_style: Some(6), ..Default::default() },
            ],
            pages: vec![Page {
                id: 0,
                shapes: vec![Shape { id: 1, line_style: Some(3), fill_style: Some(3), text_style: Some(3), ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        for selector in selectors {
            assert_eq!(e.evaluate(Scope::PageShape { page: 0, shape: 1 }, selector), super::super::Value::Number(65534.));
            assert_eq!(e.evaluate(Scope::Style(0), selector), super::super::Value::Number(0.));
        }
    }
    #[test]
    fn style_cycle_errors_and_deleted_character_defaults() {
        let doc = Document {
            styles: vec![
                Style { id: 1, line_style: Some(2), sheet: sheet(vec![c("LineWeight", "0", Some("Inh"))]), ..Default::default() },
                Style { id: 2, line_style: Some(1), sheet: sheet(vec![c("LineWeight", "0", Some("Inh"))]), ..Default::default() },
            ],
            pages: vec![Page {
                id: 0,
                shapes: vec![Shape {
                    id: 1,
                    line_style: Some(1),
                    sheet: Sheet {
                        sections: vec![Section {
                            name: "Character".into(),
                            rows: vec![Row { index: Some(0), deleted: true, ..Default::default() }],
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 0, shape: 1 };
        assert_eq!(e.inheritance_error(scope, "LineWeight"), Some(Error::Cycle));
        assert!(!e.defaults_allowed(scope, "Char.Size"));
    }
    #[test]
    fn named_rows_no_formula_and_master_explicit_format_override() {
        let user = Section {
            name: "User".into(),
            rows: vec![
                Row { name: Some("Keep".into()), cells: vec![c("Value", "7", Some("1+6"))], ..Default::default() },
                Row { name: Some("Remove".into()), cells: vec![c("Value", "9", None)], ..Default::default() },
            ],
            ..Default::default()
        };
        let local = Section {
            name: "User".into(),
            rows: vec![
                Row { name: Some("Keep".into()), cells: vec![c("Value", "4", Some("No Formula"))], ..Default::default() },
                Row { name: Some("Remove".into()), deleted: true, ..Default::default() },
            ],
            ..Default::default()
        };
        let doc = Document {
            styles: vec![Style {
                id: 1,
                sheet: sheet(vec![c("LineWeight", "1", None), c("FillPattern", "2", None), c("EnableFillProps", "0", None)]),
                ..Default::default()
            }],
            masters: vec![Master {
                id: 2,
                shapes: vec![Shape {
                    id: 5,
                    sheet: Sheet { cells: vec![c("LineWeight", "8", None)], sections: vec![user], ..Default::default() },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            pages: vec![Page {
                id: 0,
                shapes: vec![Shape {
                    id: 1,
                    master: Some(2),
                    line_style: Some(1),
                    fill_style: Some(1),
                    sheet: Sheet { sections: vec![local], ..Default::default() },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 0, shape: 1 };
        assert_eq!(e.cell(scope, "LineWeight").unwrap().value.as_deref(), Some("8"));
        assert_eq!(e.cell(scope, "User.Keep").unwrap().formula.as_deref(), Some("No Formula"));
        assert!(e.cell(scope, "User.Remove").is_none());
        assert!(!e.defaults_allowed(scope, "User.Remove"));
        assert_ne!(e.cell(scope, "FillPattern").unwrap().value.as_deref(), Some("2"));
    }
    #[test]
    fn hostile_style_chain_is_bounded() {
        let styles = (0..140).map(|id| Style { id, line_style: (id < 139).then_some(id + 1), ..Default::default() }).collect();
        let doc = Document { styles, ..Default::default() };
        assert!(matches!(Evaluator::new(&doc), Err(Error::Limit)));
    }
    #[test]
    fn corpus_basic_rect_and_probe_inheritance_witnesses() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/generated");
        let file = root.join("master/basic-rect.vsdx");
        if !file.exists() {
            return;
        }
        let doc = crate::read(&file).unwrap();
        let e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: doc.pages.first().unwrap().id, shape: 1 };
        assert_eq!(e.cell(scope, "LocPinX").unwrap().formula.as_deref(), Some("Width*0.5"));
        let file = root.join("basic/0001-probe.vsdx");
        let doc = crate::read(&file).unwrap();
        let e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: doc.pages.first().unwrap().id, shape: 4 };
        assert_eq!(e.cell(scope, "LocPinX").unwrap().formula.as_deref(), Some("GUARD(Width*0.5)"));
        assert!(!e.effective_geometry(scope, 0).unwrap().iter().any(|r| r.index == 3));
    }
}

/// Style inclusion flags are specified as Boolean parse tokens.
/// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/21efe913-d726-4b89-933c-20e8662d90d5
fn style_enabled(sheet: &Effective<'_>, category: Category) -> bool {
    let name = match category {
        Category::Line => "enablelineprops",
        Category::Fill => "enablefillprops",
        Category::Text => "enabletextprops",
    };
    let Some(entry) = sheet.cells.get(name) else {
        return true;
    };
    let cell = entry.named.cell;
    let value = cell.formula.as_deref().filter(|s| *s != "No Formula" && *s != "Inh").or(cell.value.as_deref()).unwrap_or("1");
    !matches!(value.trim().to_ascii_lowercase().as_str(), "0" | "false" | "false()")
}

#[cfg(test)]
mod audit {
    use super::*;
    #[test]
    #[ignore = "full local corpus construction audit, run explicitly for Phase 1"]
    fn all_corpus_inheritance_construction() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
        let mut pending = vec![root.join("generated"), root.join("blank"), root.join("ms-shipped")];
        let mut files = vec![];
        while let Some(path) = pending.pop() {
            if path.is_dir() {
                for entry in std::fs::read_dir(path).unwrap() {
                    pending.push(entry.unwrap().path());
                }
            } else if path.extension().and_then(|s| s.to_str()).is_some_and(|s| matches!(s, "vsdx" | "vssx" | "vstx")) {
                files.push(path);
            }
        }
        files.sort();
        let mut failures = vec![];
        let mut omitted = 0;
        let mut omitted_files = 0;
        for file in &files {
            let document = crate::read(file).unwrap();
            match Evaluator::new(&document) {
                Ok(e) => {
                    let n = e.inheritance.omitted_master_children;
                    omitted += n;
                    if n > 0 {
                        omitted_files += 1;
                        println!("OMITTED {} {}", file.display(), n);
                    }
                }
                Err(error) => failures.push((file.display().to_string(), error)),
            }
        }
        println!("inheritance files={} errors={} omitted_children={} omitted_files={}", files.len(), failures.len(), omitted, omitted_files);
        assert!(failures.is_empty(), "{failures:?}");
    }
}
