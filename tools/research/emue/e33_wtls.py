import numpy as np
from scipy.optimize import least_squares
y=np.array([708.1,974.1,1290.1]); w=np.array([146.6,213.7,285.0]); uw=np.array([0.75,0.67,1.53])
Vp=np.array([[39.5,1.7,2.2],[1.7,37.0,3.0],[2.2,3.0,41.2]])
def wtls(Vy):
    Ly=np.linalg.cholesky(np.linalg.inv(Vy))  # whitening
    def res(p):
        A,B=p[:2]; wt=p[2:]
        return np.r_[Ly.T@(y-(A+B*wt)), (w-wt)/uw]
    r=least_squares(res,np.r_[0,4,w],xtol=1e-14,ftol=1e-14)
    J=r.jac; cov=np.linalg.inv(J.T@J)
    return r.x[:2],cov[:2,:2], 2*r.cost
for name,V in (('printed V',Vp),('diag only',np.diag(np.diag(Vp)))):
    (A,B),C,chi=wtls(V)
    ww,uww=213.33,0.11
    yv=A+B*ww; uy=np.sqrt(C[0,0]+C[1,1]*ww**2+uww**2*B**2+2*C[0,1]*ww)
    print(name,'A',A,'B',B,'VAB',C.ravel(),'chi2',chi,'y',yv,'u',uy)
A,B=88.94,4.19; print('printed eq E3.3.19:', A+B*213.33, np.sqrt(285+6.36e-3*213.33**2+0.11**2*B**2-2*1.30*213.33))
