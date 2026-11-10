// Renders app/assets/logo/logo.svg to the PNG and ICO files of the application.
//   node tools/logo/render_icons.js        (from the repository root; needs playwright and ImageMagick)
// Writes app/assets/logo/logo.png (512 px, used in the interface) and
// app/windows/runner/resources/app_icon.ico (16-256 px).
const { chromium } = require('playwright');
const { execFileSync } = require('child_process');
const fs = require('fs');
const os = require('os');
const path = require('path');

(async () => {
  const svg = fs.readFileSync('app/assets/logo/logo.svg', 'utf8');
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'logo-'));
  const browser = await chromium.launch();
  const render = async (size, file) => {
    const page = await browser.newPage({ viewport: { width: size, height: size } });
    await page.setContent(`<html><body style="margin:0;background:transparent">${svg.replace('<svg ', `<svg width="${size}" height="${size}" `)}</body></html>`);
    await page.screenshot({ path: file, omitBackground: true, clip: { x: 0, y: 0, width: size, height: size } });
    await page.close();
  };
  await render(512, 'app/assets/logo/logo.png');
  const sizes = [16, 20, 24, 32, 40, 48, 64, 128, 256];
  const files = [];
  for (const s of sizes) {
    const f = path.join(tmp, `icon-${s}.png`);
    await render(s, f);
    files.push(f);
  }
  await browser.close();
  execFileSync('convert', [...files, 'app/windows/runner/resources/app_icon.ico']);
  console.log('wrote app/assets/logo/logo.png and app/windows/runner/resources/app_icon.ico');
})();
