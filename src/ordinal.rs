//! English ordinal suffixes (`1st`, `2nd`, `3rd`, `4th`, …).

use alloc::format;
use alloc::string::String;

use crate::number::Integer;

/// Format an integer with its English ordinal suffix: `1` → `"1st"`,
/// `22` → `"22nd"`, `113` → `"113th"`.
///
/// Negative numbers keep their sign (`-1` → `"-1st"`). The suffix is chosen from
/// the magnitude, so `-21` → `"-21st"`.
///
/// # Note
///
/// `u128`/`usize` inputs above `i128::MAX` (~1.7e38) saturate to `i128::MAX`
/// before formatting. This is unreachable on 64-bit platforms and far outside any
/// realistic ordinal use.
#[must_use]
pub fn ordinal<I: Integer>(n: I) -> String {
    let v = n.to_i128();
    // `unsigned_abs` avoids overflow at `i128::MIN`.
    let abs = v.unsigned_abs();
    let last_two = (abs % 100) as u64;
    let last_one = (abs % 10) as u64;

    let suffix = if (11..=13).contains(&last_two) {
        "th"
    } else {
        match last_one {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };

    let mut s = format!("{v}");
    s.push_str(suffix);
    s
}
