import json, sys, time, urllib.request, urllib.parse
def get(url, tries=6):
    for i in range(tries):
        try:
            req=urllib.request.Request(url, headers={"User-Agent":"uncertainty-literature-search/0.1 (https://github.com/unilker/uncertainty)"})
            with urllib.request.urlopen(req, timeout=90) as r: return json.load(r)
        except Exception as e: time.sleep(2**i+1)
    raise SystemExit("failed")
tag,q,since=sys.argv[1],sys.argv[2],sys.argv[3]
url="https://api.crossref.org/works?"+urllib.parse.urlencode({"query.bibliographic":q,"filter":f"from-pub-date:{since}","rows":25,"select":"DOI,title,container-title,issued,is-referenced-by-count,author,type"})
d=get(url)
for w in d["message"]["items"]:
    y=(w.get("issued",{}).get("date-parts") or [[None]])[0][0]
    a=w.get("author") or []
    print(f'{tag}|{y}|{w.get("is-referenced-by-count")}|{(w.get("container-title") or [""])[0][:30]}|{(w.get("title") or [""])[0][:140]}|{w.get("DOI")}|{(a[0].get("family","") if a else "")}|{w.get("type")}')
