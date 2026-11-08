import numpy as np
x1,ux1=10252.0,16.0
q1=np.array([82.243,113.088,149.677]); u1=np.array([0.017,0.023,0.028])
q2=np.array([1108.49,1077.17,1039.78]); u2=np.array([0.54,0.53,0.57])
C1=np.zeros((3,3)); C2=np.zeros((3,3))
np.fill_diagonal(C1,u1**2); np.fill_diagonal(C2,u2**2)
for (i,j),a,b in zip([(0,1),(0,2),(1,2)],[2.31e-4,2.54e-4,3.58e-4],[1.26e-1,1.22e-1,1.20e-1]):
    C1[i,j]=C1[j,i]=a; C2[i,j]=C2[j,i]=b
S=q1+q2; x=x1*q1/S
J=np.zeros((3,7))  # x1, q1a..c, q2a..c
J[:,0]=q1/S
for k in range(3): J[k,1+k]=x1*q2[k]/S[k]**2; J[k,4+k]=-x1*q1[k]/S[k]**2
V=np.zeros((7,7)); V[0,0]=ux1**2; V[1:4,1:4]=C1; V[4:7,4:7]=C2
Vx=J@V@J.T
np.set_printoptions(precision=3,suppress=True)
print('x',x); print('Vx',Vx); print('u',np.sqrt(np.diag(Vx)))
print('printed Vx diag 39.5 37.0 41.2; off 1.7 2.2 3.0')
# MFC curve check
def C1f(q): return -2.493/q+1.021-1.590e-4*q+1.975e-7*q*q
def C2f(q): return -39.967/q+5.306/np.sqrt(q)+0.6104+0.01195*np.sqrt(q)-1.249e-4*q
for qa,qb in ((84,1116),(115,1085),(152,1048)): print(qa*C1f(qa), qb*C2f(qb))
VC1=np.array([[9.2e-4,-1.84e-5,9.5e-8,-1.3e-10],[-1.84e-5,4.4e-7,-2.4e-9,3.4e-12],[9.5e-8,-2.4e-9,1.4e-10,-2.1e-14],[-1.3e-10,3.4e-12,-2.1e-14,3.3e-17]])
for q in (84,115,152):
    g=np.array([1/q,1,q,q*q]); uc=np.sqrt(g@VC1@g); urep=0.00013*q*C1f(q)
    print('MFC1',q,'u_cal(q)',q*uc,'u_rep',urep,'total',np.hypot(q*uc,urep))
