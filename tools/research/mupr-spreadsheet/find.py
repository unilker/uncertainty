import sys, openpyxl
wb = openpyxl.load_workbook(sys.argv[1], data_only=(sys.argv[3]=="v"), read_only=True)
ws = wb[sys.argv[2]]
r0, r1, c0, c1 = map(int, sys.argv[4:8])
for row in ws.iter_rows(min_row=r0, max_row=r1, min_col=c0, max_col=c1):
    for c in row:
        if c.value not in (None, ""):
            v = c.value
            if isinstance(v, float): v = round(v, 6)
            print(c.coordinate, repr(v)[:300])
