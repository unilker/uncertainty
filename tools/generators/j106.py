import numpy as np
from scipy import integrate, stats
Phi=stats.norm.cdf
g0=lambda e: stats.gamma.pdf(e,4,scale=0.25)
um=0.25;T=2
def RC(r): A=T-2*r*um; return integrate.quad(lambda e:(Phi((A-e)/um)-Phi(-e/um))*g0(e),T,np.inf)[0]
def RP1(r): A=T-2*r*um; return integrate.quad(lambda e:(1-Phi((A-e)/um))*g0(e),0,T)[0]
def RP2(r): A=T-2*r*um; return integrate.quad(lambda e:(1-(Phi((A-e)/um)-Phi(-e/um)))*g0(e),0,T)[0]
print('prior nonconf',1-stats.gamma.cdf(2,4,scale=0.25))
for r in [0,0.5,0.6,0.65,0.7,1]:
    print(r,RC(r),RP1(r),RP2(r))
from scipy.optimize import brentq
r=brentq(lambda r:RC(r)-0.001,-1,2); print('r for 0.1%',r,T-2*r*um,RP1(r),RP2(r))
# resistors
g=lambda e: stats.norm.pdf(e,1500,0.12); um2=0.04; AL,AU=1499.82,1500.18; TL,TU=1499.8,1500.2
F=lambda e: Phi((AU-e)/um2)-Phi((AL-e)/um2)
rc=integrate.quad(lambda e:F(e)*g(e),-np.inf,TL)[0]+integrate.quad(lambda e:F(e)*g(e),TU,np.inf)[0]
rp=integrate.quad(lambda e:(1-F(e))*g(e),TL,TU)[0]
pc=stats.norm.cdf(TU,1500,.12)-stats.norm.cdf(TL,1500,.12)
acc=integrate.quad(lambda e:F(e)*g(e),-np.inf,np.inf)[0]
print('resistors',pc,rc,rp,acc)
# fig 17: u0=T/6 centred, Cm=2,10 w=0
for Cm in [2,4,6,8,10]:
    T=1.0; u0=T/6; um=T/(4*Cm); g=lambda e: stats.norm.pdf(e,0.5,u0)
    for wf in [0]:
        AL,AU=0,1
        F=lambda e: Phi((AU-e)/um)-Phi((AL-e)/um)
        rc=integrate.quad(lambda e:F(e)*g(e),-np.inf,0)[0]+integrate.quad(lambda e:F(e)*g(e),1,np.inf)[0]
        rp=integrate.quad(lambda e:(1-F(e))*g(e),0,1)[0]
        print('Cm',Cm,rc*100,rp*100)
