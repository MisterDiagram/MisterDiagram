//! Render diagnostics retain ShapeSheet identities without changing error values.
use super::{Error, Evaluator, Key, Scope};
impl Evaluator<'_> {
    /// Enable bounded dependency context only for drawing diagnostics.
    pub fn enable_render_diagnostics(&mut self) {
        self.render_diagnostics = true;
    }
    /// Stable document identity for warnings and render failures.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
    pub fn scope_label(&self, scope: Scope) -> String {
        let label = match scope {
            Scope::Document => "document sheet".into(),
            Scope::Page(page) => format!("page {page} pagesheet"),
            Scope::Master(master) => format!("master {master} pagesheet"),
            Scope::Style(style) => format!("style sheet {style}"),
            Scope::PageShape { page, shape } => format!("page {page} shape {shape}"),
            Scope::MasterShape { master, shape } => format!("master {master} shape {shape}"),
        };
        if let Some(shape) = self.shape(scope) {
            let name = if shape.name_u.is_empty() { &shape.name } else { &shape.name_u };
            let name: String = name.chars().take(256).map(|c| if c.is_control() { ' ' } else { c }).collect();
            if !name.is_empty() {
                return format!("{label} ({name})");
            }
        }
        label
    }
    pub(super) fn note_result(&mut self, scope: Scope, name: &str, value: &super::Value, before: u64) {
        if !self.render_diagnostics {
            return;
        }
        let key = Key { scope, name: name.chars().take(65536).collect() };
        if let Some(error) = value.error()
            && (self.diagnostic_serial == before || self.failure_cell.as_ref().is_none_or(|(_, prior)| *prior != error))
        {
            self.failure_cell = Some((key.clone(), error));
            self.diagnostic_serial = self.diagnostic_serial.saturating_add(1);
        }
        self.last_cell = Some(key);
    }
    /// Attach the consumed cell, its provider and the failing dependency, if any.
    /// Formula errors remain typed for sheet inspection and numeric oracle comparisons.
    pub fn render_error(&self, scope: Scope, operation: &str, error: impl std::fmt::Display) -> String {
        let key = self.failure_cell.as_ref().map(|(key, _)| key).or(self.last_cell.as_ref());
        let Some(key) = key else { return format!("{} {operation}: {error}", self.scope_label(scope)) };
        let name = self
            .resolved_named_cells(key.scope)
            .into_iter()
            .find(|n| n.name.eq_ignore_ascii_case(&key.name))
            .map_or_else(|| key.name.clone(), |n| n.name);
        let provider = self
            .formula_origin(key.scope, &key.name)
            .filter(|p| *p != key.scope)
            .map(|p| format!(" (from {})", self.scope_label(p)))
            .unwrap_or_default();
        let label = self.scope_label(key.scope);
        if scope == key.scope {
            format!("{label} {operation} {name}{provider}: {error}")
        } else {
            format!("{} {operation}: {label} {name}{provider}: {error}", self.scope_label(scope))
        }
    }
    pub(super) fn begin_diagnostic(&mut self) {
        if self.render_diagnostics && self.stack.is_empty() {
            self.failure_cell = None;
            self.last_cell = None;
            self.diagnostic_serial = 0;
        }
    }
    /// A required drawing dimension is not silently replaced by a default on error.
    pub fn render_number(&mut self, scope: Scope, name: &str) -> Result<f64, String> {
        self.evaluate(scope, name)
            .numeric()
            .map_err(|error| self.render_error(scope, "", error))
            .and_then(|n| if n.is_finite() && n.abs() <= 1e6 { Ok(n) } else { Err(self.render_error(scope, "", Error::Limit)) })
    }
}
