"""Recalculates KAPSAM's spreadsheet verification workbooks in LibreOffice Calc and reads their sheet Check.

For every library project that has a verification workbook, `kapsam run <project> -o <file>.xlsx` writes the
workbook; LibreOffice (headless, through its UNO interface) opens it, recalculates every formula and the script
reads KAPSAM's value, the spreadsheet's value, the difference and the tolerance of every row of the sheet Check.

    python3 tools/research/spreadsheet_check.py [kapsam binary] [output.md]

Needs LibreOffice with its Python bridge (`import uno`). Writes a Markdown table (default
docs/research/spreadsheet-check.md) and exits with status 1 if a row is outside its tolerance.
"""
import glob
import json
import os
import re
import subprocess
import sys
import tempfile
import time

import uno
from com.sun.star.beans import PropertyValue

BIN = sys.argv[1] if len(sys.argv) > 1 else "target/release/kapsam"
OUT = sys.argv[2] if len(sys.argv) > 2 else "docs/research/spreadsheet-check.md"


def prop(name, value):
    p = PropertyValue()
    p.Name, p.Value = name, value
    return p


def connect(port):
    soffice = subprocess.Popen(
        ["soffice", "--headless", "--invisible", "--nologo", "--norestore", "--nodefault",
         f"--accept=socket,host=127.0.0.1,port={port};urp;"],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    local = uno.getComponentContext()
    resolver = local.ServiceManager.createInstanceWithContext("com.sun.star.bridge.UnoUrlResolver", local)
    for _ in range(120):
        try:
            ctx = resolver.resolve(f"uno:socket,host=127.0.0.1,port={port};urp;StarOffice.ComponentContext")
            desktop = ctx.ServiceManager.createInstanceWithContext("com.sun.star.frame.Desktop", ctx)
            return soffice, desktop
        except Exception:  # noqa: BLE001 - LibreOffice is still starting
            time.sleep(0.5)
    soffice.kill()
    raise SystemExit("LibreOffice did not start")


REF = re.compile(r"=(?:\$?'?([A-Za-z][\w ]*)'?[.!])?\$?([A-Z]{1,3})\$?(\d+)")


def is_copy(doc, cell, sheet, depth=0):
    """True when the cell holds a constant, or a bare reference (through any chain of bare references) to a cell
    that holds a constant: such a row checks that an input was transcribed, not the spreadsheet's arithmetic."""
    formula = cell.getFormula()
    if not formula.startswith("="):
        return True
    m = REF.fullmatch(formula)
    if not m or depth > 10:
        return False
    name = (m.group(1) or "").lstrip("$")
    if name and not doc.Sheets.hasByName(name):
        raise SystemExit(f"unresolved reference {formula!r} (sheet {name!r}; sheets {doc.Sheets.getElementNames()})")
    target_sheet = doc.Sheets.getByName(name) if name else sheet
    target = target_sheet.getCellRangeByName(m.group(2) + m.group(3))
    return is_copy(doc, target, target_sheet, depth + 1)


def read_check(desktop, path):
    url = uno.systemPathToFileUrl(os.path.abspath(path))
    doc = desktop.loadComponentFromURL(url, "_blank", 0, (prop("Hidden", True),))
    try:
        doc.calculateAll()
        sheet = doc.Sheets.getByIndex(1)  # About, Check, …
        rows = []
        r = 4
        while True:
            what = sheet.getCellByPosition(0, r).getString()
            if not what:
                break
            cells = [sheet.getCellByPosition(c, r) for c in range(1, 5)]
            err = [c.getError() for c in cells]
            vals = [c.getValue() for c in cells]
            # A row is "computed" when the spreadsheet value comes from a calculation; it is a copy when the cell is
            # a constant or a bare reference to one input cell (an input value or a fixed coverage factor).
            copy = is_copy(doc, cells[1], sheet)
            rows.append({"what": what, "kapsam": vals[0], "sheet": vals[1], "diff": vals[2], "tol": vals[3],
                         "error": any(err), "copy": copy})
            r += 1
        return rows
    finally:
        doc.close(True)


def recalculated_copy(src, dst, port=2005):
    """Opens a workbook in LibreOffice, recalculates it and saves it as .xlsx with the computed values stored,
    so that viewers that do not calculate (previews, phones) also show the results."""
    soffice, desktop = connect(port)
    try:
        doc = desktop.loadComponentFromURL(uno.systemPathToFileUrl(os.path.abspath(src)), "_blank", 0,
                                           (prop("Hidden", True),))
        doc.calculateAll()
        doc.storeToURL(uno.systemPathToFileUrl(os.path.abspath(dst)), (prop("FilterName", "Calc MS Excel 2007 XML"),))
        doc.close(True)
    finally:
        try:
            desktop.terminate()
        except Exception:  # noqa: BLE001
            pass
        soffice.wait(timeout=30)


def main():
    projects = []
    with tempfile.TemporaryDirectory() as tmp:
        for f in sorted(glob.glob(os.environ.get("LIBRARY_GLOB", "library/*/*.json"))):
            pid = json.load(open(f, encoding="utf-8"))["metadata"]["id"]
            x = os.path.join(tmp, pid + ".xlsx")
            r = subprocess.run([BIN, "run", f, "--no-mcm", "-o", x], capture_output=True, text=True)
            if r.returncode == 0:
                projects.append((pid, x))
        soffice, desktop = connect(2002)
        try:
            results = [(pid, read_check(desktop, x)) for pid, x in projects]
        finally:
            try:
                desktop.terminate()
            except Exception:  # noqa: BLE001
                pass
            soffice.wait(timeout=30)
    bad = 0
    lines = ["# Spreadsheet verification of the library", "",
             "Generated by `python3 tools/research/spreadsheet_check.py` (research log D94): for every project with a "
             "verification workbook, KAPSAM writes the workbook, LibreOffice Calc recalculates it and the rows of its "
             "sheet Check are compared with their tolerances.", "",
             f"{len(results)} workbooks, {sum(len(r) for _, r in results)} compared values, of which "
             f"{sum(1 for _, r in results for q in r if not q['copy'])} are computed by spreadsheet formulas and "
             f"{sum(1 for _, r in results for q in r if q['copy'])} repeat an input or a fixed factor (a check of the "
             "transcription, not of the arithmetic).", "",
             "| Project | values (computed) | largest difference / tolerance | worst quantity | result |",
             "|---|---:|---:|---|---|"]
    for pid, rows in results:
        worst = max(rows, key=lambda q: (q["error"], q["diff"] / q["tol"] if q["tol"] else 0))
        ratio = worst["diff"] / worst["tol"] if worst["tol"] else 0
        ok = not worst["error"] and all(q["diff"] <= q["tol"] for q in rows)
        bad += not ok
        what = worst["what"].replace("|", "∣")
        computed = sum(1 for q in rows if not q["copy"])
        lines.append(f"| {pid} | {len(rows)} ({computed}) | {ratio:.2g} | {what} | {'OK' if ok else 'FAIL'} |")
    lines += ["", f"Workbooks outside a tolerance: {bad}."]
    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines) + "\n")
    print(f"{len(results)} workbooks, {bad} outside a tolerance; table in {OUT}")
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
