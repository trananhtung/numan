//! # numan — humanize numbers
//!
//! Turn raw numbers into the short, human-readable strings you see in
//! dashboards, CLIs and reports — and parse them back.
//!
//! ```
//! assert_eq!(numan::compact(1_234), "1.2K");
//! assert_eq!(numan::word(1_200_000), "1.2 million");
//! assert_eq!(numan::metric(1_500), "1.5 k");
//! assert_eq!(numan::ordinal(22), "22nd");
//! assert_eq!(numan::parse("1.5M").unwrap(), 1_500_000.0);
//! ```
//!
//! ## Why numan?
//!
//! Rust already has great crates for *file sizes* ([`humansize`]) and *relative
//! time* ([`chrono-humanize`]). `numan` fills the remaining gap from Python's
//! [`humanize`] and Go's [`go-humanize`]: shortening the *magnitude* of plain
//! numbers — `1.2K`, `1.2 million`, `1.2 k` — with a single focused,
//! zero-dependency, `#![no_std]` crate.
//!
//! ## Configuring output
//!
//! Use [`Formatter`] for full control over precision, spacing and sign:
//!
//! ```
//! use numan::Formatter;
//!
//! let f = Formatter::compact().precision(2).space(true);
//! assert_eq!(f.format(1_234_567), "1.23 M");
//! ```
//!
//! [`humansize`]: https://crates.io/crates/humansize
//! [`chrono-humanize`]: https://crates.io/crates/chrono-humanize
//! [`humanize`]: https://github.com/jmoiron/humanize
//! [`go-humanize`]: https://github.com/dustin/go-humanize

#![no_std]
#![doc(html_root_url = "https://docs.rs/numan/0.1.0")]
// These casts are intentional: magnitude formatting only needs the leading
// significant digits, so lossy float/int conversions are by design.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless
)]

extern crate alloc;

use alloc::string::String;

mod format;
mod number;
mod ordinal;
mod parse;

pub use format::{Formatter, Notation};
pub use number::{Integer, Number};
pub use ordinal::ordinal;
pub use parse::{parse, ParseError};

/// Format a number in **compact** notation: `1_234` → `"1.2K"`.
///
/// Shorthand for [`Formatter::compact`]`().format(value)`.
///
/// ```
/// assert_eq!(numan::compact(950), "950");
/// assert_eq!(numan::compact(12_300), "12.3K");
/// assert_eq!(numan::compact(-1_500_000), "-1.5M");
/// ```
#[must_use]
pub fn compact<N: Number>(value: N) -> String {
    Formatter::compact().format(value)
}

/// Format a number in **word** notation: `1_200_000` → `"1.2 million"`.
///
/// Shorthand for [`Formatter::word`]`().format(value)`.
///
/// ```
/// assert_eq!(numan::word(2_500), "2.5 thousand");
/// assert_eq!(numan::word(7_000_000_000_i64), "7 billion");
/// ```
#[must_use]
pub fn word<N: Number>(value: N) -> String {
    Formatter::word().format(value)
}

/// Format a number in **SI metric** notation: `1_500` → `"1.5 k"`.
///
/// Shorthand for [`Formatter::metric`]`().format(value)`.
///
/// ```
/// assert_eq!(numan::metric(2_200_000), "2.2 M");
/// ```
#[must_use]
pub fn metric<N: Number>(value: N) -> String {
    Formatter::metric().format(value)
}
