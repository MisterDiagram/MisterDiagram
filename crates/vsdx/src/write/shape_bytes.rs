//! Preserve unchanged Shape serialization when another shape changes its page.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
use super::{WriteError, error};
use misterdiagram_model::XmlElement;
use quick_xml::{Reader, events::Event};
use std::{collections::BTreeMap, ops::Range, rc::Rc};

#[derive(Default)]
struct Namespaces {
    id: usize,
    bindings: BTreeMap<Rc<str>, Rc<str>>,
    bytes: usize,
}
struct Span {
    range: Range<usize>,
    namespaces: Rc<Namespaces>,
}
struct Frame {
    start: usize,
    id: Option<u32>,
    namespaces: Rc<Namespaces>,
}
fn charge(used: &mut usize, amount: usize, limit: usize, message: &str) -> Result<(), WriteError> {
    *used = used.checked_add(amount).filter(|n| *n <= limit).ok_or_else(|| error(message))?;
    Ok(())
}
fn spans(bytes: &[u8]) -> Result<BTreeMap<u32, Span>, WriteError> {
    spans_with_budget(bytes, misterdiagram_opc::xml::MAX_NODES)
}
fn spans_with_budget(bytes: &[u8], namespace_limit: usize) -> Result<BTreeMap<u32, Span>, WriteError> {
    crate::limits::preflight(bytes).map_err(|e| error(e.to_string()))?;
    let mut reader = Reader::from_reader(bytes);
    let mut stack = Vec::<Frame>::new();
    let mut out = BTreeMap::new();
    let mut count = 0usize;
    let mut namespace_entries = 0usize;
    let mut context_id = 0usize;
    loop {
        let start = usize::try_from(reader.buffer_position()).map_err(|_| error("shape byte offset"))?;
        let event = reader.read_event().map_err(|e| error(format!("shape byte XML: {e}")))?;
        let closes = matches!(event, Event::End(_) | Event::Empty(_));
        match &event {
            Event::Start(e) | Event::Empty(e) => {
                charge(&mut count, 1, misterdiagram_opc::xml::MAX_NODES, "shape byte node count")?;
                if stack.len() >= misterdiagram_opc::xml::MAX_DEPTH {
                    return Err(error("shape byte tree limit"));
                }
                // Sibling shapes share their context. Changed declarations copy
                // only bounded map entries; prefix and URI strings remain shared.
                let mut namespaces = stack.last().map(|f| f.namespaces.clone()).unwrap_or_default();
                let mut changed_context = false;
                let mut id = None;
                for a in e.attributes().with_checks(false) {
                    charge(&mut count, 1, misterdiagram_opc::xml::MAX_NODES, "shape byte node count")?;
                    let a = a.map_err(|e| error(e.to_string()))?;
                    let key = std::str::from_utf8(a.key.as_ref()).map_err(|e| error(e.to_string()))?;
                    if key == "xmlns" || key.starts_with("xmlns:") {
                        let prefix = key.strip_prefix("xmlns:").unwrap_or("");
                        let value = a.unescape_value().map_err(|e| error(e.to_string()))?;
                        if namespaces.bindings.get(prefix).is_some_and(|v| v.as_ref() == value) {
                            continue;
                        }
                        if !changed_context {
                            charge(&mut namespace_entries, namespaces.bindings.len(), namespace_limit, "shape namespace copy limit")?;
                            context_id = context_id.checked_add(1).ok_or_else(|| error("shape namespace count"))?;
                            namespaces = Rc::new(Namespaces { id: context_id, bindings: namespaces.bindings.clone(), bytes: namespaces.bytes });
                            changed_context = true;
                        }
                        charge(&mut namespace_entries, 1, namespace_limit, "shape namespace copy limit")?;
                        let ns = Rc::get_mut(&mut namespaces).ok_or_else(|| error("shape namespace ownership"))?;
                        let prior = ns.bindings.get_key_value(prefix).map_or(0, |(k, v)| k.len().saturating_add(v.len()));
                        let added = prefix.len().checked_add(value.len()).ok_or_else(|| error("shape namespace byte limit"))?;
                        ns.bytes =
                            ns.bytes.checked_sub(prior).and_then(|n| n.checked_add(added)).ok_or_else(|| error("shape namespace byte limit"))?;
                        ns.bindings.insert(Rc::from(prefix), Rc::from(value.as_ref()));
                    } else if key == "ID" {
                        id = a.unescape_value().map_err(|e| error(e.to_string()))?.parse::<u32>().ok();
                    }
                }
                let qname = e.name();
                let name = std::str::from_utf8(qname.as_ref()).map_err(|e| error(e.to_string()))?;
                let (prefix, local) = name.split_once(':').unwrap_or(("", name));
                if local != "Shape"
                    || namespaces.bindings.get(prefix).map(|s| s.as_ref()) != Some("http://schemas.microsoft.com/office/visio/2012/main")
                {
                    id = None;
                }
                stack.push(Frame { start, id, namespaces });
            }
            Event::Eof => {
                if !stack.is_empty() {
                    return Err(error("unclosed shape byte XML"));
                }
                break;
            }
            _ => {}
        }
        if closes {
            let frame = stack.pop().ok_or_else(|| error("unbalanced shape byte XML"))?;
            if let Some(id) = frame.id {
                let end = usize::try_from(reader.buffer_position()).map_err(|_| error("shape byte offset"))?;
                if out.insert(id, Span { range: frame.start..end, namespaces: frame.namespaces }).is_some() {
                    return Err(error("duplicate shape byte ID"));
                }
            }
        }
    }
    Ok(out)
}
fn shapes(root: &XmlElement) -> Result<BTreeMap<u32, &XmlElement>, WriteError> {
    let mut out = BTreeMap::new();
    let mut pending = vec![(root, 0usize)];
    let mut count = 0usize;
    while let Some((e, depth)) = pending.pop() {
        count = count.checked_add(1).ok_or_else(|| error("shape tree count"))?;
        if depth >= misterdiagram_opc::xml::MAX_DEPTH || count > misterdiagram_opc::xml::MAX_NODES {
            return Err(error("shape tree limit"));
        }
        if e.is_visio("Shape")
            && let Some(id) = e.attr("ID").and_then(|s| s.parse::<u32>().ok())
        {
            out.insert(id, e);
        }
        let next = depth.checked_add(1).ok_or_else(|| error("shape tree depth"))?;
        pending.extend(e.elements().map(|e| (e, next)));
        if pending.len() > misterdiagram_opc::xml::MAX_NODES {
            return Err(error("shape tree limit"));
        }
    }
    Ok(out)
}
fn append(out: &mut Vec<u8>, bytes: &[u8]) -> Result<(), WriteError> {
    if out.len().checked_add(bytes.len()).is_none_or(|n| n > misterdiagram_opc::MAX_XML_PART) {
        return Err(error("serialized XML byte limit"));
    }
    out.extend_from_slice(bytes);
    Ok(())
}
pub(super) fn preserve(old: &XmlElement, new: &XmlElement, source: &[u8], rendered: Vec<u8>) -> Result<Vec<u8>, WriteError> {
    if source.len() > misterdiagram_opc::MAX_XML_PART {
        return Err(error("shape source byte limit"));
    }
    let little_endian = source.starts_with(&[0xff, 0xfe]);
    if !little_endian && !source.starts_with(&[0xfe, 0xff]) {
        return preserve_utf8(old, new, source, rendered);
    }
    // OPC also admits UTF-16 parts. Retaining their encoding keeps unchanged
    // Shape spans byte-identical without mixing UTF-16 into a UTF-8 page.
    let decoded = misterdiagram_opc::xml::decoded_xml(source).map_err(|e| error(e.to_string()))?;
    let utf8 = preserve_utf8(old, new, &decoded, rendered)?;
    let text = std::str::from_utf8(&utf8).map_err(|e| error(e.to_string()))?;
    let body = text.strip_prefix("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>").ok_or_else(|| error("shape XML declaration"))?;
    let header = "<?xml version=\"1.0\" encoding=\"UTF-16\" standalone=\"yes\"?>";
    let mut out = Vec::new();
    append(&mut out, if little_endian { &[0xff, 0xfe] } else { &[0xfe, 0xff] })?;
    for unit in header.encode_utf16().chain(body.encode_utf16()) {
        append(&mut out, &if little_endian { unit.to_le_bytes() } else { unit.to_be_bytes() })?;
    }
    Ok(out)
}
fn preserve_utf8(old: &XmlElement, new: &XmlElement, source: &[u8], rendered: Vec<u8>) -> Result<Vec<u8>, WriteError> {
    // Our own strict publications already use canonical XML. In that common
    // authoring case equal subtrees serialize identically without span scans.
    if super::xml::tree(old)? == source {
        return Ok(rendered);
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut pending = vec![old];
    while let Some(node) = pending.pop() {
        if node.is_visio("Shape")
            && let Some(id) = node.attr("ID").and_then(|s| s.parse::<u32>().ok())
            && !seen.insert(id)
        {
            return Ok(rendered);
        }
        pending.extend(node.elements());
    }
    let old_shapes = shapes(old)?;
    let new_shapes = shapes(new)?;
    let same = new_shapes.iter().filter_map(|(id, shape)| (old_shapes.get(id) == Some(shape)).then_some(*id)).collect::<Vec<_>>();
    if same.is_empty() {
        return Ok(rendered);
    }
    let old_spans = spans(source)?;
    let new_spans = spans(&rendered)?;
    let mut replacements = Vec::new();
    let mut equal_contexts = BTreeMap::new();
    let mut comparison_entries = 0usize;
    let mut comparison_bytes = 0usize;
    for id in same {
        let (Some(a), Some(b)) = (old_spans.get(&id), new_spans.get(&id)) else { continue };
        let key = (a.namespaces.id, b.namespaces.id);
        let equal = if let Some(equal) = equal_contexts.get(&key) {
            *equal
        } else {
            charge(&mut comparison_entries, a.namespaces.bindings.len(), misterdiagram_opc::xml::MAX_NODES, "shape namespace comparison limit")?;
            charge(&mut comparison_bytes, a.namespaces.bytes, misterdiagram_opc::MAX_XML_PART, "shape namespace comparison limit")?;
            let equal = a.namespaces.bindings == b.namespaces.bindings;
            equal_contexts.insert(key, equal);
            equal
        };
        if equal {
            replacements.push((&a.range, &b.range));
        }
    }
    replacements.sort_by_key(|(_, range)| range.start);
    let mut out = Vec::new();
    let mut cursor = 0usize;
    for (a, b) in replacements {
        if b.start < cursor {
            continue;
        } // An unchanged enclosing group already retained this child.
        append(&mut out, rendered.get(cursor..b.start).ok_or_else(|| error("shape output range"))?)?;
        append(&mut out, source.get(a.clone()).ok_or_else(|| error("shape source range"))?)?;
        cursor = b.end;
    }
    append(&mut out, rendered.get(cursor..).ok_or_else(|| error("shape output tail"))?)?;
    crate::limits::preflight(&out).map_err(|e| error(e.to_string()))?;
    Ok(out)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_quotes_comments_and_nested_shape_bytes_survive() {
        let source = format!(
            "<PageContents xmlns='{}'><Shapes><Shape ID='1' Name='a &gt; b'><Shapes><Shape ID='2'/></Shapes><!-- original --></Shape><Shape ID='3'/></Shapes></PageContents>",
            "http://schemas.microsoft.com/office/visio/2012/main"
        );
        let old = crate::parse_xml(source.as_bytes()).unwrap();
        let mut new = old.clone();
        new.children.push(misterdiagram_model::XmlNode::Text("changed".into()));
        let rendered = super::super::xml::tree(&new).unwrap();
        let output = preserve(&old, &new, source.as_bytes(), rendered).unwrap();
        assert!(
            String::from_utf8(output.clone())
                .unwrap()
                .contains("<Shape ID='1' Name='a &gt; b'><Shapes><Shape ID='2'/></Shapes><!-- original --></Shape>")
        );
        assert_eq!(crate::parse_xml(&output).unwrap(), new);
    }
    #[test]
    fn namespace_contexts_share_strings_and_bound_copy_work() {
        let source = "<Shapes xmlns='http://schemas.microsoft.com/office/visio/2012/main'><Shape ID='1'/><Shape ID='2'/></Shapes>";
        let parsed = spans_with_budget(source.as_bytes(), 1).unwrap();
        assert!(Rc::ptr_eq(&parsed[&1].namespaces, &parsed[&2].namespaces));
        let churn = "<a xmlns:p='urn:large'><b xmlns:q='urn:two'><c xmlns:r='urn:three'/></b></a>";
        assert!(spans_with_budget(churn.as_bytes(), 4).is_err());
        assert!(spans_with_budget(churn.as_bytes(), 6).is_ok());
        let mut used = misterdiagram_opc::MAX_XML_PART;
        assert!(charge(&mut used, 1, misterdiagram_opc::MAX_XML_PART, "shape namespace comparison limit").is_err());
    }
    #[test]
    fn utf16_shape_bytes_keep_their_original_endianness() {
        let shape = "<Shape ID='1' Name='original &gt; text'/>";
        let source = format!("<PageContents xmlns='http://schemas.microsoft.com/office/visio/2012/main'><Shapes>{shape}</Shapes></PageContents>");
        for little in [false, true] {
            let encode = |s: &str| s.encode_utf16().flat_map(|u| if little { u.to_le_bytes() } else { u.to_be_bytes() }).collect::<Vec<_>>();
            let mut bytes = if little { vec![0xff, 0xfe] } else { vec![0xfe, 0xff] };
            bytes.extend(encode(&source));
            let old = crate::parse_xml(&bytes).unwrap();
            let mut new = old.clone();
            new.children.push(misterdiagram_model::XmlNode::Text("changed".into()));
            let rendered = super::super::xml::tree(&new).unwrap();
            let output = preserve(&old, &new, &bytes, rendered).unwrap();
            assert!(output.starts_with(&bytes[..2]));
            let original_shape = encode(shape);
            assert!(output.windows(original_shape.len()).any(|w| w == original_shape));
            assert_eq!(crate::parse_xml(&output).unwrap(), new);
        }
        let empty = XmlElement::default();
        assert!(preserve(&empty, &empty, &[0xff, 0xfe, 0], Vec::new()).is_err());
    }
    #[test]
    fn hostile_spans_are_bounded_and_rejected() {
        for input in [b"<a><Shape ID='1'>".as_slice(), b"<a></b>", b"<a x='unterminated>"] {
            assert!(spans(input).is_err());
        }
        assert!(spans(format!("{}{}", "<a>".repeat(257), "</a>".repeat(257)).as_bytes()).is_err());
        let duplicate = format!("<Shapes xmlns='{}'><Shape ID='1'/><Shape ID='1'/></Shapes>", "http://schemas.microsoft.com/office/visio/2012/main");
        assert!(spans(duplicate.as_bytes()).is_err());
    }
}
