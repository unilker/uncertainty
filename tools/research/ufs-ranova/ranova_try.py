import math, statistics as st
from scipy import stats
data=[[3898,4139,4466,4693],[3910,3993,4201,4126],[5708,5903,4061,3782],[5028,4754,5450,5416],
[4640,4401,4248,4191],[5182,5023,4662,4839],[3028,3224,3023,2901],[3966,4283,4131,3788]]
c=1.5
beta=2*stats.norm.cdf(c)-1-2*c*stats.norm.pdf(c)+2*c*c*(1-stats.norm.cdf(c))
def clip(x,m,s): return min(max(x,m-c*s),m+c*s)
def classical(d):
    n=len(d); allv=[v for r in d for v in r]; gm=sum(allv)/len(allv)
    msa=sum((r[0]-r[1])**2/2+(r[2]-r[3])**2/2 for r in d)/(2*n)
    mss=sum(2*(((r[0]+r[1])/2-(sum(r)/4))**2+((r[2]+r[3])/2-sum(r)/4)**2) for r in d)/n
    mst=sum(4*(sum(r)/4-gm)**2 for r in d)/(n-1)
    return gm, msa, (mss-msa)/2, (mst-mss)/4
def ranova(d, variant=0, iters=500):
    n=len(d)
    # initial values
    gm,va,vs,vt=classical(d)
    sA=math.sqrt(va); sS=math.sqrt(max(vs,1e-9)); sT=math.sqrt(max(vt,1e-9))
    if variant==1:
        # median/MAD start
        diffs=[abs(r[0]-r[1]) for r in d]+[abs(r[2]-r[3]) for r in d]
        sA=st.median(diffs)/2/0.6745*math.sqrt(2)/math.sqrt(2)
    mu=gm
    m=[[(r[0]+r[1])/2,(r[2]+r[3])/2] for r in d]
    M=[sum(r)/4 for r in d]
    for it in range(iters):
        # level analysis
        ss=0
        for i,r in enumerate(d):
            for j in range(2):
                a,b=r[2*j],r[2*j+1]; mm=m[i][j]
                xa,xb=clip(a,mm,sA),clip(b,mm,sA)
                m[i][j]=(xa+xb)/2
                ss+=(xa-m[i][j])**2+(xb-m[i][j])**2
        sA=math.sqrt(ss/(2*n*beta))
        # level sample
        ssm=math.sqrt(sS**2+sA**2/2)
        ss=0; Mi=[]
        for i in range(n):
            ya,yb=clip(m[i][0],M[i],ssm),clip(m[i][1],M[i],ssm)
            M[i]=(ya+yb)/2
            ss+=(ya-M[i])**2+(yb-M[i])**2
        vsm=ss/(n*beta); sS=math.sqrt(max(vsm-sA**2/2,0))
        # level target
        stm=math.sqrt(sT**2+sS**2/2+sA**2/4)
        zs=[clip(x,mu,stm) for x in M]
        mu=sum(zs)/n
        vtm=sum((z-mu)**2 for z in zs)/((n-1)*beta)
        sT=math.sqrt(max(vtm-sS**2/2-sA**2/4,0))
    tot=math.sqrt(sT**2+sS**2+sA**2)
    return mu,sT,sS,sA,tot
print('beta',beta)
print('classical',[math.sqrt(x) if i else x for i,x in enumerate(classical(data))])
for v in (0,):
    print(ranova(data,v))
print('target: 4408.3237 565.39868 319.04834 167.94308 total 670.57617')

def algA(x, tol=1e-12, it=1000):
    xs=sorted(x); n=len(x)
    med=st.median(x); s=1.483*st.median([abs(v-med) for v in x]); m=med
    for _ in range(it):
        d=1.5*s
        w=[min(max(v,m-d),m+d) for v in x]
        m2=sum(w)/n; s2=1.134*math.sqrt(sum((v-m2)**2 for v in w)/(n-1))
        if abs(m2-m)<tol and abs(s2-s)<tol: m,s=m2,s2; break
        m,s=m2,s2
    return m,s
def ranova2(d):
    da=[r[0]-r[1] for r in d]+[r[2]-r[3] for r in d]
    _,sa=algA(da); sA=sa/math.sqrt(2)
    D=[(r[0]+r[1])/2-(r[2]+r[3])/2 for r in d]
    _,sd=algA(D); ssm=sd/math.sqrt(2); sS=math.sqrt(max(ssm**2-sA**2/2,0))
    M=[sum(r)/4 for r in d]
    mu,stm=algA(M); sT=math.sqrt(max(stm**2-sS**2/2-sA**2/4,0))
    return mu,sT,sS,sA,math.sqrt(sT**2+sS**2+sA**2)
print('algA-diffs', ranova2(data))
