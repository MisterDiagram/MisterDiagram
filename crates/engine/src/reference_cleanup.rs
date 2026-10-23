//! Preserve values when an authored shape reference loses its owner.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
use crate::{
    Result,
    edit::*,
    operation,
    sheet::{Address, address, install},
};
use misterdiagram_model::{Cell, Document, Page, Shape, Sheet};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use std::collections::{BTreeMap, BTreeSet};

/// Resolve references with the same bounded parser and scope rules as evaluation.
/// Quoted strings are data; explicit references to other pages keep their own scope.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
fn references(e: &Evaluator<'_>, scope: Scope, formula: &str) -> Result<BTreeMap<String, Scope>> {
    if formula.len() > 65536 {
        return Err(operation("formula byte limit"));
    }
    if !formula.contains('!') {
        return Ok(BTreeMap::new());
    }
    let expression = misterdiagram_sheet::parse(formula).map_err(operation)?;
    let mut pending = vec![&expression];
    let mut shapes = BTreeMap::new();
    while let Some(expression) = pending.pop() {
        use misterdiagram_sheet::Expr;
        match expression {
            Expr::Reference(reference) => {
                let (target, _) = e.reference_target(scope, reference).map_err(operation)?;
                if matches!(target, Scope::PageShape { .. } | Scope::MasterShape { .. }) {
                    shapes.insert(reference.clone(), target);
                }
            }
            Expr::Unary(_, child) | Expr::Qualified(child, _) => pending.push(child),
            Expr::Binary(_, a, b) => pending.extend([a.as_ref(), b.as_ref()]),
            Expr::Call { args, .. } => pending.extend(args),
            _ => {}
        }
        if pending.len() > 8192 {
            return Err(operation("reference traversal limit"));
        }
    }
    Ok(shapes)
}
fn referenced_shapes(e: &Evaluator<'_>, scope: Scope, formula: &str) -> Result<BTreeSet<Scope>> {
    Ok(references(e, scope, formula)?.into_values().collect())
}
pub(crate) enum Capture {
    Active(Address),
    Inactive(usize, Cell),
}
pub(crate) fn apply(sheet: &mut Sheet, capture: Capture) -> Result<()> {
    match capture {
        Capture::Active(cell) => install(sheet, cell),
        Capture::Inactive(index, cell) => {
            let target = sheet
                .cells
                .iter_mut()
                .chain(sheet.sections.iter_mut().flat_map(|s| s.cells.iter_mut().chain(s.rows.iter_mut().flat_map(|r| &mut r.cells))))
                .nth(index)
                .ok_or_else(|| operation("inactive cell missing"))?;
            *target = cell;
            Ok(())
        }
    }
}
/// Canonicalize copied internal references before the existing numeric ID remap.
/// The parsed references supply exact token spellings; quoted text is left intact.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
fn canonicalize(e: &Evaluator<'_>, scope: Scope, formula: &str) -> Result<String> {
    let replacements = references(e, scope, formula)?
        .into_iter()
        .filter_map(|(raw, target)| {
            let Scope::PageShape { shape, .. } = target else { return None };
            let (_, cell) = raw.rsplit_once('!')?;
            let canonical = format!("Sheet.{shape}!{cell}");
            (raw != canonical).then_some((raw, canonical))
        })
        .collect::<BTreeMap<_, _>>();
    let boundary = |c: char| c.is_whitespace() || matches!(c, '+' | '-' | '*' | '/' | '^' | '&' | '<' | '>' | '=' | '%' | '(' | ')' | ',' | ';');
    let mut out = String::new();
    let mut at = 0;
    let mut quoted = false;
    let mut previous = None;
    while let Some(tail) = formula.get(at..).filter(|s| !s.is_empty()) {
        if !quoted
            && previous.is_none_or(boundary)
            && let Some((raw, replacement)) = replacements
                .iter()
                .find(|(raw, _)| tail.starts_with(raw.as_str()) && tail.get(raw.len()..).and_then(|s| s.chars().next()).is_none_or(boundary))
        {
            out.push_str(replacement);
            at += raw.len();
            previous = raw.chars().last();
        } else {
            let c = tail.chars().next().ok_or_else(|| operation("formula character missing"))?;
            if c == '"' {
                quoted = !quoted;
            }
            out.push(c);
            at += c.len_utf8();
            previous = Some(c);
        }
        if out.len() > 65536 {
            return Err(operation("rewritten formula limit"));
        }
    }
    Ok(out)
}
fn captures(doc: &Document, e: &mut Evaluator<'_>, scope: Scope, invalid: &impl Fn(Scope) -> bool) -> Result<Vec<Capture>> {
    let mut updates = vec![];
    // Inactive formulas have no evaluated result. Preserve their serialized cache and
    // deletion marker while removing a reference whose owner is being removed.
    for (index, named) in e.source_named_cells(scope).map_err(operation)?.into_iter().enumerate() {
        if !(named.section.is_some_and(|s| s.deleted) || named.row.is_some_and(|r| r.deleted)) {
            continue;
        }
        if let Some(formula) = named.cell.formula.as_deref()
            && referenced_shapes(e, scope, formula)?.into_iter().any(invalid)
        {
            let mut cell = named.cell.clone();
            cell.formula = None;
            cell.ref_by.clear();
            updates.push(Capture::Inactive(index, cell));
        }
        if updates.len() > 100000 {
            return Err(operation("reference cleanup cell limit"));
        }
    }
    for named in e.named_cells(scope).to_vec() {
        let Some(formula) = named.cell.formula.as_deref() else { continue };
        if !referenced_shapes(e, scope, formula)?.into_iter().any(invalid) {
            continue;
        }
        let mut cell = address(doc, scope, &named.name)?;
        let value = if matches!(named.name.as_str(), "BegTrigger" | "EndTrigger") {
            misterdiagram_sheet::Value::Number(0.)
        } else {
            e.evaluate(scope, &named.name)
        };
        if let Some(error) = value.error() {
            return Err(operation(error));
        }
        cell.cell.value = Some(match value {
            misterdiagram_sheet::Value::Color { rgb: [r, g, b], .. } => format!("#{r:02x}{g:02x}{b:02x}"),
            misterdiagram_sheet::Value::Bool(value) => u8::from(value).to_string(),
            misterdiagram_sheet::Value::Number(value) if cell.cell.name == "Font" => {
                if !value.is_finite() || !(0. ..=f64::from(u32::MAX)).contains(&value) || value.fract() != 0. {
                    return Err(operation("invalid captured font identity"));
                }
                e.font_name(value as u32).ok_or_else(|| operation("captured font missing"))?.to_owned()
            }
            misterdiagram_sheet::Value::Number(value) => e.serialized_number(scope, &named.name, &cell.cell, value).map_err(operation)?,
            v => v.text(),
        });
        cell.cell.formula = None;
        cell.cell.error = None;
        cell.cell.ref_by.clear();
        updates.push(Capture::Active(cell));
        if updates.len() > 100000 {
            return Err(operation("reference cleanup cell limit"));
        }
    }
    Ok(updates)
}
/// Literalize references outside a copied subtree in the original evaluation context.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.masters.drop
pub(crate) fn isolate(doc: &Document, page: u32, copy: &mut Shape) -> Result<()> {
    let mut allowed = BTreeSet::new();
    let mut pending = vec![(&*copy, 0usize)];
    while let Some((shape, depth)) = pending.pop() {
        if depth > 128 || allowed.len() >= 100000 {
            return Err(operation("copy shape limit"));
        }
        allowed.insert(shape.id);
        pending.extend(shape.shapes.iter().map(|s| (s, depth + 1)));
    }
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(operation)?;
    let mut e = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    fn visit(doc: &Document, e: &mut Evaluator<'_>, page: u32, shape: &mut Shape, allowed: &BTreeSet<u32>, depth: usize) -> Result<()> {
        if depth > 128 {
            return Err(operation("copy nesting limit"));
        }
        for cell in captures(
            doc,
            e,
            Scope::PageShape { page, shape: shape.id },
            &|target| !matches!(target, Scope::PageShape {page:p,shape:id} if p == page && allowed.contains(&id)),
        )? {
            apply(&mut shape.sheet, cell)?;
        }
        for cell in shape
            .sheet
            .cells
            .iter_mut()
            .chain(shape.sheet.sections.iter_mut().flat_map(|s| s.cells.iter_mut().chain(s.rows.iter_mut().flat_map(|r| &mut r.cells))))
        {
            if let Some(formula) = &mut cell.formula {
                *formula = canonicalize(e, Scope::PageShape { page, shape: shape.id }, formula)?;
            }
        }
        let source = doc.pages.iter().find(|p| p.id == page).ok_or_else(|| operation("source page missing"))?;
        for link in source.connects.iter().filter(|c| c.from_sheet == shape.id && !allowed.contains(&c.to_sheet)) {
            let (prefix, trigger) = match link.from_cell.as_deref() {
                Some("BeginX" | "BeginY") => ("Begin", "BegTrigger"),
                Some("EndX" | "EndY") => ("End", "EndTrigger"),
                _ => continue,
            };
            let scope = Scope::PageShape { page, shape: shape.id };
            for axis in ["X", "Y"] {
                let name = format!("{prefix}{axis}");
                let value = e.evaluate(scope, &name).numeric().map_err(operation)?;
                set_number(&mut shape.sheet, &name, value)?;
            }
            set_number(&mut shape.sheet, trigger, 0.)?;
        }
        for child in &mut shape.shapes {
            visit(doc, e, page, child, allowed, depth + 1)?;
        }
        Ok(())
    }
    visit(doc, &mut e, page, copy, &allowed, 0)
}
/// Capture before ungroup removes a referenced group, including cells outside Connect rows.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.ungroup
pub(crate) fn removing(doc: &Document, page: u32, removed: u32) -> Result<Vec<(Scope, Capture)>> {
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(operation)?;
    let mut e = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    let scopes = e
        .scopes()
        .filter(|scope| {
            matches!(scope, Scope::Page(p) if *p == page) || matches!(scope, Scope::PageShape { page:p,shape } if *p == page && *shape != removed)
        })
        .collect::<Vec<_>>();
    let mut updates = vec![];
    for scope in scopes {
        for cell in captures(doc, &mut e, scope, &|target| target == Scope::PageShape { page, shape: removed })? {
            updates.push((scope, cell));
        }
        if updates.len() > 100000 {
            return Err(operation("reference cleanup cell limit"));
        }
    }
    let source = doc.pages.iter().find(|p| p.id == page).ok_or_else(|| operation("page not found"))?;
    for link in source.connects.iter().filter(|c| c.to_sheet == removed && c.from_sheet != removed) {
        let (prefix, trigger) = match link.from_cell.as_deref() {
            Some("BeginX" | "BeginY") => ("Begin", "BegTrigger"),
            Some("EndX" | "EndY") => ("End", "EndTrigger"),
            _ => continue,
        };
        let scope = Scope::PageShape { page, shape: link.from_sheet };
        for name in [format!("{prefix}X"), format!("{prefix}Y"), trigger.to_owned()] {
            let mut cell = address(doc, scope, &name)?;
            let value = if name == trigger { 0. } else { e.evaluate(scope, &name).numeric().map_err(operation)? };
            cell.cell.value = Some(value.to_string());
            cell.cell.formula = None;
            cell.cell.error = None;
            updates.push((scope, Capture::Active(cell)));
            if updates.len() > 100000 {
                return Err(operation("reference cleanup cell limit"));
            }
        }
    }
    Ok(updates)
}
/// Detaching a moved end removes its rows and resets the corresponding trigger.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
pub(crate) fn detach(page: &mut Page, id: u32, begin: bool, end: bool) -> Result<()> {
    let sheet = &mut shape_mut(&mut page.shapes, id)?.sheet;
    if begin {
        set_number(sheet, "BegTrigger", 0.)?;
    }
    if end {
        set_number(sheet, "EndTrigger", 0.)?;
    }
    page.connects.retain(|c| {
        c.from_sheet != id
            || !matches!(c.from_cell.as_deref(), Some("BeginX" | "BeginY") if begin)
                && !matches!(c.from_cell.as_deref(), Some("EndX" | "EndY") if end)
    });
    Ok(())
}
/// Replacing an end with a literal also breaks its glue, as Selection.Move does in W1.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/beginx-cell-1-d-endpoints-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/endx-cell-1-d-endpoints-section
pub(crate) fn move_ends(page: &mut Page, id: u32, old: [f64; 4], new: [f64; 4]) -> Result<()> {
    let [bx, by, ex, ey] = new;
    let [ox, oy, px, py] = old;
    let begin = bx != ox || by != oy;
    let end = ex != px || ey != py;
    let sheet = &mut shape_mut(&mut page.shapes, id)?.sheet;
    if begin {
        set_number(sheet, "BeginX", bx)?;
        set_number(sheet, "BeginY", by)?;
    }
    if end {
        set_number(sheet, "EndX", ex)?;
        set_number(sheet, "EndY", ey)?;
    }
    if begin || end {
        detach(page, id, begin, end)?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reference_scan_is_quoted_scoped_and_bounded() {
        let mut doc = misterdiagram_model::skeleton::document(false);
        doc.pages[0].shapes = [1, 3].map(|id| misterdiagram_model::skeleton::rectangle(id, 0., 0., 1., 1., "").unwrap()).to_vec().into();
        let page = doc.pages[0].id;
        let scope = Scope::PageShape { page, shape: 1 };
        let e = Evaluator::new(&doc).unwrap();
        assert_eq!(
            referenced_shapes(&e, scope, "Sheet.1!Width+LEN(\"a\"\"Sheet.2!Width\")+sheet.3!Height").unwrap(),
            BTreeSet::from([scope, Scope::PageShape { page, shape: 3 }])
        );
        assert!(referenced_shapes(&e, scope, &"x".repeat(65537)).is_err());
        assert!(referenced_shapes(&e, scope, "Sheet.99999999999999999!Width").is_err());
    }
}

#[cfg(test)]
mod theme_font_tests {
    use super::*;
    use misterdiagram_model::{Part, Row, Section, skeleton};

    /// Freezing a theme-only font reference retains its face rather than a process-local ID.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/font-cell-character-section
    #[test]
    fn copied_and_removed_theme_font_references_keep_the_face_name() {
        let mut document = skeleton::document(false);
        let xml = b"<theme><fontScheme><minorFont><latin typeface='Contributor Theme Face'/></minorFont></fontScheme></theme>";
        document.parts.insert("visio/theme/theme1.xml".into(), Part { bytes: xml.to_vec().into(), ..Default::default() });
        for (id, formula) in [(1, "FONT(\"Contributor Theme Face\")"), (2, "Sheet.1!Char.Font")] {
            let mut shape = skeleton::rectangle(id, 1., 1., 2., 1., "Label").unwrap();
            shape.sheet.sections.push(Section {
                name: "Character".into(),
                rows: vec![Row {
                    index: Some(0),
                    cells: vec![Cell { name: "Font".into(), formula: Some(formula.into()), value: Some("0".into()), ..Default::default() }],
                    ..Default::default()
                }],
                ..Default::default()
            });
            document.pages[0].shapes.push(shape);
        }
        let mut copied = document.pages[0].shapes[1].clone();
        isolate(&document, 0, &mut copied).unwrap();
        let font = copied.sheet.sections.iter().find(|s| s.name == "Character").unwrap().rows[0].cells.iter().find(|c| c.name == "Font").unwrap();
        assert_eq!(font.value.as_deref(), Some("Contributor Theme Face"));
        assert!(font.formula.is_none());
        let captured = removing(&document, 0, 1).unwrap();
        assert!(captured.iter().any(|(scope, capture)| matches!(
            (scope, capture),
            (Scope::PageShape { page: 0, shape: 2 }, Capture::Active(address))
                if address.cell.name == "Font" && address.cell.value.as_deref() == Some("Contributor Theme Face") && address.cell.formula.is_none()
        )));
        assert!(!document.face_names.values().any(|name| name == "Contributor Theme Face"));
    }
}
