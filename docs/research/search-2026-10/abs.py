import json, sys, time, urllib.request, html, re
for doi in sys.argv[1:]:
    for i in range(5):
        try:
            req=urllib.request.Request("https://api.crossref.org/works/"+doi, headers={"User-Agent":"uncertainty-literature-search/0.1 (https://github.com/unilker/uncertainty)"})
            m=json.load(urllib.request.urlopen(req, timeout=60))["message"]; break
        except Exception as e: time.sleep(2**i); m=None
    if not m: print(doi,"FAILED"); continue
    a=html.unescape(re.sub("<[^>]+>"," ",m.get("abstract") or "")).strip()
    au="; ".join(f"{x.get('family','')} {x.get('given','')}" for x in (m.get("author") or [])[:8])
    y=(m.get("issued",{}).get("date-parts") or [[None]])[0]
    print(f"== {doi}\n{au}\n{(m.get('title') or [''])[0]} | {(m.get('container-title') or [''])[0]} {m.get('volume','')} ({y[0]}) {m.get('page','') or m.get('article-number','')} | cited {m.get('is-referenced-by-count')}\n{re.sub(r'\s+',' ',a)[:1300]}\n")
