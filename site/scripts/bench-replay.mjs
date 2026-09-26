#!/usr/bin/env node
// Replay the one command the site writes for the Beatles Bench section, on the
// every-language page, and check it prints the output the page shows.
//
//   BEATLES_BENCH=path/to/beatles-bench node scripts/bench-replay.mjs [--write]
//
// The command answers from the bench's saved recording, so it needs no key
// and sends no request. The key and the server address are removed from its
// environment. THINKTHEN_BIN names the command; the default is thinkthen on
// PATH. --write saves what it printed in place of checking it.

import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { execFileSync } from 'node:child_process';

const SITE = process.cwd();
const BENCH = process.env.BEATLES_BENCH;
const FILE = path.join(SITE, 'src', 'data', 'bench', 'every-language.json');
const fail = (m) => { console.error(`bench-replay: ${m}`); process.exit(1); };

if (!BENCH) fail('set BEATLES_BENCH to a checkout of the bench');
const pin = fs.readFileSync(path.join(SITE, 'src', 'data', 'bench', 'PIN'), 'utf8').trim();
const head = execFileSync('git', ['-C', BENCH, 'rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
if (head !== pin) fail(`the checkout is at ${head}, and src/data/bench/PIN names ${pin}`);

const page = JSON.parse(fs.readFileSync(FILE, 'utf8'));
const env = { ...process.env };
delete env.THINKTHEN_API_KEY;
delete env.THINKTHEN_BASE_URL;
if (process.env.THINKTHEN_BIN) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'bench-replay-'));
  fs.symlinkSync(path.resolve(process.env.THINKTHEN_BIN), path.join(dir, 'thinkthen'));
  env.PATH = `${dir}${path.delimiter}${env.PATH}`;
}
const got = execFileSync('sh', ['-c', page.command], { cwd: path.join(BENCH, page.runsIn), env, encoding: 'utf8' });

if (process.argv.includes('--write')) {
  fs.writeFileSync(FILE, JSON.stringify({ ...page, pin, output: got }, null, 2) + '\n');
  console.log(`bench-replay: wrote ${path.relative(SITE, FILE)}`);
} else if (got !== page.output) {
  fail(`the every-language command printed\n${got}and the page shows\n${page.output}`);
} else if (page.pin !== pin) {
  fail(`the page was checked at ${page.pin}, and PIN names ${pin}. Run again with --write`);
} else {
  console.log('bench-replay: the every-language command prints what the page shows');
}
