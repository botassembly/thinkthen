#!/usr/bin/env node
// On a phone a table stacks each row into a card (ticket 0049). A card from a
// table of three or more columns names each cell past the first with its
// column heading. This step copies the heading into a data-label attribute on
// every such cell of every built page, and site.css prints it. It runs after
// astro build. A table whose first row is not all headings, or that has two
// columns, gets no labels, and neither does a cell that spans columns.
//
// The same pass marks each short code span outside a code pane with class
// "short". A span of SHORT characters or fewer, such as a model name, a flag,
// a key name or a version, then never splits across lines (site.css).

import fs from 'node:fs';
import path from 'node:path';

const DIST = path.join(process.cwd(), 'dist');
const SHORT = 24;

function walk(dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    return e.isDirectory() ? walk(p) : p.endsWith('.html') ? [p] : [];
  });
}

const text = (html) => html.replace(/<[^>]+>/g, '').replace(/&amp;/g, '&').replace(/\s+/g, ' ').trim();
const chars = (html) => html.replace(/<[^>]+>/g, '').replace(/&(#\d+|#x[\da-f]+|\w+);/gi, '_').length;
const attr = (s) => s.replace(/&/g, '&amp;').replace(/"/g, '&quot;');

function label(table) {
  const rows = table.match(/<tr\b[\s\S]*?<\/tr>/g) || [];
  if (!rows.length || /<td\b/.test(rows[0])) return table;
  const heads = [...rows[0].matchAll(/<th\b[^>]*>([\s\S]*?)<\/th>/g)].map((m) => text(m[1]));
  if (heads.length < 3) return table;
  let out = table;
  for (const row of rows.slice(1)) {
    let col = 0;
    const labelled = row.replace(/<td\b([^>]*)>/g, (tag, rest) => {
      const at = col;
      const span = Number((/colspan="(\d+)"/.exec(rest) || [0, 1])[1]);
      col += span;
      if (at === 0 || span > 1 || !heads[at] || /data-label=/.test(rest)) return tag;
      return `<td${rest} data-label="${attr(heads[at])}">`;
    });
    out = out.replace(row, () => labelled);
  }
  return out;
}

let shorts = 0;
function markShort(html) {
  return html.split(/(<pre\b[\s\S]*?<\/pre>)/).map((part, i) => i % 2 ? part : part.replace(
    /<code\b([^>]*)>([\s\S]*?)<\/code>/g,
    (whole, rest, inner) => {
      if (chars(inner) > SHORT || /\bshort\b/.test(rest)) return whole;
      shorts += 1;
      const cls = /\bclass="([^"]*)"/.exec(rest);
      return cls
        ? `<code${rest.replace(cls[0], `class="${cls[1]} short"`)}>${inner}</code>`
        : `<code${rest} class="short">${inner}</code>`;
    },
  )).join('');
}

let pages = 0;
let tables = 0;
for (const file of walk(DIST)) {
  const body = fs.readFileSync(file, 'utf8');
  let next = markShort(body);
  next = next.replace(/<table\b[\s\S]*?<\/table>/g, (t) => {
    const done = label(t);
    if (done !== t) tables += 1;
    return done;
  });
  if (next !== body) {
    fs.writeFileSync(file, next);
    pages += 1;
  }
}
console.log(`label-tables: labelled ${tables} tables and marked ${shorts} short code spans on ${pages} pages`);
