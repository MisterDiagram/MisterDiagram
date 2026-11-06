//! Original synthetic text records protect editability and unchanged legacy glyphs.
//! https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/dd585d0a-5d7c-4034-963a-1141af836972
#[path = "support/wmf_document.rs"]
mod picture;
#[path = "support/emf_text.rs"]
mod recipe;
#[path = "support/wmf.rs"]
mod wmf;
use misterdiagram_render::{
    metafile, raster,
    scene::{self, Op, Options},
    svg,
};
fn document(bytes: Vec<u8>) -> misterdiagram_model::Document {
    let mut doc = picture::document(bytes);
    doc.pages[0].shapes[0]
        .foreign_data
        .as_mut()
        .unwrap()
        .attributes
        .insert("xmlns:r".into(), "http://schemas.openxmlformats.org/officeDocument/2006/relationships".into());
    let bytes = misterdiagram_vsdx::write(&doc).unwrap().bytes;
    misterdiagram_vsdx::import(&bytes).unwrap()
}
fn legacy(mut s: scene::Scene) -> scene::Scene {
    s.ops.retain(|op| !matches!(op, Op::MetaText(_)));
    s
}
fn strings(xml: &str) -> Vec<String> {
    let doc = resvg::usvg::roxmltree::Document::parse(xml).unwrap();
    doc.descendants().filter(|n| n.has_tag_name("text")).map(|n| n.descendants().filter(|n| n.is_text()).filter_map(|n| n.text()).collect()).collect()
}
#[test]
fn metafile_records_are_searchable_clipped_and_symbol_safe() {
    let s = scene::page(&document(recipe::fixture()), 0, Options::default()).unwrap();
    assert!(s.warnings.is_empty());
    let records: Vec<_> = s.ops.iter().filter_map(|op| if let Op::MetaText(r) = op { Some(r) } else { None }).collect();
    assert_eq!(records.len(), 4);
    assert_eq!(records.iter().map(|r| r.outlines_only).collect::<Vec<_>>(), [false, false, true, true]);
    let xml = svg::render(&s, Options::default()).unwrap();
    assert_eq!(strings(&xml), ["Searchable 144F", "Centred"]);
    let doc = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
    let center = doc.descendants().find(|n| n.has_tag_name("text") && n.text() == Some("Centred")).unwrap();
    assert_eq!(center.attribute("text-anchor"), Some("middle"));
    assert_eq!(center.attribute("font-weight"), Some("bold"));
    assert!(center.ancestors().any(|n| n.attribute("clip-path").is_some()));
    assert!(doc.descendants().any(|n| n.has_tag_name("clipPath") && n.children().any(|n| n.has_tag_name("rect"))));
    assert!(xml.contains("Arimo"));
    let exact = svg::render(&s, Options { font_names: scene::FontNames::Exact, ..Default::default() }).unwrap();
    assert!(!exact.contains("Arimo"));
    assert!(xml.matches("<path").count() >= 4);
    assert_eq!(svg::warnings(&s, Options::default()), ["shape 7: 2 metafile text records kept as outlines (symbol font or glyph indexes)"]);
    assert!(svg::warnings(&s, Options { text_outlines: true, ..Default::default() }).is_empty());
    let options = Options { text_outlines: true, ..Default::default() };
    assert_eq!(svg::render(&s, options).unwrap(), svg::render(&legacy(s.clone()), options).unwrap());
    let image = raster::render(&s, Options::default()).unwrap();
    let mut png = std::io::Cursor::new(Vec::new());
    image.write_to(&mut png, image::ImageFormat::Png).unwrap();
    let hash = png.get_ref().iter().fold(0xcbf29ce484222325u64, |h, b| (h ^ u64::from(*b)).wrapping_mul(0x100000001b3));
    // Captured from the same saved original fixture using the base release binary.
    assert_eq!(hash, 0x15d1f20452beb3f0);
    assert_eq!(image, raster::render(&legacy(s.clone()), Options::default()).unwrap());
    assert_eq!(
        misterdiagram_render::pdf::render(std::slice::from_ref(&s), Options::default()).unwrap(),
        misterdiagram_render::pdf::render(&[legacy(s)], Options::default()).unwrap()
    );
}
#[test]
fn metafile_text_spans_and_numbers_are_hostile_input() {
    let s = scene::page(&document(recipe::fixture()), 0, Options::default()).unwrap();
    let index = s.ops.iter().position(|op| matches!(op, Op::MetaText(_))).unwrap();
    for kind in 0..9 {
        let mut bad = s.clone();
        let count = bad.ops.len();
        let Op::MetaText(r) = &mut bad.ops[index] else { panic!("record") };
        match kind {
            0 => r.outline_ops = usize::MAX,
            1 => r.size = f64::NAN,
            2 => r.transform = kurbo::Affine::scale(f64::INFINITY),
            3 => r.clip = Some((kurbo::Rect::new(0., 0., f64::NAN, 1.), kurbo::Affine::IDENTITY)),
            4 => r.outline_ops = count,
            5 => bad.ops[index + 1] = Op::Pop,
            6 => bad.ops[index + 1] = bad.ops[index].clone(),
            7 => {
                r.transform = kurbo::Affine::scale(1e308);
                r.outline_ops = 0;
            }
            _ => {
                r.outline_ops = 1;
                bad.ops[index + 1] = Op::Clip { path: kurbo::BezPath::new(), even_odd: false };
            }
        }
        assert!(svg::render(&bad, Options::default()).is_err(), "{kind}");
        assert!(raster::render(&bad, Options::default()).is_err(), "{kind}");
    }
    assert!(scene::SceneBudget::new(1).charge_ops(&s.ops).is_err());
    let bytes = recipe::fixture();
    for end in 0..bytes.len() {
        assert!(std::panic::catch_unwind(|| metafile::import(&bytes[..end])).is_ok());
    }
}
#[test]
fn metafile_zero_ink_and_escaped_records_keep_one_element_each() {
    let records = [
        recipe::font(1, 400, 0),
        recipe::record(37, &1u32.to_le_bytes()),
        recipe::text((20, 20), &" <&>\"' \0\r\n".encode_utf16().collect::<Vec<_>>(), 0),
        recipe::text((20, 40), &[32, 32], 0),
        recipe::text((20, 60), &[0xe000], 0),
    ];
    let s = scene::page(&document(recipe::file(&records)), 0, Options::default()).unwrap();
    let xml = svg::render(&s, Options::default()).unwrap();
    assert_eq!(strings(&xml), [" <&>\"' \u{fffd}\r\n", "  "]);
    assert_eq!(svg::warnings(&s, Options::default()).len(), 1);
    let metadata: Vec<_> = s.ops.iter().filter_map(|op| if let Op::MetaText(r) = op { Some(r) } else { None }).collect();
    assert_eq!(metadata[1].outline_ops, 0);
    assert!(metadata[2].outlines_only);
}
#[test]
fn wmf_text_and_extended_rectangles_keep_charset_and_padding() {
    let mut font = vec![0; 50];
    font[0..2].copy_from_slice(&(-16i16).to_le_bytes());
    font[8..10].copy_from_slice(&400i16.to_le_bytes());
    font[18..23].copy_from_slice(b"Arial");
    let mut records = wmf::setup();
    records.push(wmf::packed(0x02fb, &font));
    records.push(wmf::record(0x012d, &[2]));
    // setup creates a pen and brush, so the font takes handle2.
    let mut plain = 3u16.to_le_bytes().to_vec();
    plain.extend(b"odd\0");
    plain.extend([20i16, 20].into_iter().flat_map(i16::to_le_bytes));
    records.push(wmf::packed(0x0521, &plain));
    for opts in [0i16, 2, 4, 6] {
        let mut body = [50i16, 20, 3, opts].into_iter().flat_map(i16::to_le_bytes).collect::<Vec<_>>();
        if opts & 6 != 0 {
            body.extend([11i16, 22, 333, 444].into_iter().flat_map(i16::to_le_bytes));
        }
        body.extend(b"abc\0");
        records.push(wmf::packed(0x0a32, &body));
    }
    let imported = metafile::import(&wmf::file(&records, None)).unwrap();
    let metadata: Vec<_> = imported.ops.iter().filter_map(|op| if let Op::MetaText(r) = op { Some(r) } else { None }).collect();
    assert_eq!(metadata.len(), 5);
    assert_eq!(metadata[0].text, "odd");
    for (r, clipped) in metadata.iter().skip(1).zip([false, false, true, true]) {
        assert_eq!(r.clip.is_some(), clipped);
        if let Some((rect, _)) = r.clip {
            assert_eq!(rect, kurbo::Rect::new(11., 22., 333., 444.));
        }
    }
    font[13] = 2;
    let mut records = wmf::setup();
    records.push(wmf::packed(0x02fb, &font));
    records.push(wmf::record(0x012d, &[2]));
    records.push(wmf::packed(0x0521, &plain));
    let imported = metafile::import(&wmf::file(&records, None)).unwrap();
    assert!(imported.ops.iter().any(|op| matches!(op,Op::MetaText(r) if r.outlines_only)));
}

