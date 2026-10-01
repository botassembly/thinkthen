#!/usr/bin/env node
// Replay each SQL sample that examples/REPLAY names against its database
// extension, built from this working tree, and compare what the database
// printed with the sample's saved <sample>.sql.out. The run writes what it
// proved to examples/bindings-proof.json, beside the library entries that
// scripts/smoke-bindings.mjs writes. scripts/check-binding-proofs.mjs reads
// that file in every build.
//
//   node scripts/smoke-sql.mjs                  replay every listed SQL sample
//   node scripts/smoke-sql.mjs PATTERN          only the samples whose path holds PATTERN
//   node scripts/smoke-sql.mjs --update         write each sample's .sql.out from its run
//   node scripts/smoke-sql.mjs --allow-missing  pass when a toolchain is missing
//
// A sample runs in a fresh folder that holds its page's files/ and the
// extension as the install page names it. THINKTHEN_CACHE names a fresh
// copy of recordings/thinkthen.jsonl in a folder of mode 0700. The run sets
// no key and no address, so a missed answer fails with no request sent. A
// sample passes when it exits 0 and prints its .sql.out byte for byte.
// narrow() in binding-proofs.mjs finds the answers it read.
//
// SQLite needs the pinned 3.50.0 CLI that databases/sqlite/setup.sh builds
// into ~/.cache/thinkthen-toolchains/sqlite-3500000-host, and Rust with the
// offline Cargo cache. The CLI runs in its default list mode. DuckDB needs
// the pinned 1.5.5 CLI, source and static archives that
// databases/duckdb/tools/setup.sh --fetch puts in
// ~/.cache/thinkthen-toolchains/duckdb/v1.5.5, CMake, a C++ compiler and
// Rust. cpp/build.sh builds the extension offline, and the CLI runs as
// duckdb -unsigned -list. A database
// whose toolchain is missing reports "not run" and keeps its old entries,
// and the run exits 1 unless --allow-missing is given.

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { readReplayList, replayLine, sampleHashes, sourceTree, PROOF_FILE, fixtureLines, narrow, leakedVariables, sampleSurface, samplePage, cargoFolders, sha256 } from './binding-proofs.mjs';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const repo = path.resolve(site, '..');
const examples = path.join(site, 'examples');
const only = process.argv.slice(2).filter((a) => !a.startsWith('--'));
const update = process.argv.includes('--update');
const allowMissing = process.argv.includes('--allow-missing');

// No key, address, backend or setting from the shell may reach a sample.
const leaked = leakedVariables(process.env);
if (leaked.length) {
  console.error(`smoke-sql: unset ${leaked.join(', ')} first. The replay run sends nothing and reads no setting from the shell.`);
  process.exit(2);
}

const run = (cmd, args, opts = {}) => spawnSync(cmd, args, { encoding: 'utf8', maxBuffer: 1 << 26, ...opts });
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'thinkthen-sql-'));
const TOOLCHAINS = path.join(os.homedir(), '.cache', 'thinkthen-toolchains');

