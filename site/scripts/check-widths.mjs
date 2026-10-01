#!/usr/bin/env node
// No built page is wider than the screen (ticket 0049). The check serves
// dist/ on a local port and opens every page at 375, 768 and 1280 px in
// headless Chromium. A page fails when the document scrolls sideways. A
// table or code pane that scrolls inside itself passes. The check names the
// widest element that sticks out, so the cause is easy to find.
//
// It uses playwright-core and the Chromium build that matches it. Get the
// browser once with `npx playwright-core install chromium`.

import fs from 'node:fs';
import http from 'node:http';
import path from 'node:path';
import { chromium } from 'playwright-core';

const DIST = path.join(process.cwd(), 'dist');
const WIDTHS = [375, 768, 1280];
const TABS = 6;

const TYPES = {
  '.html': 'text/html; charset=utf-8', '.css': 'text/css', '.js': 'text/javascript',
  '.mjs': 'text/javascript', '.json': 'application/json', '.svg': 'image/svg+xml',
  '.png': 'image/png', '.jpg': 'image/jpeg', '.webp': 'image/webp', '.gif': 'image/gif',
  '.ico': 'image/x-icon', '.woff2': 'font/woff2', '.wasm': 'application/wasm',
};

function walk(dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    return e.isDirectory() ? walk(p) : p.endsWith('.html') ? [p] : [];
  });
}

const pages = walk(DIST)
  .filter((file) => !/<meta http-equiv="refresh"/.test(fs.readFileSync(file, 'utf8')))
  .map((file) => '/' + path.relative(DIST, file).split(path.sep).join('/').replace(/index\.html$/, ''))
  .sort();
if (!pages.length) {
  console.error('check-widths: dist/ holds no pages. Run npm run build first.');
  process.exit(1);
}

const server = http.createServer((req, res) => {
  let rel = decodeURIComponent(new URL(req.url, 'http://x').pathname);
  if (rel.endsWith('/')) rel += 'index.html';
  const file = path.join(DIST, rel);
  if (!file.startsWith(DIST) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) {
    res.writeHead(404).end();
    return;
  }
  res.writeHead(200, { 'content-type': TYPES[path.extname(file)] || 'application/octet-stream' });
  fs.createReadStream(file).pipe(res);
});
await new Promise((ok) => server.listen(0, '127.0.0.1', ok));
const origin = `http://127.0.0.1:${server.address().port}`;

let browser;
try {
  browser = await chromium.launch();
} catch (e) {
  server.close();
  console.error(`check-widths: no Chromium for playwright-core. Run npx playwright-core install chromium.\n${e.message.split('\n')[0]}`);
  process.exit(1);
}

// The widest element whose right edge passes the screen, outside any box
// that scrolls or clips it.
function measure() {
  const screen = document.documentElement.clientWidth;
  const wide = document.documentElement.scrollWidth;
  if (wide <= screen) return null;
  const clipped = (el) => {
    for (let p = el.parentElement; p && p !== document.body; p = p.parentElement) {
      const o = getComputedStyle(p).overflowX;
      if (o !== 'visible') return true;
    }
    return false;
  };
  let worst = null;
  for (const el of document.body.querySelectorAll('*')) {
    const r = el.getBoundingClientRect();
    if (r.right <= screen + 0.5 || clipped(el)) continue;
    if (!worst || r.right > worst.right) {
      const name = el.tagName.toLowerCase() + (el.className && typeof el.className === 'string' ? '.' + el.className.trim().split(/\s+/).join('.') : '');
      worst = { right: Math.round(r.right), name, text: (el.textContent || '').trim().slice(0, 60) };
    }
  }
  return { wide, worst };
}

const problems = [];
const jobs = WIDTHS.flatMap((width) => pages.map((page) => ({ width, page })));
async function tab(context) {
  const view = await context.newPage();
  for (let job = jobs.shift(); job; job = jobs.shift()) {
    await view.setViewportSize({ width: job.width, height: 900 });
    await view.goto(origin + job.page, { waitUntil: 'load' });
    const found = await view.evaluate(measure);
    if (found) {
      const w = found.worst;
      problems.push(`${job.page} at ${job.width} px is ${found.wide} px wide` +
        (w ? `: ${w.name} reaches ${w.right} px ("${w.text}")` : ''));
    }
  }
  await view.close();
}

try {
  const context = await browser.newContext();
  await Promise.all(Array.from({ length: TABS }, () => tab(context)));
} finally {
  await browser.close();
  server.close();
}

if (problems.length) {
  console.error(`check-widths: ${problems.length} page views are wider than the screen:`);
  const shown = problems.sort().slice(0, 40);
  for (const p of shown) console.error('  ' + p);
  if (problems.length > shown.length) console.error(`  and ${problems.length - shown.length} more`);
  process.exit(1);
}
console.log(`check-widths: ${pages.length} pages fit the screen at ${WIDTHS.join(', ')} px`);
