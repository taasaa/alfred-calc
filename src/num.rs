//! Number parsing and display helpers.

/// Parse a leading number from a token such as `100`, `1.5`, `1,234`, `-3`.
/// Returns the value and the byte offset where the number ended.
pub fn split_number(input: &str) -> Option<(f64, usize)> {
    let bytes = input.as_bytes();
    let mut end = 0;
    let mut seen_digit = false;
    for (i, b) in bytes.iter().enumerate() {
        match b {
            b'0'..=b'9' => {
                seen_digit = true;
                end = i + 1;
            }
            b'.' | b',' => end = i + 1,
            b'-' | b'+' if i == 0 => end = i + 1,
            _ => break,
        }
    }
    if !seen_digit {
        return None;
    }
    let raw: String = input[..end].chars().filter(|c| *c != ',').collect();
    let value = raw.parse::<f64>().ok()?;
    Some((value, end))
}

/// Format a number for display: thousands separators, trailing zeros trimmed,
/// scientific notation only when the value is extreme.
pub fn fmt(value: f64) -> String {
    if !value.is_finite() {
        return format!("{}", value);
    }
    if value == 0.0 {
        return "0".to_string();
    }
    let abs = value.abs();
    if abs >= 1e21 || abs < 1e-9 {
        return format!("{:.6e}", value);
    }
    let decimals = if abs >= 1000.0 {
        2
    } else if abs >= 1.0 {
        4
    } else {
        8
    };
    let mut s = format!("{:.*}", decimals, value);
    if s.contains('.') {
        s = s.trim_end_matches('0').trim_end_matches('.').to_string();
    }
    group_thousands(&s)
}

fn group_thousands(s: &str) -> String {
    let (sign, rest) = match s.strip_prefix('-') {
        Some(r) => ("-", r),
        None => ("", s),
    };
    let (int_part, frac) = match rest.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (rest, None),
    };
    let mut out = String::new();
    for (i, ch) in int_part.chars().enumerate() {
        if i > 0 && (int_part.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    match frac {
        Some(f) => format!("{}{}.{}", sign, out, f),
        None => format!("{}{}", sign, out),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_leading_numbers() {
        assert_eq!(split_number("100eur"), Some((100.0, 3)));
        assert_eq!(split_number("1.5 hours"), Some((1.5, 3)));
        assert_eq!(split_number("1,234"), Some((1234.0, 5)));
        assert_eq!(split_number("eur"), None);
    }

    #[test]
    fn formats_without_noise() {
        assert_eq!(fmt(5000.0), "5,000");
        assert_eq!(fmt(1.5), "1.5");
        assert_eq!(fmt(0.000001), "0.000001");
        assert_eq!(fmt(-1234.5), "-1,234.5");
    }
}
