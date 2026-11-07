import numpy as np
from scipy.stats import multivariate_normal as mvn
R=np.array([[1,.107,.125,.177],[.107,1,.311,.404],[.125,.311,1,.539],[.177,.404,.539,1]])
m=np.array([99.18,97.70,99.33,98.94]); s=np.array([1.37,1.02,1.05,1.22])
Sc=R*np.outer(s,s); lo,hi=95.0,105.0
def spec(cm):
    u=0.028*cm; Sm=R*np.outer(u,u)
    Sp=np.linalg.inv(np.linalg.inv(Sc)+np.linalg.inv(Sm)); cp=Sp@(np.linalg.inv(Sc)@m+np.linalg.inv(Sm)@cm)
    # box probability by inclusion–exclusion of CDF corners
    from itertools import product
    P=0
    for corner in product([0,1],repeat=4):
        x=np.array([hi if c else lo for c in corner]); sign=(-1)**(4-sum(corner))
        P+=sign*mvn(cp,Sp,abseps=1e-8,releps=1e-8).cdf(x)
    return 1-P
print('cm=m',spec(m),'cm=TL',spec(np.full(4,lo)),'cm=TU',spec(np.full(4,hi)))
rng=np.random.default_rng(1); N=10_000_000
L=np.linalg.cholesky(Sc); c=m+rng.standard_normal((N,4))@L.T
z=rng.standard_normal((N,4)); Lr=np.linalg.cholesky(R)
cm=c+(z@Lr.T)*(0.028*c)           # likelihood sd 0.028*c (true); variant with c_m below
acc=((cm>=lo)&(cm<=hi)).all(1); conf=((c>=lo)&(c<=hi)).all(1)
print('global (u=0.028c)', (acc&~conf).mean())
