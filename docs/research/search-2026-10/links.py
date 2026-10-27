import json, sys, urllib.request
for doi in sys.argv[1:]:
    try:
        r = json.load(urllib.request.urlopen(f"https://api.crossref.org/works/{doi}", timeout=30))["message"]
        links = [(l.get("content-type"), l.get("URL")) for l in r.get("link", [])]
        lic = [l.get("URL") for l in r.get("license", [])]
        print(doi, "|", r.get("publisher"), "|", lic[:1], "|", links[:3])
    except Exception as e:
        print(doi, "ERR", e)