#[test]
fn emf_charset_neighbors_ansi_and_record_clips_are_distinct() {
    for offset in [33usize, 34, 35, 36, 37, 38, 39] {
        let mut font = recipe::font(1, 400, 0);
        font[offset] = 2;
        let mut text = recipe::text((20, 30), &[65, 66], 4);
        // EMR_EXTTEXTOUTA uses bytes at the same record offset as the wide form.
        text[0..4].copy_from_slice(&83u32.to_le_bytes());
        text[76..78].copy_from_slice(b"AB");
        let bytes = recipe::file(&[font, recipe::record(37, &1u32.to_le_bytes()), text]);
        let imported = metafile::import(&bytes).unwrap();
        let record = imported.ops.iter().find_map(|op| if let Op::MetaText(r) = op { Some(r) } else { None }).unwrap();
        assert_eq!(record.text, "AB");
        assert_eq!(record.outlines_only, offset == 35);
        assert_eq!(record.clip.unwrap().0, kurbo::Rect::new(20., 40., 360., 80.));
    }
}

/// Decode the replacement SVG instead of consulting its scene metadata. Reconstruct
/// the same font's glyph geometry in SVG coordinates and compare every control point
/// with the independently retained drawing stream.
fn assert_svg_placement_matches_retained_glyphs(s: &scene::Scene, xml: &str) {
    use kurbo::{Affine, PathEl};
    let parsed = resvg::usvg::roxmltree::Document::parse(xml).unwrap();
    let text = parsed.descendants().find(|n| n.has_tag_name("text")).unwrap();
    assert!(text.attribute("font-family").unwrap().starts_with("'Arial'"));
    let size = text.attribute("font-size").unwrap().parse::<f64>().unwrap();
    let transform: Vec<f64> = text
        .attribute("transform")
        .unwrap()
        .strip_prefix("matrix(")
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let transform = Affine::new(transform.try_into().unwrap());
    let db = misterdiagram_fonts::FontDb::global();
    let face = db.face("Arial", "Regular");
    let glyphs = misterdiagram_fonts::shape(&face, text.text().unwrap(), &[], |c| c);
    let unit = size / face.upem;
    let advance = glyphs.iter().map(|g| f64::from(g.x_advance)).sum::<f64>() * unit;
    let anchor = match text.attribute("text-anchor").unwrap() {
        "start" => 0.,
        "middle" => -advance / 2.,
        "end" => -advance,
        other => panic!("{other}"),
    };
    let view = Affine::new([72., 0., 0., -72., 0., s.page_size.1 * 72.]);
    let index = s.ops.iter().position(|op| matches!(op, Op::MetaText(_))).unwrap();
    let Op::MetaText(record) = &s.ops[index] else { panic!("record") };
    let retained: Vec<_> = s.ops[index + 1..index + 1 + record.outline_ops]
        .iter()
        .filter_map(|op| if let Op::Path { path, .. } = op { Some(view * path) } else { None })
        .collect();
    assert_eq!(retained.len(), glyphs.len());
    let coordinates = |element: &PathEl| -> Vec<f64> {
        match element {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => vec![p.x, p.y],
            PathEl::QuadTo(a, b) => vec![a.x, a.y, b.x, b.y],
            PathEl::CurveTo(a, b, c) => vec![a.x, a.y, b.x, b.y, c.x, c.y],
            PathEl::ClosePath => vec![],
        }
    };
    let mut pen = 0.;
    for (glyph, old) in glyphs.iter().zip(retained) {
        let reconstructed = transform
            * Affine::translate((anchor, 0.))
            * Affine::scale(unit)
            * Affine::translate((pen + f64::from(glyph.x_offset), -f64::from(glyph.y_offset)))
            * db.outline(&face, glyph.gid).as_ref();
        assert_eq!(reconstructed.elements().len(), old.elements().len());
        for (new, old) in reconstructed.elements().iter().zip(old.elements()) {
            assert_eq!(std::mem::discriminant(new), std::mem::discriminant(old));
            for (a, b) in coordinates(new).iter().zip(coordinates(old)) {
                assert!((a - b).abs() < 1e-7, "SVG glyph control point {a} differs from retained {b}");
            }
        }
        pen += f64::from(glyph.x_advance);
    }
}

