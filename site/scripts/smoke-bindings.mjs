#!/usr/bin/env node
// Replay each library sample that examples/REPLAY names against its
// binding, built from this working tree, and write what each run proved to
// examples/bindings-proof.json. scripts/check-binding-proofs.mjs reads that
// file in every build.
//
//   node scripts/smoke-bindings.mjs            replay every listed sample
//   node scripts/smoke-bindings.mjs PATTERN    only the samples whose path holds PATTERN
//
// A sample runs in a fresh folder that starts with a copy of its page's
// files/. THINKTHEN_CACHE names a fresh copy of recordings/thinkthen.jsonl.
// The run sets no key and no address, so a missed answer fails with no
// request sent. A sample passes when it exits 0, so every assert held.
//
// The store loads the whole fixture, so it cannot say which answers a
// sample read. The runner finds them in three steps. It keeps the answers
// whose question holds one of the sample's string literals of 12
// characters or more, and the sample must pass with only those. It then
// drops each kept answer in turn and runs the sample again. An answer
// whose loss fails the sample is one the sample read. The final set must
// pass on its own.
//
// A binding whose toolchain is missing reports "not run" and keeps its old
// entry. The run needs Rust with the offline Cargo cache, and each
// binding's own toolchain: a stable Python 3.12 or later with uv and maturin,
// R 4.2 or later with dplyr, g++, gcc with Objective-C, GnuCOBOL, and GNAT.
// The C++, Objective-C, COBOL and Ada samples link the C door that
// sdlc/scripts/installed.sh lays out, and the runner compiles each sample
// once.

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { nativeBuild } from '../src/data/native-builds.mjs';
import { readReplayList, sampleHashes, sourceTree, sha256, PROOF_FILE, fixtureLines } from './binding-proofs.mjs';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const repo = path.resolve(site, '..');
const examples = path.join(site, 'examples');
const only = process.argv.slice(2).filter((a) => !a.startsWith('--'));

// No key, address, backend or setting from the shell may reach a sample.
const leaked = Object.keys(process.env).filter((n) => n.startsWith('THINKTHEN_') || ['TYPESAFE_API_KEY', 'LIQUIDAI_API_KEY', 'LIQUID_API_KEY', 'OLLAMA_API_KEY'].includes(n));
if (leaked.length) {
  console.error(`smoke-bindings: unset ${leaked.join(', ')} first. The replay run sends nothing and reads no setting from the shell.`);
  process.exit(2);
}

const run = (cmd, args, opts = {}) => spawnSync(cmd, args, { encoding: 'utf8', maxBuffer: 1 << 26, ...opts });
const has = (cmd) => run('sh', ['-c', `command -v ${cmd}`]).status === 0;
const firstLine = (cmd, args) => (run(cmd, args).stdout || '').split('\n')[0].trim();

const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'thinkthen-bindings-'));
const home = path.join(tmp, 'home');
fs.mkdirSync(home);

// The Cargo workspace members a binding builds on, from cargo metadata.
function cargoFolders(manifest) {
  const done = run('cargo', ['metadata', '--format-version', '1', '--locked', '--offline', '--manifest-path', path.join(repo, manifest)]);
  if (done.status !== 0) throw new Error(`cargo metadata failed for ${manifest}\n${done.stderr}`);
  return JSON.parse(done.stdout).packages
    .filter((p) => p.source === null)
    .map((p) => path.relative(repo, path.dirname(p.manifest_path)));
}

// The C door, built once and laid out as an install would.
let door = null;
function cDoor() {
  if (door) return door;
  const out = path.join(tmp, 'c-door');
  const done = run('sh', ['-c', '. sdlc/scripts/installed.sh && native_install "$1" "$2"', 'sh', repo, out], { cwd: repo });
  if (done.status !== 0) throw new Error(`native_install failed\n${done.stdout}${done.stderr}`);
  door = out;
  return door;
}

