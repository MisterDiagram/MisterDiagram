//! Original synthetic glue cases plus the explicitly authorized Unlicense fixture.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
use misterdiagram_engine::Session;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
fn run(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd} {p}: {e}"))
}
fn numbers(s: &Session, id: u32, names: &[&str]) -> Vec<f64> {
    let themes = misterdiagram_theme::Themes::from_document(s.document()).unwrap();
    let mut e = Evaluator::new(s.document()).unwrap().with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    names.iter().map(|n| e.evaluate(Scope::PageShape { page: s.document().pages[0].id, shape: id }, n).numeric().unwrap()).collect()
}
fn close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (a, b) in actual.iter().zip(expected) {
        assert!((a - b).abs() <= 1e-6, "actual {actual:?}, Visio {expected:?}");
    }
}
#[test]
fn third_party_fixture_hash_render_and_native_group_glue() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ntwrk-clean-and-flat/affinity_native.vssx");
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(bytes.len(), 50820);
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), "5df59f8a80af6878d47418bbbc3100718b08f10b167e67a15ff514455e0ae4ae");
    let doc = misterdiagram_vsdx::import(&bytes).unwrap();
    assert_eq!(doc.masters.len(), 15);
    for (i, m) in doc.masters.iter().enumerate() {
        let scene = misterdiagram_render::scene::master(&doc, i, Default::default()).unwrap();
        let image = misterdiagram_render::raster::render(&scene, Default::default()).unwrap();
        assert!(image.pixels().any(|p| p.0[3] > 0 && p.0[..3].iter().any(|c| *c < 245)), "blank {}", m.name_u);
    }
    for name in ["Router", "Servers"] {
        for compound in [false, true] {
            let mut s = Session::new();
            run(&mut s, "stencil.open", json!({"path":path}));
            let a = run(&mut s, "master.drop", json!({"master":name,"x":2.5,"y":6.4,"width":0.63,"height":1}))["id"].as_u64().unwrap() as u32;
            let b = run(&mut s, "shape.draw.rect", json!({"x":2,"y":2,"width":1.02,"height":1}))["id"].as_u64().unwrap() as u32;
            let c = if compound {
                run(&mut s, "connector.add", json!({"from":a,"to":b,"glue":"dynamic"}))["id"].as_u64().unwrap() as u32
            } else {
                let c = run(&mut s, "master.drop", json!({"stencil":0,"master":"Dynamic connector"}))["id"].as_u64().unwrap() as u32;
                for (end, target) in [("begin", a), ("end", b)] {
                    run(&mut s, "connect.dynamic", json!({"id":c,"target":target,"end":end}));
                }
                c
            };
            close(&numbers(&s, c, &["BeginX", "BeginY", "EndX", "EndY"]), &[2.5, 5.9, 2.5, 3.]);
        }
    }
}
#[test]
fn interior_points_do_not_stop_dynamic_perimeter_walk() {
    for count in [1, 2, 3] {
        for compound in [false, true] {
            let mut s = Session::new();
            run(&mut s, "shape.draw.rect", json!({"x":0.72,"y":2.12,"width":2.567,"height":1.752}));
            run(&mut s, "shape.draw.rect", json!({"x":3.55,"y":0.35,"width":0.9,"height":1.1}));
            run(&mut s, "shape.connection.add", json!({"id":2,"x":0.45,"y":0.88}));
            if count == 2 {
                run(&mut s, "shape.connection.add", json!({"id":2,"x":0.225,"y":0.44}));
            }
            let target = if count == 3 {
                let ellipse = run(&mut s, "shape.draw.ellipse", json!({"x":3.55,"y":1.01,"width":0.9,"height":0.44}))["id"].as_u64().unwrap();
                let group = run(&mut s, "shape.group", json!({"ids":[2,ellipse]}))["id"].as_u64().unwrap();
                run(&mut s, "shape.connection.add", json!({"id":group,"x":"Width*0.5","y":"Height*0.6"}));
                group as u32
            } else {
                2
            };
            let c = if compound {
                run(&mut s, "connector.add", json!({"from":1,"to":target,"glue":"dynamic"}))["id"].as_u64().unwrap() as u32
            } else {
                let c = run(&mut s, "master.drop", json!({"master":"Dynamic connector"}))["id"].as_u64().unwrap() as u32;
                for (end, target) in [("begin", 1), ("end", target)] {
                    run(&mut s, "connect.dynamic", json!({"id":c,"target":target,"end":end}));
                }
                c
            };
            close(&numbers(&s, c, &["BeginX", "BeginY", "EndX", "EndY"]), &[2.0035, 2.12, 3.55, 0.9]);
            close(
                &numbers(&s, c, &["Geometry1.X1", "Geometry1.Y1", "Geometry1.X2", "Geometry1.Y2", "Geometry1.X3", "Geometry1.Y3"]),
                &[0., 0., 0., -1.22, 1.5465, -1.22],
            );
        }
    }
}

