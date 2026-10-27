# 4. The Monte Carlo method

JCGM 101:2008 (GUM Supplement 1) replaces the propagation of *uncertainties* by the propagation of
*distributions*: values are drawn at random from the probability distribution of every input, the model is
evaluated for each draw, and the resulting values of the output describe its probability distribution. No
linearisation, no assumption of a normal output and no effective degrees of freedom are needed.

## 4.1 Steps (JCGM 101 clause 7)

1. Assign a probability distribution to every input quantity (the same information as in chapter 3; JCGM 101
   clause 6 gives the rules).
2. Choose the number of trials M.
3. Draw M vectors of input values; correlated Gaussian inputs are drawn jointly (Cholesky factor of the
   covariance matrix).
4. Evaluate the model for each vector.
5. From the M output values compute the estimate (mean), the standard uncertainty (standard deviation) and a
   coverage interval for the required coverage probability.

The program uses the random-number generator recommended by JCGM 101 (enhanced Wichmann–Hill, Annex C.3),
the Box–Muller transform for Gaussian values and the generators of Annex C for the other distributions. Every
run is **reproducible**: the seed (`"seed"`, default 1) and the number of trials are printed in the report.

### Type A inputs in the Monte Carlo method

For n observations JCGM 101 (6.4.9) assigns a scaled and shifted t-distribution with n − 1 degrees of
freedom, mean x̄ and scale s/√n. Its standard deviation is (s/√n)·√((n − 1)/(n − 3)), larger than s/√n, and for
n ≤ 3 its variance is infinite: the Monte Carlo standard deviation is then unstable although the coverage
interval is well defined (see EA-4/02 S6 in the library).

NIST TN 1900 uses another convention: it rescales the t-distribution so that its standard deviation equals
s/√n (scale s/√n · √((n − 3)/(n − 1))). The two conventions give different Monte Carlo results for the same
data. To reproduce TN 1900, enter the rescaled scale in a `t` input; since the GUM framework takes the scale
of a `t` input as its standard uncertainty, such projects run the Monte Carlo method only (TN 1900 E11 and
E13 in the library).

### Correlated inputs

Correlations are supported for Gaussian inputs. The program refuses to propagate correlations between
non-Gaussian inputs instead of silently ignoring them (JCGM 101 gives no general method; model the common
effect explicitly instead).

## 4.2 Number of trials

* **Fixed:** `"trials": 1000000` (the default). JCGM 101 7.2.2 notes that M = 10⁶ can often be expected to
  give a 95 % coverage interval correct to one or two significant decimal digits.
* **Adaptive** (JCGM 101 7.9): `"adaptive": { "ndig": 2 }`. Trials are added in batches until the standard
  deviations of the batch means of the estimate, the standard uncertainty and both interval endpoints are all
  below δ/5 (with `"tolerance_divisor": 5`; the default 1 corresponds to 7.9.4 as printed), where δ is the
  numerical tolerance of u(y) to `ndig` significant digits. The report states whether the procedure converged
  before `max_trials`.

The Monte Carlo results carry sampling noise. In the validation of the program, tolerances for Monte Carlo
values are about five standard deviations of this noise, measured by repeated runs
(`core/examples/mcm_dispersion.rs`).

## 4.3 Coverage intervals

* **Probabilistically symmetric** interval: the 2.5 % and 97.5 % quantiles for p = 95 % (JCGM 101 7.7.2).
* **Shortest** interval: the shortest of all intervals containing the fraction p of the values. For an
  asymmetric distribution it differs from the symmetric one — for a quantity bounded below by zero (JCGM 101
  9.4, comparison loss) its lower end can be zero.

Choose with `"interval": "shortest"` (default) or `"symmetric"`. For a symmetric output distribution both
estimate the same interval, but the symmetric one has much less sampling noise; the library uses it where the
distribution is known to be symmetric (JCGM 101 9.2, 9.3).

By default the coverage probability of the Monte Carlo method equals that of the GUM result (95 % with the
t-distribution, 95.45 % with k = 2), so that both can be compared.

## 4.4 Validation of the GUM result (JCGM 101 clause 8)

The GUM result is **validated** if the endpoints of its interval y ± U differ from the endpoints of the Monte
Carlo interval by no more than the numerical tolerance δ:

* write u(y) with n_dig significant digits as c × 10ˡ; then δ = ½ × 10ˡ;
* d_low = |y − U − y_low|, d_high = |y + U − y_high|;
* validated if d_low ≤ δ and d_high ≤ δ.

n_dig is set by `"validation_ndig"` (default 2). The report prints δ, d_low, d_high and the verdict. Near the
threshold the verdict is itself subject to Monte Carlo noise — JCGM 101 9.2.3 obtains "validated" in one run and
"not validated" in another for the same model.

## 4.5 When the GUM framework fails

The library contains the classical cases:

| Situation | Example | What happens |
|---|---|---|
| one dominant rectangular input | JCGM 101 9.2.4; EA-4/02 S9, S10 | the output is nearly rectangular; ±2u is too wide for 95 % |
| sensitivity coefficients zero at the estimate | JCGM 101 9.3 (mass), 9.4 (comparison loss) | first-order GUM misses most of the uncertainty |
| strongly asymmetric output | JCGM 101 9.4 | symmetric ±U is meaningless; the shortest interval starts at zero |
| large relative uncertainties in a quotient | QUAM A4 | the output is skewed; mean and median differ |

In all these cases the Monte Carlo result is the one to report; the GUM result is useful as a check of the
model and to read the budget.
