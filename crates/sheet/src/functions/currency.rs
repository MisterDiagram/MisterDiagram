//! Numeric currency evaluation; display symbols and locale parsing need a currency host.
use crate::{Context, Error, Value, eval::eval, parser::Expr};

// Currency identifiers are format constants, not exchange-rate conversions.
// https://learn.microsoft.com/en-us/office/client-developer/visio/about-currency-constants
const CODES: &str = "SYS XXX EUR USD ATS AUD BEF CAD CHF CNY DEM DKK ESP FIM FRF GBP GRD HKD HUF IDR IEP ILS ITL JPY KRW LUF MXN MYR NLG NOK NZD PHP PLZ PTE ROL RUR SEK SGD THB TWD XEU YUN ZAR ARS BMD BOB BRR BSD CLP COP CRC CZK DOP ECS EGP HNL INR JMD JOD KWD MOP NIO PAB PEN PKR PYG SAR SIT SKK SVC TRY TTD UYU VEB VND BRL PLN RUB YUM BYB UAH AFA ALL DZD ADP AOA XCD AMD AWG AZM BHD BDT BBD BYR BZD XOF BTN BAM BWP BND BGL BGN BIF KHR XAF CVE KYD KMF CDF HRK CUP CYP DJF TPE ERN EEK ETB FKP FJD XPF GMD GEL GHC GIP GTQ GNF GWP GYD HTG ISK IRR IQD KZT KES KPW KGS LAK LVL LBP LSL LRD LYD LTL MKD MGF MWK MVR MTL MRO MUR MDL MNT MAD MZM MMK NAD NPR ANG NGN OMR PGK QAR RWF SHP WST STD SCR SLL SBD SOS LKR SDD SRG SZL SYP TJR TJS TZS TOP TND TMM UGX AED UZS VUV YER ZMK ZWD VEF MGA RSD CSD";

/// CY's numeric projection retains the amount, independently of its display currency.
/// Values requiring unverified rounding or locale-specific parsing stay unsupported.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cy-function
pub(super) fn call(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Value {
    let result = (|| {
        if args.len() > 2 {
            return Err(Error::Value);
        }
        let Some(first) = args.first().filter(|arg| !matches!(arg, Expr::Missing)) else {
            return Err(Error::Unsupported("CY omitted amount".into()));
        };
        let value = eval(first, context, budget, depth + 1);
        if let Some(error) = value.error() {
            return Err(error);
        }
        if let Some(currency) = args.get(1).filter(|arg| !matches!(arg, Expr::Missing)) {
            currency_id(&eval(currency, context, budget, depth + 1))?;
        }
        match value {
            Value::Number(number) => number_amount(number),
            Value::Text(text) => decimal_amount(&text),
            _ => Err(Error::Value),
        }
    })();
    Value::from(result)
}

fn currency_id(value: &Value) -> Result<(), Error> {
    if let Some(error) = value.error() {
        return Err(error);
    }
    let valid = match value {
        Value::Number(id) => {
            id.is_finite() && id.fract() == 0. && (*id == 0. || *id == 1. || (10. ..=50.).contains(id) || (56. ..=204.).contains(id))
        }
        Value::Text(code) => code.len() == 3 && CODES.split_ascii_whitespace().any(|known| known.eq_ignore_ascii_case(code)),
        _ => false,
    };
    if valid { Ok(()) } else { Err(Error::Value) }
}

fn number_amount(number: f64) -> Result<f64, Error> {
    let whole = number.abs().floor();
    let fraction = number.fract().abs();
    if !number.is_finite() || whole > 922_337_203_685_477. || whole == 922_337_203_685_477. && fraction > if number < 0. { 0.5808 } else { 0.5807 } {
        return Err(Error::Value);
    }
    let scaled = fraction * 10_000.;
    if (scaled - scaled.round()).abs() > 1e-7 {
        return Err(Error::Unsupported("CY rounding beyond four decimal places".into()));
    }
    Ok(number)
}

// Validate decimal strings exactly against the documented signed 64-bit currency
// range before converting to the evaluator's f64 numeric representation.
fn decimal_amount(text: &str) -> Result<f64, Error> {
    if text.len() > 4096 {
        return Err(Error::Limit);
    }
    let text = text.trim();
    if text.chars().any(|c| matches!(c, '$' | '€' | '£' | '¥' | ',' | '(' | ')')) {
        return Err(Error::Unsupported("CY locale-formatted amount".into()));
    }
    let negative = text.starts_with('-');
    let unsigned = text.strip_prefix(['-', '+']).unwrap_or(text);
    let (whole, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    if whole.is_empty() && fraction.is_empty() || !whole.bytes().chain(fraction.bytes()).all(|b| b.is_ascii_digit()) {
        return Err(Error::Value);
    }
    if fraction.len() > 4 {
        return Err(Error::Unsupported("CY rounding beyond four decimal places".into()));
    }
    let digits = whole.bytes().chain(fraction.bytes()).chain(std::iter::repeat_n(b'0', 4 - fraction.len()));
    let mut scaled = 0u64;
    for digit in digits {
        scaled = scaled.checked_mul(10).and_then(|n| n.checked_add(u64::from(digit - b'0'))).ok_or(Error::Value)?;
    }
    let bound = if negative { 1u64 << 63 } else { i64::MAX as u64 };
    if scaled > bound {
        return Err(Error::Value);
    }
    text.parse::<f64>().map_err(|_| Error::Value)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Host;
    impl Context for Host {
        fn resolve(&mut self, _: &str) -> Value {
            Value::Error(Error::Ref)
        }
    }
    fn run(formula: &str) -> Value {
        crate::formula(formula, &mut Host)
    }
    #[test]
    fn currency_amounts_keep_numeric_value_and_validate_ids() {
        for formula in [r#"CY(17.125,"USD")"#, "CY(17.125,11)", r#"CY("17.125","XXX")"#, "CY(17.125)", "CY(17.125,)"] {
            assert_eq!(run(formula), Value::Number(17.125), "{formula}");
        }
        assert_eq!(run(r#"CY(-2.125,"EUR")*2"#), Value::Number(-4.25));
        for formula in ["CY(1,2)", "CY(1,55)", "CY(1,205)", "CY(1,11.5)", r#"CY(1,"ZZZ")"#, r#"CY("not a number","USD")"#, "CY(1,11,0)"] {
            assert_eq!(run(formula), Value::Error(Error::Value), "{formula}");
        }
        assert_eq!(run("CY(1/0,11)"), Value::Error(Error::Div0));
        assert_eq!(run("CY(1,REF())"), Value::Error(Error::Ref));
    }
    #[test]
    fn currency_range_and_unsupported_precision_are_explicit() {
        for amount in ["922337203685477.5807", "-922337203685477.5808"] {
            assert!(matches!(run(&format!("CY(\"{amount}\",11)")), Value::Number(_)));
        }
        for amount in ["922337203685477.5808", "-922337203685477.5809", "99999999999999999999999999999999"] {
            assert_eq!(run(&format!("CY(\"{amount}\",11)")), Value::Error(Error::Value));
        }
        assert_eq!(run("CY(1e15,11)"), Value::Error(Error::Value));
        assert_eq!(number_amount(f64::NAN), Err(Error::Value));
        assert_eq!(decimal_amount(&"1".repeat(4097)), Err(Error::Limit));
        for formula in ["CY()", "CY(,11)", "CY(1.123456,11)", r#"CY("$1.00",11)"#] {
            assert!(matches!(run(formula), Value::Error(Error::Unsupported(_))), "{formula}");
        }
    }
}
