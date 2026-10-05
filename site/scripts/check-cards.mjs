#!/usr/bin/env node
// Check actual decoded head values and local social image bytes.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { builtPages, SITE } from '../src/lib/listed-pages.mjs';
import { parseHtml, findElement, textContent } from '../src/lib/html.mjs';

export function cardProblems(page, dist) {
  if (page.kind === 'stub') return [];
  const problems = [];
  const fail = code => problems.push(`${page.route}: ${code}`);
  const head = findElement(parseHtml(page.html), n => n.tag === 'head');
  const nodes = [];
  function walk(node) { if (!node) return; nodes.push(node); for (const child of node.kids || []) walk(child); }
  walk(head);
  function one(name, predicate, value) {
    const matches = nodes.filter(predicate);
    if (matches.length !== 1) fail(`${name}-count`);
    const result = matches.length === 1 ? value(matches[0]) : '';
    if (!result?.trim()) fail(`${name}-empty`);
    return result;
  }
  const title = one('title', n => n.tag === 'title', textContent);
  const meta = name => one(name, n => n.tag === 'meta' && (n.attrs.property === name || n.attrs.name === name), n => n.attrs.content || '');
  const description = meta('description');
  const tags = Object.fromEntries(['og:title', 'og:description', 'og:url', 'og:image', 'og:image:alt', 'twitter:title', 'twitter:description', 'twitter:image', 'twitter:image:alt', 'twitter:card'].map(name => [name, meta(name)]));
  for (const [name, expected] of [['og:title', title], ['twitter:title', title], ['og:description', description], ['twitter:description', description]]) {
    if (tags[name] !== expected) fail(`${name}-equality`);
  }
  const canonical = nodes.filter(n => n.tag === 'link' && n.attrs.rel === 'canonical');
  if (page.kind === 'listed' && canonical.length !== 1) fail('canonical-count');
  const expectedUrl = page.kind === 'unlisted' ? SITE + page.route : canonical[0]?.attrs.href;
  if (tags['og:url'] !== expectedUrl) fail('og:url-equality');
  if (tags['twitter:card'] !== 'summary_large_image') fail('twitter:card-value');
  if (tags['og:image'] !== tags['twitter:image']) fail('image-equality');
  for (const name of ['og:image', 'twitter:image']) {
    let image;
    try {
      const url = new URL(tags[name]);
      if (url.origin !== SITE || url.search || url.hash) throw new Error('address');
      image = path.resolve(dist, '.' + decodeURIComponent(url.pathname));
      if (!image.startsWith(path.resolve(dist) + path.sep)) throw new Error('path');
    } catch { fail(`${name}-address`); continue; }
    if (!fs.existsSync(image) || !fs.statSync(image).isFile()) { fail(`${name}-missing`); continue; }
    const bytes = fs.readFileSync(image);
    if (bytes.length < 24 || !bytes.subarray(0, 8).equals(Buffer.from([137,80,78,71,13,10,26,10])) || bytes.toString('ascii', 12, 16) !== 'IHDR' || bytes.readUInt32BE(16) !== 1200 || bytes.readUInt32BE(20) !== 630) fail(`${name}-png-size`);
  }
  return problems;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const dist = path.resolve(process.argv[2] || 'dist');
  const pages = builtPages(dist).filter(p => p.kind !== 'stub');
  const problems = pages.flatMap(page => cardProblems(page, dist));
  if (problems.length) { console.error(`check-cards: ${problems.length} problems\n  ${problems.join('\n  ')}`); process.exit(1); }
  console.log(`check-cards: ${pages.length} pages each share matching metadata and a card`);
}
