//! Reference functions, in-memory assignment and custom-function arguments.
use crate::{Context, Error, Value, eval::eval, parser::Expr};

pub const NAMES: &[&str] = &["SETATREF", "SETATREFEXPR", "SETATREFEVAL", "DEPENDSON", "SETF", "GETREF", "EVALCELL", "ARG", "EVALTEXT"];
const LIMIT: usize = 65536;

pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    if !NAMES.contains(&name) {
        return None;
    }
    Some(match dispatch(name, args, context, budget, depth) {
        Ok(value) => value,
        Err(error) => Value::Error(error),
    })
}
fn dispatch(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    match name {
        "SETATREF" => set_at_ref(args, context, budget, depth),
        "SETATREFEXPR" | "SETATREFEVAL" => set_expression(name, args, context, budget, depth),
        "GETREF" => {
            arity(args, 1, 1)?;
            Ok(Value::Text(reference(first(args)?, context, budget, depth)?))
        }
        "DEPENDSON" => depends_on(args, context, budget, depth),
        "SETF" => set_formula(args, context, budget, depth),
        "EVALCELL" => eval_cell(args, context, budget, depth),
        "ARG" => argument(args, context, budget, depth),
        "EVALTEXT" => eval_text(args, context, budget, depth),
        _ => Err(Error::Value),
    }
}
fn arity(args: &[Expr], min: usize, max: usize) -> Result<(), Error> {
    if (min..=max).contains(&args.len()) { Ok(()) } else { Err(Error::Value) }
}
fn first(args: &[Expr]) -> Result<&Expr, Error> {
    args.first().ok_or(Error::Value)
}
fn checked(value: Value) -> Result<Value, Error> {
    if let Some(error) = value.error() {
        return Err(error);
    }
    if matches!(&value, Value::Text(s) if s.len() > LIMIT) {
        return Err(Error::Limit);
    }
    Ok(value)
}
fn string(value: Value) -> Result<String, Error> {
    match checked(value)? {
        Value::Text(s) => Ok(s),
        _ => Err(Error::Value),
    }
}
/// GETREF preserves reference identity without reading a value or adding a dependency.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/getref-function
fn reference(expr: &Expr, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<String, Error> {
    let value = match expr {
        Expr::Reference(s) | Expr::Text(s) => {
            if s.len() > LIMIT {
                return Err(Error::Limit);
            }
            s.clone()
        }
        Expr::Qualified(base, suffix) => {
            let mut s = string(eval(base, context, budget, depth + 1))?;
            if s.len().saturating_add(suffix.len()) > LIMIT {
                return Err(Error::Limit);
            }
            s.push_str(suffix);
            s
        }
        _ => string(eval(expr, context, budget, depth + 1))?,
    };
    if value.is_empty() { Err(Error::Ref) } else { Ok(value) }
}
/// On reading, ignore_eval suppresses the reference and set_expression is not executed.
/// Update redirection belongs to the future edit path, which limits chains to ten.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/setatref-function
fn set_at_ref(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    arity(args, 1, 3)?;
    if let Some(ignore) = args.get(2)
        && eval(ignore, context, budget, depth + 1).truth()?
    {
        return Ok(Value::Number(0.));
    }
    let target = reference(first(args)?, context, budget, depth)?;
    Ok(context.resolve(&target))
}
/// SETATREFEXPR starts at zero; explicit values and SETATREFEVAL evaluate normally on reading.
/// Rewriting an expression during an edit is separate from read evaluation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/setatrefexpr-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/setatrefeval-function
fn set_expression(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    arity(args, usize::from(name == "SETATREFEVAL"), 1)?;
    Ok(args.first().map(|e| eval(e, context, budget, depth + 1)).unwrap_or(Value::Number(0.)))
}
/// DEPENDSON always returns false. The host omits dependencies for Event/Action cells.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/dependson-function
fn depends_on(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    arity(args, 1, 1024)?;
    for arg in args {
        let target = reference(arg, context, budget, depth)?;
        context.dependency(&target);
    }
    Ok(Value::Bool(false))
}
/// Validate the replacement before assigning through the host's in-memory formula overlay.
/// The numeric-zero return is observed in MWFCONCTRL_M.vssx master2 shape23 IsDropTarget.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/setf-function
fn set_formula(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    arity(args, 2, 2)?;
    let target = first(args)?;
    if !matches!(target, Expr::Text(_)) && !matches!(target, Expr::Call { name, .. } if name == "GETREF") {
        return Err(Error::Value);
    }
    let target = string(eval(target, context, budget, depth + 1))?;
    if target.is_empty() {
        return Err(Error::Ref);
    }
    let expr = args.get(1).ok_or(Error::Value)?;
    let value = checked(eval(expr, context, budget, depth + 1))?;
    let formula = value.text();
    if formula.len() > LIMIT {
        return Err(Error::Limit);
    }
    crate::parse(&formula).map_err(|e| Error::Parse(e.to_string()))?;
    context.set_formula(&target, &formula)?;
    Ok(Value::Number(0.))
}
/// Arguments are evaluated at the caller; the host evaluates the referenced formula in
/// its own scope with a bounded argument frame, without reusing an ordinary cell cache.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/evalcell-function
fn eval_cell(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    arity(args, 1, 1023)?;
    if args.len().is_multiple_of(2) {
        return Err(Error::Value);
    }
    let target = reference(first(args)?, context, budget, depth)?;
    let mut bindings = Vec::with_capacity(args.len() / 2);
    for pair in args.get(1..).ok_or(Error::Value)?.as_chunks::<2>().0 {
        let name = string(eval(first(pair)?, context, budget, depth + 1))?;
        let expr = pair.get(1).ok_or(Error::Value)?;
        if !matches!(expr, Expr::Missing) {
            let value = checked(eval(expr, context, budget, depth + 1))?;
            bindings.push((name, value));
        }
    }
    checked(context.eval_cell(&target, &bindings))
}
/// Defaults are lazy. Neither a binding nor a default produces an explicit value error:
/// the reference does not define a value for this case, so zero is not assumed.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/arg-function
fn argument(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    arity(args, 1, 2)?;
    let name = string(eval(first(args)?, context, budget, depth + 1))?;
    if let Some(value) = context.argument(&name) {
        return checked(value);
    }
    args.get(1).filter(|e| !matches!(e, Expr::Missing)).map(|e| checked(eval(e, context, budget, depth + 1))).unwrap_or(Err(Error::Value))
}
/// Shape text is parsed with the normal limits, then evaluated using the current budget.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/evaltext-function
fn eval_text(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    arity(args, 1, 1)?;
    let target = reference(first(args)?, context, budget, depth)?;
    let text = string(context.shape_text(&target))?;
    if text.is_empty() {
        return Ok(Value::Number(0.));
    }
    let expr = crate::parse(&text).map_err(|e| Error::Parse(e.to_string()))?;
    checked(eval(&expr, context, budget, depth + 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    #[derive(Default)]
    struct Example {
        formulas: BTreeMap<String, String>,
        arguments: BTreeMap<String, Value>,
        dependencies: Vec<String>,
        reads: Vec<String>,
        text: String,
    }
    impl Context for Example {
        fn resolve(&mut self, name: &str) -> Value {
            self.reads.push(name.into());
            match name {
                "Prop.Width" => Value::Number(4.),
                "User.DeltaX" => Value::Number(2.),
                "ParentShape!PinX" => Value::Number(5.),
                "User.GridX" => Value::Number(2.),
                _ => self.formulas.get(name).cloned().map(|f| crate::formula(&f, self)).unwrap_or(Value::Error(Error::Ref)),
            }
        }
        fn set_formula(&mut self, name: &str, formula: &str) -> Result<(), Error> {
            self.formulas.insert(name.into(), formula.into());
            Ok(())
        }
        fn eval_cell(&mut self, name: &str, args: &[(String, Value)]) -> Value {
            let previous = std::mem::take(&mut self.arguments);
            self.arguments = args.iter().cloned().collect();
            let value = self.resolve(name);
            self.arguments = previous;
            value
        }
        fn argument(&self, name: &str) -> Option<Value> {
            self.arguments.get(name).cloned()
        }
        fn dependency(&mut self, name: &str) {
            self.dependencies.push(name.into());
        }
        fn shape_text(&mut self, _: &str) -> Value {
            Value::Text(self.text.clone())
        }
    }
    #[test]
    fn microsoft_setatref_read_examples() {
        let mut c = Example::default();
        assert_eq!(crate::formula("SETATREF(Prop.Width)", &mut c), Value::Number(4.));
        assert_eq!(crate::formula("SETATREF(User.DeltaX,SETATREFEVAL(SETATREFEXPR()-ParentShape!PinX))+ParentShape!PinX", &mut c), Value::Number(7.));
        assert_eq!(crate::formula("INT(SETATREFEXPR()/User.GridX+.5)*User.GridX", &mut c), Value::Number(0.));
        assert_eq!(crate::formula("SETATREFEXPR(3)", &mut c), Value::Number(3.));
        assert_eq!(crate::formula("SETATREFEVAL(3+4)", &mut c), Value::Number(7.));
        c.reads.clear();
        assert_eq!(crate::formula("SETATREF(Missing,SETF(GETREF(PinX),9),TRUE)", &mut c), Value::Number(0.));
        assert!(c.reads.is_empty());
        assert!(c.formulas.is_empty());
    }
    #[test]
    fn microsoft_setf_and_getref_examples() {
        let mut c = Example::default();
        assert_eq!(crate::formula("SETF(GETREF(Scratch.A1),1.5in*6+1ft)", &mut c), Value::Number(0.));
        assert_eq!(c.formulas.get("Scratch.A1").map(String::as_str), Some("21"));
        assert_eq!(crate::formula(r#"SETF(GETREF(Scratch.A1),"1.5in*6+1ft")"#, &mut c), Value::Number(0.));
        assert_eq!(c.formulas.get("Scratch.A1").map(String::as_str), Some("1.5in*6+1ft"));
        assert_eq!(crate::formula(r#"SETF(GETREF(Scratch.A1),"""Say """"ahh""""""")"#, &mut c), Value::Number(0.));
        assert_eq!(c.resolve("Scratch.A1"), Value::Text("Say \"ahh\"".into()));
        c.reads.clear();
        assert_eq!(crate::formula("SETF(GETREF(PinX),5.1)", &mut c), Value::Number(0.));
        assert!(c.reads.is_empty());
        assert!(c.dependencies.is_empty());
        let before = c.formulas.clone();
        assert_eq!(crate::formula("SETF(PinX,9)", &mut c), Value::Error(Error::Value));
        assert!(matches!(crate::formula(r#"SETF("PinX","1+")"#, &mut c), Value::Error(Error::Parse(_))));
        assert_eq!(c.formulas, before);
    }
    #[test]
    fn microsoft_custom_function_examples_and_lazy_default() {
        let mut c = Example::default();
        c.formulas.insert("User.MiddleValue".into(), r#"IF(ARG("A")>ARG("B"),IF(ARG("B")>ARG("C"),ARG("B"),IF(ARG("A")>ARG("C"),ARG("C"),ARG("A"))),IF(ARG("A")>ARG("C"),ARG("A"),IF(ARG("B")>ARG("C"),ARG("C"),ARG("B"))))"#.into());
        assert_eq!(crate::formula(r#"EVALCELL(User.MiddleValue,"A",3,"B",9,"C",5)"#, &mut c), Value::Number(5.));
        assert_eq!(crate::formula(r#"EVALCELL(User.MiddleValue,"A",12,"B",0,"C",21)"#, &mut c), Value::Number(12.));
        assert_eq!(crate::formula(r#"ARG("Absent",4)"#, &mut c), Value::Number(4.));
        assert_eq!(crate::formula(r#"ARG("Absent")"#, &mut c), Value::Error(Error::Value));
        c.arguments.insert("Present".into(), Value::Number(2.));
        assert_eq!(crate::formula(r#"ARG("Present",1/0)"#, &mut c), Value::Number(2.));
    }
    #[test]
    fn microsoft_dependson_and_evaltext_examples() {
        let mut c = Example { text: "4 ft + 0.5 ft".into(), ..Example::default() };
        assert_eq!(crate::formula("DEPENDSON(PinX,PinY)", &mut c), Value::Bool(false));
        assert_eq!(c.dependencies, ["PinX", "PinY"]);
        assert!(c.reads.is_empty());
        assert_eq!(crate::formula("EVALTEXT(Line.2!theText)", &mut c), Value::Number(54.));
        c.text.clear();
        assert_eq!(crate::formula("EVALTEXT(TheText)", &mut c), Value::Number(0.));
    }
    #[test]
    fn hostile_references_text_and_nested_eval_are_bounded() {
        let mut c = Example { text: "λ".repeat(LIMIT), ..Example::default() };
        assert_eq!(crate::formula("EVALTEXT(TheText)", &mut c), Value::Error(Error::Limit));
        c.text = "EVALTEXT(TheText)".into();
        assert_eq!(crate::formula("EVALTEXT(TheText)", &mut c), Value::Error(Error::Limit));
        let ast = Expr::Call { name: "GETREF".into(), args: vec![Expr::Reference("λ".repeat(LIMIT))] };
        assert_eq!(crate::evaluate(&ast, &mut c), Value::Error(Error::Limit));
        for formula in ["GETREF()", "SETF()", "SETATREFEVAL()", "EVALCELL(User.F,1)", "DEPENDSON()", "EVALTEXT(2)", "ARG()"] {
            assert!(matches!(crate::formula(formula, &mut c), Value::Error(_)), "{formula}");
        }
    }
}
