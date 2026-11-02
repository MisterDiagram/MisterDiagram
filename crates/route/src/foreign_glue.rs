//! Foreign geometry envelopes used by dynamic glue, observed in Brief26b.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/foreign-image-info-section
#[path = "../../render/src/metafile/header.rs"]
mod metafile_header;
use metafile_header::{Kind, sniff};
use misterdiagram_geom::{Point, Rect};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};

fn number(e: &mut Evaluator<'_>, scope: Scope, name: &str) -> Result<f64, String> {
    let n = e.evaluate(scope, name).numeric().map_err(|v| v.to_string())?;
    if n.is_finite() && n.abs() <= 1e6 { Ok(n) } else { Err("foreign glue coordinate limit".into()) }
}
fn word(bytes: &[u8], offset: usize) -> Result<i32, String> {
    let end = offset.checked_add(4).ok_or("foreign glue header offset")?;
    let value = bytes.get(offset..end).ok_or("truncated foreign glue header")?;
    Ok(i32::from_le_bytes(value.try_into().map_err(|_| "foreign glue header word")?))
}
/// EMR_HEADER Bounds is in device units, Frame in .01 mm, and Device/Millimeters
/// describes the recording device. Convert its declared ink envelope to the
/// picture rectangle before clipping to the Foreign shape.
/// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/e4a35c41-e8e3-43f9-bc07-a18e99bb866d
fn emf_bounds(bytes: &[u8], picture: Rect, shape: Rect) -> Result<Rect, String> {
    if bytes.len() < 88 || word(bytes, 0)? != 1 || word(bytes, 4)? < 88 || sniff(bytes) != Some(Kind::Emf) {
        return Err("invalid foreign glue EMF header".into());
    }
    let values = [8, 12, 16, 20, 24, 28, 32, 36, 72, 76, 80, 84]
        .map(|offset| word(bytes, offset).map(f64::from))
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    let [left, top, right, bottom, fx, fy, fr, fb, dx, dy, mx, my] = values.as_slice() else {
        return Err("foreign glue header fields".into());
    };
    if left > right || top > bottom || fr <= fx || fb <= fy || *dx <= 0. || *dy <= 0. || *mx <= 0. || *my <= 0. {
        return Err("invalid foreign glue EMF bounds".into());
    }
    let x = |v: f64| picture.x0 + picture.width() * (v * mx / dx * 100. - fx) / (fr - fx);
    let y = |v: f64| picture.y1 - picture.height() * (v * my / dy * 100. - fy) / (fb - fy);
    let ink = Rect::new(x(*left), y(*bottom), x(*right), y(*top));
    if [ink.x0, ink.y0, ink.x1, ink.y1].iter().any(|v| !v.is_finite() || v.abs() > 1e6) {
        return Err("foreign glue ink bound limit".into());
    }
    // Visio's EMF hit boundary is just inside the clipping edge; the original
    // full/rotated cases observe .0001 inch while interior ink edges stay exact.
    let ix = 0.0001_f64.min(shape.width() / 2.);
    let iy = 0.0001_f64.min(shape.height() / 2.);
    Ok(ink.intersect(Rect::new(shape.x0 + ix, shape.y0 + iy, shape.x1 - ix, shape.y1 - iy)))
}
fn media<'a>(e: &Evaluator<'a>, mut scope: Scope) -> Result<Option<&'a [u8]>, String> {
    for _ in 0..=128 {
        let shape = e.shape(scope).ok_or("foreign glue shape missing")?;
        if let Some(foreign) = &shape.foreign_data {
            let Some(rel) = foreign.child("Rel") else { return Ok(None) };
            let Some((_, id)) = rel.attributes.iter().find(|(name, _)| {
                name.rsplit(':').next() == Some("id")
                    && rel
                        .attribute_namespaces
                        .get(name)
                        .is_some_and(|ns| ns == "http://schemas.openxmlformats.org/officeDocument/2006/relationships")
            }) else {
                return Ok(None);
            };
            let part = match scope {
                Scope::PageShape { page, .. } => e.document.pages.iter().find(|p| p.id == page).map(|p| &p.part),
                Scope::MasterShape { master, .. } => e.document.masters.iter().find(|m| m.id == master).map(|m| &m.part),
                _ => None,
            }
            .ok_or("foreign glue owner missing")?;
            let target = e
                .document
                .parts
                .get(part)
                .and_then(|p| p.relationships.iter().find(|r| r.id == *id && !r.external))
                .ok_or("foreign glue relationship missing")?;
            return Ok(Some(&e.document.parts.get(&target.target).ok_or("foreign glue media missing")?.bytes));
        }
        let Some(master) = e.master_source(scope) else { return Ok(None) };
        scope = master;
    }
    Err("foreign glue inheritance depth limit".into())
}
/// Keep the ordinary alignment walk for native geometry and bitmap-only groups.
/// EMFs expose an ink envelope; combine their clipped bounds with other leaves.
pub(crate) fn envelopes(e: &mut Evaluator<'_>, page: u32, shape: u32) -> Result<Option<Vec<Rect>>, String> {
    let mut pending = vec![(shape, 0usize)];
    let mut boxes = Vec::new();
    let mut found = false;
    let mut visited = 0usize;
    while let Some((id, depth)) = pending.pop() {
        visited += 1;
        if visited > 16384 || depth > 128 {
            return Err("foreign glue tree limit".into());
        }
        let scope = Scope::PageShape { page, shape: id };
        let s = e.shape(scope).ok_or("foreign glue descendant missing")?;
        if !s.shapes.is_empty() {
            let remaining = 16384usize.saturating_sub(visited);
            if pending.len().checked_add(s.shapes.len()).is_none_or(|count| count > remaining) {
                return Err("foreign glue tree limit".into());
            }
            pending.extend(s.shapes.iter().map(|s| (s.id, depth + 1)));
            continue;
        }
        let width = number(e, scope, "Width")?;
        let height = number(e, scope, "Height")?;
        let alignment = Rect::new(0., 0., width, height);
        let bytes = media(e, scope)?;
        let rect = if let Some(bytes) = bytes.filter(|b| sniff(b).is_some()) {
            found = true;
            let x = number(e, scope, "ImgOffsetX")?;
            let y = number(e, scope, "ImgOffsetY")?;
            let w = number(e, scope, "ImgWidth")?;
            let h = number(e, scope, "ImgHeight")?;
            if w <= 0. || h <= 0. || width <= 0. || height <= 0. {
                continue;
            }
            let picture = Rect::new(x, y, x + w, y + h);
            if bytes.len() > metafile_header::BYTE_LIMIT {
                return Err("metafile byte limit".into());
            }
            match sniff(bytes) {
                Some(Kind::Emf) => emf_bounds(bytes, picture, alignment)?,
                Some(Kind::Wmf) => {
                    let frame = metafile_header::wmf_frame(bytes)?;
                    // Unlike EMF, WMF has no separate device-ink rectangle. Its
                    // logical frame maps to the entire image rectangle.
                    if frame.start > bytes.len() || frame.inch <= 0. || frame.rect.is_some_and(|[l, t, r, b]| l == r || t == b) {
                        return Err("invalid foreign glue WMF frame".into());
                    }
                    let ix = 0.0001_f64.min(width / 2.);
                    let iy = 0.0001_f64.min(height / 2.);
                    picture.intersect(Rect::new(ix, iy, width - ix, height - iy))
                }
                None => alignment,
            }
        } else {
            alignment
        };
        if rect.width() > 0. && rect.height() > 0. {
            let rect = e.shape_to_page(scope).map_err(|v| v.to_string())?.transform_rect_bbox(rect);
            if [rect.x0, rect.y0, rect.x1, rect.y1].iter().any(|v| !v.is_finite() || v.abs() > 1e6) {
                return Err("foreign glue transformed bound limit".into());
            }
            boxes.push(rect);
        }
    }
    Ok((found && !boxes.is_empty()).then_some(boxes))
}
/// Advance a chosen alignment-side point to the nearest ink edge on that side.
pub(crate) fn project(point: Point, alignment: Rect, ink: &[Rect]) -> Point {
    let distances = [(point.x - alignment.x0).abs(), (point.x - alignment.x1).abs(), (point.y - alignment.y0).abs(), (point.y - alignment.y1).abs()];
    let side = distances.iter().enumerate().min_by(|a, b| a.1.total_cmp(b.1)).map_or(0, |(i, _)| i);
    ink.iter()
        .map(|r| match side {
            0 => Point::new(r.x0, point.y.clamp(r.y0, r.y1)),
            1 => Point::new(r.x1, point.y.clamp(r.y0, r.y1)),
            2 => Point::new(point.x.clamp(r.x0, r.x1), r.y0),
            _ => Point::new(point.x.clamp(r.x0, r.x1), r.y1),
        })
        .min_by(|a, b| (*a - point).hypot2().total_cmp(&(*b - point).hypot2()))
        .unwrap_or(point)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hostile_wide_group_is_rejected_before_queue_growth() {
        use misterdiagram_model::{Document, Page, Shape};
        let group = Shape { id: 1, shapes: (2..=16385).map(|id| Shape { id, ..Shape::default() }).collect(), ..Shape::default() };
        let document = Document { pages: vec![Page { id: 1, shapes: vec![group].into(), ..Page::default() }], ..Document::default() };
        let mut evaluator = Evaluator::new(&document).unwrap();
        assert_eq!(envelopes(&mut evaluator, 1, 1), Err("foreign glue tree limit".into()));
    }
    #[test]
    fn hostile_foreign_glue_headers_are_bounded() {
        for length in 0..88 {
            assert!(emf_bounds(&vec![0; length], Rect::new(0., 0., 1., 1.), Rect::new(0., 0., 1., 1.)).is_err());
        }
        let mut bytes = vec![0; 88];
        bytes[0..4].copy_from_slice(&1i32.to_le_bytes());
        bytes[4..8].copy_from_slice(&88i32.to_le_bytes());
        bytes[40..44].copy_from_slice(b" EMF");
        assert!(emf_bounds(&bytes, Rect::new(0., 0., 1., 1.), Rect::new(0., 0., 1., 1.)).is_err());
        assert!(word(&bytes, usize::MAX).is_err());
    }
}

