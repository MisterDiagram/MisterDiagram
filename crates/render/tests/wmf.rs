//! WMF record coverage uses contributor-original bytes only.
//! https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/
#[path = "support/wmf.rs"]
mod recipe;
use kurbo::{Rect, Shape as _};
use misterdiagram_render::{
    metafile::{self, Kind},
    scene::{Op, Paint},
};
use recipe::{file, packed, record, setup};
fn colored_bounds(ops: &[Op]) -> Vec<Rect> {
    ops.iter()
        .filter_map(|op| match op {
            Op::Path { path, paint: Paint::Solid(c), .. } if (c.r, c.g, c.b) == (30, 90, 150) => Some(path.bounding_box()),
            _ => None,
        })
        .collect()
}
#[test]
fn sniff_is_bytes_only_and_rejects_near_matches() {
    for kind in [1u16, 2] {
        for version in [0x100u16, 0x300] {
            let mut b = recipe::rectangle();
            b[..2].copy_from_slice(&kind.to_le_bytes());
            b[4..6].copy_from_slice(&version.to_le_bytes());
            assert_eq!(metafile::sniff(&b), Some(Kind::Wmf));
            assert!(metafile::import(&b).is_ok());
        }
    }
    for (offset, value) in [(0, 3u16), (2, 8), (4, 0x200)] {
        let mut b = recipe::rectangle();
        b[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
        assert_eq!(metafile::sniff(&b), None);
    }
    let b = recipe::rectangle();
    for n in 0..6 {
        assert_eq!(metafile::sniff(&b[..n]), None);
    }
    assert_eq!(metafile::sniff(b"\x89PNG\r\n\x1a\n"), None);
    let mut emf = vec![0; 88];
    emf[40..44].copy_from_slice(b" EMF");
    assert_eq!(metafile::sniff(&emf), Some(Kind::Emf));
    assert_eq!(metafile::sniff(&0x9ac6cdd7u32.to_le_bytes()), Some(Kind::Wmf));
    assert!(metafile::import(&0x9ac6cdd7u32.to_le_bytes()).is_err());
}
#[test]
fn plain_window_and_placeable_nonzero_origin_have_exact_coordinates() {
    let imported = metafile::import(&recipe::rectangle()).unwrap();
    assert!(imported.warnings.is_empty());
    assert_eq!(imported.frame, Rect::new(0., 0., 72., 36.));
    assert_eq!(colored_bounds(&imported.ops), vec![Rect::new(9., 9., 63., 27.)]);
    let mut records = setup();
    records[1] = record(0x20b, &[200, 100]);
    records[2] = record(0x20c, &[600, 1200]);
    records.push(record(0x41b, &[650, 1000, 350, 400]));
    let b = file(&records, Some(([100, 200, 1300, 800], 600)));
    assert_eq!(metafile::sniff(&b), Some(Kind::Wmf));
    let imported = metafile::import(&b).unwrap();
    assert!(imported.warnings.is_empty());
    assert_eq!(imported.frame, Rect::new(0., 0., 144., 72.));
    assert_eq!(colored_bounds(&imported.ops), vec![Rect::new(36., 18., 108., 54.)]);
}
#[test]
fn dynamic_mapping_and_saved_dc_restore_the_actual_coordinates() {
    let mut r = setup();
    r.extend([
        record(0x41b, &[540, 1260, 180, 180]),
        record(0x1e, &[]),
        record(0x20c, &[1440, 2880]),
        record(0x20d, &[100, 200]),
        record(0x41b, &[540, 1260, 180, 180]),
        record(0x127, &[-1]),
        record(0x20f, &[100, 200]),
        record(0x211, &[200, 400]),
        record(0x41b, &[540, 1260, 180, 180]),
    ]);
    let imported = metafile::import(&file(&r, None)).unwrap();
    assert!(imported.warnings.is_empty());
    assert_eq!(colored_bounds(&imported.ops), vec![Rect::new(9., 9., 63., 27.), Rect::new(14.5, 9.5, 41.5, 18.5), Rect::new(19., 14., 73., 32.)]);
    let mut r = setup();
    r.extend([record(0x410, &[1, 2, 1, 2]), record(0x412, &[1, 2, 1, 2]), record(0x20e, &[-720, -1440]), record(0x41b, &[540, 1260, 180, 180])]);
    let imported = metafile::import(&file(&r, None)).unwrap();
    assert_eq!(colored_bounds(&imported.ops), vec![Rect::new(-31.5, -13.5, -4.5, -4.5)]);
}
#[test]
fn polygon_dash_text_bitmap_and_clip_records_are_played() {
    let mut r = setup();
    r.push(record(0x324, &[3, 180, 180, 1260, 180, 720, 540]));
    let p = metafile::import(&file(&r, None)).unwrap();
    assert!(p.warnings.is_empty());
    assert_eq!(colored_bounds(&p.ops), vec![Rect::new(9., 9., 63., 27.)]);
    let mut r = setup();
    r.extend([record(0x2fa, &[1, 20, 0, 0, 0]), record(0x12d, &[2]), record(0x325, &[2, 180, 360, 1260, 360])]);
    let p = metafile::import(&file(&r, None)).unwrap();
    assert!(p.warnings.is_empty());
    assert!(p.ops.iter().any(|op| matches!(op,Op::Path {path,..} if path.elements().len()>20)));
    let mut r = setup();
    let mut font = vec![0u8; 18];
    font[..2].copy_from_slice(&(-240i16).to_le_bytes());
    font.extend(b"DejaVu Sans");
    font.resize(50, 0);
    r.push(packed(0x2fb, &font));
    r.push(record(0x12d, &[2]));
    let mut text = 4i16.to_le_bytes().to_vec();
    text.extend(b"WMF!");
    text.extend(300i16.to_le_bytes());
    text.extend(180i16.to_le_bytes());
    r.push(packed(0x521, &text));
    let p = metafile::import(&file(&r, None)).unwrap();
    assert!(p.warnings.is_empty(), "{:?}", p.warnings);
    assert!(p.ops.len() > 3);
    for function in [0xf43, 0xb41, 0x940] {
        let mut r = setup();
        r.push(recipe::bitmap(function));
        let p = metafile::import(&file(&r, None)).unwrap();
        assert!(p.warnings.is_empty(), "{:?}", p.warnings);
        assert!(p.ops.iter().any(|op| matches!(op,Op::Image {pixels,..} if pixels.dimensions()==(2,1) && pixels.get_pixel(0,0).0==[255,0,0,255] && pixels.get_pixel(1,0).0==[0,255,0,255])));
    }
    let mut r = setup();
    r.extend([record(0x416, &[360, 720, 0, 0]), record(0x41b, &[540, 1260, 180, 180])]);
    let p = metafile::import(&file(&r, None)).unwrap();
    assert!(p.warnings.is_empty());
    assert!(p.ops.iter().any(|op| matches!(op,Op::Clip {path,..} if path.bounding_box()==Rect::new(0.,0.,36.,18.))));
}
#[test]
fn truncated_zero_sized_and_unknown_records_are_bounded() {
    let b = recipe::rectangle();
    for n in 0..b.len() {
        let _ = metafile::import(&b[..n]);
    }
    let mut r = setup();
    r.extend([record(0x7777, &[]), record(0x7778, &[]), record(0x626, &[])]);
    let p = metafile::import(&file(&r, None)).unwrap();
    assert_eq!(p.warnings.len(), 1);
    assert!(p.warnings[0].starts_with("2 records"));
    let mut b = recipe::rectangle();
    b[18..22].fill(0);
    assert!(metafile::import(&b).unwrap().warnings.iter().any(|s| s.contains("ends early")));
    let short = file(&[record(0x20b, &[]), record(0x20c, &[720, 1440])], None);
    let h = metafile::header::wmf_frame(&short).unwrap();
    assert_eq!(h.rect, Some([0., 0., 1440., 720.]));
    let mut huge = vec![0; 128 * 1024 * 1024 + 1];
    huge[..6].copy_from_slice(&b[..6]);
    assert_eq!(metafile::import(&huge).err().as_deref(), Some("metafile byte limit"));
}

#[path = "support/wmf_document.rs"]
mod original_document;
#[test]
fn foreign_image_offsets_rotation_and_inherited_media_keep_the_frame() {
    use misterdiagram_render::scene::{self, Options};
    let mut doc = original_document::document(recipe::rectangle());
    let scene = scene::page(&doc, 0, Options::default()).unwrap();
    assert!(scene.warnings.is_empty(), "{:?}", scene.warnings);
    let b = colored_bounds(&scene.ops);
    assert_eq!(b.len(), 1);
    let expected = Rect::new(3.625, 2.875, 4.375, 5.125);
    for (got, want) in [b[0].x0, b[0].y0, b[0].x1, b[0].y1].into_iter().zip([expected.x0, expected.y0, expected.x1, expected.y1]) {
        assert!((got - want).abs() < 1e-9, "{:?}", b[0]);
    }
    let media_shape = doc.pages[0].shapes[0].clone();
    doc.masters[0].shapes = vec![media_shape].into();
    let relationships = doc.parts.get(&doc.pages[0].part).unwrap().relationships.clone();
    doc.parts.entry(doc.masters[0].part.clone()).or_default().relationships = relationships;
    let master = doc.masters[0].id;
    let shape = &mut doc.pages[0].shapes[0];
    shape.master = Some(master);
    shape.master_shape = Some(7);
    shape.foreign_data = None;
    let inherited = scene::page(&doc, 0, Options::default()).unwrap();
    assert!(inherited.warnings.is_empty());
    assert_eq!(colored_bounds(&inherited.ops), b);
}

#[test]
fn object_reuse_curves_escape_and_malformed_dibs_keep_the_reader_bounded() {
    let mut r = setup();
    r.extend([
        record(0x418, &[540, 1260, 180, 180]),
        record(0x61c, &[80, 80, 540, 1260, 180, 180]),
        record(0x817, &[360, 1260, 180, 720, 540, 1260, 180, 180]),
        record(0x81a, &[360, 1260, 180, 720, 540, 1260, 180, 180]),
        record(0x830, &[360, 1260, 180, 720, 540, 1260, 180, 180]),
        record(0x1f0, &[1]),
        record(0x2fc, &[0, 0x1234, 86, 0]),
        record(0x12d, &[1]),
        record(0x626, &[0, 0]),
        record(0x41b, &[540, 1260, 180, 180]),
    ]);
    let p = metafile::import(&file(&r, None)).unwrap();
    assert!(p.warnings.is_empty(), "{:?}", p.warnings);
    assert!(colored_bounds(&p.ops).len() >= 4);
    assert!(p.ops.iter().any(|op| matches!(op,Op::Path{paint:Paint::Solid(c),..} if (c.r,c.g,c.b)==(52,18,86))));
    let p = metafile::import(&recipe::rectangle()).unwrap();
    assert!(
        !p.ops.iter().any(|op| matches!(op,Op::Path{paint:Paint::Solid(c),..} if (c.r,c.g,c.b)==(0,0,0))),
        "handle zero must select the null pen"
    );
    let mut interleaved = setup();
    let mut font = vec![0u8; 50];
    font[..2].copy_from_slice(&(-240i16).to_le_bytes());
    font[18..29].copy_from_slice(b"DejaVu Sans");
    interleaved.extend([
        record(0xf7, &[]),              // Other slot 2.
        packed(0x2fb, &font),           // Font slot 3, interleaved with pens and brushes.
        record(0x6ff, &[]),             // Other slot 4.
        record(0x2fc, &[0, 255, 0, 0]), // Red brush slot 5.
        record(0x12d, &[5]),
        record(0x1f0, &[2]),
        record(0x2fc, &[0, 0x1234, 86, 0]), // Distinct brush reuses slot 2.
        record(0x12d, &[2]),
        record(0x41b, &[540, 1260, 180, 180]),
        record(0x1f0, &[2]),
        record(0x12d, &[5]), // Select red before trying the deleted handle.
        record(0x12d, &[2]), // Formerly valid, now deleted: leave selected state intact.
        record(0x41b, &[540, 1260, 180, 180]),
    ]);
    let p = metafile::import(&file(&interleaved, None)).unwrap();
    assert!(p.warnings.is_empty());
    let colors: Vec<_> = p
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::Path { paint: Paint::Solid(c), .. } => Some((c.r, c.g, c.b)),
            _ => None,
        })
        .collect();
    assert_eq!(colors, vec![(52, 18, 86), (255, 0, 0)]);
    let bitmap = recipe::bitmap(0xf43);
    for end in (6..bitmap.len()).step_by(2) {
        let mut partial = bitmap[..end].to_vec();
        partial[..4].copy_from_slice(&((end / 2) as u32).to_le_bytes());
        let mut r = setup();
        r.push(partial);
        let p = metafile::import(&file(&r, None)).unwrap();
        assert!(!p.warnings.is_empty());
    }
}

