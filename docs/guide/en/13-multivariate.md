# 13. Several output quantities (JCGM 102)

Many measurements give more than one result from the same input quantities. Examples are the resistance and
reactance of an impedance, the magnitude and phase of a complex quantity, and several temperatures measured
with one calibrated thermometer. Their estimates are correlated, and a later calculation that uses them needs
the covariances as well as the standard uncertainties. The `multivariate` section implements JCGM 102:2011
(GUM Supplement 2) for this purpose. It is validated against the examples of clauses 6, 7 and 9 of that
Supplement (library projects `jcgm102-*`).

## 13.1 Models

- **Explicit model** Y = f(X): each output has its own measurement function, as in `model`.
- **Implicit model** h(Y, X) = 0 (`"implicit": true`): each output has an equation in the inputs and the
  outputs that equals zero, together with a starting value. The program solves the equations by Newton's
  method, using exact derivatives. An example is the resistance thermometer, R₀(1 + Aθ + Bθ²) − r R_S = 0,
  which need not be rewritten as a formula for θ (9.5.2.4 note).

Input quantities are specified as in a model project. In addition, inputs can come from a series of n sets
of simultaneous indication values (`indications`, 5.3.2). Their estimates are the averages. The matrix M of
sums of squares and products gives the covariance matrix:

- for the GUM framework, U_x = M/(n(n − 1)) (`"covariance": "mean"`), or the covariance matrix of the
  multivariate t-distribution, M/((ν − 2)n) with ν = n − N (`"covariance": "t"`, 9.4.2.5);
- for the Monte Carlo method, draws from the multivariate t-distribution t_ν(x̄, S/n) with S = M/ν (5.3.2.4).

The expressions may use `atan2(y, x)`, the four-quadrant inverse tangent. It gives the phase of a complex
quantity in (−π, π] (9.3.1.1 note).

## 13.2 GUM uncertainty framework (clause 6)

U_y = C_x U_x C_xᵀ, where C_x is the m × N matrix of sensitivity coefficients ∂f_j/∂X_i (formula 3). For an
implicit model, C_y U_y C_yᵀ = C_x U_x C_xᵀ with C_y = ∂h/∂Y and C_x = ∂h/∂X (formula 8). The program
obtains the sensitivities ∂Y/∂X = −C_y⁻¹ C_x by solving the linear system, not by inverting C_y (Annex B).

The report gives:

- the estimates and their standard uncertainties;
- the correlation matrix;
- the sensitivity matrix;
- two 95 % coverage regions for the Gaussian distribution N(y, U_y) (6.5):
  - the **hyper-ellipsoid** (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², with k_p² the p-quantile of χ²_m (Table 1:
    2.45 for m = 2);
  - the **hyper-rectangle** y_j ± k_q u(y_j), whose m intervals each have coverage probability
    q = 1 − (1 − p)/m (Table 2: 2.24 for m = 2). This region has coverage probability at least p.

For two outputs the areas of both regions are given as well.

## 13.3 Monte Carlo method (clause 7)

The same draws of the inputs are evaluated for all outputs, so the correlations between the outputs follow
directly from the draws. The estimate and the covariance matrix are the average and the sample covariance
matrix of the draws (7.6). The coverage factors are found as follows:

- **k_p** (7.7.2): each draw is transformed to ẙ_r = L⁻¹(y_r − y), with U_y = LLᵀ. k_p is the distance
  |ẙ_r| that a fraction p of the draws does not exceed.
- **k_q** (7.7.3): the same, with the distance max_j |y_j,r − y_j|/u(y_j).
- **smallest coverage region**, for two outputs (7.7.4): the area is approximated on a 100 × 100 mesh.
- **coverage of the GUM regions**: the fraction of the Monte Carlo draws that falls inside them.

The adaptive procedure (7.8.3) runs at least eleven batches of max(10⁴, 100/(1 − p)) trials each. It stops
when twice the standard deviation of the batch averages is within the numerical tolerance for every result:
the estimates, the standard uncertainties, the largest eigenvalue of the correlation matrix, k_p and k_q.

## 13.4 Validation (clause 8)

When both methods are run with the same coverage probability, the GUM results are compared with the Monte
Carlo results. The quantities compared are y_j, u(y_j), λ_max of the correlation matrix, k_p and k_q. The
tolerances are formed from the Monte Carlo values with `validation_ndig` significant digits (default 2). The
report states separately whether the hyper-ellipsoid and the hyper-rectangle are validated. In the additive
model with a rectangular common effect (9.2.3), for example, the estimates, uncertainties and correlation
agree, but neither coverage region does: the GUM k_p of 2.45 compares with 2.38 from the Monte Carlo method.

The reported result is the Monte Carlo result unless the GUM hyper-ellipsoid is validated.

## 13.5 Points to note

- When an output depends exactly on the others, U_y is singular and no hyper-ellipsoid exists. An example is
  Z² = R² + X² for the impedance (9.4.2.3 note 2). The program then reports the singularity rather than a
  coverage factor.
- With few indication values the multivariate t-distribution has heavy tails. For ν ≤ 4 the Monte Carlo
  standard deviations converge slowly: in 9.4 (ν = 3), different seeds give u(R) between 0.128 Ω and 0.136 Ω
  with 10⁶ trials.
- Near a singularity of the model, the GUM results can be wrong by orders of magnitude, as for the phase of a
  complex quantity close to zero (9.3). Ellipses and rectangles are then not suitable coverage regions
  (9.3.4.4).
- Findings in the Supplement are recorded in the research log (F50, F51):
  - the matrices printed in the 7.7.2 example are the transposes of the Cholesky factor and of its inverse;
  - set 6 of Table 8 is the rounded average of sets 1 to 5.
