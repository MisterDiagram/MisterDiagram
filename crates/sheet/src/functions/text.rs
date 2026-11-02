//! Bounded text functions and deterministic English format pictures.
use crate::{
    Context, Error, Value,
    eval::eval,
    parser::{BinaryOp, Expr},
    units,
};
pub const NAMES: &[&str] = &["FORMAT", "FORMATEX", "STRSAME", "TRIM", "LEN", "FIND", "LEFT", "RIGHT", "MID", "INDEX", "LOOKUP", "LISTSEP", "CHAR"];
const LIMIT: usize = 65536;
fn text(value: &Value) -> Result<String, Error> {
    if matches!(value, Value::Text(s) if s.len() > LIMIT) {
        return Err(Error::Limit);
    }
    if let Some(e) = value.error() {
        return Err(e);
    }
    let s = value.text();
    if s.len() > LIMIT { Err(Error::Limit) } else { Ok(s) }
}
fn get(values: &[Value], i: usize) -> Result<&Value, Error> {
    values.get(i).ok_or(Error::Value)
}
fn string(values: &[Value], i: usize) -> Result<String, Error> {
    text(get(values, i)?)
}
fn num(values: &[Value], i: usize) -> Result<f64, Error> {
    get(values, i)?.numeric()
}
fn optional(args: &[Expr], values: &[Value], i: usize) -> Option<Value> {
    args.get(i).filter(|e| !matches!(e, Expr::Missing)).and_then(|_| values.get(i)).cloned()
}
fn result(result: Result<String, Error>) -> Value {
    match result {
        Ok(s) if s.len() <= LIMIT => Value::Text(s),
        Ok(_) => Value::Error(Error::Limit),
        Err(e) => Value::Error(e),
    }
}
/// Dispatch text functions; LISTSEP uses the host locale separator.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/listsep-function
pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    if !NAMES.contains(&name) {
        return None;
    }
    let values: Vec<_> = args.iter().map(|e| eval(e, context, budget, depth + 1)).collect();
    Some(match name {
        "LEN" => {
            if args.len() != 1 {
                Value::Error(Error::Value)
            } else {
                match string(&values, 0) {
                    Ok(s) => Value::Number(s.chars().count() as f64),
                    Err(e) => Value::Error(e),
                }
            }
        }
        "LEFT" | "RIGHT" => result(edge(name, args, &values)),
        "MID" => result(mid(args, &values)),
        "FIND" => Value::from(find(args, &values)),
        "STRSAME" => same(args, &values),
        "TRIM" => result(trim(args, &values)),
        "INDEX" => index(args, &values),
        "LOOKUP" => lookup(args, &values),
        "LISTSEP" => {
            if args.is_empty() {
                result(Ok(context.list_separator().into()))
            } else {
                Value::Error(Error::Value)
            }
        }
        "FORMAT" | "FORMATEX" => result(format_value(name, args, &values, context)),
        "CHAR" => result(char_code(args, &values)),
        _ => Value::Error(Error::Value),
    })
}
/// CHAR returns the character for a Unicode scalar value; zero, surrogates and values past
/// U+10FFFF are errors, and a fractional code is truncated like the other integer arguments.
/// Witness: the Space shapes of a Microsoft floor-plan template label themselves with
/// Prop.Name&CHAR(10)&User.visFMAreaDisplay (#89).
/// https://learn.microsoft.com/en-us/office/client-developer/visio/char-function
fn char_code(args: &[Expr], values: &[Value]) -> Result<String, Error> {
    if args.len() != 1 {
        return Err(Error::Value);
    }
    let code = num(values, 0)?;
    if !code.is_finite() || !(1.0..=1_114_111.0).contains(&code) {
        return Err(Error::Value);
    }
    char::from_u32(code.trunc() as u32).map(String::from).ok_or(Error::Value)
}
/// Normalize the explicit MS-VSDX v20150316 section 2.5.3.166 whitespace set.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/trim-function
fn trim(args: &[Expr], values: &[Value]) -> Result<String, Error> {
    if args.len() != 1 {
        return Err(Error::Value);
    }
    let input = string(values, 0)?;
    let mut output = String::with_capacity(input.len());
    let mut gap = false;
    for ch in input.chars() {
        if matches!(ch, '\u{0009}'..='\u{000d}' | ' ' | '\u{00a0}' | '\u{1680}' | '\u{180e}' | '\u{2000}'..='\u{200a}' | '\u{202f}' | '\u{205f}' | '\u{3000}')
        {
            gap = !output.is_empty();
        } else {
            if gap {
                output.push(' ');
                gap = false;
            }
            output.push(ch);
        }
    }
    Ok(output)
}

