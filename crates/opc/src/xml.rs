//! A small, bounded XML DOM over quick-xml for reading parts, plus an escaping string builder for
//! writing them.
//!
//! Reading is lenient about namespaces: elements and attributes are matched by local name, which is
//! how real-world files are best understood (prefixes are conventional and stable in practice).

use std::fmt::Write as _;

use quick_xml::events::Event;

use crate::OpcError;

/// Maximum element nesting accepted when parsing.
pub const MAX_DEPTH: usize = 256;
/// Maximum retained elements, attributes and coalesced text nodes in one part.
pub const MAX_NODES: usize = 5_000_000;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct El {
    /// Qualified name as written (`p:sp`).
    pub name: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    El(El),
    Text(String),
}

fn local_of(name: &str) -> &str {
    name.rsplit_once(':').map(|(_, l)| l).unwrap_or(name)
}

impl El {
    pub fn local(&self) -> &str {
        local_of(&self.name)
    }
    pub fn prefix(&self) -> &str {
        self.name.split_once(':').map(|(p, _)| p).unwrap_or("")
    }
    pub fn is(&self, local: &str) -> bool {
        self.local() == local
    }
    /// Attribute by local name (`id` matches `id`; `r:id` matches `id` only when no plain `id`
    /// exists). Pass a qualified name (`r:id`) to require that prefix's local name with any prefix.
    pub fn attr(&self, name: &str) -> Option<&str> {
        if let Some((_, l)) = name.split_once(':') {
            return self.attrs.iter().find(|(k, _)| k.contains(':') && !k.starts_with("xmlns") && local_of(k) == l).map(|(_, v)| v.as_str());
        }
        self.attrs
            .iter()
            .find(|(k, _)| k == name)
            .or_else(|| self.attrs.iter().find(|(k, _)| !k.starts_with("xmlns") && local_of(k) == name))
            .map(|(_, v)| v.as_str())
    }
    pub fn has_attr(&self, name: &str) -> bool {
        self.attr(name).is_some()
    }
    pub fn set_attr(&mut self, name: &str, value: &str) {
        if let Some(a) = self.attrs.iter_mut().find(|(k, _)| k == name) {
            a.1 = value.to_string();
        } else {
            self.attrs.push((name.to_string(), value.to_string()));
        }
    }
    pub fn i64(&self, name: &str) -> Option<i64> {
        let v = self.attr(name)?.trim();
        v.parse::<i64>().ok().or_else(|| v.parse::<f64>().ok().filter(|f| f.is_finite()).map(|f| f.clamp(-9.0e18, 9.0e18) as i64))
    }
    pub fn i32(&self, name: &str) -> Option<i32> {
        self.i64(name).map(|v| v.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
    }
    pub fn u32(&self, name: &str) -> Option<u32> {
        self.i64(name).map(|v| v.clamp(0, u32::MAX as i64) as u32)
    }
    pub fn f64(&self, name: &str) -> Option<f64> {
        self.attr(name)?.trim().parse::<f64>().ok().filter(|f| f.is_finite())
    }
    pub fn bool(&self, name: &str) -> Option<bool> {
        match self.attr(name)?.trim() {
            "1" | "true" | "on" => Some(true),
            "0" | "false" | "off" => Some(false),
            _ => None,
        }
    }
    pub fn elements(&self) -> impl Iterator<Item = &El> {
        self.children.iter().filter_map(|n| match n {
            Node::El(e) => Some(e),
            Node::Text(_) => None,
        })
    }
    pub fn elements_mut(&mut self) -> impl Iterator<Item = &mut El> {
        self.children.iter_mut().filter_map(|n| match n {
            Node::El(e) => Some(e),
            Node::Text(_) => None,
        })
    }
    pub fn child(&self, local: &str) -> Option<&El> {
        self.elements().find(|e| e.is(local))
    }
    pub fn children_named<'a>(&'a self, local: &'a str) -> impl Iterator<Item = &'a El> + 'a {
        self.elements().filter(move |e| e.is(local))
    }
    /// Follow a chain of child local names.
    pub fn path(&self, names: &[&str]) -> Option<&El> {
        let mut cur = self;
        for n in names {
            cur = cur.child(n)?;
        }
        Some(cur)
    }
    /// First descendant (depth-first, bounded) with this local name.
    pub fn find(&self, local: &str) -> Option<&El> {
        fn rec<'a>(e: &'a El, local: &str, depth: usize) -> Option<&'a El> {
            if depth > MAX_DEPTH {
                return None;
            }
            for c in e.elements() {
                if c.is(local) {
                    return Some(c);
                }
                if let Some(f) = rec(c, local, depth + 1) {
                    return Some(f);
                }
            }
            None
        }
        rec(self, local, 0)
    }
    /// All descendants with this local name (bounded depth).
    pub fn find_all<'a>(&'a self, local: &str, out: &mut Vec<&'a El>) {
        fn rec<'a>(e: &'a El, local: &str, depth: usize, out: &mut Vec<&'a El>) {
            if depth > MAX_DEPTH {
                return;
            }
            for c in e.elements() {
                if c.is(local) {
                    out.push(c);
                }
                rec(c, local, depth + 1, out);
            }
        }
        rec(self, local, 0, out)
    }
    /// Concatenated text content of this element and its descendants.
    pub fn text(&self) -> String {
        let mut s = String::new();
        fn rec(e: &El, s: &mut String, depth: usize) {
            if depth > MAX_DEPTH {
                return;
            }
            for c in &e.children {
                match c {
                    Node::Text(t) => s.push_str(t),
                    Node::El(e) => rec(e, s, depth + 1),
                }
            }
        }
        rec(self, &mut s, 0);
        s
    }
    /// Value of `<x val="…"/>` child.
    pub fn child_val(&self, local: &str) -> Option<&str> {
        self.child(local).and_then(|c| c.attr("val"))
    }
    /// Serialize to a string (no XML declaration).
    pub fn to_xml(&self) -> String {
        let mut s = String::new();
        write_el(self, &mut s, 0);
        s
    }
    /// Remove descendant elements with any of these local names (bounded).
    pub fn remove_all(&mut self, locals: &[&str]) {
        fn rec(e: &mut El, locals: &[&str], depth: usize) {
            if depth > MAX_DEPTH {
                return;
            }
            e.children.retain(|c| !matches!(c, Node::El(x) if locals.contains(&x.local())));
            for c in e.elements_mut() {
                rec(c, locals, depth + 1);
            }
        }
        rec(self, locals, 0)
    }
}

