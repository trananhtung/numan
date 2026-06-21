//! Parse humanized magnitude strings back into [`f64`].

use alloc::string::String;
use core::fmt;

/// Error returned by [`parse`] when the input is not a recognizable number.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    /// The input was empty or only whitespace.
    Empty,
    /// The numeric portion could not be parsed as a number.
    InvalidNumber(String),
    /// The trailing suffix was not a recognized magnitude unit.
    UnknownSuffix(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => f.write_str("empty input"),
            ParseError::InvalidNumber(s) => write!(f, "invalid number: {s:?}"),
            ParseError::UnknownSuffix(s) => write!(f, "unknown magnitude suffix: {s:?}"),
        }
    }
}

impl core::error::Error for ParseError {}

/// Parse a humanized number such as `"1.2K"`, `"1.5 million"`, `"3M"` or
/// `"-2.5bn"` back into an [`f64`].
///
/// Recognizes, case-insensitively:
/// - compact suffixes `K`, `M`, `B` (`bn`), `T`;
/// - short-scale words `thousand` … `decillion`;
/// - SI metric prefixes `k`, `M`, `G`, `T`, `P`, `Z`, `Y`, `R`, `Q`.
///
/// The letter `m`/`M` means *million* (compact convention), never SI *milli*. SI
/// *exa* (`E`) is **not** parsed, to avoid clashing with exponent notation like
/// `1e3`; `metric` output at exa therefore returns [`ParseError::UnknownSuffix`]
/// rather than a wrong value. Thousands separators (`,` and `_`) are ignored.
///
/// # Errors
///
/// Returns [`ParseError::Empty`] for blank input, [`ParseError::InvalidNumber`]
/// if the numeric part is malformed, or [`ParseError::UnknownSuffix`] if the
/// (alphabetic) suffix is not a recognized magnitude unit.
pub fn parse(s: &str) -> Result<f64, ParseError> {
    let t = s.trim();
    if t.is_empty() {
        return Err(ParseError::Empty);
    }

    let (num_part, suffix_part) = split_number_suffix(t);

    // A unit suffix is alphabetic; anything else (digits, extra dots, `0x…`)
    // means the input is a malformed number, not an unknown unit.
    if !suffix_part.is_empty() && !suffix_part.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(ParseError::InvalidNumber(String::from(t)));
    }

    // Strip thousands separators before parsing the numeric part.
    let cleaned: String = num_part.chars().filter(|&c| c != ',' && c != '_').collect();
    let base: f64 = cleaned
        .parse()
        .map_err(|_| ParseError::InvalidNumber(String::from(t)))?;

    let mult = multiplier(suffix_part)
        .ok_or_else(|| ParseError::UnknownSuffix(String::from(suffix_part)))?;

    Ok(base * mult)
}

/// Split a trimmed input into its leading numeric text and trailing unit suffix.
fn split_number_suffix(t: &str) -> (&str, &str) {
    let bytes = t.as_bytes();
    let mut i = 0;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let mut seen_dot = false;
    let mut seen_exp = false;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_digit() || c == b',' || c == b'_' {
            i += 1;
        } else if c == b'.' && !seen_dot && !seen_exp {
            seen_dot = true;
            i += 1;
        } else if (c == b'e' || c == b'E') && !seen_exp {
            // Only treat `e` as an exponent if followed by an optional sign + digit;
            // otherwise it is the start of a unit suffix (and avoids eating `exa`).
            let mut j = i + 1;
            if j < bytes.len() && (bytes[j] == b'+' || bytes[j] == b'-') {
                j += 1;
            }
            if j < bytes.len() && bytes[j].is_ascii_digit() {
                seen_exp = true;
                i = j;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    (&t[..i], t[i..].trim())
}

/// Map a (possibly empty) magnitude suffix to its multiplier.
///
/// Letters are chosen to round-trip what `compact`/`word`/`metric` emit:
/// `m`/`M` means *million* (compact convention), never SI *milli*; `q` is SI
/// *quetta* (`1e30`), matching `metric` output. SI *exa* (`E`) is intentionally
/// unsupported to avoid clashing with exponent notation like `1e3`.
fn multiplier(suffix: &str) -> Option<f64> {
    if suffix.is_empty() {
        return Some(1.0);
    }
    let s = suffix.to_ascii_lowercase();
    let m = match s.as_str() {
        "k" | "thousand" => 1e3,
        "m" | "mn" | "million" => 1e6,
        "b" | "bn" | "g" | "billion" => 1e9,
        "t" | "trillion" => 1e12,
        "p" | "quadrillion" => 1e15,
        "quintillion" => 1e18,
        "z" | "sextillion" => 1e21,
        "y" | "septillion" => 1e24,
        "r" | "octillion" => 1e27,
        "q" | "nonillion" => 1e30,
        "decillion" => 1e33,
        _ => return None,
    };
    Some(m)
}
