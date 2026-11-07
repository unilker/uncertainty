import re, sys, html
src=open(sys.argv[1],encoding='utf-8').read()
out=[]
for page in re.findall(r'<page[^>]*>(.*?)</page>', src, re.S):
    words=[(float(a),float(b),float(c),float(d),html.unescape(t)) for a,b,c,d,t in re.findall(r'<word xMin="([\d.]+)" yMin="([\d.]+)" xMax="([\d.]+)" yMax="([\d.]+)">(.*?)</word>', page)]
    lines=[]
    for w in words:
        for L in lines:
            if abs(L['y']-w[1])<2.0 and w[0]>=L['x']-1:
                gap=w[0]-L['x']; h=w[3]-w[1]
                L['t']+=('' if gap<0.22*h else ' ')+w[4]; L['x']=w[2]; break
        else:
            lines.append({'y':w[1],'x':w[2],'t':w[4]})
    lines.sort(key=lambda L:L['y'])
    out.append('\n'.join(L['t'] for L in lines)); out.append('\f')
open(sys.argv[2],'w',encoding='utf-8').write('\n'.join(out))
