//! Strict publication followed, only on failure, by predecessor-scoped recovery.
use crate::{Result, operation};
use misterdiagram_model::Document;
use misterdiagram_route::GlueHost;
use misterdiagram_vsdx::{
    evaluate::{Evaluator, StoredErrors},
    write::WrittenParts,
};
use std::sync::Arc;

pub(crate) fn stored_errors(before: &Document) -> Result<StoredErrors> {
    #[cfg(test)]
    COUNTS.with(|counts| counts.borrow_mut().1 += 1);
    let themes = misterdiagram_theme::Themes::from_document(before).map_err(operation)?;
    Evaluator::new(before).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host).stored_errors().map_err(operation)
}
pub(crate) fn prepare(
    before: &Document,
    document: &mut Document,
    validated: bool,
    stored: Option<Arc<StoredErrors>>,
) -> Result<(WrittenParts, GlueHost)> {
    #[cfg(test)]
    COUNTS.with(|counts| counts.borrow_mut().0 += 1);
    let mut glue = match &stored {
        None => GlueHost::prepare_scoped(before, document, validated),
        Some(errors) => GlueHost::prepare_with_stored_errors(before, document, validated, errors.clone()),
    }
    .map_err(operation)?;
    misterdiagram_route::reroute(before, document, &mut glue, None).map_err(operation)?;
    let themes = misterdiagram_theme::Themes::from_document(document).map_err(operation)?;
    let written = match stored {
        None => misterdiagram_vsdx::write::write_parts_with_glue_host(document, Some(&themes), Some(&misterdiagram_text::Host), Some(&mut glue)),
        Some(errors) => misterdiagram_vsdx::write::write_parts_with_stored_errors(
            document,
            Some(&themes),
            Some(&misterdiagram_text::Host),
            Some(&mut glue),
            errors,
        ),
    }
    .map_err(operation)?;
    Ok((written, glue))
}
#[cfg(test)]
thread_local! {
    pub(crate) static COUNTS: std::cell::RefCell<(usize, usize)> = const { std::cell::RefCell::new((0, 0)) };
}
