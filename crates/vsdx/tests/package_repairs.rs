//! Contributor-original repair fixtures. Native witnesses: field/dangling-connects-2026-10-31
//! and field/duplicate-ids-2026-10-31. No third-party document bytes are embedded.
use misterdiagram_model::{Connect, Document, Shape, skeleton};
use misterdiagram_vsdx::{
    evaluate::{Evaluator, Scope},
    import, import_warnings, write,
};
use std::io::{Cursor, Read, Write};

fn input(shapes: &str, connects: &str, master: bool) -> Vec<u8> {
    let base = write(&skeleton::document(false)).unwrap().bytes;
    let mut input = zip::ZipArchive::new(Cursor::new(base)).unwrap();
    let mut out = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let path = if master { "visio/masters/master1.xml" } else { "visio/pages/page1.xml" };
    let root = if master { "MasterContents" } else { "PageContents" };
    let replacement = format!(
        "<{root} xmlns='{}'><!-- original noncanonical source --><Shapes>{shapes}</Shapes><Connects>{connects}</Connects></{root}>",
        skeleton::VISIO_NS
    );
    for i in 0..input.len() {
        let mut part = input.by_index(i).unwrap();
        let mut bytes = Vec::new();
        part.read_to_end(&mut bytes).unwrap();
        out.start_file(part.name(), zip::write::SimpleFileOptions::default()).unwrap();
        out.write_all(if part.name() == path { replacement.as_bytes() } else { &bytes }).unwrap();
    }
    out.finish().unwrap().into_inner()
}
fn shape(id: u32, name: &str, width: u32) -> String {
    format!(
        "<Shape ID='{id}' NameU='{name}' Name='{name}'><Cell N='Width' V='{width}'/><Cell N='Height' V='1'/><Extra xmlns='urn:original'>{name}</Extra></Shape>"
    )
}
fn group(id: u32, name: &str, children: &str) -> String {
    format!("<Shape ID='{id}' Type='Group' NameU='{name}'><Shapes>{children}</Shapes></Shape>")
}
fn row(from: u32, to: u32) -> String {
    format!("<Connect FromSheet='{from}' ToSheet='{to}' FromCell='BeginX' ToCell='PinX' FromPart='9' ToPart='3' original='attribute'/>")
}
fn contents(d: &Document, master: bool) -> (&[Shape], &[Connect]) {
    if master { (&d.masters[0].shapes, &d.masters[0].connects) } else { (&d.pages[0].shapes, &d.pages[0].connects) }
}
fn inventory(shapes: &[Shape]) -> Vec<(u32, String)> {
    let mut out = Vec::new();
    let mut pending: Vec<_> = shapes.iter().rev().collect();
    while let Some(s) = pending.pop() {
        out.push((s.id, s.name_u.clone()));
        pending.extend(s.shapes.iter().rev());
    }
    out
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
#[test]
fn imported_dangling_connects_are_repaired_on_open_and_reported_on_save() {
    for master in [false, true] {
        let shapes = format!(
            "{}{}<Shape ID='7' Del='1'/><Shape ID='8' Del='true'><Shapes><Shape ID='9'/></Shapes></Shape>",
            shape(5, "source", 2),
            shape(6, "target", 3)
        );
        let valid = [(5, 6), (5, 0), (0, 5)];
        let invalid = [(99, 6), (5, 99), (98, 99), (7, 6), (5, 7), (5, 9)];
        let rows = valid.into_iter().chain(invalid).map(|(a, b)| row(a, b)).collect::<String>();
        let d = import(&input(&shapes, &rows, master)).unwrap();
        let (_, cs) = contents(&d, master);
        assert_eq!(cs.iter().map(|c| (c.from_sheet, c.to_sheet)).collect::<Vec<_>>(), valid);
        assert!(
            cs.iter()
                .all(|c| c.from_part == Some(9) && c.to_part == Some(3) && c.attributes.get("original").map(String::as_str) == Some("attribute"))
        );
        let before = d.clone();
        let w = write(&d).unwrap();
        assert_eq!(w.warning_count, invalid.len());
        assert_eq!(w.warnings.len(), invalid.len());
        assert!(w.warnings.iter().all(|w| w.contains(if master { "master 2" } else { "page 0" }) && w.contains("Connect")));
        let parts = misterdiagram_vsdx::write::write_parts_with_glue_host(&d, None, None, None).unwrap();
        assert_eq!(parts.warnings, w.warnings);
        assert_eq!(parts.warning_count, w.warning_count);
        assert_eq!(d, before, "writing does not mutate its source");
        let clean = import(&w.bytes).unwrap();
        assert_eq!(contents(&clean, master).1, cs);
        let path = if master { "visio/masters/master1.xml" } else { "visio/pages/page1.xml" };
        let raw = clean.parts[path].xml.as_ref().unwrap().child("Connects").unwrap();
        assert_eq!(raw.elements().filter(|e| e.is_visio("Connect")).count(), valid.len(), "saved bytes contain no hidden dangling rows");
        assert_eq!(write(&clean).unwrap().warning_count, 0);
    }
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
#[test]
fn duplicate_shapes_keep_first_depth_first_occurrence_and_drop_loser_subtrees() {
    let first = shape(5, "First", 2);
    let second = shape(5, "Second", 9);
    let cases = [
        (format!("{first}{second}"), vec![5]),
        (format!("{}{second}", group(20, "Group", &first)), vec![20, 5]),
        (format!("{first}{}", group(20, "Group", &second)), vec![5, 20]),
        (group(20, "Group", &format!("{first}{second}")), vec![20, 5]),
        (format!("{}{}", group(20, "FirstGroup", &first), group(20, "SecondGroup", &shape(6, "DiscardedChild", 7))), vec![20, 5]),
    ];
    for master in [false, true] {
        for (shapes, expected) in &cases {
            // The unrelated unchanged source shape forces the byte-preservation path.
            let xml = format!("{shapes}{}", shape(90, "Unchanged", 1));
            let d = import(&input(&xml, &row(90, 5), master)).unwrap();
            let (ss, cs) = contents(&d, master);
            assert_eq!(inventory(ss).iter().map(|x| x.0).collect::<Vec<_>>(), expected.iter().copied().chain([90]).collect::<Vec<_>>());
            assert_eq!(cs.len(), 1);
            let scope = if master { Scope::MasterShape { master: 2, shape: 5 } } else { Scope::PageShape { page: 0, shape: 5 } };
            assert_eq!(Evaluator::new(&d).unwrap().evaluate(scope, "Width").numeric().unwrap(), 2.);
            let w = write(&d).unwrap();
            assert_eq!(w.warning_count, 1);
            assert!(w.warnings[0].contains(if master { "master 2" } else { "page 0" }));
            assert!(w.warnings[0].contains(&format!("shape ID {}", if expected == &vec![20, 5] && shapes.contains("SecondGroup") { 20 } else { 5 })));
            let clean = import(&w.bytes).unwrap();
            assert_eq!(inventory(contents(&clean, master).0), inventory(ss));
            let path = if master { "visio/masters/master1.xml" } else { "visio/pages/page1.xml" };
            let text = String::from_utf8(clean.parts[path].bytes.to_vec()).unwrap();
            assert!(!text.contains("Second"));
            assert!(text.contains(">First</Extra>"));
            assert_eq!(import_warnings(&clean), (vec![], 0));
            assert_eq!(write(&clean).unwrap().warning_count, 0);
        }
    }
}
#[test]
fn caller_built_models_use_the_same_repairs_after_resource_validation() {
    let mut d = skeleton::document(false);
    d.pages[0].shapes =
        vec![skeleton::rectangle(5, 1., 1., 1., 1., "First").unwrap(), skeleton::rectangle(5, 5., 1., 1., 1., "Second").unwrap()].into();
    d.pages[0].connects.push(Connect { from_sheet: 5, to_sheet: 99, ..Default::default() });
    let before = d.clone();
    let w = write(&d).unwrap();
    assert_eq!(w.warnings.iter().filter(|w| w.contains(": dropped ")).count(), 2);
    assert_eq!(w.warning_count, w.warnings.len());
    assert_eq!(d, before);
    let clean = import(&w.bytes).unwrap();
    assert_eq!(clean.pages[0].shapes.len(), 1);
    assert!(clean.pages[0].connects.is_empty());
    d.pages[0].shapes = vec![Shape { id: 5, ..Default::default() }; 100_001].into();
    assert!(write(&d).unwrap_err().to_string().contains("count"));
    let mut deep = Shape { id: 6, ..Default::default() };
    for id in 0..130 {
        deep = Shape { id: id + 10, shapes: vec![deep], ..Default::default() };
    }
    let loser = Shape { id: 5, shapes: vec![deep], ..Default::default() };
    d.pages[0].shapes = vec![Shape { id: 5, ..Default::default() }, loser].into();
    assert!(write(&d).unwrap_err().to_string().contains("nesting"));
    d.pages[0].shapes.clear();
    d.pages[0].connects = vec![Connect { from_sheet: 5, to_sheet: 99, ..Default::default() }; 100_001].into();
    assert!(write(&d).unwrap_err().to_string().contains("connect count"));
}
#[test]
fn reader_checks_original_resource_limits_and_endpoint_syntax_before_repair() {
    let too_many = "<Shape ID='5'/>".repeat(100_001);
    assert!(import(&input(&too_many, "", false)).is_err());
    let deep = format!("{}{}{}", "<Shape ID='5'/>", "<Shape ID='5'><Shapes>".repeat(130), "</Shapes></Shape>".repeat(130));
    assert!(import(&input(&deep, "", false)).is_err());
    for xml in ["<Connect ToSheet='99'/>", "<Connect FromSheet='not-a-number' ToSheet='99'/>", "<Connect FromSheet='1' ToSheet='4294967296'/>"] {
        assert!(import(&input("", xml, false)).is_err());
    }
    let d = import(&input("", &row(99, 98).repeat(105), false)).unwrap();
    assert_eq!(import_warnings(&d).0.len(), 100);
    assert_eq!(import_warnings(&d).1, 105);
    let w = write(&d).unwrap();
    assert_eq!(w.warnings.len(), 100);
    assert_eq!(w.warning_count, 105);
}
#[test]
fn deleted_sentinel_members_keep_distinct_master_shape_payloads() {
    for master in [false, true] {
        let xml = "<Shape ID='4294967295' MasterShape='6' Del='1'><Extra xmlns='urn:original'>six</Extra></Shape><Shape ID='4294967295' MasterShape='7' Del='true'><Extra xmlns='urn:original'>seven</Extra></Shape>";
        let d = import(&input(xml, "", master)).unwrap();
        let w = write(&d).unwrap();
        let clean = import(&w.bytes).unwrap();
        assert_eq!(w.warning_count, 0);
        assert_eq!(contents(&clean, master).0.iter().map(|s| s.master_shape).collect::<Vec<_>>(), [Some(6), Some(7)]);
        let path = if master { "visio/masters/master1.xml" } else { "visio/pages/page1.xml" };
        let text = String::from_utf8(clean.parts[path].bytes.to_vec()).unwrap();
        assert!(text.contains(">six</Extra>"));
        assert!(text.contains(">seven</Extra>"));
    }
}
#[test]
fn sentinel_exception_is_narrow_and_endpoint_identity_is_local() {
    for attributes in ["", "Del='1'", "MasterShape='6'"] {
        let xml = format!("<Shape ID='4294967295' {attributes}/><Shape ID='4294967295' {attributes}/>");
        let d = import(&input(&xml, "", false)).unwrap();
        assert_eq!(d.pages[0].shapes.len(), 1);
        assert_eq!(import_warnings(&d).1, 1);
    }
    let mut d = skeleton::document(false);
    d.pages[0].shapes.push(skeleton::rectangle(5, 1., 1., 1., 1., "").unwrap());
    let mut other = skeleton::page(1, "Other", false);
    other.shapes.push(skeleton::rectangle(99, 1., 1., 1., 1., "").unwrap());
    d.pages.push(other);
    d.pages[0].connects.push(Connect { from_sheet: 5, to_sheet: 99, ..Default::default() });
    d.masters[0].connects.push(Connect { from_sheet: 5, to_sheet: 99, ..Default::default() });
    let w = write(&d).unwrap();
    assert_eq!(w.warnings.iter().filter(|w| w.contains(": dropped ")).count(), 2);
    assert_eq!(w.warning_count, w.warnings.len());
    let clean = import(&w.bytes).unwrap();
    assert!(clean.pages[0].connects.is_empty());
    assert!(clean.masters[0].connects.is_empty());
}
#[test]
fn private_oss_repairs_match_native_connects_and_universal_shape_inventory() {
    use std::collections::BTreeSet;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../diagramcraft-corpus/field/oss-2026-10-22");
    if !root.is_dir() {
        eprintln!("skip: private OSS corpus absent");
        return;
    }
    let names = [
        "asposenet__Examples__Data__Working-with-Pages__CopyVisioPage_Out",
        "asposenet__Examples__Data__Working-with-Pages__InsertBlankPage_Out",
        "asposenet__Examples__Data__Working-with-Pages__SetPageOrientation_Out",
        "asposejava__Examples__src__main__resources__Pages__CopyVisioPage_Out",
        "asposejava__Examples__src__main__resources__Shapes__ReplaceShapePicture_Out",
        "asposejava__Examples__src__main__resources__Shapes__ConfigureTimeLine_Out",
        "asposejava__Examples__src__main__resources__Shapes__RefreshMilestone_Out",
        "asposejava__Examples__src__main__resources__Shapes__RefreshTimeLine_Out",
        "asposejava__Examples__src__main__resources__Shapes__SetMilestoneProps_Out",
    ];
    fn ids(shapes: &[Shape], path: &mut Vec<u32>, out: &mut Vec<(Vec<u32>, u32, String)>) {
        for s in shapes {
            out.push((path.clone(), s.id, s.name_u.clone()));
            path.push(s.id);
            ids(&s.shapes, path, out);
            path.pop();
        }
    }
    for (i, name) in names.iter().enumerate() {
        let d = misterdiagram_vsdx::read(root.join("files").join(format!("{name}.vsdx"))).unwrap();
        let saved = write(&d).unwrap();
        assert!(saved.warning_count > 0, "{name}");
        let reopened = import(&saved.bytes).unwrap();
        let native = misterdiagram_vsdx::read(root.join("visio").join(name).join("resaved.vsdx")).unwrap();
        assert_eq!(reopened.pages.len(), native.pages.len());
        for page in &reopened.pages {
            let expected = native.pages.iter().find(|p| p.id == page.id).unwrap();
            let connects =
                |cs: &[Connect]| cs.iter().map(|c| (c.from_sheet, c.from_cell.clone(), c.to_sheet, c.to_cell.clone())).collect::<BTreeSet<_>>();
            assert_eq!(connects(&page.connects), connects(&expected.connects), "{name} page {}", page.id);
            if i >= 5 {
                let mut actual = vec![];
                let mut wanted = vec![];
                ids(&page.shapes, &mut vec![], &mut actual);
                ids(&expected.shapes, &mut vec![], &mut wanted);
                assert_eq!(actual, wanted, "{name}");
            }
        }
        eprintln!("{name}: saved and reopened; native Connect sets{} match", if i >= 5 { " and ID/NameU/nesting" } else { "" });
    }
}
/// Native deleted-duplicates supplement: deleted owners reserve only their own ID.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
#[test]
fn imported_deleted_owners_do_not_reserve_descendant_ids() {
    for master in [false, true] {
        for parent in [false, true] {
            for deleted_first in [false, true] {
                for value in ["1", "true"] {
                    let live = shape(5, "Live", 7);
                    let dead = if parent { group(20, "DeletedParent", &shape(5, "HiddenChild", 2)) } else { shape(5, "Deleted", 2) };
                    let dead = dead.replacen("<Shape ", &format!("<Shape Del='{value}' "), 1);
                    let xml = if deleted_first { format!("{dead}{live}") } else { format!("{live}{dead}") };
                    let d = import(&input(&xml, "", master)).unwrap();
                    let scope = if master { Scope::MasterShape { master: 2, shape: 5 } } else { Scope::PageShape { page: 0, shape: 5 } };
                    let expected_live = parent || !deleted_first;
                    let mut evaluator = Evaluator::new(&d).unwrap();
                    assert_eq!(evaluator.shape(scope).is_some(), expected_live, "master={master} parent={parent} first={deleted_first} Del={value}");
                    if expected_live {
                        assert_eq!(evaluator.evaluate(scope, "Width").numeric().unwrap(), 7.);
                    }
                    let written = write(&d).unwrap();
                    let clean = import(&written.bytes).unwrap();
                    let mut evaluator = Evaluator::new(&clean).unwrap();
                    assert_eq!(evaluator.shape(scope).is_some(), expected_live);
                    if expected_live {
                        assert_eq!(evaluator.evaluate(scope, "Width").numeric().unwrap(), 7.);
                    }
                    assert_eq!(written.warnings.iter().filter(|w| w.contains("duplicate shape ID")).count(), usize::from(!parent));
                    assert_eq!(write(&clean).unwrap().warning_count, 0);
                }
            }
        }
    }
}
#[test]
fn caller_built_deleted_identity_matches_import_and_save() {
    for master in [false, true] {
        for parent in [false, true] {
            for deleted_first in [false, true] {
                let live = skeleton::rectangle(5, 6., 1., 7., 1., "Live").unwrap();
                let mut dead = skeleton::rectangle(if parent { 20 } else { 5 }, 1., 1., 2., 1., "Deleted").unwrap();
                dead.sheet.attributes.insert("Del".into(), "1".into());
                if parent {
                    dead.shape_type = Some("Group".into());
                    dead.shapes.push(skeleton::rectangle(5, 1., 1., 2., 1., "Hidden").unwrap());
                }
                let shapes = if deleted_first { vec![dead, live] } else { vec![live, dead] };
                let mut d = skeleton::document(false);
                if master {
                    d.masters[0].shapes = shapes.into();
                } else {
                    d.pages[0].shapes = shapes.into();
                }
                let scope = if master { Scope::MasterShape { master: 2, shape: 5 } } else { Scope::PageShape { page: 0, shape: 5 } };
                let expected_live = parent || !deleted_first;
                assert_eq!(
                    Evaluator::new(&d).unwrap().shape(scope).is_some(),
                    expected_live,
                    "master={master} parent={parent} first={deleted_first}"
                );
                let clean = import(&write(&d).unwrap().bytes).unwrap();
                let mut evaluator = Evaluator::new(&clean).unwrap();
                assert_eq!(evaluator.shape(scope).is_some(), expected_live);
                if expected_live {
                    assert_eq!(evaluator.evaluate(scope, "Width").numeric().unwrap(), 7.);
                }
            }
        }
    }
}
#[test]
fn deleted_id_reservations_are_bounded_for_caller_built_models() {
    let mut d = skeleton::document(false);
    for group in 0..2 {
        let mut parent = Shape { id: 200_000 + group, ..Default::default() };
        for id in group * 50_000..(group + 1) * 50_000 {
            let mut deleted = Shape { id, ..Default::default() };
            deleted.sheet.attributes.insert("Del".into(), "1".into());
            parent.shapes.push(deleted);
        }
        d.pages[0].shapes.push(parent);
    }
    assert!(Evaluator::new(&d).is_err());
}