// A binding over the C door. The runner lays out the folder a reader
// would have, with thinkthen-c/ and thinkthen-<slug>/ beside the sample,
// and runs the build lines the install page shows. Each sample compiles
// once.
function native(slug, tools, versions) {
  return {
    folder: `libraries/${slug}`,
    manifest: 'libraries/c/Cargo.toml',
    build() {
      for (const tool of [...tools, 'cargo']) if (!has(tool)) return { missing: tool };
      const n = cDoor();
      const programs = new Map();
      const program = (rel) => {
        if (programs.has(rel)) return programs.get(rel);
        const dir = path.join(tmp, 'programs', rel);
        fs.mkdirSync(dir, { recursive: true });
        fs.symlinkSync(n, path.join(dir, 'thinkthen-c'));
        fs.cpSync(path.join(repo, 'libraries', slug), path.join(dir, `thinkthen-${slug}`), { recursive: true });
        fs.copyFileSync(path.join(examples, rel), path.join(dir, path.basename(rel)));
        const build = nativeBuild(slug, path.basename(rel));
        for (const line of build.lines) {
          const done = run('sh', ['-c', line.join(' ')], { cwd: dir });
          if (done.status !== 0) throw new Error(`${rel} did not build\n${line.join(' ')}\n${done.stdout}${done.stderr}`);
        }
        programs.set(rel, path.join(dir, build.program));
        return programs.get(rel);
      };
      return {
        toolchain: [...versions(), firstLine('rustc', ['--version'])],
        prepare: program,
        command: (file, rel) => [program(rel), []],
        env: {},
      };
    },
  };
}

