#!/usr/bin/env node
// Fail the build when a function's samples stop showing the same case.
// src/data/cases.mjs holds one case per function: its question, its
// choices and its input. Each function's first Bash example, and each of
// its language and SQL samples in examples/functions/<fn>/, must hold every
// one of those strings. A question file the sample names, from the
// function's files/ folder, counts as part of the sample.
//
// A sample may split a long string across lines. Between two words the
// check takes any run of spaces and string breaks. A break closes a quote,
// joins with nothing or with +, ||, &, ++, .. or ~, or starts a Bash
// `name+=` line, and opens a quote again, with @ in Objective-C. Three
// escapes are undone first: a doubled quote ('' in SQL) and a backslash
// before a quote or a $.

import fs from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { CASES } from '../src/data/cases.mjs';

const OUTPUT = /\.(sh|out|exit)$/;
const BREAK = String.raw`["']\s*(?:[+&|.~]*|\w+\+=)\s*@?["']`;
const GAP = String.raw`(?:\s|${BREAK})+`;

const unescape = (text) => text.replace(/''/g, "'").replace(/\\(["$])/g, '$1');
const escape = (word) => word.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
const pattern = (s) => new RegExp(s.split(' ').map(escape).join(GAP));

// The strings of one case that a text lacks.
export function missing(one, text) {
  const plain = unescape(text);
  return [...one.question, ...one.choices, ...one.input].filter((s) => !pattern(s).test(plain));
}

// The samples of one function folder: the first Bash example, then every
// language and SQL sample, each with the question files it names.
export function samples(dir) {
  const names = fs.readdirSync(dir).filter((n) => fs.statSync(path.join(dir, n)).isFile()).sort();
  const bash = names.find((n) => n.endsWith('.sh'));
  const code = names.filter((n) => !OUTPUT.test(n));
  const filesDir = path.join(dir, 'files');
  const shared = fs.existsSync(filesDir) ? fs.readdirSync(filesDir) : [];
  return [bash, ...code].filter(Boolean).map((name) => {
    const own = fs.readFileSync(path.join(dir, name), 'utf8');
    const read = shared.filter((f) => own.includes(f));
    const text = [own, ...read.map((f) => fs.readFileSync(path.join(filesDir, f), 'utf8'))].join('\n');
    return { name, read, text };
  });
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
  const problems = [];
  let count = 0;
  for (const [fn, one] of Object.entries(CASES)) {
    const dir = path.join(site, 'examples', 'functions', fn);
    for (const sample of samples(dir)) {
      count += 1;
      for (const s of missing(one, sample.text)) {
        const where = [sample.name, ...sample.read].join(' + ');
        problems.push(`functions/${fn}/${where}: lacks ${JSON.stringify(s)}`);
      }
    }
  }
  if (problems.length) {
    console.error(`check-cases: ${problems.length} problems\n  ${problems.join('\n  ')}`);
    process.exit(1);
  }
  console.log(`check-cases: ${count} samples across ${Object.keys(CASES).length} functions show their function's case.`);
}
