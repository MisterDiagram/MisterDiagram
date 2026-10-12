//! List membership is supplied by the document's Relationships cell.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/listsheetref-function
use crate::{Context, Error, Value, eval::eval, parser::Expr};

pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    if name != "LISTSHEETREF" {
        return None;
    }
    // Shipped files use no argument; the reference page's example uses (1).
    // A list member has one containing list, not an indexed collection.
    if args.len() > 1 {
        return Some(Value::Error(Error::Value));
    }
    if let Some(argument) = args.first() {
        match eval(argument, context, budget, depth + 1).numeric() {
            Ok(1.) => {}
            Ok(_) => return Some(Value::Error(Error::Ref)),
            Err(error) => return Some(Value::Error(error)),
        }
    }
    Some(context.list_sheet_reference())
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Host;
    impl Context for Host {
        fn resolve(&mut self, reference: &str) -> Value {
            if reference == "Sheet.2!Height" { Value::Number(4.) } else { Value::Error(Error::Ref) }
        }
        fn list_sheet_reference(&mut self) -> Value {
            Value::Text("Sheet.2".into())
        }
    }
    #[test]
    fn list_sheet_reference_examples_and_arity() {
        for formula in ["LISTSHEETREF()!Height", "LISTSHEETREF(1)!Height"] {
            assert_eq!(crate::formula(formula, &mut Host), Value::Number(4.));
        }
        assert_eq!(crate::formula("LISTSHEETREF(2)", &mut Host), Value::Error(Error::Ref));
        assert_eq!(crate::formula("LISTSHEETREF(1,2)", &mut Host), Value::Error(Error::Value));
        assert_eq!(crate::formula("LISTSHEETREF(1/0)", &mut Host), Value::Error(Error::Div0));
    }
}
