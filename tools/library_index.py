"""Rewrites the example tables of library/README.md from the project files.

Usage:  python tools/library_index.py   (run from the repository root)
"""

import json
from pathlib import Path

LIB = Path("library")
README = LIB / "README.md"
FIELDS = {
    "chemistry": "Chemistry / Kimya",
    "electrical": "Electrical and RF / Elektrik ve RF",
    "flow": "Flow and volume / Akış ve hacim",
    "general": "General (method validation) / Genel (yöntem doğrulama)",
    "length": "Length / Uzunluk",
    "mass": "Mass / Kütle",
    "physical": "Physical properties / Fiziksel özellikler",
    "temperature": "Temperature / Sıcaklık",
}
START = "**"
END = "## Adding an example"


def text(t, lang):
    return t if isinstance(t, str) else t.get(lang) or t.get("en", "")


def main():
    rows = {}
    counts = {"model": 0, "topdown": 0}
    for path in sorted(LIB.glob("*/*.json")):
        p = json.loads(path.read_text(encoding="utf-8"))
        m = p["metadata"]
        kind = "topdown" if "topdown" in p else "model"
        counts[kind] += 1
        s = (m.get("sources") or [{}])[0]
        source = " ".join(x for x in [s.get("document", "").split(",")[0], s.get("section", "")] if x)
        rel = path.relative_to(LIB).as_posix()
        row = f"| [`{m['id']}`]({rel}) | {text(m['title'], 'en')} | {text(m['title'], 'tr')} | {source} |"
        rows.setdefault((m.get("field", "general"), kind), []).append(row)
    out = [f"**{sum(counts.values())} examples / örnek** ({counts['model']} model, {counts['topdown']} top-down).", ""]
    for field, heading in FIELDS.items():
        for kind in ("model", "topdown"):
            if (field, kind) not in rows:
                continue
            title = heading if kind == "model" else heading.replace(" / ", ", top-down / ") + ", top-down"
            out += [f"## {title}", "", "| File / Dosya | Title | Başlık | Source / Kaynak |", "|---|---|---|---|"]
            out += rows.pop((field, kind)) + [""]
    if rows:
        raise SystemExit(f"unknown fields: {sorted(rows)}")
    doc = README.read_text(encoding="utf-8")
    head = doc[: doc.index("\n" + START) + 1]
    tail = doc[doc.index(END):]
    README.write_text(head + "\n".join(out) + "\n" + tail, encoding="utf-8")


if __name__ == "__main__":
    main()
