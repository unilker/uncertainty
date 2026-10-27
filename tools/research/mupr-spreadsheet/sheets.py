import sys, openpyxl
wb = openpyxl.load_workbook(sys.argv[1], data_only=True, read_only=True)
print(wb.sheetnames)
ws = wb[sys.argv[2]]
r0, r1, c1 = int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5])
for row in ws.iter_rows(min_row=r0, max_row=r1, max_col=c1):
    vals = [(c.coordinate, c.value if not isinstance(c.value, float) else round(c.value, 6)) for c in row if c.value not in (None, "")]
    if vals: print(vals)
