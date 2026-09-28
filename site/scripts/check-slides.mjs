#!/usr/bin/env node
// A slide image from another bench run fails the build. export-slides.mjs
// records in src/data/slides.json the bench the deck quoted and each image's
// SHA-256. The recorded bench must be the one examples/beatles/bench-pin pins, and
// every image in public/learn/beatles-bench/ must match its record. Moving the
// pin, or replacing an image by hand, then fails until the slides are
// exported again from a deck that quotes the new bench.

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const dir = path.join(site, 'public', 'learn', 'beatles-bench');
const manifest = JSON.parse(fs.readFileSync(path.join(site, 'src', 'data', 'slides.json'), 'utf8'));
const pin = fs.readFileSync(path.join(site, 'examples', 'beatles', 'bench-pin'), 'utf8').trim();

const problems = [];
if (manifest.bench !== pin) {
  problems.push(`the slides come from a deck that quotes bench ${String(manifest.bench).slice(0, 8)}, and the pages pin ${pin.slice(0, 8)}`);
}
const images = fs.readdirSync(dir).filter((n) => n.endsWith('.webp')).map((n) => n.slice(0, -5));
for (const page of images) {
  const entry = manifest.slides[page];
  if (!entry) { problems.push(`${page}.webp has no entry`); continue; }
  const sum = crypto.createHash('sha256').update(fs.readFileSync(path.join(dir, `${page}.webp`))).digest('hex');
  if (sum !== entry.sha256) problems.push(`${page}.webp differs from the image exported from deck ${String(manifest.deck).slice(0, 8)}`);
}
for (const page of Object.keys(manifest.slides)) {
  if (!images.includes(page)) problems.push(`${page}.webp is missing`);
}

if (problems.length) {
  for (const p of problems) console.error(`check-slides: ${p}`);
  console.error('check-slides: export the slides again with DECK=path/to/deck npm run export-slides');
  process.exit(1);
}
console.log(`check-slides: ${images.length} slides from deck ${manifest.deck.slice(0, 8)} match bench ${pin.slice(0, 8)}`);
