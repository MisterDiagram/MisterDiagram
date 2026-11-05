//! Wrapped-picture selection and hostile comment streams use original byte recipes.
//! https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/cfc88064-d86d-4b52-9374-3ce27d456179
#[path = "support/wmf.rs"]
mod base_wmf;
#[path = "support/wmf_document.rs"]
mod document;
#[path = "support/wmf_embedded.rs"]
mod recipe;
use base_wmf as wmf;
use misterdiagram_render::{
    metafile, raster,
    scene::{self, Options},
};
const WARNING: &str = "WMF comment EMF incomplete, playing the WMF fallback";

fn picture(bytes: Vec<u8>) -> misterdiagram_model::Document {
    let mut doc = document::document(bytes);
    doc.pages[0].shapes[0]
        .foreign_data
        .as_mut()
        .unwrap()
        .attributes
        .insert("xmlns:r".into(), "http://schemas.openxmlformats.org/officeDocument/2006/relationships".into());
    doc
}

fn sized_picture(bytes: Vec<u8>) -> misterdiagram_model::Document {
    let mut doc = picture(bytes);
    let shape = &mut doc.pages[0].shapes[0];
    for (name, value) in [
        ("Width", "13"),
        ("Height", "10"),
        ("PinX", "7.5"),
        ("PinY", "6"),
        ("Angle", "0"),
        ("ImgWidth", "13"),
        ("ImgHeight", "10"),
        ("ImgOffsetX", "0"),
        ("ImgOffsetY", "0"),
    ] {
        shape.sheet.cells.retain(|c| c.name != name);
        shape.sheet.cells.push(misterdiagram_model::Cell { name: name.into(), value: Some(value.into()), ..Default::default() });
    }
    doc
}

#[test]
fn complete_comments_select_inner_frame_and_identical_packaged_pixels() {
    let inner = recipe::triangle();
    for size in [2, 80, 8192] {
        let wrapped = recipe::wrapper(&inner, size, false);
        let expected = metafile::import(&inner).unwrap();
        let actual = metafile::import(&wrapped).unwrap();
        assert_eq!(actual.frame, expected.frame);
        assert!(actual.warnings.is_empty(), "{:?}", actual.warnings);
        assert_eq!(format!("{:?}", actual.ops), format!("{:?}", expected.ops));
        let doc = picture(wrapped);
        let bytes = misterdiagram_vsdx::write(&doc).unwrap().bytes;
        let doc = misterdiagram_vsdx::import(&bytes).unwrap();
        let a = scene::page(&doc, 0, Options::default()).unwrap();
        let b = scene::page(&document::document(inner.clone()), 0, Options::default()).unwrap();
        assert_eq!(raster::render(&a, Options::default()).unwrap(), raster::render(&b, Options::default()).unwrap());
    }
}

#[test]
fn incomplete_comments_play_rectangle_with_shape_warning_for_render_and_sheet() {
    let bytes = recipe::wrapper(&recipe::triangle(), 80, true);
    let doc = document::document(bytes);
    let scene = scene::page(&doc, 0, Options::default()).unwrap();
    assert_eq!(scene.warnings, vec![format!("shape 7: {WARNING}")]);
    assert_eq!(scene::validate_foreign_parts(&doc, misterdiagram_vsdx::dump::Selection::Shape(7)).unwrap(), scene.warnings);
    let fallback = scene::page(&document::document(wmf::rectangle()), 0, Options::default()).unwrap();
    assert_eq!(raster::render(&scene, Options::default()).unwrap(), raster::render(&fallback, Options::default()).unwrap());
    let mut inherited = doc.clone();
    let original = inherited.pages[0].shapes[0].clone();
    inherited.masters[0].shapes = vec![original].into();
    let master = inherited.masters[0].id;
    let relationships = inherited.parts.get(&inherited.pages[0].part).unwrap().relationships.clone();
    inherited.parts.entry(inherited.masters[0].part.clone()).or_default().relationships = relationships;
    let shape = &mut inherited.pages[0].shapes[0];
    shape.master = Some(master);
    shape.master_shape = Some(7);
    shape.foreign_data = None;
    assert_eq!(scene::page(&inherited, 0, Options::default()).unwrap().warnings, scene.warnings);
    assert_eq!(scene::validate_foreign_parts(&inherited, misterdiagram_vsdx::dump::Selection::Shape(7)).unwrap(), scene.warnings);
}

