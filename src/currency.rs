//! Fiat currencies: static name table for offline resolution, rates fetched as units-per-USD.

use crate::currency_data::CURRENCY_NAMES;
use std::collections::HashMap;
use std::time::Duration;

const RATES_URL: &str = "https://open.er-api.com/v6/latest/USD";

const SYMBOLS: &[(&str, &str)] = &[
    ("$", "USD"),
    ("€", "EUR"),
    ("£", "GBP"),
    ("¥", "JPY"),
    ("₪", "ILS"),
    ("₹", "INR"),
    ("₽", "RUB"),
    ("₩", "KRW"),
    ("₺", "TRY"),
    ("₴", "UAH"),
    ("₫", "VND"),
    ("฿", "THB"),
    ("₦", "NGN"),
    ("₱", "PHP"),
    ("R$", "BRL"),
];

pub struct Rates {
    /// Currency code -> units of that currency per one US dollar.
    per_usd: HashMap<String, f64>,
    /// Non-fatal problems worth showing the user (stale cache, partial data).
    pub note: Option<String>,
}

impl Rates {
    pub fn value_in_usd(&self, code: &str) -> Option<f64> {
        self.per_usd
            .get(code)
            .map(|rate| 1.0 / rate)
            .filter(|v| v.is_finite())
    }

    pub fn parse(body: &str) -> Result<(HashMap<String, f64>, Option<String>), String> {
        let json: serde_json::Value =
            serde_json::from_str(body).map_err(|err| format!("unreadable rate data: {err}"))?;
        let rates = json
            .get("rates")
            .and_then(|v| v.as_object())
            .ok_or_else(|| "rate data has no currencies".to_string())?;
        let mut map = HashMap::with_capacity(rates.len());
        for (code, value) in rates {
            if let Some(rate) = value.as_f64() {
                if rate > 0.0 {
                    map.insert(code.to_uppercase(), rate);
                }
            }
        }
        if map.is_empty() {
            return Err("rate data contained no usable values".to_string());
        }
        Ok((map, None))
    }

    pub fn load() -> Result<Rates, String> {
        let cached =
            crate::fetch::get(RATES_URL, "rates_usd.json", Duration::from_secs(12 * 3600))?;
        let (per_usd, _) = Rates::parse(&cached.body)?;
        Ok(Rates {
            per_usd,
            note: cached.stale_note,
        })
    }
}

/// Resolve a typed token to an ISO currency code without touching the network.
pub fn resolve(token: &str) -> Option<&'static str> {
    let token = token.trim();
    if token.is_empty() {
        return None;
    }
    for (symbol, code) in SYMBOLS {
        if token == *symbol || token.eq_ignore_ascii_case(symbol) {
            return Some(code);
        }
    }
    let lowered = token.to_lowercase();
    if lowered.len() == 3 {
        if let Some((code, _)) = CURRENCY_NAMES
            .iter()
            .find(|(code, _)| code.eq_ignore_ascii_case(&lowered))
        {
            return Some(code);
        }
    }
    let mut matches: Vec<&'static str> = Vec::new();
    for (code, name) in CURRENCY_NAMES.iter() {
        let name = name.to_lowercase();
        let plural = format!("{name}s");
        if lowered == name || lowered == plural {
            return Some(code);
        }
        // Partial names must be unambiguous: "canadian dollar" beats "dollar".
        if name.starts_with(&format!("{lowered} ")) || name.split_whitespace().any(|w| w == lowered)
        {
            matches.push(code);
        }
    }
    match matches.len() {
        1 => Some(matches[0]),
        _ => None,
    }
}

pub fn name_of(code: &str) -> String {
    CURRENCY_NAMES
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, name)| (*name).to_string())
        .unwrap_or_else(|| code.to_string())
}

/// Currencies listed when the query names no target, e.g. `100 eur`.
/// Defaults match the operator's working pair; override with CALC_BASE_CURRENCIES.
pub fn default_targets() -> Vec<String> {
    let raw = std::env::var("CALC_BASE_CURRENCIES").unwrap_or_else(|_| "USD,ILS".to_string());
    raw.split(',')
        .map(|s| s.trim().to_uppercase())
        .filter(|s| s.len() == 3)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rates() -> Rates {
        let (per_usd, _) = Rates::parse(
            r#"{"result":"success","rates":{"USD":1,"EUR":0.8,"ILS":3.7,"GBP":0.75}}"#,
        )
        .unwrap();
        Rates {
            per_usd,
            note: None,
        }
    }

    #[test]
    fn resolves_codes_symbols_and_names() {
        assert_eq!(resolve("eur"), Some("EUR"));
        assert_eq!(resolve("EUR"), Some("EUR"));
        assert_eq!(resolve("€"), Some("EUR"));
        assert_eq!(resolve("$"), Some("USD"));
        assert_eq!(resolve("shekel"), Some("ILS"));
        assert_eq!(resolve("euros"), Some("EUR"));
        assert_eq!(resolve("dollar"), None, "ambiguous name must not guess");
        assert_eq!(resolve("kilometers"), None);
    }

    #[test]
    fn parses_rates_and_converts_via_usd() {
        let r = rates();
        let one_eur = r.value_in_usd("EUR").unwrap();
        assert!((one_eur - 1.25).abs() < 1e-9);
        let one_ils = r.value_in_usd("ILS").unwrap();
        assert!((one_ils - 1.0 / 3.7).abs() < 1e-9);
        assert!(r.value_in_usd("XXX").is_none());
    }

    #[test]
    fn rejects_garbage_rate_payloads() {
        assert!(Rates::parse("{}").is_err());
        assert!(Rates::parse("not json").is_err());
        assert!(Rates::parse(r#"{"rates":{}}"#).is_err());
    }
}
