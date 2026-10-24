//! Query parsing: split a typed line into amount, source and optional target.
//! No regex engine; the grammar is small enough to walk by hand.

use crate::num;

pub struct Query {
    pub amount: f64,
    pub from: String,
    pub target: Option<String>,
}

const CONNECTORS: &[&str] = &["to", "in", "as", "->", "=>"];

/// Currency symbols are rewritten to ISO codes before tokenizing so `100€` and
/// `100 eur` take the same path.
fn expand_symbols(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 8);
    for ch in input.chars() {
        match ch {
            '€' => out.push_str(" eur "),
            '₪' => out.push_str(" ils "),
            '$' => out.push_str(" usd "),
            '£' => out.push_str(" gbp "),
            '¥' => out.push_str(" jpy "),
            '₹' => out.push_str(" inr "),
            '₽' => out.push_str(" rub "),
            _ => out.push(ch),
        }
    }
    out
}

pub fn parse(input: &str) -> Option<Query> {
    let expanded = expand_symbols(input.trim());
    let trimmed = expanded.trim();
    let (amount, end) = num::split_number(trimmed)?;
    let rest = trimmed[end..].trim();
    if rest.is_empty() {
        return None;
    }

    let words: Vec<&str> = rest.split_whitespace().collect();
    // A connector only splits when something precedes it: `in` is also the symbol for
    // inches, so "5 in to cm" must keep `in` as the source unit.
    if let Some(index) = words.iter().skip(1).position(|w| is_connector(w)) {
        let index = index + 1;
        let from = words[..index].join(" ");
        let target = words[index + 1..].join(" ");
        return Some(Query {
            amount,
            from,
            target: (!target.is_empty()).then_some(target),
        });
    }

    Some(Query { amount, from: rest.to_string(), target: None })
}

fn is_connector(word: &str) -> bool {
    CONNECTORS.iter().any(|c| word.eq_ignore_ascii_case(c))
}

/// Longest leading run of words that the classifier recognises, with the remainder
/// as the target. Handles "100 us gallons" (source is two words) and
/// "100 usd jpy" (source is one word, target the rest).
pub fn split_prefix<T, F>(text: &str, classify: F) -> Option<(T, Option<String>)>
where
    F: Fn(&str) -> Option<T>,
{
    let words: Vec<&str> = text.split_whitespace().collect();
    for take in (1..=words.len()).rev() {
        let candidate = words[..take].join(" ");
        if let Some(kind) = classify(&candidate) {
            let rest = words[take..].join(" ");
            return Some((kind, (!rest.is_empty()).then_some(rest)));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_on_connectors() {
        let q = parse("100 eur to usd").unwrap();
        assert_eq!(q.amount, 100.0);
        assert_eq!(q.from, "eur");
        assert_eq!(q.target.as_deref(), Some("usd"));

        let q = parse("5 km in m").unwrap();
        assert_eq!(q.from, "km");
        assert_eq!(q.target.as_deref(), Some("m"));
    }

    #[test]
    fn keeps_two_word_units_whole() {
        let q = parse("2 miles per hour in kmh").unwrap();
        assert_eq!(q.from, "miles per hour");
        assert_eq!(q.target.as_deref(), Some("kmh"));
    }

    #[test]
    fn expands_currency_symbols() {
        let q = parse("100€ to $").unwrap();
        assert_eq!(q.from, "eur");
        assert_eq!(q.target.as_deref(), Some("usd"));
    }

    #[test]
    fn rejects_input_without_amount() {
        assert!(parse("eur usd").is_none());
        assert!(parse("").is_none());
        assert!(parse("100").is_none());
    }

    #[test]
    fn prefix_split_finds_longest_match() {
        let classify = |s: &str| match s {
            "us gallons" | "kg" | "usd" => Some(s.to_string()),
            _ => None,
        };
        let (from, rest) = split_prefix("us gallons", classify).unwrap();
        assert_eq!(from, "us gallons");
        assert_eq!(rest, None);

        let (from, rest) = split_prefix("usd jpy", classify).unwrap();
        assert_eq!(from, "usd");
        assert_eq!(rest.as_deref(), Some("jpy"));
    }
}
