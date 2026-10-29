//! Standard SVG text oracle. Usage: svg_text_witness --self ORACLE.svg ...
//! or svg_text_witness ORACLE.svg DRAWING.vsdx [--preview]. No vendor extensions are read.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use kurbo::{Affine, Point};
use misterdiagram_render::{scene, svg};
use roxmltree::Node;
use std::{collections::BTreeMap, path::Path};

#[derive(Clone, Debug)]
struct Line {
    text: String,
    point: Point,
}
#[derive(Clone, Copy)]
struct Context {
    matrix: Affine,
    size: f64,
}
fn number(s: &str) -> Result<f64, String> {
    s.trim().parse::<f64>().ok().filter(|v| v.is_finite() && v.abs() <= 1e9).ok_or_else(|| format!("invalid SVG number {s:?}"))
}
fn length(s: &str, size: f64) -> Result<f64, String> {
    let s = s.trim();
    for (suffix, multiplier) in [("em", size), ("px", 1.), ("pt", 1.), ("in", 72.), ("cm", 72. / 2.54), ("mm", 72. / 25.4)] {
        if let Some(n) = s.strip_suffix(suffix) {
            return Ok(number(n)? * multiplier);
        }
    }
    number(s)
}
fn matrix(s: &str) -> Result<Affine, String> {
    let t = s.parse::<svgtypes::Transform>().map_err(|e| e.to_string())?;
    let values = [t.a, t.b, t.c, t.d, t.e, t.f];
    if values.iter().any(|v| !v.is_finite() || v.abs() > 1e9) {
        return Err("SVG transform limit".into());
    }
    Ok(Affine::new(values))
}
fn css(doc: &roxmltree::Document<'_>) -> BTreeMap<String, String> {
    let mut styles = BTreeMap::new();
    for node in doc.descendants().filter(|n| n.has_tag_name("style")) {
        for item in node.text().unwrap_or("").split('}') {
            if let Some((selector, declarations)) = item.split_once('{') {
                styles.insert(selector.trim().trim_start_matches('.').to_owned(), declarations.to_owned());
            }
        }
    }
    styles
}
fn font_size(declarations: &str) -> Option<&str> {
    declarations.split(';').filter_map(|s| s.split_once(':')).find(|(name, _)| name.trim() == "font-size").map(|(_, v)| v.trim())
}
fn context(node: Node<'_, '_>, parent: Context, styles: &BTreeMap<String, String>) -> Result<Context, String> {
    let mut size = None;
    for class in node.attribute("class").unwrap_or("").split_whitespace() {
        if let Some(value) = styles.get(class).and_then(|s| font_size(s)) {
            size = Some(value);
        }
    }
    if let Some(value) = node.attribute("font-size") {
        size = Some(value);
    }
    if let Some(value) = node.attribute("style").and_then(font_size) {
        size = Some(value);
    }
    let size = size.map(|s| length(s, parent.size)).transpose()?.unwrap_or(parent.size);
    if size <= 0. || size > 1e6 {
        return Err("SVG font size limit".into());
    }
    let transform = node.attribute("transform").map(matrix).transpose()?.unwrap_or(Affine::IDENTITY);
    let matrix = parent.matrix * transform;
    if matrix.as_coeffs().iter().any(|n| !n.is_finite()) {
        return Err("SVG composed transform limit".into());
    }
    Ok(Context { matrix, size })
}
fn text_children(
    node: Node<'_, '_>,
    ctx: Context,
    styles: &BTreeMap<String, String>,
    xy: &mut Point,
    lines: &mut Vec<Line>,
    depth: usize,
) -> Result<(), String> {
    if depth > 128 {
        return Err("SVG nesting limit".into());
    }
    for child in node.children() {
        if child.is_text() {
            if let Some(line) = lines.last_mut() {
                line.text.push_str(child.text().unwrap_or(""));
            }
        } else if child.tag_name().namespace() == Some("http://www.w3.org/2000/svg") && child.has_tag_name("tspan") {
            let next = context(child, ctx, styles)?;
            let newline = child.has_attribute("x") && (child.has_attribute("dy") || child.has_attribute("y"));
            if let Some(x) = child.attribute("x") {
                xy.x = length(x, next.size)?;
            }
            if let Some(y) = child.attribute("y") {
                xy.y = length(y, next.size)?;
            }
            if let Some(dy) = child.attribute("dy") {
                xy.y += length(dy, next.size)?;
            }
            // dx is intra-line positioning. It cannot create another logical line.
            if newline {
                lines.push(Line { text: String::new(), point: next.matrix * *xy });
            }
            text_children(child, next, styles, xy, lines, depth + 1)?;
        }
    }
    Ok(())
}
fn parse(xml: &str, to_page: Option<Affine>) -> Result<Vec<Vec<Line>>, String> {
    if xml.len() > 256 * 1024 * 1024 {
        return Err("SVG byte limit".into());
    }
    let doc =
        roxmltree::Document::parse_with_options(xml, roxmltree::ParsingOptions { allow_dtd: true, nodes_limit: 1_000_000, ..Default::default() })
            .map_err(|e| e.to_string())?;
    let root = doc.root_element();
    let map = if let Some(m) = to_page {
        m
    } else {
        let values = root.attribute("viewBox").ok_or("missing viewBox")?.split_whitespace().map(number).collect::<Result<Vec<_>, _>>()?;
        let [x, y, w, h] = values.as_slice() else { return Err("viewBox needs four numbers".into()) };
        if *w <= 0. || *h <= 0. {
            return Err("empty viewBox".into());
        }
        let width = length(root.attribute("width").ok_or("missing width")?, 12.)? / 72.;
        let height = length(root.attribute("height").ok_or("missing height")?, 12.)? / 72.;
        Affine::translate((0., height)) * Affine::scale_non_uniform(width / w, -height / h) * Affine::translate((-x, -y))
    };
    if map.as_coeffs().iter().any(|n| !n.is_finite()) {
        return Err("SVG page mapping limit".into());
    }
    let styles = css(&doc);
    let mut blocks = Vec::new();
    for node in doc.descendants().filter(|n| n.tag_name().namespace() == Some("http://www.w3.org/2000/svg") && n.has_tag_name("text")) {
        let ancestors: Vec<_> = node.ancestors().filter(|n| n.is_element()).collect();
        if ancestors.len() > 128 {
            return Err("SVG ancestry limit".into());
        }
        let mut ctx = Context { matrix: map, size: 12. };
        for ancestor in ancestors.into_iter().rev() {
            ctx = context(ancestor, ctx, &styles)?;
        }
        let mut xy = Point::new(length(node.attribute("x").unwrap_or("0"), ctx.size)?, length(node.attribute("y").unwrap_or("0"), ctx.size)?);
        let mut lines = vec![Line { text: String::new(), point: ctx.matrix * xy }];
        text_children(node, ctx, &styles, &mut xy, &mut lines, 0)?;
        if lines.iter().any(|l| !l.point.x.is_finite() || !l.point.y.is_finite()) {
            return Err("SVG line position limit".into());
        }
        blocks.push(lines);
    }
    Ok(blocks)
}
fn normalized(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
fn compare(name: &str, expected: &[Vec<Line>], actual: &[Vec<Line>]) -> bool {
    let mut counts = usize::from(expected.len() != actual.len());
    let (mut chars, mut positions, mut max_x, mut max_y) = (0, 0, 0f64, 0f64);
    for (bi, (a, b)) in expected.iter().zip(actual).enumerate() {
        counts += usize::from(a.len() != b.len());
        for (li, (a, b)) in a.iter().zip(b).enumerate() {
            let dx = (a.point.x - b.point.x).abs() * 72.;
            let dy = (a.point.y - b.point.y).abs() * 72.;
            max_x = max_x.max(dx);
            max_y = max_y.max(dy);
            chars += usize::from(normalized(&a.text) != normalized(&b.text));
            positions += usize::from(dx > 0.5 || dy > 0.5);
            if normalized(&a.text) != normalized(&b.text) || dx > 0.5 || dy > 0.5 {
                eprintln!(
                    "{name} block {} line {}: expected {:?} actual {:?}, delta=({dx:.6},{dy:.6})pt",
                    bi + 1,
                    li + 1,
                    normalized(&a.text),
                    normalized(&b.text)
                );
            }
        }
    }
    println!(
        "{name}\t{}\t{}\t{}\t{}\t{counts}\t{chars}\t{positions}\t{max_x:.6}\t{max_y:.6}",
        expected.len(),
        actual.len(),
        expected.iter().map(Vec::len).sum::<usize>(),
        actual.iter().map(Vec::len).sum::<usize>()
    );
    counts + chars + positions == 0
}
fn read(path: &Path) -> Result<String, String> {
    if std::fs::metadata(path).map_err(|e| e.to_string())?.len() > 256 * 1024 * 1024 {
        return Err("SVG byte limit".into());
    }
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}
fn run() -> Result<bool, String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    println!("file\toracle_blocks\tactual_blocks\toracle_lines\tactual_lines\tcount_misses\tcharacter_misses\tposition_misses\tmax_x_pt\tmax_y_pt");
    if args.first().is_some_and(|a| a == "--self") && args.len() > 1 {
        let mut ok = true;
        for path in args.iter().skip(1) {
            let xml = read(Path::new(path))?;
            let a = parse(&xml, None)?;
            let b = parse(&xml, None)?;
            ok &= compare(path, &a, &b);
        }
        return Ok(ok);
    }
    if !(2..=3).contains(&args.len()) {
        return Err("usage: svg_text_witness --self ORACLE.svg ... | ORACLE.svg DRAWING.vsdx [--preview]".into());
    }
    let oracle = args.first().ok_or("missing oracle")?;
    let input = args.get(1).ok_or("missing drawing")?;
    if args.get(2).is_some_and(|a| a != "--preview") {
        return Err("unknown witness option".into());
    }
    let preview = args.get(2).is_some_and(|a| a == "--preview");
    let doc = misterdiagram_vsdx::read(Path::new(input)).map_err(|e| e.to_string())?;
    let opts = scene::Options { visio_export: !preview, cached_fallback: true, ..Default::default() };
    let scene = scene::page(&doc, scene::default_page_index(&doc)?, opts)?;
    let (_, _, view) = scene.canvas(opts)?;
    let actual = parse(&svg::render(&scene, opts)?, Some(view.inverse()))?;
    let expected = parse(&read(Path::new(oracle))?, None)?;
    Ok(compare(oracle, &expected, &actual))
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(true) => std::process::ExitCode::SUCCESS,
        Ok(false) => std::process::ExitCode::FAILURE,
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::FAILURE
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transforms_em_lines_and_style_spans() {
        let xml = r#"<svg xmlns="http://www.w3.org/2000/svg" width="2in" height="2in" viewBox="0 0 144 144"><style>.a{font-size:2em}</style><g transform="translate(72 0)"><text x="0" y="0" class="a" transform="rotate(90)">A<tspan dx="2em">B</tspan><tspan x="24" dy="1em">C<tspan font-size="0.5em">D</tspan></tspan></text></g></svg>"#;
        let blocks = parse(xml, None).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].len(), 2);
        assert_eq!(blocks[0][0].text, "AB");
        assert_eq!(blocks[0][1].text, "CD");
        assert!((blocks[0][0].point - Point::new(1., 2.)).hypot() < 1e-10);
        assert!((blocks[0][1].point - Point::new(2. / 3., 5. / 3.)).hypot() < 1e-10);
    }
    #[test]
    fn malformed_and_hostile_are_errors() {
        let nested = format!(
            "<svg xmlns='http://www.w3.org/2000/svg' width='1in' height='1in' viewBox='0 0 72 72'>{}<text x='1' y='1'>x</text>{}</svg>",
            "<g transform='scale(1000000000)'>".repeat(40),
            "</g>".repeat(40)
        );
        assert!(parse(&nested, None).is_err());
        for xml in [
            "<",
            "<svg><text></svg>",
            "<svg width='NaN' height='1' viewBox='0 0 0 0'/>",
            "<!DOCTYPE svg [<!ENTITY a SYSTEM 'file:///etc/passwd'>]><svg>&a;</svg>",
        ] {
            assert!(parse(xml, None).is_err());
        }
    }
}
