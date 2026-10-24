# Changelog

All notable changes to this project are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[semantic versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-10-24

### Added

- Natural-language conversion for currencies, crypto and units, as a single Rust
  binary that prints Alfred script-filter JSON.
- 108 unit definitions across 12 categories, with temperature handled as an affine
  unit and decimal and binary storage units kept distinct.
- Currency rates from `open.er-api.com` and crypto prices from CoinGecko, both
  keyless, cached for 12 hours and 15 minutes respectively.
- `CALC_BASE_CURRENCIES` to set the currencies shown when a query names no target.
  Defaults to `USD,ILS`.
- Cached data is served with its age and the refresh error when a fetch fails; no
  cache and no network produces a failure row rather than a number.
- Universal binary support for Apple silicon and Intel through a `lipo` step in the
  README.
- 25 tests, clippy clean, rustfmt clean.

[0.1.0]: https://github.com/taasaa/alfred-calc/releases/tag/v0.1.0
