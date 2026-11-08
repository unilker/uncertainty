import numpy as np
from scipy import special, stats
x=np.array([72.5,67.8,69.32,74.03,67.4,70.3,73.3,69.8,73.5,70.3,76.8,74.2,73.9,75.8,67.6,67.4,71.8,73.5,73.2,72.1,69.8,73.3])
u=np.array([2.2,1.3,1.42,1.42,6.1,5.15,1.75,1.9,1.4,1.35,2.6,2.85,3,5.05,4.25,1,3.59,5.3,1.3,2,1.7,1.4])
print('sd',x.std(ddof=1),'median u',np.median(u))
gamma=1.4826*np.median(np.abs(x-np.median(x))); print('gamma (R mad)',gamma)
def lognl(d,b,s):
    # log density of Laplace(0,b) + N(0,s^2) at d
    a1=s**2/(2*b**2)-d/b; z1=(s/b-d/s)/np.sqrt(2)
    a2=s**2/(2*b**2)+d/b; z2=(s/b+d/s)/np.sqrt(2)
    l1=a1+np.log(special.erfcx(z1))-z1**2; l2=a2+np.log(special.erfcx(z2))-z2**2
    m=np.maximum(l1,l2)
    return -np.log(4*b)+m+np.log(np.exp(l1-m)+np.exp(l2-m))
Nt,Nm=2000,1500
th=(np.arange(Nt)+0.5)/Nt; tau=gamma*np.tan(th*np.pi/2)
mu=np.linspace(64,80,Nm)
L=np.zeros((Nt,Nm))
for j in range(len(x)):
    d=x[j]-mu[None,:]
    L+=lognl(d,tau[:,None]/np.sqrt(2),u[j])
L+= stats.norm.logpdf(mu,67.4,100)[None,:]
W=np.exp(L-L.max()); W/=W.sum()
pm=W.sum(0); pt=W.sum(1)
m=(pm*mu).sum(); sd=np.sqrt((pm*(mu-m)**2).sum())
c=np.cumsum(pt); print('mean',m,'sd',sd,'tau median',tau[np.searchsorted(c,0.5)])
cm=np.cumsum(pm); print('interval',mu[np.searchsorted(cm,0.025)],mu[np.searchsorted(cm,0.975)])
