# 12. Interlaboratory precision experiments (ISO 5725-2)

The `precision_experiment` section implements the basic method of ISO 5725-2:2019. The method estimates the
repeatability and reproducibility standard deviations of a standard measurement method from an experiment in
which p laboratories each measure q levels (materials) n times. It is validated against all three examples of
Annex C, including the REML alternatives (library projects `iso5725-2-*`), and against Tables 1 to 8 and A.1.

ISO/IEC 17025:2017 7.6.3, Note 3, names the ISO 5725 series as a basis for estimating the uncertainty of test
results. The s_R found here is the starting point of the ISO 21748 approach (chapter 5).

## 12.1 Data: Forms A, B and C

The results are entered by level. Each level gives one list of results per laboratory: this is the *cell* of
that laboratory and level (Form A, 8.2.9). A missing cell is an empty list. From the cells the program
computes:

- the cell means (Form B, 8.2.10);
- the cell standard deviations (Form C, 8.2.11); a cell with a single result has a mean but no standard
  deviation (8.4.3 b).

The forms in the report carry one decimal more than the results. With `form_decimals` the means and standard
deviations are rounded as recorded, and the tests and the variances are computed from the rounded values.
The examples of the standard are worked this way (C.1.6): their test statistics agree to the printed digits
only when the same rounding is used.

## 12.2 Scrutiny for consistency and outliers (8.3)

The program applies all the tests to all the results and reports them:

| Test | Statistic | Critical values |
|---|---|---|
| Mandel's h (8.3.2) | (ȳ_i − m̂)/s(ȳ), by laboratory and level | indicators of Formula D.5 (Tables 7, 8) |
| Mandel's k (8.3.2) | s_i √p / √Σs² | indicators of Formula D.6 (Tables 7, 8) |
| Cochran (8.3.4) | C = s²max / Σs² | Formula D.1 (Table 5), with n the number of results in the majority of cells (8.3.4.3) |
| Grubbs, one value (8.3.5.1) | G = (x_max − x̄)/s and (x̄ − x_min)/s of the cell means | Formula D.2 (Table 6) |
| Grubbs, two values (8.3.5.2) | G = s²(without the two highest or lowest)/s₀² | Table 6 for p ≤ 40; Formula D.3 beyond it |

A statistic beyond its 5 % critical value marks a *straggler* (\*), one beyond its 1 % critical value a
*statistical outlier* (\*\*) (8.3.3.1). Cochran's test is repeated without the highest standard deviation as
long as that value is an outlier (8.3.4.6). The Grubbs procedure of 8.3.5.3 is followed. If a single test finds
an outlier, the other extreme is tested with that value removed, and the double tests are not applied.
Otherwise the double tests are applied.

The program does not remove anything by itself. Removal is a decision of the panel, after a search for a
technical explanation (8.3.3.1 b, 8.6.6–8.6.10). Record each removal in `exclude`, with its reason:

- a whole laboratory (`"laboratory": "1"`);
- one cell (`"level"` as well);
- a single result (`"result"` as well, numbered from 1).

The report then shows the forms with the removed values in brackets and repeats the scrutiny on the retained
results. If statistical outliers are still retained, the report says so: 8.3.3.1 c) discards them unless
there is good reason to keep them.

## 12.3 General mean and precision (8.4)

With `"method": "anova"` (the default) the program uses Formulae (23) to (31):

- m̂ = Σn_i ȳ_i / Σn_i;
- s_r² = Σ(n_i − 1)s_i² / Σ(n_i − 1);
- s_d² = Σn_i(ȳ_i − m̂)²/(p − 1), with n̄ = (Σn_i − Σn_i²/Σn_i)/(p − 1);
- s_L² = (s_d² − s_r²)/n̄, set to zero when negative (8.4.5.4);
- s_R² = s_L² + s_r².

These formulae hold for unequal numbers of results. With two results in every cell they reduce to
Formulae (29) and (30).

With `"method": "reml"` the variance components are estimated by restricted maximum likelihood (8.4.6.2,
Annex B.2). The likelihood (B.4) is maximized over σ_r² in closed form and over γ = σ_L²/σ_r² ≥ 0 numerically.
The general mean is then the weighted mean of Formula B.5, and its standard error is given by Formula B.7.
REML and ANOVA give the same estimates for balanced data unless the ANOVA estimate of σ_L² is negative. For
unequal n_ij the two methods differ slightly, as in Tables C.5 and C.6. The program reproduces the REML tables
of the standard to the printed digits, which 8.4.6.2 requires of REML software.

For each level the report also gives A_r and A_R of Annex A (Formulae A.3 and A.4, with γ = s_R/s_r). They are
the relative amounts by which s_r and s_R can be expected to differ from σ_r and σ_R, with a probability of
about 95 %. They are meant for planning (how many laboratories?), not as confidence limits.

## 12.4 Dependence on the level (8.5)

With three or more levels the program fits the four relationships of 8.5.1 to s_r and to s_R:

| | Relationship | Fit |
|---|---|---|
| I | s = b·m | b = Σ(s_j/m_j)/q (Formula 39) |
| II | s = a + b·m | weighted regression, weights 1/s², then once more with the fitted values (8.5.2.5) |
| III | s² = a_v² + (b_v·m)² | weighted regression of s² on m², weights 1/s⁴, then once more (8.5.3.2) |
| IV | lg s = c + d·lg m | unweighted regression (8.5.4) |

The report lists the fitted values at each level and plots s_r and s_R against m̂ with the fitted curves.
Choose the relationship from the plot and from what is known about the method. The standard expects a
technical judgement here (8.5.1, C.3.7). If there is no clear dependence, quote a value for the whole range
(C.1.7, C.2.7).

## 12.5 Findings in the standard

Reproducing every number of ISO 5725-2:2019 brought to light the following points. They are recorded in the
research log, and each also appears in ISO 5725-2:1994.

- **F47:** C.1.5 states that the double-high results at levels 2 and 4 are stragglers. At level 4, however,
  Table C.4 gives 0.132, which is above the 5 % critical value 0.1101. C.1.8 reports four stragglers. Table C.4
  and Formula (C.3) give three: the pair at level 2 and the cell of laboratory 5 at level 3.
- **F48:** Table C.12 gives s_R = 1.915 °C at level 4. Formulae (29) to (31) give 1.9175 °C, which is the value
  of the REML Table C.13.
- **F49:** in Table 4 the printed slope d = 0.772 does not fit the fitted values printed in the same table.
  Those values follow from d ≈ 0.769.