// Each database: the folder its extension builds from, the Cargo manifest
// of its Rust part, and build(), which returns the toolchain versions and
// how to run a sample in a folder, or the tool that is missing.
const DATABASES = {
  sqlite: {
    folder: 'databases/sqlite',
    manifest: 'databases/sqlite/Cargo.toml',
    build() {
      const host = path.join(TOOLCHAINS, 'sqlite-3500000-host');
      const cli = path.join(host, 'sqlite3');
      if (!fs.existsSync(cli)) return { missing: 'the pinned SQLite 3.50.0 CLI that databases/sqlite/setup.sh builds' };
      if (run('sh', ['-c', 'command -v cargo']).status !== 0) return { missing: 'cargo' };
      const target = path.join(repo, 'target', 'site-sqlite');
      const done = run('cargo', ['build', '--quiet', '--release', '--locked', '--offline'], { cwd: path.join(repo, this.folder), env: { ...process.env, CARGO_TARGET_DIR: target } });
      if (done.status !== 0) throw new Error(`cargo build failed in ${this.folder}\n${done.stdout}${done.stderr}`);
      const library = path.join(target, 'release', 'libthinkthen0.so');
      const env = { LD_LIBRARY_PATH: host };
      return {
        toolchain: [`SQLite ${run(cli, ['--version'], { env }).stdout.split(' ')[0]}`, run('rustc', ['--version']).stdout.trim()],
        // The install page loads the library as ./thinkthen.
        lay: (dir) => fs.copyFileSync(library, path.join(dir, 'thinkthen.so')),
        command: (sample) => [cli, ['-batch'], fs.readFileSync(sample, 'utf8')],
        env,
      };
    },
  },
  duckdb: {
    folder: 'databases/duckdb',
    manifest: 'databases/duckdb/bridge/Cargo.toml',
    build() {
      const tools = path.join(TOOLCHAINS, 'duckdb', 'v1.5.5');
      const cli = path.join(tools, 'duckdb');
      if (!fs.existsSync(cli)) return { missing: 'the pinned DuckDB 1.5.5 CLI that databases/duckdb/tools/setup.sh fetches' };
      if (!fs.existsSync(path.join(tools, 'static-libs', 'libduckdb_static.a'))) return { missing: 'the pinned DuckDB source and static archives that databases/duckdb/tools/setup.sh --fetch fetches' };
      for (const tool of ['cargo', 'cmake']) if (run('sh', ['-c', `command -v ${tool}`]).status !== 0) return { missing: tool };
      const target = path.join(repo, 'target', 'site-duckdb');
      const done = run('sh', [path.join(repo, this.folder, 'cpp', 'build.sh')], { cwd: repo, env: { ...process.env, THINKTHEN_DUCKDB_CPP_BUILD: path.join(target, 'cpp'), CARGO_TARGET_DIR: path.join(target, 'bridge') } });
      if (done.status !== 0) throw new Error(`cpp/build.sh failed in ${this.folder}\n${done.stdout}${done.stderr}`);
      const extension = path.join(target, 'cpp', 'extension', 'thinkthen', 'thinkthen.duckdb_extension');
      return {
        toolchain: [`DuckDB ${run(cli, ['--version']).stdout.split(' ')[0]}`, run('cmake', ['--version']).stdout.split('\n')[0], run('rustc', ['--version']).stdout.trim()],
        // The install page loads the extension as ./thinkthen.duckdb_extension.
        lay: (dir) => fs.copyFileSync(extension, path.join(dir, 'thinkthen.duckdb_extension')),
        command: (sample) => [cli, ['-unsigned', '-list'], fs.readFileSync(sample, 'utf8')],
        env: {},
      };
    },
  },
};

const list = readReplayList(examples).filter((line) => replayLine(line).rel.endsWith('.sql')).filter((line) => !only.length || only.some((o) => line.includes(o)));
const proofPath = path.join(examples, PROOF_FILE);
const proof = fs.existsSync(proofPath) ? JSON.parse(fs.readFileSync(proofPath, 'utf8')) : {};
const fixture = fixtureLines(path.join(site, 'recordings', 'thinkthen.jsonl'));
const states = fixture.filter((l) => !l.key);
const built = new Map();
const failed = [];
const notRun = [];
let proved = 0;

// One run of a sample over the given answers.
function attempt(rel, db, answers) {
  const work = fs.mkdtempSync(path.join(tmp, 'run-'));
  const cache = path.join(work, 'cache');
  const cwd = path.join(work, 'work');
  fs.mkdirSync(cache, { mode: 0o700 });
  const files = path.join(examples, path.dirname(rel), 'files');
  if (fs.existsSync(files)) fs.cpSync(files, cwd, { recursive: true });
  else fs.mkdirSync(cwd);
  db.lay(cwd);
  fs.writeFileSync(path.join(cache, 'thinkthen.jsonl'), [...states, ...answers].map((l) => l.text).join('\n') + '\n');
  const [cmd, args, input] = db.command(path.join(examples, rel));
  const env = {
    PATH: process.env.PATH, LC_ALL: 'C.UTF-8', HOME: work,
    XDG_CACHE_HOME: path.join(work, 'xdg-cache'), XDG_CONFIG_HOME: path.join(work, 'xdg-config'),
    THINKTHEN_CACHE: cache, ...db.env,
  };
  const done = run(cmd, args, { cwd, env, input });
  fs.rmSync(work, { recursive: true, force: true });
  return { ok: done.status === 0 && !done.stderr, stdout: done.stdout, output: `${done.stdout}${done.stderr}`.trim() };
}