#[test]
fn headers_and_first_window_variants_preserve_admission_and_orientation() {
    let mut r = setup();
    r[1] = record(0x20b, &[200, 100]);
    r.push(record(0x41b, &[740, 1360, 380, 280]));
    let p = metafile::import(&file(&r, None)).unwrap();
    assert_eq!(colored_bounds(&p.ops), vec![Rect::new(9., 9., 63., 27.)]);
    let mut r = setup();
    r[2] = record(0x20c, &[-720, -1440]);
    r.push(record(0x41b, &[-180, -180, -540, -1260]));
    let p = metafile::import(&file(&r, None)).unwrap();
    assert_eq!(colored_bounds(&p.ops), vec![Rect::new(9., 9., 63., 27.)]);
    let absent = file(&[record(0x20c, &[720, 1440])], None);
    assert_eq!(metafile::header::wmf_frame(&absent).unwrap().rect, Some([0., 0., 1440., 720.]));
    let zero = file(&[record(0x20c, &[0, 1440])], None);
    assert_eq!(metafile::header::wmf_frame(&zero).unwrap().rect, None);
    let placeable = file(&setup(), Some(([0, 0, 1440, 720], 0)));
    assert_eq!(metafile::header::wmf_frame(&placeable).unwrap().inch, 1440.);
    for n in 0..40 {
        assert!(metafile::import(&placeable[..n]).is_err(), "length={n}");
    }
    for words in [0u32, 1, 2, u32::MAX] {
        let mut b = recipe::rectangle();
        b[18..22].copy_from_slice(&words.to_le_bytes());
        assert!(metafile::import(&b).unwrap().warnings.iter().any(|w| w.contains("ends early")));
    }
    let valid = recipe::rectangle();
    let mut exact = valid.clone();
    exact.resize(metafile::header::BYTE_LIMIT, 0);
    assert!(metafile::import(&exact).is_ok());
}
#[test]
fn pending_lines_flush_before_independent_mapping_and_style_changes() {
    let mut r = setup();
    r.extend([
        record(0x2fa, &[0, 20, 0, 0, 0]),
        record(0x12d, &[2]),
        record(0x214, &[180, 180]),
        record(0x213, &[180, 720]),
        record(0x20c, &[1440, 1440]),
        record(0x214, &[360, 180]),
        record(0x213, &[360, 720]),
    ]);
    let p = metafile::import(&file(&r, None)).unwrap();
    assert!(p.warnings.is_empty());
    let black: Vec<_> = p
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::Path { path, paint: Paint::Solid(c), .. } if (c.r, c.g, c.b) == (0, 0, 0) => Some(path.bounding_box()),
            _ => None,
        })
        .collect();
    assert_eq!(black.len(), 2);
    assert!((black[0].center().y - 9.).abs() < 1e-9);
    assert!((black[1].center().y - 9.).abs() < 1e-9);
    assert!((black[0].height() - 1.).abs() < 1e-9);
    assert!((black[1].height() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
    let mut r = setup();
    r.extend([
        record(0x1e, &[]),
        record(0x2fc, &[0, 255, 0, 0]),
        record(0x12d, &[2]),
        record(0x41b, &[540, 1260, 180, 180]),
        record(0x127, &[-1]),
        record(0x41b, &[540, 1260, 180, 180]),
        record(0x1f0, &[15]),
        record(0x12d, &[15]),
    ]);
    let p = metafile::import(&file(&r, None)).unwrap();
    assert!(p.warnings.is_empty());
    assert_eq!(colored_bounds(&p.ops).len(), 1);
}
#[test]
fn nested_flipped_foreign_shape_uses_the_same_world_transform() {
    use misterdiagram_model::{Cell, skeleton};
    use misterdiagram_render::scene::{self, Options};
    let mut doc = original_document::document(recipe::rectangle());
    let children = doc.pages[0].shapes.clone();
    let mut group = skeleton::rectangle(20, 10., 20., 10., 10., "").unwrap();
    group.shape_type = Some("Group".into());
    group.sheet.sections.clear();
    group.shapes = children.to_vec();
    group.sheet.cells.push(Cell { name: "FlipX".into(), value: Some("1".into()), ..Default::default() });
    doc.pages[0].shapes = vec![group].into();
    let scene = scene::page(&doc, 0, Options::default()).unwrap();
    assert!(scene.warnings.is_empty());
    let bounds = colored_bounds(&scene.ops);
    assert_eq!(bounds.len(), 1);
    for (got, want) in [bounds[0].x0, bounds[0].y0, bounds[0].x1, bounds[0].y1].into_iter().zip([15.625, 22.875, 16.375, 25.125]) {
        assert!((got - want).abs() < 1e-9, "{:?}", bounds[0]);
    }
}

#[test]
fn window_and_viewport_scale_records_each_change_independent_axes() {
    let mut r = setup();
    r.extend([
        record(0x410, &[1, 3, 1, 2]),
        record(0x41b, &[540, 1260, 180, 180]),
        record(0x412, &[2, 1, 1, 2]),
        record(0x41b, &[540, 1260, 180, 180]),
    ]);
    let p = metafile::import(&file(&r, None)).unwrap();
    assert!(p.warnings.is_empty());
    assert_eq!(colored_bounds(&p.ops), vec![Rect::new(4.5, 3., 31.5, 9.), Rect::new(9., 1.5, 63., 4.5)]);
}

#[test]
fn aggregate_polypolygon_points_fail_before_path_allocation() {
    // Each individual count fits a WORD; together they exceed the existing path budget.
    let mut body = vec![2, 0, 255, 255, 255, 255];
    body.resize(6 + 2 * 65535 * 4, 0);
    let mut records = setup();
    records.push(packed(0x538, &body));
    let error = metafile::import(&file(&records, None)).err().unwrap();
    assert!(error.starts_with("metafile operation limit:"), "{error}");
}
