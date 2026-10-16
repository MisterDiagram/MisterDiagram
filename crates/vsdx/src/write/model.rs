//! Serialize only local sheet overrides, retaining extension children in modelled XML.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
//! https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
//! https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
use super::*;
use crate::evaluate::{Evaluator, Scope, cell_name};
use diagramcraft_sheet::{Expr, Value};
pub(super) struct Writer<'a> {
    pub evaluator: Evaluator<'a>,
    pub source_state: super::source::SourceState,
    pub warnings: Vec<String>,
    pub warning_count: usize,
}
pub(super) fn node(name: &str) -> XmlElement {
    XmlElement { name: name.into(), namespace: diagramcraft_model::skeleton::VISIO_NS.into(), ..Default::default() }
}
pub(super) fn set(e: &mut XmlElement, key: &str, value: Option<String>) {
    if let Some(v) = value {
        e.attributes.insert(key.into(), v);
    } else {
        e.attributes.remove(key);
    }
}
pub(super) fn replace(e: &mut XmlElement, name: &str, items: Vec<XmlElement>) {
    let mut items = Some(items);
    let mut output = Vec::new();
    for c in std::mem::take(&mut e.children) {
        if matches!(&c,XmlNode::Element(e) if e.is_visio(name)) {
            if let Some(v) = items.take() {
                output.extend(v.into_iter().map(XmlNode::Element));
            }
        } else {
            output.push(c);
        }
    }
    if let Some(v) = items {
        output.extend(v.into_iter().map(XmlNode::Element));
    }
    e.children = output;
}
fn base(old: Option<&XmlElement>, name: &str) -> XmlElement {
    old.cloned().unwrap_or_else(|| node(name))
}
impl Writer<'_> {
    fn warn(&mut self, scope: Scope, name: &str, message: impl std::fmt::Display) {
        self.warning_count = self.warning_count.saturating_add(1);
        if self.warnings.len() < 100 {
            self.warnings.push(format!("{scope:?} {name}: {message}; retained last serialized value"));
        }
    }
    /// Preserve Themed sentinels; recalculate concrete formula caches with injected hosts.
    /// An unsupported evaluator result is not proof that Visio's formula has an error.
    /// String literals retain their spelling even when they look like numbers.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/label-cell-shape-data-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    fn cell(&mut self, c: &Cell, name: &str, scope: Scope, old: Option<&XmlElement>) -> XmlElement {
        let mut e = base(old, "Cell");
        e.attributes = c.attributes.clone().into_iter().collect();
        let mut value = c.value.clone();
        let mut formula = c.formula.clone();
        let mut err = c.error.clone();
        let has_changes = !self.source_state.changed.is_empty() || !self.source_state.contexts.is_empty();
        if has_changes
            && c.value.as_deref() != Some("Themed")
            && c.formula.as_deref().is_some_and(|f| !f.is_empty() && !f.eq_ignore_ascii_case("No Formula"))
        {
            let evaluated = self.evaluator.evaluate_snapshot(scope, name);
            if self.evaluator.depends_on_changed(scope, name, &self.source_state.changed, &self.source_state.contexts) {
                match evaluated {
                    Value::Number(n) if n.is_finite() => match if c.name != "TextBkgnd"
                        && !constant_rgb(c.formula.as_deref().unwrap_or(""))
                        && c.value.as_deref().and_then(|v| v.parse::<f64>().ok()) == Some(n)
                        && (0. ..24.).contains(&n)
                        && n.fract() == 0.
                        && Evaluator::baseline_metadata(scope, name).is_some_and(|m| m.3 == 251)
                    {
                        Ok(n.to_string())
                    } else {
                        self.evaluator.serialized_number(scope, name, c, n)
                    } {
                        Ok(v) => {
                            value = Some(v);
                            if constant_rgb(c.formula.as_deref().unwrap_or("")) {
                                formula = None;
                            }
                            err = None;
                        }
                        Err(e) => self.warn(scope, name, e),
                    },
                    Value::Bool(b) => {
                        value = Some(u8::from(b).to_string());
                        err = None;
                    }
                    Value::Text(t) => {
                        value = Some(t);
                        err = None;
                    }
                    Value::Color { rgb, .. } => {
                        // Evaluator palette indices are process-local. Persist concrete RGB,
                        // never an index that is absent from the document's Colors table.
                        value = Some(format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]));
                        if constant_rgb(c.formula.as_deref().unwrap_or("")) {
                            formula = None;
                        }
                        err = None;
                    }
                    Value::Error(error) => self.warn(scope, name, error),
                    _ => self.warn(scope, name, "non-scalar cache"),
                }
            }
        }
        // Literal spelling is already valid V data. Only color and language identities
        // require conversion; parsing other literals destroys numeric-looking strings.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/label-cell-shape-data-section
        // Authored/edited literals also carry a type. Font literals retain their
        // supplied name or numeric identity; neither is an evaluated FONT result.
        // Witness: generated/basic/0001-probe stores Font V="Calibri".
        // https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
        if c.formula.as_deref().is_none_or(|f| f.is_empty() || f.eq_ignore_ascii_case("No Formula"))
            && self.source_state.changed.contains(&(scope, name.to_ascii_lowercase()))
            && c.value.as_deref().is_some_and(|v| !v.is_empty() && v != "Themed")
            && c.error.is_none()
            && c.units.as_deref() != Some("STR")
            && c.name != "Font"
            && (Evaluator::baseline_metadata(scope, name).is_some_and(|m| m.3 == 251) || matches!(c.name.as_str(), "LangID" | "DocLangID"))
        {
            match self.evaluator.cached_cell(c) {
                // Preserve explicit standard palette indexes. Converting 0/1 to RGB changes
                // ResultIU when the drawing also contains a custom black/white entry.
                // Witness: generated/basic/0001-probe.vsdx StyleSheet 0 LineColor=0, FillForegnd=1.
                // https://learn.microsoft.com/en-us/office/client-developer/visio/linecolor-cell-line-format-section
                Value::Number(n)
                    if c.name != "TextBkgnd"
                        && (0. ..24.).contains(&n)
                        && n.fract() == 0.
                        && Evaluator::baseline_metadata(scope, name).is_some_and(|m| m.3 == 251) =>
                {
                    value = Some(n.to_string());
                }
                Value::Number(n) => match self.evaluator.serialized_number(scope, name, c, n) {
                    Ok(v) => value = Some(v),
                    Err(e) => self.warn(scope, name, e),
                },
                Value::Color { rgb, .. } => value = Some(format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])),
                _ => {}
            }
        }
        for (key, v) in [("N", Some(c.name.clone())), ("V", value), ("F", formula), ("U", c.units.clone()), ("E", err)] {
            set(&mut e, key, v);
        }
        replace(&mut e, "RefBy", c.ref_by.clone());
        e
    }
    pub(super) fn sheet(&mut self, s: &Sheet, scope: Scope, name: &str, old: Option<&XmlElement>) -> Result<XmlElement, WriteError> {
        let mut e = base(old, name);
        e.attributes = s.attributes.clone().into_iter().collect();
        let cells = s
            .cells
            .iter()
            .map(|c| {
                self.cell(c, &c.name, scope, old.and_then(|o| o.elements().find(|e| e.is_visio("Cell") && e.attr("N") == Some(c.name.as_str()))))
            })
            .collect();
        replace(&mut e, "Cell", cells);
        let mut sections = vec![];
        for s in &s.sections {
            let old = old.and_then(|o| {
                o.elements().find(|e| {
                    e.is_visio("Section") && e.attr("N") == Some(s.name.as_str()) && e.attr("IX").and_then(|n| n.parse::<u32>().ok()) == s.index
                })
            });
            let mut section = base(old, "Section");
            section.attributes = s.attributes.clone().into_iter().collect();
            set(&mut section, "N", Some(s.name.clone()));
            set(&mut section, "IX", s.index.map(|i| i.to_string()));
            if s.deleted || section.attributes.contains_key("Del") {
                set(&mut section, "Del", Some(u8::from(s.deleted).to_string()));
            }
            let mut cells = vec![];
            for c in &s.cells {
                let name = if s.name == "Geometry" {
                    format!("Geometry{}.{}", s.index.unwrap_or(0).saturating_add(1), c.name)
                } else {
                    format!("{}.{}", s.name, c.name)
                };
                cells.push(self.cell(
                    c,
                    &name,
                    scope,
                    old.and_then(|o| o.elements().find(|e| e.is_visio("Cell") && e.attr("N") == Some(c.name.as_str()))),
                ));
            }
            replace(&mut section, "Cell", cells);
            let mut rows = vec![];
            for (row_position, r) in s.rows.iter().enumerate() {
                let old = old.and_then(|o| {
                    o.elements().find(|e| {
                        e.is_visio("Row") && e.attr("N") == r.name.as_deref() && e.attr("IX").and_then(|n| n.parse::<u32>().ok()) == r.index
                    })
                });
                let mut row = base(old, "Row");
                row.attributes = r.attributes.clone().into_iter().collect();
                set(&mut row, "N", r.name.clone());
                set(&mut row, "IX", r.index.map(|i| i.to_string()));
                set(&mut row, "T", r.row_type.clone());
                if r.deleted || row.attributes.contains_key("Del") {
                    set(&mut row, "Del", Some(u8::from(r.deleted).to_string()));
                }
                let mut cells = vec![];
                for c in &r.cells {
                    let name = cell_name(s, r, row_position, c).map_err(|e| error(e.to_string()))?;
                    cells.push(self.cell(
                        c,
                        &name,
                        scope,
                        old.and_then(|o| o.elements().find(|e| e.is_visio("Cell") && e.attr("N") == Some(c.name.as_str()))),
                    ));
                }
                replace(&mut row, "Cell", cells);
                rows.push(row);
            }
            replace(&mut section, "Row", rows);
            sections.push(section);
        }
        replace(&mut e, "Section", sections);
        replace(&mut e, "Trigger", s.triggers.clone());
        Ok(e)
    }
    /// Shapes retain their own overrides, Text run markers, foreign data and child order.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub(super) fn shapes(&mut self, shapes: &[Shape], parent: Scope, old: Option<&XmlElement>, depth: usize) -> Result<Vec<XmlElement>, WriteError> {
        if depth > 128 {
            return Err(error("shape nesting cap"));
        }
        let mut out = vec![];
        for s in shapes {
            let scope = match parent {
                Scope::Page(p) => Scope::PageShape { page: p, shape: s.id },
                Scope::Master(m) => Scope::MasterShape { master: m, shape: s.id },
                _ => return Err(error("invalid shape parent")),
            };
            let old = old.and_then(|o| {
                o.elements().find(|e| {
                    e.is_visio("Shape")
                        && e.attr("ID").and_then(|s| s.parse::<u32>().ok()) == Some(s.id)
                        && (s.id != u32::MAX || e.attr("MasterShape").and_then(|v| v.parse::<u32>().ok()) == s.master_shape)
                })
            });
            let instance_sheet = self.instance_sheet(&s.sheet, scope, old.is_none())?;
            let mut e = self.sheet(&instance_sheet, scope, "Shape", old)?;
            set(&mut e, "ID", Some(s.id.to_string()));
            set(&mut e, "Type", s.shape_type.clone());
            for (k, v) in [
                ("Master", s.master),
                ("MasterShape", s.master_shape),
                ("LineStyle", s.line_style),
                ("FillStyle", s.fill_style),
                ("TextStyle", s.text_style),
            ] {
                set(&mut e, k, v.map(|n| n.to_string()));
            }
            for (k, v) in [("Name", &s.name), ("NameU", &s.name_u)] {
                set(&mut e, k, (!v.is_empty()).then(|| v.clone()));
            }
            replace(&mut e, "Text", s.text.clone().into_iter().collect());
            replace(&mut e, "ForeignData", s.foreign_data.clone().into_iter().collect());
            let children = self.shapes(&s.shapes, parent, old.and_then(|e| e.child("Shapes")), depth + 1)?;
            if !children.is_empty() {
                let mut container = base(old.and_then(|e| e.child("Shapes")), "Shapes");
                replace(&mut container, "Shape", children);
                replace(&mut e, "Shapes", vec![container]);
            } else {
                replace(&mut e, "Shapes", vec![]);
            }
            out.push(e);
        }
        Ok(out)
    }
    /// Page/master contents and endpoint topology use their typed records.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
    pub(super) fn contents(
        &mut self,
        shapes: &[Shape],
        connects: &[Connect],
        scope: Scope,
        name: &str,
        old: Option<&XmlElement>,
    ) -> Result<XmlElement, WriteError> {
        let mut e = base(old, name);
        let nodes = self.shapes(shapes, scope, old.and_then(|e| e.child("Shapes")), 0)?;
        let mut container = base(old.and_then(|e| e.child("Shapes")), "Shapes");
        replace(&mut container, "Shape", nodes);
        if !shapes.is_empty() || old.is_some_and(|e| e.child("Shapes").is_some()) {
            replace(&mut e, "Shapes", vec![container]);
        }
        let mut container = base(old.and_then(|e| e.child("Connects")), "Connects");
        let mut nodes = vec![];
        for c in connects {
            let mut e = node("Connect");
            e.attributes = c.attributes.clone().into_iter().collect();
            for (k, v) in [
                ("FromSheet", Some(c.from_sheet.to_string())),
                ("ToSheet", Some(c.to_sheet.to_string())),
                ("FromCell", c.from_cell.clone()),
                ("ToCell", c.to_cell.clone()),
                ("FromPart", c.from_part.map(|n| n.to_string())),
                ("ToPart", c.to_part.map(|n| n.to_string())),
            ] {
                set(&mut e, k, v);
            }
            nodes.push(e);
        }
        replace(&mut container, "Connect", nodes);
        if !connects.is_empty() || old.is_some_and(|e| e.child("Connects").is_some()) {
            replace(&mut e, "Connects", vec![container]);
        }
        Ok(e)
    }
}
/// Only a literal RGB call loses F; guarded, dependent and themed colours retain formulas.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/rgb-function
fn constant_rgb(formula: &str) -> bool {
    matches!(diagramcraft_sheet::parse(formula),Ok(Expr::Call{name,args}) if name.eq_ignore_ascii_case("RGB") && args.len()==3 && args.iter().all(|a|matches!(a,Expr::Number(n,None) if n.is_finite() && *n>=0. && *n<=255. && n.fract()==0.)))
}
