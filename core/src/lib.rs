//! # uncertainty-core
//!
//! Evaluation of measurement uncertainty according to
//!
//! * JCGM 100:2008 — *Guide to the expression of uncertainty in measurement*
//!   (GUM uncertainty framework, module [`gum`]);
//! * JCGM 101:2008 — *Supplement 1: Propagation of distributions using a Monte
//!   Carlo method* (module [`mcm`]);
//! * top-down evaluation from validation and quality-control data —
//!   Nordtest TR 537 and Eurachem/CITAC MUPR (2026) (module [`topdown`]);
//! * straight-line calibration and inverse prediction — EURACHEM CG4
//!   Appendix E.4 (module [`calibration`]);
//! * statistics for proficiency testing — ISO 13528:2022 (module [`pt`]);
//! * repeatability and reproducibility from interlaboratory experiments —
//!   ISO 5725-2:2019 (module [`interlab`]);
//! * rounding of reported results — GUM 7.2.6, MUPR §12 (module [`report`]).
//!
//! A measurement model is built from input quantities with assigned
//! distributions ([`distribution`]) and a measurement function written as a
//! text expression ([`expr`]), e.g.
//!
//! ```
//! use uncertainty_core::{Model, Distribution, gum, mcm};
//!
//! // EURACHEM/CITAC CG4 (2012) Example A1: c = 1000 m P / V
//! let model = Model::builder()
//!     .quantity("m", Distribution::Normal { mean: 100.28, sd: 0.05 })
//!     .quantity("P", Distribution::rectangular_half_width(0.9999, 0.0001))
//!     .quantity("V", Distribution::Normal { mean: 100.0, sd: 0.07 })
//!     .output("c", "1000 * m * P / V")
//!     .build()
//!     .unwrap();
//! let g = gum::evaluate(&model, &gum::GumOptions::default()).unwrap();
//! assert!((g.estimate - 1002.7).abs() < 0.05);
//! assert!((g.combined_standard_uncertainty - 0.86).abs() < 0.01);
//! ```
//!
//! Every algorithm cites the clause of the document it implements; the
//! reference examples used for validation are listed in
//! `docs/research/validation-matrix.md`.

// `!(x > 0.0)` is used deliberately so that NaN parameters are rejected;
// index loops mirror the subscripts of the formulas in the standards.
#![allow(clippy::neg_cmp_op_on_partial_ord, clippy::needless_range_loop)]

pub mod bias;
pub mod calibration;
pub mod conformity;
pub mod distribution;
pub mod error;
pub mod expr;
pub mod gls;
pub mod gum;
pub mod interlab;
pub mod mcm;
pub mod model;
pub mod pt;
pub mod report;
pub mod rng;
pub mod special;
pub mod topdown;

pub use distribution::{Distribution, TypeA};
pub use error::UncError;
pub use model::{InputQuantity, Model, ModelBuilder};
