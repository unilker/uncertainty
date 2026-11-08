"""JCGM GUM-5:2026 Example 10 (total suspended particulate matter), 10.6.3-10.6.4: risks by quadrature.

Lognormal prior (meanlog -2.325, sdlog 0.434), T_U = 0.2 mg/m3, relative uncertainty 7 %.
mode 'hat':  normal part of the joint PDF with sd 0.07*rho_hat, as in the R code of 10.6.3.1 (normalized by C);
             reproduces the printed 0.5 % / 0.9 % (A_U = T_U), 0.002 % / 5.8 % (0.17), 1.7 % / 0.11 % (0.22).
mode 'true': conditional PDF of rho_hat given rho with sd 0.07*rho (C = 1); the library projects use it.
Specific risks use sd 0.07*rho_hat in both modes for 'hat' (likelihood of 10.6.4.3): 11.3 % at 0.225.
Run: python3 -I tools/research/gum5/ex10_risks.py (numpy, scipy). Output saved in the research log (F80).
"""
import numpy as np
from scipy import integrate, stats, optimize
mu,s=-2.325,0.434; TU=0.2; ur=0.07
g=lambda r: stats.lognorm.pdf(r, s, scale=np.exp(mu))
def h(rh,r,mode):
    sd= ur*rh if mode=='hat' else ur*r
    return stats.norm.pdf(rh, r, sd)
def glob(AU,mode):
    P=lambda rh,r: g(r)*h(rh,r,mode)
    C=integrate.dblquad(P,0,3,0,3,epsabs=1e-12)[0]   # r outer, rh inner
    rc=integrate.dblquad(P,TU,3,0,AU,epsabs=1e-12)[0]/C
    rp=integrate.dblquad(P,0,TU,AU,3,epsabs=1e-12)[0]/C
    return C,rc,rp
for mode in ['hat','true']:
    print(mode,[ (AU,)+tuple(round(v,6) for v in glob(AU,mode)) for AU in (0.17,0.2,0.22)])
def spec(rh,mode,cons):
    P=lambda r: g(r)*h(rh,r,mode)
    num=integrate.quad(P,TU,5)[0] if cons else integrate.quad(P,0,TU)[0]
    return num/integrate.quad(P,0,5,points=[rh])[0]
for mode in ['hat','true']:
    print(mode,'spec prod 0.225',spec(0.225,mode,False),'cons 0.19',spec(0.19,mode,True))
print('q', stats.lognorm.ppf([0.025,0.975],s,scale=np.exp(mu)))
