//! Lazy formula-presence and path-length queries through the document context.
use crate::{Context, Error, Value, eval::eval, parser::Expr};
pub const NAMES: &[&str] = &["FORMULAEXISTS", "PATHLENGTH"];
pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    if !NAMES.contains(&name) {
        return None;
    }
    Some(match query(name, args, context, budget, depth) {
        Ok(value) => value,
        Err(error) => Value::Error(error),
    })
}
/// Check a reference's formula without evaluating its value; constants are formulas too.
/// No Formula is distinct from an ordinary constant stored without an XML F attribute.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/formulaexists-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pathlength-function
fn query(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    if args.is_empty() || args.len() > if name == "FORMULAEXISTS" { 1 } else { 2 } {
        return Err(Error::Value);
    }
    let reference = super::points::reference(args.first().ok_or(Error::Value)?, context, budget, depth + 1)?;
    if name == "FORMULAEXISTS" {
        return Ok(context.formula_exists(&reference));
    }
    let segment = match args.get(1).filter(|e| !matches!(e, Expr::Missing)) {
        None => None,
        Some(expr) => {
            let n = eval(expr, context, budget, depth + 1).numeric()?;
            if n < 1. || n > f64::from(u32::MAX) || n.fract() != 0. {
                return Err(Error::Ref);
            }
            Some(usize::try_from(n as u32).map_err(|_| Error::Ref)?)
        }
    };
    Ok(context.path_length(&reference, segment))
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Host {
        reads: usize,
    }
    impl Context for Host {
        fn resolve(&mut self, _: &str) -> Value {
            self.reads += 1;
            Value::Error(Error::Div0)
        }
        fn formula_exists(&mut self, r: &str) -> Value {
            Value::Number(if r == "Width" { 1. } else { 0. })
        }
        fn path_length(&mut self, r: &str, s: Option<usize>) -> Value {
            if r != "Geometry1.Path" {
                Value::Error(Error::Ref)
            } else {
                match s {
                    None => Value::Number(7.),
                    Some(1) => Value::Number(3.),
                    Some(2) => Value::Number(4.),
                    _ => Value::Error(Error::Ref),
                }
            }
        }
    }
    #[test]
    fn documented_reference_syntax_is_lazy() {
        let mut host = Host { reads: 0 };
        assert_eq!(crate::formula("FORMULAEXISTS(Width)", &mut host), Value::Number(1.));
        assert_eq!(crate::formula("FORMULAEXISTS(\"Missing\")", &mut host), Value::Number(0.));
        assert_eq!(crate::formula("PATHLENGTH(Geometry1.Path)", &mut host), Value::Number(7.));
        assert_eq!(crate::formula("PATHLENGTH(IF(1,Geometry1.Path,Missing),2)", &mut host), Value::Number(4.));
        assert_eq!(host.reads, 0);
    }
    #[test]
    fn malformed_arguments_and_hostile_references_return_errors() {
        let mut host = Host { reads: 0 };
        for f in ["PATHLENGTH()", "FORMULAEXISTS()", "FORMULAEXISTS(Width,1)"] {
            assert_eq!(crate::formula(f, &mut host), Value::Error(Error::Value));
        }
        for f in
            ["PATHLENGTH(Geometry1.Path,0)", "PATHLENGTH(Geometry1.Path,-1)", "PATHLENGTH(Geometry1.Path,1.5)", "PATHLENGTH(Geometry1.Path,1e300)"]
        {
            assert_eq!(crate::formula(f, &mut host), Value::Error(Error::Ref));
        }
        let args = [Expr::Reference("é".repeat(65536))];
        let mut budget = 100;
        assert_eq!(call("PATHLENGTH", &args, &mut host, &mut budget, 0), Some(Value::Error(Error::Limit)));
        let mut budget = 0;
        assert_eq!(call("PATHLENGTH", &[Expr::Reference("Geometry1.Path".into())], &mut host, &mut budget, 0), Some(Value::Error(Error::Limit)));
    }
}