fn set(sheet: &mut misterdiagram_model::Sheet, name: &str, value: f64) {
    let c = misterdiagram_model::Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() };
    if let Some(old) = sheet.cells.iter_mut().find(|c| c.name == name) {
        *old = c;
    } else {
        sheet.cells.push(c);
    }
}
fn picture_stencil(format: &str, grouped: bool, small: bool) -> std::path::PathBuf {
    use misterdiagram_model::{Part, Relationship, XmlElement, XmlNode, skeleton};
    let mut doc = skeleton::document(false);
    let mut child = skeleton::rectangle(5, 0., 0., 0.63, 1., "").unwrap();
    child.shape_type = Some("Foreign".into());
    if format != "faceplate" {
        child.sheet.sections.clear();
    }
    for (n, v) in [
        ("ImgWidth", if small { 0.3 } else { 0.63 }),
        ("ImgHeight", if small { 0.4 } else { 1. }),
        ("ImgOffsetX", if small { 0.1 } else { 0. }),
        ("ImgOffsetY", if small { 0.2 } else { 0. }),
    ] {
        set(&mut child.sheet, n, v);
    }
    let ns = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    child.foreign_data = Some(XmlElement {
        name: "ForeignData".into(),
        namespace: skeleton::VISIO_NS.into(),
        attributes: [("ForeignType".into(), if format == "emf" { "EnhMetaFile" } else { "Bitmap" }.into())].into(),
        children: vec![XmlNode::Element(XmlElement {
            name: "Rel".into(),
            namespace: skeleton::VISIO_NS.into(),
            attributes: [("r:id".into(), "picture".into()), ("xmlns:r".into(), ns.into())].into(),
            attribute_namespaces: [("r:id".into(), ns.into())].into(),
            ..Default::default()
        })],
        ..Default::default()
    });
    if format == "faceplate" {
        child.shape_type = None;
        child.foreign_data = None;
    }
    let shape = if grouped {
        let mut group = skeleton::rectangle(7, 0., 0., 0.63, 1., "").unwrap();
        group.shape_type = Some("Group".into());
        if format == "faceplate" {
            for section in group.sheet.sections.iter_mut().filter(|s| s.name == "Geometry") {
                section.cells.push(misterdiagram_model::Cell { name: "NoShow".into(), value: Some("1".into()), ..Default::default() });
            }
        } else {
            group.sheet.sections.clear();
        }
        group.shapes = vec![child];
        group
    } else {
        child
    };
    doc.masters[0].shapes = vec![shape].into();
    doc.masters[0].name = "Invented picture".into();
    doc.masters[0].name_u = "Invented picture".into();
    let bytes = if format == "emf" {
        original_emf()
    } else {
        let mut out = std::io::Cursor::new(Vec::new());
        image::RgbImage::from_pixel(8, 8, image::Rgb([40, 90, 180])).write_to(&mut out, image::ImageFormat::Png).unwrap();
        out.into_inner()
    };
    let media = format!("visio/media/original.{format}");
    doc.parts.insert(
        media.clone(),
        Part { bytes: bytes.into(), content_type: Some(if format == "emf" { "image/x-emf" } else { "image/png" }.into()), ..Default::default() },
    );
    doc.parts.entry(doc.masters[0].part.clone()).or_default().relationships.push(Relationship {
        id: "picture".into(),
        type_uri: format!("{ns}/image"),
        target: media,
        external: false,
    });
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/brief26b-test-inputs");
    std::fs::create_dir_all(&root).unwrap();
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let serial = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let path = root.join(format!("{}-{serial}-{format}-{grouped}-{small}.vssx", std::process::id()));
    std::fs::write(&path, misterdiagram_vsdx::write_as(&doc, misterdiagram_vsdx::SaveKind::Stencil).unwrap().bytes).unwrap();
    path
}
fn original_emf() -> Vec<u8> {
    fn rec(out: &mut Vec<u8>, kind: u32, values: &[u32]) {
        for n in [kind, 8 + 4 * values.len() as u32].into_iter().chain(values.iter().copied()) {
            out.extend_from_slice(&n.to_le_bytes());
        }
    }
    let mut records = Vec::new();
    rec(&mut records, 37, &[0x80000008]);
    for (i, c) in [0x2828dc, 0x46b41e, 0xdc461e].into_iter().enumerate() {
        rec(&mut records, 39, &[i as u32 + 1, 0, c, 0]);
        rec(&mut records, 37, &[i as u32 + 1]);
        rec(&mut records, 43, &[i as u32 * 100, 0, (i as u32 + 1) * 100, 200]);
    }
    rec(&mut records, 14, &[0, 0, 20]);
    let mut bytes = Vec::new();
    for n in [1u32, 88, 0, 0, 300, 200, 0, 0, 7620, 5080, 0x464d4520, 0x10000, 88 + records.len() as u32, 12, 4, 0, 0, 0, 300, 200, 76, 51] {
        bytes.extend_from_slice(&n.to_le_bytes());
    }
    bytes.extend(records);
    bytes
}
#[test]
fn original_image_group_witness_endpoints_and_routes() {
    for format in ["png", "emf"] {
        for kind in ["full", "small", "plain", "rotate", "flip", "side-small", "side-flip-small"] {
            for compound in [false, true] {
                let path = picture_stencil(format, kind != "plain", kind.contains("small"));
                let mut s = Session::new();
                run(&mut s, "stencil.open", json!({"path":path}));
                let a = run(&mut s, "master.drop", json!({"master":"Invented picture","x":2.5,"y":6.4}))["id"].as_u64().unwrap() as u32;
                if kind == "rotate" {
                    run(&mut s, "shape.rotate", json!({"id":a,"angle":std::f64::consts::FRAC_PI_2}));
                }
                if kind.contains("flip") {
                    run(&mut s, "sheet.set", json!({"id":a,"name":"FlipX","value":1}));
                }
                // The vendor case has an interior connection point, while other witnesses do not.
                if format == "png" && kind == "full" {
                    run(&mut s, "shape.connection.add", json!({"id":a,"x":"Width*0.49","y":"Height*0.48"}));
                }
                let side = kind.starts_with("side");
                let b = run(
                    &mut s,
                    "shape.draw.rect",
                    if side { json!({"x":5,"y":6,"width":1,"height":1}) } else { json!({"x":2,"y":2,"width":1.02,"height":1}) },
                )["id"]
                    .as_u64()
                    .unwrap() as u32;
                let c = if compound {
                    run(&mut s, "connector.add", json!({"from":a,"to":b,"glue":"dynamic"}))["id"].as_u64().unwrap() as u32
                } else {
                    let c = run(&mut s, "master.drop", json!({"stencil":0,"master":"Dynamic connector"}))["id"].as_u64().unwrap() as u32;
                    for (end, target) in [("begin", a), ("end", b)] {
                        run(&mut s, "connect.dynamic", json!({"id":c,"target":target,"end":end}));
                    }
                    c
                };
                let expected = if side {
                    let x = if format == "png" {
                        2.815
                    } else if kind.contains("flip") {
                        2.715
                    } else {
                        2.584212598425197
                    };
                    [x, 6.45, 5., 6.45]
                } else {
                    let y = if format == "emf" {
                        if kind == "small" {
                            6.098425196850394
                        } else if kind == "rotate" {
                            6.0851
                        } else {
                            5.9001
                        }
                    } else if kind == "rotate" {
                        6.085
                    } else {
                        5.9
                    };
                    [2.5, y, 2.5, 3.]
                };
                close(&numbers(&s, c, &["BeginX", "BeginY", "EndX", "EndY"]), &expected);
                let route = if side { [0., 0.125, expected[2] - expected[0], 0.125] } else { [0.125, 0., 0.125, expected[3] - expected[1]] };
                close(&numbers(&s, c, &["Geometry1.X1", "Geometry1.Y1", "Geometry1.X2", "Geometry1.Y2"]), &route);
            }
        }
    }
}