fn write_el(e: &El, s: &mut String, depth: usize) {
    if depth > MAX_DEPTH {
        return;
    }
    s.push('<');
    s.push_str(&e.name);
    for (k, v) in &e.attrs {
        let _ = write!(s, " {}=\"{}\"", k, esc_attr(v));
    }
    if e.children.is_empty() {
        s.push_str("/>");
        return;
    }
    s.push('>');
    for c in &e.children {
        match c {
            Node::Text(t) => s.push_str(&esc(t)),
            Node::El(c) => write_el(c, s, depth + 1),
        }
    }
    s.push_str("</");
    s.push_str(&e.name);
    s.push('>');
}

/// A parsed document: its root element and the namespace declarations on the root.
#[derive(Clone, Debug, Default)]
pub struct Doc {
    pub root: El,
}

impl Doc {
    /// Namespace declarations (`xmlns:x`, `xmlns`) on the root element.
    pub fn ns_decls(&self) -> Vec<(String, String)> {
        self.root.attrs.iter().filter(|(k, _)| k == "xmlns" || k.starts_with("xmlns:")).cloned().collect()
    }
}

/// Build a DOM directly from validated XML events, without an intermediate tree.
pub trait TreeBuilder {
    type Element;
    type Error: From<OpcError>;
    fn start(&mut self, element: El) -> Result<Self::Element, Self::Error>;
    fn end(&mut self);
    fn element(parent: &mut Self::Element, child: Self::Element);
    fn last_text(parent: &mut Self::Element) -> Option<&mut String>;
    fn text(parent: &mut Self::Element, text: String);
}
struct RawTree;
impl TreeBuilder for RawTree {
    type Element = El;
    type Error = OpcError;
    fn start(&mut self, element: El) -> Result<El, OpcError> {
        Ok(element)
    }
    fn end(&mut self) {}
    fn element(parent: &mut El, child: El) {
        parent.children.push(Node::El(child));
    }
    fn last_text(parent: &mut El) -> Option<&mut String> {
        if let Some(Node::Text(text)) = parent.children.last_mut() { Some(text) } else { None }
    }
    fn text(parent: &mut El, text: String) {
        parent.children.push(Node::Text(text));
    }
}

