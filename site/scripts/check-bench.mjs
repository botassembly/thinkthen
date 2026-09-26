#!/usr/bin/env node
// The Beatles Bench pages come from one pinned bench commit through
// scripts/pull-bench.mjs. A hand edit to a pulled file fails the build: every
// file must match the SHA-256 the pull wrote, and the pull must match the pin.

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';

const SITE = process.cwd();
const DATA = path.join(SITE, 'src', 'data', 'bench');
const pin = fs.readFileSync(path.join(DATA, 'PIN'), 'utf8').trim();
const manifest = JSON.parse(fs.readFileSync(path.join(DATA, 'manifest.json'), 'utf8'));
const problems = [];

if (manifest.pin !== pin) problems.push(`the manifest names ${manifest.pin}, and PIN names ${pin}`);
for (const [file, want] of Object.entries(manifest.files)) {
  const full = path.join(SITE, file);
  if (!fs.existsSync(full)) { problems.push(`${file} is missing`); continue; }
  const got = crypto.createHash('sha256').update(fs.readFileSync(full)).digest('hex');
  if (got !== want) problems.push(`${file} differs from the pull. Edit it in the bench and pull again`);
}
for (const dir of ['src/pages/beatles-bench', 'public/beatles-bench/img']) {
  for (const f of fs.readdirSync(path.join(SITE, dir))) {
    const rel = `${dir}/${f}`;
    if (!f.endsWith('.astro') && !(rel in manifest.files)) problems.push(`${rel} did not come from the pull`);
  }
}

if (problems.length) {
  for (const p of problems) console.error(`check-bench: ${p}`);
  process.exit(1);
}
console.log(`bench check: ${Object.keys(manifest.files).length} pulled files match the bench at ${pin.slice(0, 8)}`);
