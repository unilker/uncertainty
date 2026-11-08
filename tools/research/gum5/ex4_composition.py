"""JCGM GUM-5:2026 Example 4 (gravimetric CO2 in N2 mixture, ISO 6142-1): amount fractions, standard
uncertainties and correlations by the multivariate LPU with central differences, the correlation matrix of
the molar masses (4.3.4.6), and the leading contributions to u(y_CO2), u(y_N2), u(y_H2O).
Independent check of library/chemistry/gum5-e4-gravimetric-gas-mixture.json (research log F82).
Run: python3 -I tools/research/gum5/ex4_composition.py (numpy).
"""
import numpy as np
comp=['Ar','CH4','CO','CO2','H2','H2O','N2','O2']
nu={'Ar':[0,0,0,0,1],'CH4':[4,1,0,0,0],'CO':[0,1,0,1,0],'CO2':[0,1,0,2,0],'H2':[2,0,0,0,0],'H2O':[2,0,0,1,0],'N2':[0,0,2,0,0],'O2':[0,0,0,2,0]}
A=[(1.00784,1.00811),(12.0096,12.0116),(14.00643,14.00728),(15.99903,15.99977),(39.792,39.963)]
imp1={'CH4':(0.10,0.06),'CO':(0.5,0.3),'H2O':(1.9,0.1),'N2':(4.3,2.1),'O2':(5.3,2.6)}  # µmol/mol, CO2 major
imp2={'Ar':(5.0,3.0),'CH4':(0.008,0.005),'CO':(0.015,0.009),'CO2':(0.010,0.006),'H2':(0.025,0.015),'H2O':(0.010,0.006),'O2':(0.005,0.003)}
r=[(-152.0243,0.0005),(93.4951,0.0008),(717.2740,0.0007)]
# input vector: r0,r1,r2, A(5), imp1(5), imp2(7)
x0=[v for v,_ in r]+[(a+b)/2 for a,b in A]+[v*1e-6 for v,_ in imp1.values()]+[v*1e-6 for v,_ in imp2.values()]
u=[s for _,s in r]+[(b-a)/np.sqrt(12) for a,b in A]+[s*1e-6 for _,s in imp1.values()]+[s*1e-6 for _,s in imp2.values()]
k1=list(imp1); k2=list(imp2)
def f(x):
    r0,r1,r2=x[0:3]; Av=np.array(x[3:8]); i1=x[8:13]; i2=x[13:20]
    M={c:np.dot(nu[c],Av) for c in comp}
    x1={c:0.0 for c in comp}; x2={c:0.0 for c in comp}
    for c,v in zip(k1,i1): x1[c]=v
    for c,v in zip(k2,i2): x2[c]=v
    x1['CO2']=1-sum(i1); x2['N2']=1-sum(i2)
    MP1=sum(x1[c]*M[c] for c in comp); MP2=sum(x2[c]*M[c] for c in comp)
    m1=r1-r0; m2=r2-r1
    n={c:m1*x1[c]/MP1+m2*x2[c]/MP2 for c in comp}; nt=sum(n.values())
    return np.array([n[c]/nt*1e6 for c in comp])
x0=np.array(x0); u=np.array(u); y=f(x0)
J=np.zeros((8,len(x0)))
for i in range(len(x0)):
    h=u[i]*1e-3; e=np.zeros(len(x0)); e[i]=h; J[:,i]=(f(x0+e)-f(x0-e))/(2*h)
V=J@np.diag(u**2)@J.T; s=np.sqrt(np.diag(V)); R=V/np.outer(s,s)
for c,yy,ss in zip(comp,y,s): print(f"{c:4s} {yy:14.6f} {ss:10.5f} {100*ss/yy:8.4f}%")
np.set_printoptions(precision=2,suppress=True); print(R)
# molar mass correlation
Mj=np.array([nu[c] for c in comp],float); UA=np.diag([((b-a)/np.sqrt(12))**2 for a,b in A]); UM=Mj@UA@Mj.T; sm=np.sqrt(np.diag(UM))
print('M',Mj@np.array([(a+b)/2 for a,b in A]), sm); np.set_printoptions(precision=3); print(UM/np.outer(sm,sm))
names=['r0','r1','r2','A_H','A_C','A_N','A_O','A_Ar']+['x1_'+c for c in k1]+['x2_'+c for c in k2]
for row in (3,6,5):
    c2=(J[row]*u)**2; o=np.argsort(c2)[::-1][:5]
    print(comp[row], [(names[i], round(np.sqrt(c2[i]),3)) for i in o])
