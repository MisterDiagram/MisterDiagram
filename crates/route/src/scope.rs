//! Conservative routing exclusions. Unknown formula effects keep the full closure.
use misterdiagram_model::{Document, Shape, Sheet};

const FORMAT: &[&str] = &["fillforegnd", "fillbkgnd", "fillpattern", "linecolor", "linepattern", "lineweight"];

/// A proven empty route population or an edit to unreferenced presentation/data
/// cells needs no glue hooks. Geometry, connection rows, globals and unknown
/// formula effects retain the full page closure, including obstacle corridors.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/objtype-cell-miscellaneous-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/begtrigger-cell-glue-info-section
pub(crate) fn unaffected(before: &Document, after: &Document) -> bool {
    if before.pages.len() != after.pages.len()
        || before.main_part != after.main_part
        || before.event_list != after.event_list
        || before.root_relationships != after.root_relationships
        || before.master_shortcuts != after.master_shortcuts
        || before.macros_removed != after.macros_removed
        || before.document_sheet != after.document_sheet
        || before.document_settings != after.document_settings
        || before.styles != after.styles
        || before.masters != after.masters
        || before.colors != after.colors
        || before.face_names != after.face_names
        || before.parts != after.parts
    {
        return false;
    }
    let mut changed = Vec::new();
    let mut text = false;
    for (old, new) in before.pages.iter().zip(&after.pages) {
        if old.id != new.id
            || old.name != new.name
            || old.name_u != new.name_u
            || old.part != new.part
            || old.sheet != new.sheet
            || old.connects != new.connects
            || old.attributes != new.attributes
        {
            return false;
        }
        let mut pending = vec![(&old.shapes, &new.shapes)];
        let mut count = 0usize;
        while let Some((old, new)) = pending.pop() {
            if old.len() != new.len() {
                return false;
            }
            for (a, b) in old.iter().zip(new) {
                count += 1;
                if count > 100000 {
                    return false;
                }
                let mut same = a.clone();
                same.sheet = b.sheet.clone();
                same.text = b.text.clone();
                same.shapes = b.shapes.clone();
                if same != *b || !sheet_changes(&a.sheet, &b.sheet, &mut changed) {
                    return false;
                }
                text |= a.text != b.text;
                pending.push((&a.shapes, &b.shapes));
            }
        }
    }
    // A reference anywhere is deliberately wider than the routing dependency graph.
    // Indirect/dynamic references and side effects are never proved independent.
    let mut sheets = vec![&after.document_sheet];
    sheets.extend(after.styles.iter().map(|s| &s.sheet));
    let mut shapes: Vec<&Shape> = Vec::new();
    for page in &after.pages {
        sheets.push(&page.sheet);
        shapes.extend(&page.shapes);
    }
    for master in &after.masters {
        sheets.push(&master.sheet);
        shapes.extend(&master.shapes);
    }
    let mut count = 0usize;
    while let Some(shape) = shapes.pop() {
        count += 1;
        if count > 100000 {
            return false;
        }
        sheets.push(&shape.sheet);
        shapes.extend(&shape.shapes);
    }
    let mut bytes = 0usize;
    for sheet in sheets {
        for cell in sheet.cells.iter().chain(sheet.sections.iter().flat_map(|s| s.cells.iter().chain(s.rows.iter().flat_map(|r| &r.cells)))) {
            let Some(formula) = &cell.formula else { continue };
            bytes = bytes.saturating_add(formula.len());
            if bytes > 64 * 1024 * 1024 {
                return false;
            }
            let f = formula.to_ascii_lowercase();
            if !pure_formula(formula) {
                return false;
            }
            if text && ["text", "char.", "para."].iter().any(|s| f.contains(s)) || changed.iter().any(|name| f.contains(name)) {
                return false;
            }
        }
    }
    true
}
fn sheet_changes(a: &Sheet, b: &Sheet, changed: &mut Vec<String>) -> bool {
    if a.attributes != b.attributes {
        return false;
    }
    for name in FORMAT {
        let old: Vec<_> = a.cells.iter().filter(|c| c.name.eq_ignore_ascii_case(name)).collect();
        let new: Vec<_> = b.cells.iter().filter(|c| c.name.eq_ignore_ascii_case(name)).collect();
        if old != new && !changed.iter().any(|n| n == name) {
            changed.push((*name).into());
        }
    }
    let old_property: Vec<_> = a.sections.iter().filter(|s| s.name == "Property").collect();
    let new_property: Vec<_> = b.sections.iter().filter(|s| s.name == "Property").collect();
    if old_property != new_property && !changed.iter().any(|n| n == "prop.") {
        changed.push("prop.".into());
    }
    let mut old = a.clone();
    let mut new = b.clone();
    for sheet in [&mut old, &mut new] {
        sheet.cells.retain(|c| !FORMAT.iter().any(|n| c.name.eq_ignore_ascii_case(n)));
        sheet.sections.retain(|s| s.name != "Property");
    }
    old == new
}

pub(crate) fn empty_routes(doc: &Document) -> bool {
    if doc.pages.iter().any(|p| !p.connects.is_empty() || !literal_nonconnector(&p.sheet)) {
        return false;
    }
    // Without inheritance and with literal object categories, no cell or implicit
    // formula can turn one of these shapes into a connector during preparation.
    if !literal_nonconnector(&doc.document_sheet) {
        return false;
    }
    if doc.styles.iter().any(|s| !literal_nonconnector(&s.sheet)) {
        return false;
    }
    let mut pending: Vec<_> = doc.pages.iter().flat_map(|p| &p.shapes).collect();
    let mut count = 0usize;
    while let Some(shape) = pending.pop() {
        count += 1;
        if count > 100000 || shape.master.is_some() || shape.master_shape.is_some() {
            return false;
        }
        if !literal_nonconnector(&shape.sheet) {
            return false;
        }
        pending.extend(&shape.shapes);
    }
    true
}

