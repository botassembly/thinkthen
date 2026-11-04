// Makes public/social-card.png, the 1200x630 card that link previews show for every page,
// from the brand mark, colors and fonts in product.config.mjs (socialCard).
// It needs browser-box's sandboxed Chromium container: run it after a brand change
// and commit the PNG.
//   node scripts/browser-box-job.mjs scripts/social-card.mjs
import fs from 'node:fs';
import path from 'node:path';
import { launchBrowser } from './browser.mjs';
import { pathToFileURL } from 'node:url';
import product from '../product.config.mjs';

const c = product.socialCard;
const url = (f) => pathToFileURL(path.resolve(f)).href;
const fonts = (c.fonts ?? []).map((f) => `@font-face { font-family: '${f.family}'; src: url('${url(f.file)}'); font-weight: ${f.weight ?? 400}; }`).join('\n');
const host = new URL(product.site).host;
const html = `<!doctype html><html><head><meta charset="utf-8"><style>
${fonts}
* { margin: 0; box-sizing: border-box; }
body { width: 1200px; height: 630px; background: ${c.background}; color: ${c.text}; font-family: ${c.bodyFont ?? 'sans-serif'};
  display: flex; flex-direction: column; justify-content: space-between; padding: 72px 88px; }
.top { display: flex; align-items: center; gap: 36px; }
.top img { height: ${c.markHeight ?? 140}px; }
.name { font-family: ${c.nameFont ?? 'inherit'}; font-weight: 700; font-size: 112px; line-height: 1; letter-spacing: -1px; }
.tag { font-size: 46px; line-height: 1.25; max-width: 1000px; font-weight: 400; }
.foot { display: flex; justify-content: space-between; align-items: center; font-family: ${c.monoFont ?? 'monospace'}; font-size: 32px; color: ${c.muted}; }
.foot b { color: ${c.accent}; font-weight: inherit; }
.rule { height: 8px; width: 160px; background: ${c.accent}; border-radius: 4px; }
</style></head><body>
<div class="top"><img src="${url(c.mark)}" alt="">${c.showName === false ? '' : `<span class="name">${product.name}</span>`}</div>
<div><div class="rule"></div><p class="tag" style="margin-top: 28px">${c.tagline}</p></div>
<div class="foot"><span><b>$</b> ${c.command}</span><span>${host}</span></div>
</body></html>`;
const page = path.resolve('.astro/social-card.html');
fs.mkdirSync(path.dirname(page), { recursive: true });
fs.writeFileSync(page, html);
const browser = await launchBrowser();
const tab = await browser.newPage();
await tab.setViewport({ width: 1200, height: 630 });
await tab.goto(pathToFileURL(page).href, { waitUntil: 'networkidle0' });
await tab.screenshot({ path: path.resolve('public/social-card.png') });
await browser.close();
console.log('wrote public/social-card.png');
