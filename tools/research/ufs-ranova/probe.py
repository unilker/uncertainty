import math
from scipy import stats, optimize
A1=[[3898,4139,4466,4693],[3910,3993,4201,4126],[5708,5903,4061,3782],[5028,4754,5450,5416],
[4640,4401,4248,4191],[5182,5023,4662,4839],[3028,3224,3023,2901],[3966,4283,4131,3788]]
A2=[[787,769,811,780],[338,327,651,563],[289,297,211,204],[662,702,238,246],[229,215,208,218],
[346,374,525,520],[324,321,77,73],[56,61,116,120],[189,189,176,168],[61,61,91,119]]
pub={'A1':(4408.3237,565.39868,319.04834,167.94308),'A2':(297.31,179.67,123.81,11.144)}
def beta(c): return 2*stats.norm.cdf(c)-1-2*c*stats.norm.pdf(c)+2*c*c*(1-stats.norm.cdf(c))
def huber(x,c=1.5,div=0,it=2000):
    n=len(x); m=sorted(x)[n//2] if n%2 else sum(sorted(x)[n//2-1:n//2+1])/2
    s=1.4826*sorted(abs(v-m) for v in x)[n//2]
    b=beta(c)
    for _ in range(it):
        w=[min(max(v,m-c*s),m+c*s) for v in x]
        m=sum(w)/n; s=math.sqrt(sum((v-m)**2 for v in w)/((n-div)*b))
    return m,s
for k,d in (('A1',A1),('A2',A2)):
    M=[sum(r)/4 for r in d]
    print(k,'classical mean',sum(M)/len(M),'pub',pub[k][0])
    for div in (0,1):
        print('  huber M div',div,huber(M,div=div))
    allv=[v for r in d for v in r]
    print('  huber all',huber(allv), huber(allv,div=1))
print('---- levels')
for k,d in (('A1',A1),('A2',A2)):
    n=len(d)
    da=[r[0]-r[1] for r in d]+[r[2]-r[3] for r in d]
    # Huber about zero for differences? location free
    for div in (0,1):
        m,s=huber(da,div=div); print(k,'anal diffs div',div,'sA',s/math.sqrt(2),'loc',m)
    D=[(r[0]+r[1])/2-(r[2]+r[3])/2 for r in d]
    for div in (0,1):
        m,s=huber(D,div=div); print(k,'samp D div',div,'s/sqrt2',s/math.sqrt(2),'loc',m)
    # pooled values within sample: deviations from pair mean
    print(k,'pub',pub[k])
