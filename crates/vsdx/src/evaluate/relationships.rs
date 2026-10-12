//! Relationship type 5 identifies the containing list; type 2 is the reverse edge.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/relationships-cell-shape-layout-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/listsheetref-function
use super::{Error, Evaluator, Scope};
use diagramcraft_sheet::Expr;

impl Evaluator<'_> {
    pub(super) fn list_sheet_reference(&mut self, scope: Scope) -> Result<String, Error> {
        if !matches!(scope, Scope::PageShape { .. } | Scope::MasterShape { .. }) {
            return Err(Error::Ref);
        }
        self.add_dependency(scope, "Relationships");
        let formula = self.effective_formula(scope, "Relationships").ok_or(Error::Ref)?;
        let expression = diagramcraft_sheet::parse(&formula).map_err(|e| Error::Parse(e.to_string()))?;
        let local_override = self.canonical_key(scope, "Relationships").is_some_and(|key| self.overrides.contains_key(&key));
        let origin = if local_override { scope } else { self.formula_origin(scope, "Relationships").unwrap_or(scope) };
        let mut pending = vec![&expression];
        let mut target = None;
        while let Some(expression) = pending.pop() {
            if self.eval_work >= 100000 {
                return Err(Error::Limit);
            }
            self.eval_work += 1;
            match expression {
                Expr::Number(_, _) => {}
                Expr::Call { name, args } if name == "SUM" || name == "GUARD" => {
                    if pending.len().saturating_add(args.len()) > 8192 {
                        return Err(Error::Limit);
                    }
                    pending.extend(args);
                }
                Expr::Call { name, args } if name == "DEPENDSON" => {
                    let Some(Expr::Number(kind, None)) = args.first() else { return Err(Error::Ref) };
                    if *kind != 5. {
                        continue;
                    }
                    for argument in args.get(1..).ok_or(Error::Ref)? {
                        if self.eval_work >= 100000 {
                            return Err(Error::Limit);
                        }
                        self.eval_work += 1;
                        let Expr::Call { name, args } = argument else { return Err(Error::Ref) };
                        let Some((prefix, function)) = name.rsplit_once('!') else { return Err(Error::Ref) };
                        if !function.eq_ignore_ascii_case("SHEETREF") || !args.is_empty() {
                            return Err(Error::Ref);
                        }
                        let reference = self.remap_master_reference(scope, origin, &format!("{prefix}!Width"))?;
                        let (candidate, _) = self.reference_target(scope, &reference)?;
                        if candidate == scope || target.is_some_and(|old| old != candidate) {
                            return Err(Error::Ref);
                        }
                        target = Some(candidate);
                    }
                }
                _ => return Err(Error::Unsupported("Relationships expression".into())),
            }
        }
        match target {
            // ThePage anchors the already-rebound ID, so a caller executing an inherited
            // formula cannot accidentally rebind the dynamic reference a second time.
            Some(Scope::PageShape { page, shape }) if matches!(scope, Scope::PageShape { page: p, .. } if page == p) => {
                Ok(format!("ThePage!Sheet.{shape}"))
            }
            Some(Scope::MasterShape { master, shape }) if matches!(scope, Scope::MasterShape { master: m, .. } if master == m) => {
                Ok(format!("ThePage!Sheet.{shape}"))
            }
            _ => Err(Error::Ref),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diagramcraft_model::{Cell, Document, Page, Shape, Sheet};
    use diagramcraft_sheet::Value;
    fn cell(name: &str, value: &str, formula: Option<&str>) -> Cell {
        Cell { name: name.into(), value: Some(value.into()), formula: formula.map(str::to_string), ..Default::default() }
    }
    fn document(relationships: Option<&str>) -> Document {
        Document {
            pages: vec![Page {
                id: 7,
                shapes: vec![
                    Shape {
                        id: 1,
                        sheet: Sheet {
                            cells: vec![cell("Relationships", "0", relationships), cell("Width", "999", Some("IFERROR(LISTSHEETREF()!Height,3)"))],
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Shape { id: 2, sheet: Sheet { cells: vec![cell("Height", "4", None)], ..Default::default() }, ..Default::default() },
                    Shape { id: 3, sheet: Sheet { cells: vec![cell("Height", "5", None)], ..Default::default() }, ..Default::default() },
                ],
                ..Default::default()
            }],
            ..Default::default()
        }
    }
    const MEMBER: Scope = Scope::PageShape { page: 7, shape: 1 };
    #[test]
    fn list_membership_uses_relationships_and_tracks_edits() {
        let doc = document(Some("SUM(DEPENDSON(4,Sheet.3!SheetRef()),DEPENDSON(5,Sheet.2!SheetRef()))"));
        let mut evaluator = Evaluator::new(&doc).unwrap();
        assert_eq!(evaluator.evaluate(MEMBER, "Width"), Value::Number(4.));
        evaluator.set_formula(MEMBER, "Relationships", "SUM(DEPENDSON(5,Sheet.3!SheetRef()))").unwrap();
        assert_eq!(evaluator.evaluate(MEMBER, "Width"), Value::Number(5.));
        evaluator.set_formula(MEMBER, "Sheet.3!Height", "8").unwrap();
        assert_eq!(evaluator.evaluate(MEMBER, "Width"), Value::Number(8.));
    }
    #[test]
    fn missing_ambiguous_and_dangling_lists_fail_without_cached_substitution() {
        for formula in [
            None,
            Some("0"),
            Some("DEPENDSON(2,Sheet.2!SheetRef())"),
            Some("DEPENDSON(5,Sheet.2!SheetRef(),Sheet.3!SheetRef())"),
            Some("DEPENDSON(5,Sheet.9!SheetRef())"),
            Some("DEPENDSON(5,Sheet.1!SheetRef())"),
            Some("DEPENDSON(5,Sheet.2!Width)"),
        ] {
            let doc = document(formula);
            let mut evaluator = Evaluator::new(&doc).unwrap();
            assert_eq!(evaluator.list_sheet_reference(MEMBER), Err(Error::Ref), "{formula:?}");
            assert_eq!(evaluator.evaluate(MEMBER, "Width"), Value::Number(3.), "{formula:?}");
        }
    }
    #[test]
    fn relationship_work_is_bounded() {
        let doc = document(Some("DEPENDSON(5,Sheet.2!SheetRef())"));
        let mut evaluator = Evaluator::new(&doc).unwrap();
        evaluator.eval_work = 100000;
        assert_eq!(evaluator.list_sheet_reference(MEMBER), Err(Error::Limit));
        let doc = document(Some(&format!("{}0{}", "SUM(".repeat(129), ")".repeat(129))));
        let mut evaluator = Evaluator::new(&doc).unwrap();
        assert!(matches!(evaluator.list_sheet_reference(MEMBER), Err(Error::Parse(_))));
    }
    #[test]
    fn inherited_relationships_rebind_once_and_local_edits_use_instance_ids() {
        use diagramcraft_model::Master;
        let master = Shape {
            id: 8,
            sheet: Sheet { cells: vec![cell("Height", "4", None)], ..Default::default() },
            shapes: vec![Shape {
                id: 5,
                sheet: Sheet {
                    cells: vec![
                        cell("Relationships", "0", Some("DEPENDSON(5,Sheet.8!SheetRef())")),
                        cell("Width", "999", Some("LISTSHEETREF()!Height")),
                    ],
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        };
        let instance = Shape {
            id: 349,
            master: Some(43),
            sheet: Sheet { cells: vec![cell("Height", "7", None)], ..Default::default() },
            shapes: vec![Shape { id: 351, master_shape: Some(5), ..Default::default() }],
            ..Default::default()
        };
        let doc = Document {
            masters: vec![Master { id: 43, shapes: vec![master], ..Default::default() }],
            pages: vec![Page { id: 0, shapes: vec![instance], ..Default::default() }],
            ..Default::default()
        };
        let mut evaluator = Evaluator::new(&doc).unwrap();
        let member = Scope::PageShape { page: 0, shape: 351 };
        assert_eq!(evaluator.evaluate(Scope::MasterShape { master: 43, shape: 5 }, "Width"), Value::Number(4.));
        assert_eq!(evaluator.evaluate(member, "Width"), Value::Number(7.));
        evaluator.set_formula(member, "Relationships", "DEPENDSON(5,Sheet.349!SheetRef())").unwrap();
        assert_eq!(evaluator.evaluate(member, "Width"), Value::Number(7.));
    }
}
