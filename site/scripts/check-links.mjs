#!/usr/bin/env node
// Internal links resolve against the source route and must land on canonical
// files and anchors in dist/. The one exception is the install path of a
// binding with no page yet. BINDING_PATHS_WITHOUT_PAGES in catalog.mjs lists
// them. A listed path that has a page fails too, so the list stays exact.
// A link with a #fragment must also find that id or name on the page it
// lands on, and a bare #fragment must find it on its own page.

import fs from 'node:fs';
import path from 'node:path';
import { SITE, isStub } from '../src/lib/listed-pages.mjs';
import { ALIASES, routeFile, validateCompatibility } from './redirect-contract.mjs';
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

// The built file a link lands on, or null.
function target(href) {
  const clean = href.split('#')[0].split('?')[0];
  if (have.has(clean)) return clean;
  const index = clean.replace(/\/$/, '') + '/index.html';
  if (have.has(index)) return index;
  return null;
}
const lands = (href) => !href.split('#')[0].split('?')[0] || target(href) !== null;

const anchors = new Map();
function idsOf(file) {
  if (!anchors.has(file)) {
    const body = fs.readFileSync(path.join(DIST, file), 'utf8');
    anchors.set(file, new Set([...body.matchAll(/\s(?:id|name)="([^"]+)"/g)].map((m) => m[1])));
  }
  return anchors.get(file);
}
function fragmentOf(href) {
  const at = href.indexOf('#');
  if (at < 0 || at === href.length - 1) return null;
  try { return decodeURIComponent(href.slice(at + 1)); } catch { return href.slice(at + 1); }
}

const withoutPage = new Set(BINDING_PATHS_WITHOUT_PAGES);
const stale = [...withoutPage].filter((p) => lands(p));
if (stale.length) {
  console.error(`binding paths listed as having no page, and a page exists: ${stale.join(', ')}`);
  process.exit(1);
}

const compatibility = [];
for (const [alias, fixed] of Object.entries(ALIASES)) {
  try { validateCompatibility(fs.readFileSync(routeFile(DIST, alias), 'utf8'), alias, fixed, DIST); }
  catch (e) { compatibility.push(e.message.includes('ENOENT') ? `alias compatibility: ${alias}: missing redirect stub` : e.message); }
}
const redirects = [];
const broken = [];
const brokenAnchors = [];
const strayCode = [];
// A draft post builds only when THINKTHEN_DRAFTS=1 asks for it. A normal
// build that holds one fails, so a draft cannot deploy by accident.
const drafts = [];
for (const file of html) {
  const body = fs.readFileSync(file, 'utf8');
  const from = '/' + path.relative(DIST, file).split(path.sep).join('/');
  if (body.includes('data-draft')) drafts.push(from);
  if (/<\/table>\s*<code(?:\s|>)/i.test(body)) strayCode.push(from);
  if (isStub(body)) {
    if (!Object.hasOwn(ALIASES, from.replace(/\/index\.html$/, ''))) compatibility.push(`undeclared redirect stub: ${from}`);
    continue;
  }
  const sourceRoute = from.endsWith('/index.html') ? from.slice(0, -'index.html'.length) : from;
  for (const m of body.matchAll(/(?:href|src)="([^"]*)"/g)) {
    let url;
    try { url = new URL(m[1].replace(/&amp;/g, '&'), SITE + sourceRoute); } catch { continue; }
    if (url.origin !== SITE) continue;
    const href = url.pathname + url.search + url.hash;
    if (withoutPage.has(href)) continue;
    if (!lands(href)) { broken.push(`${from} -> ${m[1]}`); continue; }
    const file = target(href);
    if (isStub(fs.readFileSync(path.join(DIST, file), 'utf8'))) {
      redirects.push(`${from} -> ${m[1]}`); continue;
    }
    const frag = fragmentOf(href);
    if (frag && !idsOf(file).has(frag)) brokenAnchors.push(`${from} -> ${m[1]}`);
  }
}

if (drafts.length && process.env.THINKTHEN_DRAFTS !== '1') {
  console.error(`draft posts in a normal build: ${drafts.join(', ')}`);
  process.exit(1);
}

if (compatibility.length || redirects.length) {
  for (const message of compatibility) console.error(message);
  for (const message of redirects) console.error(`redirect target: ${message}`);
  process.exit(1);
}

if (broken.length) {
  console.error(`broken internal links: ${broken.length}`);
  for (const b of [...new Set(broken)].sort()) console.error('  ' + b);
  process.exit(1);
}

if (brokenAnchors.length) {
  console.error(`links to a missing anchor: ${brokenAnchors.length}`);
  for (const b of [...new Set(brokenAnchors)].sort()) console.error('  ' + b);
  process.exit(1);
}

if (strayCode.length) {
  console.error(`stray code tag after a table: ${strayCode.join(', ')}`);
  process.exit(1);
}

console.log(`link check: ${html.length} pages, every internal link and anchor lands, apart from ${withoutPage.size} binding install paths with no page yet`);
