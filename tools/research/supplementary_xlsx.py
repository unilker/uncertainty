"""Formatted Excel tables for the supplementary material (openpyxl): one row per record, one field per cell,
a styled and frozen header row with filters, wrapped text, column widths, and a sheet that explains the columns.
"""
import re

from openpyxl import Workbook
from openpyxl.styles import Alignment, Border, Font, PatternFill, Side
from openpyxl.utils import get_column_letter

NAVY = "1B2F5E"
RED = "D0101C"
STRIPE = "F4F5F8"


def num(s):
    """A plain number as a number (so that Excel can sort and compute with it), anything else as text."""
    s = (s or "").strip()
    if re.fullmatch(r"[+-]?(\d+(\.\d*)?|\.\d+)([eE][+-]?\d+)?", s):
        v = float(s)
        return int(v) if v.is_integer() and "." not in s and "e" not in s.lower() else v
    return s


def table(path, sheet, title, columns, rows, notes=(), total_row=None):
    wb = Workbook()
    ws = wb.active
    ws.title = sheet
    thin = Side(style="thin", color="BBBBBB")
    border = Border(left=thin, right=thin, top=thin, bottom=thin)
    ws.cell(row=1, column=1, value=title).font = Font(bold=True, size=14, color=NAVY)
    ws.cell(row=2, column=1, value="KAPSAM, TÜBİTAK UME — https://github.com/unilker/uncertainty").font = Font(
        italic=True, size=9, color="595959")
    head = 4
    for c, (name, width) in enumerate(columns, start=1):
        cell = ws.cell(row=head, column=c, value=name)
        cell.font = Font(bold=True, color="FFFFFF")
        cell.fill = PatternFill("solid", fgColor=NAVY)
        cell.alignment = Alignment(wrap_text=True, vertical="center")
        cell.border = border
        ws.column_dimensions[get_column_letter(c)].width = width
    ws.row_dimensions[head].height = 30
    for i, row in enumerate(rows):
        r = head + 1 + i
        for c, v in enumerate(row, start=1):
            cell = ws.cell(row=r, column=c, value=v if v != "" else None)
            cell.alignment = Alignment(wrap_text=True, vertical="top")
            cell.border = border
            if i % 2:
                cell.fill = PatternFill("solid", fgColor=STRIPE)
    last = head + len(rows)
    if total_row:
        r = last + 1
        for c, v in enumerate(total_row, start=1):
            cell = ws.cell(row=r, column=c, value=v if v != "" else None)
            cell.font = Font(bold=True)
            cell.border = Border(top=Side(style="medium", color=RED))
    ws.freeze_panes = ws.cell(row=head + 1, column=2)
    ws.auto_filter.ref = f"A{head}:{get_column_letter(len(columns))}{last}"
    ws.print_title_rows = f"{head}:{head}"
    ws.page_setup.orientation = "landscape"
    ws.page_setup.fitToWidth = 1
    ws.page_setup.fitToHeight = 0
    ws.sheet_properties.pageSetUpPr.fitToPage = True

    if notes:
        wn = wb.create_sheet("Notes")
        wn.cell(row=1, column=1, value=title).font = Font(bold=True, size=14, color=NAVY)
        wn.cell(row=3, column=1, value="Column").font = Font(bold=True, color="FFFFFF")
        wn.cell(row=3, column=2, value="Meaning").font = Font(bold=True, color="FFFFFF")
        for c in (1, 2):
            wn.cell(row=3, column=c).fill = PatternFill("solid", fgColor=NAVY)
        for i, (k, v) in enumerate(notes):
            a = wn.cell(row=4 + i, column=1, value=k)
            b = wn.cell(row=4 + i, column=2, value=v)
            a.font = Font(bold=True)
            for cell in (a, b):
                cell.alignment = Alignment(wrap_text=True, vertical="top")
        wn.column_dimensions["A"].width = 28
        wn.column_dimensions["B"].width = 110
    wb.properties.creator = "KAPSAM (tools/research/article_supplementary.py)"
    wb.properties.title = title
    wb.save(path)
