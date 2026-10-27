import numpy as np, math
from scipy import stats, optimize
def cochran_crit(p,n,a): return 1/(1+(p-1)*stats.f.ppf(a/p,(p-1)*(n-1),n-1))
def grubbs1_crit(n,a):
    t=stats.t.ppf(1-a/(2*n),n-2); return (n-1)*t/math.sqrt(n*(n-2+t*t))
D1={0.001:(-4.2493,1.0012,0.0443),0.005:(-3.6613,0.9558,0.0388),0.01:(-3.3101,0.9250,0.0362),0.025:(-2.8580,0.8833,0.0322),0.05:(-2.5075,0.8501,0.0289),0.1:(-2.1615,0.8169,0.0251)}
def grubbs2_crit(n,a):  # one-tailed at a
    g=D1[a]; f=g[0]+g[1]*n+g[2]*n*n
    F=stats.f.ppf((1-a)**(1/f),2,n-3); return 1/(1+2/(n-3)*F)
def h_ind(p,a): t=stats.t.ppf(1-a/2,p-2); return (p-1)*t/math.sqrt(p*(p-2+t*t))
def k_ind(p,n,a): return math.sqrt(p/(1+(p-1)*stats.f.ppf(a,(p-1)*(n-1),n-1)))
print('cochran p8 n3', cochran_crit(8,3,.05), cochran_crit(8,3,.01))
print('cochran n2 p15/16 5%', cochran_crit(15,2,.05), cochran_crit(16,2,.05), 'p9', cochran_crit(9,2,.05), cochran_crit(9,2,.01),'p8',cochran_crit(8,2,.05))
for n in (8,9,15,16): print('grubbs',n, grubbs1_crit(n,.05),grubbs1_crit(n,.01), grubbs2_crit(n,.025), grubbs2_crit(n,.005))
print('h p9', h_ind(9,.01), h_ind(9,.05), 'k p9 n2', k_ind(9,2,.01), k_ind(9,2,.05))

# ---------------- examples
C1 = [  # lab -> 4 levels lists
 [[0.71,0.71,0.70,0.71],[1.20,1.18,1.23,1.21],[1.68,1.70,1.68,1.69],[3.26,3.26,3.20,3.24]],
 [[0.69,0.67,0.68],[1.22,1.21,1.22],[1.64,1.64,1.65],[3.20,3.20,3.20]],
 [[0.66,0.65,0.69],[1.28,1.31,1.30],[1.61,1.61,1.62],[3.37,3.36,3.38]],
 [[0.67,0.65,0.66],[1.23,1.18,1.20],[1.68,1.66,1.66],[3.16,3.22,3.23]],
 [[0.70,0.69,0.66,0.71,0.69],[1.31,1.22,1.22,1.24],[1.64,1.67,1.60,1.66,1.68],[3.20,3.19,3.18,3.27,3.24]],
 [[0.73,0.74,0.73],[1.39,1.36,1.37],[1.70,1.73,1.73],[3.27,3.31,3.29]],
 [[0.71,0.71,0.69],[1.20,1.26,1.26],[1.69,1.70,1.68],[3.27,3.24,3.23]],
 [[0.70,0.65,0.68],[1.24,1.22,1.30],[1.67,1.68,1.67],[3.25,3.26,3.26]]]
def level(cells):
    cells=[np.array(c) for c in cells if len(c)>0]
    n=np.array([len(c) for c in cells]); y=np.array([c.mean() for c in cells]); s=np.array([c.std(ddof=1) if len(c)>1 else np.nan for c in cells])
    p=len(cells); m=(n*y).sum()/n.sum()
    sr2=np.nansum((n-1)*s**2)/(n-1).sum()
    sd2=(n*(y-m)**2).sum()/(p-1); nb=(n.sum()-(n**2).sum()/n.sum())/(p-1)
    sL2=max(0,(sd2-sr2)/nb); return dict(p=p,m=m,sr=math.sqrt(sr2),sL=math.sqrt(sL2),sR=math.sqrt(sr2+sL2),y=y,s=s,n=n)
