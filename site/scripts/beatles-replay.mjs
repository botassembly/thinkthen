#!/usr/bin/env node
// Run every command on the Beatles Bench pages against a bench checkout and
// check what it prints. Every command answers from a saved recording, so it
// needs no key and sends no request.
//
//   BEATLES_BENCH=path/to/beatles-bench npm run beatles-replay            check
//   BEATLES_BENCH=path/to/beatles-bench npm run beatles-replay -- --write  record
//
// THINKTHEN_BIN names the command (default: thinkthen on PATH). Each step runs
// in a fresh copy of the bench's committed files, so a step that writes a file
// leaves the checkout clean. Output is stdout and stderr together, as a
// terminal shows them.

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { execFileSync, spawnSync } from 'node:child_process';
import { STEPS } from '../src/data/beatles.mjs';

const bench = process.env.BEATLES_BENCH;
if (!bench) { console.error('beatles-replay: set BEATLES_BENCH to a bench checkout'); process.exit(2); }
const write = process.argv.includes('--write');
const out = path.join(process.cwd(), 'src/data/beatles/runs.json');

const git = (...args) => execFileSync('git', ['-C', bench, ...args], { encoding: 'utf8' }).trim();
const commit = git('rev-parse', 'HEAD');

const copy = fs.mkdtempSync(path.join(os.tmpdir(), 'beatles-'));
execFileSync('sh', ['-c', `git -C "$1" archive HEAD | tar -x -C "$2"`, 'sh', bench, copy]);

const env = { ...process.env };
if (env.THINKTHEN_BIN) env.PATH = `${path.dirname(path.resolve(env.THINKTHEN_BIN))}:${env.PATH}`;
delete env.THINKTHEN_API_KEY;

const runs = {};
for (const step of STEPS) {
  const done = spawnSync('bash', ['-c', `{ ${step.command}\n} 2>&1`], { cwd: path.join(copy, step.dir), env, encoding: 'utf8' });
  runs[step.key] = { command: step.command, output: done.stdout.replace(/\n$/, ''), exit: done.status };
}
fs.rmSync(copy, { recursive: true, force: true });

if (write) {
  fs.writeFileSync(out, JSON.stringify({ bench: commit, runs }, null, 1) + '\n');
  console.log(`beatles-replay: wrote ${STEPS.length} runs from bench ${commit.slice(0, 8)}`);
  process.exit(0);
}

const saved = JSON.parse(fs.readFileSync(out, 'utf8'));
const differ = STEPS.filter((s) => JSON.stringify(saved.runs[s.key]) !== JSON.stringify(runs[s.key]));
if (differ.length) {
  console.error(`beatles-replay: ${differ.length} of ${STEPS.length} runs differ: ${differ.map((s) => s.key).join(', ')}`);
  process.exit(1);
}
console.log(`beatles-replay: all ${STEPS.length} runs match, bench ${commit.slice(0, 8)}`);
