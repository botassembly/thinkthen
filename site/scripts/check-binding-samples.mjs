#!/usr/bin/env node
// Check replay sample coverage and install package/archive names.

import fs from 'node:fs';
import path from 'node:path';
import { SURFACES } from '../src/data/catalog.mjs';
import { readReplayList, replayLine, librarySamples } from './binding-samples.mjs';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const repo = path.resolve(site, '..');
const examples = path.join(site, 'examples');

const problems = [];

const listed = readReplayList(examples);
// Every listed sample exists, and every SQL sample has expected output.
for (const line of listed) {
  const { rel } = replayLine(line);
  if (!fs.existsSync(path.join(examples, rel))) problems.push(`examples/REPLAY names ${rel}, which does not exist.`);
  if (rel.endsWith('.sql') && !fs.existsSync(path.join(examples, `${rel}.out`))) problems.push(`examples/${rel} has no saved output. Run node scripts/smoke-sql.mjs --update ${rel}, then read examples/${rel}.out.`);
}
const runner = (rel) => rel.endsWith('.sql') ? 'smoke-sql.mjs' : 'smoke-bindings.mjs';

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
    const archive = /^thinkthen-([a-z0-9-]+?)-VERSION-TARGET\.tar\.gz$/.exec(line);
    if (!archive) continue;
    // A wrapper name ends at a space or a semicolon, so a prefix such as co never matches cobol.
    const wrapper = new RegExp(`part_source_wrapper ${archive[1]}[ ;]`);
    const made = pack.includes(`pack "thinkthen-${archive[1]}-$V-$TARGET.tar.gz"`) || wrapper.test(pack);
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

if (problems.length) {
  console.error(`check-binding-samples: ${problems.length} problems\n  ${problems.join('\n  ')}`);
  process.exit(1);
}
console.log(`check-binding-samples: ${listed.length} samples listed; package and archive names match.`);
