import math, statistics as st
T1="""C279a Cr 875.80 21.57 25.22 33.18 3.8
C279a Cu 123.77 4.01 4.08 5.72 4.6
C279a Ni 16.28 0.34 0.00 0.34 2.1
C279a Pb 27.29 0.41 0.19 0.45 1.7
C279a Zn 58.70 8.19 0.00 8.19 13.9
C282a Cr 304.53 7.85 5.98 9.87 3.2
C282a Cu 46.80 1.53 0.68 1.67 3.6
C282a Ni 35.79 0.97 0.52 1.10 3.1
C282a Pb 124.56 6.78 2.44 7.21 5.8
C282a Zn 107.02 1.82 2.77 3.32 3.1
LGC6145 Cr 41.50 0.80 0.91 1.22 2.9
LGC6145 Cu 49.83 2.04 0.00 2.04 4.1
LGC6145 Ni 20.24 1.36 0.44 1.43 7.1
LGC6145 Pb 37.55 0.88 0.00 0.88 2.3
LGC6145 Zn 136.00 3.74 3.36 5.03 3.7
LGC6187 Cr 75.66 1.45 2.53 2.92 3.9
LGC6187 Cu 73.08 1.46 1.94 2.43 3.3
LGC6187 Ni 30.21 0.44 0.37 0.57 1.9
LGC6187 Pb 67.14 0.65 0.73 0.98 1.5
LGC6187 Zn 402.62 6.47 5.46 8.46 2.1"""
T2="""1007.50 14.55 875.80 8.89 -131.7 17.0 -13.1 1.7
136.69 2.87 123.77 1.50 -12.9 3.2 -9.5 2.4
18.16 0.51 16.28 0.07 -1.9 0.5 -10.3 2.8
30.00 0.83 27.29 0.10 -2.7 0.8 -9.0 2.8
64.40 1.63 58.70 1.75 -5.7 2.4 -8.8 3.7
321.00 13.88 304.53 2.46 -16.5 14.1 -5.1 4.4
55.30 1.84 46.80 0.39 -8.5 1.9 -15.4 3.4
38.60 2.17 35.79 0.26 -2.8 2.2 -7.3 5.7
136.91 8.27 124.56 1.62 -12.4 8.4 -9.0 6.2
115.90 2.75 107.02 0.92 -8.9 2.9 -7.7 2.5
47.60 0.90 41.50 0.32 -6.1 1.0 -12.8 2.0
62.20 1.80 49.83 0.44 -12.4 1.9 -19.9 3.0
39.00 1.25 20.24 0.32 -18.8 1.3 -48.1 3.3
45.10 1.15 37.55 0.19 -7.6 1.2 -16.8 2.6
137.00 3.00 136.00 1.29 -1.0 3.3 -0.7 2.4
84.00 4.70 75.66 0.82 -8.3 4.8 -9.9 5.7
83.60 2.05 73.08 0.66 -10.5 2.2 -12.6 2.6
34.70 0.85 30.21 0.15 -4.5 0.9 -12.9 2.5
77.20 2.25 67.14 0.26 -10.1 2.3 -13.0 2.9
439.00 13.00 402.62 2.15 -36.4 13.2 -8.3 3.0"""
rows1=[l.split() for l in T1.splitlines()]; rows2=[list(map(float,l.split())) for l in T2.splitlines()]
rsd=[]; relu=[]
for a,b in zip(rows1,rows2):
    mat,met=a[0],a[1]; mean,sw,sb,sI,RSD=map(float,a[2:])
    sIc=math.hypot(sw,sb); RSDc=100*sIc/mean
    xref,uref,xobs,uobs,bias,ub,rb,rub=b
    uobs_c=math.sqrt((sIc**2-0.5*sw**2)/11); ub_c=math.hypot(uobs_c,uref)
    flags=[]
    if abs(sIc-sI)>0.006: flags.append(f"sI {sIc:.2f}")
    if abs(RSDc-RSD)>0.051: flags.append(f"RSD {RSDc:.2f}")
    if abs(uobs_c-uobs)>0.006: flags.append(f"uobs {uobs_c:.3f}")
    if abs(ub_c-ub)>0.051: flags.append(f"u(d) {ub_c:.2f}")
    if abs((xobs-xref)-bias)>0.051: flags.append(f"bias {xobs-xref:.2f}")
    if abs(100*(xobs-xref)/xref-rb)>0.051: flags.append(f"rel bias {100*(xobs-xref)/xref:.2f}")
    if abs(100*ub_c/xref-rub)>0.051: flags.append(f"rel u {100*ub_c/xref:.2f}")
    print(mat,met,flags)
    rsd.append((mat,met,RSDc)); relu.append(100*ub_c/xref)
r=[v for m,e,v in rsd if not (m=='C279a' and e=='Zn')]
print('median RSD excl', st.median(r), 'rms RSD excl', math.sqrt(sum(v*v for v in r)/len(r)), 'rms all', math.sqrt(sum(v*v for _,_,v in rsd)/20))
print('relu rms', math.sqrt(sum(v*v for v in relu)/20), 'median', st.median(relu), 'mean', st.mean(relu))
u2=0.033**2+0.036**2+(0.004*2.89)**2+(0.0009*2.89)**2+(0.985/984.74)**2; print('u2',u2, math.sqrt(u2), 'U at 27.8', 2*math.sqrt(u2)*27.8, 'at 27.29', 2*math.sqrt(u2)*27.29)
print('5/sqrt3', 5/math.sqrt(3))
