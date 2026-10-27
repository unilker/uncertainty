import math
exec(open('probe.py').read().split("for k,d in")[0])
def anal(d,c=1.5,it=3000):
    n=len(d); b=beta(c)
    pairs=[(r[0],r[1]) for r in d]+[(r[2],r[3]) for r in d]
    m=[(a+bb)/2 for a,bb in pairs]
    s=math.sqrt(sum((a-bb)**2/2 for a,bb in pairs)/len(pairs))
    for _ in range(it):
        ss=0
        for i,(a,bb) in enumerate(pairs):
            xa,xb=[min(max(x,m[i]-c*s),m[i]+c*s) for x in (a,bb)]
            m[i]=(xa+xb)/2; ss+=(xa-m[i])**2+(xb-m[i])**2
        s=math.sqrt(ss/(len(pairs)*b))
    return s,m
for k,d in (('A1',A1),('A2',A2)):
    sA,m=anal(d)
    print(k,'sA',sA,'pub',pub[k][3])