/// Character counts use Unicode scalar values, avoiding invalid UTF-8 slices.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/len-function-visioshapesheet
/// https://learn.microsoft.com/en-us/office/client-developer/visio/left-function-visioshapesheet
/// https://learn.microsoft.com/en-us/office/client-developer/visio/right-function-visioshapesheet
fn edge(name: &str, args: &[Expr], values: &[Value]) -> Result<String, Error> {
    if !(1..=2).contains(&args.len()) {
        return Err(Error::Value);
    }
    let s = string(values, 0)?;
    let n = optional(args, values, 1).map(|v| v.numeric()).transpose()?.unwrap_or(1.);
    if n < 0. {
        return Err(Error::Value);
    }
    let count = s.chars().count();
    let n = bounded_count(n, count);
    let skip = if name == "RIGHT" { count.saturating_sub(n) } else { 0 };
    Ok(s.chars().skip(skip).take(n).collect())
}
fn bounded_count(n: f64, max: usize) -> usize {
    if n >= max as f64 {
        max
    } else if n <= 0. {
        0
    } else {
        n.trunc() as usize
    }
}
/// Positions are one-based. A negative count follows the published rule and returns
/// the remaining substring; this edge is reference-based, not yet a Visio corpus measurement.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/mid-function-visioshapesheet
fn mid(args: &[Expr], values: &[Value]) -> Result<String, Error> {
    if args.len() != 3 {
        return Err(Error::Value);
    }
    let s = string(values, 0)?;
    let start = num(values, 1)?;
    let length = num(values, 2)?;
    if start < 1. {
        return Err(Error::Value);
    }
    let count = s.chars().count();
    let skip = bounded_count(start - 1., count);
    let take = if length < 0. { count } else { bounded_count(length, count) };
    Ok(s.chars().skip(skip).take(take).collect())
}
/// One-based literal substring search, with optional start and case folding.
/// Unicode positions follow LEN; allocations are capped by the 65536-byte inputs.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/find-function
fn find(args: &[Expr], values: &[Value]) -> Result<f64, Error> {
    if !(2..=4).contains(&args.len()) {
        return Err(Error::Value);
    }
    let needle = string(values, 0)?;
    let haystack = string(values, 1)?;
    let count = haystack.chars().count();
    let start = optional(args, values, 2).map(|v| v.numeric()).transpose()?.unwrap_or(1.);
    if start < 1. || start > count as f64 {
        return Err(Error::Value);
    }
    let start = bounded_count(start - 1., count);
    let ignore = optional(args, values, 3).map(|v| v.truth()).transpose()?.unwrap_or(false);
    if needle.is_empty() {
        return Ok(start as f64 + 1.);
    }
    let offset = haystack.char_indices().nth(start).map(|(i, _)| i).ok_or(Error::Value)?;
    let rest = haystack.get(offset..).ok_or(Error::Value)?;
    if !ignore {
        let at = rest.find(&needle).ok_or(Error::Value)?;
        return Ok(start as f64 + rest.get(..at).ok_or(Error::Value)?.chars().count() as f64 + 1.);
    }
    // A lowercase expansion can change byte and character counts. Retain the original
    // scalar index so a later match still reports a position in the original string.
    let mut folded = String::new();
    let mut positions = Vec::new();
    for (index, ch) in rest.chars().enumerate() {
        positions.push((folded.len(), index));
        folded.extend(ch.to_lowercase());
        if folded.len() > LIMIT * 3 {
            return Err(Error::Limit);
        }
    }
    let needle = needle.to_lowercase();
    if needle.len() > LIMIT * 3 {
        return Err(Error::Limit);
    }
    let at = folded.find(&needle).ok_or(Error::Value)?;
    let index = positions.partition_point(|(byte, _)| *byte <= at).checked_sub(1).ok_or(Error::Value)?;
    Ok(start as f64 + positions.get(index).ok_or(Error::Value)?.1 as f64 + 1.)
}
/// Optional ignoreCase is false; Unicode case conversion handles the page's cät/CÄT example.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/strsame-function
fn same(args: &[Expr], values: &[Value]) -> Value {
    let f = || -> Result<bool, Error> {
        if !(2..=3).contains(&args.len()) {
            return Err(Error::Value);
        }
        let a = string(values, 0)?;
        let b = string(values, 1)?;
        let ignore = optional(args, values, 2).map(|v| v.truth()).transpose()?.unwrap_or(false);
        Ok(if ignore { a.to_lowercase() == b.to_lowercase() } else { a == b })
    };
    match f() {
        Ok(b) => Value::Bool(b),
        Err(e) => Value::Error(e),
    }
}
/// INDEX uses zero-based positions and preserves empty fields, including end fields.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/index-function
fn index(args: &[Expr], values: &[Value]) -> Value {
    let f = || -> Result<Value, Error> {
        if !(2..=4).contains(&args.len()) {
            return Err(Error::Value);
        }
        let index = num(values, 0)?;
        let s = string(values, 1)?;
        let delimiter = optional(args, values, 2).map(|v| text(&v)).transpose()?.unwrap_or_else(|| ";".into());
        let fallback = optional(args, values, 3).unwrap_or(Value::Text(String::new()));
        if index < 0. || index > LIMIT as f64 {
            return Ok(fallback);
        }
        let index = bounded_count(index, LIMIT);
        let value = if delimiter.is_empty() { (index == 0).then_some(s.as_str()) } else { s.split(&delimiter).nth(index) };
        Ok(value.map(|v| Value::Text(v.into())).unwrap_or(fallback))
    };
    match f() {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    }
}
/// LOOKUP searches whole delimited items without case sensitivity; a delimiter in the key fails.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/lookup-function
fn lookup(args: &[Expr], values: &[Value]) -> Value {
    let f = || -> Result<f64, Error> {
        if !(2..=3).contains(&args.len()) {
            return Err(Error::Value);
        }
        let key = string(values, 0)?;
        let s = string(values, 1)?;
        let delimiter = optional(args, values, 2).map(|v| text(&v)).transpose()?.unwrap_or_else(|| ";".into());
        if delimiter.is_empty() {
            return Ok(if key.to_lowercase() == s.to_lowercase() { 0. } else { -1. });
        }
        if key.contains(&delimiter) {
            return Ok(-1.);
        }
        let key = key.to_lowercase();
        Ok(s.split(&delimiter).position(|v| v.to_lowercase() == key).map(|n| n as f64).unwrap_or(-1.))
    };
    Value::from(f())
}

