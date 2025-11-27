//! Hermes Math
//!
//! Contains math helpers and a tiny stub for parsing simple dynamic expressions.

use regex::Regex;

/// Parse a very small subset of math expressions: `a * t + b` style linear expressions.
pub fn parse_linear(expr: &str) -> Option<(f64, f64)> {
    // very naive parser for `a * t + b` or `a*t+b`
    let re = Regex::new(r"^\s*([+-]?[0-9\.]+)\s*\*\s*t\s*([+-]\s*[0-9\.]+)?\s*$").ok()?;
    let caps = re.captures(expr)?;
    let a = caps.get(1)?.as_str().replace(" ", "").parse::<f64>().ok()?;
    let b = caps.get(2)
        .map(|m| m.as_str().replace(" ", ""))
        .unwrap_or_else(|| String::from("+0"))
        .parse::<f64>()
        .ok()?;
    Some((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_simple() {
        assert_eq!(parse_linear("2.5 * t + 1"), Some((2.5, 1.0)));
        assert_eq!(parse_linear("-1*t-0.5"), Some((-1.0, -0.5)));
    }
}
