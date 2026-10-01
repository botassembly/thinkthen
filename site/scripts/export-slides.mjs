#!/usr/bin/env node
// Export the talk's slides the Beatles Bench pages show, and the social card
// each page shares.
//
//   DECK=path/to/deck npm run export-slides
//
// DECK names the talk's deck folder in its own git checkout. The deck's
// build.sh renders slides/<NN-name>/slide.png and commits it, order.txt gives
// each slide's number, and common.py names the bench the deck quotes as
// BENCH_AT. This script reads those files at the deck commit in
// examples/beatles/deck-pin, never at the checkout's HEAD. An entry in
// src/data/slides.json names its slide by name, and an entry may name its own
// deck commit to keep a slide the pinned deck dropped. For each entry it writes:
//
//   - public/learn/beatles-bench/<page>.webp, 1600 pixels wide. An optional
//     height keeps only the top of the slide. An entry marked cardOnly, such
//     as a blog post's card, gets no webp.
//   - public/og/<page>.png, the social card, 1200 by 630. The whole slide is
//     scaled to 630 pixels high and padded left and right on the deck's
//     background colour, --tt-ground in deck.css. Nothing is cropped.
//
// The script then records the deck commit, the bench, and each image's
// SHA-256 in slides.json. check-slides.mjs checks that record in the build.
// The cards are committed because the Pages workflow cannot read the deck.
//
// It stops unless the pinned deck's BENCH_AT names the commit
// examples/beatles/bench-pin pins. It trusts the deck's build to have rendered
// the committed slides after BENCH_AT moved. It needs ImageMagick's convert
// with WebP.

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { execFileSync } from 'node:child_process';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const out = path.join(site, 'public', 'learn', 'beatles-bench');
const cards = path.join(site, 'public', 'og');
const file = path.join(site, 'src', 'data', 'slides.json');
const deck = process.env.DECK;
if (!deck) { console.error('export-slides: set DECK to the deck folder'); process.exit(2); }

const git = (...args) => execFileSync('git', ['-C', deck, ...args], { maxBuffer: 1 << 28 });
const commit = fs.readFileSync(path.join(site, 'examples', 'beatles', 'deck-pin'), 'utf8').trim();
const prefix = git('rev-parse', '--show-prefix').toString().trim();
const show = (rel, at = commit) => git('show', `${at}:${prefix}${rel}`);
// The folder of a slide by its name, through order.txt at that commit.
function folder(name, at) {
  const names = show('order.txt', at).toString().split('\n').map((l) => l.split('#')[0].trim()).filter(Boolean);
  const i = names.indexOf(name);
  if (i < 0) { console.error(`export-slides: deck ${at.slice(0, 8)} has no slide named ${name}`); process.exit(1); }
  return `${String(i + 1).padStart(2, '0')}-${name}`;
}

const pin = fs.readFileSync(path.join(site, 'examples', 'beatles', 'bench-pin'), 'utf8').trim();
const at = /^BENCH_AT = "([0-9a-f]+)"$/m.exec(show('common.py').toString());
if (!at) { console.error('export-slides: the deck names no BENCH_AT in common.py'); process.exit(1); }
if (!pin.startsWith(at[1])) {
  console.error(`export-slides: the deck quotes bench ${at[1]}, and the pages pin ${pin.slice(0, 8)}`);
  process.exit(1);
}
const ground = /--tt-ground:\s*(#[0-9A-Fa-f]{6})/.exec(show('deck.css').toString());
if (!ground) { console.error('export-slides: deck.css names no --tt-ground colour'); process.exit(1); }

const sha = (p) => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const manifest = JSON.parse(fs.readFileSync(file, 'utf8'));
fs.mkdirSync(cards, { recursive: true });
for (const [page, entry] of Object.entries(manifest.slides)) {
  const from = entry.deck || commit;
  const png = show(`slides/${folder(entry.slide, from)}/slide.png`, from);
  if (!entry.cardOnly) {
    const crop = entry.height ? ['-crop', `1600x${entry.height}+0+0`, '+repage'] : [];
    const target = path.join(out, `${page}.webp`);
    execFileSync('convert', ['png:-', '-resize', '1600x900', ...crop, '-quality', '85', `webp:${target}`], { input: png });
    entry.sha256 = sha(target);
  }
  const card = path.join(cards, `${page}.png`);
  execFileSync('convert', ['png:-', '-resize', 'x630', '-background', ground[1], '-gravity', 'center',
    '-extent', '1200x630', '-strip', `png:${card}`], { input: png });
  entry.card = sha(card);
}
manifest.deck = commit;
manifest.bench = pin;
fs.writeFileSync(file, JSON.stringify(manifest, null, 2) + '\n');
console.log(`export-slides: wrote ${Object.keys(manifest.slides).length} slides and cards from deck ${commit.slice(0, 8)}, bench ${pin.slice(0, 8)}`);
