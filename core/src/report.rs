//! Rounding and formatting of results for reports.
//!
//! * GUM 7.2.6: u_c(y) and U should be given with at most two significant
//!   digits; rounding uncertainties *up* may be appropriate.
//! * Eurachem/CITAC MUPR (2026) §12: at most two significant figures for U;
//!   the measured value is given to the same decimal place; a discarded 5 is
//!   rounded to the even neighbour, and figures beyond the first discarded one
//!   are not considered.
//!
//! * EA-4/02 M:2022 §6.3 (calibration certificates): U is normally rounded
//!   to two significant digits by ordinary rounding, but it is rounded up
//!   when ordinary rounding would decrease it by more than 5 %.
//!
//! Rounding works on the decimal representation of the number (not on binary
//! floating-point arithmetic), so 0.0123 really is treated as 0.0123.

/// How the last kept digit is chosen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rounding {
    /// MUPR §12: look only at the first discarded digit; below 5 down, above
    /// 5 up, exactly 5 to the even neighbour.
    HalfEvenFirstDiscarded,
    /// Conventional: round half away from zero using all discarded digits.
    HalfUp,
    /// Round up (towards +∞ for positive values) whenever any discarded digit
    /// is non-zero — conservative rounding of uncertainties (GUM 7.2.6;
    /// practice of Nordtest TR 537).
    Up,
    /// EA-4/02 §6.3: ordinary rounding (half up) unless it lowers the value
    /// by more than 5 %; then the value is rounded up. Used for uncertainties
    /// only.
    UpIfMoreThanFivePercentLower,
}

/// Decimal digits and exponent: value = 0.d₁d₂d₃… × 10^exp, d₁ ≠ 0.
fn decimal_digits(x: f64) -> (Vec<u8>, i32) {
    // 17 significant digits round-trip any f64; trailing noise is trimmed
    // by using the shortest representation that parses back exactly.
    let s = format!("{:e}", x.abs());
    let (mantissa, exp) = s.split_once('e').expect("exponent form");
    let exp: i32 = exp.parse().expect("integer exponent");
    let digits: Vec<u8> = mantissa.bytes().filter(u8::is_ascii_digit).map(|b| b - b'0').collect();
    (digits, exp + 1)
}

/// Rounds |x| to the decimal place 10^place (e.g. place = −3 → 0.001) and
/// returns the integer count of units of 10^place.
fn round_to_place_units(x: f64, place: i32, rule: Rounding) -> u128 {
    if x == 0.0 {
        return 0;
    }
    let (digits, exp) = decimal_digits(x);
    // Number of digits kept: those with weight ≥ 10^place.
    let keep = exp - place;
    if keep < 0 {
        // Everything is below half a unit except for `Up`.
        return match rule {
            Rounding::Up | Rounding::UpIfMoreThanFivePercentLower => 1,
            _ => 0,
        };
    }
    let keep = keep as usize;
    let mut units: u128 = 0;
    for i in 0..keep {
        units = units * 10 + *digits.get(i).unwrap_or(&0) as u128;
    }
    let first = *digits.get(keep).unwrap_or(&0);
    let rest_nonzero = digits.iter().skip(keep + 1).any(|&d| d != 0);
    let round_up = match rule {
        Rounding::HalfEvenFirstDiscarded => first > 5 || (first == 5 && units % 2 == 1),
        Rounding::HalfUp => first >= 5,
        Rounding::Up => first > 0 || rest_nonzero,
        Rounding::UpIfMoreThanFivePercentLower => {
            let kept = units as f64 * 10f64.powi(place);
            first >= 5 || x.abs() - kept > 0.05 * x.abs() * (1.0 + 1e-12)
        }
    };
    if round_up {
        units += 1;
    }
    units
}

/// Rounds x to the decimal place 10^place.
pub fn round_to_place(x: f64, place: i32, rule: Rounding) -> f64 {
    let units = round_to_place_units(x, place, rule) as f64;
    let v = if place >= 0 { units * 10f64.powi(place) } else { units / 10f64.powi(-place) };
    if x < 0.0 {
        -v
    } else {
        v
    }
}

/// Decimal place of the last significant digit when x is rounded to `sig`
/// significant figures (accounting for a carry, e.g. 0.0996 → 0.10).
pub fn significant_place(x: f64, sig: u32, rule: Rounding) -> i32 {
    assert!(sig >= 1);
    let (_, exp) = decimal_digits(x);
    let place = exp - sig as i32;
    let units = round_to_place_units(x, place, rule);
    if units >= 10u128.pow(sig) {
        place + 1
    } else {
        place
    }
}

/// Rounds x to `sig` significant figures.
pub fn round_significant(x: f64, sig: u32, rule: Rounding) -> f64 {
    if x == 0.0 || !x.is_finite() {
        return x;
    }
    round_to_place(x, significant_place(x, sig, rule), rule)
}

