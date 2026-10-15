//! Bounded namespace-aware serialization of retained and authored XML trees.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
use super::{WriteError, error};
use diagramcraft_model::{XmlElement, XmlNode};
use std::collections::BTreeMap;
const LIMIT: usize = diagramcraft_opc::MAX_XML_PART;
fn append(out: &mut String, s: &str) -> Result<(), WriteError> {
    if out.len().checked_add(s.len()).is_none_or(|n| n > LIMIT) {
        return Err(error("serialized XML byte limit"));
    }
    out.push_str(s);
    Ok(())
}
fn escaped(out: &mut String, s: &str, attribute: bool) -> Result<(), WriteError> {
    if s.len() > LIMIT {
        return Err(error("XML string byte limit"));
    }
    for c in s.chars() {
        match c {
            '&' => append(out, "&amp;")?,
            '<' => append(out, "&lt;")?,
            '>' => append(out, "&gt;")?,
            '"' if attribute => append(out, "&quot;")?,
            '\r' => append(out, "&#13;")?,
            '\n' if attribute => append(out, "&#10;")?,
            '\t' if attribute => append(out, "&#9;")?,
            c if (c < ' ' && c != '\n' && c != '\t') || c == '\u{fffe}' || c == '\u{ffff}' => return Err(error("invalid XML character")),
            c => append(out, c.encode_utf8(&mut [0; 4]))?,
        }
    }
    Ok(())
}
fn valid_name(s: &str) -> bool {
    !s.is_empty()
        && s.split(':').count() <= 2
        && s.split(':').all(|p| {
            let mut chars = p.chars();
            chars.next().is_some_and(|c| c == '_' || c.is_alphabetic()) && chars.all(|c| c.is_alphanumeric() || matches!(c, '_' | '-' | '.'))
        })
}
pub(super) fn tree(root: &XmlElement) -> Result<Vec<u8>, WriteError> {
    tree_format(root, false)
}
// Observed authored app.xml: CRLF declaration, size before baseType, expanded empty elements.
// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
pub(super) fn authored_app(root: &XmlElement) -> Result<Vec<u8>, WriteError> {
    tree_format(root, true)
}
fn tree_format(root: &XmlElement, app: bool) -> Result<Vec<u8>, WriteError> {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>");
    out.push_str(if app { "\r\n" } else { "\n" });
    let mut count = 0;
    node(root, &BTreeMap::from([("xml".into(), "http://www.w3.org/XML/1998/namespace".into())]), 0, &mut count, &mut out, app)?;
    crate::limits::preflight(out.as_bytes()).map_err(|e| error(e.to_string()))?;
    Ok(out.into_bytes())
}
fn node(e: &XmlElement, parent: &BTreeMap<String, String>, depth: usize, count: &mut usize, out: &mut String, app: bool) -> Result<(), WriteError> {
    *count = count.saturating_add(1);
    if depth >= diagramcraft_opc::xml::MAX_DEPTH || *count > diagramcraft_opc::xml::MAX_NODES || e.attributes.len() > 65536 {
        return Err(error("XML tree limit"));
    }
    if !valid_name(&e.name) {
        return Err(error("invalid XML element name"));
    }
    let mut ns = parent.clone();
    for (k, v) in &e.attributes {
        if k == "xmlns" {
            ns.insert(String::new(), v.clone());
        } else if let Some(p) = k.strip_prefix("xmlns:") {
            ns.insert(p.into(), v.clone());
        }
    }
    let found = ns.iter().find(|(_, v)| **v == e.namespace).map(|(k, _)| k.clone());
    let (name, declare) = match found {
        Some(p) if p.is_empty() => (e.name.clone(), false),
        Some(p) => (format!("{p}:{}", e.name), false),
        None => {
            ns.insert(String::new(), e.namespace.clone());
            (e.name.clone(), true)
        }
    };
    append(out, "<")?;
    append(out, &name)?;
    if declare {
        append(out, " xmlns=\"")?;
        escaped(out, &e.namespace, true)?;
        append(out, "\"")?;
    }
    let mut expanded = std::collections::BTreeSet::new();
    let mut attributes: Vec<_> = e.attributes.iter().collect();
    if app && e.name == "vector" {
        attributes.sort_by_key(|(k, _)| if k.as_str() == "size" { 0 } else { 1 });
    }
    for (k, v) in attributes {
        if declare && k == "xmlns" {
            continue;
        }
        if !valid_name(k) {
            return Err(error("invalid XML attribute name"));
        }
        if let Some((prefix, _)) = k.split_once(':')
            && prefix != "xmlns"
            && !ns.contains_key(prefix)
        {
            return Err(error("unbound XML attribute prefix"));
        }
        if k != "xmlns" && !k.starts_with("xmlns:") {
            let (uri, local) = if let Some((prefix, local)) = k.split_once(':') {
                (ns.get(prefix).map(String::as_str).unwrap_or(""), local)
            } else {
                ("", k.as_str())
            };
            if !expanded.insert((uri, local)) {
                return Err(error("duplicate expanded attribute"));
            }
        }
        append(out, " ")?;
        append(out, k)?;
        append(out, "=\"")?;
        escaped(out, v, true)?;
        append(out, "\"")?;
    }
    if e.children.is_empty() && !app {
        append(out, "/>")?;
        return Ok(());
    }
    append(out, ">")?;
    for c in &e.children {
        match c {
            XmlNode::Text(s) => escaped(out, s, false)?,
            XmlNode::Element(e) => node(e, &ns, depth + 1, count, out, app)?,
        }
    }
    append(out, "</")?;
    append(out, &name)?;
    append(out, ">")
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Authored app.xml uses paired empty tags; ordinary retained XML is unaffected.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
    #[test]
    fn authored_app_empty_elements_keep_visio_spelling() {
        let root = XmlElement {
            name: "Properties".into(),
            namespace: "urn:properties".into(),
            children: vec![XmlNode::Element(XmlElement { name: "Template".into(), namespace: "urn:properties".into(), ..Default::default() })],
            ..Default::default()
        };
        assert!(String::from_utf8(authored_app(&root).unwrap()).unwrap().contains("<Template></Template>"));
        assert!(String::from_utf8(tree(&root).unwrap()).unwrap().contains("<Template/>"));
    }
}
