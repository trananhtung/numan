//! End-to-end behavioral spec for the public `numan` API.

// Parsed results are exact integer-valued floats, so `==` is intentional here.
#![allow(clippy::float_cmp)]

use numan::{compact, metric, ordinal, parse, word, Formatter, Notation, ParseError};

// ---------------------------------------------------------------------------
// compact()
// ---------------------------------------------------------------------------

#[test]
fn compact_below_thousand_is_unchanged() {
    assert_eq!(compact(0), "0");
    assert_eq!(compact(5), "5");
    assert_eq!(compact(42), "42");
    assert_eq!(compact(999), "999");
}

#[test]
fn compact_thousands() {
    assert_eq!(compact(1_000), "1K");
    assert_eq!(compact(1_234), "1.2K");
    assert_eq!(compact(12_300), "12.3K");
    assert_eq!(compact(1_990), "2K"); // rounds 1.99 -> 2.0 -> trimmed
}

#[test]
fn compact_millions_billions_trillions() {
    assert_eq!(compact(1_000_000), "1M");
    assert_eq!(compact(1_500_000), "1.5M");
    assert_eq!(compact(12_345_678), "12.3M");
    assert_eq!(compact(1_000_000_000_i64), "1B");
    assert_eq!(compact(1_000_000_000_000_i64), "1T");
}

#[test]
fn compact_negative() {
    assert_eq!(compact(-1_500), "-1.5K");
    assert_eq!(compact(-2_000_000), "-2M");
}

#[test]
fn compact_rounding_carries_into_next_suffix() {
    // 999_950 / 1000 = 999.95 -> rounds to 1000.0 -> must become "1M", not "1000K".
    assert_eq!(compact(999_950), "1M");
}

#[test]
fn compact_floats() {
    assert_eq!(compact(0.5), "0.5");
    assert_eq!(compact(1_234.5_f64), "1.2K");
}

// ---------------------------------------------------------------------------
// word()
// ---------------------------------------------------------------------------

#[test]
fn word_below_thousand_is_unchanged() {
    assert_eq!(word(0), "0");
    assert_eq!(word(500), "500");
}

#[test]
fn word_scales() {
    assert_eq!(word(1_000), "1 thousand");
    assert_eq!(word(1_500), "1.5 thousand");
    assert_eq!(word(1_000_000), "1 million");
    assert_eq!(word(1_200_000), "1.2 million");
    assert_eq!(word(7_000_000_000_i64), "7 billion");
    assert_eq!(word(2_500_000_000_000_i64), "2.5 trillion");
}

#[test]
fn word_negative() {
    assert_eq!(word(-1_000_000), "-1 million");
}

// ---------------------------------------------------------------------------
// metric()
// ---------------------------------------------------------------------------

#[test]
fn metric_si_prefixes() {
    assert_eq!(metric(500), "500");
    assert_eq!(metric(1_000), "1 k");
    assert_eq!(metric(1_500), "1.5 k");
    assert_eq!(metric(1_000_000), "1 M");
    assert_eq!(metric(2_200_000), "2.2 M");
    assert_eq!(metric(1_000_000_000_i64), "1 G");
    assert_eq!(metric(1_000_000_000_000_i64), "1 T");
}

// ---------------------------------------------------------------------------
// ordinal()
// ---------------------------------------------------------------------------

#[test]
fn ordinal_basic_suffixes() {
    assert_eq!(ordinal(1), "1st");
    assert_eq!(ordinal(2), "2nd");
    assert_eq!(ordinal(3), "3rd");
    assert_eq!(ordinal(4), "4th");
    assert_eq!(ordinal(0), "0th");
}

#[test]
fn ordinal_teens_are_all_th() {
    assert_eq!(ordinal(11), "11th");
    assert_eq!(ordinal(12), "12th");
    assert_eq!(ordinal(13), "13th");
}

