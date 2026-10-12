//! Document binding for the L0 evaluated geometry converter.
use super::{Error, Evaluator, Scope};
use diagramcraft_geom::{GeometryFlags, GeometryPath, GeometryRow, Rect, RowKind};

impl Evaluator<'_> {
    /// Resolve inherited rows and formulas before building shape-local paths.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
    pub fn geometry_paths(&mut self, scope: Scope, clip: Rect) -> Result<Vec<GeometryPath>, Error> {
        let width = self.evaluate(scope, "Width").numeric()?;
        let height = self.evaluate(scope, "Height").numeric()?;
        let indices: Vec<_> = self.geometry_indices(scope).collect();
        let mut paths = Vec::new();
        for index in indices {
            let prefix = format!("Geometry{}", index.checked_add(1).ok_or(Error::Limit)?);
            let mut flag = |name: &str| -> Result<bool, Error> {
                let name = format!("{prefix}.{name}");
                if self.cell(scope, &name).is_none() { Ok(false) } else { self.evaluate(scope, &name).truth() }
            };
            let flags = GeometryFlags { no_fill: flag("NoFill")?, no_line: flag("NoLine")?, no_show: flag("NoShow")?, no_snap: flag("NoSnap")? };
            if flags.no_show {
                continue;
            }
            let source = self.effective_geometry(scope, index)?;
            let mut rows = Vec::with_capacity(source.len());
            for source in source {
                let kind: RowKind = source.row_type.ok_or(Error::Ref)?.parse().map_err(|_| Error::Unsupported("geometry row type".into()))?;
                let mut cell = |column: &str| self.evaluate(scope, &format!("{prefix}.{column}{}", source.index));
                let x = cell("X").numeric()?;
                let y = cell("Y").numeric()?;
                let mut number = |column: &str| -> Result<f64, Error> {
                    if source.cells.iter().any(|c| c.name == column) { cell(column).numeric() } else { Ok(0.) }
                };
                let a = if kind == RowKind::PolylineTo { 0. } else { number("A")? };
                let b = number("B")?;
                let c = number("C")?;
                let d = number("D")?;
                let data = match kind {
                    RowKind::PolylineTo => cell("A").text(),
                    RowKind::NURBSTo => cell("E").text(),
                    _ => String::new(),
                };
                rows.push(GeometryRow { kind, x, y, a, b, c, d, data });
            }
            paths.push(diagramcraft_geom::geometry(&rows, width, height, clip, flags).map_err(|e| match e {
                diagramcraft_geom::GeometryError::Limit => Error::Limit,
                diagramcraft_geom::GeometryError::Invalid => Error::Value,
            })?);
        }
        Ok(paths)
    }
}
