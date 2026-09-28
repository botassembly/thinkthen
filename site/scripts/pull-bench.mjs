#!/usr/bin/env node
// Copy from a Beatles Bench checkout every file the Beatles examples read, so
// the smoke run needs no checkout.
//
//   BEATLES_BENCH=path/to/beatles-bench node scripts/pull-bench.mjs
//
// The checkout must sit at the commit examples/beatles/BENCH names, with no
// local changes. Each example runs under strace in a copy of that commit, and
// every file it opens inside the copy lands in examples/beatles/bench/ at the
// same path. THINKTHEN_BIN names the command, as in smoke.mjs.

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { execFileSync, spawnSync } from 'node:child_process';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const root = path.join(site, 'examples', 'beatles');
const out = path.join(root, 'bench');
const checkout = process.env.BEATLES_BENCH;
if (!checkout) { console.error('pull-bench: set BEATLES_BENCH to a bench checkout'); process.exit(2); }

const pin = fs.readFileSync(path.join(root, 'BENCH'), 'utf8').trim();
const git = (...args) => execFileSync('git', ['-C', checkout, ...args], { encoding: 'utf8', maxBuffer: 1 << 28 }).trim();
if (git('rev-parse', 'HEAD') !== pin) { console.error(`pull-bench: the checkout is not at ${pin}`); process.exit(1); }
if (git('status', '--porcelain')) { console.error('pull-bench: the checkout has local changes'); process.exit(1); }

const bin = path.resolve(process.env.THINKTHEN_BIN || path.join(site, '..', 'target', 'release', 'thinkthen'));
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'pull-bench-'));
const copy = path.join(tmp, 'bench');
fs.mkdirSync(copy);
execFileSync('sh', ['-c', 'git -C "$1" archive HEAD | tar -x -C "$2"', 'sh', checkout, copy]);
fs.mkdirSync(path.join(tmp, 'bin'));
fs.symlinkSync(bin, path.join(tmp, 'bin', 'thinkthen'));

const folders = JSON.parse(fs.readFileSync(path.join(root, 'folders.json'), 'utf8'));
const env = { ...process.env, PATH: `${path.join(tmp, 'bin')}:${process.env.PATH}`, HOME: path.join(tmp, 'home') };
delete env.THINKTHEN_API_KEY;

// Only a committed file counts. A file an example writes is not copied.
const tracked = new Set(git('ls-files').split('\n'));
const read = new Set();
for (const [slug, folder] of Object.entries(folders)) {
  for (const name of fs.readdirSync(path.join(root, slug)).filter((n) => n.endsWith('.sh'))) {
    const log = path.join(tmp, 'trace');
    const text = fs.readFileSync(path.join(root, slug, name), 'utf8');
    spawnSync('strace', ['-f', '-qq', '-e', 'trace=openat', '-o', log, 'bash', '-c', text], { cwd: path.join(copy, folder), env });
    for (const line of fs.readFileSync(log, 'utf8').split('\n')) {
      // A thread's call can split across two lines, so the result is not read.
      // A committed file that was asked for was there to open.
      const m = /openat\([^,]+, "([^"]+)", ([A-Z_|]+)/.exec(line);
      if (!m || /O_WRONLY|O_RDWR|O_CREAT/.test(m[2])) continue;
      const file = path.resolve(path.join(copy, folder), m[1]);
      if (tracked.has(path.relative(copy, file))) read.add(path.relative(copy, file));
    }
  }
}

fs.rmSync(out, { recursive: true, force: true });
for (const rel of [...read].sort()) {
  fs.mkdirSync(path.dirname(path.join(out, rel)), { recursive: true });
  fs.copyFileSync(path.join(copy, rel), path.join(out, rel));
}
// Every folder an example runs in exists, even one it reads nothing from.
for (const folder of Object.values(folders)) fs.mkdirSync(path.join(out, folder), { recursive: true });
fs.rmSync(tmp, { recursive: true, force: true });
console.log(`pull-bench: copied ${read.size} files from bench ${pin.slice(0, 8)}`);
