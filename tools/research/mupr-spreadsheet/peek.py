import sys, openpyxl
wb = openpyxl.load_workbook(sys.argv[1], data_only=True, read_only=True)
print(wb.sheetnames)
ws = wb["Performance_Model"] if "Performance_Model" in wb.sheetnames else None
if ws is None:
    for n in wb.sheetnames:
        if "erform" in n: ws = wb[n]
print(ws.title)
for row in ws.iter_rows(min_row=1, max_row=20, max_col=40):
    vals = [(c.coordinate, c.value) for c in row if c.value is not None]
    if vals: print(vals)
