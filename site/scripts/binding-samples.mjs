// Shared replay sample paths and recording inputs.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { execFileSync, spawnSync } from 'node:child_process';

// Checkout scans read indexed source, including staged additions. Build output
// remains outside the source set even when it sits beneath a source folder.
export function trackedFiles(root) {
  const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
  // An archive beneath another checkout must never borrow its parent's index.
  if (!fs.existsSync(path.join(repo, '.git'))) throw new Error(`source scan requires a checkout: ${repo}`);
  root = fs.realpathSync(root);
  const checkout = fs.realpathSync(repo);
  const relative = path.relative(checkout, root);
  if (relative === '..' || relative.startsWith(`..${path.sep}`) || path.isAbsolute(relative)) {
    throw new Error(`source scan is outside the checkout: ${root}`);
  }
  const env = { ...process.env };
  for (const name of Object.keys(env)) if (name.startsWith('GIT_')) delete env[name];
  return [...new Set(execFileSync('git', ['-C', checkout, '--literal-pathspecs', 'ls-files', '-z', '--', relative || '.'], { env, encoding: 'utf8' })
    .split('\0').filter(Boolean))].sort().map((name) => path.join(checkout, name));
}

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
  if (rel.startsWith('learn/python/files.')) {
    return path.resolve(examples, '../../specification/fixtures/files');
  }
  return path.join(examples, path.dirname(rel).replace(/\/more$/, ''), 'files');
}
// No key, address, backend or setting from the shell may reach a sample.
// The JVM toolchain homes name only where a compiler sits, so they may.
const TOOLCHAIN_HOMES = new Set(['THINKTHEN_JDK_HOME', 'THINKTHEN_KOTLIN_HOME', 'THINKTHEN_SCALA_HOME']);
export function leakedVariables(env) {
  return Object.keys(env).filter((n) => (n.startsWith('THINKTHEN_') && !TOOLCHAIN_HOMES.has(n)) || ['TYPESAFE_API_KEY', 'LIQUIDAI_API_KEY', 'LIQUID_API_KEY', 'OLLAMA_API_KEY'].includes(n));
}

// The library and SQL samples under examples/, outside each page's files/.
// Every one needs a REPLAY line.
const SAMPLE = /\.(py|ts|rb|R|rs|c|cpp|m|cob|adb|java|kt|scala|cs|go|swift|zig|php|dart|sql)$/;
export function librarySamples(examples) {
  return trackedFiles(examples).map((file) => path.relative(examples, file))
    .filter((rel) => !rel.split(path.sep).slice(0, -1).includes('files') && SAMPLE.test(rel));
}

// Admit compiler commands through the shared four-build supervisor.
// The supervisor retains the checkpoint barrier and owns child cleanup.
export function compile(cmd, args, opts = {}) {
  const lock = '/tmp/thinkthen-heavy.lock';
  const helper = path.resolve(path.dirname(new URL(import.meta.url).pathname), '../../sdlc/scripts/build-admission.py');
  const env = { ...process.env, ...opts.env, RUSTC_WRAPPER: '', CARGO_BUILD_JOBS: '2' };
  return spawnSync('python3', [helper, '--lock', lock, '--', cmd, ...args], {
    encoding: 'utf8', maxBuffer: 1 << 26, ...opts, env,
  });
}
