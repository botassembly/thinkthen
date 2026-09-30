#!/usr/bin/env node
// A broken internal link fails the build. Every href that starts with / must
// land on a file in dist/. The one exception is the install path of a
// binding with no page yet. BINDING_PATHS_WITHOUT_PAGES in catalog.mjs lists
// them. A listed path that has a page fails too, so the list stays exact.

import fs from 'node:fs';
import path from 'node:path';
import { BINDING_PATHS_WITHOUT_PAGES } from '../src/data/catalog.mjs';

const DIST = path.join(process.cwd(), 'dist');

function walk(dir, acc = []) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) walk(full, acc);
    else acc.push(full);
  }
  return acc;
}

const files = walk(DIST);
const html = files.filter((f) => f.endsWith('.html'));
const have = new Set(files.map((f) => '/' + path.relative(DIST, f).split(path.sep).join('/')));

function lands(href) {
  const clean = href.split('#')[0].split('?')[0];
  if (!clean) return true;
  if (have.has(clean)) return true;
  if (have.has(clean.replace(/\/$/, '') + '/index.html')) return true;
  if (clean === '/' && have.has('/index.html')) return true;
  return false;
}

const withoutPage = new Set(BINDING_PATHS_WITHOUT_PAGES);
const stale = [...withoutPage].filter((p) => lands(p));
if (stale.length) {
  console.error(`binding paths listed as having no page, and a page exists: ${stale.join(', ')}`);
  process.exit(1);
}

const broken = [];
const strayCode = [];
// A draft post builds only when THINKTHEN_DRAFTS=1 asks for it. A normal
// build that holds one fails, so a draft cannot deploy by accident.
const drafts = [];
for (const file of html) {
  const body = fs.readFileSync(file, 'utf8');
  const from = '/' + path.relative(DIST, file).split(path.sep).join('/');
  if (body.includes('data-draft')) drafts.push(from);
  if (/<\/table>\s*<code(?:\s|>)/i.test(body)) strayCode.push(from);
  for (const m of body.matchAll(/(?:href|src)="(\/[^"]*)"/g)) {
    if (!lands(m[1]) && !withoutPage.has(m[1])) broken.push(`${from} -> ${m[1]}`);
  }
}

if (drafts.length && process.env.THINKTHEN_DRAFTS !== '1') {
  console.error(`draft posts in a normal build: ${drafts.join(', ')}`);
  process.exit(1);
}

if (broken.length) {
  console.error(`broken internal links: ${broken.length}`);
  for (const b of [...new Set(broken)].sort()) console.error('  ' + b);
  process.exit(1);
}

if (strayCode.length) {
  console.error(`stray code tag after a table: ${strayCode.join(', ')}`);
  process.exit(1);
}

console.log(`link check: ${html.length} pages, every internal link lands, apart from ${withoutPage.size} binding install paths with no page yet`);
