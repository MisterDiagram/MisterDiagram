//! Bounded evaluation with host-provided reference, palette and font lookup.
use crate::{
    Error, Value,
    parser::{BinaryOp, Expr, UnaryOp},
    units,
};

/// Host hooks retain the cached value until their host subsystem is implemented.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function
pub trait HostHooks {
    fn hook(&mut self, _name: &str, _arguments: &[Value], cached: &Value) -> Value {
        cached.clone()
    }
}
pub struct CachedHooks;
impl HostHooks for CachedHooks {}

/// The host binds universal references to cells and document Colors/FaceNames tables.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-formulas
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/facenames-element-visiodocument_type-complextypevisio-xml
pub trait Context {
    fn resolve(&mut self, reference: &str) -> Value;
    fn cached(&self) -> Value {
        Value::Error(Error::Ref)
    }
    fn color(&self, _index: u32) -> Option<[u8; 3]> {
        None
    }
    fn color_index(&mut self, _rgb: [u8; 3]) -> Option<u32> {
        None
    }
    fn font(&self, _index: u32) -> Option<String> {
        None
    }
    fn font_index(&self, _name: &str) -> Option<u32> {
        None
    }
    fn hook(&mut self, _name: &str, _args: &[Value]) -> Value {
        self.cached()
    }
}
/// Frozen Phase 1 hook membership, matched exactly rather than by a broad prefix.
pub fn is_hook(name: &str) -> bool {
    include_str!("../tests/hooks.txt").lines().any(|l| l.split('#').next().is_some_and(|s| s.trim().eq_ignore_ascii_case(name)))
}
/// Evaluate a parsed formula, bounding both visited nodes and caller-built AST nesting.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-operators
pub fn evaluate(expr: &Expr, context: &mut dyn Context) -> Value {
    let mut budget = 16384;
    eval(expr, context, &mut budget, 0)
}
pub fn formula(formula: &str, context: &mut dyn Context) -> Value {
    match crate::parse(formula) {
        Ok(expr) => evaluate(&expr, context),
        Err(e) => Value::Error(Error::Parse(e.to_string())),
    }
}
pub(crate) fn eval(expr: &Expr, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Value {
    if *budget == 0 || depth > 128 {
        return Value::Error(Error::Limit);
    }
    *budget -= 1;
    match expr {
        Expr::Missing => Value::Number(0.0),
        Expr::Qualified(base, suffix) => {
            let value = eval(base, context, budget, depth + 1);
            if let Some(e) = value.error() {
                return Value::Error(e);
            }
            let base = value.text();
            if base.len().saturating_add(suffix.len()) > 65536 {
                return Value::Error(Error::Limit);
            }
            context.resolve(&format!("{base}{suffix}"))
        }
        Expr::Number(n, u) => units::number(*n, u.as_deref()),
        Expr::Text(s) => {
            if s.len() > 65536 {
                Value::Error(Error::Limit)
            } else {
                Value::Text(s.clone())
            }
        }
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Reference(r) => context.resolve(r),
        Expr::Unary(op, e) => {
            let n = eval(e, context, budget, depth + 1).numeric();
            Value::from(n.map(|v| match op {
                UnaryOp::Plus => v,
                UnaryOp::Minus => -v,
                UnaryOp::Percent => v / 100.0,
            }))
        }
        Expr::Binary(op, a, b) => {
            let left = eval(a, context, budget, depth + 1);
            if let Some(e) = left.error() {
                return Value::Error(e);
            }
            let right = eval(b, context, budget, depth + 1);
            binary(*op, left, right)
        }
        Expr::Call { name, args } => {
            if args.len() > 1024 {
                return Value::Error(Error::Limit);
            }
            if is_hook(name) {
                let values: Vec<_> = args.iter().map(|a| eval(a, context, budget, depth + 1)).collect();
                return context.hook(name, &values);
            }
            crate::functions::call(name, args, context, budget, depth)
        }
    }
}
fn binary(op: BinaryOp, a: Value, b: Value) -> Value {
    if let Some(e) = b.error() {
        return Value::Error(e);
    }
    if matches!(op, BinaryOp::Concat) {
        let mut a = a.text();
        let b = b.text();
        if a.len().saturating_add(b.len()) > 65536 {
            return Value::Error(Error::Limit);
        }
        a.push_str(&b);
        return Value::Text(a);
    }
    let (x, y) = match (a.numeric(), b.numeric()) {
        (Ok(x), Ok(y)) => (x, y),
        (Err(e), _) | (_, Err(e)) => return Value::Error(e),
    };
    let near = (x - y).abs() <= 1e-10 * x.abs().max(y.abs()).max(1.0);
    match op {
        BinaryOp::Add => Value::number(x + y),
        BinaryOp::Sub => Value::number(x - y),
        BinaryOp::Mul => Value::number(x * y),
        BinaryOp::Div => {
            if y == 0.0 {
                Value::Error(Error::Div0)
            } else {
                Value::number(x / y)
            }
        }
        BinaryOp::Pow => Value::number(x.powf(y)),
        BinaryOp::Eq => Value::Bool(near),
        BinaryOp::Ne => Value::Bool(!near),
        BinaryOp::Lt => Value::Bool(x < y && !near),
        BinaryOp::Le => Value::Bool(x < y || near),
        BinaryOp::Gt => Value::Bool(x > y && !near),
        BinaryOp::Ge => Value::Bool(x > y || near),
        BinaryOp::ExactEq => Value::Bool(x == y),
        BinaryOp::ExactNe => Value::Bool(x != y),
        BinaryOp::ExactLt => Value::Bool(x < y),
        BinaryOp::ExactLe => Value::Bool(x <= y),
        BinaryOp::ExactGt => Value::Bool(x > y),
        BinaryOp::ExactGe => Value::Bool(x >= y),
        BinaryOp::Concat => Value::Error(Error::Value),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    struct TestContext;
    impl Context for TestContext {
        fn resolve(&mut self, r: &str) -> Value {
            match r {
                "Width" => Value::Number(2.0),
                "cycle" => Value::Error(Error::Cycle),
                _ => Value::Error(Error::Ref),
            }
        }
        fn cached(&self) -> Value {
            Value::Number(42.0)
        }
    }
    #[test]
    fn microsoft_operator_examples() {
        for (f, n) in [("4+5*6", 34.0), ("(4+5)*6", 54.0), ("5*\"2\"", 10.0), ("5*\"sheep\"", 0.0), ("37%", 0.37), ("Width*.5", 1.0)] {
            assert_eq!(formula(f, &mut TestContext), Value::Number(n), "{f}")
        }
        assert_eq!(formula("2*5 & \" cents\"", &mut TestContext), Value::Text("10 cents".into()));
    }
    #[test]
    fn errors_and_cached_hooks_are_values() {
        assert_eq!(formula("1/0", &mut TestContext), Value::Error(Error::Div0));
        assert_eq!(formula("Missing", &mut TestContext), Value::Error(Error::Ref));
        assert_eq!(formula("cycle", &mut TestContext), Value::Error(Error::Cycle));
        assert_eq!(formula("THEMEVAL()", &mut TestContext), Value::Number(42.0));
        assert!(matches!(formula("UNKNOWN()", &mut TestContext), Value::Error(Error::Unsupported(_))));
    }
    #[test]
    fn public_ast_limits() {
        let mut e = Expr::Number(1.0, None);
        for _ in 0..130 {
            e = Expr::Unary(UnaryOp::Minus, Box::new(e));
        }
        assert_eq!(evaluate(&e, &mut TestContext), Value::Error(Error::Limit));
    }
}