/// Parse XML bytes into a DOM. Bounded depth and node count; entity references are resolved.
pub fn parse(bytes: &[u8]) -> Result<Doc, OpcError> {
    parse_with_builder(bytes, &mut RawTree).map(|root| Doc { root })
}

/// Parse directly into the consumer's retained representation, under the same XML limits.
pub fn parse_with_builder<B: TreeBuilder>(bytes: &[u8], builder: &mut B) -> Result<B::Element, B::Error> {
    parse_with_node_limit(bytes, MAX_NODES, builder)
}
fn parse_with_node_limit<B: TreeBuilder>(bytes: &[u8], limit: usize, builder: &mut B) -> Result<B::Element, B::Error> {
    if bytes.len() > crate::MAX_XML_PART {
        return Err(OpcError::Xml("XML byte cap exceeded".into()).into());
    }
    // Skip a UTF-8 BOM; UTF-16 parts (rare) are decoded first.
    let owned;
    let bytes = if bytes.starts_with(&[0xFF, 0xFE]) || bytes.starts_with(&[0xFE, 0xFF]) {
        owned = decode_utf16(bytes)?;
        owned.as_bytes()
    } else {
        bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes)
    };
    let mut reader = quick_xml::Reader::from_reader(bytes);
    {
        let c = reader.config_mut();
        c.trim_text(false);
        c.check_end_names = true;
        c.allow_unmatched_ends = false;
        c.expand_empty_elements = false;
    }
    let mut stack: Vec<B::Element> = vec![];
    let mut root: Option<B::Element> = None;
    let mut nodes = NodeBudget { used: 0, limit };
    let mut buf = Vec::new();
    loop {
        let ev = reader.read_event_into(&mut buf).map_err(|e| OpcError::Xml(format!("{e}")))?;
        match ev {
            Event::Start(_) | Event::Empty(_) if root.is_some() => {
                return Err(OpcError::Xml("multiple root elements".into()).into());
            }
            Event::Start(ref b) | Event::Empty(ref b) => {
                nodes.charge()?;
                if stack.len() >= MAX_DEPTH {
                    return Err(OpcError::Xml("document too large or too deeply nested".into()).into());
                }
                let el = builder.start(start_el(b, &reader, &mut nodes)?)?;
                if matches!(ev, Event::Start(_)) {
                    stack.push(el);
                } else {
                    builder.end();
                    match stack.last_mut() {
                        Some(p) => B::element(p, el),
                        None => root = Some(el),
                    }
                }
            }
            Event::End(_) => {
                if let Some(el) = stack.pop() {
                    builder.end();
                    match stack.last_mut() {
                        Some(p) => B::element(p, el),
                        None => root = Some(el),
                    }
                }
            }
            Event::Text(t) => {
                // XML1.0 normalizes physical CR/CRLF before character-reference expansion.
                // https://www.w3.org/TR/xml/#sec-line-ends
                let text = t.xml10_content().map_err(xml_error)?;
                check_chars(&text)?;
                if let Some(p) = stack.last_mut() {
                    push_text::<B>(p, &text, &mut nodes)?;
                } else if !text.trim().is_empty() {
                    return Err(xml_error("text outside root").into());
                }
            }
            Event::CData(t) => {
                // XML1.0 normalizes physical CR/CRLF before character-reference expansion.
                // https://www.w3.org/TR/xml/#sec-line-ends
                let text = t.xml10_content().map_err(xml_error)?;
                check_chars(&text)?;
                let p = stack.last_mut().ok_or_else(|| xml_error("CDATA outside root"))?;
                push_text::<B>(p, &text, &mut nodes)?;
            }
            Event::GeneralRef(r) => {
                let p = stack.last_mut().ok_or_else(|| xml_error("entity outside root"))?;
                let text = if r.is_char_ref() {
                    r.resolve_char_ref().map_err(xml_error)?.ok_or_else(|| xml_error("invalid character reference"))?.to_string()
                } else {
                    match r.decode().map_err(xml_error)?.as_ref() {
                        "amp" => "&",
                        "lt" => "<",
                        "gt" => ">",
                        "quot" => "\"",
                        "apos" => "'",
                        _ => return Err(xml_error("undeclared entity").into()),
                    }
                    .to_string()
                };
                check_chars(&text)?;
                push_text::<B>(p, &text, &mut nodes)?;
            }
            Event::DocType(_) => return Err(xml_error("DOCTYPE is not supported").into()),
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    if !stack.is_empty() {
        return Err(OpcError::Xml("unclosed element".into()).into());
    }
    let root = root.ok_or_else(|| OpcError::Xml("no root element".into()))?;
    Ok(root)
}

struct NodeBudget {
    used: usize,
    limit: usize,
}
impl NodeBudget {
    fn charge(&mut self) -> Result<(), OpcError> {
        self.used = self.used.saturating_add(1);
        if self.used > self.limit {
            return Err(xml_error(format!("XML node limit: found {}, limit {}", self.used, self.limit)));
        }
        Ok(())
    }
}
fn push_text<B: TreeBuilder>(p: &mut B::Element, s: &str, nodes: &mut NodeBudget) -> Result<(), OpcError> {
    if s.is_empty() {
        return Ok(());
    }
    if let Some(t) = B::last_text(p) {
        t.push_str(s);
    } else {
        nodes.charge()?;
        B::text(p, s.to_string());
    }
    Ok(())
}

fn xml_error(e: impl std::fmt::Display) -> OpcError {
    OpcError::Xml(e.to_string())
}

fn check_chars(s: &str) -> Result<(), OpcError> {
    if s.chars().all(xml_char) { Ok(()) } else { Err(xml_error("invalid XML character")) }
}

fn start_el(b: &quick_xml::events::BytesStart, reader: &quick_xml::Reader<&[u8]>, nodes: &mut NodeBudget) -> Result<El, OpcError> {
    let name = std::str::from_utf8(b.name().as_ref()).map_err(xml_error)?.to_string();
    let mut attrs = vec![];
    let mut names = std::collections::HashSet::new();
    for a in b.attributes().with_checks(false) {
        let a = a.map_err(xml_error)?;
        nodes.charge()?;
        if !names.insert(a.key.0) {
            return Err(xml_error("duplicate XML attribute"));
        }
        let k = std::str::from_utf8(a.key.as_ref()).map_err(xml_error)?.to_string();
        // Normalize literal attribute whitespace before expanding character references.
        // https://www.w3.org/TR/xml/#AVNormalize
        let raw = reader.decoder().decode(&a.value).map_err(xml_error)?;
        let normalized = raw.replace("\r\n", "\n").replace(['\r', '\n', '\t'], " ");
        let v = quick_xml::escape::unescape(&normalized).map_err(xml_error)?.into_owned();
        check_chars(&v)?;
        attrs.push((k, v));
    }
    Ok(El { name, attrs, children: vec![] })
}

fn decode_utf16(bytes: &[u8]) -> Result<String, OpcError> {
    if !bytes.len().is_multiple_of(2) {
        return Err(xml_error("odd UTF-16 byte length"));
    }
    let le = bytes.starts_with(&[0xFF, 0xFE]);
    let units: Vec<u16> =
        bytes.get(2..).unwrap_or(&[]).as_chunks::<2>().0.iter().map(|c| if le { u16::from_le_bytes(*c) } else { u16::from_be_bytes(*c) }).collect();
    let s = String::from_utf16(&units).map_err(xml_error)?;
    Ok(match s.strip_prefix("<?xml").and_then(|r| r.find("?>").map(|i| i + 7)) {
        Some(end) => s.get(end..).unwrap_or("").to_string(),
        None => s,
    })
}

/// Is `c` allowed in XML 1.0 text?
fn xml_char(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r') || (c >= ' ' && c != '\u{FFFE}' && c != '\u{FFFF}')
}

/// Escape text content (and drop characters XML 1.0 forbids).
pub fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '\r' => o.push_str("&#13;"),
            c if xml_char(c) => o.push(c),
            _ => {}
        }
    }
    o
}

