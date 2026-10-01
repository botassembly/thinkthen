#!/usr/bin/env node
// No built page is wider than the screen (ticket 0049). The check serves
// dist/ on a local port and opens every page in headless Chromium at every
// width from 320 to 1600 px in steps of 40, and at 375, 768, 820 and 1024 px.
// A page fails when the document scrolls sideways. It also fails when a
// table or a code pane is wider than the column that holds it, or scrolls
// inside itself. The check names the element that sticks out, so the cause
// is easy to find.
//
// It uses playwright-core and the Chromium build that matches it. Get the
// browser once with `npx playwright-core install chromium`.

import fs from 'node:fs';
import path from 'node:path';
import { chromium } from 'playwright-core';
import { DIST, serveDist } from './serve-dist.mjs';

const STEPS = Array.from({ length: (1600 - 320) / 40 + 1 }, (_, i) => 320 + i * 40);
const WIDTHS = [...new Set([...STEPS, 375, 768, 820, 1024])].sort((a, b) => a - b);
const TABS = 6;

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

const server = await serveDist();
const { origin } = server;

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
  const name = (el) => el.tagName.toLowerCase() + (el.className && typeof el.className === 'string' ? '.' + el.className.trim().split(/\s+/).join('.') : '');
  const panes = [];
  for (const el of document.querySelectorAll('main table, main pre')) {
    const box = el.parentElement;
    const cs = getComputedStyle(box);
    const edge = box.getBoundingClientRect().right - parseFloat(cs.paddingRight) - parseFloat(cs.borderRightWidth);
    const right = el.getBoundingClientRect().right;
    if (right > edge + 1 || el.scrollWidth > el.clientWidth + 1) {
      panes.push(`${name(el)} ("${(el.textContent || '').trim().slice(0, 40)}") is ${Math.round(Math.max(right - edge, el.scrollWidth - el.clientWidth))} px wider than its column`);
    }
  }
  const screen = document.documentElement.clientWidth;
  const wide = document.documentElement.scrollWidth;
  if (wide <= screen) return panes.length ? { panes } : null;
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
      worst = { right: Math.round(r.right), name: name(el), text: (el.textContent || '').trim().slice(0, 60) };
    }
  }
  return { wide, worst, panes };
}

const problems = [];
// Each tab loads a page once and then resizes it through every width.
const jobs = [...pages];
async function tab(context) {
  const view = await context.newPage();
  for (let page = jobs.shift(); page; page = jobs.shift()) {
    await view.setViewportSize({ width: WIDTHS[0], height: 900 });
    await view.goto(origin + page, { waitUntil: 'load' });
    for (const width of WIDTHS) {
      await view.setViewportSize({ width, height: 900 });
      await view.evaluate(() => new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done))));
      const found = await view.evaluate(measure);
      if (!found) continue;
      const w = found.worst;
      if (found.wide) {
        problems.push(`${page} at ${width} px is ${found.wide} px wide` +
          (w ? `: ${w.name} reaches ${w.right} px ("${w.text}")` : ''));
      }
      for (const pane of found.panes) problems.push(`${page} at ${width} px: ${pane}`);
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
  console.error(`check-widths: ${problems.length} page views are wider than the screen or a column:`);
  const shown = problems.sort().slice(0, 40);
  for (const p of shown) console.error('  ' + p);
  if (problems.length > shown.length) console.error(`  and ${problems.length - shown.length} more`);
  process.exit(1);
}
console.log(`check-widths: ${pages.length} pages fit the screen at ${WIDTHS.length} widths from ${WIDTHS[0]} to ${WIDTHS.at(-1)} px, and every table and code pane fits its column`);
