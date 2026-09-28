#!/usr/bin/env node
// Run every example on the site and compare what it prints with the output
// the page shows. Every example answers from a saved recording, so the run
// needs no key and sends no request.
//
//   node scripts/smoke.mjs            check every example
//   node scripts/smoke.mjs --update   write what each example printed
//   node scripts/smoke.mjs PATTERN    run only the examples whose path holds PATTERN
//
// An example is a script at examples/<page>/<name>.sh. Its output sits beside
// it in <name>.out, and a nonzero exit code in <name>.exit. Files the script
// reads sit in examples/<page>/files/ and are copied into a fresh folder for
// each page. The scripts of a page run in name order in that one folder. A
// Beatles Bench page runs in a copy of examples/beatles/bench/, in the folder
// examples/beatles/folders.json names for it. A Beatles page with no bench
// folder runs from its own files/, as any other page does. Its recordings
// come from the talk's deck.
//
// THINKTHEN_BIN names the command. The default is the build of this
// repository, ../target/release/thinkthen. A `thinkthen` call that names no
// --replay folder answers from recordings/. A line that starts with `test` is
// an assert: when it fails, the example fails.
//
// The CLI runner does not invoke installed host libraries or SQL extensions.
// examples/SKIP lists each kind with the reason, and the run counts them.

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const root = path.join(site, 'examples');
const bench = path.join(root, 'beatles', 'bench');
const update = process.argv.includes('--update');
const only = process.argv.slice(2).filter((a) => !a.startsWith('--'));

const bin = path.resolve(process.env.THINKTHEN_BIN || path.join(site, '..', 'target', 'release', 'thinkthen'));
if (!fs.existsSync(bin)) {
  console.error(`smoke: no thinkthen at ${bin}. Build it with cargo build --release, or set THINKTHEN_BIN.`);
  process.exit(2);
}
if (spawnSync('jq', ['--version']).status !== 0) {
  console.error('smoke: the examples need jq on PATH.');
  process.exit(2);
}

function walk(dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) return p === bench || e.name === 'files' ? [] : walk(p);
    return [p];
  });
}

const all = walk(root).map((p) => path.relative(root, p)).sort();
const scripts = all.filter((p) => p.endsWith('.sh'));
const stem = (p) => p.replace(/\.(out|exit)$/, '');
const results = new Set(all.filter((p) => /\.(out|exit)$/.test(p) && all.includes(`${stem(p)}.sh`)));
const kept = new Set(['SKIP', 'beatles/folders.json', 'beatles/bench-pin']);

// examples/SKIP: a glob, then the reason, on each line.
const skips = fs.readFileSync(path.join(root, 'SKIP'), 'utf8').split('\n')
  .filter((l) => l.trim() && !l.startsWith('#'))
  .map((l) => {
    const [glob, ...why] = l.trim().split(/\s+/);
    return { glob, why: why.join(' '), re: new RegExp('^' + glob.replace(/[.+^${}()|[\]\\]/g, '\\$&').replace(/\*\*/g, '\0').replace(/\*/g, '[^/]*').replace(/\0/g, '.*') + '$'), count: 0 };
  });
const unrun = all.filter((p) => !p.endsWith('.sh') && !results.has(p) && !kept.has(p));
const orphans = [];
for (const p of unrun) {
  const skip = skips.find((s) => s.re.test(p));
  if (skip) skip.count += 1;
  else orphans.push(p);
}
if (orphans.length) {
  console.error(`smoke: these files are neither run nor listed in examples/SKIP:\n  ${orphans.join('\n  ')}`);
  process.exit(1);
}

const folders = JSON.parse(fs.readFileSync(path.join(root, 'beatles', 'folders.json'), 'utf8'));
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'thinkthen-smoke-'));

// A checkout made under umask 002 leaves its folders writable by the group,
// and thinkthen then warns that another user may change a recording folder.
// The copy a page runs in keeps only its owner's write bits, as a checkout
// made under umask 022 does.
function ownerWrites(dir) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    fs.chmodSync(p, fs.statSync(p).mode & ~0o022);
    if (e.isDirectory()) ownerWrites(p);
  }
}

// The site's recordings, copied so the copy can drop the group write bit.
const recordings = path.join(tmp, 'recordings');
fs.cpSync(path.join(site, 'recordings'), recordings, { recursive: true });
fs.chmodSync(recordings, 0o755);
ownerWrites(recordings);