#[test]
fn non_placeable_zero_and_one_pens_stay_one_pixel_at_large_shape_placement() {
    use kurbo::Shape as _;
    use misterdiagram_render::scene::{Op, Paint};
    let scene = scene::page(&sized_picture(recipe::pens()), 0, Options::default()).unwrap();
    assert!(scene.warnings.is_empty());
    let widths: Vec<_> = scene
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::Path { path, paint: Paint::Solid(c), .. } if (c.r, c.g, c.b) == (0, 0, 0) => {
                let b = path.bounding_box();
                Some(b.width().min(b.height()) * 96.)
            }
            _ => None,
        })
        .collect();
    assert_eq!(widths.len(), 4);
    for width in widths {
        assert!((width - 1.).abs() < 1e-8, "{width}");
    }
}

/// Optional private witness writer: no input or output asset is embedded in this test.
#[test]
fn write_requested_private_fixtures() {
    let Some(output) = std::env::var_os("MISTERDIAGRAM_WRAPPED_FIXTURES") else { return };
    let input = std::env::var_os("MISTERDIAGRAM_INNER_EMF").expect("explicit inner EMF path");
    let inner = std::fs::read(input).unwrap();
    let output = std::path::PathBuf::from(output);
    std::fs::create_dir_all(&output).unwrap();
    for (name, bytes) in [
        ("complete", recipe::wrapper(&inner, 4096, false)),
        ("truncated", recipe::wrapper(&inner, 4096, true)),
        ("fallback", wmf::rectangle()),
        ("pens", recipe::pens()),
        ("inner", inner),
    ] {
        let extension = if name == "inner" { "emf" } else { "wmf" };
        std::fs::write(output.join(format!("{name}.{extension}")), &bytes).unwrap();
        let written = misterdiagram_vsdx::write(&sized_picture(bytes)).unwrap();
        std::fs::write(output.join(format!("{name}.vsdx")), written.bytes).unwrap();
    }
}

#[test]
fn contradictory_sizes_metadata_missing_duplicate_and_reordered_chunks_are_bounded() {
    let original = recipe::wrapper(&recipe::triangle(), 80, false);
    for (offset, value) in [(26, 0u32), (30, 0), (42, 1), (46, 0), (46, u32::MAX), (50, 0), (50, 8193), (54, u32::MAX), (58, 0), (58, u32::MAX)] {
        let mut bad = original.clone();
        bad[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        recipe::checksum(&mut bad);
        let imported = metafile::import(&bad).unwrap();
        assert!(imported.warnings.iter().any(|s| s == WARNING), "offset={offset} value={value}");
    }
    let first_end = 18 + 44 + 80;
    for mutation in 0..4 {
        let mut bad = original.clone();
        match mutation {
            0 => {
                bad.drain(18..first_end);
            }
            1 => {
                let part = bad[18..first_end].to_vec();
                bad.splice(first_end..first_end, part);
            }
            2 => {
                let part = bad[18..first_end].to_vec();
                bad.copy_within(first_end..first_end + 124, 18);
                bad[first_end..first_end + 124].copy_from_slice(&part);
            }
            _ => {
                bad[18..22].fill(0);
            }
        }
        recipe::checksum(&mut bad);
        let imported = metafile::import(&bad).unwrap();
        assert!(imported.warnings.iter().any(|s| s == WARNING), "mutation={mutation}");
    }
    for end in 0..original.len() {
        let _ = metafile::import(&original[..end]);
    }
    let bad = recipe::wrapper(&[0; 160], 80, false);
    assert!(metafile::import(&bad).err().unwrap().contains("invalid EMF header"));
}
