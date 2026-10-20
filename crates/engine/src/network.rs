//! Native redraws of ecceman/affinity square icons, public domain (Unlicense).
//! Pinned source: 48d4a4fa87957cb542678ed5952227028b8657f5; see ATTRIBUTION.md.
//! The FC switch is contributor-original, MIT OR Apache-2.0.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
use crate::{Result, edit::*, operation};
use misterdiagram_model::{Document, Row, Section};

// Each point is an integer on a 48-unit, 0.75-inch icon canvas. Paths are
// centerline redraws, not embedded SVG outlines. A single weight applies to all.
type Path = &'static [(i16, i16)];
const FRAME: Path = &[(3, 0), (45, 0), (48, 3), (48, 45), (45, 48), (3, 48), (0, 45), (0, 3), (3, 0)];
const CLOUD: &[Path] = &[&[
    (10, 14),
    (7, 16),
    (6, 20),
    (7, 24),
    (10, 27),
    (14, 27),
    (14, 31),
    (17, 36),
    (22, 39),
    (28, 39),
    (33, 36),
    (35, 31),
    (35, 27),
    (39, 26),
    (42, 23),
    (42, 19),
    (40, 16),
    (36, 14),
    (10, 14),
]];
const FIREWALL: &[Path] = &[
    &[(8, 12), (40, 12), (40, 36), (8, 36), (8, 12)],
    &[(8, 20), (40, 20)],
    &[(8, 28), (40, 28)],
    &[(16, 12), (16, 20)],
    &[(32, 12), (32, 20)],
    &[(24, 20), (24, 28)],
    &[(16, 28), (16, 36)],
    &[(32, 28), (32, 36)],
];
const ROUTER: &[Path] = &[
    &[(24, 28), (24, 40)],
    &[(19, 35), (24, 40), (29, 35)],
    &[(24, 20), (24, 8)],
    &[(19, 13), (24, 8), (29, 13)],
    &[(8, 24), (20, 24)],
    &[(15, 19), (20, 24), (15, 29)],
    &[(40, 24), (28, 24)],
    &[(33, 19), (28, 24), (33, 29)],
];
const SWITCH: &[Path] = &[
    &[(9, 31), (22, 31)],
    &[(14, 26), (9, 31), (14, 36)],
    &[(26, 31), (39, 31)],
    &[(34, 26), (39, 31), (34, 36)],
    &[(9, 17), (22, 17)],
    &[(17, 12), (22, 17), (17, 22)],
    &[(26, 17), (39, 17)],
    &[(31, 12), (26, 17), (31, 22)],
];
const CORE: &[Path] = &[
    &[(20, 18), (28, 18), (30, 20), (30, 28), (28, 30), (20, 30), (18, 28), (18, 20), (20, 18)],
    &[(24, 30), (24, 42)],
    &[(20, 38), (24, 42), (28, 38)],
    &[(24, 18), (24, 6)],
    &[(20, 10), (24, 6), (28, 10)],
    &[(30, 24), (42, 24)],
    &[(38, 20), (42, 24), (38, 28)],
    &[(18, 24), (6, 24)],
    &[(10, 20), (6, 24), (10, 28)],
    &[(30, 30), (39, 39)],
    &[(33, 39), (39, 39), (39, 33)],
    &[(18, 18), (9, 9)],
    &[(9, 15), (9, 9), (15, 9)],
    &[(18, 30), (9, 39)],
    &[(9, 33), (9, 39), (15, 39)],
    &[(30, 18), (39, 9)],
    &[(33, 9), (39, 9), (39, 15)],
];
const WIFI: &[Path] = &[
    &[(24, 8), (24, 23)],
    &[(21, 24), (21, 28), (24, 30), (27, 28), (27, 24), (24, 22), (21, 24)],
    &[(16, 19), (13, 24), (13, 29), (16, 34)],
    &[(32, 19), (35, 24), (35, 29), (32, 34)],
    &[(11, 14), (7, 21), (6, 26), (7, 32), (11, 39)],
    &[(37, 14), (41, 21), (42, 26), (41, 32), (37, 39)],
];
const SERVER: &[Path] = &[
    &[(16, 10), (32, 10), (32, 38), (16, 38), (16, 10)],
    &[(19, 32), (29, 32)],
    &[(19, 27), (29, 27)],
    &[(22, 16), (26, 16), (26, 20), (22, 20), (22, 16)],
];
const DESKTOP: &[Path] = &[&[(10, 17), (38, 17), (38, 36), (10, 36), (10, 17)], &[(24, 17), (24, 10)], &[(16, 10), (32, 10)]];
const LAPTOP: &[Path] = &[&[(12, 18), (36, 18), (36, 35), (12, 35), (12, 18)], &[(12, 18), (8, 12), (40, 12), (36, 18)], &[(20, 15), (28, 15)]];
const STORAGE: &[Path] = &[
    &[(9, 33), (12, 37), (18, 39), (30, 39), (36, 37), (39, 33), (36, 29), (30, 27), (18, 27), (12, 29), (9, 33)],
    &[(9, 33), (9, 14), (12, 10), (18, 8), (30, 8), (36, 10), (39, 14), (39, 33)],
    &[(9, 24), (12, 20), (18, 18), (30, 18), (36, 20), (39, 24)],
];
// Original line-drawn FC letters distinguish SAN switching without a font asset.
const FC: &[Path] = &[
    &[(9, 31), (39, 31)],
    &[(14, 26), (9, 31), (14, 36)],
    &[(34, 26), (39, 31), (34, 36)],
    &[(15, 10), (15, 22), (22, 22)],
    &[(15, 17), (21, 17)],
    &[(34, 21), (31, 22), (27, 20), (27, 12), (31, 10), (34, 11)],
];
const ICONS: &[(&str, &str, &str, &[Path])] = &[
    ("network.cloud", "Cloud", "#58758C", CLOUD),
    ("network.firewall", "Firewall", "#A78245", FIREWALL),
    ("network.router", "Router", "#477F7D", ROUTER),
    ("network.switch.core", "Core switch", "#477F7D", CORE),
    ("network.switch.access", "Access switch", "#477F7D", SWITCH),
    ("network.wireless_ap", "Wireless AP", "#70827F", WIFI),
    ("compute.server", "Server", "#6C737C", SERVER),
    ("compute.desktop", "Desktop", "#8B9299", DESKTOP),
    ("compute.laptop", "Laptop", "#8B9299", LAPTOP),
    ("storage.array", "Storage array", "#85749C", STORAGE),
    ("storage.switch", "FC switch", "#85749C", FC),
];
/// Construct the compiled stencil with native Geometry and shared text transforms.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/text-transform-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
pub(crate) fn document() -> Result<Document> {
    let mut doc = crate::files::blank_document(false);
    doc.masters.clear();
    for (ordinal, (name, label, color, paths)) in ICONS.iter().enumerate() {
        let id = u32::try_from(ordinal + 2).map_err(operation)?;
        let mut shape = misterdiagram_model::skeleton::rectangle(5, 0., 0., 1.5, 1.25, label).map_err(operation)?;
        shape.sheet.sections.clear();
        if let Some(text) = &mut shape.text {
            text.children.insert(
                0,
                misterdiagram_model::XmlNode::Element(misterdiagram_model::XmlElement {
                    name: "cp".into(),
                    namespace: misterdiagram_model::skeleton::VISIO_NS.into(),
                    attributes: [("IX".into(), "0".into())].into(),
                    ..Default::default()
                }),
            );
        }
        for (key, value, formula) in [
            ("LineColor", *color, None),
            ("FillPattern", "0", None),
            ("LineWeight", "0.03125", None),
            ("LinePattern", "1", None),
            ("Rounding", "0", None),
            ("TxtPinX", "0.75", Some("Width*0.5")),
            ("TxtPinY", "0.125", None),
            ("TxtWidth", "1.5", Some("Width")),
            ("TxtHeight", "0.25", None),
            ("TxtLocPinX", "0.75", Some("TxtWidth*0.5")),
            ("TxtLocPinY", "0.125", Some("TxtHeight*0.5")),
            ("TxtAngle", "0", None),
            ("LeftMargin", "0", None),
            ("RightMargin", "0", None),
            ("TopMargin", "0", None),
            ("BottomMargin", "0", None),
            ("ShapeKeywords", *name, None),
        ] {
            set(&mut shape.sheet, key, value, formula)?;
        }
        shape.sheet.sections.push(Section {
            name: "Character".into(),
            rows: vec![Row { index: Some(0), cells: vec![cell("Size", "0.125", None), cell("Color", "#39424B", None)], ..Default::default() }],
            ..Default::default()
        });
        let mut geometry = Section {
            name: "Geometry".into(),
            index: Some(0),
            cells: ["NoFill", "NoLine", "NoShow", "NoSnap", "NoQuickDrag"]
                .iter()
                .map(|n| cell(n, if *n == "NoFill" { "1" } else { "0" }, None))
                .collect(),
            ..Default::default()
        };
        for path in std::iter::once(&FRAME).chain(paths.iter()) {
            for (at, (x, y)) in path.iter().enumerate() {
                let dx = (f64::from(*x) - 24.) / 64.;
                let dy = (f64::from(*y) - 56.) / 64.;
                geometry.rows.push(Row {
                    index: Some(u32::try_from(geometry.rows.len() + 1).map_err(operation)?),
                    row_type: Some(if at == 0 { "MoveTo" } else { "LineTo" }.into()),
                    cells: vec![
                        cell("X", &(0.75 + dx).to_string(), Some(&format!("Width*0.5+({dx})"))),
                        cell("Y", &(1.25 + dy).to_string(), Some(&format!("Height+({dy})"))),
                    ],
                    ..Default::default()
                });
            }
        }
        shape.sheet.sections.push(geometry);
        let mut connection = Section { name: "Connection".into(), ..Default::default() };
        let points = std::iter::once((0., 0.5))
            .chain(std::iter::once((1., 0.5)))
            .chain((1..8).flat_map(|i| [(f64::from(i) / 8., 0.), (f64::from(i) / 8., 1.)]));
        for (x, y) in points {
            connection.rows.push(Row {
                index: Some(u32::try_from(connection.rows.len()).map_err(operation)?),
                row_type: Some("Connection".into()),
                cells: vec![
                    cell("X", &(1.5 * x).to_string(), Some(&format!("Width*{x}"))),
                    cell("Y", &(1.25 * y).to_string(), Some(&format!("Height*{y}"))),
                    cell("DirX", "0", None),
                    cell("DirY", "0", None),
                    cell("Type", "0", None),
                    cell("AutoGen", "0", None),
                    cell("Prompt", "", None),
                ],
                ..Default::default()
            });
        }
        shape.sheet.sections.push(connection);
        let mut master = misterdiagram_model::skeleton::dynamic_connector();
        master.id = id;
        master.name = (*name).into();
        master.name_u = (*name).into();
        master.part = format!("visio/masters/master{}.xml", ordinal + 1);
        master.attributes.insert("MasterType".into(), "2".into());
        master.attributes.insert("Prompt".into(), (*label).into());
        master.sheet.sections.clear();
        for (n, v) in [("PageWidth", 1.5), ("PageHeight", 1.25), ("DrawingResizeType", 1.)] {
            set_number(&mut master.sheet, n, v)?;
        }
        master.shapes = vec![shape].into();
        doc.masters.push(master);
    }
    crate::files::update_palette(&mut doc)?;
    let bytes = misterdiagram_vsdx::write_as(&doc, misterdiagram_vsdx::SaveKind::Stencil).map_err(operation)?.bytes;
    misterdiagram_vsdx::import(&bytes).map_err(operation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;
    use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
    use serde_json::json;
    #[test]
    fn network_library_is_native_consistent_and_drops_by_semantic_name() {
        let doc = document().unwrap();
        assert_eq!(doc.masters.len(), 11);
        let mut e = Evaluator::new(&doc).unwrap();
        let mut labels = Vec::new();
        for master in &doc.masters {
            assert_eq!(master.shapes.len(), 1);
            let shape = &master.shapes[0];
            assert!(shape.foreign_data.is_none());
            let scope = Scope::MasterShape { master: master.id, shape: shape.id };
            let values = ["TxtPinX", "TxtPinY", "TxtWidth", "TxtHeight", "TxtLocPinX", "TxtLocPinY", "TxtAngle"]
                .map(|name| e.evaluate(scope, name).numeric().unwrap());
            assert_eq!(values, [0.75, 0.125, 1.5, 0.25, 0.75, 0.125, 0.], "all masters must share the label offset");
            labels.push(values);
            let connection = shape.sheet.sections.iter().find(|s| s.name == "Connection").unwrap();
            assert_eq!(connection.rows.len(), 16);
            assert!(shape.sheet.sections.iter().any(|s| s.name == "Geometry" && s.rows.len() > 10));
            for name in ["Width", "Height", "PinX", "PinY"] {
                assert_eq!((e.evaluate(scope, name).numeric().unwrap() * 64.).fract(), 0.);
            }
        }
        let out = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/brief25");
        std::fs::create_dir_all(&out).unwrap();
        std::fs::write(out.join("network.vssx"), misterdiagram_vsdx::write_as(&doc, misterdiagram_vsdx::SaveKind::Stencil).unwrap().bytes).unwrap();
        let mut s = Session::new();
        assert!(s.execute("stencil.open", &json!({"builtin":"invalid"})).is_err());
        assert!(s.execute("stencil.open", &json!({"builtin":"network","path":"anything"})).is_err());
        s.execute("stencil.open", &json!({"builtin":"network"})).unwrap();
        assert_eq!(s.execute("stencil.open", &json!({"builtin":"network"})).unwrap()["stencil"], 1);
        assert_eq!(s.execute("stencil.search", &json!({"query":"network"})).unwrap()["matched"], 6);
        for (i, (name, _, _, _)) in ICONS.iter().enumerate() {
            let id = s.execute("master.drop", &json!({"master":name,"x":1.+(i%4)as f64*2.,"y":7.-(i/4)as f64*2.})).unwrap()["id"].as_u64().unwrap();
            assert_eq!(id, i as u64 + 1);
        }
        std::fs::write(out.join("network-dropped.vsdx"), misterdiagram_vsdx::write(s.document()).unwrap().bytes).unwrap();
        let count = s.document().masters.len();
        s.execute("master.drop", &json!({"master":"network.cloud"})).unwrap();
        assert_eq!(s.document().masters.len(), count);
        s.execute("file.new", &json!({"discard":true})).unwrap();
        s.execute("master.drop", &json!({"master":"storage.switch"})).unwrap();
    }
}
