"""JCGM GUM-5:2026 Example 9 (torque sensor, Y = beta X): OLS and WLS with the GUF (9.4, 9.8, 9.9) and the
Bayesian posterior (9.17) by quadrature, non-informative (N(1, 100^2), a_i = b_i = 0) and informative
(N(1, 0.1^2), a_i = 3, b_i = 2 s_i^2). Output: OLS 1.01065 +- 0.00152, nu 2.72; WLS 1.00849 +- 0.00077,
nu 13.31 (13.17 with the rounded u of Table 9.2); NI 1.00921 +- 0.00106 [1.00701, 1.01121];
Inf 1.00909 +- 0.00087 [1.00731, 1.01076]. Printed (Table 9.3): 1.0107/0.0015, 1.0085/0.0008 (nu 14),
1.0092/0.0011 [1.0070, 1.0112], 1.0091/0.0009 [1.0073, 1.0108].
Run: python3 -I tools/research/gum5/ex9_bayes.py (numpy, scipy).
"""
import numpy as np
from scipy import stats
x=np.array([0.101,0.201,0.305,0.501,1.001,3.000,4.001,5.007]); y=np.array([0.0950,0.1966,0.3016,0.4983,1.0083,3.0266,4.0466,5.0666])
s=np.array([0.0055,0.0052,0.0041,0.0041,0.0098,0.0082,0.0121,0.0379]); n=np.array([6,6,6,6,6,6,6,3])
u=s/np.sqrt(n); nu=n-1
def ls(w):  # w = n_i W_i
    b=(w*x*y).sum()/(w*x*x).sum(); ui=w*x*u/(w*x*x).sum(); uc=np.sqrt((ui**2).sum()); ne=uc**4/(ui**4/nu).sum()
    k=stats.t.ppf(0.975,ne); return b,uc,ne,(b-k*uc,b+k*uc)
print('u(y)',np.round(u,4))
print('OLS',ls(n*1.0)); print('WLS',ls(1/u**2))
def bayes(a,bb,m0,s0):
    B=np.linspace(0.99,1.03,400001)
    lp=stats.norm.logpdf(B,m0,s0)
    for i in range(8):
        nuu=n[i]-1+2*a[i]; sc=np.sqrt(((n[i]-1)*s[i]**2+2*bb[i])/(n[i]*x[i]**2*nuu))
        lp+=stats.t.logpdf(B,nuu,loc=y[i]/x[i],scale=sc)
    p=np.exp(lp-lp.max()); p/=np.trapezoid(p,B); m=np.trapezoid(B*p,B); sd=np.sqrt(np.trapezoid((B-m)**2*p,B))
    c=np.cumsum(p)*(B[1]-B[0]); lo=np.interp(0.025,c,B); hi=np.interp(0.975,c,B); return m,sd,(lo,hi)
z=np.zeros(8); print('NI',bayes(z,z,1,100))
print('Inf',bayes(np.full(8,3.0),2*s**2,1,0.1))
ur=np.array([0.0022,0.0021,0.0017,0.0017,0.0040,0.0033,0.0049,0.0219])
w=1/ur**2; b=(w*x*y).sum()/(w*x*x).sum(); ui=w*x*ur/(w*x*x).sum(); uc=np.sqrt((ui**2).sum()); print('WLS rounded u', b, uc, uc**4/(ui**4/nu).sum())
w=1/u**2; ui=w*x*u/(w*x*x).sum(); print('shares', np.round(ui**2/(ui**2).sum(),3))