#[test]
fn ordinal_compound() {
    assert_eq!(ordinal(21), "21st");
    assert_eq!(ordinal(22), "22nd");
    assert_eq!(ordinal(23), "23rd");
    assert_eq!(ordinal(100), "100th");
    assert_eq!(ordinal(101), "101st");
    assert_eq!(ordinal(111), "111th");
    assert_eq!(ordinal(113), "113th");
}

#[test]
fn ordinal_negative_keeps_sign() {
    assert_eq!(ordinal(-1), "-1st");
    assert_eq!(ordinal(-11), "-11th");
}

#[test]
fn ordinal_accepts_various_integer_types() {
    assert_eq!(ordinal(1_u8), "1st");
    assert_eq!(ordinal(2_u64), "2nd");
    assert_eq!(ordinal(3_i128), "3rd");
}

// ---------------------------------------------------------------------------
// parse()
// ---------------------------------------------------------------------------

#[test]
fn parse_compact_suffixes() {
    assert_eq!(parse("1.2K").unwrap(), 1_200.0);
    assert_eq!(parse("3M").unwrap(), 3_000_000.0);
    assert_eq!(parse("-2.5bn").unwrap(), -2_500_000_000.0);
    assert_eq!(parse("1.2k").unwrap(), 1_200.0);
}

#[test]
fn parse_words() {
    assert_eq!(parse("1.5 million").unwrap(), 1_500_000.0);
    assert_eq!(parse("5 thousand").unwrap(), 5_000.0);
    assert_eq!(parse("2 BILLION").unwrap(), 2_000_000_000.0);
}

#[test]
fn parse_plain_and_separators() {
    assert_eq!(parse("1000").unwrap(), 1_000.0);
    assert_eq!(parse("2.5").unwrap(), 2.5);
    assert_eq!(parse("1,234").unwrap(), 1_234.0);
    assert_eq!(parse("1_000").unwrap(), 1_000.0);
    assert_eq!(parse("  7 B ").unwrap(), 7_000_000_000.0);
}

#[test]
fn parse_errors() {
    assert_eq!(parse(""), Err(ParseError::Empty));
    assert_eq!(parse("   "), Err(ParseError::Empty));
    assert!(matches!(parse("abc"), Err(ParseError::InvalidNumber(_))));
    assert!(matches!(
        parse("5 zorps"),
        Err(ParseError::UnknownSuffix(_))
    ));
}

#[test]
fn parse_round_trips_compact() {
    for &v in &[1_234.0_f64, 56_700.0, 8_900_000.0, 12_000_000_000.0] {
        let s = compact(v);
        let back = parse(&s).unwrap();
        // Compact keeps ~2 significant digits, so allow 5% tolerance.
        let rel_err = (back - v).abs() / v;
        assert!(rel_err < 0.05, "round trip {v} -> {s} -> {back}");
    }
}

// ---------------------------------------------------------------------------
// Formatter configuration
// ---------------------------------------------------------------------------

#[test]
fn formatter_precision() {
    assert_eq!(Formatter::compact().precision(2).format(1_234_567), "1.23M");
    assert_eq!(Formatter::compact().precision(0).format(1_234), "1K");
    assert_eq!(Formatter::compact().precision(3).format(1_234), "1.234K");
}

#[test]
fn formatter_trim_zeros_off() {
    assert_eq!(
        Formatter::compact().trim_zeros(false).format(1_000_000),
        "1.0M"
    );
    assert_eq!(Formatter::compact().trim_zeros(false).format(1_000), "1.0K");
}

#[test]
fn formatter_space_toggle() {
    assert_eq!(Formatter::compact().space(true).format(1_234), "1.2 K");
    assert_eq!(Formatter::word().space(false).format(1_000_000), "1million");
}

#[test]
fn formatter_sign_plus() {
    assert_eq!(Formatter::compact().sign_plus(true).format(1_500), "+1.5K");
    assert_eq!(Formatter::compact().sign_plus(true).format(-1_500), "-1.5K");
}

