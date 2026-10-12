//! Reference-bound text queries. The document host supplies formatting and shaped metrics.
use crate::{Context, Error, Value, eval::eval, parser::Expr};
/// https://learn.microsoft.com/en-us/office/client-developer/visio/textwidth-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/textheight-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shapetext-function
pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    if !matches!(name, "TEXTWIDTH" | "TEXTHEIGHT" | "SHAPETEXT") {
        return None;
    }
    Some(
        (|| {
            if !(1..=2).contains(&args.len()) {
                return Err(Error::Value);
            }
            let Some(Expr::Reference(reference)) = args.first() else {
                return Err(Error::Ref);
            };
            context.dependency(reference);
            let arg = args.get(1).map(|e| eval(e, context, budget, depth + 1).numeric()).transpose()?;
            if name == "SHAPETEXT" {
                let flag = arg.unwrap_or(0.);
                if !flag.is_finite() || !(0. ..=127.).contains(&flag) || flag.fract() != 0. {
                    return Err(Error::Value);
                }
                let source = context.shape_text_flags(reference, flag as u32);
                if let Value::Error(e) = source {
                    return Err(e);
                }
                let source = source.text();
                if source.len() > 65536 {
                    return Err(Error::Limit);
                }
                let mut out = String::new();
                for c in source.chars() {
                    match c {
                        '\u{ad}' if flag as u32 & 1 == 0 => {}
                        '\t' if flag as u32 & 4 != 0 => out.push(' '),
                        '\t' if flag as u32 & 8 != 0 => out.push_str("        "),
                        '\r' | '\n' if flag as u32 & 16 != 0 => out.push(' '),
                        '‘' | '’' if flag as u32 & 32 != 0 => out.push('\''),
                        '“' | '”' if flag as u32 & 32 != 0 => out.push('"'),
                        _ => out.push(c),
                    }
                    if out.len() > 65536 {
                        return Err(Error::Limit);
                    }
                }
                if flag as u32 & 64 != 0 {
                    let mut previous = false;
                    out = out
                        .chars()
                        .filter_map(|c| {
                            let space = c.is_whitespace();
                            let keep = !space || !previous;
                            previous = space;
                            keep.then_some(if space { ' ' } else { c })
                        })
                        .collect();
                }
                return Ok(Value::Text(out));
            }
            if arg.is_some_and(|n| !n.is_finite() || n <= 0. || n > 1e6) {
                return Err(Error::Value);
            }
            Ok(context.text_metric(reference, arg, name == "TEXTHEIGHT"))
        })()
        .unwrap_or_else(Value::Error),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Host;
    impl Context for Host {
        fn resolve(&mut self, _: &str) -> Value {
            Value::Number(2.)
        }
        fn shape_text(&mut self, _: &str) -> Value {
            Value::Text("One\t\tTwo\r\nThree".into())
        }
        fn text_metric(&mut self, _: &str, width: Option<f64>, height: bool) -> Value {
            Value::Number(if height { width.unwrap_or(1.) } else { 3. })
        }
    }
    #[test]
    fn reference_examples_and_hostile_text_queries() {
        assert_eq!(crate::formula("TEXTWIDTH(TheText)", &mut Host), Value::Number(3.));
        assert_eq!(crate::formula("TEXTHEIGHT(TheText,Width-0.5in)", &mut Host), Value::Number(1.5));
        assert_eq!(crate::formula("SHAPETEXT(TheText,84)", &mut Host), Value::Text("One Two Three".into()));
        for f in ["TEXTWIDTH()", "TEXTHEIGHT(TheText,-1)", "SHAPETEXT(TheText,1e99)", "TEXTWIDTH(1)"] {
            assert!(matches!(crate::formula(f, &mut Host), Value::Error(_)));
        }
    }
}