def grubbs(x):
    x=np.sort(np.array(x)); p=len(x); m=x.mean(); s=x.std(ddof=1)
    s0=((x-m)**2).sum()
    hi=x[:-2]; lo=x[2:]
    return dict(low=(m-x[0])/s, high=(x[-1]-m)/s, dlow=((lo-lo.mean())**2).sum()/s0, dhigh=((hi-hi.mean())**2).sum()/s0)
def cochran(s, w=False):
    v=np.array(s)**2; v=v[~np.isnan(v)]; return v.max()/v.sum()
print('--- C.1')
for j in range(4):
    L=level([C1[i][j] for i in range(8)]); g=grubbs(L['y'])
    print(j+1, f"m {L['m']:.4f} sr {L['sr']:.4f} sR {L['sR']:.4f} C {cochran(L['s']):.3f}", {k:round(v,3) for k,v in g.items()})
print('--- C.1 from rounded cell means (Table C.2)')
for j in range(4):
    L=level([C1[i][j] for i in range(8)]); g=grubbs(np.round(L['y'],3))
    print(j+1, {k:round(v,3) for k,v in g.items()}, 'C rounded s', round(cochran(np.round(L['s'],3)),3))
C2txt="""91.0 97.0 96.5 104.0|89.6 97.2 97.0 104.0
89.7 98.5 97.2 102.6|89.8 97.2 97.0 103.6
88.0 97.8 94.2 103.0|87.5 94.5 95.8 99.5
89.2 96.8 96.0 102.5|88.5 97.5 98.0 103.5
89.0 97.2 98.2 101.0|90.0 - 98.5 100.2
88.5 97.8 99.5 102.2|90.5 97.2 103.2 102.0
88.9 96.6 98.2 102.8|88.2 97.5 99.0 102.2
- 96.0 98.4 102.6|- 97.5 97.4 103.9
90.1 95.5 98.2 102.8|88.4 96.8 96.7 102.0
86.0 95.2 94.8 99.8|85.8 95.0 93.0 100.8
87.6 93.2 93.6 98.2|84.4 93.4 93.9 97.8
88.2 95.8 95.8 101.7|87.4 95.4 95.4 101.2
91.0 98.2 98.0 104.5|90.4 99.5 97.0 105.6
87.5 97.0 97.1 105.2|87.8 95.5 96.6 101.8
87.5 95.0 97.8 101.5|87.6 95.2 99.2 100.9
88.8 95.0 97.2 99.5|85.0 93.2 97.8 99.8"""
C2=[]
for line in C2txt.split('\n'):
    a,b=line.split('|'); a=a.split(); b=b.split()
    C2.append([[float(x) for x in (a[j],b[j]) if x!='-'] for j in range(4)])
# lab 5 level 2 single value dropped (8.4.3)
C2[4][1]=[]
print('--- C.2')
for j in range(4):
    L=level([C2[i][j] for i in range(16)]); g=grubbs(L['y'])
    print(j+1, L['p'], f"m {L['m']:.4f} sr {L['sr']:.4f} sR {L['sR']:.4f} C {cochran(L['s']):.3f}", {k:round(v,3) for k,v in g.items()})
C3=[[4.44,4.39,9.34,9.34,17.40,16.90,19.23,19.23,24.28,24.00],[4.03,4.23,8.42,8.33,14.42,14.50,16.06,16.22,20.40,19.91],
[3.70,3.70,7.60,7.40,13.60,13.60,14.50,15.10,19.30,19.70],[4.10,4.10,8.93,8.80,14.60,14.20,15.60,15.50,20.30,20.30],
[3.97,4.04,7.89,8.12,13.73,13.92,15.54,15.78,20.53,20.88],[3.75,4.03,8.76,9.24,13.90,14.06,16.42,16.58,18.56,16.58],
[3.70,3.80,8.00,8.30,14.10,14.20,14.90,16.00,19.70,20.50],[3.91,3.90,8.04,8.07,14.84,14.84,15.41,15.22,21.10,20.78],
[4.02,4.07,8.44,8.17,14.24,14.10,15.14,15.44,20.71,21.66]]
C3=[[r[2*j:2*j+2] for j in range(5)] for r in C3]
print('--- C.3 all labs')
for j in range(5):
    L=level([C3[i][j] for i in range(9)]); g=grubbs(L['y'])
    gl=grubbs(sorted(L['y'])[:-1])  # low after excluding high
    print(j+1, f"C {cochran(L['s']):.3f}", {k:round(v,3) for k,v in g.items()}, 'low w/o high', round(gl['low'],3))
    y=L['y']; sy=y.std(ddof=1); h=(y-y.mean())/sy; s=L['s']; k=s*math.sqrt(len(s))/math.sqrt((s**2).sum())
    print('   h', np.round(h,2), ' k', np.round(k,2))
