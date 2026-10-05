// Shared replay sample paths and recording inputs.

import fs from 'node:fs';
import path from 'node:path';

export const REPLAY_FILE = 'REPLAY';

// examples/REPLAY: one sample path a line, relative to examples/. A line
// may end with backend=NAME, and the run then names that backend. The whole
// line selects the sample and its backend.
export function readReplayList(examples) {
  return fs.readFileSync(path.join(examples, REPLAY_FILE), 'utf8').split('\n')
    .map((l) => l.trim()).filter((l) => l && !l.startsWith('#'));
}

// One REPLAY line as its sample path and its backend, or null for none.
export function replayLine(line) {
  const [rel, ...rest] = line.split(/\s+/);
  const named = rest.map((r) => /^backend=([a-z0-9-]+)$/.exec(r));
  if (named.some((m) => !m) || named.length > 1) throw new Error(`examples/REPLAY: "${line}" holds more than a path and backend=NAME`);
  return { rel, backend: named[0]?.[1] ?? null };
}

// Preserve each recording line and the answer key/address used to select it.
// State lines have no answer key and are copied alongside the answers.
export function fixtureLines(file) {
  return fs.readFileSync(file, 'utf8').split('\n').filter((text) => text.trim()).map((text) => {
    const row = JSON.parse(text);
    return { text, key: row.key ?? null, url: row.url ?? null };
  });
}

// A sample on a function page sits at functions/<fn>/<surface>.<ext>, or at
// functions/<fn>/more/<surface>.<ext> when the page shows it under
// Reference. Every other sample sits at <section>/<surface>/<name>.<ext>.
export function sampleSurface(rel) {
  return rel.startsWith('functions/') ? path.basename(rel).replace(/\..*$/, '') : rel.split('/')[1];
}
// The files/ folder a sample reads: its page's, beside the sample or one
// folder up for a sample under more/.
export function sampleFiles(examples, rel) {
  return path.join(examples, path.dirname(rel).replace(/\/more$/, ''), 'files');
}
// No key, address, backend or setting from the shell may reach a sample.
export function leakedVariables(env) {
  return Object.keys(env).filter((n) => n.startsWith('THINKTHEN_') || ['TYPESAFE_API_KEY', 'LIQUIDAI_API_KEY', 'LIQUID_API_KEY', 'OLLAMA_API_KEY'].includes(n));
}

// The library and SQL samples under examples/, outside each page's files/.
// Every one needs a REPLAY line.
const SAMPLE = /\.(py|ts|rb|R|rs|c|cpp|m|cob|adb|java|kt|scala|cs|go|swift|zig|php|dart|sql)$/;
export function librarySamples(examples) {
  const found = [];
  const walk = (dir) => {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, e.name);
      if (e.isDirectory()) { if (e.name !== 'files') walk(p); } else if (SAMPLE.test(e.name)) found.push(path.relative(examples, p));
    }
  };
  walk(examples);
  return found.sort();
}
