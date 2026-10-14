//! PDF-1 regression: inspect the actual ToUnicode streams and shown glyph codes.
use diagramcraft_model::skeleton;
use diagramcraft_render::{pdf, scene};
use hayro_syntax::{Pdf, content::ops::TypedInstruction, object::Object};
use std::collections::BTreeMap;
use std::sync::OnceLock;

const PHRASE: &str = "flash office file traffic";

// Only the bfchar form emitted by krilla is needed here. Reject malformed entries
// rather than silently dropping them and giving the exporter a false pass.
fn bfchar(text: &str) -> Result<BTreeMap<u16, Vec<u16>>, String> {
    if text.len() > 1024 * 1024 {
        return Err("CMap test input too large".into());
    }
    let mut map = BTreeMap::new();
    let mut remaining = None;
    let units = |token: &str| -> Result<Vec<u16>, String> {
        let hex = token.strip_prefix('<').and_then(|t| t.strip_suffix('>')).ok_or("invalid hex string")?;
        if hex.is_empty() || hex.len() % 4 != 0 || !hex.is_ascii() {
            return Err("invalid UTF-16 hex length".into());
        }
        hex.as_bytes()
            .as_chunks::<4>()
            .0
            .iter()
            .map(|chunk| u16::from_str_radix(std::str::from_utf8(chunk).map_err(|e| e.to_string())?, 16).map_err(|e| e.to_string()))
            .collect()
    };
    for line in text.lines() {
        let tokens: Vec<_> = line.split_whitespace().collect();
        if let [count, "beginbfchar"] = tokens.as_slice() {
            if remaining.is_some() {
                return Err("nested bfchar block".into());
            }
            remaining = Some(count.parse::<usize>().map_err(|e| e.to_string())?);
        } else if tokens.as_slice() == ["endbfchar"] {
            if remaining != Some(0) {
                return Err("bfchar count mismatch".into());
            }
            remaining = None;
        } else if let Some(count) = remaining.as_mut() {
            let [src, dst] = tokens.as_slice() else { return Err("invalid bfchar pair".into()) };
            let src = units(src)?;
            let dst = units(dst)?;
            if src.len() != 1 || String::from_utf16(&dst).is_err() {
                return Err("invalid bfchar code or destination".into());
            }
            *count = count.checked_sub(1).ok_or("too many bfchar pairs")?;
            if map.insert(src[0], dst).is_some() {
                return Err("duplicate bfchar source".into());
            }
        }
    }
    if remaining.is_some() || map.is_empty() {
        return Err("missing or incomplete bfchar block".into());
    }
    Ok(map)
}

struct TextLayer {
    map: BTreeMap<u16, Vec<u16>>,
    shown: Vec<u16>,
}

