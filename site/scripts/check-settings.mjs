#!/usr/bin/env node
// The built Settings page must say what specification/settings.md says. For
// every row it looks for the name, what it does, the default with its note,
// the allowed values, and each surface's spelling. It fails on a surface the
// row does not reach, and on a citation a public reader cannot follow. The
// build runs it after the Astro build.

import fs from 'node:fs';
import path from 'node:path';
import { parseSettings, plain, SURFACES, ABSENT } from '../src/lib/settings-table.mjs';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const spec = fs.readFileSync(path.join(site, '..', 'specification', 'settings.md'), 'utf8');
const page = fs.readFileSync(path.join(site, 'dist', 'install', 'settings', 'index.html'), 'utf8');
const { precedence, rows } = parseSettings(spec);

// Tags split words, and a closing tag can sit before a comma, so the
// comparison drops every space.
const squash = (s) => s.replace(/\s+/g, '');
const text = (html) => squash(html.replace(/<[^>]*>/g, ' ')
  .replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&quot;/g, '"')
  .replace(/&#39;|&#x27;/g, "'").replace(/&amp;/g, '&'));

const problems = [];
const expect = (where, body, want) => {
  if (want && !body.includes(squash(plain(want)))) problems.push(`${where}: the page does not say "${plain(want).slice(0, 80)}"`);
};

const cards = [...page.matchAll(/<section class="setting"([^>]*)>([\s\S]*?)<\/section>/g)];
if (cards.length !== rows.length) problems.push(`the table has ${rows.length} settings and the page shows ${cards.length}`);

for (const row of rows) {
  const card = cards.find((c) => c[1].includes(` id="${row.id}"`));
  if (!card) { problems.push(`${row.name}: no setting on the page at #${row.id}`); continue; }
  const body = text(card[2]);
  for (const part of [row.name, row.does, row.default.value, row.default.note, row.allowed]) expect(row.name, body, part);
  for (const s of SURFACES) {
    const shown = card[2].includes(`data-surface="${s}"`);
    if (row.on[s] === ABSENT) {
      if (shown) problems.push(`${row.name}: the page shows ${s}, and the table says ${ABSENT}`);
    } else if (!shown) {
      problems.push(`${row.name}: the page leaves out ${s}`);
    } else {
      expect(`${row.name}, ${s}`, body, `${s} ${row.on[s]}`);
    }
  }
}

const whole = text(page);
for (const b of precedence) for (const part of b.kind === 'p' ? [b.text] : b.items) expect('precedence', whole, part);

// The repository's records stay off the page.
const main = page.slice(page.indexOf('<main'), page.indexOf('</main>'));
const cited = text(main).match(/ADR\d+|[Tt]ickets?\d+|sdlc\/|Ian'sruling|Batchingdesign|Noreasonrecorded|Notbuiltyet|ontheway/);
if (cited) problems.push(`the page cites a record a public reader cannot follow: "${cited[0]}"`);

// No page types a default. A page reads a default through setting() in
// src/lib/settings-table.mjs, so the typed number or number word after
// "default is", "defaults to" or "default of" in the site's source is a value
// the table no longer governs. An expression, such as {THROTTLE.default},
// passes. A range such as "1 to 32" is not scanned: find's own bound of
// "2 to 255" units shares the options range and is not a setting, so the scan
// would flag it. The lookup guards ranges by failing on a missing setting.
const WORDS = 'zero|one|two|three|four|five|six|seven|eight|nine|ten|twenty|thirty|sixty|hundred';
const TYPED = new RegExp(`\\bdefaults?\\s+(?:is|to|of)\\s+(?:<code>|\`)?(\\d[\\d,.]*\\d|\\d|${WORDS})\\b`, 'gi');
const src = path.join(site, 'src');
const walk = (dir) => fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
  const p = path.join(dir, e.name);
  return e.isDirectory() ? walk(p) : [p];
});
for (const file of walk(src).filter((f) => /\.(astro|mjs|md)$/.test(f) && !f.endsWith('settings-table.mjs'))) {
  const lines = fs.readFileSync(file, 'utf8').split('\n');
  lines.forEach((line, i) => {
    for (const m of line.matchAll(TYPED)) {
      problems.push(`${path.relative(site, file)}:${i + 1} types the default "${m[1]}". Read it with setting() from src/lib/settings-table.mjs`);
    }
  });
}

// A default the built site states must be one the table holds, whatever
// wrote it: a page, a caption, or a sample.
const held = new Set(rows.flatMap((r) => plain(`${r.default.value} ${r.default.note}`).match(/\d[\d,]*(?:\.\d+)?/g) || []));
const STATED = /\bdefaults?\s+(?:is|to|of)\s+(\d[\d,]*(?:\.\d+)?)/gi;
const walkHtml = (dir) => walk(dir).filter((f) => f.endsWith('.html'));
for (const file of walkHtml(path.join(site, 'dist'))) {
  const words = fs.readFileSync(file, 'utf8').replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ');
  for (const m of words.matchAll(STATED)) {
    const n = m[1].replace(/[.,]$/, '');
    if (!held.has(n)) problems.push(`${path.relative(site, file)} says the default is ${n}, and no default in the table is ${n}`);
  }
}

if (problems.length) {
  console.error(`check-settings: the Settings page and specification/settings.md disagree\n  ${problems.join('\n  ')}`);
  process.exit(1);
}
console.log(`check-settings: the page shows all ${rows.length} settings, as the table says, and cites no record. No source types a default, and every default the site states is in the table`);
