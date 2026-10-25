//! Dispersion study: repeats the JCGM 101 clause 9 examples with independent
//! seeds and reports the standard deviation of each MCM output (estimate,
//! u(y), interval endpoints and length). Used to set the tolerances of the
//! validation tests at about five standard deviations, and reported in
//! docs/research/validation-matrix.md.
//!
//!     cargo run --release --example mcm_dispersion -- [repeats] [trials]

use uncertainty_core::mcm::{self, McmOptions, Trials};
use uncertainty_core::{Distribution, Model};

fn comparison_loss(x1: f64, r: f64) -> Model {
    let mut b = Model::builder()
        .quantity("X1", Distribution::Normal { mean: x1, sd: 0.005 })
        .quantity("X2", Distribution::Normal { mean: 0.0, sd: 0.005 })
        .output("dY", "1e6 * (X1^2 + X2^2)");
    if r != 0.0 {
        b = b.correlation("X1", "X2", r);
    }
    b.build().unwrap()
}

fn mass() -> Model {
    Model::builder()
        .quantity("m_Rc", Distribution::Normal { mean: 100_000.000, sd: 0.050 })
        .quantity("dm_Rc", Distribution::Normal { mean: 1.234, sd: 0.020 })
        .quantity("rho_a", Distribution::rectangular_half_width(1.20, 0.10))
        .quantity("rho_W", Distribution::rectangular_half_width(8e3, 1e3))
        .quantity("rho_R", Distribution::rectangular_half_width(8.00e3, 0.05e3))
        .output("dm", "(m_Rc + dm_Rc) * (1 + (rho_a - 1.2) * (1/rho_W - 1/rho_R)) - 100000")
        .build()
        .unwrap()
}

fn sd(v: &[f64]) -> f64 {
    let m = v.iter().sum::<f64>() / v.len() as f64;
    (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0)).sqrt()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let repeats: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(20);
    let trials: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2_000_000);
    let cases: Vec<(&str, Model)> = vec![
        ("mass calibration [mg]", mass()),
        ("comparison loss x1=0.000 r=0 [1e-6]", comparison_loss(0.0, 0.0)),
        ("comparison loss x1=0.010 r=0 [1e-6]", comparison_loss(0.010, 0.0)),
        ("comparison loss x1=0.050 r=0 [1e-6]", comparison_loss(0.050, 0.0)),
        ("comparison loss x1=0.000 r=0.9 [1e-6]", comparison_loss(0.0, 0.9)),
        ("comparison loss x1=0.010 r=0.9 [1e-6]", comparison_loss(0.010, 0.9)),
        ("comparison loss x1=0.050 r=0.9 [1e-6]", comparison_loss(0.050, 0.9)),
    ];
    println!("M = {trials}, {repeats} repeats; standard deviation over repeats:");
    println!(
        "{:<40} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "case", "y", "u", "short lo", "short hi", "short len", "sym lo", "sym hi"
    );
    for (name, model) in &cases {
        let mut cols: Vec<Vec<f64>> = vec![Vec::new(); 7];
        for seed in 0..repeats as u64 {
            let r = mcm::evaluate(
                model,
                &McmOptions { trials: Trials::Fixed(trials), seed: 1000 + seed, ..Default::default() },
            )
            .unwrap();
            let vals = [
                r.estimate,
                r.standard_uncertainty,
                r.shortest_interval.0,
                r.shortest_interval.1,
                r.shortest_interval.1 - r.shortest_interval.0,
                r.symmetric_interval.0,
                r.symmetric_interval.1,
            ];
            for (c, v) in cols.iter_mut().zip(vals) {
                c.push(v);
            }
        }
        print!("{name:<40}");
        for c in &cols {
            print!(" {:>10.3e}", sd(c));
        }
        println!();
    }
}
