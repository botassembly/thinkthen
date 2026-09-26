// Read specification/settings.md into the Settings page's data. The page and
// scripts/check-settings.mjs both call parseSettings, so they read the file
// the same way. Nothing on the page is typed from the table by hand.
//
// The table's 17 columns are fixed. sdlc/scripts/settings fails the product's
// spec rung when one moves, and parseSettings fails the site build.

import fs from 'node:fs';
import path from 'node:path';
import { REPO } from '../data/repo.mjs';

export const COLUMNS = [
  'Setting', 'What it does', 'Default', 'Allowed values',
  'Command flag', 'Environment variable', 'Configuration file', 'Question-file key',
  'Rust', 'Python', 'TypeScript', 'Ruby', 'R', 'C',
  'DuckDB', 'PostgreSQL', 'SQLite',
];

// The surface columns, in three groups for the page.
export const SURFACE_GROUPS = [
  ['Command line', ['Command flag', 'Environment variable', 'Configuration file', 'Question-file key']],
  ['Languages', ['Rust', 'Python', 'TypeScript', 'Ruby', 'R', 'C']],
  ['Databases', ['DuckDB', 'PostgreSQL', 'SQLite']],
];
export const SURFACES = SURFACE_GROUPS.flatMap(([, list]) => list);

export const ABSENT = 'not on this surface';

const REPO_BLOB = `${REPO}/blob/main/`;

// The sections the page carries. A missing one fails the build.
const NEEDED = ['What counts as a setting', 'Precedence', 'How to read a cell', 'Settings', 'Settings on the way'];

export function slug(name) {
  return name.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '');
}

function sections(text) {
  const out = new Map();
  let name = null;
  for (const line of text.split('\n')) {
    const h = /^## (.+)$/.exec(line);
    if (h) { name = h[1].trim(); out.set(name, []); continue; }
    if (name) out.get(name).push(line);
  }
  for (const n of NEEDED) {
    if (!out.has(n)) throw new Error(`settings.md: no "## ${n}" section`);
  }
  return out;
}

// Paragraphs and "- " lists, as blocks of Markdown text.
function blocksOf(lines) {
  const blocks = [];
  let para = [];
  let list = null;
  const flush = () => {
    if (para.length) blocks.push({ kind: 'p', text: para.join(' ') });
    if (list) blocks.push({ kind: 'ul', items: list });
    para = [];
    list = null;
  };
  for (const line of lines) {
    if (!line.trim()) { flush(); continue; }
    if (line.startsWith('- ')) {
      if (para.length) { blocks.push({ kind: 'p', text: para.join(' ') }); para = []; }
      list = list || [];
      list.push(line.slice(2).trim());
    } else if (list && /^\s+\S/.test(line)) {
      list[list.length - 1] += ' ' + line.trim();
    } else {
      if (list) { blocks.push({ kind: 'ul', items: list }); list = null; }
      para.push(line.trim());
    }
  }
  flush();
  return blocks;
}

function cellsOf(line) {
  const inner = line.trim().replace(/^\|/, '').replace(/\|$/, '');
  return inner.split(/(?<!\\)\|/).map((c) => c.trim().replace(/\\\|/g, '|'));
}

