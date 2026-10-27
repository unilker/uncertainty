import json, sys, time, urllib.request, urllib.parse
def get(url, tries=6):
    for i in range(tries):
        try:
            req=urllib.request.Request(url, headers={"User-Agent":"uncertainty-literature-search/0.1"})
            with urllib.request.urlopen(req, timeout=60) as r: return json.load(r)
        except urllib.error.HTTPError as e:
            if e.code==429: time.sleep(2**i+1); continue
            raise
    raise SystemExit("rate limited")
q = sys.argv[1]
url = "https://api.openalex.org/works?" + urllib.parse.urlencode({"search": q, "filter": "from_publication_date:2008-01-01", "per-page": 50, "sort":"relevance_score:desc"})
d = get(url)
print(d["meta"]["count"])
for w in d["results"]:
    src=(w.get("primary_location") or {}).get("source") or {}
    print(json.dumps({"id":w["id"],"doi":w.get("doi"),"title":w.get("title"),"year":w.get("publication_year"),"venue":src.get("display_name"),"cited":w.get("cited_by_count"),"oa":(w.get("open_access") or {}).get("is_oa")}, ensure_ascii=False))
