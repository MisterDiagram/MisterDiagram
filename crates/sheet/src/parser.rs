//! Bounded ShapeSheet formula lexer and parser.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/about-operators
use std::fmt;
const MAX_BYTES: usize = 64 * 1024;
const MAX_TOKENS: usize = 8192;
const MAX_DEPTH: usize = 128;

/// Formula syntax keeps universal units and reference spellings for contextual evaluation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-formulas
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-units-of-measure-visio-shapesheet-reference
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Missing,
    Number(f64, Option<String>),
    Text(String),
    Bool(bool),
    Reference(String),
    Qualified(Box<Expr>, String),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Call { name: String, args: Vec<Expr> },
}
/// Visio gives all unary operators precedence over exponentiation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-operators
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Percent,
}
/// Symbolic comparisons use Visio tolerance; underscore alternatives request exact comparison.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-operators
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Concat,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    ExactLt,
    ExactLe,
    ExactGt,
    ExactGe,
    ExactEq,
    ExactNe,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub offset: usize,
    pub message: String,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "formula byte {}: {}", self.offset, self.message)
    }
}
impl std::error::Error for ParseError {}
fn error(offset: usize, message: &str) -> ParseError {
    ParseError { offset, message: message.into() }
}
#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number(f64, Option<String>),
    Text(String),
    Name(String),
    Op(BinaryOp),
    Percent,
    Open,
    Close,
    Comma,
}
struct Lex<'a> {
    input: &'a str,
    at: usize,
}
impl<'a> Lex<'a> {
    fn ch(&self) -> Option<char> {
        self.input.get(self.at..)?.chars().next()
    }
    fn bump(&mut self) -> Option<char> {
        let c = self.ch()?;
        self.at += c.len_utf8();
        Some(c)
    }
    fn skip(&mut self) {
        while self.ch().is_some_and(char::is_whitespace) {
            self.bump();
        }
    }
    fn slice(&self, start: usize) -> &str {
        self.input.get(start..self.at).unwrap_or("")
    }
    fn number(&mut self) -> Result<Token, ParseError> {
        let start = self.at;
        while self.ch().is_some_and(|c| c.is_ascii_digit()) {
            self.bump();
        }
        if self.ch() == Some('.') {
            self.bump();
            while self.ch().is_some_and(|c| c.is_ascii_digit()) {
                self.bump();
            }
        }
        if self.ch().is_some_and(|c| c == 'e' || c == 'E') {
            let save = self.at;
            self.bump();
            if self.ch().is_some_and(|c| c == '+' || c == '-') {
                self.bump();
            }
            let digits = self.at;
            while self.ch().is_some_and(|c| c.is_ascii_digit()) {
                self.bump();
            }
            if self.at == digits {
                self.at = save;
            }
        }
        let mut value = self.slice(start).parse::<f64>().map_err(|_| error(start, "invalid number"))?;
        if !value.is_finite() {
            return Err(error(start, "non-finite number"));
        }
        let number_end = self.at;
        self.skip();
        // A whitespace-separated fractional part belongs to the same number-unit pair.
        // The units reference explicitly permits 5 1/2 in implicit-unit expressions.
        if self.at > number_end && self.ch().is_some_and(|c| c.is_ascii_digit()) {
            let saved = self.at;
            while self.ch().is_some_and(|c| c.is_ascii_digit()) {
                self.bump();
            }
            let numerator = self.slice(saved).parse::<f64>().map_err(|_| error(saved, "invalid fraction"))?;
            self.skip();
            if self.ch() == Some('/') {
                self.bump();
                self.skip();
                let denominator_start = self.at;
                while self.ch().is_some_and(|c| c.is_ascii_digit()) {
                    self.bump();
                }
                let denominator =
                    self.slice(denominator_start).parse::<f64>().map_err(|_| error(denominator_start, "invalid fraction denominator"))?;
                if denominator == 0.0 || !denominator.is_finite() || !numerator.is_finite() {
                    return Err(error(saved, "invalid fraction"));
                }
                value += numerator / denominator;
                if !value.is_finite() {
                    return Err(error(start, "non-finite mixed number"));
                }
                self.skip();
            } else {
                self.at = saved;
            }
        }
        let unit_start = self.at;
        if self.ch() == Some('[') {
            self.bracket()?;
        } else if self.ch().is_some_and(|c| c.is_ascii_alphabetic()) {
            while self.ch().is_some_and(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.') {
                self.bump();
            }
            if self.slice(unit_start).trim_end_matches('.').eq_ignore_ascii_case("SQ") {
                self.skip();
                while self.ch().is_some_and(|c| c.is_ascii_alphabetic() || c == '.') {
                    self.bump();
                }
            }
            if self.ch() == Some('/') && matches!(self.slice(unit_start).to_ascii_uppercase().as_str(), "FEET" | "CICERO") {
                self.bump();
                while self.ch().is_some_and(|c| c.is_ascii_alphabetic()) {
                    self.bump();
                }
            }
            self.skip();
            // A unit exponent is attached to the unit. Numeric exponentiation without a unit
            // remains the ordinary ^ operator.
            if self.ch() == Some('^') {
                let saved = self.at;
                self.bump();
                if self.ch() == Some('-') {
                    self.bump();
                }
                let digits = self.at;
                while self.ch().is_some_and(|c| c.is_ascii_digit()) {
                    self.bump();
                }
                if digits == self.at {
                    self.at = saved;
                }
            }
            self.skip();
            if self.ch() == Some('[') {
                self.bracket()?;
            }
        } else if self.ch().is_some_and(|c| c == '\'' || c == '"' || c == '°') {
            self.bump();
        }
        let unit = self.slice(unit_start).trim();
        Ok(Token::Number(value, if unit.is_empty() { None } else { Some(unit.into()) }))
    }
    fn bracket(&mut self) -> Result<(), ParseError> {
        let start = self.at;
        let mut depth = 0usize;
        while let Some(c) = self.bump() {
            match c {
                '[' => {
                    depth += 1;
                    if depth > MAX_DEPTH {
                        return Err(error(start, "bracket nesting limit"));
                    }
                }
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
        Err(error(start, "unclosed reference or unit bracket"))
    }
    fn quoted(&mut self, quote: char) -> Result<String, ParseError> {
        let start = self.at;
        self.bump();
        let mut out = String::new();
        loop {
            match self.bump() {
                Some(c) if c == quote => {
                    if self.ch() == Some(quote) {
                        self.bump();
                        out.push(quote)
                    } else {
                        return Ok(out);
                    }
                }
                Some(c) => out.push(c),
                None => return Err(error(start, "unclosed quoted text")),
            }
        }
    }
    fn name(&mut self) -> Result<Token, ParseError> {
        let start = self.at;
        while let Some(c) = self.ch() {
            if c == '[' {
                self.bracket()?;
            } else if c == '\'' {
                self.quoted('\'')?;
            } else if c.is_whitespace() || matches!(c, '+' | '-' | '*' | '/' | '^' | '&' | '<' | '>' | '=' | '%' | '(' | ')' | ',' | ';' | '"') {
                break;
            } else if c == ']' {
                return Err(error(self.at, "unmatched reference bracket"));
            } else if c.is_control() {
                return Err(error(self.at, "control character in reference"));
            } else {
                self.bump();
            }
        }
        if self.at == start {
            return Err(error(start, "unexpected token"));
        }
        let name = self.slice(start);
        Ok(match name.to_ascii_uppercase().as_str() {
            "_LT_" => Token::Op(BinaryOp::ExactLt),
            "_LE_" => Token::Op(BinaryOp::ExactLe),
            "_GT_" => Token::Op(BinaryOp::ExactGt),
            "_GE_" => Token::Op(BinaryOp::ExactGe),
            "_EQ_" => Token::Op(BinaryOp::ExactEq),
            "_NE_" => Token::Op(BinaryOp::ExactNe),
            _ => Token::Name(name.into()),
        })
    }
    fn tokens(mut self) -> Result<Vec<(Token, usize)>, ParseError> {
        let mut out = vec![];
        self.skip();
        if self.ch() == Some('=') {
            self.bump();
        }
        loop {
            self.skip();
            let at = self.at;
            let Some(c) = self.ch() else { break };
            if out.len() >= MAX_TOKENS {
                return Err(error(at, "token limit"));
            }
            let token = match c {
                '0'..='9' => self.number()?,
                '.' if self.input.get(self.at + 1..).and_then(|s| s.chars().next()).is_some_and(|c| c.is_ascii_digit()) => self.number()?,
                '"' => Token::Text(self.quoted('"')?),
                '{' => {
                    self.bump();
                    while self.ch().is_some_and(|c| c.is_ascii_hexdigit() || c == '-') {
                        self.bump();
                    }
                    if self.bump() != Some('}') {
                        return Err(error(at, "invalid GUID literal"));
                    }
                    Token::Text(self.slice(at).into())
                }
                '+' => {
                    self.bump();
                    Token::Op(BinaryOp::Add)
                }
                '-' => {
                    self.bump();
                    Token::Op(BinaryOp::Sub)
                }
                '*' => {
                    self.bump();
                    Token::Op(BinaryOp::Mul)
                }
                '/' => {
                    self.bump();
                    Token::Op(BinaryOp::Div)
                }
                '^' => {
                    self.bump();
                    Token::Op(BinaryOp::Pow)
                }
                '&' => {
                    self.bump();
                    Token::Op(BinaryOp::Concat)
                }
                '=' => {
                    self.bump();
                    Token::Op(BinaryOp::Eq)
                }
                '<' => {
                    self.bump();
                    let op = match self.ch() {
                        Some('=') => {
                            self.bump();
                            BinaryOp::Le
                        }
                        Some('>') => {
                            self.bump();
                            BinaryOp::Ne
                        }
                        _ => BinaryOp::Lt,
                    };
                    Token::Op(op)
                }
                '>' => {
                    self.bump();
                    let op = if self.ch() == Some('=') {
                        self.bump();
                        BinaryOp::Ge
                    } else {
                        BinaryOp::Gt
                    };
                    Token::Op(op)
                }
                '%' => {
                    self.bump();
                    Token::Percent
                }
                '(' => {
                    self.bump();
                    Token::Open
                }
                ')' => {
                    self.bump();
                    Token::Close
                }
                ',' | ';' => {
                    self.bump();
                    Token::Comma
                }
                _ => self.name()?,
            };
            out.push((token, at));
        }
        Ok(out)
    }
}
struct Parsed {
    expr: Expr,
    depth: usize,
}
struct Parser {
    tokens: Vec<(Token, usize)>,
    at: usize,
    nodes: usize,
}
impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.at).map(|t| &t.0)
    }
    fn position(&self) -> usize {
        self.tokens.get(self.at).map(|t| t.1).unwrap_or(0)
    }
    fn next(&mut self) -> Option<Token> {
        let token = self.peek()?.clone();
        self.at += 1;
        Some(token)
    }
    fn node(&mut self, expr: Expr, depth: usize) -> Result<Parsed, ParseError> {
        self.nodes += 1;
        if depth > MAX_DEPTH || self.nodes > MAX_TOKENS {
            return Err(error(self.position(), "expression complexity limit"));
        }
        Ok(Parsed { expr, depth })
    }
    fn expression(&mut self, min: u8, depth: usize) -> Result<Parsed, ParseError> {
        if depth >= MAX_DEPTH {
            return Err(error(self.position(), "expression nesting limit"));
        }
        let mut left = match self.next() {
            Some(Token::Number(n, u)) => {
                let has_unit = u.is_some();
                let mut value = self.node(Expr::Number(n, u), 1)?;
                if has_unit {
                    while matches!(self.peek(), Some(Token::Number(_, Some(_)))) {
                        if let Some(Token::Number(n, u)) = self.next() {
                            let rhs = self.node(Expr::Number(n, u), 1)?;
                            value = self.node(Expr::Binary(BinaryOp::Add, Box::new(value.expr), Box::new(rhs.expr)), value.depth + 1)?;
                        }
                    }
                }
                value
            }
            Some(Token::Text(t)) => self.node(Expr::Text(t), 1)?,
            Some(Token::Name(name)) => {
                if self.peek() == Some(&Token::Open) {
                    self.next();
                    let mut args = vec![];
                    let mut d = 0;
                    if self.peek() != Some(&Token::Close) {
                        loop {
                            let arg = if matches!(self.peek(), Some(Token::Comma | Token::Close)) {
                                self.node(Expr::Missing, 1)?
                            } else {
                                self.expression(0, depth + 1)?
                            };
                            d = d.max(arg.depth);
                            args.push(arg.expr);
                            if self.peek() != Some(&Token::Comma) {
                                break;
                            }
                            self.next();
                        }
                    }
                    if self.next() != Some(Token::Close) {
                        return Err(error(self.position(), "expected closing parenthesis"));
                    }
                    self.node(Expr::Call { name: name.to_ascii_uppercase(), args }, d + 1)?
                } else if name.eq_ignore_ascii_case("true") || name.eq_ignore_ascii_case("false") {
                    self.node(Expr::Bool(name.eq_ignore_ascii_case("true")), 1)?
                } else {
                    self.node(Expr::Reference(name), 1)?
                }
            }
            Some(Token::Op(BinaryOp::Add | BinaryOp::Sub)) => {
                let minus = self.tokens.get(self.at.saturating_sub(1)).is_some_and(|t| t.0 == Token::Op(BinaryOp::Sub));
                let arg = self.expression(8, depth + 1)?;
                self.node(Expr::Unary(if minus { UnaryOp::Minus } else { UnaryOp::Plus }, Box::new(arg.expr)), arg.depth + 1)?
            }
            Some(Token::Open) => {
                let e = self.expression(0, depth + 1)?;
                if self.next() != Some(Token::Close) {
                    return Err(error(self.position(), "expected closing parenthesis"));
                }
                e
            }
            _ => return Err(error(self.position(), "expected expression")),
        };
        loop {
            if let Some(Token::Name(suffix)) = self.peek()
                && suffix.starts_with('!')
            {
                let suffix = suffix.clone();
                self.next();
                left = self.node(Expr::Qualified(Box::new(left.expr), suffix), left.depth + 1)?;
                continue;
            }
            if self.peek() == Some(&Token::Percent) {
                if 8 < min {
                    break;
                }
                self.next();
                left = self.node(Expr::Unary(UnaryOp::Percent, Box::new(left.expr)), left.depth + 1)?;
                continue;
            }
            let Some(Token::Op(op)) = self.peek() else { break };
            let op = *op;
            let binding = precedence(op);
            if binding < min {
                break;
            }
            self.next();
            let right = self.expression(binding + 1, depth + 1)?;
            let d = left.depth.max(right.depth) + 1;
            left = self.node(Expr::Binary(op, Box::new(left.expr), Box::new(right.expr)), d)?;
        }
        Ok(left)
    }
}
fn precedence(op: BinaryOp) -> u8 {
    match op {
        BinaryOp::Eq | BinaryOp::Ne | BinaryOp::ExactEq | BinaryOp::ExactNe => 1,
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge | BinaryOp::ExactLt | BinaryOp::ExactLe | BinaryOp::ExactGt | BinaryOp::ExactGe => {
            2
        }
        BinaryOp::Concat => 3,
        BinaryOp::Add | BinaryOp::Sub => 4,
        BinaryOp::Mul | BinaryOp::Div => 5,
        BinaryOp::Pow => 6,
    }
}
/// Parse a universal formula with bounded bytes, tokens, recursion and final AST depth.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-operators
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-formulas
pub fn parse(input: &str) -> Result<Expr, ParseError> {
    if input.len() > MAX_BYTES {
        return Err(error(0, "formula byte limit"));
    }
    let tokens = Lex { input, at: 0 }.tokens()?;
    let mut parser = Parser { tokens, at: 0, nodes: 0 };
    let result = parser.expression(0, 0)?;
    if parser.peek().is_some() {
        return Err(error(parser.position(), "unexpected trailing token"));
    }
    Ok(result.expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn documented_precedence_examples() {
        let n = |x| Expr::Number(x, None);
        assert_eq!(
            parse("4 + 5 * 6"),
            Ok(Expr::Binary(BinaryOp::Add, Box::new(n(4.)), Box::new(Expr::Binary(BinaryOp::Mul, Box::new(n(5.)), Box::new(n(6.))))))
        );
        assert_eq!(
            parse("(4 + 5) * 6"),
            Ok(Expr::Binary(BinaryOp::Mul, Box::new(Expr::Binary(BinaryOp::Add, Box::new(n(4.)), Box::new(n(5.)))), Box::new(n(6.))))
        );
        assert_eq!(parse("-5^2"), Ok(Expr::Binary(BinaryOp::Pow, Box::new(Expr::Unary(UnaryOp::Minus, Box::new(n(5.)))), Box::new(n(2.)))));
        assert!(matches!(parse("2*5 & \" cents\""), Ok(Expr::Binary(BinaryOp::Concat, _, _))));
        assert!(matches!(parse("37%"), Ok(Expr::Unary(UnaryOp::Percent, _))));
    }
    #[test]
    fn references_strings_calls_and_units() {
        for reference in [
            "Geometry1.X1",
            "User.x",
            "Prop.x.Format",
            "Scratch.A1",
            "Sheet.17!PinX",
            "ThePage!PageWidth",
            "TheDoc!User.x",
            "Pages[Page Name]!Sheet.1!PinX",
            "Masters['A B']!Width",
            "'Page Name'!Sheet.2!PinX",
        ] {
            assert_eq!(parse(reference), Ok(Expr::Reference(reference.into())));
        }
        assert_eq!(
            parse("=gUaRd(IF(TRUE;1;2))"),
            Ok(Expr::Call {
                name: "GUARD".into(),
                args: vec![Expr::Call { name: "IF".into(), args: vec![Expr::Bool(true), Expr::Number(1., None), Expr::Number(2., None)] }]
            })
        );
        assert_eq!(parse("\"a\"\"b\""), Ok(Expr::Text("a\"b".into())));
        for unit in ["DL", "DP", "DT", "DA", "DE", "mm", "mm^2", "SQ. MM", "FEET/INCH", "[DL,1]"] {
            assert_eq!(parse(&format!("1.2e-3 {unit}")), Ok(Expr::Number(0.0012, Some(unit.into()))));
        }
    }
    #[test]
    fn exact_comparisons_and_inequality_precedence() {
        for (s, op) in [
            ("_GT_", BinaryOp::ExactGt),
            ("_GE_", BinaryOp::ExactGe),
            ("_LT_", BinaryOp::ExactLt),
            ("_LE_", BinaryOp::ExactLe),
            ("_EQ_", BinaryOp::ExactEq),
            ("_NE_", BinaryOp::ExactNe),
        ] {
            assert!(matches!(parse(&format!("5 {s} 2")),Ok(Expr::Binary(o,_,_))if o==op));
        }
        assert!(matches!(parse("1 < 2 = TRUE"), Ok(Expr::Binary(BinaryOp::Eq, _, _))));
    }
    #[test]
    fn hostile_inputs_are_bounded() {
        for input in ["", "(", "\"oops", "1e309", "1 +", "[oops", "a\u{0}b", "é + )"] {
            assert!(parse(input).is_err(), "{input}");
        }
        assert!(parse(&"x".repeat(MAX_BYTES + 1)).is_err());
        assert!(parse(&format!("{}1{}", "(".repeat(500), ")".repeat(500))).is_err());
        assert!(parse(&"1+".repeat(4000)).is_err());
        assert!(parse(&format!("IF({})", vec!["1"; 5000].join(","))).is_err());
        assert_eq!(parse("User.é"), Ok(Expr::Reference("User.é".into())));
    }
}
#[cfg(test)]
mod corpus_syntax_tests {
    use super::*;
    #[test]
    fn optional_arguments_dynamic_scopes_and_mixed_units() {
        assert_eq!(
            parse("SETATREF(User.x,,1)"),
            Ok(Expr::Call { name: "SETATREF".into(), args: vec![Expr::Reference("User.x".into()), Expr::Missing, Expr::Number(1., None)] })
        );
        assert_eq!(
            parse("LISTSHEETREF()!User.x"),
            Ok(Expr::Qualified(Box::new(Expr::Call { name: "LISTSHEETREF".into(), args: vec![] }), "!User.x".into()))
        );
        assert_eq!(
            parse("8FT 4IN"),
            Ok(Expr::Binary(BinaryOp::Add, Box::new(Expr::Number(8., Some("FT".into()))), Box::new(Expr::Number(4., Some("IN".into())))))
        );
        assert_eq!(parse("30°"), Ok(Expr::Number(30., Some("°".into()))));
        assert_eq!(
            parse("USE({035534F5-0002-0000-8E40-00608CF305B2})"),
            Ok(Expr::Call { name: "USE".into(), args: vec![Expr::Text("{035534F5-0002-0000-8E40-00608CF305B2}".into())] })
        );
        assert!(parse("abc]").is_err());
        assert!(parse("1 2").is_err());
    }
}

#[cfg(test)]
mod fractional_tests {
    use super::*;
    #[test]
    fn documented_fractional_implicit_literal() {
        assert_eq!(parse("5 1/2 [in,D]"), Ok(Expr::Number(5.5, Some("[in,D]".into()))));
        assert_eq!(parse("5 / 2"), Ok(Expr::Binary(BinaryOp::Div, Box::new(Expr::Number(5.0, None)), Box::new(Expr::Number(2.0, None)))));
        assert!(parse("5 1/0 in").is_err());
        assert!(parse("5 1/").is_err());
    }
}