#[test]
fn metafile_anchor_transform_and_device_clips_keep_legacy_paths() {
    use scene::TextAnchor;
    for (horizontal, anchor) in [(0u32, TextAnchor::Start), (6, TextAnchor::Middle), (2, TextAnchor::End)] {
        for vertical in [0u32, 8, 24] {
            for current in [0u32, 1] {
                for height in [-16i32, 16] {
                    for world in [[1f32, 0., 0., 1., 0., 0.], [1.25, 0.25, 0.5, 0.75, 7., 11.]] {
                        let mut font = recipe::font(1, 400, 0);
                        font[12..16].copy_from_slice(&height.to_le_bytes());
                        font[20..24].copy_from_slice(&450i32.to_le_bytes());
                        let records = [
                            font,
                            recipe::record(37, &1u32.to_le_bytes()),
                            recipe::record(35, &world.into_iter().flat_map(f32::to_le_bytes).collect::<Vec<_>>()),
                            recipe::record(22, &(horizontal | vertical | current).to_le_bytes()),
                            recipe::record(27, &[70i32, 95].into_iter().flat_map(i32::to_le_bytes).collect::<Vec<_>>()),
                            recipe::record(30, &[5i32, 7, 370, 180].into_iter().flat_map(i32::to_le_bytes).collect::<Vec<_>>()),
                            recipe::text((120, 40), &"Anchor".encode_utf16().collect::<Vec<_>>(), 4),
                        ];
                        let options = Options { page_canvas: true, ..Default::default() };
                        let s = scene::page(&document(recipe::file(&records)), 0, options).unwrap();
                        let r = s.ops.iter().find_map(|op| if let Op::MetaText(r) = op { Some(r) } else { None }).unwrap();
                        assert_eq!(r.anchor, anchor);
                        assert_eq!(r.dc_clips.len(), 1);
                        let xml = svg::render(&s, options).unwrap();
                        assert_eq!(strings(&xml), ["Anchor"]);
                        assert!(xml.contains("mt0c0"));
                        assert!(xml.contains("mt0r"));
                        assert_svg_placement_matches_retained_glyphs(&s, &xml);
                        let outlined = Options { text_outlines: true, ..options };
                        assert_eq!(svg::render(&s, outlined).unwrap(), svg::render(&legacy(s.clone()), outlined).unwrap());
                        assert_eq!(raster::render(&s, options).unwrap(), raster::render(&legacy(s), options).unwrap());
                    }
                }
            }
        }
    }
}

