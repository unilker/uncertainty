import numpy as np
r=np.array([0.06300,1.3314,2.5193]); ur=np.array([0.00006,0.0004,0.0008])/2
A=np.array([203.9730435,205.9744652,206.9758968,207.9766520]); uA=np.full(4,1.2e-6)
def f(r):
    s=r[0]+r[1]+1+r[2]; return np.array([r[0],r[1],1,r[2]])/s
x=f(r); print('x',x)
J=np.zeros((4,3)); h=1e-7
for i in range(3):
    d=np.zeros(3); d[i]=h; J[:,i]=(f(r+d)-f(r-d))/(2*h)
C=J@np.diag(ur**2)@J.T; ux=np.sqrt(np.diag(C)); print('ux',ux); print('corr',np.round(C/np.outer(ux,ux),3))
Ar=A@x; u2=A@C@A+ (x**2*uA**2).sum(); print('Ar',Ar,'u',np.sqrt(u2),'U',2*np.sqrt(u2))
rng=np.random.default_rng(1); K=10**6
R=r+rng.standard_normal((K,3))*ur; s=R.sum(1)+1; X=np.c_[R[:,0],R[:,1],np.ones(K),R[:,2]]/s[:,None]
AA=A+rng.standard_normal((K,4))*uA; v=(AA*X).sum(1); q=np.quantile(v,[.025,.975]); print('MC',v.mean(),(q[1]-q[0])/2,v.std())
