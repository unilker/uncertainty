"""ISO Guide 35 Example C.1 (chromium in soil, 20 bottles x 3), as reprinted in Possolo & Bodnar, Metrologia 55
(2018) 147, Table 3: one-way ANOVA (s_bb, s_r) and the prior scales of the Bayesian model of section 4.4."""
import statistics as st
W = [(121.30, 128.74, 119.91), (120.87, 121.32, 119.24), (122.44, 122.96, 123.45), (117.60, 119.66, 118.96),
     (110.65, 112.34, 110.29), (117.29, 120.79, 121.42), (115.27, 121.45, 117.48), (118.96, 123.78, 123.29),
     (118.67, 116.67, 114.58), (126.24, 123.51, 126.20), (128.65, 122.02, 121.93), (126.84, 124.72, 123.14),
     (122.61, 128.48, 126.20), (118.95, 123.82, 118.11), (118.74, 118.23, 117.38), (119.74, 121.78, 121.01),
     (121.21, 123.28, 116.38), (129.30, 124.10, 122.02), (136.81, 129.80, 128.47), (127.81, 117.66, 122.90)]
if __name__ == "__main__":
    means = [st.mean(b) for b in W]
    sx = st.stdev(means)
    sw = (sum(st.variance(b) for b in W) / len(W)) ** 0.5
    ss = (sx ** 2 - sw ** 2 / 3) ** 0.5
    print("grand mean", st.mean(means), "s_x", sx, "s_w", sw, "s_bb", ss, "u(mean) ANOVA", sx / len(W) ** 0.5)
    print("prior medians: tau", sx, "sigma", st.median([st.stdev(b) for b in W]))
