"""JCGM GUM-5:2026 Example 2 (benzo[a]pyrene), 2.4.2: Monte Carlo method.

Bivariate normal for (f, m_ISE) (Table 2.1), bivariate t with nu = n - N = 1 and scale S/n for
(A_E, A_ISE) (2.4.2.3), M = 1e7, draws with a negative input rejected (2.4.2.4); median and shortest
95 % interval of m_E = f A_E / A_ISE m_ISE. Printed: median 0.014 ng, interval [~0, 0.032] ng.
Output (seed 1): 9 322 828 draws kept, median 0.01415, shortest [1.1e-7, 0.03206] ng.
Run: python3 -I tools/research/gum5/ex2_mcm.py (numpy).
"""
import numpy as np
rng=np.random.default_rng(1)
M=10_000_000
mu1=np.array([0.616,0.2455]); C1=np.array([[0.017**2,-3.3e-5],[-3.3e-5,0.0036**2]])
x=rng.multivariate_normal(mu1,C1,M)
S=np.array([[548861202,-1220621757],[-1220621757,11877338582]])/3
L=np.linalg.cholesky(S); nu=1
z=rng.standard_normal((M,2))@L.T; w=rng.chisquare(nu,M)
a=np.array([85114,917546])+z/np.sqrt(w/nu)[:,None]
keep=(x>0).all(1)&(a>0).all(1)
m=x[keep,0]*a[keep,0]/a[keep,1]*x[keep,1]
print(keep.sum())
def shortest(v,p=0.95):
    v=np.sort(v); n=len(v); q=int(round(p*n)); w=v[q:]-v[:n-q]; i=w.argmin(); return v[i],v[i+q]
print(np.median(m), shortest(m), np.quantile(m,[0.025,0.975]))
# without rejection
m2=x[:,0]*a[:,0]/a[:,1]*x[:,1]; print(np.median(m2),shortest(m2))