// Each binding: how to build it once, how to run one sample, and what it
// builds on. A build returns null when a tool is missing.
const BINDINGS = {
  python: {
    folder: 'libraries/python',
    manifest: 'libraries/python/Cargo.toml',
    build() {
      const python = ['python3.14', 'python3.13', 'python3.12', 'python3'].map((p) => run('sh', ['-c', `command -v ${p}`]).stdout.trim())
        .find((p) => p && run(p, ['-c', 'import sys; sys.exit(sys.version_info < (3, 12) or sys.version_info.releaselevel != "final")']).status === 0);
      if (!python) return { missing: 'a stable Python 3.12 or later' };
      for (const tool of ['uv', 'maturin', 'cargo']) if (!has(tool)) return { missing: tool };
      const out = path.join(tmp, 'python');
      const step = (cmd, args, opts) => {
        const done = run(cmd, args, opts);
        if (done.status !== 0) throw new Error(`${cmd} ${args.join(' ')} failed\n${done.stdout}${done.stderr}`);
      };
      step('maturin', ['build', '--quiet', '--locked', '--offline', '-o', path.join(out, 'wheel')], { cwd: path.join(repo, 'libraries/python'), env: { ...process.env, PYO3_PYTHON: python } });
      step('uv', ['venv', '--quiet', '--offline', '--python', python, path.join(out, 'venv')]);
      const venvPython = path.join(out, 'venv', 'bin', 'python');
      // pandas and what it needs, at the gate's pins.
      const pins = fs.readFileSync(path.join(repo, 'libraries/python/requirements-dev.txt'), 'utf8').split('\n')
        .map((l) => l.match(/^(pandas|numpy|python-dateutil|six)==(\S+)/)).filter(Boolean).map((m) => `${m[1]}==${m[2]}`);
      step('uv', ['pip', 'install', '--quiet', '--offline', '--python', venvPython, ...pins]);
      const wheel = fs.readdirSync(path.join(out, 'wheel')).find((n) => n.endsWith('.whl'));
      step('uv', ['pip', 'install', '--quiet', '--offline', '--no-deps', '--python', venvPython, path.join(out, 'wheel', wheel)]);
      const versions = run(venvPython, ['-c', 'import sys, pandas; print(f"Python {sys.version.split()[0]}, pandas {pandas.__version__}")']).stdout.trim();
      return {
        toolchain: [versions, firstLine('maturin', ['--version']), firstLine('rustc', ['--version'])],
        command: (file) => [venvPython, [file]],
        env: {},
      };
    },
  },
  r: {
    folder: 'libraries/r',
    manifest: 'libraries/r/thinkthen/src/rust/Cargo.toml',
    build() {
      for (const tool of ['R', 'Rscript', 'cargo']) if (!has(tool)) return { missing: tool };
      const lib = path.join(tmp, 'rlib');
      fs.mkdirSync(lib);
      const user = run('Rscript', ['-e', 'cat(path.expand(strsplit(Sys.getenv("R_LIBS_USER"), ":")[[1]]), sep = "\\n")']).stdout.split('\n');
      const libs = [lib, path.join(os.homedir(), '.cache/thinkthen-toolchains/r-library'), ...user].filter((l) => l && fs.existsSync(l)).join(':');
      const ready = run('Rscript', ['-e', 'if (!requireNamespace("dplyr", quietly = TRUE)) quit(status = 1)'], { env: { ...process.env, R_LIBS: libs } });
      if (ready.status !== 0) return { missing: 'the R package dplyr' };
      const done = run('R', ['CMD', 'INSTALL', '-l', lib, 'thinkthen'], { cwd: path.join(repo, 'libraries/r'), env: { ...process.env, R_LIBS: libs, CARGO_TARGET_DIR: path.join(repo, 'target/r') } });
      if (done.status !== 0) throw new Error(`R CMD INSTALL failed\n${done.stdout}${done.stderr}`);
      const dplyr = run('Rscript', ['-e', 'cat(format(packageVersion("dplyr")))'], { env: { ...process.env, R_LIBS: libs } }).stdout.trim();
      return {
        toolchain: [`${firstLine('R', ['--version'])}, dplyr ${dplyr}`, firstLine('rustc', ['--version'])],
        command: (file) => ['Rscript', ['--vanilla', file]],
        env: { R_LIBS: libs },
      };
    },
  },
  cpp: native('cpp', ['c++'], () => [firstLine('c++', ['--version'])]),
  'objective-c': native('objective-c', ['gcc'], () => [firstLine('gcc', ['--version'])]),
  cobol: native('cobol', ['cobc'], () => [firstLine('cobc', ['--version'])]),
  ada: native('ada', ['gnatmake'], () => [firstLine('gnatmake', ['--version'])]),
};

// Which binding runs each page's samples.
const PAGE_BINDING = { python: 'python', pandas: 'python', r: 'r', cpp: 'cpp', 'objective-c': 'objective-c', cobol: 'cobol', ada: 'ada' };

const list = readReplayList(examples).filter((rel) => !only.length || only.some((o) => rel.includes(o)));
const proofPath = path.join(examples, PROOF_FILE);
const proof = fs.existsSync(proofPath) ? JSON.parse(fs.readFileSync(proofPath, 'utf8')) : {};
const fixture = fixtureLines(path.join(site, 'recordings', 'thinkthen.jsonl'));
const states = fixture.filter((l) => !l.key);
const built = new Map();
const failed = [];
const notRun = [];
let proved = 0;

