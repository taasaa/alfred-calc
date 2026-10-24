//! alfred-calc: natural-language currency, crypto and unit conversion for Alfred.
//!
//! `alfred-calc "100 eur to usd"` prints Alfred script-filter JSON on stdout.

mod alfred;
mod crypto;
mod currency;
mod currency_data;
mod fetch;
mod num;
mod parse;
mod units;

use alfred::{Item, Output};

const HINT: &str = "Try 100 eur to usd, 2 btc to usd, 5 km in m, 2 tb to gb";

#[derive(Clone)]
enum Kind {
    Unit(units::UnitRef),
    Fiat(String),
    Crypto(&'static str),
}

impl Kind {
    fn label(&self) -> String {
        match self {
            Kind::Unit(unit) => {
                let symbol = unit.symbol();
                let name = unit
                    .unit()
                    .names
                    .iter()
                    .copied()
                    .find(|name| *name != symbol)
                    .unwrap_or(symbol);
                format!("{name} ({})", unit.display())
            }
            Kind::Fiat(code) => format!("{} ({code})", currency::name_of(code)),

            Kind::Crypto(symbol) => {
                format!("{} ({})", crypto::name_of(symbol), symbol.to_uppercase())
            }
        }
    }

    fn symbol(&self) -> String {
        match self {
            Kind::Unit(unit) => unit.display(),
            Kind::Fiat(code) => code.clone(),
            Kind::Crypto(symbol) => symbol.to_uppercase(),
        }
    }
}

/// Units first (local and unambiguous), then crypto symbols, then fiat names.
fn classify(token: &str) -> Option<Kind> {
    if let Some(unit) = units::resolve(token) {
        return Some(Kind::Unit(unit));
    }
    if let Some(symbol) = crypto::resolve(token) {
        return Some(Kind::Crypto(symbol));
    }
    if let Some(code) = currency::resolve(token) {
        return Some(Kind::Fiat(code.to_string()));
    }
    None
}

/// Lazily loaded market data. Every failure is reported, never silently replaced.
#[derive(Default)]
struct Market {
    rates: Option<currency::Rates>,
    prices: Option<crypto::Prices>,
    notes: Vec<String>,
}

impl Market {
    /// Value of one unit of `kind` in US dollars.
    fn usd(&mut self, kind: &Kind) -> Result<f64, String> {
        match kind {
            Kind::Fiat(code) => {
                if self.rates.is_none() {
                    let rates = currency::Rates::load()
                        .map_err(|err| format!("currency rates unavailable: {err}"))?;
                    if let Some(note) = rates.note.clone() {
                        self.notes.push(note);
                    }
                    self.rates = Some(rates);
                }
                let rates = self.rates.as_ref().expect("rates loaded above");
                rates.value_in_usd(code).ok_or_else(|| {
                    format!(
                        "{} ({code}) missing from rate data",
                        currency::name_of(code)
                    )
                })
            }
            Kind::Crypto(symbol) => {
                if self.prices.is_none() {
                    let prices = crypto::Prices::load()
                        .map_err(|err| format!("crypto prices unavailable: {err}"))?;
                    if let Some(note) = prices.note.clone() {
                        self.notes.push(note);
                    }
                    self.prices = Some(prices);
                }
                let prices = self.prices.as_ref().expect("prices loaded above");
                prices.value_in_usd(symbol).ok_or_else(|| {
                    format!(
                        "{} has no price in the market list",
                        crypto::name_of(symbol)
                    )
                })
            }
            Kind::Unit(_) => Err("measurements have no exchange rate".to_string()),
        }
    }
}

pub fn run(query: &str) -> Output {
    let query = query.trim();
    if query.is_empty() {
        return Output::single(Item::info("Type an amount".to_string(), HINT.to_string()));
    }

    let Some(parsed) = parse::parse(query) else {
        return Output::single(Item::info(
            "Start with a number".to_string(),
            HINT.to_string(),
        ));
    };

    let Some((from, mut leftover)) = parse::split_prefix(&parsed.from, classify) else {
        return Output::single(Item::error(
            format!("Unknown unit or currency: {}", parsed.from),
            HINT.to_string(),
        ));
    };
    if parsed.target.is_some() {
        leftover = parsed.target.clone();
    }

    let target = match leftover {
        None => None,
        Some(text) => match parse::split_prefix(&text, classify) {
            Some((kind, _)) => Some(kind),
            None => {
                return Output::single(Item::error(
                    format!("Unknown target: {text}"),
                    HINT.to_string(),
                ))
            }
        },
    };

    match from {
        Kind::Unit(unit) => convert_units(parsed.amount, unit, target),
        other => convert_money(parsed.amount, other, target),
    }
}

fn convert_units(amount: f64, from: units::UnitRef, target: Option<Kind>) -> Output {
    match target {
        Some(Kind::Unit(to)) => {
            if to.category != from.category {
                return Output::single(Item::error(
                    format!(
                        "Cannot convert {} to {}",
                        from.category_name(),
                        to.category_name()
                    ),
                    "Units must share a category".to_string(),
                ));
            }
            match units::convert(amount, from, to) {
                Some(value) => Output::single(result_item(
                    value,
                    &to.display(),
                    &kind_of(from),
                    &kind_of(to),
                )),
                None => Output::single(Item::error(
                    format!("Cannot convert {} to {}", from.symbol(), to.symbol()),
                    "Units must share a category".to_string(),
                )),
            }
        }
        Some(other) => Output::single(Item::error(
            format!("Cannot convert a measurement to {}", other.symbol()),
            "Units and currencies do not mix".to_string(),
        )),
        None => {
            let items: Vec<Item> = units::siblings(from)
                .into_iter()
                .filter(|candidate| *candidate != from)
                .filter_map(|candidate| {
                    let value = units::convert(amount, from, candidate)?;
                    Some(result_item(
                        value,
                        &candidate.display(),
                        &kind_of(from),
                        &kind_of(candidate),
                    ))
                })
                .collect();
            match items.is_empty() {
                true => Output::single(Item::error(
                    format!("No conversions available for {}", from.category_name()),
                    HINT.to_string(),
                )),
                false => Output { items },
            }
        }
    }
}

fn convert_money(amount: f64, from: Kind, target: Option<Kind>) -> Output {
    let mut market = Market::default();
    let from_usd = match market.usd(&from) {
        Ok(value) => value,
        Err(err) => return Output::single(Item::error("Conversion unavailable".to_string(), err)),
    };

    let targets = match target {
        Some(Kind::Unit(unit)) => {
            return Output::single(Item::error(
                format!("Cannot convert {} to {}", from.symbol(), unit.symbol()),
                "Currencies and measurements do not mix".to_string(),
            ))
        }
        Some(kind) => vec![kind],
        None => currency::default_targets()
            .into_iter()
            .filter(|code| as_fiat(&from) != Some(code.as_str()))
            .map(Kind::Fiat)
            .collect(),
    };

    let mut items = Vec::new();
    let mut failures = Vec::new();
    for to in targets {
        match market.usd(&to) {
            Ok(to_usd) => {
                let value = amount * from_usd / to_usd;
                items.push(result_item(value, &to.symbol(), &from, &to));
            }
            Err(err) => failures.push(Item::error(format!("No rate for {}", to.symbol()), err)),
        }
    }

    if items.is_empty() {
        return match failures.into_iter().next() {
            Some(item) => Output::single(item),
            None => Output::single(Item::error(
                "Conversion unavailable".to_string(),
                HINT.to_string(),
            )),
        };
    }

    let suffix = market.notes.join("; ");
    if !suffix.is_empty() {
        for item in items.iter_mut() {
            if let Some(subtitle) = item.subtitle.as_mut() {
                subtitle.push_str(&format!("  ·  {suffix}"));
            }
        }
        items.push(Item::info("Market data warning".to_string(), suffix));
    }
    Output { items }
}

fn as_fiat(kind: &Kind) -> Option<&str> {
    match kind {
        Kind::Fiat(code) => Some(code.as_str()),
        _ => None,
    }
}

fn kind_of(unit: units::UnitRef) -> Kind {
    Kind::Unit(unit)
}

fn result_item(value: f64, unit_symbol: &str, from: &Kind, to: &Kind) -> Item {
    let title = format!("{} {}", num::fmt(value), unit_symbol);
    let subtitle = format!("{} → {}", from.label(), to.label());
    Item::result(title, subtitle, num::fmt(value))
}

fn main() {
    let query = std::env::args().nth(1).unwrap_or_default();
    let output = run(&query);
    match serde_json::to_string(&output) {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("alfred-calc: could not encode output: {err}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first(query: &str) -> Item {
        let output = run(query);
        output.items.into_iter().next().expect("at least one item")
    }

    #[test]
    fn converts_units_end_to_end() {
        let item = first("5 km in m");
        assert_eq!(item.title, "5,000 m");
        assert!(item.valid);

        let item = first("2 tb to gb");
        assert_eq!(item.title, "2,000 GB");

        let item = first("100 c to f");
        assert_eq!(item.title, "212 °F");
    }

    #[test]
    fn lists_siblings_without_target() {
        let output = run("5 km");
        assert!(output.items.len() > 3);
        assert!(output.items.iter().all(|item| item.valid));
        assert!(output.items.iter().any(|item| item.title.ends_with(" mi")));
    }

    #[test]
    fn empty_query_shows_hint() {
        let item = first("");
        assert!(!item.valid);
        assert!(item.title.contains("Type an amount"));
    }

    #[test]
    fn unknown_token_fails_loudly() {
        let item = first("5 blargles to m");
        assert!(!item.valid);
        assert!(item.title.contains("Unknown unit or currency"));
    }

    #[test]
    fn mismatched_categories_fail_loudly() {
        let item = first("5 km to kg");
        assert!(!item.valid);
        assert!(item.title.contains("Cannot convert"));

        let item = first("5 km to usd");
        assert!(!item.valid);
        assert!(item.title.contains("Cannot convert"));
    }
}
