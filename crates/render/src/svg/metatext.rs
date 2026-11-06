//! Editable text preserves record anchors and logical clipping rectangles.
//! https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/dd585d0a-5d7c-4034-963a-1141af836972
use crate::scene::{FontNames, MetaText, TextAnchor};
use misterdiagram_geom::Affine;
use std::fmt::Write as _;
const LIMIT: usize = 256 * 1024 * 1024;
// Display can expand subnormal f64 values to more than 300 decimal characters.
// Reserve 384 per scalar, six scalars per curve, plus path/group markup.
const PATH_ELEMENT_BYTES: usize = 3072;
const RECORD_BYTES: usize = 8192;
pub(super) fn write(out: &mut String, record: &MetaText, view: Affine, id: usize, names: FontNames) -> Result<(), String> {
    write_limited(out, record, view, id, names, LIMIT)
}
fn write_limited(out: &mut String, record: &MetaText, view: Affine, id: usize, names: FontNames, limit: usize) -> Result<(), String> {
    let extra = record
        .text
        .len()
        .saturating_mul(6)
        .saturating_add(record.family.len().saturating_add(record.measured_family.len()).saturating_mul(8))
        .saturating_add(RECORD_BYTES);
    if out.len().saturating_add(extra) > limit {
        return Err("SVG output limit".into());
    }
    let mut groups = 0;
    for (j, (path, even_odd)) in record.dc_clips.iter().enumerate() {
        if out.len().saturating_add(path.elements().len().saturating_mul(PATH_ELEMENT_BYTES)).saturating_add(extra).saturating_add(4096) > limit {
            return Err("SVG output limit".into());
        }
        write!(
            out,
            "<defs><clipPath id=\"mt{id}c{j}\"><path clip-rule=\"{}\" d=\"{}\"/></clipPath></defs><g clip-path=\"url(#mt{id}c{j})\">",
            if *even_odd { "evenodd" } else { "nonzero" },
            (view * path).to_svg()
        )
        .map_err(|e| e.to_string())?;
        groups += 1;
    }
    if let Some((r, m)) = record.clip {
        write!(out,"<defs><clipPath id=\"mt{id}r\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" transform=\"{}\"/></clipPath></defs><g clip-path=\"url(#mt{id}r)\">",r.x0.min(r.x1),r.y0.min(r.y1),r.width().abs(),r.height().abs(),super::matrix(view*m)).map_err(|e|e.to_string())?;
        groups += 1;
    }
    let c = record.color;
    out.push_str("<text font-family=\"");
    super::text::font_family(out, &record.family, &record.measured_family, names);
    write!(out,"\" font-size=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{}\" font-weight=\"{}\" font-style=\"{}\" text-anchor=\"{}\" xml:space=\"preserve\" x=\"0\" y=\"0\" transform=\"{}\">",record.size,c.r,c.g,c.b,f64::from(c.a)/255.,if record.weight>=600{"bold"}else{"normal"},if record.italic{"italic"}else{"normal"},match record.anchor{TextAnchor::Start=>"start",TextAnchor::Middle=>"middle",TextAnchor::End=>"end"},super::matrix(view*record.transform)).map_err(|e|e.to_string())?;
    for (i, fragment) in record.text.split('\r').enumerate() {
        if i > 0 {
            out.push_str("&#13;");
        }
        super::text::escape(out, fragment);
    }
    out.push_str("</text>");
    for _ in 0..groups {
        out.push_str("</g>");
    }
    if out.len() > limit {
        return Err("SVG output limit".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn escaped_record_budget_stops_before_growth() {
        let record = MetaText {
            text: "<&>".repeat(10000),
            family: "Quoted \" face".repeat(100),
            measured_family: String::new(),
            weight: 400,
            italic: false,
            size: 12.,
            transform: Affine::IDENTITY,
            anchor: TextAnchor::Start,
            color: misterdiagram_color::Rgba::BLACK,
            clip: None,
            dc_clips: vec![],
            outline_ops: 0,
            outlines_only: false,
            shape: None,
        };
        let mut out = String::new();
        assert!(write_limited(&mut out, &record, Affine::IDENTITY, 0, FontNames::Fallback, 32768).unwrap_err().contains("limit"));
        assert!(out.is_empty());
    }
    #[test]
    fn subnormal_clip_coordinates_are_budgeted_before_serialization() {
        let tiny = f64::from_bits(1);
        let mut path = kurbo::BezPath::new();
        path.move_to((tiny, tiny));
        for _ in 0..100 {
            path.curve_to((tiny, tiny), (tiny, tiny), (tiny, tiny));
        }
        assert!(path.to_svg().len() > 32768);
        let record = MetaText {
            text: "A".into(),
            family: "Arial".into(),
            measured_family: "Arimo".into(),
            weight: 400,
            italic: false,
            size: 12.,
            transform: Affine::IDENTITY,
            anchor: TextAnchor::Start,
            color: misterdiagram_color::Rgba::BLACK,
            clip: None,
            dc_clips: vec![(path, false)],
            outline_ops: 0,
            outlines_only: false,
            shape: None,
        };
        let mut out = String::new();
        assert!(write_limited(&mut out, &record, Affine::IDENTITY, 0, FontNames::Fallback, 32768).unwrap_err().contains("limit"));
        assert!(out.is_empty());
    }
}
