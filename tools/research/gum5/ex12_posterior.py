"""JCGM GUM-5:2026 Example 12 (between-bottle homogeneity of N2): ANOVA, u*_bb and the posterior of the
Bayesian hierarchical model on a (tau, sigma) grid, with B_i and mu integrated analytically.

Independent check of core/src/pt.rs::between_unit_posterior. Output (grid 6001 x 1401): tau mean 1.356e-4,
median 1.126e-4, 2.5/97.5 % 5.24e-6 / 3.971e-4, density largest at tau = 0; sigma mean 6.885e-4,
2.5/97.5 % 5.639e-4 / 8.463e-4; mu 0.4251439. Stan (12.4.5): 0.00014, —, 0.00001 / 0.00040; 0.00069,
0.00056 / 0.00085; 0.42514. Run: python3 -I tools/research/gum5/ex12_posterior.py (numpy).
"""
import numpy as np
Y=np.array([[0.424577,0.425167,0.425379,0.424522,0.424805],[0.425572,0.425411,0.423638,0.425301,0.424527],
[0.424152,0.425517,0.425638,0.424207,0.425135],[0.426320,0.424672,0.425211,0.425533,0.425864],
[0.424855,0.425079,0.425413,0.424729,0.424725],[0.425104,0.424773,0.426424,0.424266,0.424632],
[0.425750,0.424917,0.424779,0.425086,0.425318],[0.425547,0.426483,0.424631,0.425968,0.424620],
[0.426326,0.424646,0.425205,0.426302,0.425020],[0.425968,0.424069,0.425988,0.425489,0.423936]])
g,n=Y.shape; m=Y.mean(1); gm=Y.mean()
SSB=n*((m-gm)**2).sum(); SSW=((Y-m[:,None])**2).sum()
print('SSB',SSB,'MSB',SSB/(g-1),'SSW',SSW,'MSW',SSW/(g*(n-1)))
MSW=SSW/(g*(n-1)); print('u_bb*',np.sqrt(MSW/n)*(2/(g*(n-1)))**0.25, 'rel', np.sqrt(MSW/n)*(2/(g*(n-1)))**0.25/gm)
mu0=0.4250; s0=0.025*mu0; tau0=0.0015*mu0; sig0=0.0020*mu0
# Stan scales by mean(y): tau_ ~ cauchy(0, tau0/scale) equivalent to tau ~ cauchy(0,tau0)
tau=(0.0076*np.linspace(0,1,6001)**2)[None,:]; sig=np.linspace(0.0002,0.0016,1401)[:,None]
v=tau**2+sig**2/n
# integrate mu: ybar_i ~ N(mu, v), mu ~ N(mu0,s0^2)
S1=m.sum(); S2=(m**2).sum()
prec=g/v+1/s0**2; mean_mu=(S1/v+mu0/s0**2)/prec
logL=(-g/2*np.log(v) -0.5*(S2/v + mu0**2/s0**2 - prec*mean_mu**2) -0.5*np.log(prec)
      -(g*(n-1))*np.log(sig) - SSW/(2*sig**2))
logp=logL - np.log(1+(tau/tau0)**2) - np.log(1+(sig/sig0)**2)
P=np.exp(logp-logp.max())
t_=tau[0]; wt=np.gradient(t_); s_=sig[:,0]; ws=np.gradient(s_)
pt=(P*ws[:,None]).sum(0)*wt; pt/=pt.sum(); ps=(P*wt[None,:]).sum(1)*ws; ps/=ps.sum()
t=tau[0]; s=sig[:,0]
def summ(x,p):
    c=np.cumsum(p); mean=(x*p).sum(); med=np.interp(0.5,c,x)
    q=np.interp([0.025,0.975],c,x)
    return mean,med,q,np.sqrt(((x-mean)**2*p).sum())
print('tau',summ(t,pt)); print('sig',summ(s,ps))
mm=(mean_mu*P).sum()/P.sum(); print('mu',mm)
