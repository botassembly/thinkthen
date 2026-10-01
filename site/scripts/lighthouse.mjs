#!/usr/bin/env node
// Lighthouse scores for twelve pages, one of each kind (ticket 0051). The
// script serves dist/ on a local port and audits each page on Lighthouse's
// mobile and desktop presets. It prints the four scores of each run.
//
// It fails when a run scores below 100 on accessibility or on SEO. While
// NOINDEX is true in src/data/catalog.mjs, every page fails the SEO audit
// "Page is blocked from indexing" by design. The 404 page always does. The
// script lets that one audit fail on those pages, and fails on any other
// failed SEO audit. Performance and best practices print and never fail
// the run, because local timings vary from run to run.
//
// Run it after `npm run build`. Pass an origin, such as
// `npm run lighthouse -- https://thinkthen.dev`, to audit a deployed site.
// It uses the Chromium that playwright-core installs, or CHROME_PATH.

import fs from 'node:fs';
import path from 'node:path';
import * as chromeLauncher from 'chrome-launcher';
import lighthouse from 'lighthouse';
import desktop from 'lighthouse/core/config/desktop-config.js';
import { chromium } from 'playwright-core';
import { NOINDEX } from '../src/data/catalog.mjs';
import { DIST, serveDist } from './serve-dist.mjs';

const PAGES = [
  '/', '/install/', '/install/python/', '/install/backends/typesafe/',
  '/reference/', '/reference/functions/filter/', '/functions/decide/',
  '/learn/', '/learn/beatles-bench/strings/', '/blog/introducing-thinkthen/',
  '/learn/tutorial/', '/404.html',
];
const CATEGORIES = ['performance', 'accessibility', 'best-practices', 'seo'];
const BLOCKED = 'is-crawlable';

const remote = process.argv[2];
let server;
if (!remote) {
  const missing = PAGES.filter((p) => !fs.existsSync(path.join(DIST, p.endsWith('/') ? p + 'index.html' : p)));
  if (missing.length) {
    console.error(`lighthouse: dist/ lacks ${missing.join(', ')}. Run npm run build, or update PAGES.`);
    process.exit(1);
  }
  server = await serveDist();
}
const origin = remote ? remote.replace(/\/$/, '') : server.origin;

const chrome = await chromeLauncher.launch({
  chromePath: process.env.CHROME_PATH || chromium.executablePath(),
  chromeFlags: ['--headless=new', '--no-sandbox'],
});

const problems = [];
const rows = [];
try {
  for (const page of PAGES) {
    const scores = [];
    for (const [form, config] of [['mobile', undefined], ['desktop', desktop]]) {
      const { lhr } = await lighthouse(origin + page, { port: chrome.port, output: 'json', logLevel: 'error', onlyCategories: CATEGORIES }, config);
      if (lhr.runtimeError) {
        problems.push(`${page} ${form}: ${lhr.runtimeError.code} ${lhr.runtimeError.message}`);
        scores.push(...CATEGORIES.map(() => '--'));
        continue;
      }
      const score = (c) => Math.round(lhr.categories[c].score * 100);
      scores.push(...CATEGORIES.map(score));
      const failed = (c) => lhr.categories[c].auditRefs
        .filter((r) => r.weight > 0 && lhr.audits[r.id].score !== null && lhr.audits[r.id].score < 1)
        .map((r) => r.id);
      const blockedOk = NOINDEX || page === '/404.html';
      const seo = failed('seo').filter((id) => !(blockedOk && id === BLOCKED));
      const a11y = failed('accessibility');
      if (a11y.length) problems.push(`${page} ${form}: accessibility ${score('accessibility')}, failed ${a11y.join(', ')}`);
      if (seo.length) problems.push(`${page} ${form}: SEO ${score('seo')}, failed ${seo.join(', ')}`);
    }
    rows.push([page, ...scores]);
  }
} finally {
  chrome.kill();
  server?.close();
}

const head = ['page', 'mobile P', 'A', 'BP', 'SEO', 'desktop P', 'A', 'BP', 'SEO'];
const width = Math.max(...PAGES.map((p) => p.length));
for (const row of [head, ...rows]) console.log(row[0].padEnd(width), ...row.slice(1).map((v, i) => String(v).padStart(i % 4 ? 4 : 9)));

if (problems.length) {
  console.error(`\nlighthouse: ${problems.length} runs below the budget\n  ${problems.join('\n  ')}`);
  process.exit(1);
}
const blocked = NOINDEX ? 'every page, while NOINDEX is true' : 'the 404 page';
console.log(`\nlighthouse: ${PAGES.length} pages on mobile and desktop score 100 on accessibility and on SEO, apart from "blocked from indexing" on ${blocked}`);
