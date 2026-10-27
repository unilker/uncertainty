import sys, openpyxl
wb = openpyxl.load_workbook(sys.argv[1], data_only=True, read_only=True)
ws = wb[sys.argv[2]]
r0, r1, c0, c1 = map(int, sys.argv[3:7])
for rn, row in enumerate(ws.iter_rows(min_row=r0, max_row=r1, min_col=c0, max_col=c1), start=r0):
    vals = [(getattr(c, "coordinate", "?"), round(c.value, 6) if isinstance(c.value, float) else c.value) for c in row if c.value not in (None, "", "-")]
    if vals: print(rn, vals)
