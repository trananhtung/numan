//! Magnitude formatting: the [`Formatter`] builder and [`Notation`] styles.

use alloc::format;
use alloc::string::String;

use crate::number::Number;

/// Short business suffixes (powers of 1000): `""`, `K`, `M`, `B`, `T`.
/// Magnitudes beyond a trillion fall back to scientific notation.
const COMPACT: [&str; 5] = ["", "K", "M", "B", "T"];

/// Largest format precision `core::fmt` accepts (it stores precision as a `u16`).
const MAX_PRECISION: usize = u16::MAX as usize;

/// Short-scale magnitude words (powers of 1000).
const WORD: [&str; 12] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
    "nonillion",
    "decillion",
];

/// SI metric prefixes for large magnitudes (powers of 1000).
const METRIC: [&str; 11] = ["", "k", "M", "G", "T", "P", "E", "Z", "Y", "R", "Q"];

/// Which family of suffixes to use when shortening a number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum Notation {
    /// Short business suffixes: `K`, `M`, `B`, `T` (e.g. `1.2K`). The default.
    #[default]
    Compact,
    /// Full short-scale words: `thousand`, `million`, `billion`, … (e.g. `1.2 million`).
    Word,
    /// SI metric prefixes: `k`, `M`, `G`, `T`, … (e.g. `1.2 k`).
    Metric,
}

/// A configurable number humanizer.
///
/// Construct one of the presets ([`Formatter::compact`], [`Formatter::word`],
/// [`Formatter::metric`]) and tweak it with the builder methods, then call
/// [`Formatter::format`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Formatter {
    notation: Notation,
    precision: usize,
    trim_zeros: bool,
    space: bool,
    sign_plus: bool,
}

impl Default for Formatter {
    /// The [`Formatter::compact`] preset.
    fn default() -> Self {
        Self::compact()
    }
}

impl Formatter {
    /// Compact preset: `1.2K`, 1 decimal, trailing zeros trimmed, no space.
    #[must_use]
    pub const fn compact() -> Self {
        Self {
            notation: Notation::Compact,
            precision: 1,
            trim_zeros: true,
            space: false,
            sign_plus: false,
        }
    }

    /// Word preset: `1.2 million`, 1 decimal, trailing zeros trimmed, with space.
    #[must_use]
    pub const fn word() -> Self {
        Self {
            notation: Notation::Word,
            precision: 1,
            trim_zeros: true,
            space: true,
            sign_plus: false,
        }
    }

    /// Metric (SI) preset: `1.2 k`, 1 decimal, trailing zeros trimmed, with space.
    #[must_use]
    pub const fn metric() -> Self {
        Self {
            notation: Notation::Metric,
            precision: 1,
            trim_zeros: true,
            space: true,
            sign_plus: false,
        }
    }

    /// Start from a [`Notation`] with that notation's preset defaults.
    #[must_use]
    pub const fn new(notation: Notation) -> Self {
        match notation {
            Notation::Compact => Self::compact(),
            Notation::Word => Self::word(),
            Notation::Metric => Self::metric(),
        }
    }

    /// Set the maximum number of fractional digits (default `1`).
    #[must_use]
    pub const fn precision(mut self, precision: usize) -> Self {
        self.precision = precision;
        self
    }

    /// Whether to trim trailing zeros (and a dangling `.`). Default `true`.
    #[must_use]
    pub const fn trim_zeros(mut self, trim: bool) -> Self {
        self.trim_zeros = trim;
        self
    }

    /// Whether to put a space between the number and its suffix.
    #[must_use]
    pub const fn space(mut self, space: bool) -> Self {
        self.space = space;
        self
    }

    /// Whether to prefix non-negative numbers with an explicit `+`.
    #[must_use]
    pub const fn sign_plus(mut self, sign_plus: bool) -> Self {
        self.sign_plus = sign_plus;
        self
    }

    /// Humanize `value` into a [`String`] using this formatter's settings.
    #[must_use]
    pub fn format<N: Number>(&self, value: N) -> String {
        self.format_f64(value.to_f64())
    }

    const fn suffixes(self) -> &'static [&'static str] {
        match self.notation {
            Notation::Compact => &COMPACT,
            Notation::Word => &WORD,
            Notation::Metric => &METRIC,
        }
    }

    fn format_f64(&self, value: f64) -> String {
        // Non-finite values (`is_nan`/`is_infinite` are available in `core`).
        if value.is_nan() {
            return String::from("NaN");
        }
        if value.is_infinite() {
            return String::from(if value.is_sign_negative() {
                "-inf"
            } else {
                "inf"
            });
        }

        // Work on the magnitude so float `Display` never injects its own sign;
        // `is_sign_negative` maps `-0.0` to a positive magnitude.
        let negative = value.is_sign_negative();
        let magnitude = if negative { -value } else { value };

        // `core::fmt` stores precision as a `u16`; clamp to avoid a panic.
        let precision = self.precision.min(MAX_PRECISION);

        let suffixes = self.suffixes();
        let max_idx = suffixes.len() - 1;
        let mut v = magnitude;
        let mut idx = 0;
        while v >= 1000.0 && idx < max_idx {
            v /= 1000.0;
            idx += 1;
        }

        // Beyond the largest suffix, fall back to scientific notation. This stays
        // readable and round-trips through `parse` instead of emitting "1000T".
        if idx == max_idx && v >= 1000.0 {
            return self.with_sign(negative, false, &format!("{magnitude:e}"));
        }

        let mut rendered = format!("{v:.precision$}");

        // Rounding can push the mantissa up to 1000 (e.g. 999.95 -> "1000.0");
        // promote it to the next suffix so we never emit "1000K".
        if idx < max_idx {
            if let Ok(parsed) = rendered.parse::<f64>() {
                if parsed >= 1000.0 {
                    idx += 1;
                    v /= 1000.0;
                    rendered = format!("{v:.precision$}");
                }
            }
        }

        if self.trim_zeros {
            rendered = trim_trailing_zeros(&rendered);
        }

        let suffix = suffixes[idx];
        let body = if suffix.is_empty() {
            rendered
        } else {
            let sep = if self.space { " " } else { "" };
            let mut b = rendered;
            b.push_str(sep);
            b.push_str(suffix);
            b
        };

        self.with_sign(negative, is_all_zero(&body), &body)
    }

    /// Prefix `body` with the appropriate sign. A value whose magnitude rounds to
    /// zero is never signed (no `-0`, no `+-0`).
    fn with_sign(&self, negative: bool, is_zero: bool, body: &str) -> String {
        let mut out = String::new();
        if is_zero {
            // no sign
        } else if negative {
            out.push('-');
        } else if self.sign_plus {
            out.push('+');
        }
        out.push_str(body);
        out
    }
}

/// Whether a rendered body represents zero (only `0`, `.` characters).
fn is_all_zero(s: &str) -> bool {
    s.bytes().all(|b| b == b'0' || b == b'.')
}

/// Remove trailing zeros and a dangling decimal point from a rendered number.
fn trim_trailing_zeros(s: &str) -> String {
    if s.contains('.') {
        String::from(s.trim_end_matches('0').trim_end_matches('.'))
    } else {
        String::from(s)
    }
}
