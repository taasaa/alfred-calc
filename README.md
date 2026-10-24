# alfred-calc

Converts currencies, crypto and units from natural language. A single Rust binary
answers queries like `100 eur to usd`, `2 btc to usd` and `5 km in m` in about 3 ms,
with no runtime and no API keys. It is built to be driven by an Alfred Script Filter,
and it also works on its own from a shell.

Verified on macOS 27.0.1 with Alfred 5.8.1. The binary is universal, so it runs on
Apple silicon and Intel Macs.

## Requirements

- macOS.
- Alfred 5 with the Powerpack, for the workflow parts. The command-line binary needs
  nothing beyond macOS.
- Rust and, for a universal binary, the second Apple target, if you build from source.

## Install

1. Build the binary.

   ```sh
   cargo build --release
   rustup target add x86_64-apple-darwin
   cargo build --release --target x86_64-apple-darwin
   lipo -create -output alfred-calc \
     target/release/alfred-calc \
     target/x86_64-apple-darwin/release/alfred-calc
   ```

   Skip the last three commands if you only use one architecture.

2. In Alfred, open Workflows and add a new blank workflow. Put `alfred-calc` inside
   the folder Alfred created for it.

3. Add a Script Filter to the workflow with these settings.

   | Setting | Value |
   | --- | --- |
   | Keyword | `calc` |
   | Language | `/bin/bash` |
   | With input as | `argv` |
   | Script | `./alfred-calc "$1"` |

   The script path is relative because Alfred runs scripts from the workflow folder.
   Alfred does not provide a variable holding that folder, so an absolute path or a
   variable would break.

4. Optional: add a Fallback trigger to the same Script Filter. Alfred shows fallback
   results only when its own results are empty, so queries then work with no keyword
   at all. Enable the fallback under Alfred's Features, Default Results, Fallbacks.

## Usage

```sh
alfred-calc "100 eur to usd"    # 112.0092 USD      Euro (EUR) → United States dollar (USD)
alfred-calc "2 btc to usd"      # 161,358 USD        Bitcoin (BTC) → United States dollar (USD)
alfred-calc "5 km in m"         # 5,000 m            kilometer (km) → meter (m)
alfred-calc "2 tb to gb"        # 2,000 GB           terabyte (TB) → gigabyte (GB)
alfred-calc "100 c to f"        # 212 °F             celsius (°C) → fahrenheit (°F)
alfred-calc "100 eur"           # converts to USD and ILS, your configured pair
```

Money results depend on the day's rates; the figures above were taken on 24 October
2026. Unit results are exact.

Accepted forms:

- A connector between source and target: `to`, `in`, `as`, `->`, `=>`.
- Two tokens with no connector: `100eur usd`.
- Currency symbols, rewritten before parsing: `100€ to $`.
- Unit names in singular or plural, British spelling, and common abbreviations:
  `kilometres`, `lbs`, `US gallons`, `miles per hour`.

Leave out the target and the program lists conversions to every other unit in the
same category, or to your configured currencies for money.

## Configuration

| Variable | Default | Effect |
| --- | --- | --- |
| `CALC_BASE_CURRENCIES` | `USD,ILS` | Comma-separated targets shown when a currency query names no target. |
| `alfred_workflow_cache` | set by Alfred | Where fetched rates and prices are cached. Without it the program uses `~/Library/Caches/alfred-calc`. |

## Coverage

- 108 unit definitions in 12 categories: length, area, volume, weight, speed,
  rotation, temperature, pressure, time, energy, power, and data storage. Decimal and
  binary storage units are separate, so `gb` and `gib` differ by the expected amount.
- About 166 currencies. Rates come from `open.er-api.com` as units per US dollar and
  are cached for 12 hours. 160 currencies have local names for matching.
- 46 crypto assets resolved by symbol or name. Prices come from CoinGecko's free
  market list, which covers the top 250 assets by market cap, and are cached for 15
  minutes.

Nothing in the program needs an API key.

When a refresh fails, the program keeps serving the cached file but labels its age in
the result row. With no cache and no network it returns a failure row instead of a
number.

## Development

```sh
cargo test                  # 25 tests
cargo clippy --all-targets  # expected: no findings
cargo fmt --check
```

`docs/what-it-is.md` explains how a query is parsed, why the conversion maths is
built the way it is, which alternatives were rejected, and what is out of scope.

## Licence

MIT. See `LICENSE`.
