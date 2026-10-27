import sys, openpyxl
wb = openpyxl.load_workbook(sys.argv[1], data_only=True, read_only=True)
ws = wb["Trueness"]
for rn, row in enumerate(ws.iter_rows(min_row=16, max_row=200, min_col=1, max_col=80), start=16):
    d = {c.column_letter: c.value for c in row if hasattr(c, "column_letter")}
    if d.get("BX") not in (None, "-", ""):
        print(rn, d.get("BX"), d.get("AG"), d.get("D"), d.get("BY"), d.get("BZ"), d.get("CA"))
    if d.get("BS") not in (None, "-", ""):
        print("RM/Spk", rn, d.get("BS"), d.get("K"), d.get("F"), d.get("BT"), d.get("BU"), d.get("BV"))
