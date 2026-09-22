#!/usr/bin/env node
// A broken internal link fails the build. Every href that starts with / must
// land on a file in dist/.

import fs from 'node:fs';
import path from 'node:path';

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

const broken = [];
for (const file of html) {
  const body = fs.readFileSync(file, 'utf8');
  const from = '/' + path.relative(DIST, file).split(path.sep).join('/');
  for (const m of body.matchAll(/(?:href|src)="(\/[^"]*)"/g)) {
    if (!lands(m[1])) broken.push(`${from} -> ${m[1]}`);
  }
}

if (broken.length) {
  console.error(`broken internal links: ${broken.length}`);
  for (const b of [...new Set(broken)].sort()) console.error('  ' + b);
  process.exit(1);
}

console.log(`link check: ${html.length} pages, every internal link lands`);