#[test]
fn formatter_new_from_notation_matches_presets() {
    assert_eq!(Formatter::new(Notation::Compact), Formatter::compact());
    assert_eq!(Formatter::new(Notation::Word), Formatter::word());
    assert_eq!(Formatter::new(Notation::Metric), Formatter::metric());
}

// ---------------------------------------------------------------------------
// Non-finite floats
// ---------------------------------------------------------------------------

#[test]
fn non_finite_floats() {
    assert_eq!(compact(f64::NAN), "NaN");
    assert_eq!(compact(f64::INFINITY), "inf");
    assert_eq!(compact(f64::NEG_INFINITY), "-inf");
}

// ---------------------------------------------------------------------------
// Regression tests from the adversarial pre-publish review
// ---------------------------------------------------------------------------

#[test]
fn precision_extremes_do_not_panic() {
    // core::fmt stores precision as u16; large precision must clamp, not panic.
    let huge = Formatter::compact().precision(100_000).format(1_234);
    let max = Formatter::compact().precision(65_535).format(1_234);
    assert_eq!(huge, max);
    assert!(huge.starts_with("1.2"));
    // usize::MAX must also be safe.
    let _ = Formatter::compact().precision(usize::MAX).format(1_234);
}

#[test]
fn negative_zero_renders_without_sign() {
    assert_eq!(compact(-0.0_f64), "0");
    assert_eq!(word(-0.0_f64), "0");
    assert_eq!(metric(-0.0_f64), "0");
    // trim_zeros(false) keeps the decimals but must still carry no sign.
    assert_eq!(
        Formatter::compact().trim_zeros(false).format(-0.0_f64),
        "0.0"
    );
}

#[test]
fn small_negative_rounding_to_zero_has_no_sign() {
    assert_eq!(compact(-0.04_f64), "0");
    assert_eq!(Formatter::compact().sign_plus(true).format(-0.0_f64), "0");
}

#[test]
fn compact_beyond_table_uses_scientific_and_round_trips() {
    let s = compact(1e36_f64);
    assert!(!s.contains("1000"), "unexpected overflow rendering: {s}");
    let back = parse(&s).unwrap();
    assert!((back - 1e36).abs() / 1e36 < 1e-9, "{s} -> {back}");
    assert_eq!(compact(-1e36_f64), alloc_minus(&s));
}

fn alloc_minus(s: &str) -> String {
    let mut out = String::from("-");
    out.push_str(s);
    out
}

#[test]
fn parse_rejects_malformed_numbers_as_invalid() {
    for s in ["1.2.3", "1k1", "1 2", "0x10", "1.2.3K"] {
        assert!(
            matches!(parse(s), Err(ParseError::InvalidNumber(_))),
            "input {s:?} -> {:?}",
            parse(s)
        );
    }
}

#[test]
fn parse_unknown_alpha_suffix_is_unknown_suffix() {
    assert!(matches!(parse("12zz"), Err(ParseError::UnknownSuffix(_))));
    assert!(matches!(
        parse("5 zorps"),
        Err(ParseError::UnknownSuffix(_))
    ));
}

#[test]
fn metric_round_trips_through_parse() {
    for p in [1e3, 1e6, 1e9, 1e12, 1e15, 1e21, 1e24, 1e27, 1e30] {
        let s = metric(p);
        let back = parse(&s).unwrap();
        assert!(
            (back - p).abs() / p < 1e-9,
            "metric round trip {p} -> {s} -> {back}"
        );
    }
}

#[test]
fn metric_exa_fails_loudly_not_silently() {
    // SI exa "E" clashes with exponent notation, so it is not parseable — but it
    // must return an error, never silently mis-parse to a wrong value.
    let s = metric(1e18_f64);
    assert!(parse(&s).is_err(), "expected parse error for {s:?}");
}

#[test]
fn defaults_match_compact_preset() {
    assert_eq!(Formatter::default(), Formatter::compact());
    assert_eq!(Notation::default(), Notation::Compact);
}