fn text_layer() -> &'static TextLayer {
    static LAYER: OnceLock<TextLayer> = OnceLock::new();
    LAYER.get_or_init(|| {
        let mut doc = skeleton::document(false);
        doc.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 6., 1., PHRASE).unwrap());
        let bytes = diagramcraft_vsdx::write(&doc).unwrap().bytes;
        let doc = diagramcraft_vsdx::import(&bytes).unwrap();
        assert_eq!(doc.pages.len(), 1);
        assert_eq!(doc.pages[0].shapes.len(), 1);
        let options = scene::Options::default();
        let page = scene::page(&doc, 0, options).unwrap();
        let runs: Vec<_> = page
            .ops
            .iter()
            .filter_map(|op| match op {
                scene::Op::Text { layout, .. } => Some(&layout.runs),
                _ => None,
            })
            .flatten()
            .filter(|r| !r.cells.is_empty())
            .collect();
        assert!(!runs.is_empty());
        assert!(runs.iter().all(|r| r.face.family == "Carlito"), "fixture must use bundled Carlito via the default Calibri request");
        assert!(runs.iter().any(|r| r.glyphs.len() < r.text.chars().count()), "fixture must actually shape ligatures");
        let bytes = pdf::render(&[page], options).unwrap();
        let pdf = Pdf::new(bytes).unwrap();
        assert_eq!(pdf.pages().len(), 1);
        let mut maps = Vec::new();
        for object in pdf.objects() {
            if let Object::Stream(stream) = object {
                let data = stream.decoded().unwrap(); // Includes FlateDecode CMaps.
                if data.windows(b"beginbfchar".len()).any(|w| w == b"beginbfchar") {
                    maps.push(bfchar(std::str::from_utf8(&data).unwrap()).unwrap());
                }
            }
        }
        assert_eq!(maps.len(), 1, "single-face fixture must have one CMap");
        let map = maps.pop().unwrap();
        let page = &pdf.pages()[0];
        let mut ops = page.typed_operations();
        let mut font = None;
        let mut shown = Vec::new();
        let mut show = |bytes: &[u8]| {
            assert_eq!(bytes.len() % 2, 0, "Identity-H glyph codes have two bytes");
            shown.extend(bytes.as_chunks::<2>().0.iter().map(|c| u16::from_be_bytes(*c)));
        };
        while let Some(op) = ops.next() {
            match op {
                TypedInstruction::TextFont(f) => {
                    let name = f.0.as_ref().to_vec();
                    if let Some(previous) = &font {
                        assert_eq!(previous, &name, "single-face fixture changed font resources");
                    }
                    let resource = page.resources().get_font(f.0).unwrap();
                    let encoding: hayro_syntax::object::Name<'_> = resource.get(b"Encoding").unwrap();
                    assert_eq!(encoding.as_ref(), b"Identity-H");
                    font = Some(name);
                }
                TypedInstruction::ShowText(t) => show(t.0.as_bytes()),
                TypedInstruction::NextLineAndShowText(t) => show(t.0.as_bytes()),
                TypedInstruction::ShowTextWithParameters(t) => show(t.2.as_bytes()),
                TypedInstruction::ShowTexts(t) => {
                    for item in t.0.iter::<Object<'_>>() {
                        match item {
                            Object::String(s) => show(s.as_bytes()),
                            Object::Number(_) => {}
                            other => panic!("unexpected TJ operand: {other:?}"),
                        }
                    }
                }
                _ => {}
            }
        }
        assert!(font.is_some() && !shown.is_empty(), "fixture must contain text-showing operators");
        TextLayer { map, shown }
    })
}

#[test]
fn ligature_text_extracts_in_full() {
    let layer = text_layer();
    let units: Vec<_> = layer.shown.iter().filter_map(|code| layer.map.get(code)).flatten().copied().collect();
    let extracted = String::from_utf16(&units).unwrap();
    let compact = |s: &str| s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    assert_eq!(compact(&extracted), compact(PHRASE), "assertion a: ligature extraction must contain the full phrase");
    println!("assertion a: PASS, {} non-space characters, extracted {extracted:?}", compact(PHRASE).len());
}

#[test]
fn destinations_are_cluster_sized() {
    let layer = text_layer();
    let max = layer.map.values().map(Vec::len).max().unwrap();
    assert!(max <= 4, "assertion b: no ToUnicode destination over 4 UTF-16 code units; found {max}");
    println!("assertion b: PASS, maximum destination {max} UTF-16 code units");
}

#[test]
fn every_shown_glyph_has_a_mapping() {
    let layer = text_layer();
    let missing: Vec<_> = layer.shown.iter().filter(|code| !layer.map.contains_key(code)).collect();
    assert!(missing.is_empty(), "assertion c: every shown glyph must have a bfchar entry; unmapped codes {missing:?}");
    println!("assertion c: PASS, {} shown glyphs, {} CMap entries, zero unmapped", layer.shown.len(), layer.map.len());
}

#[test]
fn cmap_reader_rejects_malformed_input() {
    assert_eq!(bfchar("1 beginbfchar\n<0001> <00660069>\nendbfchar").unwrap()[&1], [0x66, 0x69]);
    for text in [
        "",
        "1 beginbfchar",
        "2 beginbfchar\n<0001> <0066>\nendbfchar",
        "1 beginbfchar\n<0001> <D800>\nendbfchar",
        "1 beginbfchar\n<0001> <xyz1>\nendbfchar",
        "1 beginbfchar\n<0001> <061>\nendbfchar",
        "1 beginbfchar\n<00010002> <0066>\nendbfchar",
        "2 beginbfchar\n<0001> <0066>\n<0001> <0069>\nendbfchar",
        "1 beginbfchar\n1 beginbfchar",
    ] {
        assert!(bfchar(text).is_err(), "accepted malformed CMap {text:?}");
    }
    assert!(bfchar(&"x".repeat(1024 * 1024 + 1)).is_err());
}
