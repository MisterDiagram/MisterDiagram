//! Prove a clamped circular expression constant without using its cached value.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/min-function
//! https://learn.microsoft.com/en-us/office/client-developer/visio/max-function
use super::{BoundContext, Error, Evaluator, Key, Scope, Value};
use diagramcraft_sheet::parser::{BinaryOp, Expr, UnaryOp};
use std::collections::BTreeSet;

#[derive(Clone, Copy)]
struct Range(f64, f64);
impl Range {
    const INVALID: Self = Self(f64::NAN, f64::NAN);
    const UNKNOWN: Self = Self(f64::NEG_INFINITY, f64::INFINITY);
    fn point(n: f64) -> Self {
        Self(n, n)
    }
    fn exact(self) -> Option<f64> {
        (self.0.is_finite() && self.0 == self.1).then_some(self.0)
    }
    fn valid(self) -> Self {
        if self.0.is_nan() || self.1.is_nan() || self.0 > self.1 { Self::INVALID } else { self }
    }
}
impl Evaluator<'_> {
    pub(super) fn prove_clamped_cycle(&mut self, key: &Key) -> Value {
        if self.resolving_cycle || !self.arguments.is_empty() {
            return Value::Error(Error::Cycle);
        }
        self.resolving_cycle = true;
        // Speculative scalar leaves must not leave cycle errors in the normal value cache.
        let values = std::mem::take(&mut self.values);
        let result = self.range_cell(key, &mut BTreeSet::new(), 0).exact();
        self.values = values;
        self.resolving_cycle = false;
        result.map_or(Value::Error(Error::Cycle), Value::number)
    }
    fn range_cell(&mut self, key: &Key, visited: &mut BTreeSet<Key>, depth: usize) -> Range {
        if !visited.insert(key.clone()) {
            return Range::UNKNOWN;
        }
        if depth > 48 || self.work >= 100_000 {
            return Range::INVALID;
        }
        self.work += 1;
        self.add_dependency_key(key.clone());
        let result = if let Some(formula) = self.effective_formula(key.scope, &key.name).filter(|f| f != "No Formula") {
            let origin = self.formula_origin(key.scope, &key.name).unwrap_or(key.scope);
            match diagramcraft_sheet::parse(&formula) {
                Ok(expr) => self.range_expr(key.scope, origin, &expr, visited, depth + 1),
                Err(_) => Range::INVALID,
            }
        } else {
            self.cell(key.scope, &key.name).and_then(|c| self.cached_cell(c).numeric().ok()).map_or(Range::INVALID, Range::point)
        };
        visited.remove(key);
        result
    }
    fn range_expr(&mut self, scope: Scope, origin: Scope, expr: &Expr, visited: &mut BTreeSet<Key>, depth: usize) -> Range {
        if depth > 48 || self.work >= 100_000 {
            return Range::INVALID;
        }
        self.work += 1;
        match expr {
            Expr::Reference(reference) => {
                let key = self
                    .remap_master_reference(scope, origin, reference)
                    .ok()
                    .and_then(|r| self.reference_target(scope, &r).ok())
                    .and_then(|(s, n)| self.canonical_key(s, &n));
                key.map_or(Range::INVALID, |k| self.range_cell(&k, visited, depth + 1))
            }
            Expr::Unary(op, e) => {
                let a = self.range_expr(scope, origin, e, visited, depth + 1);
                match op {
                    UnaryOp::Plus => a,
                    UnaryOp::Minus => Range(-a.1, -a.0),
                    UnaryOp::Percent => Range(a.0 / 100., a.1 / 100.),
                }
            }
            Expr::Binary(op, a, b) => {
                let a = self.range_expr(scope, origin, a, visited, depth + 1);
                let b = self.range_expr(scope, origin, b, visited, depth + 1);
                match op {
                    BinaryOp::Add => Range(a.0 + b.0, a.1 + b.1).valid(),
                    BinaryOp::Sub => Range(a.0 - b.1, a.1 - b.0).valid(),
                    BinaryOp::Div if b.exact().is_some_and(|n| n > 0.) => Range(a.0 / b.0, a.1 / b.0).valid(),
                    BinaryOp::Mul if b.exact().is_some_and(|n| n > 0.) => Range(a.0 * b.0, a.1 * b.0).valid(),
                    _ if a.exact().is_some() && b.exact().is_some() => {
                        self.range_scalar(scope, origin, &Expr::Binary(*op, Box::new(Expr::Number(a.0, None)), Box::new(Expr::Number(b.0, None))))
                    }
                    _ => Range::INVALID,
                }
            }
            Expr::Call { name, args } if matches!(name.as_str(), "MIN" | "MAX") && !args.is_empty() => {
                let mut ranges = args.iter().map(|e| self.range_expr(scope, origin, e, visited, depth + 1));
                let first = ranges.next().unwrap_or(Range::INVALID);
                ranges.fold(first, |a, b| {
                    if a.0.is_nan() || b.0.is_nan() {
                        Range::INVALID
                    } else if name == "MIN" {
                        Range(a.0.min(b.0), a.1.min(b.1))
                    } else {
                        Range(a.0.max(b.0), a.1.max(b.1))
                    }
                })
            }
            Expr::Call { name, args } if name == "IF" && args.len() == 3 => {
                let condition = args.first().map_or(Range::INVALID, |e| self.range_expr(scope, origin, e, visited, depth + 1));
                if let Some(n) = condition.exact() {
                    args.get(if n == 0. { 2 } else { 1 }).map_or(Range::INVALID, |e| self.range_expr(scope, origin, e, visited, depth + 1))
                } else {
                    Range::INVALID
                }
            }
            Expr::Call { name, args } if name == "CEILING" && args.len() == 2 => {
                let a = args.first().map_or(Range::INVALID, |e| self.range_expr(scope, origin, e, visited, depth + 1));
                let step = args.get(1).map_or(Range::INVALID, |e| self.range_expr(scope, origin, e, visited, depth + 1));
                if step.exact().is_some_and(|n| n > 0.) {
                    Range((a.0 / step.0).ceil() * step.0, (a.1 / step.0).ceil() * step.0).valid()
                } else {
                    Range::INVALID
                }
            }
            Expr::Call { name, args } if name == "GUARD" && args.len() == 1 => {
                args.first().map_or(Range::INVALID, |e| self.range_expr(scope, origin, e, visited, depth + 1))
            }
            // Do not speculatively execute assignment or arbitrary host calls.
            Expr::Number(..) | Expr::Bool(_) => self.range_scalar(scope, origin, expr),
            Expr::Call { name, .. } if matches!(name.as_str(), "TEXTHEIGHT" | "TEXTWIDTH") => self.range_scalar(scope, origin, expr),
            _ => Range::INVALID,
        }
    }
    fn range_scalar(&mut self, scope: Scope, origin: Scope, expr: &Expr) -> Range {
        let mut context = BoundContext { evaluator: self, scope, origin, cached: Value::Error(Error::Cycle) };
        match diagramcraft_sheet::evaluate(expr, &mut context).numeric() {
            Ok(n) => Range::point(n),
            Err(Error::Cycle) => Range::UNKNOWN,
            Err(_) => Range::INVALID,
        }
    }
}
