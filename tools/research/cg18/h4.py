import numpy as np, math
mN=[0,50,100,150,200,250,300,350,400]
mc=[0,50.000006,99.999987,149.999993,200.000013,250.000019,300.000000,350.000006,400.000010]
Ib=[0,50.000067,100.000100,150.000233,200.000267,250.000100,300.000200,350.000267,400.000400]
It=[0,50.000067,100.000200,150.000267,200.000267,250.000200,300.000200,350.000267,400.000400]
# budget u(mref), u(I) in mg (printed)
umref=np.array([0,0.014,0.023,0.037,0.038,0.053,0.062,0.076,0.077])
uI=np.array([0.042,0.051,0.053,0.055,0.058,0.062,0.067,0.072,0.077])
# reconstruct u(I): rep 0.030, dig0 0.029 (d/sqrt12=0.1/sqrt12=0.0289), digL 0.0289, ecc urel 0.000144 *L? in mg: 0.000144? per g?
d=0.1/math.sqrt(12)
for L in mN:
    ecc = 0.10*L/(2*200*math.sqrt(3))
    u=math.sqrt(0.030**2+d**2+(d**2 if L else 0)+ecc**2)
    print(L, round(ecc,4), round(u,4))
print('---fit')
uI=[]
for L in mN:
    ecc = 0.10*L/(2*200*math.sqrt(3))
    uI.append(math.sqrt(0.030**2+d**2+(d**2 if L else 0)+ecc**2))
uI=np.array(uI)
# u(mref) from components: u(mc)=U/2 per weight, drift, buoyancy var
w={50:(0.030/2,0.005),100:(0.050/2,0.010),200:(0.090/2,0.015),'200*':(0.090/2,0.015)}
combo={0:[],50:[50],100:[100],150:[100,50],200:[200],250:[200,50],300:[200,100],350:[200,100,50],400:[200,'200*']}
rho_a,rho0,rhoa1,u_rho=1.090,1.2,1.045,60.0
rho=8000.0
def ub2(mn):
    mg=mn*1000
    return mg**2*((rho_a-rho0)*(rho_a-rho0-2*(rhoa1-rho0))*u_rho**2/rho**4)
umr=[]
for L in mN:
    parts=combo[L]
    uc=sum(w[p][0] for p in parts)  # linear sum (correlated)?
    ud=sum(w[p][1] for p in parts)
    var=uc**2+ud**2+ub2(L)
    umr.append(math.sqrt(var) if L else 0)
    print(L, 'uc',round(uc,4),'ud',round(ud,4),'ub2',ub2(L),'umref',round(math.sqrt(var),4) if L else 0)
umr=np.array(umr)
def fit(e, sm):
    U=np.outer(umr,umr)+np.diag(uI**2)+sm**2*np.eye(9)
    X=np.array(It).reshape(-1,1)
    P=np.linalg.inv(U)
    Ua=np.linalg.inv(X.T@P@X)
    a=Ua@X.T@P@e
    v=X@a-e
    chi=v@P@v
    return a[0],math.sqrt(Ua[0,0]),Ua[0,0],chi
for name,I in (('budget',Ib),('table',It)):
    e=(np.array(I)-np.array(mc))*1000
    print(name, np.round(e,3))
    for sm in (0,0.05,0.25):
        print('  sm',sm, fit(e,sm))
