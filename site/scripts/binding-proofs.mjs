// What scripts/smoke-bindings.mjs writes and scripts/check-binding-proofs.mjs
// reads: the replay list, the hashes of a sample's own files, the recorded
// answers, and the tree hash of the folders a binding builds on.

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { spawnSync } from 'node:child_process';

export const PROOF_FILE = 'bindings-proof.json';
export const REPLAY_FILE = 'REPLAY';

export const sha256 = (data) => crypto.createHash('sha256').update(data).digest('hex');

// examples/REPLAY: one sample path a line, relative to examples/. A line
// may end with backend=NAME, and the run then names that backend. The whole
// line keys the sample's proof entry.
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

// The site-owned inputs of one sample: the sample, its saved output when
// one exists, and each file in its page's files/.
export function sampleHashes(examples, rel) {
  const file = path.join(examples, rel);
  const output = `${file}.out`;
  const filesDir = path.join(path.dirname(file), 'files');
  const files = {};
  const walk = (dir) => {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, e.name);
      if (e.isDirectory()) walk(p);
      else files[path.relative(examples, p)] = sha256(fs.readFileSync(p));
    }
  };
  if (fs.existsSync(filesDir)) walk(filesDir);
  return {
    sample: sha256(fs.readFileSync(file)),
    output: fs.existsSync(output) ? sha256(fs.readFileSync(output)) : null,
    files: Object.fromEntries(Object.entries(files).sort()),
  };
}

// The fixture's lines. An answer line carries its key, the address it was
// posted to, its question and the text of the shared state it was asked
// with. A state line carries no key.
export function fixtureLines(file) {
  const rows = fs.readFileSync(file, 'utf8').split('\n').filter((l) => l.trim()).map((text) => ({ text, row: JSON.parse(text) }));
  const states = new Map(rows.filter(({ row }) => !row.key).map(({ row }) => [row.sha256, row.state ?? '']));
  return rows.map(({ text, row }) => ({
    text, key: row.key ?? null, url: row.url ?? null, question: row.question ?? '', state: row.key ? states.get(row.state) ?? '' : '',
  }));
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
export function samplePage(rel) {
  return rel.startsWith('functions/') ? `/functions/${rel.split('/')[1]}/` : `/install/${rel.split('/')[1]}/`;
}

// No key, address, backend or setting from the shell may reach a sample.
export function leakedVariables(env) {
  return Object.keys(env).filter((n) => n.startsWith('THINKTHEN_') || ['TYPESAFE_API_KEY', 'LIQUIDAI_API_KEY', 'LIQUID_API_KEY', 'OLLAMA_API_KEY'].includes(n));
}

// The store loads the whole fixture, so it cannot say which answers a sample
// read. The answers worth keeping first are those whose question or shared
// state holds one of the sample's string literals of 12 characters or more.
// A literal that holds JSON, as a C sample's does, also gives each quoted
// string inside it. `passes(answers)` runs the sample over those answers.
// The kept answers must pass. Each one is then dropped in turn, and an
// answer whose loss fails the sample is one it read. Returns the answers
// read, or a reason the runner cannot tell.
export function narrow(fixture, text, passes) {
  const unescape = (s) => s.replace(/\\n/g, '\n').replace(/\\(.)/g, '$1');
  const whole = [...text.matchAll(/"((?:[^"\\\n]|\\.){12,})"|'((?:[^'\\\n]|\\.){12,})'/g)].map((m) => unescape(m[1] ?? m[2]));
  const inner = whole.flatMap((l) => [...l.matchAll(/"((?:[^"\\]|\\.){12,})"/g)].map((m) => m[1]));
  const literals = [...new Set([...whole, ...inner])];
  let kept = fixture.filter((l) => l.key && literals.some((x) => l.question.includes(x) || l.state.includes(x)));
  if (!passes(kept)) return { reason: 'the answers whose question or shared state holds one of its literals do not answer it. The runner cannot tell which answers it read.' };
  for (const line of [...kept]) {
    const without = kept.filter((l) => l !== line);
    if (passes(without)) kept = without;
  }
  if (!kept.length || !passes(kept)) return { reason: 'the answers it read do not answer it on their own.' };
  return { kept };
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

// The Cargo workspace members a binding builds on, from cargo metadata.
export function cargoFolders(repo, manifest) {
  const done = spawnSync('cargo', ['metadata', '--format-version', '1', '--locked', '--offline', '--manifest-path', path.join(repo, manifest)], { encoding: 'utf8', maxBuffer: 1 << 26 });
  if (done.status !== 0) throw new Error(`cargo metadata failed for ${manifest}\n${done.stderr}`);
  return JSON.parse(done.stdout).packages
    .filter((p) => p.source === null)
    .map((p) => path.relative(repo, path.dirname(p.manifest_path)));
}

// One hash over every tracked file under the folders, as the working tree
// holds it.
export function sourceTree(repo, folders) {
  const listed = spawnSync('git', ['ls-files', '-z', '--', ...folders], { cwd: repo, encoding: 'utf8', maxBuffer: 1 << 26 });
  if (listed.status !== 0) throw new Error(`git ls-files failed: ${listed.stderr}`);
  const lines = listed.stdout.split('\0').filter(Boolean).sort().map((rel) => {
    const p = path.join(repo, rel);
    return `${rel}\0${fs.existsSync(p) ? sha256(fs.readFileSync(p)) : 'missing'}\n`;
  });
  return sha256(lines.join(''));
}
