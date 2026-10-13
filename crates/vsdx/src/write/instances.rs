//! Instance caches for inherited formulas whose values differ from the master.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
use super::*;
use diagramcraft_sheet::Value;
impl model::Writer<'_> {
    pub(super) fn instance_sheet(&mut self, sheet: &Sheet, scope: Scope) -> Result<Sheet, WriteError> {
        let mut bytes = super::validate::sheet(sheet)?;
        let mut result = sheet.clone();
        let Some(master) = self.evaluator.master_source(scope) else { return Ok(result) };
        let local: BTreeSet<_> =
            self.evaluator.source_named_cells(scope).map_err(|e| error(e.to_string()))?.into_iter().map(|n| n.name.to_ascii_lowercase()).collect();
        let inherited = self.evaluator.named_cells(master).to_vec();
        let mut count =
            sheet.cells.len() + sheet.sections.iter().map(|s| s.cells.len() + s.rows.iter().map(|r| r.cells.len()).sum::<usize>()).sum::<usize>();
        for n in inherited {
            if local.contains(&n.name.to_ascii_lowercase()) {
                continue;
            }
            // Do not materialize formatting styles or deleted rows as master overrides.
            if !matches!(self.evaluator.formula_origin(scope, &n.name), Some(Scope::MasterShape { .. })) {
                continue;
            }
            let value = self.evaluator.evaluate_snapshot(scope, &n.name);
            let source = self.evaluator.local_source_cell(master, &n.name).unwrap_or(n.cell);
            if matches!(value, Value::Error(_)) || source.value.as_deref() == Some("Themed") {
                continue;
            }
            let text_value = matches!(value, Value::Text(_)) || source.units.as_deref() == Some("STR");
            let value = match value {
                Value::Number(v) => match self.evaluator.serialized_number(scope, &n.name, source, v) {
                    Ok(v) => v,
                    Err(_) => continue,
                },
                Value::Bool(v) => u8::from(v).to_string(),
                Value::Text(v) => v,
                Value::Color { rgb, .. } => format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]),
                _ => continue,
            };
            let raw = source.value.as_deref().unwrap_or("");
            let cached = if source.name != "Font" || raw.parse::<u32>().is_err() {
                match self.evaluator.cached_cell(source) {
                    Value::Number(v) => self.evaluator.serialized_number(master, &n.name, source, v).unwrap_or_else(|_| raw.into()),
                    _ => raw.into(),
                }
            } else {
                raw.into()
            };
            let equal = value == cached
                || (!text_value && value.starts_with('#') && value.eq_ignore_ascii_case(&cached))
                || !text_value
                    && match (value.parse::<f64>(), cached.parse::<f64>()) {
                        (Ok(a), Ok(b)) => a.is_finite() && b.is_finite() && (a - b).abs() <= 1e-6,
                        _ => false,
                    };
            if equal {
                continue;
            }
            // Charge before retaining an expanded string or cloning its metadata.
            let metadata_bytes = n.section.map_or(0, |s| s.name.len()).saturating_add(
                n.row.map_or(0, |r| r.name.as_ref().map_or(0, String::len).saturating_add(r.row_type.as_ref().map_or(0, String::len))),
            );
            bytes = bytes
                .saturating_add(value.len())
                .saturating_add(n.cell.name.len())
                .saturating_add(n.cell.units.as_ref().map_or(0, String::len))
                .saturating_add(metadata_bytes)
                .saturating_add(3);
            if bytes > crate::limits::MAX_SHEET_BYTES {
                return Err(error("instance sheet byte limit"));
            }
            count = count.saturating_add(1);
            if count > crate::limits::MAX_SHEET_CELLS {
                return Err(error("instance cell limit"));
            }
            let cell = Cell {
                name: n.cell.name.clone(),
                formula: Some("Inh".into()),
                value: Some(value),
                units: source.units.clone(),
                ..Default::default()
            };
            if let Some(section) = n.section {
                let i = if let Some(i) = result
                    .sections
                    .iter()
                    .position(|s| s.name.eq_ignore_ascii_case(&section.name) && s.index.unwrap_or(0) == section.index.unwrap_or(0))
                {
                    i
                } else {
                    if result.sections.len() >= 16_384 {
                        return Err(error("instance section limit"));
                    }
                    result.sections.push(Section { name: section.name.clone(), index: section.index, ..Default::default() });
                    result.sections.len() - 1
                };
                let dest = result.sections.get_mut(i).ok_or_else(|| error("instance section index"))?;
                if dest.deleted {
                    continue;
                }
                let section_cells = dest.cells.len().saturating_add(dest.rows.iter().map(|r| r.cells.len()).sum::<usize>());
                if section_cells >= crate::limits::MAX_SECTION_CELLS {
                    return Err(error("instance section cell limit"));
                }
                if let Some(row) = n.row {
                    let position = section.rows.iter().position(|r| std::ptr::eq(r, row)).ok_or_else(|| error("inherited row identity"))?;
                    let index = row.index.unwrap_or(u32::try_from(position).map_err(|_| error("inherited row index"))?);
                    let j = if let Some(j) = dest.rows.iter().enumerate().position(|(i, r)| match (&row.name, &r.name) {
                        (Some(a), Some(b)) => a.eq_ignore_ascii_case(b),
                        (None, None) => r.index.unwrap_or(u32::try_from(i).unwrap_or(u32::MAX)) == index,
                        _ => false,
                    }) {
                        j
                    } else {
                        if dest.rows.len() >= crate::limits::MAX_SECTION_ROWS {
                            return Err(error("instance row limit"));
                        }
                        let j = dest
                            .rows
                            .iter()
                            .enumerate()
                            .position(|(i, r)| r.index.unwrap_or(u32::try_from(i).unwrap_or(u32::MAX)) > index)
                            .unwrap_or(dest.rows.len());
                        dest.rows.insert(j, Row { name: row.name.clone(), index: Some(index), row_type: row.row_type.clone(), ..Default::default() });
                        j
                    };
                    let dest = dest.rows.get_mut(j).ok_or_else(|| error("instance row index"))?;
                    if !dest.deleted {
                        dest.cells.push(cell);
                    }
                } else {
                    dest.cells.push(cell);
                }
            } else {
                result.cells.push(cell);
            }
        }
        super::validate::sheet(&result)?;
        Ok(result)
    }
}
