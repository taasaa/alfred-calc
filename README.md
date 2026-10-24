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

Install the packaged workflow:

1. Download `alfred-calc.alfredworkflow` from the latest release.
2. Double-click the file and confirm the import when Alfred asks.
3. Optional: enable prefix-free queries by ticking the Calc fallback under Alfred's
   Features, Default Results, Fallbacks. Without it, type `calc` before a query.

The package holds the universal binary and the workflow definition, so nothing else is
needed.

To build the package yourself:

```sh
rustup target add x86_64-apple-darwin   # Intel slice, optional
./tools/build-workflow.sh               # writes alfred-calc.alfredworkflow
```

The script builds both architectures, links them into one binary, and zips that binary
with `alfred/info.plist`.

### Wiring it by hand

If you would rather build the Alfred objects yourself, create a blank workflow, put
the binary in its folder, and add a Script Filter with keyword `calc`, language
`/bin/bash`, input mode `argv`, and the script `./alfred-calc "$1"`.

The path must stay relative. Alfred runs scripts from the workflow folder but does not
expose that folder in a variable, so `$alfred_workflow_directory` expands to nothing
and the script exits 127 with no visible result.

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
./tools/build-workflow.sh   # rebuilds the .alfredworkflow package
```

`alfred/info.plist` is the workflow definition and the source of truth for the
installed workflow. Edit it there rather than in Alfred's preferences folder, then
rebuild the package.

`docs/what-it-is.md` explains how a query is parsed, why the conversion maths is
built the way it is, which alternatives were rejected, and what is out of scope.

## Licence

MIT. See `LICENSE`.
