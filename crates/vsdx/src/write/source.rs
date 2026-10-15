//! Source caches and dependency dirtiness for a model-driven save.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
use super::*;

pub(super) type CellKey = (Scope, String);
#[derive(Default)]
pub(super) struct SourceState {
    pub changed: BTreeSet<CellKey>,
    pub contexts: BTreeSet<Scope>,
}
impl SourceState {
    pub fn new(doc: &Document, evaluator: &Evaluator<'_>) -> Result<Self, WriteError> {
        let mut original: BTreeMap<CellKey, std::collections::VecDeque<_>> = BTreeMap::new();
        let mut original_count = 0usize;
        let mut contexts = BTreeSet::new();
        let mut sheets = BTreeMap::new();
        let mut colors_changed = false;
        let mut fonts_changed = false;
        if let Some(root) = source(doc, &doc.main_part).filter(|_| doc.parts.get(&doc.main_part).is_some_and(|p| !p.bytes.is_empty())) {
            let old_colors: BTreeMap<_, _> = root
                .child("Colors")
                .into_iter()
                .flat_map(|e| e.elements())
                .filter(|e| e.is_visio("ColorEntry"))
                .filter_map(|e| Some((e.attr("IX")?.parse::<u32>().ok()?, e.attr("RGB")?.to_string())))
                .collect();
            let old_fonts: Vec<_> = root
                .child("FaceNames")
                .into_iter()
                .flat_map(|e| e.elements())
                .filter(|e| e.is_visio("FaceName"))
                .filter_map(|e| e.attr("NameU"))
                .collect();
            colors_changed = old_colors != doc.colors;
            fonts_changed = old_fonts != doc.face_names.values().map(String::as_str).collect::<Vec<_>>();
            if let Some(sheet) = root.child("DocumentSheet") {
                sheets.insert(Scope::Document, sheet);
            }
            if let Some(styles) = root.child("StyleSheets") {
                for style in styles.elements().filter(|e| e.is_visio("StyleSheet")) {
                    if let Some(id) = style.attr("ID").and_then(|n| n.parse().ok()) {
                        sheets.insert(Scope::Style(id), style);
                    }
                }
            }
        }
        for (kind, path, id) in doc.pages.iter().map(|p| (false, &p.part, p.id)).chain(doc.masters.iter().map(|m| (true, &m.part, m.id))) {
            let parent = if kind { Scope::Master(id) } else { Scope::Page(id) };
            let listing = if kind { "Masters" } else { "Pages" };
            let item = if kind { "Master" } else { "Page" };
            let old = doc
                .parts
                .values()
                .filter(|p| !p.bytes.is_empty())
                .filter_map(|p| p.xml.as_ref())
                .find(|e| e.is_visio(listing))
                .and_then(|e| e.elements().find(|e| e.is_visio(item) && e.attr("ID").and_then(|n| n.parse().ok()) == Some(id)));
            if let Some(sheet) = old.and_then(|e| e.child("PageSheet")) {
                sheets.insert(parent, sheet);
            }
            if let Some(root) = source(doc, path).filter(|_| doc.parts.get(path).is_some_and(|p| !p.bytes.is_empty())) {
                let mut pending = vec![(root, parent)];
                while let Some((e, owner)) = pending.pop() {
                    if let Some(shapes) = e.child("Shapes") {
                        for s in shapes.elements().filter(|s| s.is_visio("Shape")) {
                            let Some(shape) = s.attr("ID").and_then(|n| n.parse().ok()) else { continue };
                            let scope = if kind { Scope::MasterShape { master: id, shape } } else { Scope::PageShape { page: id, shape } };
                            sheets.insert(scope, s);
                            if evaluator.source_parent(scope) != Some(owner) {
                                contexts.insert(scope);
                            }
                            pending.push((s, scope));
                        }
                    }
                }
            }
        }
        let mut name_bytes = 0usize;
        for (scope, e) in &sheets {
            let sheet = crate::parse_sheet(e).map_err(|e| error(e.to_string()))?;
            if let Some(current) = evaluator.sheet(*scope) {
                let metadata = |s: &Sheet| {
                    s.sections
                        .iter()
                        .map(|section| {
                            (
                                section.name.clone(),
                                section.index,
                                section.deleted,
                                section
                                    .rows
                                    .iter()
                                    .map(|r| {
                                        (
                                            r.name.clone(),
                                            r.index,
                                            r.row_type.clone().or_else(|| (section.name == "Connection").then(|| "Connection".into())),
                                            r.deleted,
                                        )
                                    })
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect::<Vec<_>>()
                };
                if metadata(&sheet) != metadata(current) || sheet.attributes != current.attributes {
                    contexts.insert(*scope);
                }
            }
            let mut add = |name: String, c: &Cell| -> Result<(), WriteError> {
                name_bytes = name_bytes.saturating_add(name.len());
                if original_count >= 1_000_000 || name_bytes > 64 * 1024 * 1024 {
                    return Err(error("source cache index limit"));
                }
                original_count += 1;
                original.entry((*scope, name.to_ascii_lowercase())).or_default().push_back((
                    c.value.clone(),
                    c.formula.clone(),
                    c.units.clone(),
                    c.error.clone(),
                ));
                Ok(())
            };
            for c in &sheet.cells {
                add(c.name.clone(), c)?;
            }
            for s in &sheet.sections {
                let prefix = if s.name == "Geometry" { format!("Geometry{}", s.index.unwrap_or(0).saturating_add(1)) } else { s.name.clone() };
                for c in &s.cells {
                    add(format!("{prefix}.{}", c.name), c)?;
                }
                for (i, r) in s.rows.iter().enumerate() {
                    for c in &r.cells {
                        add(crate::evaluate::cell_name(s, r, i, c).map_err(|e| error(e.to_string()))?, c)?;
                    }
                }
            }
            if let Some(s) = evaluator.shape(*scope) {
                let attrs = [
                    ("Master", s.master),
                    ("MasterShape", s.master_shape),
                    ("LineStyle", s.line_style),
                    ("FillStyle", s.fill_style),
                    ("TextStyle", s.text_style),
                ];
                if s.text.as_ref() != e.child("Text") || attrs.iter().any(|(k, v)| e.attr(k).and_then(|v| v.parse::<u32>().ok()) != *v) {
                    contexts.insert(*scope);
                }
            }
        }
        for style in &doc.styles {
            let scope = Scope::Style(style.id);
            if let Some(e) = sheets.get(&scope)
                && [("LineStyle", style.line_style), ("FillStyle", style.fill_style), ("TextStyle", style.text_style)]
                    .iter()
                    .any(|(k, v)| e.attr(k).and_then(|n| n.parse::<u32>().ok()) != *v)
            {
                contexts.insert(scope);
            }
        }
        let mut changed = BTreeSet::new();
        for scope in evaluator.scopes() {
            if !sheets.contains_key(&scope) {
                contexts.insert(scope);
            }
            for n in evaluator.source_named_cells(scope).map_err(|e| error(e.to_string()))? {
                let c = n.cell;
                let key = (scope, n.name.to_ascii_lowercase());
                let same = original
                    .get_mut(&key)
                    .and_then(|values| values.pop_front())
                    .is_some_and(|(v, f, u, err)| v == c.value && f == c.formula && u == c.units && err == c.error);
                let resource_changed = colors_changed && Evaluator::baseline_metadata(scope, &n.name).is_some_and(|m| m.3 == 251)
                    || fonts_changed && c.name == "Font"
                    || c.formula.as_deref().is_some_and(|f| resource_formula(f, colors_changed, fonts_changed));
                if !same || resource_changed {
                    changed.insert(key);
                }
            }
        }
        changed.extend(original.into_iter().filter(|(_, values)| !values.is_empty()).map(|(key, _)| key));
        Ok(Self { changed, contexts })
    }
}

// Inspect parsed calls, never quoted text or coincidentally named references.
fn resource_formula(formula: &str, colors: bool, fonts: bool) -> bool {
    use diagramcraft_sheet::Expr;
    if !colors && !fonts {
        return false;
    }
    let Ok(expr) = diagramcraft_sheet::parse(formula) else { return true };
    let mut pending = vec![&expr];
    let mut count = 0usize;
    while let Some(e) = pending.pop() {
        count += 1;
        if count > 8192 {
            return true;
        }
        match e {
            Expr::Call { name, args } => {
                let name = name.to_ascii_uppercase();
                if (colors || fonts) && (name.starts_with("THEME") || name == "EVALTEXT")
                    || fonts && matches!(name.as_str(), "FONT" | "FONTTOID")
                    || colors
                        && matches!(
                            name.as_str(),
                            "RGB" | "HSL" | "RED" | "GREEN" | "BLUE" | "HUE" | "SAT" | "LUM" | "TINT" | "SHADE" | "MSOTINT" | "MSOSHADE" | "LUMDIFF"
                        )
                {
                    return true;
                }
                pending.extend(args);
            }
            Expr::Unary(_, e) | Expr::Qualified(e, _) => pending.push(e),
            Expr::Binary(_, a, b) => {
                pending.push(a);
                pending.push(b);
            }
            _ => {}
        }
    }
    false
}
