// Validate the complete static matrix and fresh, normalized support-inbox answers.
import fs from 'node:fs';
import path from 'node:path';
import { isDeepStrictEqual } from 'node:util';
import { functions, surfaces, cellId } from '../src/lib/function-surfaces.mjs';
import { selectQuestionCall } from '../src/lib/examples.mjs';

const fresh = process.argv.includes('--fresh');
const errors = [];
let verified = 0;
let unverified = 0;
for (const fn of functions) {
  const caseFile = path.join('examples', 'functions', fn, 'case.json');
  try {
    const shared = JSON.parse(fs.readFileSync(caseFile, 'utf8'));
    if (!shared || typeof shared !== 'object' || Array.isArray(shared)) throw new Error('shared case must be an object');
  } catch { errors.push(`${fn}: missing or invalid shared support-inbox case.json`); }
  const command = path.join('.example-runs', cellId(fn, 'command'), 'answers.json');
  for (const { id } of surfaces) {
    const cell = cellId(fn, id);
    const dir = path.join('examples', cell);
    const reason = path.join(dir, 'unverified');
    if (!fs.existsSync(dir)) { errors.push(`${cell}: missing example directory`); continue; }
    const shown = fs.readdirSync(dir).filter((f) => f.startsWith('show.'));
    if (shown.length !== 1) errors.push(`${cell}: needs exactly one show.EXT file`);
    if (shown.length === 1) {
      try {
        const source = fs.readFileSync(path.join(dir, shown[0]), 'utf8').replace(/^#!.*\n/, '').trimEnd();
        selectQuestionCall(source, JSON.parse(fs.readFileSync(path.join(dir, 'call.json'), 'utf8')), cell);
      } catch (error) { errors.push(`${cell}: missing or invalid question-call selection (${error.message})`); }
    }
    if (fs.existsSync(reason)) {
      unverified++;
      if (!fs.readFileSync(reason, 'utf8').trim()) errors.push(`${cell}: empty unverified reason`);
      if (fs.existsSync(path.join(dir, 'output.txt'))) errors.push(`${cell}: unverified cell must not display saved output`);
      if (!fs.existsSync(path.join(dir, 'run.sh'))) continue;
    }
    verified++;
    if (!fs.existsSync(path.join(dir, 'run.sh'))) errors.push(`${cell}: missing run.sh or unverified reason`);
    if (fresh) {
      const actual = path.join('.example-runs', cell, 'answers.json');
      try {
        const expected = JSON.parse(fs.readFileSync(command, 'utf8'));
        const answers = JSON.parse(fs.readFileSync(actual, 'utf8'));
        if (!isDeepStrictEqual(answers, expected)) errors.push(`${cell}: answers differ from the command's shared case`);
      } catch (error) { errors.push(`${cell}: missing or invalid fresh semantic answers (${error.code ?? error.name})`); }
    }
  }
}
// Check the displayed example, rather than a build launcher that may use unrelated locals.
const generic = /\b(?:row|rows|result|results|value|values)\b/i;
function inspect(dir) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const file = path.join(dir, entry.name);
    if (entry.isDirectory()) { inspect(file); continue; }
    if (!/^show\./.test(entry.name)) continue;
    const text = fs.readFileSync(file, 'utf8');
    for (const line of text.split('\n')) {
      // Assignment, declaration and SQL aliases. Quoted JSON field names are not variables.
      const declaration = line.match(/(?:\b(?:let|const|var|auto|val|final)\s+|^\s*)(\w+)\s*(?::[^=]+)?(?::=|=(?!=))/);
      const alias = line.match(/\bAS\s+(row|rows|result|results|value|values)\b/i);
      const iteration = line.match(/\b(?:for|foreach)\s*(?:\(\s*)?(?:(?:const\s+)?(?:auto|var|let|final)\s*[&*]?\s*)?(\w+)\s+(?:in|of)\b/) ?? line.match(/\b(?:auto|var|let)\s*[&*]?\s*(\w+)\s*:/);
      const typedLoop = line.match(/\b(?:for|foreach)\s*\(\s*[^;=]+?\s+(\w+)\s*:/) ?? line.match(/\bfor\s+(?:\w+\s*,\s*)?(\w+)\s*:?=\s*range\b/) ?? line.match(/\bfor\s*\(.+\)\s*\|(\w+)(?:\s*,|\|)/);
      const typed = line.match(/\b(?:Row(?:<[^;]+?>)?|Answer(?:<[^;]+?>)?)\s*[&*]?\s*(\w+)\s*=/);
      if (entry.name.endsWith('.sql') && /\bSELECT\s+(?:[\w]+\.)?answer\s*(?:,|FROM|$)/i.test(line)) errors.push(`${file}: alias the answer column for its meaning`);
      if ([declaration?.[1], alias?.[1], iteration?.[1], typedLoop?.[1], typed?.[1]].some((name) => name && generic.test(name))) errors.push(`${file}: name the answer for its meaning: ${line.trim()}`);
    }
  }
}
inspect('examples');
if (errors.length) {
  console.error(`${errors.length} function example problem(s):\n${errors.slice(0, 12).join('\n')}${errors.length > 12 ? '\nUse the matrix manifest to inspect the remaining cells.' : ''}`);
  process.exit(1);
}
console.log(`Function examples: ${functions.length * surfaces.length} cells; ${verified} runnable; ${unverified} withhold public output with a reason; ${fresh ? `fresh answers compared for ${verified} runnable cells` : 'cell coverage checked'}; meaningful answer names`);
