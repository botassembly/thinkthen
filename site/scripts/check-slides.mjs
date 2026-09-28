#!/usr/bin/env node
// A slide image or social card from another bench run fails the build.
// export-slides.mjs records in src/data/slides.json the bench the deck quoted
// and the SHA-256 of each page's slide and card. The recorded bench must be the
// one examples/beatles/bench-pin pins. Every image in
// public/learn/beatles-bench/ and every card in public/og/ must match its
// record, and every record needs both. Moving the pin, or replacing an image by
// hand, then fails until the slides are exported again from a deck that quotes
// the new bench.

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const dir = path.join(site, 'public', 'learn', 'beatles-bench');
const cards = path.join(site, 'public', 'og');
const manifest = JSON.parse(fs.readFileSync(path.join(site, 'src', 'data', 'slides.json'), 'utf8'));
const pin = fs.readFileSync(path.join(site, 'examples', 'beatles', 'bench-pin'), 'utf8').trim();

const problems = [];
if (manifest.bench !== pin) {
  problems.push(`the slides come from a deck that quotes bench ${String(manifest.bench).slice(0, 8)}, and the pages pin ${pin.slice(0, 8)}`);
}
// Each folder holds one image per page, recorded under `field`.
function images(folder, ext, field) {
  const found = fs.readdirSync(folder).filter((n) => n.endsWith(ext)).map((n) => n.slice(0, -ext.length));
  const where = path.relative(site, folder);
  for (const page of found) {
    const entry = manifest.slides[page];
    if (!entry) { problems.push(`${where}/${page}${ext} has no entry`); continue; }
    const sum = crypto.createHash('sha256').update(fs.readFileSync(path.join(folder, `${page}${ext}`))).digest('hex');
    if (sum !== entry[field]) problems.push(`${where}/${page}${ext} differs from the image exported from deck ${String(manifest.deck).slice(0, 8)}`);
  }
  for (const page of Object.keys(manifest.slides)) {
    if (!found.includes(page)) problems.push(`${where}/${page}${ext} is missing`);
  }
  return found.length;
}
const count = images(dir, '.webp', 'sha256');
images(cards, '.png', 'card');

if (problems.length) {
  for (const p of problems) console.error(`check-slides: ${p}`);
  console.error('check-slides: export the slides again with DECK=path/to/deck npm run export-slides');
  process.exit(1);
}
console.log(`check-slides: ${count} slides and their cards from deck ${manifest.deck.slice(0, 8)} match bench ${pin.slice(0, 8)}`);
