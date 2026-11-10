//! Resolved formatting in the public format commands' parameter vocabulary.
use super::round4;
use misterdiagram_model::text::Align;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};

fn number(e: &mut Evaluator<'_>, scope: Scope, name: &str, default: f64) -> f64 {
    e.evaluate(scope, name).numeric().ok().filter(|n| n.is_finite() && n.abs() <= 1e6).unwrap_or(default)
}
fn color(e: &mut Evaluator<'_>, scope: Scope, name: &str) -> String {
    let [r, g, b] = e.color_rgb(scope, name).unwrap_or([0, 0, 0]);
    format!("#{r:02x}{g:02x}{b:02x}")
}
/// Read the same inherited colour, weight and text defaults as rendering.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fill-format-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/line-format-section
pub(super) fn style(e: &mut Evaluator<'_>, scope: Scope) -> Value {
    let weight = match e.evaluate(scope, "LineWeight").numeric() {
        Ok(n) if n.is_finite() && n.abs() <= 1e6 => n,
        Err(_) if e.cell(scope, "LineWeight").and_then(|c| c.value.as_deref()).and_then(|v| v.parse::<f64>().ok()) == Some(0.) => 0.,
        _ => 0.01,
    };
    let (text, paragraph) = e.first_text_style(scope);
    let text_color = text.fill.map_or_else(|| "#000000".into(), |c| format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b));
    let align = match paragraph.align {
        Some(Align::Center) => 1,
        Some(Align::Right) => 2,
        Some(Align::Justify | Align::JustLow) => 3,
        Some(Align::Distributed) => 4,
        _ => 0,
    };
    json!({
        "fill": {"color":color(e,scope,"FillForegnd"),"pattern":number(e,scope,"FillPattern",1.).max(0.) as u32,
            "transparency":number(e,scope,"FillForegndTrans",0.).clamp(0.,1.)},
        "line": {"color":color(e,scope,"LineColor"),"pattern":number(e,scope,"LinePattern",1.).max(0.) as u32,
            "weight":round4(weight.max(0.)*72.),"units":"PT", "begin_arrow":number(e,scope,"BeginArrow",0.).max(0.) as u32,
            "end_arrow":number(e,scope,"EndArrow",0.).max(0.) as u32,"transparency":number(e,scope,"LineColorTrans",0.).clamp(0.,1.)},
        "text": {"font":text.font,"size":round4(text.size.unwrap_or(12.)),"color":text_color,
            "bold":text.bold.unwrap_or(false),"italic":text.italic.unwrap_or(false),"underline":text.underline.is_some(),"align":align}
    })
}