/// Unit provenance comes from explicit literal units and the host's reference metadata.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/formatex-function
fn expression_unit(expr: &Expr, context: &dyn Context, depth: usize) -> Option<String> {
    if depth > 128 {
        return None;
    }
    match expr {
        Expr::Number(_, unit) => unit.clone().filter(|s| !matches!(s.as_str(), "NUM" | "STR" | "BOOL")),
        Expr::Reference(r) => context.unit(r).filter(|s| !matches!(s.as_str(), "NUM" | "STR" | "BOOL")),
        Expr::Unary(_, e) => expression_unit(e, context, depth + 1),
        Expr::Binary(op, a, b) => {
            let a = expression_unit(a, context, depth + 1);
            let b = expression_unit(b, context, depth + 1);
            match op {
                BinaryOp::Add | BinaryOp::Sub => {
                    if a.as_deref().is_some_and(|s| s.eq_ignore_ascii_case("FT")) && b.as_deref().is_some_and(|s| s.eq_ignore_ascii_case("IN")) {
                        Some("F_I".into())
                    } else if a.as_deref().is_some_and(|s| s.eq_ignore_ascii_case("C")) && b.as_deref().is_some_and(|s| s.eq_ignore_ascii_case("D")) {
                        Some("C_D".into())
                    } else if a.as_deref().is_some_and(|s| s.eq_ignore_ascii_case("P")) && b.as_deref().is_some_and(|s| s.eq_ignore_ascii_case("PT"))
                    {
                        Some("P_PT".into())
                    } else {
                        a.or(b)
                    }
                }
                BinaryOp::Mul => match (a, b) {
                    (Some(a), Some(b)) if a.eq_ignore_ascii_case(&b) => Some(format!("{a}^2")),
                    (a, None) => a,
                    (None, b) => b,
                    _ => None,
                },
                BinaryOp::Div => match (a, b) {
                    (a, None) => a,
                    _ => None,
                },
                _ => None,
            }
        }
        Expr::Call { name, args } if matches!(name.as_str(), "GUARD" | "ABS" | "ROUND" | "TRUNC" | "SETATREFEXPR") => {
            args.first().and_then(|e| expression_unit(e, context, depth + 1))
        }
        _ => None,
    }
}
fn clean_unit(s: &str) -> String {
    s.trim().trim_matches(['[', ']']).split(',').next().unwrap_or("").trim().to_ascii_uppercase()
}
/// FORMATEX unit strings accept localized display aliases; literal formula units
/// retain their angular apostrophe/quote meaning.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/formatex-function
fn format_unit(s: &str) -> Result<String, Error> {
    let unit = clean_unit(s);
    if unit == "°" {
        return Ok("DEG".into());
    }
    if unit == "'" {
        return Ok("FT".into());
    }
    if unit == "\"" {
        return Ok("IN".into());
    }
    if let Some(base) = unit.strip_prefix("SQ.").or_else(|| unit.strip_prefix("SQ ")) {
        let base = base.trim().trim_end_matches('.');
        if !matches!(base, "IN" | "FT" | "YD" | "MI" | "MM" | "CM" | "M" | "KM") {
            return Err(Error::Value);
        }
    }
    if unit.starts_with("P.^") || unit.starts_with("C.^") || unit.starts_with("D.^") {
        return Err(Error::Value);
    }
    units::factor(&unit).map_err(|_| Error::Value)?;
    Ok(unit)
}
/// FORMAT preserves expression units; FORMATEX scales untyped expressions using srcUnit.
/// Every language ID formats with the en-US picture rules: text pictures do not depend on the
/// locale, and locale-specific number and date pictures are an approximation recorded in
/// docs/gaps.md (#90). Non-Gregorian calendars are explicit unsupported values.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/format-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/formatex-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-format-pictures
fn format_value(name: &str, args: &[Expr], values: &[Value], context: &dyn Context) -> Result<String, Error> {
    if (name == "FORMAT" && args.len() != 2) || (name == "FORMATEX" && !(2..=6).contains(&args.len())) {
        return Err(Error::Value);
    }
    let value = get(values, 0)?;
    if let Some(e) = value.error() {
        return Err(e);
    }
    let picture = string(values, 1)?;
    // visFmtNumGenNoUnits: empty Shape Data formats select FIELDPICTURE(0).
    // PCRTSK_M's text callout witnesses general text without a unit suffix.
    // https://learn.microsoft.com/en-us/office/vba/api/visio.visfieldformats
    let general = picture == "esc(0)";
    // Numbered field formats are pictures too. ANNOT_U master26 shape8 uses
    // FIELDPICTURE(8), three decimal places without a unit suffix.
    // https://learn.microsoft.com/en-us/office/vba/api/visio.visfieldformats
    let picture = if let Some(code) = picture.strip_prefix("esc(").and_then(|s| s.strip_suffix(')')).filter(|_| !general) {
        match code.parse::<u32>() {
            Ok(code @ 2..=9) => {
                let places = (code - 2) / 2;
                let decimals = if places == 0 { String::new() } else { format!(".{}", "0".repeat(places as usize)) };
                format!("0{decimals}{}", if code % 2 == 1 { " u" } else { "" })
            }
            _ => return Err(Error::Value),
        }
    } else {
        picture
    };
    if picture.len() > 4096 {
        return Err(Error::Limit);
    }
    if let Some(language) = optional(args, values, 4) {
        // A language ID is accepted as given (LangID cells resolve to the numeric LCID); the
        // en-GB (2057) template behind #90 formats its "@ " and value pictures like en-US.
        if let Some(e) = language.error() {
            return Err(e);
        }
        if !matches!(language.numeric(), Ok(id) if id.is_finite() && id >= 0.) {
            return Err(Error::Value);
        }
    }
    if let Some(calendar) = optional(args, values, 5) {
        let calendar = calendar.numeric()?;
        if calendar != 0. && calendar != 1. {
            return Err(Error::Unsupported(format!("format calendar {calendar}")));
        }
    }
    if let Value::Text(s) = value {
        if s.len() > LIMIT {
            return Err(Error::Limit);
        }
        return if general { Ok(s.clone()) } else { format_text(s, &picture) };
    }
    let n = value.numeric()?;
    let source = args.first().and_then(|e| expression_unit(e, context, 0));
    let src = optional(args, values, 2).map(|v| text(&v).and_then(|s| format_unit(&s))).transpose()?;
    let dst = optional(args, values, 3).map(|v| text(&v).and_then(|s| format_unit(&s))).transpose()?;
    let internal = if source.is_none() { n * units::factor(src.as_deref().unwrap_or(""))? } else { n };
    let mut unit = clean_unit(dst.as_deref().or(source.as_deref()).or(src.as_deref()).unwrap_or(""));
    // An explicit base-unit target keeps the expression's area exponent.
    // Native controls t008..t010: square inches to "mm" displays square millimeters.
    let dimensional_source = clean_unit(source.as_deref().or(src.as_deref()).unwrap_or(""));
    let area = |s: &str| {
        s.starts_with("SQ ") || s.starts_with("SQ.") || s.ends_with("^2") || matches!(s, "AC" | "ACRE" | "ACRES" | "HA" | "HECTARE" | "HECTARES")
    };
    if area(&dimensional_source) && !area(&unit) && dst.is_some() {
        unit.push_str("^2");
    }
    // Prefix names are observed numeric-picture unit overrides, distinct from
    // formula literal suffixes and FORMATEX source scaling.
    // field/format-pictures-2026-11-02/native-b and native-b-controls.
    let picture = if let Some(rest) = picture.strip_prefix("<,") {
        let (family, tail) = rest.split_once('>').ok_or(Error::Value)?;
        if tail.starts_with("<,") {
            return Err(Error::Value);
        }
        match clean_unit(family).as_str() {
            "FEET/INCH" | "PICAPOINTS" | "CICERO/DIDOT" | "IN_F" | "MI_F" | "FT" | "IN" => unit = clean_unit(family),
            "°" => unit = "DEG".into(),
            // Native treats this literal prefix as the unchanged input unit.
            "F_I" => {}
            _ => return Err(Error::Value),
        }
        tail
    } else {
        picture.as_str()
    };
    let output = internal / units::factor(&unit)?;
    if !output.is_finite() {
        return Err(Error::Num);
    }
    if general {
        return Ok(Value::number(output).text());
    }
    if date_picture(picture) {
        return format_date(internal, picture);
    }
    if matches!(unit.as_str(), "°" | "'" | "\"") {
        let degrees = internal.abs().to_degrees();
        let whole = degrees.floor();
        let minutes = ((degrees - whole) * 60.).floor();
        let seconds = ((degrees - whole) * 60. - minutes) * 60.;
        let mut pieces = Vec::new();
        for (value, label) in [(whole, "°"), (minutes, "'"), (seconds, "\"")] {
            if value != 0. || picture.starts_with('0') {
                pieces.push(format_numeric(value, picture, label)?);
            }
        }
        return Ok(format!("{}{}", if internal < 0. { "-" } else { "" }, pieces.join(" ")));
    }
    if matches!(unit.as_str(), "C_D" | "CICERO/DIDOT" | "P_PT" | "PICAPOINTS" | "F_I" | "FEET/INCH") {
        return format_compound(internal, picture, &unit);
    }
    format_numeric(output, picture, &unit)
}
/// Multipart display applies the numeric picture to the smallest subunit, then
/// carries its rounded value before deciding which zero components to omit.
/// Native evidence: field/format-pictures-2026-11-02/native-b/results.json.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-format-pictures
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-units-of-measure-visio-shapesheet-reference
fn format_compound(internal: f64, picture: &str, unit: &str) -> Result<String, Error> {
    let tokens = picture_tokens(picture)?;
    let start = tokens.iter().position(|p| matches!(p, Picture::Code('0' | '#'))).ok_or(Error::Value)?;
    let mut end = start;
    let mut number = String::new();
    while let Some(Picture::Code(c)) = tokens.get(end) {
        match c {
            '0'..='9' | '#' | '.' | ',' | '/' => number.push(*c),
            'e' | 'E' if matches!(tokens.get(end + 1), Some(Picture::Code('+' | '-'))) => number.push(*c),
            '+' | '-' if number.ends_with(['e', 'E']) => number.push(*c),
            ' ' if matches!(tokens.get(end + 1), Some(Picture::Code('0' | '#'))) => number.push(*c),
            _ => break,
        }
        end += 1;
    }
    let encode = |slice: &[Picture]| -> String {
        let mut out = String::new();
        for token in slice {
            match token {
                Picture::Code(c) => out.push(*c),
                Picture::Literal(s) => {
                    for c in s.chars() {
                        out.push('\\');
                        out.push(c);
                    }
                }
            }
        }
        out
    };
    let label_start = tokens.get(end..).ok_or(Error::Value)?.iter().position(|p| matches!(p, Picture::Code('u' | 'U'))).map(|i| i + end);
    let mut label_end = label_start.unwrap_or(end);
    let mut label_space = String::new();
    let mut count = 0;
    let mut upper = false;
    if let Some(at) = label_start {
        upper = matches!(tokens.get(at), Some(Picture::Code('U')));
        let mut before = at;
        while before > end && matches!(tokens.get(before - 1), Some(Picture::Code(c)) if c.is_whitespace()) {
            before -= 1;
        }
        label_space = encode(tokens.get(before..at).ok_or(Error::Value)?);
        while tokens.get(label_end).is_some_and(|p| matches!(p, Picture::Code('u' | 'U'))) {
            label_end += 1;
            count += 1;
        }
    }
    let typography = matches!(unit, "C_D" | "CICERO/DIDOT" | "P_PT" | "PICAPOINTS");
    let (major_unit, minor_unit) = match unit {
        "C_D" | "CICERO/DIDOT" => ("C", "D"),
        "P_PT" | "PICAPOINTS" => ("P", "PT"),
        _ => ("FT", "IN"),
    };
    let smallest = internal.abs() / units::factor(minor_unit)?;
    if !smallest.is_finite() {
        return Err(Error::Num);
    }
    let rounded_text = numeric_picture(smallest, &number)?.replace(',', "");
    let rounded = if rounded_text.trim().is_empty() { 0. } else { units::numeric_text(&rounded_text)? };
    let major = (rounded / 12.).floor();
    let minor = rounded - major * 12.;
    if !major.is_finite() || !minor.is_finite() {
        return Err(Error::Num);
    }
    let required = number.starts_with('0');
    let integer = number.split(['.', ' ']).next().unwrap_or("0");
    let mut pieces = Vec::new();
    for (value, unit, numeric, last) in [(major, major_unit, integer, false), (minor, minor_unit, number.as_str(), true)] {
        if required || value != 0. || (rounded == 0. && last == (internal != 0.)) {
            let mut piece = numeric_picture(value, numeric)?;
            if count != 0 && !(typography && last && count == 1) {
                piece.push_str(&label_space);
                piece.push_str(&label(unit, count, upper));
            }
            pieces.push(piece);
        }
    }
    let join = if typography && count == 1 && label_space.is_empty() { "" } else { " " };
    let prefix = format_numeric(0., &encode(tokens.get(..start).ok_or(Error::Value)?), unit)?;
    let suffix = format_numeric(0., &encode(tokens.get(label_end..).ok_or(Error::Value)?), unit)?;
    let result = format!("{prefix}{}{}{suffix}", if internal < 0. { "-" } else { "" }, pieces.join(join));
    if result.len() > LIMIT {
        return Err(Error::Limit);
    }
    Ok(result)
}

