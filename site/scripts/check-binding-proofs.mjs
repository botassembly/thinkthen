#!/usr/bin/env node
// Check examples/bindings-proof.json against the site's own files. The build
// runs it, and it needs only Node and git.
//
// It fails when a line that examples/REPLAY lists has no entry, or an
// entry has no listed line. A line is a sample path, and a backend=NAME
// when the run names a backend. It fails when a sample, its saved output, a
// file it reads, or a recorded answer it read no longer matches its entry.
// Run scripts/smoke-bindings.mjs, or scripts/smoke-sql.mjs for a SQL
// sample, on a host with the toolchains to prove the sample again.
//
// A change in a binding's folder or in a Cargo workspace member it builds
// on only warns, and names each page to prove again. Another queue's commit
// never turns the site build red.
//
// With --strict, that change fails instead. The Pages workflow runs it that
// way, so a deploy needs proof that every sample passed against the code it
// ships. The code release can run it that way too. The ThinkThen queue owner
// decides whether it does.
//
// It fails when a library or SQL sample under examples/ has no REPLAY
// line, or a listed SQL sample has no saved output beside it as
// <sample>.out.
//
// It also fails when an install line does not name the package that the
// binding's own metadata names.

import fs from 'node:fs';
import path from 'node:path';
import { SURFACES } from '../src/data/catalog.mjs';
import { readReplayList, replayLine, sampleHashes, fixtureLines, sourceTree, sha256, PROOF_FILE, librarySamples } from './binding-proofs.mjs';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const repo = path.resolve(site, '..');
const examples = path.join(site, 'examples');

const strict = process.argv.includes('--strict');
const problems = [];
const warnings = [];
const stalePages = new Set();

const listed = readReplayList(examples);
const proofPath = path.join(examples, PROOF_FILE);
const proof = fs.existsSync(proofPath) ? JSON.parse(fs.readFileSync(proofPath, 'utf8')) : {};
const answers = new Map(fixtureLines(path.join(site, 'recordings', 'thinkthen.jsonl')).filter((l) => l.key).map((l) => [l.key, sha256(l.text)]));
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);
// smoke-sql.mjs proves the SQL samples, and smoke-bindings.mjs the rest.
const runner = (rel) => (rel.endsWith('.sql') ? 'smoke-sql.mjs' : 'smoke-bindings.mjs');

// A REPLAY line keys its entry. Its sample path names the files.
for (const line of listed) {
  const { rel } = replayLine(line);
  if (!fs.existsSync(path.join(examples, rel))) { problems.push(`examples/REPLAY names ${rel}, which does not exist.`); continue; }
  if (rel.endsWith('.sql') && !fs.existsSync(path.join(examples, `${rel}.out`))) problems.push(`examples/${rel} has no saved output. Run node scripts/${runner(rel)} --update ${rel}, then read examples/${rel}.out.`);
  const entry = proof[line];
  if (!entry) { problems.push(`${line} has no proof. Run node scripts/${runner(rel)} ${rel}.`); continue; }
  const now = sampleHashes(examples, rel);
  if (now.sample !== entry.sample) problems.push(`${line}: ${rel} changed after its proof. Run node scripts/${runner(rel)} ${rel}.`);
  if (now.output !== entry.output) problems.push(`${line}: the saved output changed after its proof.`);
  if (!same(now.files, entry.files)) problems.push(`${line}: a file the sample reads changed after its proof.`);
  for (const [key, hash] of Object.entries(entry.answers)) {
    if (!answers.has(key)) problems.push(`${line} read the recorded answer ${key}, which recordings/ no longer holds.`);
    else if (answers.get(key) !== hash) problems.push(`${line} read the recorded answer ${key}, which changed after its proof.`);
  }
  if (sourceTree(repo, entry.sources.folders) !== entry.sources.tree) {
    stalePages.add(entry.page);
    (strict ? problems : warnings).push(`${entry.page}: ${entry.sources.folders.join(', ')} changed after the proof of ${line}. Prove the page again with npm run test-docs.`);
  }
}
for (const line of Object.keys(proof)) {
  if (!listed.includes(line)) problems.push(`${PROOF_FILE} holds ${line}, which examples/REPLAY does not list.`);
}

// Every library and SQL sample replays.
const replayed = new Set(listed.map((line) => replayLine(line).rel));
for (const rel of librarySamples(examples)) {
  if (!replayed.has(rel)) problems.push(`examples/${rel} has no line in examples/REPLAY. Add one and run node scripts/${runner(rel)} ${rel}.`);
}

// Each registry install line names the package its binding's metadata
// names.
const read = (rel) => fs.readFileSync(path.join(repo, rel), 'utf8');
const pythonName = () => read('libraries/python/pyproject.toml').match(/^name = "(.+)"/m)[1];
const maven = () => {
  const pom = read('libraries/jvm/pom.xml');
  return `${pom.match(/<groupId>(.+?)<\/groupId>/)[1]}:${pom.match(/<artifactId>(.+?)<\/artifactId>/)[1]}`;
};
const PACKAGE = {
  python: pythonName,
  polars: pythonName,
  pandas: pythonName,
  typescript: () => JSON.parse(read('libraries/typescript/package.json')).name,
  ruby: () => read('libraries/ruby/thinkthen.gemspec').match(/\.name\s*=\s*["'](.+?)["']/)[1],
  r: () => read('libraries/r/thinkthen/DESCRIPTION').match(/^Package:\s*(\S+)/m)[1],
  rust: () => read('crates/thinkthen/Cargo.toml').match(/^name = "(.+)"/m)[1],
  java: () => maven(),
  kotlin: () => maven(),
  scala: () => maven(),
  go: () => read('libraries/go/go.mod').match(/^module (\S+)/m)[1],
  csharp: () => read('libraries/csharp/Botassembly.ThinkThen.nuspec').match(/<id>(.+?)<\/id>/)[1],
  php: () => JSON.parse(read('libraries/php/composer.json')).name,
  dart: () => read('libraries/dart/pubspec.yaml').match(/^name: (\S+)/m)[1],
};
// Each release archive an install line names is one release-pack makes.
const pack = read('sdlc/scripts/release-pack');
for (const surface of SURFACES) {
  for (const [line] of surface.install) {
    const archive = /^thinkthen-([a-z-]+?)-VERSION-TARGET\.tar\.gz$/.exec(line);
    if (!archive) continue;
    const made = archive[1] === 'c' ? pack.includes('pack "thinkthen-c-$V-$TARGET.tar.gz"') : pack.includes(`part_source_wrapper ${archive[1]}`);
    if (!made) problems.push(`/install/${surface.slug}/: release-pack makes no archive named ${line}.`);
  }
}

for (const surface of SURFACES) {
  const name = PACKAGE[surface.slug];
  if (!name) continue;
  const want = name();
  const escaped = want.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const pattern = new RegExp(`(^|[\\s"'/:])${escaped}($|[\\s"'\\[@:])`);
  // A release archive row names the C library, which the archive check covers.
  for (const [line] of surface.install.filter(([l]) => !l.endsWith('.tar.gz'))) {
    if (!pattern.test(line)) problems.push(`/install/${surface.slug}/: the install line "${line}" does not name the package ${want}.`);
  }
}

for (const w of warnings) console.warn(`check-binding-proofs: warning: ${w}`);
if (problems.length) {
  console.error(`check-binding-proofs: ${problems.length} problems\n  ${problems.join('\n  ')}`);
  process.exit(1);
}
console.log(`check-binding-proofs: ${listed.length} replayed samples match their proofs; ${stalePages.size} pages to prove again.`);
