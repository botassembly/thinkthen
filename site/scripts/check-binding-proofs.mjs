#!/usr/bin/env node
// Check examples/bindings-proof.json against the site's own files. The build
// runs it, and it needs only Node and git.
//
// It fails when a sample that examples/REPLAY lists has no entry, or an
// entry has no listed sample. It fails when a sample, its saved output, a
// file it reads, or a recorded answer it read no longer matches its entry.
// Run scripts/smoke-bindings.mjs on a host with the toolchains to prove the
// sample again.
//
// A change in a binding's folder or in a Cargo workspace member it builds
// on only warns, and names each page to prove again. Another queue's commit
// never turns the site build red. The marketing lead reruns the replay at
// each published checkpoint tag.
//
// It also fails when an install line does not name the package that the
// binding's own metadata names.

import fs from 'node:fs';
import path from 'node:path';
import { SURFACES } from '../src/data/catalog.mjs';
import { readReplayList, sampleHashes, fixtureLines, sourceTree, sha256, PROOF_FILE } from './binding-proofs.mjs';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const repo = path.resolve(site, '..');
const examples = path.join(site, 'examples');

const problems = [];
const warnings = [];

const listed = readReplayList(examples);
const proofPath = path.join(examples, PROOF_FILE);
const proof = fs.existsSync(proofPath) ? JSON.parse(fs.readFileSync(proofPath, 'utf8')) : {};
const answers = new Map(fixtureLines(path.join(site, 'recordings', 'thinkthen.jsonl')).filter((l) => l.key).map((l) => [l.key, sha256(l.text)]));
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);

for (const rel of listed) {
  if (!fs.existsSync(path.join(examples, rel))) { problems.push(`examples/REPLAY names ${rel}, which does not exist.`); continue; }
  const entry = proof[rel];
  if (!entry) { problems.push(`${rel} has no proof. Run node scripts/smoke-bindings.mjs ${rel}.`); continue; }
  const now = sampleHashes(examples, rel);
  if (now.sample !== entry.sample) problems.push(`${rel} changed after its proof. Run node scripts/smoke-bindings.mjs ${rel}.`);
  if (now.output !== entry.output) problems.push(`${rel}'s saved output changed after its proof.`);
  if (!same(now.files, entry.files)) problems.push(`a file ${rel} reads changed after its proof.`);
  for (const [key, hash] of Object.entries(entry.answers)) {
    if (!answers.has(key)) problems.push(`${rel} read the recorded answer ${key}, which recordings/ no longer holds.`);
    else if (answers.get(key) !== hash) problems.push(`${rel} read the recorded answer ${key}, which changed after its proof.`);
  }
  if (sourceTree(repo, entry.sources.folders) !== entry.sources.tree) {
    warnings.push(`${entry.page}: ${entry.sources.folders.join(', ')} changed after the proof of ${rel}. Prove the page again.`);
  }
}
for (const rel of Object.keys(proof)) {
  if (!listed.includes(rel)) problems.push(`${PROOF_FILE} holds ${rel}, which examples/REPLAY does not list.`);
}

// Each install line names the package its binding's metadata names.
const read = (rel) => fs.readFileSync(path.join(repo, rel), 'utf8');
const pythonName = () => read('libraries/python/pyproject.toml').match(/^name = "(.+)"/m)[1];
const PACKAGE = {
  python: pythonName,
  polars: pythonName,
  pandas: pythonName,
  typescript: () => JSON.parse(read('libraries/typescript/package.json')).name,
  ruby: () => read('libraries/ruby/thinkthen.gemspec').match(/\.name\s*=\s*["'](.+?)["']/)[1],
  r: () => read('libraries/r/thinkthen/DESCRIPTION').match(/^Package:\s*(\S+)/m)[1],
  rust: () => read('crates/thinkthen/Cargo.toml').match(/^name = "(.+)"/m)[1],
};
for (const surface of SURFACES) {
  const name = PACKAGE[surface.slug];
  if (!name) continue;
  const want = name();
  const escaped = want.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const pattern = new RegExp(`(^|[\\s"'/:])${escaped}($|[\\s"'\\[@:])`);
  for (const [line] of surface.install) {
    if (!pattern.test(line)) problems.push(`/install/${surface.slug}/: the install line "${line}" does not name the package ${want}.`);
  }
}

for (const w of warnings) console.warn(`check-binding-proofs: warning: ${w}`);
if (problems.length) {
  console.error(`check-binding-proofs: ${problems.length} problems\n  ${problems.join('\n  ')}`);
  process.exit(1);
}
console.log(`check-binding-proofs: ${listed.length} replayed samples match their proofs; ${warnings.length} pages to prove again.`);