/// Formats x rounded to the decimal place 10^place.
pub fn format_at_place(x: f64, place: i32, rule: Rounding) -> String {
    let v = round_to_place(x, place, rule);
    let decimals = if place < 0 { (-place) as usize } else { 0 };
    format!("{v:.decimals$}")
}

/// A measured value and expanded uncertainty rounded for reporting.
#[derive(Clone, Debug, PartialEq)]
pub struct ReportedResult {
    pub value: String,
    pub uncertainty: String,
    /// Decimal place of the last reported digit.
    pub place: i32,
}

/// Rounds U to `sig` significant figures with `uncertainty_rule` and the
/// value to the same decimal place with `value_rule` [GUM 7.2.6; MUPR §12].
pub fn report_result(
    value: f64,
    expanded_uncertainty: f64,
    sig: u32,
    uncertainty_rule: Rounding,
    value_rule: Rounding,
) -> ReportedResult {
    let place = significant_place(expanded_uncertainty, sig, uncertainty_rule);
    ReportedResult {
        value: format_at_place(value, place, value_rule),
        uncertainty: format_at_place(expanded_uncertainty, place, uncertainty_rule),
        place,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Rounding::*;

    #[test]
    fn mupr_table_2_examples() {
        // c = 0.1559, U = 0.0123 mg/L → (0.156 ± 0.012) mg/L
        let r = report_result(0.1559, 0.0123, 2, HalfEvenFirstDiscarded, HalfEvenFirstDiscarded);
        assert_eq!((r.value.as_str(), r.uncertainty.as_str()), ("0.156", "0.012"));
        // c = 5364.9, U = 235.9 µg/L → (5360 ± 240) µg/L: third figure 5,
        // fourth not considered, 3 is odd → up.
        let r = report_result(5364.9, 235.9, 2, HalfEvenFirstDiscarded, HalfEvenFirstDiscarded);
        assert_eq!((r.value.as_str(), r.uncertainty.as_str()), ("5360", "240"));
        // c = 5364.9, U = 264.2 µg/L → (5360 ± 260) µg/L.
        let r = report_result(5364.9, 264.2, 2, HalfEvenFirstDiscarded, HalfEvenFirstDiscarded);
        assert_eq!((r.value.as_str(), r.uncertainty.as_str()), ("5360", "260"));
        // The rule ignores the fourth figure: 245.9 → 240 (4 even), whereas
        // conventional rounding gives 250.
        assert_eq!(round_significant(245.9, 2, HalfEvenFirstDiscarded), 240.0);
        assert_eq!(round_significant(245.9, 2, HalfUp), 250.0);
    }

    #[test]
    fn rounding_up_as_in_nordtest() {
        // Nordtest TR 537: 2 × 3.20 = 6.40 → 7 %; 10.4 → 11 %; 21.6 → 22 %;
        // 9.74 → 10 %; 17.6 → 18 % (whole per cent).
        for (x, expected) in [(6.40, 7.0), (10.4, 11.0), (21.6, 22.0), (9.74, 10.0), (17.6, 18.0)] {
            assert_eq!(round_to_place(x, 0, Up), expected);
        }
        // Exact values are not rounded up.
        assert_eq!(round_to_place(7.0, 0, Up), 7.0);
    }

    #[test]
    fn decimal_rather_than_binary_rounding() {
        // 0.285 is 0.28499999999999998 in binary; decimal rounding sees "285".
        assert_eq!(round_significant(0.285, 2, HalfUp), 0.29);
        assert_eq!(round_significant(0.285, 2, HalfEvenFirstDiscarded), 0.28);
        // Carry into a new digit.
        assert_eq!(significant_place(0.0996, 1, HalfUp), -1);
        assert_eq!(format_at_place(0.0996, -1, HalfUp), "0.1");
        assert_eq!(round_significant(-1.25, 2, HalfUp), -1.3);
    }

    #[test]
    fn ea4_02_five_per_cent_rule() {
        let r = UpIfMoreThanFivePercentLower;
        // EA-4/02 S2: U = 58.52 mg → 59 mg (MUPR rule gives 58 mg).
        assert_eq!(round_significant(58.52, 2, r), 59.0);
        assert_eq!(round_significant(58.52, 2, HalfEvenFirstDiscarded), 58.0);
        // 0.10599 → 0.11 (the MUPR rule gives 0.10, 5.7 % lower).
        assert_eq!(round_significant(0.10599, 2, r), 0.11);
        assert_eq!(round_significant(0.10599, 2, HalfEvenFirstDiscarded), 0.10);
        // Ordinary rounding down within 5 % is kept: 1.728 → 1.7.
        assert_eq!(round_significant(1.728, 2, r), 1.7);
        // One significant digit: 1.4 → 2 (1 would be 29 % lower); 1.04 → 1.
        assert_eq!(round_significant(1.4, 1, r), 2.0);
        assert_eq!(round_significant(1.04, 1, r), 1.0);
    }
}
