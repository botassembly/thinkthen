#!/usr/bin/env node
// Replay each listed library sample and compare its output with <sample>.out.
// Runs use saved recordings in fresh folders, with no key or network request.
// Pass a path pattern to select samples, --update to save expected output,
// or --allow-missing to skip missing toolchains.

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { buildLines } from '../src/data/build-lines.mjs';
import { BACKEND_ROUTES } from '../src/data/catalog.mjs';
import { readReplayList, replayLine, fixtureLines, leakedVariables, sampleSurface, sampleFiles } from './binding-samples.mjs';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const repo = path.resolve(site, '..');
const examples = path.join(site, 'examples');
const only = process.argv.slice(2).filter((a) => !a.startsWith('--'));
const allowMissing = process.argv.includes('--allow-missing');
const update = process.argv.includes('--update');

// No key, address, backend or setting from the shell may reach a sample.
const leaked = leakedVariables(process.env);
if (leaked.length) {
  console.error(`smoke-bindings: unset ${leaked.join(', ')} first. The replay run sends nothing and reads no setting from the shell.`);
  process.exit(2);
}

const run = (cmd, args, opts = {}) => spawnSync(cmd, args, { encoding: 'utf8', maxBuffer: 1 << 26, ...opts });
const has = (cmd, env = process.env) => run('sh', ['-c', `command -v ${cmd}`], { env }).status === 0;
// Some tools print their version on standard error.
const firstLine = (cmd, args) => {
  const done = run(cmd, args);
  return (done.stdout || done.stderr || '').split('\n')[0].trim();
};

const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'thinkthen-bindings-'));
const home = path.join(tmp, 'home');
fs.mkdirSync(home);

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

// The JVM JARs, built once by the binding's own build script.
let jars = null;
function jvmJars() {
  if (jars) return jars;
  const out = path.join(tmp, 'jvm');
  fs.mkdirSync(out);
  const done = run('sh', [path.join(repo, 'libraries/jvm/build.sh')], { env: { ...process.env, THINKTHEN_JVM_OUT: out } });
  if (done.status !== 0) throw new Error(`the JVM build failed\n${done.stdout}${done.stderr}`);
  jars = path.join(out, 'jars');
  return jars;
}

// The .NET tools write their caches here, not under the reader's home.
const DOTNET_ENV = {
  DOTNET_CLI_HOME: path.join(tmp, 'dotnet-home'), NUGET_PACKAGES: path.join(tmp, 'nuget'),
  DOTNET_CLI_TELEMETRY_OPTOUT: '1', DOTNET_SKIP_FIRST_TIME_EXPERIENCE: '1', DOTNET_NOLOGO: '1',
};

// The C# package, packed once into a folder that serves as a feed, as the
// release archive does.
let feed = null;
function csharpFeed() {
  if (feed) return feed;
  const source = path.join(tmp, 'csharp-source');
  fs.cpSync(path.join(repo, 'libraries/csharp'), source, { recursive: true, filter: (f) => !/\/(bin|obj|target)$/.test(f) });
  const out = path.join(tmp, 'csharp-feed');
  const done = run('dotnet', ['pack', 'ThinkThen.csproj', '-c', 'Release', '-o', out, '-v', 'quiet'], { cwd: source, env: { ...process.env, ...DOTNET_ENV } });
  if (done.status !== 0) throw new Error(`dotnet pack failed\n${done.stdout}${done.stderr}`);
  feed = out;
  return feed;
}

