"""Exact grid posterior of the Gaussian random-effects model with a half-Cauchy prior on tau: IUPAC 2024
Examples 3E/4M/4N (iridium) and the hierarchical Bayesian rows of Koepke et al. 2017 (NICOB defaults).
Usage: python3 bayes_grid.py (from this folder)."""
import numpy as np
from scipy import stats, optimize
x=np.array([192.21677,192.21662,192.21763]); u=np.array([0.00081,0.00029,0.00017])
w=1/u**2; xw=(w*x).sum()/w.sum(); Q=(w*(x-xw)**2).sum(); print('Q',Q,'p',stats.chi2.sf(Q,2))
print('sd',x.std(ddof=1),'gm u',np.exp(np.log(u).mean()),'naive tau',np.sqrt(x.std(ddof=1)**2-np.exp(np.log(u).mean())**2))
def post(x,u,m0,s0,scale,N=200000):
    th=(np.arange(N)+0.5)/N; t=scale*np.tan(th*np.pi/2)
    v=t[:,None]**2+u**2
    P=(1/v).sum(1)+(1/s0**2 if s0 else 0)
    num=(x/v).sum(1)+(m0/s0**2 if s0 else 0)
    mt=num/P
    ll=-0.5*np.log(v).sum(1)-0.5*np.log(P)-0.5*((x**2/v).sum(1)+(m0**2/s0**2 if s0 else 0)-P*mt**2)
    wgt=np.exp(ll-ll.max()); wgt/=wgt.sum()
    mean=(wgt*mt).sum(); var=(wgt*(1/P+mt**2)).sum()-mean**2
    c=np.cumsum(wgt); med=t[np.searchsorted(c,0.5)]
    # mu quantiles
    F=lambda z:(wgt*stats.norm.cdf((z-mt)*np.sqrt(P))).sum()
    q=[optimize.brentq(lambda z:F(z)-p,mean-50*np.sqrt(var),mean+50*np.sqrt(var)) for p in (0.025,0.975)]
    return mean,np.sqrt(var),med,q,t[np.searchsorted(c,0.025)],t[np.searchsorted(c,0.975)]
print('4M',post(x,u,192.217,0.0015,0.00042))
print('flat',post(x,u,0,0,0.00042))
from koepke2017 import G, uG, Cs, uCs, Sr, uSr
for name,(xx,uu),f in (('G',(G,uG),1),('Cs',(Cs,uCs),1/365.25),('Sr',(Sr,uSr),1/365.25)):
    mad=np.median(np.abs(xx-np.median(xx)))
    r=post(xx-np.median(xx),uu,0,0,mad)
    print(name,'mad',mad*f,'mean',(r[0]+np.median(xx))*f,'sd',r[1]*f,'q',[(q+np.median(xx))*f for q in r[3]],'tau med',r[2]*f)
