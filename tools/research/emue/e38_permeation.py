import numpy as np, math
R=8.314462618
T1,T2=52.45+273.15,62.00+273.15; r1,r2=7.4921,13.599
Ea=R*math.log(r2/r1)/(1/T1-1/T2); k0=r1*math.exp(Ea/(R*T1))
print('Ea',Ea,'k0',k0,'dm/dT at T2',r2*Ea/(R*T2**2), 'printed Ea 59164 -> dm/dT', r2*59164/(R*T2**2))
print('u(T) implied by 0.0022:', 0.0022/(r2*Ea/(R*T2**2)))
AW={'H':(1.00784,1.00811),'C':(12.0096,12.0116),'N':(14.00643,14.00728),'O':(15.99903,15.99977)}
names=['m1','m2','xw1','A_H','A_C','A_N','A_O','A_Ar']+['p2_'+k for k in ['Ar','CH4','CO','CO2','H2','H2O','O2']]
x0=[13.599e-6,0.3126,0.010]+[(a+b)/2 for a,b in AW.values()]+[39.948]+[5e-6,8e-9,1.5e-8,1e-8,2.5e-8,1e-8,5e-9]
for variant,um1 in (('OLS',0.108e-6),('EIV',0.088e-6)):
    u=[um1,0.0006,0.002]+[(b-a)/math.sqrt(12) for a,b in AW.values()]+[0.001/math.sqrt(3)]+[3e-6,5e-9,9e-9,6e-9,1.5e-8,6e-9,3e-9]
    def f(x):
        m1,m2,xw,H,C,N,O,Ar=x[:8]; i2=dict(zip(['Ar','CH4','CO','CO2','H2','H2O','O2'],x[8:]))
        M={'NH3':N+3*H,'H2O':2*H+O,'Ar':Ar,'CH4':C+4*H,'CO':C+O,'CO2':C+2*O,'H2':2*H,'O2':2*O,'N2':2*N}
        M1=(1-xw)*M['NH3']+xw*M['H2O']
        xN2=1-sum(i2.values()); M2=xN2*M['N2']+sum(v*M[k] for k,v in i2.items())
        n1=m1/M1; n2=m2/M2; return n1*(1-xw)/(n1+n2)
    x0a=np.array(x0); ua=np.array(u); y=f(x0a)
    J=np.array([(f(x0a+np.eye(len(x0))[i]*ua[i]*1e-3)-f(x0a-np.eye(len(x0))[i]*ua[i]*1e-3))/(2*ua[i]*1e-3) for i in range(len(x0))])
    uy=math.sqrt(((J*ua)**2).sum())
    print(variant,'y',y*1e6,'u',uy*1e6, 'rel%',100*uy/y, 'top',sorted(zip((J*ua)**2/uy**2,names),reverse=True)[:4])
# M̄1, M̄2 and their variances (EMUE: 17.04063, 28.01377, 3.99e-6, 2.42e-7, 1.19e-7, r=0.121)
