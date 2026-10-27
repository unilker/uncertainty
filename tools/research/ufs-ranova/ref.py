import math
exec(open('probe.py').read().split("def beta")[0])
A3=[[0.815,0.834,0.912,0.893],[1.8,1.83,1.94,1.93],[1.69,1.68,1.79,1.77],[2.62,2.61,2.83,2.84],[1.66,1.63,1.58,1.59],[1.52,1.53,1.47,1.50]]
A4=[[402,325,361,351],[382,319,349,362],[332,291,397,348],[280,278,358,321],[370,409,378,460],[344,318,381,392],[297,333,341,315],[336,320,292,306],[372,353,332,337],[407,361,322,382]]
A4s=[[400,491,323,355],[413,159,392,434],[315,391,252,454],[223,220,357,469],[462,343,262,293],[353,265,305,456],[298,234,152,323],[425,263,417,353],[622,189,291,272],[292,397,142,568]]
def cl(d):
    n=len(d); allv=[v for r in d for v in r]; gm=sum(allv)/len(allv)
    ssa=sum((r[0]-r[1])**2/2+(r[2]-r[3])**2/2 for r in d)
    sss=sum(2*(((r[0]+r[1])/2-(sum(r)/4))**2+((r[2]+r[3])/2-sum(r)/4)**2) for r in d)
    sst=sum(4*(sum(r)/4-gm)**2 for r in d)
    va=ssa/(2*n); vs=(sss/n-va)/2; vt=(sst/(n-1)-sss/n)/4
    return dict(mean=gm,ssa=ssa,sss=sss,sst=sst,vt=vt,vs=vs,va=va,sA=math.sqrt(va),sS=math.sqrt(max(vs,0)),sT=math.sqrt(max(vt,0)))
for name,d in [('A1',A1),('A2',A2),('A3',A3),('A4',A4),('A4s',A4s)]:
    r=cl(d); print(name,{k:round(v,6) for k,v in r.items()})
    m=r['mean']; print('   rel% A S T', 100*r['sA']/m, 100*r['sS']/m, 100*r['sT']/m, 'meas',100*math.sqrt(r['sA']**2+r['sS']**2)/m)
L=[[math.log(v) for v in row] for row in A2]
r=cl(L); print('A2 log',r); sm=math.sqrt(r['va']+max(r['vs'],0)); print(' sGmeas',sm,'FU',math.exp(2*sm),math.exp(2*r['sS']),math.exp(2*r['sA']),'u',math.sqrt(math.exp(sm**2)-1))
print('A2 ln total', math.sqrt(r['va']+r['vs']+r['vt']))
# range A3
R1=[abs(r[0]-r[1]) for r in A3]; R2=[abs(r[2]-r[3]) for r in A3]
RA=(sum(R1)/6+sum(R2)/6)/2; sA=RA/1.128
RS=[abs((r[0]+r[1])/2-(r[2]+r[3])/2) for r in A3]; sSA=sum(RS)/6/1.128; sS=math.sqrt(sSA**2-sA**2/2)
M=[sum(r)/4 for r in A3]; mm=sum(M)/6; ST=math.sqrt(sum((x-mm)**2 for x in M)/5); sT=math.sqrt(ST**2-sSA**2/2)
print('range',RA,sA,sSA,sS,ST,sT,mm,'CV',100*sA/mm,100*sS/mm,100*sT/mm,'meas',100*math.sqrt(sA**2+sS**2)/mm)
print('range with 2/sqrt(pi)',RA/(2/math.sqrt(math.pi)))
