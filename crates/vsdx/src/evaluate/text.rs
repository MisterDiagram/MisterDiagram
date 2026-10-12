//! Lossless text markers become fully resolved layout input without an L2 dependency.
use super::{Error, Evaluator, Scope};
use diagramcraft_color::Rgba;
use diagramcraft_geom::Affine;
use diagramcraft_model::{XmlElement, XmlNode, text::*};
use diagramcraft_sheet::Value;

pub struct TextBlock {
    pub body: TextBody,
    /// Point dimensions of the unrotated text rectangle.
    pub width: f64,
    pub height: f64,
    /// Top-left, y-down text points to y-up page inches.
    pub to_page: Affine,
}
fn indexed(prefix: &str, cell: &str, row: u32) -> String {
    if row == 0 { format!("{prefix}.{cell}") } else { format!("{prefix}.{cell}[{}]", row.saturating_add(1)) }
}
impl<'a> Evaluator<'a> {
    fn text_source(&self, mut scope: Scope) -> Result<Option<&'a XmlElement>, Error> {
        for _ in 0..128 {
            let Some(shape) = self.shape(scope) else {
                return Ok(None);
            };
            if let Some(text) = &shape.text {
                return Ok(Some(text));
            }
            let Some(master) = self.inheritance.masters.get(&scope) else {
                return Ok(None);
            };
            scope = *master;
        }
        Err(Error::Limit)
    }
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/text-element-shapesheet_type-complextypevisio-xml
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/fld-element-text_type-complextypevisio-xml
    pub(super) fn text_string(&mut self, scope: Scope, flags: u32) -> Result<String, Error> {
        let Some(source) = self.text_source(scope)? else {
            return Ok(String::new());
        };
        let mut out = String::new();
        let mut stack = vec![(source.children.iter(), 0usize)];
        let mut nodes = 0usize;
        while let Some((it, depth)) = stack.last_mut() {
            let depth = *depth;
            match it.next() {
                Some(XmlNode::Text(s)) => {
                    if out.len().saturating_add(s.len()) > 65536 {
                        return Err(Error::Limit);
                    }
                    out.push_str(s);
                }
                Some(XmlNode::Element(e)) => {
                    nodes += 1;
                    if nodes > 65536 || depth >= 128 {
                        return Err(Error::Limit);
                    }
                    if e.name == "fld" && flags & 2 != 0 {
                        out.push('\u{fffc}');
                    } else {
                        stack.push((e.children.iter(), depth + 1));
                    }
                }
                None => {
                    stack.pop();
                }
            }
        }
        if out.ends_with('\n') {
            out.pop();
        }
        Ok(out)
    }
    fn text_number(&mut self, scope: Scope, name: &str, default: f64) -> f64 {
        self.evaluate(scope, name).numeric().ok().filter(|v| v.is_finite() && v.abs() <= 1e6).unwrap_or(default)
    }
    fn text_font(&mut self, scope: Scope, name: &str) -> String {
        let id = self.text_number(scope, name, 0.);
        if (0. ..=65535.).contains(&id) { self.fonts.get(&(id as u32)).cloned().unwrap_or_else(|| "Calibri".into()) } else { "Calibri".into() }
    }
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/size-cell-character-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/font-cell-character-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/style-cell-character-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/pos-cell-character-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/color-cell-character-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/case-cell-character-section
    fn run_props(&mut self, scope: Scope, row: u32) -> RunProps {
        let font = self.text_font(scope, &indexed("Char", "Font", row));
        let color = match self.evaluate(scope, &indexed("Char", "Color", row)) {
            Value::Color { rgb: [r, g, b], .. } => Rgba::rgb(r, g, b),
            Value::Number(n) if n.is_finite() && (0. ..=65535.).contains(&n) => {
                let [r, g, b] =
                    self.palette.get(&(n as u32)).copied().or_else(|| diagramcraft_sheet::functions::default_color(n as u32)).unwrap_or([0, 0, 0]);
                Rgba::rgb(r, g, b)
            }
            _ => Rgba::BLACK,
        };
        let mut n = |name: &str, default| self.text_number(scope, &indexed("Char", name, row), default);
        let style = n("Style", 0.).clamp(0., 65535.) as u32;
        let pos = n("Pos", 0.);
        RunProps {
            font: Some(font),
            size: Some(n("Size", 12. / 72.) * 72.),
            bold: Some(style & 1 != 0),
            italic: Some(style & 2 != 0),
            underline: (style & 4 != 0).then(|| "sng".into()),
            caps: Some(if style & 8 != 0 {
                Caps::Small
            } else if n("Case", 0.) == 1. {
                Caps::All
            } else {
                Caps::None
            }),
            strike: Some(if n("Strikethru", 0.) != 0. { Strike::Single } else { Strike::None }),
            baseline: Some(if pos == 1. {
                0.3
            } else if pos == 2. {
                -0.3
            } else {
                0.
            }),
            fill: Some(color),
            ..Default::default()
        }
    }
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/horzalign-cell-paragraph-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/spline-cell-paragraph-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/indfirst-cell-paragraph-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/indleft-cell-paragraph-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/indright-cell-paragraph-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/spbefore-cell-paragraph-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/spafter-cell-paragraph-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/bullet-cell-paragraph-section
    fn para_props(&mut self, scope: Scope, row: u32, tabs: u32) -> ParaProps {
        let bullet_string = self.evaluate(scope, &indexed("Para", "BulletStr", row)).text();
        let bullet_font = self.text_font(scope, &indexed("Para", "BulletFont", row));
        let default_tab = self.text_number(scope, "DefaultTabStop", 0.5) * 72.;
        let tab_stops = self.tab_stops(scope, tabs);
        let mut n = |name: &str, default| self.text_number(scope, &indexed("Para", name, row), default);
        let line = n("SpLine", -1.2);
        let bullet = n("Bullet", 0.);
        let bullet = match bullet as i32 {
            0 => Bullet::None,
            1 => Bullet::Char { char: "•".into() },
            2 => Bullet::Char { char: "◆".into() },
            3 => Bullet::Char { char: "■".into() },
            4 => Bullet::Char { char: "□".into() },
            5 => Bullet::Char { char: "❖".into() },
            6 => Bullet::Char { char: "➢".into() },
            7 => Bullet::Char { char: "✓".into() },
            _ => Bullet::Char { char: bullet_string },
        };
        ParaProps {
            align: Some(match n("HorzAlign", 0.) as i32 {
                1 => Align::Center,
                2 => Align::Right,
                3 => Align::Justify,
                4 => Align::Distributed,
                _ => Align::Left,
            }),
            margin_left: Some(n("IndLeft", 0.) * 72.),
            margin_right: Some(n("IndRight", 0.) * 72.),
            indent: Some(n("IndFirst", 0.) * 72.),
            line_spacing: Some(if line > 0. { Spacing::Pts(line * 72.) } else { Spacing::Pct(if line == 0. { 1. } else { -line }) }),
            space_before: Some(Spacing::Pts(n("SpBefore", 0.) * 72.)),
            space_after: Some(Spacing::Pts(n("SpAfter", 0.) * 72.)),
            rtl: Some(n("TextDirection", 0.) != 0.),
            bullet: Some(bullet),
            bullet_font: Some(bullet_font),
            default_tab: Some(default_tab),
            tabs: Some(tab_stops),
            ..Default::default()
        }
    }
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/position-cell-tabs-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/alignment-cell-tabs-section
    fn tab_stops(&mut self, scope: Scope, row: u32) -> Vec<TabStop> {
        let Some(shape) = self.shape(scope) else {
            return vec![];
        };
        let Some(r) = shape.sheet.sections.iter().find(|s| s.name == "Tabs").and_then(|s| s.rows.iter().find(|r| r.index.unwrap_or(0) == row)) else {
            return vec![];
        };
        let mut out = Vec::new();
        for c in r.cells.iter().filter(|c| c.name.starts_with("Position")).take(4096) {
            if let Some(pos) = c.value.as_deref().and_then(|v| v.parse::<f64>().ok()).filter(|v| v.is_finite() && v.abs() < 1e6) {
                let suffix = c.name.trim_start_matches("Position");
                let align = r.cells.iter().find(|c| c.name == format!("Alignment{suffix}")).and_then(|c| c.value.as_deref()).unwrap_or("0");
                out.push(TabStop {
                    pos: pos * 72.,
                    align: match align {
                        "1" => "ctr",
                        "2" => "r",
                        "3" => "dec",
                        _ => "l",
                    }
                    .into(),
                });
            }
        }
        out
    }
    /// Parse cp/pp/tp run markers and field display text, preserving the formatting of every run.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cp-element-text_type-complextypevisio-xml
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/pp-element-text_type-complextypevisio-xml
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/tp-element-text_type-complextypevisio-xml
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/fld-element-text_type-complextypevisio-xml
    pub fn text_body(&mut self, scope: Scope) -> Result<TextBody, Error> {
        let source = self.text_source(scope)?;
        let mut body = TextBody {
            body: BodyProps {
                inset_l: Some(self.text_number(scope, "LeftMargin", 0.1) * 72.),
                inset_r: Some(self.text_number(scope, "RightMargin", 0.1) * 72.),
                inset_t: Some(self.text_number(scope, "TopMargin", 0.05) * 72.),
                inset_b: Some(self.text_number(scope, "BottomMargin", 0.05) * 72.),
                anchor: Some(match self.text_number(scope, "VerticalAlign", 1.) as i32 {
                    0 => Anchor::Top,
                    2 => Anchor::Bottom,
                    _ => Anchor::Middle,
                }),
                wrap: Some(true),
                ..Default::default()
            },
            ..Default::default()
        };
        let Some(source) = source else {
            return Ok(body);
        };
        let mut cr = 0;
        let mut pr = 0;
        let mut tr = 0;
        let mut para = Paragraph { props: self.para_props(scope, pr, tr), end_props: self.run_props(scope, cr), ..Default::default() };
        let mut bytes = 0usize;
        if source.children.len() > 65536 {
            return Err(Error::Limit);
        }
        for node in &source.children {
            let (text, kind) = match node {
                XmlNode::Text(s) => {
                    if s.len() > 65536 {
                        return Err(Error::Limit);
                    }
                    (s.clone(), RunKind::Text)
                }
                XmlNode::Element(e) => {
                    let index = e.attr("IX").unwrap_or("0").parse::<u32>().map_err(|_| Error::Value)?;
                    if index > 65535 {
                        return Err(Error::Limit);
                    }
                    match e.name.as_str() {
                        "cp" => {
                            cr = index;
                            continue;
                        }
                        "pp" => {
                            pr = index;
                            para.props = self.para_props(scope, pr, tr);
                            continue;
                        }
                        "tp" => {
                            tr = index;
                            para.props = self.para_props(scope, pr, tr);
                            continue;
                        }
                        "fld" => (super::limited_text(e)?, RunKind::Field { field: index.to_string() }),
                        _ => return Err(Error::Unsupported(format!("text marker {}", e.name))),
                    }
                }
            };
            bytes = bytes.checked_add(text.len()).ok_or(Error::Limit)?;
            if bytes > 65536 {
                return Err(Error::Limit);
            }
            let mut chunks = text.split('\n').peekable();
            while let Some(chunk) = chunks.next() {
                if !chunk.is_empty() {
                    para.runs.push(Run { text: chunk.trim_end_matches('\r').into(), props: self.run_props(scope, cr), kind: kind.clone() });
                }
                para.end_props = self.run_props(scope, cr);
                if chunks.peek().is_some() {
                    body.paragraphs.push(para);
                    if body.paragraphs.len() > 4096 {
                        return Err(Error::Limit);
                    }
                    para = Paragraph { props: self.para_props(scope, pr, tr), end_props: self.run_props(scope, cr), ..Default::default() };
                }
            }
        }
        if !para.runs.is_empty() || body.paragraphs.is_empty() {
            body.paragraphs.push(para);
        }
        Ok(body)
    }
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/txtpinx-cell-text-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/txtpiny-cell-text-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/txtwidth-cell-text-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/txtheight-cell-text-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/txtangle-cell-text-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/txtlocpinx-cell-text-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/txtlocpiny-cell-text-transform-section
    pub fn text_block(&mut self, scope: Scope) -> Result<TextBlock, Error> {
        let body = self.text_body(scope)?;
        let shape_width = self.evaluate(scope, "Width").numeric()?;
        let shape_height = self.evaluate(scope, "Height").numeric()?;
        let mut value = |name: &str, fallback: f64| -> Result<f64, Error> {
            if self.cell(scope, name).is_none() { Ok(fallback) } else { self.evaluate(scope, name).numeric() }
        };
        let width = value("TxtWidth", shape_width)?;
        let height = value("TxtHeight", shape_height)?;
        let x = value("TxtPinX", shape_width / 2.)?;
        let y = value("TxtPinY", shape_height / 2.)?;
        let lx = value("TxtLocPinX", width / 2.)?;
        let ly = value("TxtLocPinY", height / 2.)?;
        let angle = value("TxtAngle", 0.)?;
        if [width, height, x, y, lx, ly, angle].into_iter().any(|n| !n.is_finite() || n.abs() > 1e6) {
            return Err(Error::Limit);
        }
        let to_page = self.shape_to_page(scope)?
            * Affine::translate((x, y))
            * Affine::rotate(angle)
            * Affine::translate((-lx, height - ly))
            * Affine::scale_non_uniform(1. / 72., -1. / 72.);
        Ok(TextBlock { body, width: width * 72., height: height * 72., to_page })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diagramcraft_model::{Document, Page, Shape};
    fn document(xml: &str) -> Document {
        Document {
            pages: vec![Page {
                id: 1,
                shapes: vec![Shape { id: 1, text: Some(crate::parse_xml(xml.as_bytes()).unwrap()), ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        }
    }
    #[test]
    fn markers_fields_and_hostile_text() {
        let doc = document("<Text><cp IX='0'/>Alpha<fld IX='0'>42</fld>\n<pp IX='0'/>Beta\n</Text>");
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 1, shape: 1 };
        let body = e.text_body(scope).unwrap();
        assert_eq!(body.paragraphs.len(), 2);
        assert_eq!(body.paragraphs[0].text(), "Alpha42");
        assert_eq!(body.paragraphs[1].text(), "Beta");
        assert_eq!(e.text_string(scope, 2).unwrap(), "Alpha\u{fffc}\nBeta");
        for xml in
            ["<Text><cp IX='4294967295'/>x</Text>".into(), "<Text><cp IX='invalid'/></Text>".into(), format!("<Text>{}</Text>", "x".repeat(65537))]
        {
            let doc = document(&xml);
            assert!(Evaluator::new(&doc).unwrap().text_body(scope).is_err());
        }
    }
    #[test]
    fn generated_text_group_resolves_runs_and_transforms() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
        if !root.exists() {
            eprintln!("local corpus absent: text corpus test skipped");
            return;
        }
        let mut files = 0;
        for entry in std::fs::read_dir(root.join("generated/text")).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|s| s != "vsdx") {
                continue;
            }
            let doc = crate::read(&path).unwrap();
            let mut e = Evaluator::new(&doc).unwrap();
            for page in &doc.pages {
                for shape in &page.shapes {
                    let block = e.text_block(Scope::PageShape { page: page.id, shape: shape.id }).unwrap();
                    assert!(block.width > 0. && block.height > 0., "{}", path.display());
                    assert!(block.to_page.as_coeffs().iter().all(|v| v.is_finite()));
                    assert!(!block.body.paragraphs.is_empty());
                }
            }
            files += 1;
        }
        assert!(files >= 10);
    }
}
