#!/usr/bin/env node
// Export the talk's slides the Beatles Bench pages show.
//
//   DECK=path/to/deck npm run export-slides
//
// DECK names the talk's deck folder in its own git checkout. The deck's
// build.sh renders slides/<NN-name>/slide.png and commits it, and common.py
// names the bench commit the deck quotes as BENCH_AT. This script reads each
// slide.png from the deck's HEAD commit, scales it to 1600 pixels wide, and
// writes public/learn/beatles-bench/<page>.webp. src/data/slides.json names
// the slide for each page and an optional height to keep from the top. The
// script then records the deck commit, the bench, and each image's SHA-256
// there. check-slides.mjs checks that record in the build.
//
// It stops unless the deck folder is clean and its BENCH_AT names the commit
// examples/beatles/bench-pin pins. It trusts the deck's build to have rendered
// the committed slides after BENCH_AT moved. It needs ImageMagick's convert
// with WebP.

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { execFileSync } from 'node:child_process';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const out = path.join(site, 'public', 'learn', 'beatles-bench');
const file = path.join(site, 'src', 'data', 'slides.json');
const deck = process.env.DECK;
if (!deck) { console.error('export-slides: set DECK to the deck folder'); process.exit(2); }

const git = (...args) => execFileSync('git', ['-C', deck, ...args], { maxBuffer: 1 << 28 });
if (git('status', '--porcelain', '--', '.').length) { console.error('export-slides: the deck folder has local changes'); process.exit(1); }
const commit = git('rev-parse', 'HEAD').toString().trim();
const prefix = git('rev-parse', '--show-prefix').toString().trim();
const show = (rel) => git('show', `${commit}:${prefix}${rel}`);

const pin = fs.readFileSync(path.join(site, 'examples', 'beatles', 'bench-pin'), 'utf8').trim();
const at = /^BENCH_AT = "([0-9a-f]+)"$/m.exec(show('common.py').toString());
if (!at) { console.error('export-slides: the deck names no BENCH_AT in common.py'); process.exit(1); }
if (!pin.startsWith(at[1])) {
  console.error(`export-slides: the deck quotes bench ${at[1]}, and the pages pin ${pin.slice(0, 8)}`);
  process.exit(1);
}

const manifest = JSON.parse(fs.readFileSync(file, 'utf8'));
for (const [page, entry] of Object.entries(manifest.slides)) {
  const crop = entry.height ? ['-crop', `1600x${entry.height}+0+0`, '+repage'] : [];
  const target = path.join(out, `${page}.webp`);
  execFileSync('convert', ['png:-', '-resize', '1600x900', ...crop, '-quality', '85', `webp:${target}`],
    { input: show(`slides/${entry.slide}/slide.png`) });
  entry.sha256 = crypto.createHash('sha256').update(fs.readFileSync(target)).digest('hex');
}
manifest.deck = commit;
manifest.bench = pin;
fs.writeFileSync(file, JSON.stringify(manifest, null, 2) + '\n');
console.log(`export-slides: wrote ${Object.keys(manifest.slides).length} slides from deck ${commit.slice(0, 8)}, bench ${pin.slice(0, 8)}`);