#[test]
fn private_played_metafile_record_multisets_match_svg_text() {
    use std::collections::BTreeMap;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../diagramcraft-corpus/field/oss-2026-10-22/files");
    if !root.exists() {
        return;
    }
    let cases = [
        ("asposenet__Examples__Data__Working-Shapes__Drawing1.vsdx", 405usize),
        ("asposejava__Examples__src__main__resources__Shapes__Apply3DRotationEffects_Out.vsdx", 405),
        ("asposejava__Examples__src__main__resources__Shapes__ConnectVisioSubShapes_Out.vsdx", 405),
        ("asposenet__Examples__Data__OLE-Objects__Drawing1.vsdx", 2),
    ];
    let mut shared_count = None;
    for (file, expected) in cases {
        let document = misterdiagram_vsdx::read(root.join(file)).unwrap();
        let options = Options { page_canvas: true, cached_fallback: true, ..Default::default() };
        let s = scene::page(&document, document.page_index(1).unwrap(), options).unwrap();
        let records: Vec<_> = s.ops.iter().filter_map(|op| if let Op::MetaText(r) = op { Some(r) } else { None }).collect();
        let faces: std::collections::BTreeSet<_> = records.iter().map(|r| (&r.family, &r.measured_family)).collect();
        eprintln!("metafile-fonts {file}: {faces:?}");
        assert_eq!(records.len(), expected, "{file}: record census");
        if expected == 2 {
            assert_eq!(records.len(), 2, "{file}");
        } else {
            if let Some(count) = shared_count {
                assert_eq!(records.len(), count, "{file}");
            }
            shared_count = Some(records.len());
            assert!(!records.is_empty());
        }
        assert!(records.iter().all(|r| !r.outlines_only));
        assert!(svg::warnings(&s, options).is_empty());
        let xml = svg::render(&s, options).unwrap();
        let actual = strings(&xml);
        let mut counts = BTreeMap::<String, usize>::new();
        for text in actual {
            *counts.entry(text).or_default() += 1;
        }
        for r in &records {
            let n = counts.get_mut(&r.text).unwrap_or_else(|| panic!("missing record {:?}", r.text));
            assert!(*n > 0);
            *n -= 1;
        }
        let shape_texts = s.ops.iter().filter(|op| matches!(op, Op::Text { .. })).count();
        assert_eq!(xml.matches("<text ").count(), records.len() + shape_texts);
        if expected == 405 {
            for text in ["72F", "144F", "300F", "LINK"] {
                assert!(records.iter().any(|r| r.text == text));
            }
        } else {
            assert!(records.iter().all(|r| r.clip.is_some()));
        }
        eprintln!("{file}: played={} added={} reference={expected} bytes={}", records.len(), xml.matches("<text ").count() - shape_texts, xml.len());
    }
}