// A Default cell holds the value, then where the rule is written, and
// sometimes a note, such as a limit on the libraries. The value ends where
// the first source starts: a link, a file path in code, an ADR, a ticket, or
// a ruling. The rest splits into clauses. A clause that starts with a source
// is a source. A clause with a source later keeps its words before it as a
// note, such as "SQL cannot name one, by tickets 0109 and 0110".
const SOURCE_START = /\[[^\]]+\]\([^)]+\)|`[^`\s]*\/[^`\s]*\.[a-z]+`|\bADR \d|\btickets? \d|\bIan's ruling/;
const SOURCE_ANY = new RegExp(`${SOURCE_START.source}|\\bNo reason recorded`);
const LINK_WORDS = /[\s,;]*\b(by|in)?\s*$/;

const letters = (s) => plain(s).replace(/[^A-Za-z0-9]/g, '');

export function splitDefault(cell) {
  const m = SOURCE_START.exec(cell);
  const value = m ? cell.slice(0, m.index).replace(/[\s.,;]+$/, '') : cell;
  if (!m || !value) return { value: cell, note: '', source: '' };
  const notes = [];
  const sources = [];
  let joins = '';
  for (const clause of cell.slice(m.index).split(/(?<=\.)\s+|;\s+/)) {
    const c = clause.trim().replace(/\.$/, '');
    if (!c) continue;
    const at = SOURCE_ANY.exec(c);
    if (!at) { notes.push(c); continue; }
    if (at.index === 0) { sources.push(c); continue; }
    const before = c.slice(0, at.index);
    const lead = before.replace(LINK_WORDS, '');
    joins += before.slice(lead.length);
    notes.push(lead);
    sources.push(c.slice(at.index));
  }
  const cap = (s) => s.charAt(0).toUpperCase() + s.slice(1);
  const out = {
    value,
    note: notes.map((n) => `${cap(n)}.`).join(' '),
    source: sources.map(cap).join('. '),
  };
  // Every letter of the cell lands in one part, apart from the joining
  // words dropped before a source. A split that loses words fails.
  if (letters(cell).length !== letters(out.value + out.note + out.source + joins).length) {
    throw new Error(`settings.md: the Default cell "${cell.slice(0, 40)}" lost words when the site split it`);
  }
  return out;
}

// The page shows the prose, not where each rule is written. A sentence that
// only cites a record goes, and so does a trailing clause that cites an ADR.
const CITE_SENTENCE = /^(ADR \d+\.|The batching design\b|Tickets? \d|Ian's ruling)/;
const CITE_CLAUSE = /,\s*(by|as) ADR \d+( states)?(?=[.,])/g;
// A setting's meaning drops a record cited in brackets, such as "(ticket 0143)".
const CITE_BRACKET = /\s*\((?:ADR|tickets?) \d+(?:(?:,| and) \d+)*\)/g;

function uncited(text) {
  return text.split(/(?<=\.)\s+(?=[A-Z`'])/)
    .filter((s) => !CITE_SENTENCE.test(s.trim()))
    .join(' ')
    .replace(CITE_CLAUSE, '');
}

function uncitedBlocks(blocks) {
  return blocks.map((b) => (b.kind === 'p'
    ? { kind: 'p', text: uncited(b.text) }
    : { kind: 'ul', items: b.items.map(uncited) }));
}

// Ian's ruling 5 of 2026-09-25: no setting may do nothing. A surface the
// table marks "no effect" is left off the site, and so is a note that says so.
const NO_EFFECT = /\bno effect\b/;

export function parseSettings(text) {
  const parts = sections(text);

  const tableLines = parts.get('Settings').filter((l) => l.trim().startsWith('|'));
  if (tableLines.length < 3) throw new Error('settings.md: the Settings section holds no table');
  const head = cellsOf(tableLines[0]);
  if (head.join('|') !== COLUMNS.join('|')) {
    throw new Error(`settings.md: the table's columns changed.\n  want: ${COLUMNS.join(', ')}\n  have: ${head.join(', ')}`);
  }
  const rows = tableLines.slice(2).map((line, i) => {
    const cells = cellsOf(line);
    if (cells.length !== COLUMNS.length) {
      throw new Error(`settings.md: table row ${i + 1} has ${cells.length} cells, not ${COLUMNS.length}`);
    }
    const [name, does, dflt, allowed, ...surfaces] = cells;
    if (!name) throw new Error(`settings.md: table row ${i + 1} has no setting name`);
    const def = splitDefault(dflt);
    return {
      name,
      id: slug(name),
      does: does.replace(CITE_BRACKET, ''),
      default: { ...def, note: def.note.split(/(?<=\.)\s+/).filter((n) => !NO_EFFECT.test(n)).join(' ') },
      allowed,
      on: Object.fromEntries(SURFACES.map((s, j) => [s, NO_EFFECT.test(surfaces[j]) ? ABSENT : surfaces[j]])),
    };
  });
  const ids = new Set();
  for (const r of rows) {
    if (ids.has(r.id)) throw new Error(`settings.md: two settings share the address #${r.id}`);
    ids.add(r.id);
  }

  return {
    whatCounts: blocksOf(parts.get('What counts as a setting')),
    precedence: uncitedBlocks(blocksOf(parts.get('Precedence'))),
    howToRead: blocksOf(parts.get('How to read a cell')),
    rows,
  };
}

// ------------------------------------------------------------ lookup

// A page that names a setting's default, range or allowed values reads it
// here, so no page types a value the table holds. setting('Retries').default
// is the table's Default value as a reader sees it. A name the table does not
// hold fails the build and names the page's call, so a rename or a removal
// breaks loudly in place of printing a stale value.
//
// The build runs from site/, and a bundled page has no stable path of its
// own, so the lookup finds specification/settings.md from the working
// folder up.
function specPath() {
  for (let dir = process.cwd(); ; dir = path.dirname(dir)) {
    const file = path.join(dir, 'specification', 'settings.md');
    if (fs.existsSync(file)) return file;
    if (path.dirname(dir) === dir) throw new Error(`settings lookup: no specification/settings.md above ${process.cwd()}`);
  }
}

let table = null;
const NUMBER = /\d[\d,]*(?:\.\d+)?/;

export function setting(name) {
  table ??= parseSettings(fs.readFileSync(specPath(), 'utf8')).rows;
  const row = table.find((r) => r.name === name);
  if (!row) {
    throw new Error(`settings lookup: specification/settings.md has no setting "${name}". A page asks for it through setting('${name}'). Change that call to follow the table.\n  the table holds: ${table.map((r) => r.name).join(', ')}`);
  }
  const fail = (what) => { throw new Error(`settings lookup: ${name}: ${what}`); };
  const value = plain(row.default.value);
  return {
    name: row.name,
    href: `/install/settings/#${row.id}`,
    // The Default value, such as "2", "30 seconds" or "Off".
    default: value,
    // The note that goes with the default, such as a fixed value on the libraries.
    note: plain(row.default.note),
    // The Allowed values, such as "1 to 32".
    allowed: plain(row.allowed),
    // One surface's spelling, such as setting('Prune target').surface('Configuration file').
    surface(s) {
      if (!(s in row.on)) fail(`no surface "${s}"`);
      if (row.on[s] === ABSENT) fail(`the table says ${s} is ${ABSENT}`);
      return plain(row.on[s]);
    },
    // The first number in the default, as written: "30", "100,000,000".
    get number() {
      return (NUMBER.exec(value) || fail(`the default "${value}" holds no number`))[0];
    },
    // The range in the allowed values, "LOW to HIGH ...".
    get range() {
      const m = /^(\d[\d,]*) to (\d[\d,]*)\b/.exec(plain(row.allowed)) || fail(`the allowed values "${plain(row.allowed)}" are not "LOW to HIGH"`);
      return { min: m[1], max: m[2] };
    },
    // The default for one function, from a value written as
    // "0.5 on `decide`, `tag`; none on `choose`".
    defaultOn(fn) {
      for (const clause of row.default.value.split(/;\s*/)) {
        const m = /^(.+?) on (.+)$/.exec(clause);
        if (m && [...m[2].matchAll(/`([^`]+)`/g)].some((c) => c[1] === fn)) return plain(m[1]);
      }
      return fail(`the default "${value}" names no value for ${fn}`);
    },
  };
}

// ------------------------------------------------------------ inline Markdown

const escape = (s) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');

// A link in settings.md is relative to specification/. On the site it points
// at the same file in the public repository.
function hrefOf(href) {
  if (/^[a-z]+:/.test(href)) return href;
  const [file, hash] = href.split('#');
  const resolved = path.posix.normalize(path.posix.join('specification', file));
  if (resolved.startsWith('..')) throw new Error(`settings.md: a link leaves the repository: ${href}`);
  return REPO_BLOB + resolved + (hash ? `#${hash}` : '');
}

const INLINE = /`([^`]+)`|\[([^\]]+)\]\(([^)]+)\)/g;

export function inlineHtml(md) {
  let out = '';
  let at = 0;
  for (const m of md.matchAll(INLINE)) {
    out += escape(md.slice(at, m.index));
    if (m[1] !== undefined) out += `<code>${escape(m[1])}</code>`;
    else out += `<a href="${escape(hrefOf(m[3]))}">${inlineHtml(m[2])}</a>`;
    at = m.index + m[0].length;
  }
  return out + escape(md.slice(at));
}

// The words a reader sees, with the Markdown marks taken out. The check
// compares these with the built page.
export function plain(md) {
  return md.replace(INLINE, (all, code, text) => (code !== undefined ? code : plain(text)));
}

// The prose sections, paragraphs and lists, as HTML.
export function blocksHtml(blocks) {
  return blocks.map((b) => (b.kind === 'p'
    ? `<p>${inlineHtml(b.text)}</p>`
    : `<ul>${b.items.map((i) => `<li>${inlineHtml(i)}</li>`).join('')}</ul>`)).join('');
}
