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
// posted to, and its question.
export function fixtureLines(file) {
  return fs.readFileSync(file, 'utf8').split('\n').filter((l) => l.trim()).map((text) => {
    const row = JSON.parse(text);
    return { text, key: row.key ?? null, url: row.url ?? null, question: row.question ?? '' };
  });
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
