// Read the built-in backends from the "Named backends" table in
// specification/backends.md. The Backends pages and scripts/check-settings.mjs
// both call it, so no page types a backend's address, key variable or model.

import fs from 'node:fs';
import path from 'node:path';

const HEADER = ['Name', 'Base', 'Path', 'Key variables, first nonblank wins', 'Model', 'Descriptions'];

function specPath() {
  for (let dir = process.cwd(); ; dir = path.dirname(dir)) {
    const file = path.join(dir, 'specification', 'backends.md');
    if (fs.existsSync(file)) return file;
    if (path.dirname(dir) === dir) throw new Error(`backends table: no specification/backends.md above ${process.cwd()}`);
  }
}

const cells = (line) => line.trim().replace(/^\||\|$/g, '').split('|').map((c) => c.trim());
const ticked = (cell) => [...cell.matchAll(/`([^`]+)`/g)].map((m) => m[1]);

// Each row: name, base, key variables in order, model, and whether the
// backend sends descriptions as text.
export function parseBackends(text) {
  const lines = text.split('\n');
  const start = lines.findIndex((l) => l.trim() === '## Named backends');
  if (start < 0) throw new Error('backends.md: no "## Named backends" section');
  const head = lines.findIndex((l, i) => i > start && l.startsWith('| Name |'));
  if (head < 0) throw new Error('backends.md: the Named backends section has no table');
  if (cells(lines[head]).join('|') !== HEADER.join('|')) {
    throw new Error(`backends.md: the Named backends table's columns changed. Expected: ${HEADER.join(', ')}`);
  }
  const rows = [];
  for (const line of lines.slice(head + 2)) {
    if (!line.startsWith('|')) break;
    const [name, base, postingPath, keys, model, descriptions] = cells(line);
    const row = {
      name: ticked(name)[0],
      base: ticked(base)[0],
      path: ticked(postingPath)[0],
      keys: ticked(keys),
      model: ticked(model)[0],
      text: /^as text/.test(descriptions),
    };
    if (!row.name || !row.base || !row.path || !row.keys.length || !row.model) throw new Error(`backends.md: a Named backends row lacks a name, base, key or model: ${line}`);
    rows.push(row);
  }
  if (!rows.length) throw new Error('backends.md: the Named backends table has no rows');
  return rows;
}

let table = null;

export function backends() {
  table ??= parseBackends(fs.readFileSync(specPath(), 'utf8'));
  return table;
}

// One built-in backend by name. A name the table lacks fails the build.
export function backend(name) {
  const row = backends().find((r) => r.name === name);
  if (!row) throw new Error(`backends lookup: specification/backends.md has no built-in backend "${name}". The table holds: ${backends().map((r) => r.name).join(', ')}`);
  return row;
}