print('--- C.3 final (lab 1 out, lab 6 level 5 out)')
fin=[]
for j in range(5):
    cells=[C3[i][j] for i in range(1,9) if not (i==5 and j==4)]
    L=level(cells); fin.append((L['m'],L['sr'],L['sR'])); print(j+1,L['p'],f"m {L['m']:.3f} sr {L['sr']:.4f} sR {L['sR']:.4f}")
def reml(cells):
    cells=[np.array(c) for c in cells if len(c)>0]
    n=np.array([len(c) for c in cells]); yb=np.array([c.mean() for c in cells]); ssw=sum(((c-c.mean())**2).sum() for c in cells)
    def nll(t):
        r2,L2=np.exp(t[0]),np.exp(t[1]) if len(t)>1 else 0.0
        v=r2+n*L2; w=n/v; mu=(w*yb).sum()/w.sum()
        return 0.5*(((n-1)*np.log(r2)+np.log(v)).sum()+np.log(w.sum())+ssw/r2+(w*(yb-mu)**2).sum())
    L=level(cells); t0=[math.log(L['sr']**2), math.log(max(L['sL'],L['sr']*0.1)**2)]
    r=optimize.minimize(nll,t0,method='Nelder-Mead',options=dict(xatol=1e-10,fatol=1e-12,maxiter=5000))
    r2,L2=np.exp(r.x); v=r2+n*L2; w=n/v; mu=(w*yb).sum()/w.sum()
    return mu, math.sqrt(r2), math.sqrt(r2+L2)
print('--- REML C.1'); [print(j+1, np.round(reml([C1[i][j] for i in range(8)]),4)) for j in range(4)]
print('--- REML C.2'); [print(j+1, np.round(reml([C2[i][j] for i in range(16)]),4)) for j in range(4)]
print('--- REML C.3'); [print(j+1, np.round(reml([C3[i][j] for i in range(1,9) if not (i==5 and j==4)]),4)) for j in range(5)]
# relationships with Table 1-4 data (rounded values as printed)
m=np.array([3.94,8.28,14.18,15.59,20.41]); s=np.array([0.092,0.179,0.127,0.337,0.393])
print('I b', (s/m).sum()/5)
def rel2(m,s,it=3):
    W=1/s**2; out=[]
    for _ in range(it):
        T1,T2,T3,T4,T5=W.sum(),(W*m).sum(),(W*m*m).sum(),(W*s).sum(),(W*m*s).sum()
        a=(T3*T4-T2*T5)/(T1*T3-T2**2); b=(T1*T5-T2*T4)/(T1*T3-T2**2); out.append((round(a,4),round(b,5))); W=1/(a+b*m)**2
    return out
def rel3(m,s,it=3):
    W=1/s**4; out=[]
    for _ in range(it):
        T1,T2,T3,T4,T5=W.sum(),(W*m**2).sum(),(W*m**4).sum(),(W*s**2).sum(),(W*m**2*s**2).sum()
        a2=(T3*T4-T2*T5)/(T1*T3-T2**2); b2=(T1*T5-T2*T4)/(T1*T3-T2**2); out.append((round(math.sqrt(a2),4),round(math.sqrt(b2),5))); W=1/(a2+b2*m**2)**2
    return out
print('II', rel2(m,s)); print('III', rel3(m,s))
lm,ls=np.log10(m),np.log10(s); q=5; T1,T2,T3,T4=lm.sum(),(lm**2).sum(),ls.sum(),(lm*ls).sum()
c=(T2*T3-T1*T4)/(q*T2-T1**2); d=(q*T4-T1*T3)/(q*T2-T1**2); print('IV c d', round(c,4), round(d,4), 'C', round(10**c,4))
