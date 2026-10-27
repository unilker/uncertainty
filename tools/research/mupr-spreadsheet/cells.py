import sys, openpyxl
wb = openpyxl.load_workbook(sys.argv[1], data_only=False, read_only=True)
ws = wb[sys.argv[2]]
want = set(sys.argv[3].split(","))
import re
rows = sorted({int(re.sub(r"[A-Z]", "", c)) for c in want})
for rn, row in enumerate(ws.iter_rows(min_row=rows[0], max_row=rows[-1], max_col=120), start=rows[0]):
    for c in row:
        co = getattr(c, "coordinate", None)
        if co in want: print(co, c.value)
