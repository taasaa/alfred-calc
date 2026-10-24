# alfred-calc

Natural-language conversion for Alfred: currencies, crypto, and units. Rust, single
binary, no runtime.

Replaces `alfred-calculate-anything` (PHP, then TypeScript/Bun). Scope is deliberately
narrow: **currency, crypto, units and measurements**. No colour conversion, no VAT, no
px/em/rem, no time arithmetic.

## Usage

```sh
alfred-calc "100 eur to usd"
alfred-calc "2 btc to usd"
alfred-calc "5 km in m"
alfred-calc "2 tb to gb"
alfred-calc "100€ to $"
alfred-calc "5 km"          # lists conversions to every other length unit
```

Output is Alfred script-filter JSON on stdout, so the workflow object is simply:

```
./alfred-calc "{query}"
```

Natural language accepted: `to`, `in`, `as`, `->`, `=>` as connectors, or two tokens
(`100eur usd`). Currency symbols (`$`, `€`, `£`, `¥`, `₪`, `₹`, `₽`) are rewritten to ISO
codes before parsing. Unit names accept singular, plural, British spelling and common
abbreviations (`kilometres`, `lbs`, `US gallons`, `miles per hour`).

## Data

| What | Source | Cache |
| --- | --- | --- |
| Fiat rates (units per USD, ~166 currencies) | `open.er-api.com` | 12 hours |
| Crypto prices (top 250 by market cap, USD) | CoinGecko free markets endpoint | 15 minutes |

No API keys. Cache lives in `$alfred_workflow_cache` when Alfred sets it, otherwise
`~/Library/Caches/alfred-calc`.

Failure policy: unknown tokens, cross-category conversions, and unavailable market data
all produce an invalid result row that says what went wrong. A failed refresh falls back
to the cached file only if one exists, and the row is annotated with the data's age —
stale numbers are labelled, never passed off as fresh.

Environment variables:

- `CALC_BASE_CURRENCIES` — comma-separated targets shown when no target is typed
  (default `USD,ILS,EUR,GBP`).
- `alfred_workflow_cache` — set by Alfred; overrides the cache directory.

## Build and test

```sh
cargo test
cargo build --release   # target/release/alfred-calc
```

## Layout

- `src/parse.rs` — amount + source + target, connector handling, longest-prefix split.
- `src/units.rs` — 12 categories, affine factors (temperature is just another affine
  unit), decimal versus binary storage units kept distinct.
- `src/currency.rs`, `src/currency_data.rs` — 160 ISO names for offline resolution;
  rates fetched as units-per-USD.
- `src/crypto.rs` — offline symbol table, online prices.
- `src/num.rs` — number parsing and display formatting.
- `src/fetch.rs` — cached HTTP with explicit staleness reporting.
- `src/alfred.rs` — script-filter JSON.
