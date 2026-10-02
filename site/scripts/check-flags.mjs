#!/usr/bin/env node
// Compare each function's flag data with the command's own help. The build
// runs it, so a reference page cannot list a flag the command lacks, or
// leave out one it has.
//
// For each function it reads `thinkthen <fn> --help`. It uses --help, not
// -h, because -h hides some flags. It ignores -h and --help. The data is the
// function's own options and shared flags in catalog.mjs, plus GLOBAL_FLAGS
// in flags.mjs. GLOBAL_FLAGS must hold exactly the flags every function's
// help shows. It reads no built page.
//
// THINKTHEN_BIN names the command. The default is the build of this
// repository, ../target/release/thinkthen, as smoke.mjs uses.

import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { CODE_FUNCTIONS } from '../src/data/catalog.mjs';
import { GLOBAL_FLAGS, flagsOf, namesOf } from '../src/data/flags.mjs';

const site = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const bin = path.resolve(process.env.THINKTHEN_BIN || path.join(site, '..', 'target', 'release', 'thinkthen'));
if (!fs.existsSync(bin)) {
  console.error(`check-flags: no thinkthen at ${bin}. Build it with cargo build --release, or set THINKTHEN_BIN.`);
  process.exit(2);
}

// A flag line in clap's long help starts with two to six spaces. Its
// description lines sit deeper, so a flag named in prose never counts.
const FLAG_LINE = /^ {2,6}(?:-[A-Za-z], )?(--[a-z][a-z0-9-]*)/gm;
const IGNORED = new Set(['--help']);

function helpFlags(fn) {
  const out = spawnSync(bin, [fn, '--help'], { encoding: 'utf8' });
  if (out.status !== 0) throw new Error(`check-flags: thinkthen ${fn} --help exited ${out.status}: ${out.stderr}`);
  const names = new Set([...out.stdout.matchAll(FLAG_LINE)].map((m) => m[1]).filter((n) => !IGNORED.has(n)));
  if (!names.size) throw new Error(`check-flags: thinkthen ${fn} --help shows no flag`);
  return names;
}

const dataNames = (flags) => flags.flatMap((f) => namesOf(f.spec));
const globals = new Set(dataNames(GLOBAL_FLAGS));
const faults = [];
let every = null;

for (const fn of CODE_FUNCTIONS) {
  const help = helpFlags(fn.name);
  every = every ? new Set([...every].filter((n) => help.has(n))) : new Set(help);
  const own = dataNames(flagsOf(fn));
  for (const n of own.filter((n) => globals.has(n))) faults.push(`${fn.name}: ${n} is global, so its own list must leave it out`);
  const data = new Set([...own, ...globals]);
  for (const n of help) if (!data.has(n)) faults.push(`${fn.name}: --help shows ${n}, and the data lacks it`);
  for (const n of data) if (!help.has(n)) faults.push(`${fn.name}: the data lists ${n}, and --help lacks it`);
}
for (const n of every) if (!globals.has(n)) faults.push(`every function's --help shows ${n}, so GLOBAL_FLAGS must list it`);

if (faults.length) {
  console.error(`check-flags: ${faults.length} gap(s) between the flag data and ${bin}:`);
  for (const f of faults) console.error(`  ${f}`);
  process.exit(1);
}
console.log(`check-flags: ${CODE_FUNCTIONS.length} functions match their --help, with ${globals.size} global flags`);