#[cfg(test)]
#[path = "../../render/tests/support/wmf_document.rs"]
mod wmf_document;
#[cfg(test)]
#[path = "../../render/tests/support/wmf.rs"]
mod wmf_recipe;
#[cfg(test)]
mod wmf_tests {
    use super::*;
    #[test]
    fn glue_uses_the_shared_sniff_and_transformed_clipped_wmf_frame() {
        let doc = wmf_document::document(wmf_recipe::rectangle());
        let mut e = Evaluator::new(&doc).unwrap();
        let boxes = envelopes(&mut e, 0, 7).unwrap().unwrap();
        assert_eq!(boxes.len(), 1);
        let b = boxes[0];
        for (got, want) in [b.x0, b.y0, b.x1, b.y1].into_iter().zip([3.25, 2.5, 4.75, 5.5]) {
            assert!((got - want).abs() < 1e-9, "{b:?}");
        }
        assert_eq!(project(Point::new(2., 4.), Rect::new(2., 3., 6., 5.), &boxes), Point::new(3.25, 4.));
        for bad in [vec![], vec![0; 88], b"GIF89a".to_vec()] {
            assert!(sniff(&bad).is_none());
        }
        let mut placeable = wmf_recipe::setup();
        placeable.push(wmf_recipe::record(0x41b, &[720, 1440, 0, 0]));
        let doc = wmf_document::document(wmf_recipe::file(&placeable, Some(([0, 0, 1440, 720], 1440))));
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(envelopes(&mut e, 0, 7).unwrap().unwrap(), boxes);
    }
    #[test]
    fn nested_flipped_wmf_glue_keeps_the_transformed_frame() {
        use misterdiagram_model::{Cell, skeleton};
        let mut doc = wmf_document::document(wmf_recipe::rectangle());
        let mut group = skeleton::rectangle(20, 10., 20., 10., 10., "").unwrap();
        group.shape_type = Some("Group".into());
        group.sheet.sections.clear();
        group.shapes = doc.pages[0].shapes.to_vec();
        group.sheet.cells.push(Cell { name: "FlipX".into(), value: Some("1".into()), ..Default::default() });
        doc.pages[0].shapes = vec![group].into();
        let mut e = Evaluator::new(&doc).unwrap();
        let boxes = envelopes(&mut e, 0, 20).unwrap().unwrap();
        assert_eq!(boxes.len(), 1);
        let b = boxes[0];
        for (got, want) in [b.x0, b.y0, b.x1, b.y1].into_iter().zip([15.25, 22.5, 16.75, 25.5]) {
            assert!((got - want).abs() < 1e-9, "{b:?}");
        }
    }
}
