"""Merkatas et al., Metrologia 56 (2019) 054001, Table 1 (16 measurements of G, JILA-10 corrected, HUST-18 added)
and Table 4: DL with Knapp-Hartung, ML, the NICOB Bayesian model and the Linear Pool. numpy/scipy."""
import numpy as np
from scipy import stats, optimize
from koepke2017 import dl
from bayes_grid import post

G = np.array([6.67248, 6.6729, 6.67398, 6.674255, 6.67559, 6.67422, 6.67387, 6.67222, 6.67425, 6.67349, 6.67260,
              6.67554, 6.67191, 6.67435, 6.674184, 6.674484])
U = np.array([0.00043, 0.00050, 0.00070, 0.000092, 0.00027, 0.00098, 0.00027, 0.00087, 0.00012, 0.00018, 0.00025,
              0.00016, 0.00099, 0.00013, 0.000078, 0.000078])
if __name__ == "__main__":
    mu, um, tau, Q, ukh = dl(G, U)
    t = stats.t.ppf(0.975, len(G) - 1)
    print("DL", mu, "u", um, "uKH", ukh, "tau", tau, "KH interval", mu - t * ukh, mu + t * ukh)
    def prof(tt):
        v = tt ** 2 + U ** 2; w = 1 / v; m = (w * G).sum() / w.sum()
        return 0.5 * (np.log(v) + (G - m) ** 2 / v).sum()
    r = optimize.minimize_scalar(prof, bounds=(0, 0.01), method="bounded", options={"xatol": 1e-12})
    v = r.x ** 2 + U ** 2; w = 1 / v; m = (w * G).sum() / w.sum()
    print("ML", m, "tau", r.x)
    mad = np.median(np.abs(G - np.median(G)))
    b = post(G - np.median(G), U, 0, 0, mad)
    print("Bayes", b[0] + np.median(G), "sd", b[1], "interval", [q + np.median(G) for q in b[3]], "tau median", b[2])
    print("LP", G.mean(), np.sqrt((U ** 2).mean() + ((G - G.mean()) ** 2).mean()))