#[test]
fn zero_ink_metadata_keeps_existing_comment_warning() {
    let mut comment = 4u32.to_le_bytes().to_vec();
    comment.extend(b"EMF+");
    let imported = metafile::import(&recipe::file(&[recipe::record(70, &comment), recipe::text((0, 0), &[32, 32], 0)])).unwrap();
    let empty = metafile::import(&recipe::file(&[recipe::record(70, &comment)])).unwrap();
    assert_eq!(imported.warnings, empty.warnings);
    assert_eq!(imported.ops.iter().filter(|op| matches!(op, Op::MetaText(_))).count(), 1);
}

#[test]
fn zero_extent_foreign_text_keeps_searchability_without_drawing() {
    for name in ["ImgWidth", "ImgHeight", "Width", "Height"] {
        let mut doc = document(recipe::fixture());
        let sheet = &mut doc.pages[0].shapes[0].sheet;
        sheet.cells.retain(|c| c.name != name);
        sheet.cells.push(misterdiagram_model::Cell { name: name.into(), value: Some("0".into()), ..Default::default() });
        let s = scene::page(&doc, 0, Options::default()).unwrap();
        let records: Vec<_> = s.ops.iter().filter_map(|op| if let Op::MetaText(r) = op { Some(r) } else { None }).collect();
        assert_eq!(records.len(), 4, "{name}");
        assert!(records.iter().all(|r| r.outline_ops == 0 && r.transform.determinant() == 0.));
        assert_eq!(strings(&svg::render(&s, Options::default()).unwrap()), ["Searchable 144F", "Centred"]);
        let mut absent = doc.clone();
        absent.pages[0].shapes[0].foreign_data = None;
        let empty = scene::page(&absent, 0, Options::default()).unwrap();
        assert_eq!(s.bounds, empty.bounds);
        assert_eq!(s.warnings, empty.warnings);
        assert_eq!(raster::render(&s, Options::default()).unwrap(), raster::render(&empty, Options::default()).unwrap());
        let outlines = Options { text_outlines: true, ..Default::default() };
        assert_eq!(svg::render(&s, outlines).unwrap(), svg::render(&empty, outlines).unwrap());
        assert_eq!(
            misterdiagram_render::pdf::render(&[s], Options::default()).unwrap(),
            misterdiagram_render::pdf::render(&[empty], Options::default()).unwrap()
        );
        let mut hostile = recipe::text((0, 0), &[65], 4);
        hostile[56..60].copy_from_slice(&1_000_000_000i32.to_le_bytes());
        doc.parts.get_mut("visio/media/original.emf").unwrap().bytes = recipe::file(&[hostile]).into();
        let ignored = scene::page(&doc, 0, Options::default()).unwrap();
        assert!(ignored.warnings.is_empty());
        assert!(!ignored.ops.iter().any(|op| matches!(op, Op::MetaText(_))));
        doc.parts.get_mut("visio/media/original.emf").unwrap().bytes = vec![1, 2, 3].into();
        assert!(scene::page(&doc, 0, Options::default()).unwrap().warnings.is_empty());
    }
}
