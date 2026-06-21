# numan

[![Crates.io](https://img.shields.io/crates/v/numan.svg)](https://crates.io/crates/numan)
[![Documentation](https://docs.rs/numan/badge.svg)](https://docs.rs/numan)
[![CI](https://github.com/trananhtung/numan/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/numan/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/numan.svg)](#license)
[![no_std](https://img.shields.io/badge/no__std-yes-brightgreen.svg)](#no_std)

**Humanize numbers in Rust** — turn raw numbers into the short, friendly strings
you see in dashboards, CLIs and reports, and parse them back.

```rust
assert_eq!(numan::compact(1_234),       "1.2K");
assert_eq!(numan::word(1_200_000),      "1.2 million");
assert_eq!(numan::metric(1_500),        "1.5 k");
assert_eq!(numan::ordinal(22),          "22nd");
assert_eq!(numan::parse("1.5M").unwrap(), 1_500_000.0);
```

Zero dependencies. `#![no_std]` (needs only `alloc`). One small, focused crate.

## Why numan?

Rust's ecosystem already humanizes **file sizes**
([`humansize`](https://crates.io/crates/humansize)) and **relative time**
([`chrono-humanize`](https://crates.io/crates/chrono-humanize)). What was missing
is the part Python's [`humanize`](https://github.com/jmoiron/humanize) and Go's
[`go-humanize`](https://github.com/dustin/go-humanize) are famous for: shortening
the **magnitude of plain numbers**.

| Need | Reach for |
| --- | --- |
| File sizes (`1.2 MiB`) | `humansize` |
| Relative time (`3 hours ago`) | `chrono-humanize` |
| Number → words (`forty-two`) | `num2words` |
| Locale thousands grouping | `num-format` |
| **Compact / word / metric magnitude (`1.2K`, `1.2 million`, `1.2 k`) + parsing** | **`numan`** ✅ |

## Install

```toml
[dependencies]
numan = "0.1"
```

## Usage

### Compact, word and metric notation

```rust
use numan::{compact, word, metric};

assert_eq!(compact(950),         "950");
assert_eq!(compact(12_300),      "12.3K");
assert_eq!(compact(1_500_000),   "1.5M");
assert_eq!(compact(-2_000_000),  "-2M");

assert_eq!(word(2_500),          "2.5 thousand");
assert_eq!(word(1_000_000),      "1 million");

assert_eq!(metric(1_000),        "1 k");   // SI: lowercase kilo
assert_eq!(metric(2_200_000),    "2.2 M");
```

### Ordinals

```rust
assert_eq!(numan::ordinal(1),   "1st");
assert_eq!(numan::ordinal(112), "112th");
```

### Parsing back

```rust
use numan::parse;

assert_eq!(parse("1.2K").unwrap(),       1_200.0);
assert_eq!(parse("1.5 million").unwrap(), 1_500_000.0);
assert_eq!(parse("-2.5bn").unwrap(),     -2_500_000_000.0);
assert_eq!(parse("1,234").unwrap(),       1_234.0); // separators ignored
```

Great for CLI flags like `--limit 1.5M`.

### Fine-grained control with `Formatter`

```rust
use numan::Formatter;

let f = Formatter::compact().precision(2).space(true);
assert_eq!(f.format(1_234_567), "1.23 M");

// Keep trailing zeros, force an explicit sign:
let f = Formatter::compact().trim_zeros(false).sign_plus(true);
assert_eq!(f.format(1_000_000), "+1.0M");
```

| Builder method | Default | Effect |
| --- | --- | --- |
| `precision(n)` | `1` | Max fractional digits |
| `trim_zeros(bool)` | `true` | Drop trailing `0`s and a dangling `.` |
| `space(bool)` | per-notation | Space between number and suffix |
| `sign_plus(bool)` | `false` | Prefix non-negatives with `+` |

## Behavior notes

- **Rounding** uses Rust's standard round-half-to-even, and correctly carries
  into the next suffix (`compact(999_950)` → `"1M"`, never `"1000K"`).
- **Beyond the table**, `compact` (above a trillion) falls back to scientific
  notation (`compact(1e36)` → `"1e36"`), which still round-trips through `parse`.
- **Non-finite** floats render as `"NaN"`, `"inf"`, `"-inf"`.
- **Negative zero** (and negatives that round to zero) render as `"0"` — never `"-0"`.
- In `parse`, `m`/`M` means **million** (compact convention), never SI *milli*;
  SI *exa* (`E`) is not parsed, to avoid clashing with exponent notation (`1e3`),
  so `metric` output at exa returns an error rather than a wrong value.

## no_std

`numan` is `#![no_std]` and only needs `alloc` (for `String`). It builds for
bare-metal targets such as `thumbv7em-none-eabi`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
