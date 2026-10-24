//! Crypto assets. Symbols resolve offline from a static table; USD prices come from
//! CoinGecko's free markets endpoint and are cached for 15 minutes.

use std::collections::HashMap;
use std::time::Duration;

const PRICES_URL: &str = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=250&page=1&sparkline=false";

pub const COINS: &[(&str, &str)] = &[
    ("btc", "Bitcoin"),
    ("eth", "Ethereum"),
    ("usdt", "Tether"),
    ("usdc", "USD Coin"),
    ("dai", "Dai"),
    ("busd", "Binance USD"),
    ("tusd", "TrueUSD"),
    ("sol", "Solana"),
    ("ada", "Cardano"),
    ("xrp", "XRP"),
    ("doge", "Dogecoin"),
    ("dot", "Polkadot"),
    ("link", "Chainlink"),
    ("matic", "Polygon"),
    ("avax", "Avalanche"),
    ("atom", "Cosmos"),
    ("ltc", "Litecoin"),
    ("xlm", "Stellar"),
    ("bch", "Bitcoin Cash"),
    ("uni", "Uniswap"),
    ("algo", "Algorand"),
    ("near", "NEAR Protocol"),
    ("ftm", "Fantom"),
    ("aave", "Aave"),
    ("arb", "Arbitrum"),
    ("op", "Optimism"),
    ("ton", "Toncoin"),
    ("shib", "Shiba Inu"),
    ("trx", "TRON"),
    ("etc", "Ethereum Classic"),
    ("xmr", "Monero"),
    ("fil", "Filecoin"),
    ("hbar", "Hedera"),
    ("icp", "Internet Computer"),
    ("vet", "VeChain"),
    ("grt", "The Graph"),
    ("sand", "The Sandbox"),
    ("mana", "Decentraland"),
    ("ape", "ApeCoin"),
    ("crv", "Curve DAO"),
    ("mkr", "Maker"),
    ("snx", "Synthetix"),
    ("ldo", "Lido DAO"),
    ("inj", "Injective"),
    ("sui", "Sui"),
    ("apt", "Aptos"),
];

pub struct Prices {
    /// Symbol (lowercase) -> price in USD.
    per_coin: HashMap<String, f64>,
    pub note: Option<String>,
}

impl Prices {
    pub fn value_in_usd(&self, symbol: &str) -> Option<f64> {
        self.per_coin.get(&symbol.to_lowercase()).copied()
    }

    pub fn parse(body: &str) -> Result<HashMap<String, f64>, String> {
        let json: serde_json::Value =
            serde_json::from_str(body).map_err(|err| format!("unreadable price data: {err}"))?;
        let list = json
            .as_array()
            .ok_or_else(|| "price data is not a list".to_string())?;
        let mut map = HashMap::with_capacity(list.len());
        for entry in list {
            let symbol = entry.get("symbol").and_then(|v| v.as_str());
            let price = entry.get("current_price").and_then(|v| v.as_f64());
            if let (Some(symbol), Some(price)) = (symbol, price) {
                if price > 0.0 {
                    map.insert(symbol.to_lowercase(), price);
                }
            }
        }
        if map.is_empty() {
            return Err("price data contained no usable values".to_string());
        }
        Ok(map)
    }

    pub fn load() -> Result<Prices, String> {
        let cached =
            crate::fetch::get(PRICES_URL, "crypto_usd.json", Duration::from_secs(15 * 60))?;
        let per_coin = Prices::parse(&cached.body)?;
        Ok(Prices {
            per_coin,
            note: cached.stale_note,
        })
    }
}

/// Resolve a typed token to a known coin symbol without the network.
pub fn resolve(token: &str) -> Option<&'static str> {
    let lowered = token.trim().to_lowercase();
    if lowered.is_empty() {
        return None;
    }
    if let Some((symbol, _)) = COINS.iter().find(|(symbol, _)| *symbol == lowered) {
        return Some(symbol);
    }
    let mut matches: Vec<&'static str> = Vec::new();
    for (symbol, name) in COINS {
        let name = name.to_lowercase();
        if lowered == name || lowered == format!("{name}s") {
            return Some(symbol);
        }
        if name.split_whitespace().any(|word| word == lowered) {
            matches.push(symbol);
        }
    }
    match matches.len() {
        1 => Some(matches[0]),
        _ => None,
    }
}

pub fn name_of(symbol: &str) -> String {
    COINS
        .iter()
        .find(|(s, _)| s.eq_ignore_ascii_case(symbol))
        .map(|(_, name)| (*name).to_string())
        .unwrap_or_else(|| symbol.to_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_symbols_and_names() {
        assert_eq!(resolve("btc"), Some("btc"));
        assert_eq!(resolve("BTC"), Some("btc"));
        assert_eq!(resolve("bitcoin"), Some("btc"));
        assert_eq!(resolve("bitcoins"), Some("btc"));
        assert_eq!(resolve("usdc"), Some("usdc"));
        assert_eq!(resolve("eur"), None);
    }

    #[test]
    fn parses_market_list() {
        let body = r#"[{"id":"bitcoin","symbol":"btc","name":"Bitcoin","current_price":80691},
                       {"id":"ethereum","symbol":"eth","name":"Ethereum","current_price":2376.09}]"#;
        let prices = Prices::parse(body).unwrap();
        assert_eq!(prices.get("btc").copied(), Some(80691.0));
        assert_eq!(prices.get("eth").copied(), Some(2376.09));
    }

    #[test]
    fn rejects_garbage_payloads() {
        assert!(Prices::parse("{}").is_err());
        assert!(Prices::parse("[]").is_err());
        assert!(Prices::parse(r#"[{"symbol":"btc"}]"#).is_err());
    }
}
