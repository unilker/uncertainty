import numpy as np
from scipy import odr
ex3 = dict(
 x=[1.0006,1.0010,1.9995,3.0018,3.9982,4.0043,4.9981,5.9961,7.9974,7.9995,9.9980,10.0060],
 ux=[0.00134,0.0011,0.0032,0.0034,0.0058,0.0058,0.0058,0.0078,0.0084,0.010,0.012,0.0104],
 y=[963.7988,966.2585,1912.5692,2846.9306,3754.9386,3764.5905,4647.1511,5529.9991,7246.0082,7240.4767,8884.6955,8902.6916],
 uy=[14,12.5,16.8,11.6,10.8,12.1,14.4,12.1,11.9,10.7,10.1,14.1])
ex1 = dict(x=[4.5,18.75,50],ux=[0.045,0.1875,0.5],y=[0.1969,0.7874,2.0228],uy=[0.003938,0.015748,0.040456])
def run(d, f, b0):
    data = odr.RealData(d['y'], d['x'], sx=d['uy'], sy=d['ux'])
    o = odr.ODR(data, odr.Model(f), beta0=b0, maxit=10000, sstol=1e-15, partol=1e-15)
    o.set_job(deriv=0)
    r = o.run()
    return r
lin = lambda b,y: b[0]+b[1]*y
r = run(ex1, lin, [0,25]); print('ex1', r.beta, np.sqrt(np.diag(r.cov_beta)), r.cov_beta[0,1], r.sum_square)
expf = lambda b,y: b[0]+b[1]*np.exp(b[2]*y)
r = run(ex3, expf, [-47.963,47.969,2.1283e-5]); print('ex3 exp', r.beta, np.sqrt(np.diag(r.cov_beta)), r.sum_square)
C=r.cov_beta; b=r.beta; y=4950.6
g=np.array([1,np.exp(b[2]*y),b[1]*y*np.exp(b[2]*y)]); gy=b[1]*b[2]*np.exp(b[2]*y)
print(' pred', expf(b,y), np.sqrt(gy**2*11**2+g@C@g))
powf = lambda b,y: b[0]+b[1]*y**(1+b[2])
r = run(ex3, powf, [0.12,5.1e-4,0.085]); print('ex3 pow', r.beta, np.sqrt(np.diag(r.cov_beta)), r.sum_square)
