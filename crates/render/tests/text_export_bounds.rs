//! Contributor-original text-only raster crop controls.
//! Native witness: field/export-canvas-2026-10-31/native-font-box and native-text.
//! https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
use misterdiagram_model::{Cell, Row, Section, skeleton};
use misterdiagram_render::{
    raster,
    scene::{self, Op, Options},
};
fn drawing(family: &str, text: &str, height: f64, spacing: f64, anchor: f64) -> misterdiagram_model::Document {
    let mut d = skeleton::document(false);
    d.face_names.insert(99, family.into());
    let mut s = skeleton::rectangle(1, 3., 3., 2., height, text).unwrap();
    for (name, value) in [("LinePattern", 0.), ("FillPattern", 0.), ("VerticalAlign", anchor)] {
        s.sheet.cells.retain(|c| c.name != name);
        s.sheet.cells.push(Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() });
    }
    for (name, cells) in
        [("Character", vec![("Font", "99".to_string()), ("Size", (12. / 72.).to_string())]), ("Paragraph", vec![("SpLine", spacing.to_string())])]
    {
        s.sheet.sections.push(Section {
            name: name.into(),
            rows: vec![Row {
                index: Some(0),
                cells: cells.into_iter().map(|(n, v)| Cell { name: n.into(), value: Some(v), ..Default::default() }).collect(),
                ..Default::default()
            }],
            ..Default::default()
        });
    }
    d.pages[0].shapes.push(s);
    d
}
#[test]
fn isolated_occupied_text_has_native_font_cell_extent() {
    let o = Options { scale: 2., ..Default::default() };
    for (font, expected) in [("Arial", (100, 60)), ("Calibri", (87, 63)), ("Times New Roman", (105, 59))] {
        for height in [0.1, 2.] {
            for spacing in [-1., -1.2, -1.8] {
                for anchor in [0., 1., 2.] {
                    for text in ["HILL", "HILL\n"] {
                        let s = scene::page(&drawing(font, text, height, spacing, anchor), 0, o).unwrap();
                        let image = raster::render(&s, o).unwrap();
                        assert_eq!(image.dimensions(), expected, "{font}/{height}/{spacing}/{anchor}/{text:?}");
                    }
                }
            }
        }
    }
}
#[test]
fn occupied_lines_move_with_spacing_and_empty_last_line_adds_no_crop() {
    let o = Options { scale: 2., ..Default::default() };
    for (spacing, height) in [(-1.2, 137), (-1.8, 175)] {
        for text in ["HILL\nHILL\nHILL", "HILL\nHILL\nHILL\n"] {
            let s = scene::page(&drawing("Arial", text, 0.1, spacing, 1.), 0, o).unwrap();
            assert_eq!(raster::render(&s, o).unwrap().dimensions(), (100, height));
        }
    }
}
#[test]
fn hostile_text_extent_is_rejected_without_affecting_glyph_positions() {
    let o = Options { scale: 2., ..Default::default() };
    let mut s = scene::page(&drawing("Arial", "HILL", 0.1, -1.2, 1.), 0, o).unwrap();
    let before = s
        .ops
        .iter()
        .find_map(|op| if let Op::Text { layout, transform, .. } = op { Some((layout.runs[0].glyphs.clone(), *transform)) } else { None })
        .unwrap();
    for op in &mut s.ops {
        if let Op::Text { export_bounds, .. } = op {
            *export_bounds = Some(kurbo::Rect::new(0., 0., f64::NAN, 1.));
        }
    }
    assert!(raster::render(&s, o).is_err());
    let after = s
        .ops
        .iter()
        .find_map(|op| if let Op::Text { layout, transform, .. } = op { Some((layout.runs[0].glyphs.clone(), *transform)) } else { None })
        .unwrap();
    assert_eq!(before, after);
}

#[test]
fn private_field_text_extent_matches_native() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../diagramcraft-corpus/field/vahegan-2026-10-30/in/pg2.vsdx");
    if !path.exists() {
        eprintln!("skip: private field witness absent");
        return;
    }
    let d = misterdiagram_vsdx::read(path).unwrap();
    let o = Options { scale: 2., cached_fallback: true, ..Default::default() };
    let s = scene::page(&d, scene::default_page_index(&d).unwrap(), o).unwrap();
    assert_eq!(raster::render(&s, o).unwrap().dimensions(), (361, 165));
}
