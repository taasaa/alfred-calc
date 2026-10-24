# What alfred-calc is, and why it is built this way

This document explains the design of the program. For installation and command
examples, read the README.

## The problem

Spotting a price in one currency and needing it in another, or reading a disk size in
the units someone else used, interrupts whatever you were doing. The program answers
those questions from a single line of text, in the place you already type, without
opening a browser or a calculator app.

The scope is deliberately narrow. It converts money and measurements:

- currencies and crypto
- length, area, volume, weight, speed, rotation, temperature, pressure, time, energy,
  power, and data storage

It does not do percentages, VAT, date arithmetic, CSS pixel units, or colour
conversion. Those belong to other tools, and carrying them made earlier versions large
without adding much daily use.

## How a query is read

A query has three parts: an amount, a source, and an optional target. The parser walks
the line without a regex engine, in three steps.

1. Currency symbols become currency codes, so `100€ to $` and `100 eur to usd` take
   the same path.
2. The leading number is separated from the words. `100eur` and `100 eur` are
   equivalent.
3. The remaining words are split at a connector (`to`, `in`, `as`, `->`, `=>`). Without
   a connector, the longest run of words that resolves to a known unit or currency
   becomes the source and the rest becomes the target. This is what keeps `US gallons`
   and `miles per hour` whole, and what lets `5 in to cm` treat `in` as inches rather
   than as the connector.

Resolution then runs in a fixed order: units, then crypto symbols, then currencies.
Units come first because they are local and unambiguous; a token that matches no unit
falls through to money.

`in` is a unit (inches) and also a connector. A connector only splits the line when at
least one word precedes it, so both readings work.

## The conversion model

Every unit belongs to exactly one category and carries a factor and an offset, so the
base value is `value × factor + offset`. Temperature is not special-cased: Celsius is
an offset of 273.15 from kelvin, and Fahrenheit is a factor of 5/9 plus an offset.
Testing temperature through the same path as length removed a whole branch of code.

Data storage is split into two families rather than one. Decimal units (`kb`, `mb`,
`gb`) step by 1000 and binary units (`kib`, `mib`, `gib`) step by 1024. Collapsing them
is a common bug; an earlier TypeScript version of this tool listed `kb` at exponent 0,
which made a kilobyte equal to a byte.

Money goes through one pivot. Each currency has a rate expressed as units per US
dollar, and each crypto asset has a USD price, so any pair of money units converts by
dividing one USD value by the other. This is why `2 btc to ils` works even though no
exchange quotes that pair directly.

## Data and freshness

Both data sources are free and keyless. `open.er-api.com` publishes daily rates for
about 166 currencies; CoinGecko publishes prices for the top 250 crypto assets by
market cap. An earlier version of this tool required an API key for both and returned
nothing without one.

Rates are cached for 12 hours and crypto prices for 15 minutes, in the directory
Alfred provides for the workflow, or `~/Library/Caches/alfred-calc` when run from a
shell.

A failed refresh does not silently reuse old numbers. If a cache file exists, the
program serves it and appends its age and the refresh error to the result row. With no
cache and no network it returns a row that says the data was unavailable. A stale
number labelled as stale is usable; a stale number presented as current is not.

## Failure behaviour

Every input the program cannot answer produces a visible row rather than an empty
result:

- `5 blargles to m` reports the unknown token.
- `5 km to kg` reports that the two units are in different categories.
- `5 km to usd` reports that measurements and money do not mix.
- An unreadable cache file reports that the rate data could not be parsed.

An empty result in Alfred is the one failure a user cannot diagnose, so the program
never returns one.

## Alternatives considered

**Alfred's own Unit Converter and Currency Converter workflows.** Both are free,
keyless and self-contained: one is a universal binary, the other a JavaScript script
run by `osascript`. They cover less, and the unit one answers wrongly rather than
refusing. Queries `2 tb gb`, `40 as a % of 50` and `12px to em` returned a confident
`0 ML`, `5983914.828 Mm` and `Invalid Unit` respectively. Merging the two into one
prefix-free workflow would have been about half a day of work and would have kept
those parsing faults, since both would still be separate programs behind a router.

**Keeping the PHP workflow.** It needed a PHP install, which is a large dependency for
one workflow, and it is deprecated upstream.

**The TypeScript and Bun version.** Closer to a real answer, but it ships a 61 MB
compiled binary and still needed API keys. This version is 3.8 MB for two
architectures.

**Rust over Python.** Python with `pint` would have been faster to write and would have
brought a larger unit database. Rust was chosen for start-up time and for a binary
with no interpreter to keep working across macOS upgrades. Query latency here is 3 to
4 ms including process start.

## Limits

- Crypto prices are spot prices in USD from one source. They are fine for a rough
  conversion, not for accounting.
- Currency rates update daily upstream, so intraday movement is invisible.
- Unit aliases are a fixed table. A spelling that is missing fails loudly rather than
  guessing, and adding one is a single line in `src/units.rs`.
- Percentage, VAT, time-zone and pixel conversions are out of scope by design.

## Verification

The program was developed and checked on macOS 27.0.1 with Alfred 5.8.1 on Apple
silicon. The test suite covers parsing, unit conversion including the temperature and
storage edge cases, rate and price parsing, and the stale-cache path, which is tested
against a refused connection rather than assumed. Alfred integration was verified by
invoking the workflow chain through Alfred itself and confirming the program wrote its
cache files into the directory Alfred provides.
