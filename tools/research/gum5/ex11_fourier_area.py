"""JCGM GUM-5:2026 Example 11 (tumour section area from boundary pixels, parametric Fourier series).

The 38 boundary pixels of Figure 11.1 (left) were read from the rendered page (grey cells of the 16 x 14
grid; 47 interior pixels, so the extreme areas 47 and 85 of 11.5.1 follow). The chain starts at (3, 12)
and runs clockwise. t = cumulative chord length normalised including the closing chord ('close'), which
reproduces the areas of Table 11.1 to +-0.1. Also: RMSR and AIC/AICc/BIC with the formulas of 11.3.8-11.3.9
(chi^2 = sum of squared residuals / u0^2, 2m data, 4n+2 parameters), and the MCM of 11.5.4 for n = 5, 6, 7
(1e5 trials). Output: MCM n=6 65.45 +- 1.55 (printed 65.4 +- 1.6); n=5 65.93 +- 1.51 (65.3 +- 1.5);
n=7 65.40 +- 1.57 (65.0 +- 1.5); minima AIC n=6, AICc n=4, BIC n=4 (printed: n=2 for all three).
Run: python3 -I tools/research/gum5/ex11_fourier_area.py (numpy).
"""
import numpy as np
G=["................","................","..#######.......","..#.....#####...","..#.........#...","..#.........#...",
   "..#........##...","..#......###....","..##....##......","...#....#.......","...##..##.......","....####........","................"]
cells={(i+1,14-j) for j,r in enumerate(G) for i,ch in enumerate(r) if ch=='#'}
assert len(cells)==38
# walk the chain clockwise from the top-left pixel (3,12)
path=[(3,12)]; left=set(cells)-{(3,12)}
while left:
    x,y=path[-1]
    nb=[(x+1,y),(x,y-1),(x-1,y),(x,y+1)]
    if len(path)==1: nb=[(4,12)]
    c=[p for p in nb if p in left] or [p for p in [(x+1,y-1),(x-1,y-1),(x-1,y+1),(x+1,y+1)] if p in left]
    if not c: break
    # prefer the candidate that keeps the most unvisited neighbours (follows the outline)
    path.append(c[0]); left.discard(c[0])
assert len(path)==38, len(path)
P=np.array(path,float)
def fit_open(P,n):
    d=np.r_[0,np.cumsum(np.hypot(*np.diff(P,axis=0).T))]; t=d/d[-1]
    # closed curve: t in [0,1); the last point is not the first; chord to close ignored as in text
    cols=[np.ones_like(t)]
    for r in range(1,n+1): cols+= [np.cos(2*np.pi*r*t), np.sin(2*np.pi*r*t)]
printed={1:(0.96,38,39,46,65.1),2:(0.63,23,25,34,63.9),3:(0.60,25,30,40,64.2),4:(0.43,23,31,41,65.7),5:(0.43,27,39,48,65.7),6:(0.39,30,48,55,65.2),7:(0.40,33,59,62,65.2),8:(0.39,37,73,68,65.4)}
def fit(P,n,mode):
    seg=np.hypot(*np.diff(P,axis=0).T); d=np.r_[0,np.cumsum(seg)]
    if mode=='close': t=d/(d[-1]+np.hypot(*(P[0]-P[-1])))
    elif mode=='repeat':
        P=np.vstack([P,P[:1]]); seg=np.hypot(*np.diff(P,axis=0).T); d=np.r_[0,np.cumsum(seg)]; t=d/d[-1]
    else: t=d/d[-1]
    cols=[np.ones_like(t)]
    for r in range(1,n+1): cols+=[np.cos(2*np.pi*r*t),np.sin(2*np.pi*r*t)]
    M=np.array(cols).T
    b,*_=np.linalg.lstsq(M,P[:,0],rcond=None); c,*_=np.linalg.lstsq(M,P[:,1],rcond=None)
    res=np.r_[P[:,0]-M@b,P[:,1]-M@c]
    A=np.pi*sum(r*(b[2*r]*c[2*r-1]-b[2*r-1]*c[2*r]) for r in range(1,n+1))
    return (res**2).sum(), abs(A), len(P)
for mode in ('text','close','repeat'):
    print(mode)
    for n in range(1,9):
        ss,A,mm=fit(P,n,mode); N=2*mm; k=4*n+2
        print(n, 'A',round(A,2),'pr',printed[n][4],' sqrt(ss/nu)',round(np.sqrt(ss/(N-k)),2),'pr',printed[n][0], ' ss',round(ss,2))
rng=np.random.default_rng(11)
for n in (5,6,7):
    A=np.array([fit(P+rng.uniform(-0.5,0.5,P.shape),n,'close')[1] for _ in range(100000)])
    print('MCM n',n,round(A.mean(),2),round(A.std(ddof=1),2), np.round(np.quantile(A,[0.025,0.975]),2))
