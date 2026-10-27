import json, sys, time, urllib.request, urllib.parse
ISSN = ["0026-1394","0263-2241","0957-0233","0949-1775","0018-9456","0039-9140","0003-2670","0165-9936","1618-2642","0003-2654","0003-2700","2475-9066","2352-7110","2221-870X","2673-8244","2107-6839","0169-7439","1434-6621","0026-265X","0171-8096","0970-3950","0141-6359","1361-6501","1879-0682"]
def get(url, tries=6):
    for i in range(tries):
        try:
            req=urllib.request.Request(url, headers={"User-Agent":"uncertainty-literature-search/0.1 (https://github.com/unilker/uncertainty)"})
            with urllib.request.urlopen(req, timeout=90) as r: return json.load(r)
        except Exception as e:
            time.sleep(2**i+1)
    raise SystemExit("failed")
tag,q,since=sys.argv[1],sys.argv[2],sys.argv[3]
flt=",".join([f"from-pub-date:{since}","type:journal-article"]+[f"issn:{x}" for x in ISSN])
url="https://api.crossref.org/works?"+urllib.parse.urlencode({"query.bibliographic":q,"filter":flt,"rows":60,"select":"DOI,title,container-title,issued,is-referenced-by-count,author,abstract"})
d=get(url)
with open(f"k_{tag}.jsonl","w") as f:
    for i,w in enumerate(d["message"]["items"]):
        y=(w.get("issued",{}).get("date-parts") or [[None]])[0][0]
        a=w.get("author") or []
        f.write(json.dumps({"tag":tag,"rank":i,"doi":w.get("DOI"),"title":(w.get("title") or [""])[0],"venue":(w.get("container-title") or [""])[0],"year":y,"cited":w.get("is-referenced-by-count"),"authors":"; ".join(f"{x.get('family','')} {x.get('given','')[:1]}" for x in a[:6])+(" et al." if len(a)>6 else ""),"abstract":(w.get("abstract") or "")[:2000]},ensure_ascii=False)+"\n")
print(tag,len(d["message"]["items"]),d["message"]["total-results"])
