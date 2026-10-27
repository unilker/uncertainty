import numpy as np, warnings
warnings.filterwarnings('ignore')
from scipy import odr
x=np.array([1.0006,1.0010,1.9995,3.0018,3.9982,4.0043,4.9981,5.9961,7.9974,7.9995,9.9980,10.0060])
ux=np.array([0.00134,0.0011,0.0032,0.0034,0.0058,0.0058,0.0058,0.0078,0.0084,0.010,0.012,0.0104])
y=np.array([963.7988,966.2585,1912.5692,2846.9306,3754.9386,3764.5905,4647.1511,5529.9991,7246.0082,7240.4767,8884.6955,8902.6916])
uy=np.array([14,12.5,16.8,11.6,10.8,12.1,14.4,12.1,11.9,10.7,10.1,14.1])
f={'exp':(lambda b,t: b[0]+b[1]*np.exp(b[2]*t),[-47.963,47.969,2.1283e-5]),
   'pow':(lambda b,t: b[0]+b[1]*t**(1+b[2]),[0.1213,5.1211e-4,0.08499])}
rng=np.random.default_rng(1)
for k,(g,b0) in f.items():
    out=[];bs=[]
    for _ in range(4000):
        xs=x+ux*rng.standard_normal(12); ys=y+uy*rng.standard_normal(12)
        o=odr.ODR(odr.RealData(ys,xs,sx=uy,sy=ux),odr.Model(g),beta0=b0,maxit=2000,sstol=1e-14,partol=1e-14); o.set_job(deriv=0)
        r=o.run(); bs.append(r.beta)
        out.append(g(r.beta,4950.6+11*rng.standard_normal()))
    out=np.array(out); bs=np.array(bs)
    print(k,'x mean',out.mean(),'u',out.std(ddof=1),'u(b)',bs.std(axis=0,ddof=1))
