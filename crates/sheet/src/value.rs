//! Values and non-panicking ShapeSheet errors.
use std::fmt;

/// ShapeSheet values are coerced by operators, with an invalid string becoming numeric zero.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-operators
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Text(String),
    Bool(bool),
    Point(f64, f64),
    Color { index: u32, rgb: [u8; 3], hsl: Option<[f64; 3]> },
    Error(Error),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Ref,
    Div0,
    Cycle,
    Num,
    Value,
    Parse(String),
    Unsupported(String),
    Limit,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ref => f.write_str("#REF!"),
            Self::Div0 => f.write_str("#DIV/0!"),
            Self::Cycle => f.write_str("#CYCLE!"),
            Self::Num => f.write_str("#NUM!"),
            Self::Value => f.write_str("#VALUE!"),
            Self::Limit => f.write_str("#LIMIT!"),
            Self::Parse(s) => write!(f, "#PARSE! {s}"),
            Self::Unsupported(s) => write!(f, "#NAME? {s}"),
        }
    }
}
impl Value {
    pub fn number(n: f64) -> Self {
        if n.is_finite() { Self::Number(if n == 0.0 { 0.0 } else { n }) } else { Self::Error(Error::Num) }
    }
    pub fn numeric(&self) -> Result<f64, Error> {
        let n = match self {
            Self::Number(n) => *n,
            Self::Color { index, .. } => f64::from(*index),
            Self::Bool(v) => {
                if *v {
                    1.0
                } else {
                    0.0
                }
            }
            Self::Text(s) => s.trim().parse().unwrap_or(0.0),
            Self::Point(..) => return Err(Error::Value),
            Self::Error(e) => return Err(e.clone()),
        };
        if n.is_finite() { Ok(n) } else { Err(Error::Num) }
    }
    pub fn truth(&self) -> Result<bool, Error> {
        self.numeric().map(|n| n != 0.0)
    }
    pub fn text(&self) -> String {
        match self {
            Self::Text(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::Color { index, .. } => index.to_string(),
            Self::Bool(b) => if *b { "TRUE" } else { "FALSE" }.into(),
            Self::Point(x, y) => format!("PNT({x},{y})"),
            Self::Error(e) => e.to_string(),
        }
    }
    pub fn error(&self) -> Option<Error> {
        if let Self::Error(e) = self { Some(e.clone()) } else { None }
    }
}
impl From<Result<f64, Error>> for Value {
    fn from(r: Result<f64, Error>) -> Self {
        match r {
            Ok(n) => Self::number(n),
            Err(e) => Self::Error(e),
        }
    }
}
