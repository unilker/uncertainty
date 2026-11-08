"""BIPM ISRD-06 Table 6 (benzoic acid with BTMSB-d4 in CD3OD): purity bounded by 1 g/g (research log D87).

The printed result is 999.8 +0.2/-1.5 mg/g: the GUM interval y +/- U (k = 2) truncated at 1000 mg/g (Westwood
et al. 2023, 4.1.1). The Monte Carlo evaluation restricted to w <= 1 g/g (rectangular prior on the purity,
Toman et al. 2016) is computed here independently with 10^7 draws of the eight normal inputs of the model
w = R (N_S/N_A) (M_A/M_S) (m_S/m_A) w_S, for the library project's expected values.
Run: python3 tools/research/qnmr/isrd06_bounded.py
"""
import numpy as np

inp = [(1.5000, .00026), (4.9990, .0003), (17.9964, .0003), (122.123, .0054), (226.502, .0125), (6.4659, .00124),
       (2.2223, .00124), (.9991, .0004)]  # R, N_A, N_S, M_A, M_S, m_A, m_S, w_S
rng = np.random.default_rng(20261108)
N = 10_000_000
R, NA, NS, MA, MS, mA, mS, wS = (rng.normal(m, s, N) for m, s in inp)
w = R * (NS / NA) * (MA / MS) * (mS / mA) * wS
y = inp[0][0] * inp[2][0] / inp[1][0] * inp[3][0] / inp[4][0] * inp[6][0] / inp[5][0] * inp[7][0]
c = [y / inp[0][0], -y / inp[1][0], y / inp[2][0], y / inp[3][0], -y / inp[4][0], -y / inp[5][0], y / inp[6][0], y / inp[7][0]]
u = np.sqrt(sum((ci * s) ** 2 for ci, (_, s) in zip(c, inp)))
print(f"GUM: y = {y:.9f}, u = {u:.6e}, U(k=2) = {2*u:.6e}; truncated interval [{y-2*u:.6f}, 1]")
k = np.sort(w[w <= 1.0])
f = k.size / N
m = k.mean()
sd = k.std(ddof=1)
q = int(np.ceil(0.95 * k.size))
widths = k[q - 1:] - k[: k.size - q + 1]
i = int(np.argmin(widths))
lo_s, hi_s = k[i], k[i + q - 1]
print(f"MCM all: mean {w.mean():.9f}, sd {w.std(ddof=1):.6e}")
print(f"MCM w <= 1: fraction {f:.6f}, mean {m:.9f}, sd {sd:.6e}, shortest 95 % [{lo_s:.9f}, {hi_s:.9f}]")
