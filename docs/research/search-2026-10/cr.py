import json, sys, time, urllib.request, urllib.parse
def get(url, tries=6):
    for i in range(tries):
        try:
            req=urllib.request.Request(url, headers={"User-Agent":"uncertainty-literature-search/0.1 (https://github.com/unilker/uncertainty)"})
            with urllib.request.urlopen(req, timeout=90) as r: return json.load(r)
        except Exception as e:
            time.sleep(2**i+1)
    raise SystemExit("failed "+url)
tag, q, since = sys.argv[1], sys.argv[2], sys.argv[3] if len(sys.argv)>3 else "2015-01-01"
url="https://api.crossref.org/works?"+urllib.parse.urlencode({"query.bibliographic":q,"filter":f"from-pub-date:{since},type:journal-article","rows":100,"select":"DOI,title,container-title,issued,is-referenced-by-count,author,abstract"})
d=get(url)
with open(f"cr_{tag}.jsonl","w") as f:
    for w in d["message"]["items"]:
        y=(w.get("issued",{}).get("date-parts") or [[None]])[0][0]
        a=w.get("author") or []
        first=(a[0].get("family","") if a else "")
        f.write(json.dumps({"tag":tag,"doi":w.get("DOI"),"title":(w.get("title") or [""])[0],"venue":(w.get("container-title") or [""])[0],"year":y,"cited":w.get("is-referenced-by-count"),"first":first,"n_auth":len(a),"abstract":(w.get("abstract") or "")[:1500]},ensure_ascii=False)+"\n")
print(tag, len(d["message"]["items"]), d["message"]["total-results"])