for (const line of list) {
  const { rel, backend } = replayLine(line);
  if (backend) { failed.push(`${line}: a SQL sample takes no backend.`); continue; }
  const surface = sampleSurface(rel);
  const spec = DATABASES[surface];
  if (!spec) { failed.push(`${line}: smoke-sql has no ${surface} samples.`); continue; }
  if (!built.has(surface)) {
    try {
      built.set(surface, spec.build());
    } catch (error) {
      built.set(surface, { error: error.message });
    }
  }
  const db = built.get(surface);
  if (db.missing) { notRun.push(`${line}: not run: ${db.missing} is missing`); continue; }
  if (db.error) { failed.push(`${line}: the ${surface} extension did not build\n${db.error}`); continue; }

  const outPath = path.join(examples, `${rel}.out`);
  const whole = attempt(rel, db, fixture.filter((l) => l.key));
  if (!whole.ok) { failed.push(`${line}: failed against the whole recording\n${whole.output}`); continue; }
  if (update) fs.writeFileSync(outPath, whole.stdout);
  const saved = fs.existsSync(outPath) ? fs.readFileSync(outPath, 'utf8') : null;
  if (saved === null) { failed.push(`${line}: ${rel}.out is missing. Run node scripts/smoke-sql.mjs --update ${rel}.`); continue; }
  if (whole.stdout !== saved) { failed.push(`${line}: printed other output than ${rel}.out\n--- saved\n${saved}--- printed\n${whole.stdout}`); continue; }

  const passes = (answers) => {
    const one = attempt(rel, db, answers);
    return one.ok && one.stdout === saved;
  };
  const narrowed = narrow(fixture, fs.readFileSync(path.join(examples, rel), 'utf8'), passes);
  if (narrowed.reason) { failed.push(`${line}: ${narrowed.reason}`); continue; }
  const { kept } = narrowed;

  const folders = [...new Set([spec.folder, ...cargoFolders(repo, spec.manifest)])].sort();
  proof[line] = {
    page: samplePage(rel),
    ...sampleHashes(examples, rel),
    answers: Object.fromEntries(kept.map((l) => [l.key, sha256(l.text)]).sort()),
    sources: { folders, tree: sourceTree(repo, folders) },
    toolchain: db.toolchain,
  };
  proved += 1;
  console.log(`smoke-sql: ${line} passed, reading ${kept.length} recorded answers at ${[...new Set(kept.map((l) => l.url))].join(', ')}.`);
}
fs.rmSync(tmp, { recursive: true, force: true });

const ordered = Object.fromEntries(Object.keys(proof).sort().map((k) => [k, proof[k]]));
fs.writeFileSync(proofPath, JSON.stringify(ordered, null, 2) + '\n');
for (const line of notRun) console.log(`smoke-sql: ${line}`);
if (failed.length) {
  console.error(`smoke-sql: ${failed.length} samples failed\n\n${failed.join('\n\n')}`);
  process.exit(1);
}
if (notRun.length && !allowMissing) {
  console.error(`smoke-sql: ${notRun.length} samples did not run. Install their toolchains, or pass --allow-missing to keep their old entries.`);
  process.exit(1);
}
if (notRun.length) console.log(`smoke-sql: ${notRun.length} samples did not run, and --allow-missing lets the run pass.`);
const tools = [...built.values()].filter((b) => b.toolchain).flatMap((b) => b.toolchain);
console.log(`smoke-sql: ${proved} samples replayed.${tools.length ? ` Toolchains: ${[...new Set(tools)].join('; ')}.` : ''}`);
