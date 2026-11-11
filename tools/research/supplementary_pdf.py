"""Markdown → PDF for the supplementary material (pandoc for HTML, Chromium through Playwright for the PDF).

    md_to_pdf(markdown, out_pdf, header, landscape=False)
    guide_markdown(lang) → the beginner's guide (chapters and all examples) as one Markdown document
"""
import os
import re
import subprocess
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
CSS = """
body { font-family: 'DejaVu Sans', sans-serif; font-size: 9.5pt; line-height: 1.4; color: #111; max-width: none; }
h1 { font-size: 17pt; color: #1B2F5E; border-bottom: 2px solid #D0101C; padding-bottom: 3pt; }
h2 { font-size: 13pt; color: #1B2F5E; margin-top: 16pt; }
h3 { font-size: 11pt; color: #1B2F5E; }
h4 { font-size: 10pt; }
h1, h2, h3, h4 { page-break-after: avoid; }
h1.chapter { page-break-before: always; }
nav#TOC { font-size: 10pt; page-break-after: always; }
nav#TOC ul { list-style: none; padding-left: 0; }
table { border-collapse: collapse; width: 100%; margin: 6pt 0; font-size: 8pt; page-break-inside: auto; table-layout: auto; }
col { width: auto !important; }
td:first-child { min-width: 30%; }
tr { page-break-inside: avoid; }
th { background: #1B2F5E; color: #fff; text-align: left; }
th, td { border: 0.5pt solid #bbb; padding: 2pt 4pt; vertical-align: top; }
tr:nth-child(even) td { background: #f4f5f8; }
code { font-family: 'DejaVu Sans Mono', monospace; font-size: 8pt; overflow-wrap: anywhere; }
pre { background: #f4f5f8; padding: 4pt; white-space: pre-wrap; font-size: 8pt; }
blockquote { border-left: 3pt solid #D0101C; margin-left: 0; padding-left: 8pt; color: #333; }
a { color: #1B2F5E; text-decoration: none; }
"""


def md_to_pdf(markdown, out_pdf, header, landscape=False, toc=False):
    with tempfile.TemporaryDirectory() as tmp:
        md, css, html = Path(tmp, "in.md"), Path(tmp, "style.css"), Path(tmp, "out.html")
        md.write_text(markdown, encoding="utf-8")
        css.write_text(CSS, encoding="utf-8")
        title = next((l[2:].strip() for l in markdown.splitlines() if l.startswith("# ")), header)
        subprocess.run(["pandoc", str(md), "-f", "markdown+pipe_tables+raw_html-implicit_figures", "-t", "html5",
                        "--standalone", "--embed-resources", "--css", str(css), "--metadata", f"pagetitle={title}",
                        *(["--toc", "--toc-depth=1", "--metadata", "toc-title=Contents"] if toc else []),
                        "-o", str(html)], check=True)
        subprocess.run(["node", str(HERE / "print_pdf.js"), str(html), str(out_pdf), header,
                        "landscape" if landscape else "portrait"], check=True)


def guide_markdown(lang="en"):
    """The chapters of the beginner's guide and all examples as one document, with in-document links."""
    root = HERE.parent.parent / "docs" / "guide" / "beginner" / lang
    files = sorted(p for p in root.glob("[0-9][0-9]-*.md")) + [root / "examples.md"]
    parts = []
    for i, f in enumerate(files):
        text = f.read_text(encoding="utf-8")
        # Links between the files become links inside the document.
        text = re.sub(r"\]\(([0-9][0-9]-[\w-]+|examples)\.md#([^)]+)\)", r"](#\2)", text)
        text = re.sub(r"\]\(([0-9][0-9]-[\w-]+|examples)\.md\)", r"](#ch-\1)", text)
        lines = text.split("\n", 1)
        head = lines[0]
        if head.startswith("# "):
            cls = "" if i == 0 else " .chapter"
            head = f"{head} {{#ch-{f.stem}{cls}}}"
        parts.append(head + "\n" + (lines[1] if len(lines) > 1 else ""))
    return "\n\n".join(parts)


FRONT = {
    "en": "Supplementary material S6. *Uncertainty from scratch*, the beginner's guide built into KAPSAM (chapters 1–13), "
          "followed by the step-by-step explanation of each of the {n} library examples as the application shows it on "
          "the **Overview** tab. The same text, in English and Turkish, is in the repository under `docs/guide/beginner/` "
          "and inside the application (book icon on the home screen).",
}


if __name__ == "__main__":
    import sys
    md_to_pdf(Path(sys.argv[1]).read_text(encoding="utf-8"), sys.argv[2], sys.argv[3] if len(sys.argv) > 3 else "")