// One run of a sample over the given answers. True when it exits 0.
function attempt(rel, binding, answers, keep) {
  const work = fs.mkdtempSync(path.join(tmp, 'run-'));
  const cache = path.join(work, 'cache');
  const cwd = path.join(work, 'work');
  fs.mkdirSync(cache);
  const files = path.join(examples, path.dirname(rel), 'files');
  if (fs.existsSync(files)) fs.cpSync(files, cwd, { recursive: true });
  else fs.mkdirSync(cwd);
  fs.copyFileSync(path.join(examples, rel), path.join(cwd, path.basename(rel)));
  fs.writeFileSync(path.join(cache, 'thinkthen.jsonl'), [...states, ...answers].map((l) => l.text).join('\n') + '\n');
  const [cmd, args] = binding.command(path.basename(rel), rel);
  const env = {
    PATH: process.env.PATH, LC_ALL: 'C.UTF-8', HOME: home,
    XDG_CACHE_HOME: path.join(work, 'xdg-cache'), XDG_CONFIG_HOME: path.join(work, 'xdg-config'),
    THINKTHEN_CACHE: cache, ...binding.env,
  };
  const done = run(cmd, args, { cwd, env });
  if (!keep) fs.rmSync(work, { recursive: true, force: true });
  return { ok: done.status === 0, output: `${done.stdout}${done.stderr}`.trim() };
}

for (const rel of list) {
  const page = rel.split('/')[1];
  const name = PAGE_BINDING[page];
  if (!name) { failed.push(`${rel}: no binding runs the ${page} page. Add it to PAGE_BINDING.`); continue; }
  const spec = BINDINGS[name];
  if (!built.has(name)) {
    try {
      built.set(name, spec.build());
    } catch (error) {
      built.set(name, { error: error.message });
    }
  }
  const binding = built.get(name);
  if (binding.missing) { notRun.push(`${rel}: not run, no ${binding.missing}`); continue; }
  if (binding.error) { failed.push(`${rel}: the ${name} binding did not build\n${binding.error}`); continue; }

  try {
    binding.prepare?.(rel);
  } catch (error) {
    failed.push(`${rel}: ${error.message}`);
    continue;
  }
  const whole = attempt(rel, binding, fixture.filter((l) => l.key));
  if (!whole.ok) { failed.push(`${rel}: failed against the whole recording\n${whole.output}`); continue; }

  const text = fs.readFileSync(path.join(examples, rel), 'utf8');
  const literals = [...text.matchAll(/"((?:[^"\\\n]|\\.){12,})"|'((?:[^'\\\n]|\\.){12,})'/g)].map((m) => (m[1] ?? m[2]).replace(/\\n/g, '\n').replace(/\\(.)/g, '$1'));
  let kept = fixture.filter((l) => l.key && literals.some((x) => l.question.includes(x)));
  if (!attempt(rel, binding, kept).ok) { failed.push(`${rel}: the answers whose question holds one of its literals do not answer it. The runner cannot tell which answers it read.`); continue; }
  for (const line of [...kept]) {
    const without = kept.filter((l) => l !== line);
    if (attempt(rel, binding, without).ok) kept = without;
  }
  if (!kept.length || !attempt(rel, binding, kept).ok) { failed.push(`${rel}: the answers it read do not answer it on their own.`); continue; }

  const folders = [...new Set([spec.folder, ...cargoFolders(spec.manifest)])].sort();
  proof[rel] = {
    page: `/install/${page}/`,
    ...sampleHashes(examples, rel),
    answers: Object.fromEntries(kept.map((l) => [l.key, sha256(l.text)]).sort()),
    sources: { folders, tree: sourceTree(repo, folders) },
    toolchain: binding.toolchain,
  };
  proved += 1;
  console.log(`smoke-bindings: ${rel} passed, reading ${kept.length} recorded answers.`);
}
fs.rmSync(tmp, { recursive: true, force: true });

const ordered = Object.fromEntries(Object.keys(proof).sort().map((k) => [k, proof[k]]));
fs.writeFileSync(proofPath, JSON.stringify(ordered, null, 2) + '\n');
for (const line of notRun) console.log(`smoke-bindings: ${line}`);
if (failed.length) {
  console.error(`smoke-bindings: ${failed.length} samples failed\n\n${failed.join('\n\n')}`);
  process.exit(1);
}
const tools = [...built.values()].filter((b) => b.toolchain).flatMap((b) => b.toolchain);
console.log(`smoke-bindings: ${proved} samples replayed. Toolchains: ${[...new Set(tools)].join('; ')}.`);
