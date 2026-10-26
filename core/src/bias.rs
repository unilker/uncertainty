//! Expanded uncertainty of a result that is not corrected for a known bias
//! [Magnusson & Ellison, Anal. Bioanal. Chem. 390 (2008) 201, Table 3].
//!
//! y_obs is the uncorrected result with standard uncertainty u(y_obs); Δ = −b
//! is the estimated correction (b the estimated bias) with standard
//! uncertainty u_Δ, and u_c = √(u²(y_obs) + u_Δ²) is the standard uncertainty
//! the corrected result y_obs + Δ would have. The interval reported is
//! [y_obs − U₋, y_obs + U₊].

use crate::error::UncError;
use crate::special::{normal_cdf, normal_quantile};

/// Strategies of Magnusson & Ellison Table 3 for an uncorrected result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BiasStrategy {
    /// U = k√(u²(y_obs) + Δ² + u_Δ²) (Lira & Wöger; Nordtest; NIST).
    RssU,
    /// U = k√(u²(y_obs) + (Δ/k)² + u_Δ²) (IUPAC; Barwick & Ellison).
    RssUk,
    /// U₊ = max(0, ku_c + Δ), U₋ = max(0, ku_c − Δ) (Phillips, Eberhardt & Parry, J. Res. NIST 102 (1997) 577).
    Sumu,
    /// U = ku_c + |Δ| (IUPAC; Maroto et al.; GUM F.2.4.5).
    SumuMax,
    /// Symmetric interval that contains the fraction p = 2Φ(k) − 1 of the
    /// distribution N(y_obs + Δ, u_c²) of the corrected value (Synek).
    Ue,
}

/// Half-widths (U₋, U₊) of the interval around the uncorrected result.
pub fn uncorrected_interval(
    u_obs: f64,
    delta: f64,
    u_delta: f64,
    k: f64,
    s: BiasStrategy,
) -> Result<(f64, f64), UncError> {
    if !(u_obs >= 0.0 && u_delta >= 0.0 && k > 0.0 && delta.is_finite() && u_obs.is_finite() && u_delta.is_finite()) {
        return Err(UncError::InvalidOption(
            "uncorrected bias: finite bias, non-negative uncertainties and k > 0 required".into(),
        ));
    }
    let uc = (u_obs * u_obs + u_delta * u_delta).sqrt();
    let sym = |u: f64| Ok((u, u));
    match s {
        BiasStrategy::RssU => sym(k * (u_obs * u_obs + delta * delta + u_delta * u_delta).sqrt()),
        BiasStrategy::RssUk => sym(k * (u_obs * u_obs + (delta / k).powi(2) + u_delta * u_delta).sqrt()),
        BiasStrategy::Sumu => Ok(((k * uc - delta).max(0.0), (k * uc + delta).max(0.0))),
        BiasStrategy::SumuMax => sym(k * uc + delta.abs()),
        BiasStrategy::Ue => sym(exact_coverage_half_width(delta.abs(), uc, 2.0 * normal_cdf(k) - 1.0)),
    }
}