/// Escape an attribute value.
pub fn esc_attr(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '"' => o.push_str("&quot;"),
            '\n' => o.push_str("&#10;"),
            '\r' => o.push_str("&#13;"),
            '\t' => o.push_str("&#9;"),
            c if xml_char(c) => o.push(c),
            _ => {}
        }
    }
    o
}

/// A string builder for XML parts.
#[derive(Default)]
pub struct W {
    pub s: String,
}

/// Attribute list builder.
#[derive(Default)]
pub struct A {
    pub v: Vec<(&'static str, String)>,
}

impl A {
    pub fn new() -> Self {
        A { v: vec![] }
    }
    pub fn a(mut self, k: &'static str, v: impl ToString) -> Self {
        self.v.push((k, v.to_string()));
        self
    }
    pub fn o<T: ToString>(mut self, k: &'static str, v: Option<T>) -> Self {
        if let Some(v) = v {
            self.v.push((k, v.to_string()));
        }
        self
    }
    /// Boolean attribute written as `1`/`0` when `Some`.
    pub fn b(mut self, k: &'static str, v: Option<bool>) -> Self {
        if let Some(v) = v {
            self.v.push((k, if v { "1" } else { "0" }.to_string()));
        }
        self
    }
    /// Boolean attribute written only when true.
    pub fn t(mut self, k: &'static str, v: bool) -> Self {
        if v {
            self.v.push((k, "1".to_string()));
        }
        self
    }
}

impl W {
    pub fn new() -> Self {
        W { s: String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n") }
    }
    pub fn frag() -> Self {
        W { s: String::new() }
    }
    fn attrs(&mut self, a: &A) {
        for (k, v) in &a.v {
            let _ = write!(self.s, " {}=\"{}\"", k, esc_attr(v));
        }
    }
    pub fn open(&mut self, name: &str, a: A) {
        self.s.push('<');
        self.s.push_str(name);
        self.attrs(&a);
        self.s.push('>');
    }
    pub fn open0(&mut self, name: &str) {
        self.s.push('<');
        self.s.push_str(name);
        self.s.push('>');
    }
    pub fn close(&mut self, name: &str) {
        self.s.push_str("</");
        self.s.push_str(name);
        self.s.push('>');
    }
    pub fn empty(&mut self, name: &str, a: A) {
        self.s.push('<');
        self.s.push_str(name);
        self.attrs(&a);
        self.s.push_str("/>");
    }
    pub fn empty0(&mut self, name: &str) {
        self.s.push('<');
        self.s.push_str(name);
        self.s.push_str("/>");
    }
    /// `<name val="v"/>`
    pub fn val(&mut self, name: &str, v: impl ToString) {
        self.empty(name, A::new().a("val", v));
    }
    pub fn text(&mut self, t: &str) {
        self.s.push_str(&esc(t));
    }
    /// `<name>text</name>`
    pub fn elt(&mut self, name: &str, t: &str) {
        self.open0(name);
        self.text(t);
        self.close(name);
    }
    pub fn raw(&mut self, xml: &str) {
        self.s.push_str(xml);
    }
    pub fn finish(self) -> Vec<u8> {
        self.s.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_budget_counts_attributes_and_coalesced_text_before_retaining_them() {
        // Root, attribute, one coalesced text node and child: exactly four.
        let xml = b"<r a='1'>x&amp;y<c/></r>";
        assert!(parse_with_node_limit(xml, 4, &mut RawTree).is_ok());
        let error = parse_with_node_limit(xml, 3, &mut RawTree).unwrap_err().to_string();
        assert!(error.contains("XML node limit: found 4, limit 3"), "{error}");
        assert!(parse_with_node_limit(b"<r a='1' b='2'/>", 2, &mut RawTree).unwrap_err().to_string().contains("found 3, limit 2"));
    }
    #[test]
    fn parse_basic_and_entities() {
        let d = parse(b"<?xml version=\"1.0\"?><a:p xmlns:a=\"x\"><a:r><a:t>A &amp; B &#x41; &lt;</a:t></a:r><a:x r:id=\"rId1\" id=\"5\"/></a:p>")
            .unwrap();
        assert_eq!(d.root.local(), "p");
        assert_eq!(d.root.find("t").map(|t| t.text()), Some("A & B A <".to_string()));
        let x = d.root.child("x").unwrap();
        assert_eq!(x.attr("id"), Some("5"));
        assert_eq!(x.attr("r:id"), Some("rId1"));
        assert_eq!(d.ns_decls().len(), 1);
    }

    #[test]
    fn truncated_and_garbage() {
        assert!(parse(b"<a><b><c>text").is_err());
        assert!(parse(b"<a/><b/>").is_err());
        assert!(parse(b"<a></b>").is_err());
        assert!(parse(b"").is_err());
        let deep = "<a>".repeat(MAX_DEPTH + 10);
        assert!(parse(deep.as_bytes()).is_err());
    }

    #[test]
    fn depth_boundary_is_independent_of_empty_element_syntax() {
        for depth in [MAX_DEPTH - 1, MAX_DEPTH, MAX_DEPTH + 1] {
            for leaf in ["<leaf/>", "<leaf></leaf>"] {
                let xml = format!("{}{leaf}{}", "<a>".repeat(depth - 1), "</a>".repeat(depth - 1));
                assert_eq!(parse(xml.as_bytes()).is_ok(), depth <= MAX_DEPTH, "depth {depth}, leaf {leaf}");
            }
        }
    }

    #[test]
    fn hostile_xml_is_rejected() {
        for s in [
            "<a x=unquoted/>",
            "<a x='1' x='2'/>",
            "<a>&unknown;</a>",
            "garbage<a/>trailing",
            "<a>abc&#0;def</a>",
            "<a x='&unknown;'/>",
            "<!DOCTYPE a><a/>",
        ] {
            assert!(parse(s.as_bytes()).is_err(), "{s}");
        }
        assert!(parse(b"<a>\xff</a>").is_err());
        assert!(parse(&[255, 254, 0]).is_err());
    }

    #[test]
    fn escape_drops_control_chars() {
        assert_eq!(esc("a\u{b}b<\u{0}"), "ab&lt;");
        assert_eq!(esc_attr("\"x\"\n"), "&quot;x&quot;&#10;");
    }

    #[test]
    fn roundtrip_serialize() {
        let d = parse(b"<p:sp a=\"1\"><p:t>x &amp; y</p:t><e/></p:sp>").unwrap();
        assert_eq!(d.root.to_xml(), "<p:sp a=\"1\"><p:t>x &amp; y</p:t><e/></p:sp>");
    }
}

#[cfg(test)]
mod newline_regression {
    #[test]
    fn physical_newlines_normalize_but_character_references_remain_literal() {
        let tree = super::parse(b"<Text V='a\r\nb\tc&#13;d&#10;e'>a\r\nb\rc&#13;d<![CDATA[e\r\nf\rg]]></Text>").unwrap();
        assert_eq!(tree.root.attr("V"), Some("a b c\rd\ne"));
        assert_eq!(tree.root.text(), "a\nb\nc\rde\nf\ng");
    }
}

#[cfg(test)]
mod referenced_whitespace_regression {
    #[test]
    fn referenced_whitespace_survives_parse_serialize_parse() {
        let original = super::parse(b"<x V='a&#13;b&#10;c&#9;d'>a&#13;b&#10;c&#9;d</x>").unwrap();
        let saved = original.root.to_xml();
        assert!(saved.contains("&#13;"));
        let reread = super::parse(saved.as_bytes()).unwrap();
        assert_eq!(original.root, reread.root);
    }
}
