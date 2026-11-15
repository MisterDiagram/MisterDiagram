//! Recovery warnings on file command replies, without changing edit replies.
use crate::{Result, operation};
use misterdiagram_model::Document;
use misterdiagram_vsdx::evaluate::Evaluator;
use std::collections::BTreeSet;

/// Saving preserves all source cells, including error cells not needed for drawing.
pub(crate) fn saved_warnings(document: &Document) -> Result<(Vec<String>, usize)> {
    let themes = misterdiagram_theme::Themes::from_document(document).map_err(operation)?;
    let mut e =
        Evaluator::new(document).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host).with_cached_fallback();
    let mut seen = BTreeSet::new();
    let mut warnings = Vec::new();
    let mut count = 0usize;
    let mut name_bytes = 0usize;
    for scope in e.scopes().collect::<Vec<_>>() {
        for named in e.named_cells(scope).to_vec() {
            count = count.saturating_add(1);
            if count > 2_000_000 {
                return Err(operation("saved error traversal limit"));
            }
            e.evaluate_snapshot(scope, &named.name);
            for message in e.take_fallback_warnings().0 {
                if let Some(key) = saved_key(&message)
                    && seen.insert(key)
                {
                    name_bytes = name_bytes.saturating_add(message.len());
                    if name_bytes > 64 * 1024 * 1024 {
                        return Err(operation("saved error warning limit"));
                    }
                    if warnings.len() < 100 {
                        warnings.push(message);
                    }
                }
            }
        }
    }
    Ok((warnings, seen.len()))
}
fn saved_key(warning: &str) -> Option<String> {
    warning.strip_suffix("; used the value saved in the file")?.rsplit_once(": ").map(|(cell, _)| cell.to_ascii_lowercase())
}
/// Preserve unrelated warnings and the first spelling of each recovered cell.
pub(crate) fn unique(warnings: &mut Vec<String>) {
    let mut seen = BTreeSet::new();
    warnings.retain(|warning| saved_key(warning).is_none_or(|key| seen.insert(key)));
}
