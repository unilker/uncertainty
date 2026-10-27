import sys, openpyxl
from openpyxl.utils import column_index_from_string as ci
wb = openpyxl.load_workbook(sys.argv[1], data_only=(sys.argv[3]=="v"), read_only=True)
ws = wb[sys.argv[2]]
cols = sys.argv[6].split(",")
r0, r1 = int(sys.argv[4]), int(sys.argv[5])
for rn, row in enumerate(ws.iter_rows(min_row=r0, max_row=r1, max_col=90), start=r0):
    d = {c.column_letter: c.value for c in row if hasattr(c, "column_letter")}
    out = []
    for k in cols:
        v = d.get(k)
        if isinstance(v, float): v = round(v, 6)
        out.append(f"{k}={v}")
    if any(d.get(k) not in (None, "") for k in cols[1:]):
        print(rn, " ".join(out)[:600])