#[derive(Clone, Debug)]
enum Picture {
    Literal(String),
    Code(char),
}
fn picture_tokens(s: &str) -> Result<Vec<Picture>, Error> {
    let mut out = vec![];
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(c) = chars.next() {
                    out.push(Picture::Literal(c.to_string()));
                }
            }
            '\'' | '"' => {
                let mut literal = String::new();
                for ch in chars.by_ref() {
                    if ch == c {
                        break;
                    }
                    literal.push(ch);
                }
                out.push(Picture::Literal(literal));
            }
            _ => out.push(Picture::Code(c)),
        }
    }
    Ok(out)
}
fn format_text(value: &str, picture: &str) -> Result<String, Error> {
    let tokens = picture_tokens(picture)?;
    let mut out = String::new();
    let mut i = 0;
    while let Some(token) = tokens.get(i) {
        match token {
            Picture::Literal(s) => out.push_str(s),
            Picture::Code('@') => {
                let change = match tokens.get(i + 1) {
                    Some(Picture::Code('+')) => {
                        i += 1;
                        1
                    }
                    Some(Picture::Code('-')) => {
                        i += 1;
                        -1
                    }
                    _ => 0,
                };
                match change {
                    1 => out.push_str(&value.to_uppercase()),
                    -1 => out.push_str(&value.to_lowercase()),
                    _ => out.push_str(value),
                }
            }
            Picture::Code(c) if c.is_whitespace() => out.push(*c),
            _ => return Err(Error::Value),
        }
        if out.len() > LIMIT {
            return Err(Error::Limit);
        }
        i += 1;
    }
    Ok(out)
}
/// Display labels witnessed in field/format-pictures-2026-11-02.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-format-pictures
fn label(unit: &str, count: usize, upper: bool) -> String {
    let unit = unit.trim_end_matches('.');
    let area = unit.strip_prefix("SQ.").or_else(|| unit.strip_prefix("SQ ")).map(str::trim).or_else(|| unit.strip_suffix("^2"));
    if let Some(base) = area {
        let base = base.trim().trim_end_matches('.');
        let special = matches!(base, "IN" | "FT" | "YD" | "MI" | "MM" | "CM" | "M" | "KM");
        let value = if special && count == 2 {
            format!("sq. {}", label(base, 1, false))
        } else if special && count >= 3 {
            format!("sq {}", label(base, 3, false))
        } else {
            format!("{}^2", label(base, count, false))
        };
        return if upper { value.to_uppercase() } else { value };
    }
    let (short, full, universal) = match unit {
        "IN" | "IN_F" => ("in.", "inches", "in"),
        "FT" => ("ft.", "feet", "ft"),
        "MM" => ("mm.", "millimeters", "mm"),
        "CM" => ("cm.", "centimeters", "cm"),
        "M" => ("m.", "meters", "m"),
        "KM" => ("km.", "kilometers", "km"),
        "YD" => ("yd.", "yards", "yd"),
        "MI" | "MI_F" => ("mi.", "miles", "mi"),
        "NM" => ("nm.", "nm.", "nm"),
        "PT" => ("pt.", "points", "pt"),
        "P" => ("p", "picas", "p"),
        "C" => ("c", "ciceros", "c"),
        "D" => ("d", "didots", "d"),
        "DEG" | "AD" => ("deg.", "degrees", "deg"),
        "°" => ("°", "degrees", "°"),
        "'" => ("'", "minutes", "'"),
        "\"" => ("\"", "seconds", "\""),
        "RAD" => ("rad.", "radians", "rad"),
        "EH" => ("eh.", "ehour", "eh"),
        "EM" => ("em.", "eminute", "em"),
        "ES" => ("es.", "esecond", "es"),
        "ED" => ("ed.", "eday", "ed"),
        "EW" => ("ew.", "eweek", "ew"),
        "AC" | "ACRE" | "ACRES" => ("acres", "acres", "acre"),
        "HA" | "HECTARE" | "HECTARES" => ("ha.", "hectares", "ha"),
        "DL" | "DP" | "DT" | "DA" | "DE" | "NUM" | "" => ("", "", ""),
        _ => (unit, unit, unit),
    };
    let value = match count {
        1 => short,
        2 => full,
        _ => universal,
    };
    if upper { value.to_uppercase() } else { value.to_lowercase() }
}
/// The numeric run ends before unit codes or literal suffixes. A digit in u^2
/// is a code, not a second numeric placeholder.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-format-pictures
fn format_numeric(value: f64, picture: &str, unit: &str) -> Result<String, Error> {
    let tokens = picture_tokens(picture)?;
    let mut out = String::new();
    let mut i = 0;
    let mut formatted = false;
    let text_number = tokens.iter().any(|p| matches!(p, Picture::Code('@')));
    while let Some(token) = tokens.get(i) {
        if !formatted && matches!(token, Picture::Code('0' | '#')) {
            let mut numeric = String::new();
            while let Some(Picture::Code(c)) = tokens.get(i) {
                match c {
                    '0'..='9' | '#' | '.' | ',' | '/' => numeric.push(*c),
                    'e' | 'E' if matches!(tokens.get(i + 1), Some(Picture::Code('+' | '-'))) => numeric.push(*c),
                    '+' | '-' if numeric.ends_with(['e', 'E']) => numeric.push(*c),
                    ' ' if matches!(tokens.get(i + 1), Some(Picture::Code('0' | '#'))) => numeric.push(*c),
                    _ => break,
                }
                i += 1;
            }
            if !text_number {
                out.push_str(&numeric_picture(value, &numeric)?);
            }
            formatted = true;
            continue;
        }
        match token {
            Picture::Literal(s) => out.push_str(s),
            Picture::Code('@') => out.push_str(&format_numeric(value, if unit.is_empty() { "0.0000" } else { "0.0000 u" }, unit)?),
            Picture::Code('/') if unit.is_empty() => {
                let fraction = fraction_picture(value, "0/#")?;
                let numerator = fraction.split('/').next().unwrap_or("");
                out.clear();
                out.push_str(numerator);
                out.push_str(" /");
            }
            Picture::Code('u' | 'U') => {
                let upper = matches!(token, Picture::Code('U'));
                let mut count = 1;
                while tokens.get(i + count).is_some_and(|p| matches!(p, Picture::Code('u' | 'U'))) {
                    count += 1;
                }
                out.push_str(&label(unit, count, upper));
                i += count - 1;
            }
            Picture::Code(c) if c.is_whitespace() => out.push(*c),
            Picture::Code('.') if unit.is_empty() && i > 0 && matches!(tokens.get(i - 1), Some(Picture::Code(c)) if c.is_ascii_alphabetic()) => {
                out.push('.')
            }
            Picture::Code('$') if unit.is_empty() => out.push('$'),
            Picture::Code('e' | 'E') if unit.is_empty() => return Err(Error::Value),
            _ => {}
        }
        if out.len() > LIMIT {
            return Err(Error::Limit);
        }
        i += 1;
    }
    Ok(out)
}
fn numeric_picture(value: f64, picture: &str) -> Result<String, Error> {
    if !value.is_finite() {
        return Err(Error::Num);
    }
    if picture.contains(' ') && !picture.contains('/') {
        if picture.len() > 256 || !picture.chars().all(|c| matches!(c, '0' | '#' | ' ')) {
            return Err(Error::Value);
        }
        let number = format!("{:.0}", value.abs().round());
        let mut digits = number.chars().rev();
        let mut out = String::new();
        for c in picture.chars().rev() {
            match c {
                '0' | '#' => {
                    if let Some(d) = digits.next() {
                        out.push(d);
                    } else if c == '0' {
                        out.push('0');
                    }
                }
                _ => out.push(c),
            }
        }
        out.extend(digits);
        if value < 0. {
            out.push('-');
        }
        return Ok(out.chars().rev().collect());
    }
    if picture.contains('/') {
        return fraction_picture(value, picture);
    }
    if let Some(at) = picture.find(['E', 'e']) {
        let before = picture.get(..at).ok_or(Error::Value)?;
        let after = picture.get(at + 1..).ok_or(Error::Value)?;
        let sign = after.starts_with('+');
        let after = after.trim_start_matches(['+', '-']);
        if after.is_empty() || !after.chars().all(|c| matches!(c, '0' | '#')) {
            return Err(Error::Value);
        }
        let places = before.split('.').next().unwrap_or("").chars().filter(|c| matches!(c, '0' | '#')).count();
        let exponent = if value == 0. { 0. } else { value.abs().log10().floor() - (places.max(1) - 1) as f64 };
        if !(-308. ..=308.).contains(&exponent) {
            return Err(Error::Num);
        }
        let mantissa = value / 10f64.powf(exponent);
        let e = exponent as i32;
        let digits = after.chars().filter(|c| *c == '0').count().max(1);
        return Ok(format!(
            "{}{}{}{:0width$}",
            numeric_picture(mantissa, before)?,
            picture.get(at..at + 1).ok_or(Error::Value)?,
            if e < 0 {
                "-"
            } else if sign {
                "+"
            } else {
                ""
            },
            e.unsigned_abs(),
            width = digits
        ));
    }
    let mut parts = picture.split('.');
    let integer = parts.next().unwrap_or("");
    let fraction = parts.next().unwrap_or("");
    if parts.next().is_some() || !integer.chars().all(|c| matches!(c, '0' | '#' | ',')) || !fraction.chars().all(|c| matches!(c, '0' | '#')) {
        return Err(Error::Value);
    }
    if fraction.len() > 64 || integer.len() > 256 {
        return Err(Error::Limit);
    }
    let decimal_places = fraction.len();
    let scale = 10f64.powi(decimal_places as i32);
    let rounded = if value.abs() <= f64::MAX / scale { (value * scale).round() / scale } else { value };
    let absolute = format!("{:.*}", decimal_places, rounded.abs());
    let (whole, frac) = absolute.split_once('.').unwrap_or((&absolute, ""));
    let required = integer.chars().filter(|c| *c == '0').count();
    let mut whole = if whole == "0" && required == 0 { String::new() } else { format!("{whole:0>width$}", width = required) };
    if integer.contains(',') {
        let mut grouped = String::new();
        for (i, c) in whole.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                grouped.push(',')
            }
            grouped.push(c)
        }
        whole = grouped.chars().rev().collect();
    }
    let min_fraction = fraction.rfind('0').map(|i| i + 1).unwrap_or(0);
    let mut frac = frac.to_string();
    while frac.len() > min_fraction && frac.ends_with('0') {
        frac.pop();
    }
    let mut out = format!("{}{}", if rounded < 0. { "-" } else { "" }, whole);
    if !frac.is_empty() {
        out.push('.');
        out.push_str(&frac)
    }
    Ok(out)
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.max(1)
}
/// A denominator made only of placeholders selects the closest fraction with
/// at most that many denominator digits. A # before a fixed denominator reduces it.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-format-pictures
fn fraction_picture(value: f64, picture: &str) -> Result<String, Error> {
    if !value.is_finite() {
        return Err(Error::Num);
    }
    let (left, right) = picture.split_once('/').ok_or(Error::Value)?;
    let simplify = right.starts_with('#');
    let digits = right.trim_start_matches('#');
    let variable = digits.is_empty();
    let maximum = if variable {
        let places = u32::try_from(right.len()).map_err(|_| Error::Limit)?;
        10u64.checked_pow(places).and_then(|n| n.checked_sub(1)).ok_or(Error::Limit)?
    } else {
        digits.parse::<u64>().map_err(|_| Error::Value)?.max(1)
    };
    if maximum == 0 || maximum > 65536 || !left.chars().all(|c| matches!(c, '0' | '#' | ' ')) {
        return Err(Error::Value);
    }
    let mixed = left.contains(' ');
    let absolute = value.abs();
    if absolute > 1e12 {
        return Err(Error::Limit);
    }
    let mut whole = if mixed { absolute.floor() as u64 } else { 0 };
    let part = if mixed { absolute - whole as f64 } else { absolute };
    // Both factors are bounded above, so conversion to u64 is finite and in range.
    let mut numerator = (part * maximum as f64).round() as u64;
    let mut denominator = maximum;
    if variable {
        let mut distance = f64::INFINITY;
        for candidate in 1..=maximum {
            let n = (part * candidate as f64).round() as u64;
            let error = (part - n as f64 / candidate as f64).abs();
            if error < distance {
                numerator = n;
                denominator = candidate;
                distance = error;
            }
        }
    }
    if mixed && numerator >= denominator {
        whole = whole.checked_add(numerator / denominator).ok_or(Error::Limit)?;
        numerator %= denominator;
    }
    if simplify {
        let factor = gcd(numerator, denominator);
        numerator /= factor;
        denominator /= factor;
    }
    let mut out = String::new();
    if mixed && (whole > 0 || left.starts_with('0')) {
        out.push_str(&whole.to_string());
    }
    if numerator > 0 || (!mixed && left.contains('0')) {
        if mixed {
            out.push(' ');
        }
        if denominator == 1 {
            out.push_str(&numerator.to_string());
        } else {
            out.push_str(&format!("{numerator}/{denominator}"));
        }
    }
    if value < 0. && (whole != 0 || numerator != 0) {
        out.insert(usize::from(out.starts_with(' ')), '-');
    }
    Ok(out)
}
fn date_picture(picture: &str) -> bool {
    picture.starts_with("{{")
        || picture.chars().all(|c| matches!(c, 'c' | 'C' | 'd' | 'M' | 'y' | 'w' | 'h' | 'H' | 'm' | 's' | 't' | '/' | ':' | ' ' | ',' | '-' | '.'))
            && picture.chars().any(|c| c.is_ascii_alphabetic())
}
/// Serial dates use 1899-12-30; Gregorian arithmetic is bounded to years 1 through 9999.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-date-time-and-duration-values
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-format-pictures
fn format_date(serial: f64, picture: &str) -> Result<String, Error> {
    if !(-693593. ..=2958465.99999).contains(&serial) {
        return Err(Error::Value);
    }
    // The deterministic default locale is en-US (1033), already checked by FORMAT.
    let picture = if picture == "ddddd" { "M/d/yyyy" } else { picture };
    let office = picture.starts_with("{{");
    let picture = if office { picture.strip_prefix("{{").and_then(|s| s.strip_suffix("}}")).ok_or(Error::Value)? } else { picture };
    let tokens = picture_tokens(picture)?;
    let mut days = serial.floor() as i64 - 2;
    let mut year = 1900i64;
    while days < 0 {
        year -= 1;
        days += year_days(year)
    }
    while days >= year_days(year) {
        days -= year_days(year);
        year += 1;
    }
    let month_days = [31, if year_days(year) == 366 { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 0usize;
    while let Some(count) = month_days.get(month) {
        if days < *count {
            break;
        }
        days -= *count;
        month += 1;
    }
    let day = days + 1;
    let weekday = (serial.floor() as i64 + 6).rem_euclid(7) as usize;
    let seconds = ((serial - serial.floor()) * 86400.).floor() as i64;
    let hour = seconds / 3600;
    let minute = seconds / 60 % 60;
    let second = seconds % 60;
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let weekdays = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let month_name = months.get(month).ok_or(Error::Value)?;
    let day_name = weekdays.get(weekday).ok_or(Error::Value)?;
    let mut out = String::new();
    let mut i = 0;
    let mut after_hour = false;
    while let Some(token) = tokens.get(i) {
        match token {
            Picture::Literal(s) => out.push_str(s),
            Picture::Code(c) => {
                let mut n = 1;
                while tokens.get(i + n).is_some_and(|t| matches!(t,Picture::Code(x)if x==c)) {
                    n += 1
                }
                let month_token = *c == 'M' || (office && *c == 'm' && !after_hour);
                let part = if month_token {
                    match n {
                        1 => (month + 1).to_string(),
                        2 => format!("{:02}", month + 1),
                        3 => month_name.chars().take(3).collect(),
                        4 => month_name.to_string(),
                        _ => return Err(Error::Unsupported("month picture".into())),
                    }
                } else {
                    match (*c, n) {
                        // General date pictures include the time only when present.
                        // https://learn.microsoft.com/en-us/office/client-developer/visio/about-format-pictures
                        ('c' | 'C', 1) if !office => {
                            let mut general =
                                if *c == 'c' { format!("{}/{day}/{year}", month + 1) } else { format!("{day_name}, {month_name} {day}, {year}") };
                            if seconds != 0 {
                                let clock_hour = if hour % 12 == 0 { 12 } else { hour % 12 };
                                let meridiem = if hour < 12 { "AM" } else { "PM" };
                                general.push_str(&format!(" {clock_hour}:{minute:02}:{second:02} {meridiem}"));
                            }
                            general
                        }
                        ('d' | 'D', 1) => day.to_string(),
                        ('d' | 'D', 2) => format!("{day:02}"),
                        ('d', 3) | ('w', 1) => day_name.chars().take(3).collect(),
                        ('d', 4) | ('w', 2) => day_name.to_string(),
                        ('y', 2) => format!("{:02}", year.rem_euclid(100)),
                        ('y', 4) => format!("{year:04}"),
                        ('H' | 'h', 1 | 2) => {
                            after_hour = true;
                            let h = if *c == 'h' {
                                let h = hour % 12;
                                if h == 0 { 12 } else { h }
                            } else {
                                hour
                            };
                            if n == 2 { format!("{h:02}") } else { h.to_string() }
                        }
                        ('m', 1 | 2) => {
                            if n == 2 {
                                format!("{minute:02}")
                            } else {
                                minute.to_string()
                            }
                        }
                        ('s', 1 | 2) => {
                            if n == 2 {
                                format!("{second:02}")
                            } else {
                                second.to_string()
                            }
                        }
                        ('t', 1) => {
                            if hour < 12 {
                                "A".into()
                            } else {
                                "P".into()
                            }
                        }
                        ('t', 2) => {
                            if hour < 12 {
                                "AM".into()
                            } else {
                                "PM".into()
                            }
                        }
                        ('/' | ':' | ' ' | ',' | '-' | '.', _) => c.to_string().repeat(n),
                        _ => return Err(Error::Unsupported(format!("date picture {picture}"))),
                    }
                };
                out.push_str(&part);
                i += n - 1;
            }
        }
        if out.len() > LIMIT {
            return Err(Error::Limit);
        }
        i += 1;
    }
    Ok(out)
}
fn year_days(year: i64) -> i64 {
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 366 } else { 365 }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Host;
    impl Context for Host {
        fn resolve(&mut self, r: &str) -> Value {
            if r == "Width" { Value::Number(1.) } else { Value::Error(Error::Ref) }
        }
        fn unit(&self, r: &str) -> Option<String> {
            (r == "Width").then(|| "IN".into())
        }
    }
    fn run(s: &str) -> Value {
        let Expr::Call { name, args } = crate::parse(s).unwrap() else { panic!("call required") };
        call(&name, &args, &mut Host, &mut 16384, 0).unwrap()
    }
    #[test]
    fn microsoft_string_and_list_examples() {
        for (s, v) in [
            ("LEFT(\"January 1, 2004\",3)", "Jan"),
            ("RIGHT(\"January 1, 2004\",4)", "2004"),
            ("MID(\"SSN 999-99-9999\",5,11)", "999-99-9999"),
            ("INDEX(3,\"cat;rat;;goat\")", "goat"),
            ("INDEX(54,\";1;2;3;\",,\"ERROR\")", "ERROR"),
        ] {
            assert_eq!(run(s), Value::Text(v.into()), "{s}")
        }
        assert_eq!(run("LEN(\"Seattle, WA\")"), Value::Number(11.));
        for (s, n) in
            [("LOOKUP(\"rat\",\"cat;rat;;goat\")", 1.), ("LOOKUP(\"\",\";cat;rat;;goat\")", 0.), ("LOOKUP(\"t\",\"cat;rat;;goat\",\"a\")", 3.)]
        {
            assert_eq!(run(s), Value::Number(n));
        }
        for (s, b) in [
            ("STRSAME(\"cat\",\"dog\")", false),
            ("STRSAME(\"cat\",\"cat\")", true),
            ("STRSAME(\"cat\",\"CAT\")", false),
            ("STRSAME(\"cat\",\"CAT\",TRUE)", true),
            ("STRSAME(\"cät\",\"CÄT\",TRUE)", true),
        ] {
            assert_eq!(run(s), Value::Bool(b));
        }
        assert_eq!(run("LISTSEP()"), Value::Text(",".into()));
    }
    #[test]
    fn microsoft_find_example_and_hostile_arguments() {
        assert_eq!(run(r#"FIND("2003","January 1, 2003")"#), Value::Number(12.));
        assert_eq!(run(r#"FIND("A","aA",2)"#), Value::Number(2.));
        assert_eq!(run(r#"FIND("ä","éÄ",,TRUE)"#), Value::Number(2.));
        assert_eq!(run(r#"FIND("x","İx",,TRUE)"#), Value::Number(2.));
        assert_eq!(run(r#"FIND("","abc",2)"#), Value::Number(2.));
        assert_eq!(run(r#"FIND("*","a*b")"#), Value::Number(2.));
        for s in [r#"FIND("a","ABC")"#, r#"FIND("x","abc")"#, r#"FIND("","",1)"#, r#"FIND("a","abc",0)"#, r#"FIND("a","abc",1e99)"#, "FIND()"] {
            assert_eq!(run(s), Value::Error(Error::Value), "{s}");
        }
        let args = [Expr::Text("x".into()), Expr::Text("x".repeat(LIMIT + 1))];
        assert_eq!(call("FIND", &args, &mut Host, &mut 100, 0), Some(Value::Error(Error::Limit)));
    }
    #[test]
    fn microsoft_numeric_and_text_picture_examples() {
        for (s, v) in [
            ("FORMAT(1cm/4,\"0.000 u\")", "0.250 cm."),
            ("FORMAT(1cm/4,\"0.00 U\")", "0.25 CM."),
            ("FORMAT(1cm/4,\"0.0 u\")", "0.3 cm."),
            ("FORMATEX(5.5,\"0.00 u\",\"cm\",\"ft\")", "0.18 ft."),
            ("FORMAT(12345.67,\"###.#e+#\")", "123.5e+2"),
            ("FORMAT(12.43,\"# #/4\")", "12 2/4"),
            ("FORMAT(\"Hello\",\"'You entered ('@')'\")", "You entered (Hello)"),
            ("FORMAT(\"Hello\",\"@ @+ @-\")", "Hello HELLO hello"),
            ("FORMAT(2ft 11.33in,\"0.## u\")", "2 ft. 11.33 in."),
        ] {
            assert_eq!(run(s), Value::Text(v.into()), "{s}")
        }
    }
    #[test]
    fn numbered_field_pictures_and_integral_fractions() {
        assert_eq!(run("FORMAT(2in,FIELDPICTURE(8))"), Value::Text("2.000".into()));
        assert_eq!(run("FORMAT(2mm,FIELDPICTURE(7))"), Value::Text("2.00 mm.".into()));
        assert_eq!(run(r#"FORMAT(0,"0/#")"#), Value::Text("0".into()));
        assert_eq!(run(r#"FORMAT(2,"0/#")"#), Value::Text("2".into()));
        assert!(matches!(run(r#"FORMAT(2,"esc(99999999999999999999999)")"#), Value::Error(_)));
    }
    #[test]
    fn general_field_picture_keeps_text_and_validates_numeric_units_and_options() {
        for (formula, expected) in [
            (r#"FORMAT(8,FIELDPICTURE(0))"#, "8"),
            (r#"FORMAT(2.5mm,FIELDPICTURE(0))"#, "2.5"),
            (r#"FORMATEX(1,FIELDPICTURE(0),"in","mm")"#, "25.4"),
            (r#"FORMATEX("007",FIELDPICTURE(0))"#, "007"),
            (r#"FORMATEX("Count",FIELDPICTURE(0))"#, "Count"),
        ] {
            assert_eq!(run(formula), Value::Text(expected.into()), "{formula}");
        }
        for formula in [
            r#"FORMATEX(1,FIELDPICTURE(0),"invalid")"#,
            r#"FORMATEX(1,FIELDPICTURE(0),,,-1)"#,
            r#"FORMATEX(1,FIELDPICTURE(0),,,,999)"#,
            r#"FORMATEX(1/0,FIELDPICTURE(0))"#,
        ] {
            assert!(matches!(run(formula), Value::Error(_)), "{formula}");
        }
        let args = [Expr::Text("x".repeat(LIMIT + 1)), Expr::Text("esc(0)".into())];
        assert_eq!(call("FORMAT", &args, &mut Host, &mut 100, 0), Some(Value::Error(Error::Limit)));
    }
    #[test]
    fn formatting_edges_and_date_serials() {
        assert_eq!(run(r#"FORMAT(2,"ddddd")"#), Value::Text("1/1/1900".into()));
        assert_eq!(run("FORMAT(12345.6,\"#,##0.00\")"), Value::Text("12,345.60".into()));
        assert_eq!(run("FORMAT(2,\"yyyy-MM-dd\")"), Value::Text("1900-01-01".into()));
        assert_eq!(run("FORMATEX(2,\"{{M/d/yyyy}}\")"), Value::Text("1/1/1900".into()));
        assert_eq!(run("FORMAT(2,\"ww\")"), Value::Text("Monday".into()));
        assert_eq!(run("FORMAT(2,\"'Q'#\")"), Value::Text("Q2".into()));
        assert_eq!(run("FORMATEX(1in,\"0.0 u\",\"cm\",\"mm\")"), Value::Text("25.4 mm.".into()));
        assert_eq!(run("FORMATEX(Width,\"0.0 u\",\"cm\",\"mm\")"), Value::Text("25.4 mm.".into()));
    }
    #[test]
    fn general_date_pictures_use_short_or_long_dates_and_optional_time() {
        for (formula, expected) in [
            (r#"FORMAT(2,"c")"#, "1/1/1900"),
            (r#"FORMAT(2,"C")"#, "Monday, January 1, 1900"),
            (r#"FORMAT(2.5,"c")"#, "1/1/1900 12:00:00 PM"),
            (r#"FORMATEX(2.25,"C",,,1033,0)"#, "Monday, January 1, 1900 6:00:00 AM"),
            (r#"FORMAT(2,"'c'#")"#, "c2"),
        ] {
            assert_eq!(run(formula), Value::Text(expected.into()), "{formula}");
        }
        for formula in [r#"FORMAT(3000000,"c")"#, r#"FORMATEX(2,"C",,,-1)"#, r#"FORMAT(2,"cc")"#] {
            assert!(matches!(run(formula), Value::Error(_)), "{formula}");
        }
    }
    #[test]
    fn unicode_empty_items_and_documented_negative_mid() {
        assert_eq!(run("LEFT(\"é文😀\",2)"), Value::Text("é文".into()));
        assert_eq!(run("RIGHT(\"é文😀\")"), Value::Text("😀".into()));
        assert_eq!(run("LEN(\"é文😀\")"), Value::Number(3.));
        assert_eq!(run("MID(\"abcde\",3,-1)"), Value::Text("cde".into()));
        assert_eq!(run("INDEX(2,\"a<>b<><>c\",\"<>\")"), Value::Text(String::new()));
        assert_eq!(run("LOOKUP(\"a;b\",\"a;b\")"), Value::Number(-1.));
    }
    #[test]
    fn hostile_text_arguments_and_pictures() {
        for s in ["LEFT(\"a\",-1)", "MID(\"a\",0,1)", "FORMATEX(1,\"0\",\"invalid\")", "FORMATEX(1,\"{{dddd}}\",,,-1)"] {
            assert!(matches!(run(s), Value::Error(_)), "{s}")
        }
        assert_eq!(run("LEFT(\"abc\",1e99)"), Value::Text("abc".into()));
        assert_eq!(run("INDEX(1e99,\"abc\")"), Value::Text(String::new()));
        let args = [Expr::Text("a".repeat(LIMIT + 1))];
        assert_eq!(call("LEN", &args, &mut Host, &mut 100, 0), Some(Value::Error(Error::Limit)));
    }
    #[test]
    fn char_codes_and_format_language_ids() {
        // CHAR: Unicode scalar values, fractional codes truncated; zero, surrogates and
        // out-of-range codes are errors (#89).
        assert_eq!(run("CHAR(65)"), Value::Text("A".into()));
        assert_eq!(run("CHAR(10)"), Value::Text("\n".into()));
        assert_eq!(run("CHAR(228.9)"), Value::Text("ä".into()));
        assert_eq!(run("CHAR(128512)"), Value::Text("\u{1F600}".into()));
        for s in ["CHAR(0)", "CHAR(-1)", "CHAR(55296)", "CHAR(1114112)", "CHAR()", "CHAR(1,2)", "CHAR(\"a\")"] {
            assert_eq!(run(s), Value::Error(Error::Value), "{s}");
        }
        assert_eq!(crate::formula("\"a\"&CHAR(10)&\"b\"", &mut Host), Value::Text("a\nb".into()));
        // FORMATEX: every language ID formats with the en-US rules (#90); the calendar guard stays.
        for (lang, expected) in [("", "Jane Doe "), ("1033", "Jane Doe "), ("2057", "Jane Doe "), ("1036", "Jane Doe ")] {
            let s = format!("FORMATEX(\"Jane Doe\",\"@ \",,,{lang},)");
            assert_eq!(run(&s), Value::Text(expected.into()), "{s}");
        }
        assert_eq!(run("FORMATEX(1.5,\"0.00\",,,2057,)"), run("FORMATEX(1.5,\"0.00\",,,1033,)"));
        assert_eq!(run("FORMATEX(\"x\",\"@\",,,-1,)"), Value::Error(Error::Value));
        // A non-finite number is #NUM! everywhere in the evaluator (units::number), so a NaN or
        // infinite language ID arrives as that error and propagates like any other error
        // argument (brief 39 S1).
        for id in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let args = [Expr::Text("x".into()), Expr::Text("@".into()), Expr::Missing, Expr::Missing, Expr::Number(id, None)];
            assert_eq!(call("FORMATEX", &args, &mut Host, &mut 100, 0), Some(Value::Error(Error::Num)), "{id}");
        }
        assert_eq!(run("FORMATEX(\"x\",\"@\",,,Nope,)"), Value::Error(Error::Ref));
        assert!(matches!(run("FORMATEX(\"x\",\"@\",,,2057,2)"), Value::Error(Error::Unsupported(_))));
    }
}
