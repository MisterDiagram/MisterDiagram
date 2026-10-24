//! Document binding for the L0 evaluated geometry converter.
use super::{Error, Evaluator, Scope};
use misterdiagram_geom::{GeometryFlags, GeometryPath, GeometryRow, Rect, RowKind};

static ZERO_CELLS: std::sync::LazyLock<Vec<super::Cell>> = std::sync::LazyLock::new(|| {
    ["X", "Y", "A", "B", "C", "D", "E"]
        .into_iter()
        .map(|name| super::Cell { name: name.into(), value: Some("0".into()), ..Default::default() })
        .collect()
});
impl Evaluator<'_> {
    /// Numeric coordinates omitted from an existing geometry row default to zero.
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/47b8caad-1ea2-4119-84c7-4d9901100b03
    pub(super) fn geometry_default(&self, scope: Scope, name: &str) -> Option<&'static super::Cell> {
        if name.len() > 64 {
            return None;
        }
        let lower = name.to_ascii_lowercase();
        let (section, coordinate) = lower.strip_prefix("geometry")?.split_once('.')?;
        let index = section.parse::<u32>().ok()?.checked_sub(1)?;
        let column = coordinate.get(..1)?;
        let row = coordinate.get(1..)?.parse::<u32>().ok()?;
        let kind = self.geometry_row_type(scope, index, row)?;
        let columns = match kind {
            "MoveTo" | "LineTo" | "RelMoveTo" | "RelLineTo" => "xy",
            "ArcTo" | "SplineKnot" => "xya",
            "InfiniteLine" | "RelQuadBezTo" => "xyab",
            "Ellipse" | "EllipticalArcTo" | "RelEllipticalArcTo" | "RelCubBezTo" | "SplineStart" => "xyabcd",
            "NURBSTo" => "xyabcd",
            "PolylineTo" => "xy",
            _ => return None,
        };
        if !columns.contains(column) {
            return None;
        }
        ZERO_CELLS.iter().find(|cell| cell.name.eq_ignore_ascii_case(column))
    }

    /// Resolve inherited rows and formulas before building shape-local paths.
    /// Drawing recovery is opt-in; strict callers still receive every row error.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
    pub fn geometry_paths(&mut self, scope: Scope, clip: Rect) -> Result<Vec<GeometryPath>, Error> {
        let width = self.evaluate(scope, "Width").numeric()?;
        let height = self.evaluate(scope, "Height").numeric()?;
        if self.cached_fallback {
            let connections: Vec<_> =
                self.resolved_named_cells(scope).into_iter().filter(|n| n.name.starts_with("Connections.")).map(|n| n.name).collect();
            for name in connections {
                if let Some(error) = self.evaluate(scope, &name).error() {
                    self.fallback_warning(scope, &name, &error, "skipped Connections row without a usable saved value");
                }
            }
        }
        let indices: Vec<_> = self.geometry_indices(scope).collect();
        let mut paths = Vec::new();
        for index in indices {
            let prefix = format!("Geometry{}", index.checked_add(1).ok_or(Error::Limit)?);
            let mut flag_name = prefix.clone();
            let flags = (|| -> Result<GeometryFlags, Error> {
                let mut flag = |name: &str| -> Result<bool, Error> {
                    let name = format!("{prefix}.{name}");
                    flag_name.clone_from(&name);
                    if self.cell(scope, &name).is_none() { Ok(false) } else { self.evaluate(scope, &name).truth() }
                };
                Ok(GeometryFlags { no_fill: flag("NoFill")?, no_line: flag("NoLine")?, no_show: flag("NoShow")?, no_snap: flag("NoSnap")? })
            })();
            let flags = match flags {
                Ok(flags) => flags,
                Err(error) if self.cached_fallback => {
                    self.fallback_warning(scope, &flag_name, &error, "skipped geometry section without a usable saved flag");
                    continue;
                }
                Err(error) => return Err(error),
            };
            if flags.no_show {
                continue;
            }
            let source = match self.effective_geometry(scope, index) {
                Ok(source) => source,
                Err(error) if self.cached_fallback => {
                    self.fallback_warning(scope, &prefix, &error, "skipped invalid geometry section");
                    continue;
                }
                Err(error) => return Err(error),
            };
            let mut rows = Vec::with_capacity(source.len());
            let mut names = Vec::with_capacity(source.len());
            for source in source {
                let mut attempted_name = format!("{prefix} row {}", source.index);
                let row = (|| -> Result<GeometryRow, Error> {
                    let kind: RowKind = source.row_type.ok_or(Error::Ref)?.parse().map_err(|_| Error::Unsupported("geometry row type".into()))?;
                    let mut cell = |column: &str| {
                        attempted_name = format!("{prefix}.{column}{}", source.index);
                        self.evaluate(scope, &attempted_name)
                    };
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
                        RowKind::PolylineTo => cell("A"),
                        RowKind::NURBSTo => cell("E"),
                        _ => super::Value::Text(String::new()),
                    };
                    if let Some(error) = data.error() {
                        return Err(error);
                    }
                    Ok(GeometryRow { kind, x, y, a, b, c, d, data: data.text() })
                })();
                match row {
                    Ok(row) => {
                        rows.push(row);
                        names.push(source.index);
                    }
                    Err(error) if self.cached_fallback => {
                        self.fallback_warning(scope, &attempted_name, &error, "skipped geometry row without a usable saved value");
                    }
                    Err(error) => return Err(error),
                }
            }
            let result = if self.cached_fallback {
                misterdiagram_geom::geometry_resilient(&rows, width, height, clip, flags, &mut |row, error| {
                    let name = names.get(row).map_or_else(|| prefix.clone(), |index| format!("{prefix} row {index}"));
                    self.fallback_warning(scope, &name, &error, "skipped invalid geometry row");
                })
            } else {
                misterdiagram_geom::geometry(&rows, width, height, clip, flags)
            };
            match result {
                Ok(path) => paths.push(path),
                Err(error) if self.cached_fallback => self.fallback_warning(scope, &prefix, &error, "skipped invalid geometry section"),
                Err(misterdiagram_geom::GeometryError::Limit) => return Err(Error::Limit),
                Err(misterdiagram_geom::GeometryError::Invalid) => return Err(Error::Value),
            }
        }
        Ok(paths)
    }
}