#[test]
fn signed_zero_axis_from_visio_survives_rerouting() {
    let mut s = Session::new();
    for y in [1, 5] {
        run(&mut s, "shape.draw.rect", json!({"x":1,"y":y,"width":1,"height":1}));
    }
    let c = run(&mut s, "connector.add", json!({"from":1,"to":2,"glue":"dynamic"}))["id"].as_u64().unwrap() as u32;
    run(&mut s, "sheet.set", json!({"id":c,"name":"Width","value":-0.25,"formula":"GUARD(-0.25DL)","force":true}));
    run(&mut s, "route.reroute", json!({}));
    close(&numbers(&s, c, &["Width", "Geometry1.X1", "Geometry1.X2"]), &[-0.25, -0.125, -0.125]);
}

#[test]
fn hidden_faceplate_control_and_fixed_image_line_use_native_edges() {
    for format in ["faceplate", "png"] {
        let path = picture_stencil(format, true, false);
        let mut s = Session::new();
        run(&mut s, "stencil.open", json!({"path":path}));
        let a = run(&mut s, "master.drop", json!({"master":"Invented picture","x":2.5,"y":6.4}))["id"].as_u64().unwrap();
        run(&mut s, "shape.connection.add", json!({"id":a,"x":"Width*0.49","y":"Height*0.48"}));
        let b = run(&mut s, "shape.draw.rect", json!({"x":2.02,"y":3,"width":1,"height":1}))["id"].as_u64().unwrap();
        let c = run(&mut s, "shape.draw.line", json!({"x1":2.5,"y1":6.4,"x2":2.5,"y2":3.5}))["id"].as_u64().unwrap();
        for (end, target) in [("begin", a), ("end", b)] {
            run(&mut s, "connect.dynamic", json!({"id":c,"target":target,"end":end}));
        }
        close(&numbers(&s, c as u32, &["BeginX", "BeginY", "EndX", "EndY"]), &[2.5, 5.9, 2.52, 4.]);
    }
}
