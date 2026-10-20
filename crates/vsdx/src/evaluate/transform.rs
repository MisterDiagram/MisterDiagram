//! Bounded local/group/page coordinate transforms, independent of rendering.
use super::{Error, Evaluator, Scope};

#[derive(Clone, Copy)]
struct Matrix {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    x: f64,
    y: f64,
}
impl Matrix {
    const ID: Self = Self { a: 1., b: 0., c: 0., d: 1., x: 0., y: 0. };
    fn then(self, parent: Self) -> Result<Self, Error> {
        let result = Self {
            a: parent.a * self.a + parent.c * self.b,
            b: parent.b * self.a + parent.d * self.b,
            c: parent.a * self.c + parent.c * self.d,
            d: parent.b * self.c + parent.d * self.d,
            x: parent.a * self.x + parent.c * self.y + parent.x,
            y: parent.b * self.x + parent.d * self.y + parent.y,
        };
        if [result.a, result.b, result.c, result.d, result.x, result.y].iter().all(|v| v.is_finite()) { Ok(result) } else { Err(Error::Num) }
    }
    fn point(self, (x, y): (f64, f64)) -> Result<(f64, f64), Error> {
        let (x, y) = (self.a * x + self.c * y + self.x, self.b * x + self.d * y + self.y);
        if x.is_finite() && y.is_finite() { Ok((x, y)) } else { Err(Error::Num) }
    }
    fn inverse(self) -> Result<Self, Error> {
        let det = self.a * self.d - self.b * self.c;
        if !det.is_finite() || det.abs() < 1e-15 {
            return Err(Error::Num);
        }
        let (a, b, c, d) = (self.d / det, -self.b / det, -self.c / det, self.a / det);
        Ok(Self { a, b, c, d, x: -a * self.x - c * self.y, y: -b * self.x - d * self.y })
    }
}
impl Evaluator<'_> {
    /// Complete group-child transform in y-up page inches. The renderer flips Y once.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-transform-section
    pub fn shape_to_page(&mut self, scope: Scope) -> Result<misterdiagram_geom::Affine, Error> {
        let (m, _) = self.page_transform(scope)?;
        Ok(misterdiagram_geom::Affine::new([m.a, m.b, m.c, m.d, m.x, m.y]))
    }
    /// Parent coordinates (including one-dimensional endpoints) to page inches.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/beginx-cell-1-d-endpoints-section
    pub fn parent_to_page(&mut self, scope: Scope) -> Result<misterdiagram_geom::Affine, Error> {
        match self.parents.get(&scope).copied() {
            Some(parent) => self.shape_to_page(parent),
            None => Ok(misterdiagram_geom::Affine::IDENTITY),
        }
    }
    /// Local to page conversion uses each enclosing shape's pin, angle and flips.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/loctoloc-function
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/loctopar-function
    pub(super) fn transform_point(
        &mut self,
        current: Scope,
        point: (f64, f64),
        source: Option<&str>,
        source_parent: bool,
        destination: Option<&str>,
        destination_parent: bool,
    ) -> Result<(f64, f64), Error> {
        let source = source.map(|r| self.reference_target(current, r).map(|v| v.0)).transpose()?.unwrap_or(current);
        let destination = destination.map(|r| self.reference_target(current, r).map(|v| v.0)).transpose()?.unwrap_or(current);
        let source = if source_parent { self.parents.get(&source).copied().unwrap_or(source) } else { source };
        let destination = if destination_parent { self.parents.get(&destination).copied().unwrap_or(destination) } else { destination };
        if matches!(source, Scope::Document | Scope::Style(_)) || matches!(destination, Scope::Document | Scope::Style(_)) {
            return Err(Error::Ref);
        }
        if source == destination {
            return Matrix::ID.point(point);
        }
        let (src, src_root) = self.page_transform(source)?;
        let (dst, dst_root) = self.page_transform(destination)?;
        if src_root != dst_root {
            return Err(Error::Ref);
        }
        dst.inverse()?.point(src.point(point)?)
    }
    /// Angle transforms omit pin translations, which cannot change a direction.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/angletopar-function
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/angletoloc-function
    pub(super) fn transform_angle(&mut self, current: Scope, angle: f64, source: &str, destination: &str, parent: bool) -> Result<f64, Error> {
        if !angle.is_finite() {
            return Err(Error::Num);
        }
        let source = self.reference_target(current, source)?.0;
        let destination = self.reference_target(current, destination)?.0;
        let destination = if parent { self.parents.get(&destination).copied().unwrap_or(destination) } else { destination };
        if matches!(source, Scope::Document | Scope::Style(_)) || matches!(destination, Scope::Document | Scope::Style(_)) {
            return Err(Error::Ref);
        }
        if source == destination {
            return Ok(angle.sin().atan2(angle.cos()));
        }
        let (src, src_root) = self.direction_transform(source)?;
        let (dst, dst_root) = self.direction_transform(destination)?;
        if src_root != dst_root {
            return Err(Error::Ref);
        }
        let (sin, cos) = angle.sin_cos();
        let (x, y) = dst.inverse()?.point(src.point((cos, sin))?)?;
        Ok(y.atan2(x))
    }
    fn direction_transform(&mut self, mut scope: Scope) -> Result<(Matrix, Scope), Error> {
        let mut matrix = Matrix::ID;
        for _ in 0..128 {
            let Some(parent) = self.parents.get(&scope).copied() else {
                return if matches!(scope, Scope::Page(_) | Scope::Master(_)) { Ok((matrix, scope)) } else { Err(Error::Ref) };
            };
            let angle = self.resolve(scope, "Angle").numeric()?;
            let fx = if self.resolve(scope, "FlipX").truth()? { -1. } else { 1. };
            let fy = if self.resolve(scope, "FlipY").truth()? { -1. } else { 1. };
            let (sin, cos) = angle.sin_cos();
            matrix = matrix.then(Matrix { a: cos * fx, b: sin * fx, c: -sin * fy, d: cos * fy, x: 0., y: 0. })?;
            scope = parent;
        }
        Err(Error::Limit)
    }
    fn page_transform(&mut self, mut scope: Scope) -> Result<(Matrix, Scope), Error> {
        let mut matrix = Matrix::ID;
        for _ in 0..128 {
            let Some(parent) = self.parents.get(&scope).copied() else {
                return if matches!(scope, Scope::Page(_) | Scope::Master(_)) { Ok((matrix, scope)) } else { Err(Error::Ref) };
            };
            let local = self.local_transform(scope)?;
            matrix = matrix.then(local)?;
            scope = parent;
        }
        Err(Error::Limit)
    }
    /// Rotation is about the local pin, with flips in local axes before rotation.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/pinx-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/piny-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/locpinx-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/locpiny-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/angle-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/flipx-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/flipy-cell-shape-transform-section
    fn local_transform(&mut self, scope: Scope) -> Result<Matrix, Error> {
        let x = self.resolve(scope, "PinX").numeric()?;
        let y = self.resolve(scope, "PinY").numeric()?;
        let lx = self.resolve(scope, "LocPinX").numeric()?;
        let ly = self.resolve(scope, "LocPinY").numeric()?;
        let angle = self.resolve(scope, "Angle").numeric()?;
        let fx = if self.resolve(scope, "FlipX").truth()? { -1. } else { 1. };
        let fy = if self.resolve(scope, "FlipY").truth()? { -1. } else { 1. };
        let (sin, cos) = angle.sin_cos();
        let (a, b, c, d) = (cos * fx, sin * fx, -sin * fy, cos * fy);
        Ok(Matrix { a, b, c, d, x: x - a * lx - c * ly, y: y - b * lx - d * ly })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use misterdiagram_model::{Cell, Document, Page, Shape, Sheet};
    fn shape(id: u32, x: f64, y: f64, angle: f64, flip: bool) -> Shape {
        Shape {
            id,
            sheet: Sheet {
                cells: [
                    ("PinX", x),
                    ("PinY", y),
                    ("LocPinX", 1.),
                    ("LocPinY", 1.),
                    ("Angle", angle),
                    ("FlipX", if flip { 1. } else { 0. }),
                    ("FlipY", 0.),
                ]
                .into_iter()
                .map(|(name, v)| Cell { name: name.into(), value: Some(v.to_string()), ..Default::default() })
                .collect(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
    #[test]
    fn angle_transforms_rotate_flip_and_ignore_translation_cells() {
        let mut group = shape(1, 10., 20., std::f64::consts::FRAC_PI_2, false);
        let mut child = shape(2, 4., 5., 0., true);
        child.sheet.cells.push(Cell { name: "Test".into(), formula: Some("ANGLETOPAR(0deg,Width,ThePage!PageWidth)".into()), ..Default::default() });
        child.sheet.cells.retain(|c| !matches!(c.name.as_str(), "PinX" | "PinY" | "LocPinX" | "LocPinY"));
        group.shapes.push(child);
        let doc = Document {
            pages: vec![Page { id: 0, shapes: vec![group].into(), ..Default::default() }, Page { id: 1, name: "Other".into(), ..Default::default() }],
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 0, shape: 2 };
        let actual = e.evaluate(scope, "Test").numeric().unwrap();
        assert!((actual + std::f64::consts::FRAC_PI_2).abs() < 1e-12);
        let reverse = e.transform_angle(scope, actual, "ThePage!PageWidth", "Width", false).unwrap();
        assert!(reverse.abs() < 1e-12);
        assert_eq!(e.transform_angle(scope, 0., "Width", "Pages[Other]!PageWidth", false), Err(Error::Ref));
        assert_eq!(e.transform_angle(scope, f64::INFINITY, "Width", "Width", false), Err(Error::Num));
    }
    #[test]
    fn nested_rotations_flips_and_page_destination() {
        let mut group = shape(1, 10., 20., std::f64::consts::FRAC_PI_2, false);
        group.shapes.push(shape(2, 4., 5., 0., true));
        let doc = Document {
            pages: vec![Page { id: 0, shapes: vec![group].into(), ..Default::default() }, Page { id: 1, name: "Other".into(), ..Default::default() }],
            ..Default::default()
        };
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 0, shape: 2 };
        let point = e.transform_point(scope, (2., 1.), None, false, Some("ThePage!PageWidth"), true).unwrap();
        assert!((point.0 - 6.).abs() < 1e-12 && (point.1 - 22.).abs() < 1e-12);
        let back = e.transform_point(scope, point, Some("ThePage!PageWidth"), false, None, false).unwrap();
        assert!((back.0 - 2.).abs() < 1e-12 && (back.1 - 1.).abs() < 1e-12);
        assert_eq!(e.transform_point(scope, (1., 1.), None, false, Some("Pages[Other]!PageWidth"), false), Err(Error::Ref));
        assert_eq!(e.transform_point(scope, (f64::INFINITY, 0.), None, false, None, false), Err(Error::Num));
    }
}
