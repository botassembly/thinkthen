#!/usr/bin/env node
// The built Settings page must say what specification/settings.md says. For
// every row it looks for the name, what it does, the default with its note,
// the allowed values, and each surface's spelling. It fails on a surface the
// row does not reach, and on a citation a public reader cannot follow. The
// build runs it after the Astro build.

import fs from 'node:fs';
import path from 'node:path';
import { parseSettings, plain, SURFACES, ABSENT, LEFT_OUT } from '../src/lib/settings-table.mjs';
import { backends } from '../src/lib/backends-table.mjs';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const spec = fs.readFileSync(path.join(site, '..', 'specification', 'settings.md'), 'utf8');
const page = fs.readFileSync(path.join(site, 'dist', 'install', 'settings', 'index.html'), 'utf8');
const parsed = parseSettings(spec);
const { precedence } = parsed;
const rows = parsed.rows.filter((r) => !LEFT_OUT.has(r.name));

// Tags split words, and a closing tag can sit before a comma, so the
// comparison drops every space.
const squash = (s) => s.replace(/\s+/g, '');
const text = (html) => squash(html.replace(/<[^>]*>/g, ' ')
  .replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&quot;/g, '"')
  .replace(/&#39;|&#x27;/g, "'").replace(/&amp;/g, '&'));

const problems = [];

// Each migration note the parse drops from a surface cell prints here. A
// drop this list does not name fails, so a new one reaches review. Remove an
// entry when the table drops the note itself.
const EXPECTED_DROPS = [
  { setting: 'Deadline and cancel', surface: 'Rust', clause: 'until ticket 0291' },
];
for (const d of parsed.dropped) {
  const expected = EXPECTED_DROPS.some((e) => e.setting === d.setting && e.surface === d.surface && e.clause === d.clause);
  console.log(`check-settings: dropped "${d.clause}" from ${d.setting}, ${d.surface}${expected ? '' : ', which EXPECTED_DROPS does not name'}`);
  if (!expected) problems.push(`${d.setting}, ${d.surface}: the parse dropped "${d.clause}", and EXPECTED_DROPS does not name it`);
}
for (const e of EXPECTED_DROPS) {
  if (!parsed.dropped.some((d) => e.setting === d.setting && e.surface === d.surface && e.clause === d.clause)) problems.push(`EXPECTED_DROPS names "${e.clause}" in ${e.setting}, ${e.surface}, and the parse no longer drops it`);
}
for (const name of LEFT_OUT) if (!parsed.rows.some((r) => r.name === name)) problems.push(`LEFT_OUT names ${name}, and the table has no such setting`);
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
// src/lib/settings-table.mjs. This scan catches one form only: a typed number
// or number word right after "default is", "defaults to" or "default of" in
// the site's source. An expression, such as {THROTTLE.default}, passes. It
// misses "0.5 by default", a typed word default such as a model name, and a
// typed range. A range scan would flag find's own bound of "2 to 255" units,
// which shares the options range and is not a setting. Review and the lookup,
// which fails on a missing setting, hold the rest. "one" is left out of the
// words, since "the default is one document" is a framing, not a count.
const WORDS = 'zero|two|three|four|five|six|seven|eight|nine|ten|twenty|thirty|sixty|hundred';
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
// wrote it: a page, a caption, or a sample. The check is coarse. It knows the
// numbers in the table's Default values, not which setting a sentence means,
// so a right number said of the wrong setting passes.
const held = new Set(rows.flatMap((r) => plain(r.default.value).match(/\d[\d,]*(?:\.\d+)?/g) || []));
const STATED = /\bdefaults?\s+(?:is|to|of)\s+(\d[\d,]*(?:\.\d+)?)/gi;
const walkHtml = (dir) => walk(dir).filter((f) => f.endsWith('.html'));
for (const file of walkHtml(path.join(site, 'dist'))) {
  const words = fs.readFileSync(file, 'utf8').replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ');
  for (const m of words.matchAll(STATED)) {
    const n = m[1].replace(/[.,]$/, '');
    if (!held.has(n)) problems.push(`${path.relative(site, file)} says the default is ${n}, and no default in the table is ${n}`);
  }
}

// The Backends overview must show every built-in backend as
// specification/backends.md holds it: name, address, key variables, model.
const overview = fs.readFileSync(path.join(site, 'dist', 'install', 'backends', 'index.html'), 'utf8');
const shown = [...overview.matchAll(/<tr[^>]*data-backend="([^"]+)"[^>]*>([\s\S]*?)<\/tr>/g)];
const builtIns = backends();
if (shown.length !== builtIns.length) problems.push(`the Backends overview shows ${shown.length} backends and specification/backends.md holds ${builtIns.length}`);
for (const b of builtIns) {
  const row = shown.find((m) => m[1] === b.name);
  if (!row) { problems.push(`the Backends overview has no row for ${b.name}`); continue; }
  for (const part of [b.name, b.base, ...b.keys, b.model]) {
    if (!text(row[2]).includes(squash(part))) problems.push(`the Backends overview's ${b.name} row does not say ${part}`);
  }
}

if (problems.length) {
  console.error(`check-settings: a page and the specification disagree\n  ${problems.join('\n  ')}`);
  process.exit(1);
}
console.log(`check-settings: the page shows all ${rows.length} settings, as the table says, and cites no record. No source types a default, every default the site states is in the table, and the Backends overview shows all ${backends().length} built-in backends`);