fn literal_nonconnector(sheet: &Sheet) -> bool {
    sheet.cell("ObjType").is_none_or(|c| {
        c.formula.as_deref().is_none_or(|f| matches!(f, "No Formula" | "Inh" | "0" | "1"))
            && c.value.as_deref().and_then(|v| v.parse::<f64>().ok()).is_some_and(|v| v.is_finite() && v != 2.)
    }) && !sheet
        .cells
        .iter()
        .chain(sheet.sections.iter().flat_map(|s| s.cells.iter().chain(s.rows.iter().flat_map(|r| &r.cells))))
        .any(|c| c.formula.as_deref().is_some_and(|f| !pure_formula(f)))
}

fn pure_formula(formula: &str) -> bool {
    use misterdiagram_sheet::parser::{Expr, parse};
    if matches!(formula, "Inh" | "No Formula") {
        return true;
    }
    let Ok(expr) = parse(formula) else {
        return false;
    };
    let mut pending = vec![&expr];
    while let Some(expr) = pending.pop() {
        match expr {
            Expr::Qualified(a, _) | Expr::Unary(_, a) => pending.push(a),
            Expr::Binary(_, a, b) => {
                pending.push(a);
                pending.push(b);
            }
            Expr::Call { name, args } => {
                if !matches!(
                    name.to_ascii_lowercase().as_str(),
                    "guard"
                        | "setatref"
                        | "strsame"
                        | "textwidth"
                        | "textheight"
                        | "shapetext"
                        | "rgb"
                        | "rgbtohsl"
                        | "hsl"
                        | "hsltohsl"
                        | "themeguard"
                        | "themeval"
                        | "if"
                        | "and"
                        | "or"
                        | "not"
                        | "min"
                        | "max"
                        | "abs"
                        | "sin"
                        | "cos"
                        | "tan"
                        | "atan"
                        | "atan2"
                        | "sqrt"
                        | "pow"
                        | "mod"
                        | "int"
                        | "round"
                        | "roundup"
                        | "rounddown"
                        | "pnt"
                        | "pntx"
                        | "pnty"
                        | "par"
                        | "_walkglue"
                        | "_xftrigger"
                        | "true"
                        | "false"
                ) {
                    return false;
                }
                pending.extend(args);
            }
            _ => {}
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use misterdiagram_model::{Cell, skeleton};
    fn drawing() -> Document {
        let mut d = skeleton::document(false);
        d.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 1., 1., "").unwrap());
        d.pages[0].shapes.push(skeleton::rectangle(2, 4., 1., 1., 1., "").unwrap());
        skeleton::glue_dynamic(&mut d.pages[0], 3, 2, [1, 2], [[2., 1.5], [4., 1.5]]).unwrap();
        d
    }
    #[test]
    fn geometry_and_formula_dependencies_keep_the_full_closure() {
        let before = drawing();
        let mut formatted = before.clone();
        formatted.pages[0].shapes[0].sheet.cells.push(Cell { name: "FillForegnd".into(), value: Some("3".into()), ..Default::default() });
        assert!(unaffected(&before, &formatted));
        let mut renamed = before.clone();
        renamed.pages[0].name_u = "Other universal name".into();
        assert!(!unaffected(&before, &renamed));
        let mut retargeted = before.clone();
        retargeted.main_part = "visio/other.xml".into();
        assert!(!unaffected(&before, &retargeted));
        let mut after = before.clone();
        after.pages[0].shapes[0].sheet.cells.push(Cell { name: "PinX".into(), value: Some("7".into()), ..Default::default() });
        assert!(!unaffected(&before, &after));
        let mut after = before.clone();
        after.pages[0].shapes[0].sheet.cells.push(Cell { name: "User.Offset".into(), formula: Some("Sheet.2!PinX".into()), ..Default::default() });
        assert!(!unaffected(&before, &after));
        assert!(!pure_formula("SETF(GETREF(PinX), 3)"));
        assert!(!pure_formula("EVALCELL(Prop.Position)"));
        assert!(!pure_formula("UNKNOWN(1)"));
        assert!(!pure_formula(&"(".repeat(200)));
    }
    #[test]
    fn geometry_validation_precedes_empty_route_exclusion() {
        let mut before = skeleton::document(false);
        before.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 1., 1., "").unwrap());
        let mut after = before.clone();
        after.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "PinX").unwrap().value = Some("NaN".into());
        assert!(crate::GlueHost::prepare_scoped(&before, &mut after, true).is_err());
        let before = after.clone();
        assert!(crate::GlueHost::prepare_scoped(&before, &mut after, false).is_err());
    }
    #[test]
    fn literal_rectangles_have_no_connector_dependents() {
        let mut before = skeleton::document(false);
        before.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 1., 1., "").unwrap());
        let mut after = before.clone();
        after.pages[0].shapes.push(skeleton::rectangle(2, 2., 1., 1., 1., "").unwrap());
        assert!(empty_routes(&before));
        assert!(empty_routes(&after));
        after.pages[0].shapes[0].sheet.cells.push(Cell { name: "ObjType".into(), formula: Some("IF(Prop.Kind,2,1)".into()), ..Default::default() });
        assert!(!empty_routes(&after));
    }
}
