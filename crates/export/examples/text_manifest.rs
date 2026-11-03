//! Test-build evaluator manifest for independent PPTX text readback.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use misterdiagram_model::text::RunKind;
use misterdiagram_render::scene::{EvaluatedShape, Options, SceneBudget, evaluated};
use std::io::{BufWriter, Write};
fn hex(s: &str) -> String {
    s.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
}
fn node(w: &mut impl Write, n: &EvaluatedShape, page: usize, block: &mut usize, depth: usize) -> Result<(), Box<dyn std::error::Error>> {
    if depth > 128 {
        return Err("text manifest depth limit".into());
    }
    if n.children_first {
        for c in &n.children {
            node(w, c, page, block, depth + 1)?;
        }
    }
    if !n.hidden_own
        && let Some(body) = &n.text_body
    {
        *block = block.checked_add(1).ok_or("text manifest block limit")?;
        writeln!(w, "B\t{page}\t{block}\t{}", body.paragraphs.len())?;
        for (pi, p) in body.paragraphs.iter().enumerate() {
            writeln!(w, "P\t{page}\t{block}\t{pi}\t{}", p.runs.len())?;
            for (ri, r) in p.runs.iter().enumerate() {
                let kind = if matches!(r.kind, RunKind::Break) { "br" } else { "r" };
                writeln!(
                    w,
                    "R\t{page}\t{block}\t{pi}\t{ri}\t{kind}\t{}\t{}\t{}\t{}",
                    hex(&r.text),
                    u8::from(r.props.bold.unwrap_or(false)),
                    u8::from(r.props.italic.unwrap_or(false)),
                    hex(r.props.font.as_deref().unwrap_or("Calibri"))
                )?;
            }
        }
    }
    if !n.children_first {
        for c in &n.children {
            node(w, c, page, block, depth + 1)?;
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let [input, output] = args.as_slice() else {
        return Err("usage: text_manifest IN.vsdx OUT.tsv".into());
    };
    let doc = misterdiagram_vsdx::read(input)?;
    let options = Options { visio_export: true, cached_fallback: true, ..Default::default() };
    let pages = evaluated::foreground_pages(&doc, options, &mut SceneBudget::default())?;
    let mut writer = BufWriter::new(std::fs::File::create(output)?);
    writeln!(writer, "S\t{}", pages.len())?;
    for (index, page) in pages.iter().enumerate() {
        let mut block = 0;
        for n in &page.shapes {
            node(&mut writer, n, index + 1, &mut block, 0)?;
        }
    }
    writer.flush()?;
    Ok(())
}