// What each binding's archive holds, laid out in the build folder.
const sourceArchive = (slug) => (dir) => fs.cpSync(path.join(repo, 'libraries', slug), path.join(dir, `thinkthen-${slug}`), { recursive: true });
const ARCHIVE = {
  java: (dir) => fs.cpSync(jvmJars(), path.join(dir, 'thinkthen-jvm'), { recursive: true }),
  csharp: (dir) => fs.cpSync(csharpFeed(), path.join(dir, 'thinkthen-csharp'), { recursive: true }),
};
// The Swift archive carries the C header in its system library folder.
ARCHIVE.swift = (dir) => {
  sourceArchive('swift')(dir);
  fs.mkdirSync(path.join(dir, 'thinkthen-swift/Sources/CThinkThen/include'), { recursive: true });
  fs.copyFileSync(path.join(cDoor(), 'include', 'thinkthen.h'), path.join(dir, 'thinkthen-swift/Sources/CThinkThen/include/thinkthen.h'));
};
// C needs only the C archive. A Rust project takes the crate from this
// working tree through a Cargo patch, with the locked versions.
ARCHIVE.c = () => {};
ARCHIVE.rust = (dir) => {
  fs.mkdirSync(path.join(dir, '.cargo'));
  fs.writeFileSync(path.join(dir, '.cargo/config.toml'), `[patch.crates-io]\nthinkthen = { path = ${JSON.stringify(path.join(repo, 'crates/thinkthen'))} }\n\n[net]\noffline = true\n`);
  fs.copyFileSync(path.join(repo, 'Cargo.lock'), path.join(dir, 'Cargo.lock'));
};
// Cargo patches apply only within the requirement. The page names the
// published release, so only its scratch manifest follows the working tree.
function rustRequirement(dir) {
  const version = fs.readFileSync(path.join(repo, 'crates/thinkthen/Cargo.toml'), 'utf8').match(/^version = "(\d+\.\d+)\.\d+"/m);
  if (!version) throw new Error('the working crate has no release version');
  const manifest = path.join(dir, 'Cargo.toml');
  const text = fs.readFileSync(manifest, 'utf8');
  if (!/^thinkthen = "[^"\n]+"$/m.test(text)) throw new Error('the Rust sample has no thinkthen requirement');
  fs.writeFileSync(manifest, text.replace(/^thinkthen = "[^"\n]+"$/m, `thinkthen = "${version[1]}"`));
}
// The function pages show C and Rust as fragments with no main. The runner
// puts each one inside main, as a reader would. C keeps its #include lines
// on top. Rust builds with the Rust install page's Cargo.toml.
const declaresMain = (code) => /^\s*(int|void|fn)\s+main\s*\(/m.test(code);
const FRAGMENT = {
  c(dir, name) {
    const file = path.join(dir, name);
    const lines = fs.readFileSync(file, 'utf8').trimEnd().split('\n');
    if (declaresMain(lines.join('\n'))) throw new Error('declares its own main. The runner wraps each function page fragment in main.');
    const cut = lines.findIndex((l) => l.trim() && !l.startsWith('#include'));
    fs.writeFileSync(file, [...lines.slice(0, cut), 'int main(void) {', ...lines.slice(cut), 'return 0;', '}', ''].join('\n'));
  },
  rust(dir, name) {
    const file = path.join(dir, name);
    const code = fs.readFileSync(file, 'utf8').trimEnd();
    if (declaresMain(code)) throw new Error('declares its own main. The runner wraps each function page fragment in main.');
    fs.writeFileSync(file, `fn main() -> Result<(), Box<dyn std::error::Error>> {\n${code}\nOk(())\n}\n`);
    fs.copyFileSync(path.join(examples, 'install/rust/files/Cargo.toml'), path.join(dir, 'Cargo.toml'));
  },
};
// C# runs a function page sample as top-level statements, in the project
// the C# install page shows.
FRAGMENT.csharp = (dir) => fs.copyFileSync(path.join(examples, 'install/csharp/files/first-call.csproj'), path.join(dir, 'first-call.csproj'));
// A Java, Kotlin or Scala sample on a function page names its class or its
// main for the function, such as Rank, so it builds under that name.
const JVM = new Set(['java', 'kotlin', 'scala']);
const programName = (rel) => `${rel.split('/')[1].replace(/(^|-)(.)/g, (m, dash, c) => c.toUpperCase())}${path.extname(rel)}`;
// A function page's sample builds as a reader would build it in a project
// of its own. Each entry lays out the install page's project files and
// returns the name the sample builds under. Swift and Zig take the install
// page's project files, which name the first call. Dart takes the install
// page's pubspec.yaml. Every other sample builds as sample.<ext>, which
// also keeps Go from naming its module after the language.
const projectFiles = (slug, file) => (dir) => {
  fs.cpSync(path.join(examples, 'install', slug, 'files'), dir, { recursive: true });
  return file;
};
const PROJECT = {
  swift: projectFiles('swift', 'main.swift'),
  zig: projectFiles('zig', 'first-call.zig'),
  dart: (dir, rel) => {
    fs.copyFileSync(path.join(examples, 'install/dart/files/pubspec.yaml'), path.join(dir, 'pubspec.yaml'));
    return runName(rel);
  },
};
// The name a built sample takes: the JVM program name, then the project's
// name, then the name every sample runs under.
const buildName = (slug, rel, dir) => {
  if (!rel.startsWith('functions/')) return path.basename(rel);
  if (JVM.has(slug)) return programName(rel);
  return PROJECT[slug] ? PROJECT[slug](dir, rel) : runName(rel);
};
ARCHIVE.kotlin = ARCHIVE.java;
ARCHIVE.scala = ARCHIVE.java;

// A binding with a build step. The runner lays out the folder a reader
// would have, with thinkthen-c/ and the binding's archive beside the
// sample and its files/, and runs the build lines the install page shows.
// Each sample builds once, and each attempt runs the page's run line.
function withBuild({ slug, tools, needs = () => null, versions, env = {}, buildEnv = {} }) {
  const layout = ARCHIVE[slug] ?? sourceArchive(slug);
  return {
    build() {
      const runEnv = typeof env === 'function' ? env() : env;
      for (const tool of [...tools, 'cargo']) if (!has(tool, { ...process.env, ...runEnv })) return { missing: tool };
      const missing = needs();
      if (missing) return { missing };
      const n = cDoor();
      const dirs = new Map();
      const prepare = (rel) => {
        if (dirs.has(rel)) return dirs.get(rel);
        const dir = path.join(tmp, 'programs', rel);
        const files = sampleFiles(examples, rel);
        if (fs.existsSync(files)) fs.cpSync(files, dir, { recursive: true });
        fs.mkdirSync(dir, { recursive: true });
        fs.symlinkSync(n, path.join(dir, 'thinkthen-c'));
        layout(dir);
        const name = buildName(slug, rel, dir);
        fs.copyFileSync(path.join(examples, rel), path.join(dir, name));
        if (rel.startsWith('functions/') && FRAGMENT[slug]) FRAGMENT[slug](dir, name);
        if (slug === 'rust') rustRequirement(dir);
        const lines = buildLines(slug, name, true);
        for (const line of lines.lines) {
          const done = run('sh', ['-c', line.join(' ')], { cwd: dir, env: { ...process.env, ...runEnv, ...buildEnv } });
          if (done.status !== 0) throw new Error(`${rel} did not build\n${line.join(' ')}\n${done.stdout}${done.stderr}`);
        }
        dirs.set(rel, { dir, run: lines.run.join(' ') });
        return dirs.get(rel);
      };
      return {
        toolchain: [...versions(), firstLine('rustc', ['--version'])],
        prepare,
        command: (file, rel) => ['sh', ['-c', prepare(rel).run], prepare(rel).dir],
        env: runEnv,
      };
    },
  };
}

const javaVersion = () => firstLine('java', ['-version']);
// json-c from ~/.local, where the toolchain step builds it with CMake.
const LOCAL = path.join(os.homedir(), '.local');
const JSON_C = { PKG_CONFIG_PATH: [path.join(LOCAL, 'lib/pkgconfig'), process.env.PKG_CONFIG_PATH].filter(Boolean).join(':') };
const jsonC = () => (run('pkg-config', ['--exists', 'json-c'], { env: { ...process.env, ...JSON_C } }).status === 0 ? null : 'json-c');
// The run's HOME is a scratch folder, so Cargo and rustup keep their own.
const CARGO_ENV = {
  CARGO_HOME: process.env.CARGO_HOME ?? path.join(os.homedir(), '.cargo'),
  RUSTUP_HOME: process.env.RUSTUP_HOME ?? path.join(os.homedir(), '.rustup'),
};

// A Rust build that writes only under target/<name>, as the R build does.
function cargoBuild(dir, name, env = {}) {
  const target = path.join(repo, 'target', name);
  const done = run('cargo', ['build', '--quiet', '--release', '--locked', '--offline'], { cwd: path.join(repo, dir), env: { ...process.env, ...env, CARGO_TARGET_DIR: target } });
  if (done.status !== 0) throw new Error(`cargo build failed in ${dir}\n${done.stdout}${done.stderr}`);
  return path.join(target, 'release');
}

// The repository's pinned Rust, which a project outside it would not pick.
const RUST_CHANNEL = () => fs.readFileSync(path.join(repo, 'rust-toolchain.toml'), 'utf8').match(/^channel = "(.+)"/m)[1];

// The pinned Ruby that libraries/ruby/setup-ruby.sh builds.
const RUBY_PREFIX = () => {
  const version = fs.readFileSync(path.join(repo, 'libraries/ruby/toolchain.env'), 'utf8').match(/^RUBY_VERSION=(\S+)/m)[1];
  return path.join(os.homedir(), '.cache/thinkthen-toolchains/ruby', version);
};
// Dart from the pinned Flutter, as libraries/dart/check.sh finds it. Its
// SDK folder runs no Flutter update check.
const DART_SDK = path.join(os.homedir(), '.local/opt/flutter/bin/cache/dart-sdk/bin');
const DART_PATH = { PATH: `${DART_SDK}:${process.env.PATH}` };
const SCALA_HOME = () => path.dirname(path.dirname(fs.realpathSync(run('sh', ['-c', 'command -v scalac']).stdout.trim())));

// Each binding: how to build it once, how to run one sample, and what it
// builds on. A build returns null when a tool is missing.
const BINDINGS = {
  // The npm package's files, with the addon under the name loader.js reads.
  // Each run links the package into node_modules beside the sample.
  typescript: {
    build() {
      for (const tool of ['node', 'cargo']) if (!has(tool)) return { missing: tool };
      const release = cargoBuild('libraries/typescript', 'site-typescript');
      const modules = path.join(tmp, 'typescript', 'node_modules');
      const pkg = path.join(modules, 'thinkthen');
      fs.mkdirSync(pkg, { recursive: true });
      const source = path.join(repo, 'libraries/typescript');
      const manifest = JSON.parse(fs.readFileSync(path.join(source, 'package.json'), 'utf8'));
      const assets = JSON.parse(fs.readFileSync(path.join(source, 'native-platforms.json'), 'utf8'));
      const addon = assets[`${process.platform}-${process.arch}`];
      if (!addon) throw new Error(`typescript package has no addon for ${process.platform}-${process.arch}`);
      for (const f of ['package.json', ...manifest.files.filter((f) => !f.endsWith('.node'))]) {
        fs.mkdirSync(path.dirname(path.join(pkg, f)), { recursive: true });
        fs.copyFileSync(path.join(source, f), path.join(pkg, f));
      }
      fs.copyFileSync(path.join(release, 'libthinkthen_typescript.so'), path.join(pkg, addon));
      return {
        toolchain: [`Node ${firstLine('node', ['--version'])}`, firstLine('rustc', ['--version'])],
        command: (file) => ['sh', ['-c', `ln -s "$1" node_modules && node "$2"`, 'sh', modules, file]],
        env: {},
      };
    },
  },
  // The gem's files, with the extension built for the pinned Ruby.
  ruby: {
    build() {
      const prefix = RUBY_PREFIX();
      const ruby = path.join(prefix, 'bin', 'ruby');
      if (!fs.existsSync(ruby)) return { missing: `pinned Ruby at ${prefix}` };
      if (!has('cargo')) return { missing: 'cargo' };
      const clang = run('sh', ['-c', 'for lib in /usr/lib/llvm-*/lib; do [ -e "$lib/libclang.so.1" ] && echo "$lib"; done | sort -V | tail -n 1']).stdout.trim();
      if (!clang) return { missing: 'libclang' };
      const rubyEnv = { LD_LIBRARY_PATH: path.join(prefix, 'lib') };
      const release = cargoBuild('libraries/ruby', 'site-ruby', { ...rubyEnv, RUBY: ruby, LIBCLANG_PATH: clang, PATH: `${path.join(prefix, 'bin')}:${process.env.PATH}` });
      const staged = path.join(tmp, 'ruby');
      const pkg = path.join(staged, 'libraries', 'ruby');
      const lib = path.join(pkg, 'lib');
      fs.mkdirSync(path.join(lib, 'thinkthen'), { recursive: true });
      fs.copyFileSync(path.join(release, 'libthinkthen_ruby.so'), path.join(lib, 'thinkthen', 'thinkthen.so'));
      fs.copyFileSync(path.join(repo, 'libraries/ruby/thinkthen.gemspec'), path.join(pkg, 'thinkthen.gemspec'));
      const crate = path.join(staged, 'crates', 'thinkthen');
      fs.mkdirSync(crate, { recursive: true });
      fs.copyFileSync(path.join(repo, 'crates/thinkthen/Cargo.toml'), path.join(crate, 'Cargo.toml'));
      const inventory = run(ruby, ['-rjson', '-e', 'puts JSON.generate(Gem::Specification.load(ARGV.fetch(0)).files)', path.join(pkg, 'thinkthen.gemspec')], { env: { ...process.env, ...rubyEnv } });
      if (inventory.status !== 0) throw new Error(`ruby package inventory failed\n${inventory.stderr}`);
      for (const f of JSON.parse(inventory.stdout).filter((f) => f !== 'lib/thinkthen/thinkthen.so')) {
        fs.mkdirSync(path.dirname(path.join(pkg, f)), { recursive: true });
        fs.copyFileSync(path.join(repo, 'libraries/ruby', f), path.join(pkg, f));
      }
      return {
        toolchain: [run(ruby, ['-v'], { env: { ...process.env, ...rubyEnv } }).stdout.trim(), firstLine('rustc', ['--version'])],
        command: (file) => [ruby, [file]],
        env: { ...rubyEnv, RUBYLIB: lib },
      };
    },
  },
  c: withBuild({ slug: 'c', tools: ['cc', 'pkg-config'], needs: jsonC, versions: () => [firstLine('cc', ['--version']), `json-c ${run('pkg-config', ['--modversion', 'json-c'], { env: { ...process.env, ...JSON_C } }).stdout.trim()}`], buildEnv: JSON_C, env: { LD_LIBRARY_PATH: path.join(LOCAL, 'lib') } }),
  rust: withBuild({ slug: 'rust', tools: [], versions: () => [firstLine('cargo', ['--version'])], env: { ...CARGO_ENV, RUSTUP_TOOLCHAIN: RUST_CHANNEL(), CARGO_TARGET_DIR: path.join(repo, 'target', 'site-rust') } }),
  python: {
    build() {
      const python = ['python3.14', 'python3.13', 'python3.12', 'python3'].map((p) => run('sh', ['-c', `command -v ${p}`]).stdout.trim())
        .find((p) => p && run(p, ['-c', 'import sys; sys.exit(sys.version_info < (3, 12) or sys.version_info.releaselevel != "final")']).status === 0);
      if (!python) return { missing: 'stable Python 3.12 or later' };
      for (const tool of ['uv', 'maturin', 'cargo']) if (!has(tool)) return { missing: tool };
      const out = path.join(tmp, 'python');
      const step = (cmd, args, opts) => {
        const done = run(cmd, args, opts);
        if (done.status !== 0) throw new Error(`${cmd} ${args.join(' ')} failed\n${done.stdout}${done.stderr}`);
      };
      step('maturin', ['build', '--quiet', '--locked', '--offline', '-o', path.join(out, 'wheel')], { cwd: path.join(repo, 'libraries/python'), env: { ...process.env, PYO3_PYTHON: python } });
      step('uv', ['venv', '--quiet', '--offline', '--python', python, path.join(out, 'venv')]);
      const venvPython = path.join(out, 'venv', 'bin', 'python');
      // pandas, Polars and what they need, at the gate's pins.
      const pins = fs.readFileSync(path.join(repo, 'libraries/python/requirements-dev.txt'), 'utf8').split('\n')
        .map((l) => l.match(/^(pandas|numpy|python-dateutil|six|polars|polars-runtime-32)==(\S+)/)).filter(Boolean).map((m) => `${m[1]}==${m[2]}`);
      step('uv', ['pip', 'install', '--quiet', '--offline', '--python', venvPython, ...pins]);
      const wheel = fs.readdirSync(path.join(out, 'wheel')).find((n) => n.endsWith('.whl'));
      step('uv', ['pip', 'install', '--quiet', '--offline', '--no-deps', '--python', venvPython, path.join(out, 'wheel', wheel)]);
      const versions = run(venvPython, ['-c', 'import sys, pandas, polars; print(f"Python {sys.version.split()[0]}, pandas {pandas.__version__}, Polars {polars.__version__}")']).stdout.trim();
      return {
        toolchain: [versions, firstLine('maturin', ['--version']), firstLine('rustc', ['--version'])],
        command: (file) => [venvPython, [file]],
        env: {},
      };
    },
  },
  r: {
    build() {
      for (const tool of ['R', 'Rscript', 'cargo']) if (!has(tool)) return { missing: tool };
      const lib = path.join(tmp, 'rlib');
      fs.mkdirSync(lib);
      const user = run('Rscript', ['-e', 'cat(path.expand(strsplit(Sys.getenv("R_LIBS_USER"), ":")[[1]]), sep = "\\n")']).stdout.split('\n');
      const libs = [lib, path.join(os.homedir(), '.cache/thinkthen-toolchains/r-library'), ...user].filter((l) => l && fs.existsSync(l)).join(':');
      const ready = run('Rscript', ['-e', 'if (!requireNamespace("dplyr", quietly = TRUE)) quit(status = 1)'], { env: { ...process.env, R_LIBS: libs } });
      if (ready.status !== 0) return { missing: 'R package dplyr' };
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
  cpp: withBuild({ slug: 'cpp', tools: ['c++'], versions: () => [firstLine('c++', ['--version'])] }),
  'objective-c': withBuild({ slug: 'objective-c', tools: ['gcc'], versions: () => [firstLine('gcc', ['--version'])] }),
  cobol: withBuild({ slug: 'cobol', tools: ['cobc'], versions: () => [firstLine('cobc', ['--version'])] }),
  ada: withBuild({ slug: 'ada', tools: ['gnatmake'], versions: () => [firstLine('gnatmake', ['--version'])] }),
  java: withBuild({ slug: 'java', tools: ['javac', 'java', 'kotlinc', 'scalac'], versions: () => [javaVersion()] }),
  kotlin: withBuild({ slug: 'kotlin', tools: ['javac', 'java', 'kotlinc', 'scalac'], versions: () => [javaVersion(), firstLine('kotlinc', ['-version'])] }),
  scala: withBuild({ slug: 'scala', tools: ['javac', 'java', 'kotlinc', 'scalac'], versions: () => [javaVersion(), firstLine('scalac', ['-version'])], env: () => ({ SCALA_HOME: SCALA_HOME() }) }),
  go: withBuild({ slug: 'go', tools: ['go', 'pkg-config'], versions: () => [firstLine('go', ['version'])], buildEnv: {
    GOCACHE: path.join(tmp, 'go-cache'), GOMODCACHE: path.join(tmp, 'go-mod'), GOPROXY: 'off', GOTOOLCHAIN: 'local', GOFLAGS: '-mod=mod -buildvcs=false', CGO_ENABLED: '1',
  } }),
  swift: withBuild({ slug: 'swift', tools: ['swift'], versions: () => [firstLine('swift', ['--version'])], buildEnv: { XDG_CACHE_HOME: path.join(tmp, 'swift-cache') } }),
  zig: withBuild({ slug: 'zig', tools: ['zig'], versions: () => [`Zig ${firstLine('zig', ['version'])}`], buildEnv: { ZIG_GLOBAL_CACHE_DIR: path.join(tmp, 'zig-cache') } }),
  php: withBuild({ slug: 'php', tools: ['php'], versions: () => [firstLine('php', ['--version'])] }),
  dart: withBuild({ slug: 'dart', tools: ['dart'], versions: () => [run('dart', ['--version'], { env: { ...process.env, ...DART_PATH } }).stdout.trim()], env: DART_PATH, buildEnv: {
    HOME: home, PUB_CACHE: process.env.PUB_CACHE ?? path.join(os.homedir(), '.pub-cache'),
  } }),
  csharp: withBuild({ slug: 'csharp', tools: ['dotnet'], versions: () => [`.NET SDK ${firstLine('dotnet', ['--version'])}`], buildEnv: DOTNET_ENV }),
};

// Which binding runs each page's samples.
const PAGE_BINDING = Object.fromEntries(Object.keys(BINDINGS).map((b) => [b, b]));
PAGE_BINDING.pandas = 'python';
PAGE_BINDING.polars = 'python';

const list = readReplayList(examples).filter((line) => !replayLine(line).rel.endsWith('.sql')).filter((rel) => !only.length || only.some((o) => rel.includes(o)));
const fixture = fixtureLines(path.join(site, 'recordings', 'thinkthen.jsonl'));
const states = fixture.filter((l) => !l.key);
const built = new Map();
const failed = [];
const notRun = [];
let proved = 0;

// The name a sample runs under.
const runName = (rel) => (rel.startsWith('functions/') ? `sample${path.extname(rel)}` : path.basename(rel));

// One run of a sample over the given answers. ok is true when it exits 0.
function attempt(rel, binding, answers, named = {}, keep = false) {
  const work = fs.mkdtempSync(path.join(tmp, 'run-'));
  const cache = path.join(work, 'cache');
  const cwd = path.join(work, 'work');
  fs.mkdirSync(cache);
  const files = sampleFiles(examples, rel);
  if (fs.existsSync(files)) fs.cpSync(files, cwd, { recursive: true });
  else fs.mkdirSync(cwd);
  fs.copyFileSync(path.join(examples, rel), path.join(cwd, runName(rel)));
  fs.writeFileSync(path.join(cache, 'thinkthen.jsonl'), [...states, ...answers].map((l) => l.text).join('\n') + '\n');
  const [cmd, args, dir] = binding.command(runName(rel), rel);
  const env = {
    PATH: process.env.PATH, LC_ALL: 'C.UTF-8', HOME: home,
    XDG_CACHE_HOME: path.join(work, 'xdg-cache'), XDG_CONFIG_HOME: path.join(work, 'xdg-config'),
    THINKTHEN_CACHE: cache, ...binding.env, ...named,
  };
  const done = run(cmd, args, { cwd: dir ?? cwd, env });
  if (!keep) fs.rmSync(work, { recursive: true, force: true });
  return { ok: done.status === 0, stdout: done.stdout, output: `${done.stdout}${done.stderr}`.trim() };
}

for (const line of list) {
  const { rel, backend } = replayLine(line);
  const route = backend && BACKEND_ROUTES.find((r) => r.name === backend);
  if (backend && !route) { failed.push(`${line}: ${backend} is not one of ${BACKEND_ROUTES.map((r) => r.name).join(', ')}.`); continue; }
  const named = route ? { THINKTHEN_BACKEND: route.name, ...(route.address ? { THINKTHEN_BASE_URL: route.address } : {}) } : {};
  const page = sampleSurface(rel);
  const name = PAGE_BINDING[page];
  if (!name) { failed.push(`${line}: no binding runs the ${page} page. Add it to PAGE_BINDING.`); continue; }
  const spec = BINDINGS[name];
  if (!built.has(name)) {
    try {
      built.set(name, spec.build());
    } catch (error) {
      built.set(name, { error: error.message });
    }
  }
  const binding = built.get(name);
  if (binding.missing) { notRun.push(`${line}: not run: no ${binding.missing}`); continue; }
  if (binding.error) { failed.push(`${line}: the ${name} binding did not build\n${binding.error}`); continue; }

  try {
    binding.prepare?.(rel);
  } catch (error) {
    failed.push(`${line}: ${error.message}`);
    continue;
  }
  const answers = fixture.filter((l) => l.key && (!route || l.url?.startsWith(`${route.base}/`)));
  const whole = attempt(rel, binding, answers, named);
  if (!whole.ok) { failed.push(`${line}: failed against the whole recording\n${whole.output}`); continue; }
  const saved = path.join(examples, `${rel}.out`);
  if (update && whole.stdout) fs.writeFileSync(saved, whole.stdout);
  if (update && !whole.stdout) fs.rmSync(saved, { force: true });
  const expected = fs.existsSync(saved) ? fs.readFileSync(saved, 'utf8') : '';
  if (whole.stdout && !fs.existsSync(saved)) { failed.push(`${line}: printed output, and examples/${rel}.out does not exist. Run node scripts/smoke-bindings.mjs --update ${rel}, then read the file.\n${whole.stdout}`); continue; }
  if (whole.stdout !== expected) { failed.push(`${line}: printed other output than examples/${rel}.out\n--- saved\n${expected}--- printed\n${whole.stdout}`); continue; }

  proved += 1;
  console.log(`smoke-bindings: ${line} passed.`);
}
fs.rmSync(tmp, { recursive: true, force: true });

for (const line of notRun) console.log(`smoke-bindings: ${line}`);
if (failed.length) {
  console.error(`smoke-bindings: ${failed.length} samples failed\n\n${failed.join('\n\n')}`);
  process.exit(1);
}
if (notRun.length && !allowMissing) {
  console.error(`smoke-bindings: ${notRun.length} samples did not run. Install their toolchains, or pass --allow-missing to skip missing toolchains.`);
  process.exit(1);
}
if (notRun.length) console.log(`smoke-bindings: ${notRun.length} samples did not run, and --allow-missing lets the run pass.`);
const tools = [...built.values()].filter((b) => b.toolchain).flatMap((b) => b.toolchain);
console.log(`smoke-bindings: ${proved} samples replayed.${tools.length ? ` Toolchains: ${[...new Set(tools)].join('; ')}.` : ''}`);