// The command every example calls. A function call that names no replay
// folder of its own answers from the site's recordings. A dry run sends
// nothing and takes no replay folder. audit, diff, and check send nothing.
const wrapper = path.join(tmp, 'bin');
fs.mkdirSync(wrapper);
fs.writeFileSync(path.join(wrapper, 'thinkthen'), `#!/bin/sh
case $1 in
  decide|choose|tag|score|filter|rank|find|annotate|recognize|relate) ;;
  *) exec "${bin}" "$@" ;;
esac
for a in "$@"; do
  case $a in --replay|--dry-run) exec "${bin}" "$@" ;; esac
done
exec "${bin}" "$@" --replay "${recordings}"
`, { mode: 0o755 });

const env = { ...process.env, PATH: `${wrapper}:${process.env.PATH}`, HOME: path.join(tmp, 'home'), XDG_CACHE_HOME: path.join(tmp, 'cache'), LC_ALL: 'C.UTF-8' };
delete env.THINKTHEN_API_KEY;
delete env.THINKTHEN_BASE_URL;
delete env.THINKTHEN_CACHE;

// A `test` line fails the example when it fails.
const asserting = (text) => text.split('\n').map((l) => (/^\s*test /.test(l) ? `${l} || { echo 'assert failed: ${l.trim().replace(/'/g, "'\\''")}' >&2; exit 99; }` : l)).join('\n');

let ran = 0;
const failed = [];
// The scripts of one page run in order in one folder, as a reader runs them.
const pages = new Map();
for (const rel of scripts) {
  if (only.length && !only.some((o) => rel.includes(o))) continue;
  const page = path.dirname(rel);
  if (!pages.has(page)) pages.set(page, []);
  pages.get(page).push(rel);
}
for (const [page, list] of pages) {
  const work = fs.mkdtempSync(path.join(tmp, 'run-'));
  let cwd = work;
  const folder = page.startsWith('beatles/') && folders[page.split('/')[1]];
  if (folder) {
    fs.cpSync(bench, work, { recursive: true });
    cwd = path.join(work, folder);
  } else if (fs.existsSync(path.join(root, page, 'files'))) {
    fs.cpSync(path.join(root, page, 'files'), work, { recursive: true });
  }
  ownerWrites(work);
  for (const rel of list) {
    const file = path.join(root, rel);
    const text = fs.readFileSync(file, 'utf8');
    const done = spawnSync('bash', ['-c', `{\n${asserting(text)}\n} 2>&1`], { cwd, env, encoding: 'utf8' });
    const output = done.stdout.replace(/\n+$/, '');
    const exit = done.status;
    ran += 1;
    const outFile = file.replace(/\.sh$/, '.out');
    const exitFile = file.replace(/\.sh$/, '.exit');
    // A failed assert is never saved as the expected result.
    if (exit === 99 || output.includes('assert failed:')) {
      failed.push(`${rel}: an assert failed\n${output}`);
      continue;
    }
    if (update) {
      fs.writeFileSync(outFile, output ? output + '\n' : '');
      if (exit !== 0) fs.writeFileSync(exitFile, `${exit}\n`);
      else fs.rmSync(exitFile, { force: true });
      continue;
    }
    const wantOut = fs.existsSync(outFile) ? fs.readFileSync(outFile, 'utf8').replace(/\n+$/, '') : null;
    const wantExit = fs.existsSync(exitFile) ? Number(fs.readFileSync(exitFile, 'utf8').trim()) : 0;
    if (wantOut === null) failed.push(`${rel}: no ${path.basename(outFile)}. Run with --update and read the diff.`);
    else if (output !== wantOut || exit !== wantExit) {
      failed.push(`${rel}: printed something else or exited ${exit}, not ${wantExit}\n--- expected\n${wantOut}\n--- printed\n${output}`);
    }
  }
}
fs.rmSync(tmp, { recursive: true, force: true });

const skipped = skips.reduce((n, s) => n + s.count, 0);
const skipLines = skips.filter((s) => s.count).map((s) => `  ${s.count} ${s.glob}: ${s.why}`).join('\n');
if (update && !failed.length) {
  console.log(`smoke: wrote the output of ${ran} examples. Read the diff before you commit it.`);
  process.exit(0);
}
if (failed.length) {
  console.error(`smoke: ${failed.length} of ${ran} examples failed\n\n${failed.join('\n\n')}`);
  process.exit(1);
}
console.log(`smoke: ${ran} examples match. ${skipped} samples skipped:\n${skipLines}`);
