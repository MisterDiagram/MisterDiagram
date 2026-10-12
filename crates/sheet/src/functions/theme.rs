use crate::{Context, Error, Value, eval::eval, parser::Expr};

/// Theme guards affect edits; evaluation preserves their argument.
/// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/7c2c0bc3-4bd3-46b1-b368-8b2e907cda67
/// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/bd72b8b3-fbf7-4463-ad23-cd8a5cc4acfd
/// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/b81d9641-31a2-4f60-9b74-eba256dd58e2
/// https://learn.microsoft.com/en-us/office/client-developer/visio/themeguard-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/themerestore-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/isthemed-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/theme-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/themecbv-function
pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    let arity = match name {
        "ISTHEMED" => 0..=0,
        "THEMEGUARD" | "THEMERESTORE" | "CELLISTHEMED" | "THEMEPROP" => 1..=1,
        "THEME" => 1..=1,
        "THEMEVAL" => 0..=2,
        "THEMECBV" => 2..=2,
        _ => return None,
    };
    if !arity.contains(&args.len()) {
        return Some(Value::Error(Error::Value));
    }
    if name == "ISTHEMED" {
        return Some(Value::from(context.theme_selector("ColorSchemeIndex").numeric().map(|v| if v > 0. { 1. } else { 0. })));
    }
    if name == "THEMERESTORE" {
        return Some(Value::Number(0.));
    }
    let values: Vec<_> = args.iter().map(|arg| eval(arg, context, budget, depth + 1)).collect();
    if name == "THEMECBV" {
        let stop = match values.get(1).ok_or(Error::Value).and_then(Value::numeric) {
            Ok(n) => n,
            Err(e) => return Some(Value::Error(e)),
        };
        if !(1. ..=10.).contains(&stop) || stop.fract() != 0. {
            return Some(Value::Error(Error::Value));
        }
    }
    if name == "CELLISTHEMED" {
        return Some(match values.first().ok_or(Error::Value).and_then(Value::truth) {
            Ok(v) => Value::Bool(v),
            Err(e) => Value::Error(e),
        });
    }
    if name == "THEMEGUARD" {
        return Some(values.first().cloned().unwrap_or(Value::Error(Error::Value)));
    }
    Some(context.theme(name, &values))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct C;
    impl Context for C {
        fn resolve(&mut self, _: &str) -> Value {
            Value::Number(0.)
        }
        fn theme(&mut self, _: &str, _: &[Value]) -> Value {
            Value::Number(27.)
        }
    }
    #[test]
    fn theme_guards_and_legacy_boolean_are_evaluated() {
        assert_eq!(crate::formula("THEMEGUARD(THEME(\"AccentColor2\"))", &mut C), Value::Number(27.));
        assert_eq!(crate::formula("THEMERESTORE(RGB(255,102,0))", &mut C), Value::Number(0.));
        assert_eq!(crate::formula("CELLISTHEMED(FALSE)", &mut C), Value::Bool(false));
        assert_eq!(crate::formula("CELLISTHEMED(TRUE)", &mut C), Value::Bool(true));
        assert_eq!(crate::formula("ISTHEMED()", &mut C), Value::Number(0.));
        assert_eq!(crate::formula("THEMEVAL(1,2,3)", &mut C), Value::Error(Error::Value));
        assert_eq!(crate::formula("THEMECBV(0,11)", &mut C), Value::Error(Error::Value));
    }
}
