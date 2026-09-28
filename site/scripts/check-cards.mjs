#!/usr/bin/env node
// Every built page shares a social card (Ian, 2026-09-28). Each page in dist/
// must carry og:image, og:title, og:description, twitter:card, and
// twitter:image. The card is summary_large_image. Both images name one
// absolute address on the site, and the file there is a PNG of 1200 by 630.
// A page that only redirects carries no card.

import fs from 'node:fs';
import path from 'node:path';

const DIST = path.join(process.cwd(), 'dist');
const SITE = 'https://thinkthen.dev';

function walk(dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    return e.isDirectory() ? walk(p) : p.endsWith('.html') ? [p] : [];
  });
}

const meta = (body, attr, name) => {
  const tag = body.match(new RegExp(`<meta ${attr}="${name}" content="([^"]*)"`));
  return tag ? tag[1] : null;
};

// A PNG names its width and height in bytes 16 to 24.
function size(file) {
  const head = fs.readFileSync(file).subarray(0, 24);
  if (head.toString('latin1', 1, 4) !== 'PNG') return null;
  return [head.readUInt32BE(16), head.readUInt32BE(20)];
}

const problems = [];
let pages = 0;
for (const file of walk(DIST)) {
  const body = fs.readFileSync(file, 'utf8');
  if (/<meta http-equiv="refresh"/.test(body)) continue;
  pages += 1;
  const from = '/' + path.relative(DIST, file).split(path.sep).join('/');
  const tags = {
    'og:image': meta(body, 'property', 'og:image'),
    'og:title': meta(body, 'property', 'og:title'),
    'og:description': meta(body, 'property', 'og:description'),
    'twitter:card': meta(body, 'name', 'twitter:card'),
    'twitter:image': meta(body, 'name', 'twitter:image'),
  };
  for (const [name, value] of Object.entries(tags)) {
    if (!value) problems.push(`${from}: no ${name}`);
  }
  if (tags['twitter:card'] && tags['twitter:card'] !== 'summary_large_image') problems.push(`${from}: twitter:card is ${tags['twitter:card']}`);
  for (const name of ['og:image', 'twitter:image']) {
    const url = tags[name];
    if (!url) continue;
    if (!url.startsWith(`${SITE}/`)) { problems.push(`${from}: ${name} ${url} is not an absolute address on ${SITE}`); continue; }
    const image = path.join(DIST, decodeURIComponent(url.slice(SITE.length)));
    if (!fs.existsSync(image)) { problems.push(`${from}: ${name} ${url} names no file`); continue; }
    const wide = size(image);
    if (!wide || wide[0] !== 1200 || wide[1] !== 630) problems.push(`${from}: ${name} ${url} is not a PNG of 1200 by 630`);
  }
  if (tags['og:image'] !== tags['twitter:image']) problems.push(`${from}: og:image and twitter:image differ`);
}

if (problems.length) {
  console.error(`check-cards: ${problems.length} problems\n  ${problems.join('\n  ')}`);
  process.exit(1);
}
console.log(`check-cards: ${pages} pages each share a card`);