/// Smallest U with Φ((U − d)/u) − Φ((−U − d)/u) = p (bisection; d ≥ 0).
fn exact_coverage_half_width(d: f64, u: f64, p: f64) -> f64 {
    if u == 0.0 {
        return d;
    }
    let cover = |h: f64| normal_cdf((h - d) / u) - normal_cdf((-h - d) / u);
    let (mut lo, mut hi) = (0.0, d + u * normal_quantile(0.5 + p / 2.0).max(1.0) + u);
    while cover(hi) < p {
        hi *= 2.0;
    }
    for _ in 0..64 {
        let mid = 0.5 * (lo + hi);
        if cover(mid) < p {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn formulae_of_table_3() {
        let (u, d, ud, k) = (1.0, 0.5, 0.5, 2.0);
        let uc = 1.25f64.sqrt();
        assert_eq!(uncorrected_interval(u, d, ud, k, BiasStrategy::RssU).unwrap().0, 2.0 * 1.5f64.sqrt());
        assert_eq!(uncorrected_interval(u, d, ud, k, BiasStrategy::RssUk).unwrap().0, 2.0 * (1.25f64 + 0.0625).sqrt());
        assert_eq!(uncorrected_interval(u, d, ud, k, BiasStrategy::SumuMax).unwrap().0, 2.0 * uc + 0.5);
        // The correction Δ > 0 moves the interval up (Phillips, Eberhardt & Parry 1997: U₊ = U − b).
        let (lo, hi) = uncorrected_interval(u, d, ud, k, BiasStrategy::Sumu).unwrap();
        assert!((hi - (2.0 * uc + 0.5)).abs() < 1e-15 && (lo - (2.0 * uc - 0.5)).abs() < 1e-15);
        // Ue: 2Φ(k) − 1 of the corrected-value distribution; at Δ = 0 it is k·u_c.
        let (e0, _) = uncorrected_interval(u, 0.0, ud, k, BiasStrategy::Ue).unwrap();
        assert!((e0 - 2.0 * uc).abs() < 1e-9);
        // For |Δ| > 0.8 u_c Magnusson & Ellison give U ≈ 1.65 u_c + |Δ| (p = 95 %).
        for r in [0.8, 1.0, 2.0, 4.0] {
            let (e, _) = uncorrected_interval(u, r * uc, ud, 1.959964, BiasStrategy::Ue).unwrap();
            assert!(((e - (1.645 * uc + r * uc)) / e).abs() < 0.03, "{r}: {e}");
        }
    }

    /// Interval widths of Magnusson & Ellison Fig. 2: u(y_obs) = 1, u_Δ = 0.3.
    #[test]
    fn widths_of_fig_2() {
        let w = |b: f64, s| uncorrected_interval(1.0, -b, 0.3, 2.0, s).unwrap().0;
        for b in [0.5, 1.0] {
            let (rss_k, rss, ue, max) = (
                w(b, BiasStrategy::RssUk),
                w(b, BiasStrategy::RssU),
                w(b, BiasStrategy::Ue),
                w(b, BiasStrategy::SumuMax),
            );
            assert!(rss_k < ue && ue < max && rss_k < rss && rss < max, "{b}: {rss_k} {rss} {ue} {max}");
            // "the uncertainty interval is similar to that for Ue" for |b| ≤ u(y_obs).
            assert!((rss - ue).abs() / ue < 0.1, "{b}: {rss} {ue}");
        }
        // For large bias RSSu becomes disproportionately conservative.
        assert!(w(2.0, BiasStrategy::RssU) > w(2.0, BiasStrategy::SumuMax));
    }

    /// Coverage simulation of Magnusson & Ellison Fig. 1: true value 0, true
    /// bias B, y_obs ~ N(B, 1), estimated bias b ~ N(B, 0.5²), Δ = −b,
    /// nominal 95 % (k = 1.96).
    #[test]
    fn coverage_of_fig_1() {
        let (u, ud, k) = (1.0f64, 0.5f64, 1.959964);
        let uc = (u * u + ud * ud).sqrt();
        let mut rng = Rng::new(2008);
        let n = 20_000;
        let coverage = |rng: &mut Rng, bias: f64, s: BiasStrategy| {
            let mut hit = 0;
            for _ in 0..n {
                let y = bias + u * rng.normal();
                let delta = -(bias + ud * rng.normal());
                let (lo, hi) = uncorrected_interval(u, delta, ud, k, s).unwrap();
                if y - lo <= 0.0 && 0.0 <= y + hi {
                    hit += 1;
                }
            }
            hit as f64 / n as f64
        };
        let se = 3.0 * (0.05f64 * 0.95 / n as f64).sqrt();
        // SUMU: about 95 % at zero bias, about 97.5 % at 3u_c.
        assert!((coverage(&mut rng, 0.0, BiasStrategy::Sumu) - 0.95).abs() < se + 0.003);
        assert!((coverage(&mut rng, 3.0 * uc, BiasStrategy::Sumu) - 0.975).abs() < 0.01);
        // Ue: about 98 % at zero bias, close to 95 % from about u_c on.
        let c0 = coverage(&mut rng, 0.0, BiasStrategy::Ue);
        assert!(c0 > 0.97 && c0 < 0.99, "{c0}");
        assert!((coverage(&mut rng, 3.0 * uc, BiasStrategy::Ue) - 0.95).abs() < 0.01);
        // RSSu and SUMU_MAX: consistently above 95 %.
        for b in [0.0, 1.0, 3.0] {
            assert!(coverage(&mut rng, b * uc, BiasStrategy::RssU) > 0.95);
            assert!(coverage(&mut rng, b * uc, BiasStrategy::SumuMax) > 0.95);
        }
    }
}
