// Prints an HTML file to PDF with Chromium (Playwright): node print_pdf.js in.html out.pdf "header text" [landscape]
const { chromium } = require('playwright');
(async () => {
  const [, , input, output, header, orientation] = process.argv;
  const browser = await chromium.launch();
  const page = await browser.newPage();
  await page.goto('file://' + require('path').resolve(input), { waitUntil: 'load' });
  const esc = (s) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;');
  await page.pdf({
    path: output,
    format: 'A4',
    landscape: orientation === 'landscape',
    printBackground: true,
    margin: { top: '18mm', bottom: '16mm', left: '14mm', right: '14mm' },
    displayHeaderFooter: true,
    headerTemplate: `<div style="font-size:8px;width:100%;padding:0 14mm;color:#555;font-family:DejaVu Sans">${esc(header)}</div>`,
    footerTemplate: '<div style="font-size:8px;width:100%;text-align:center;color:#555;font-family:DejaVu Sans"><span class="pageNumber"></span> / <span class="totalPages"></span></div>',
  });
  await browser.close();
})();
