#!/usr/bin/env node
// Copy every example into src/data/examples/ from the places that own it.
//
// Nothing here is typed by hand. A Bash cell comes from the deck's recorded
// run: the command from examples/run.sh, the output from examples/out/*.txt.
// A code cell comes from the deck's surfaces.md, which is drawn and not run.
// Every function-and-surface cell gets a status: run, drawn, or planned.
//
//   node scripts/pull-examples.mjs
//
// The deck folder is outside this repository, so the pulled files are
// committed here and the site build never reaches for it.

import fs from 'node:fs';
import path from 'node:path';
import { FUNCTIONS, SURFACES, HOWTOS, RECIPES } from '../src/data/catalog.mjs';

const DECK = process.env.THINKTHEN_DECK
  || '/home/ian/workspace/repos/mktg/decks/2026-09-21-thinkthen-semantic-commands';
const OUT = path.join(process.cwd(), 'src', 'data', 'examples');

const read = (p) => fs.readFileSync(p, 'utf8');
const exists = (p) => fs.existsSync(p);

// ---------------------------------------------------------------- shell runs

// Join a line that ends in a backslash with the line after it.
function joinContinuations(lines) {
  const out = [];
  for (const line of lines) {
    if (out.length && /\\$/.test(out[out.length - 1])) {
      out[out.length - 1] = out[out.length - 1].replace(/\s*\\$/, ' ') + line.trim();
    } else {
      out.push(line);
    }
  }
  return out;
}

// Pull every `cat > name <<'TXT'` body and every show/showj/showr call out of
// the deck's examples/run.sh.
function parseExamplesRun(text) {
  const raw = text.split('\n');
  const files = {};
  const calls = [];
  let i = 0;
  const lines = joinContinuations(raw);
  while (i < lines.length) {
    const line = lines[i];
    const heredoc = /<<'([A-Z]+)'/.exec(line);
    let body = null;
    if (heredoc) {
      const tag = heredoc[1];
      const collected = [];
      i += 1;
      while (i < lines.length && lines[i] !== tag) collected.push(lines[i++]);
      body = collected.join('\n');
    }
    const cat = /^cat > (\S+) <</.exec(line);
    if (cat) { files[cat[1]] = body; i += 1; continue; }

    const call = /^(show|showj|showr) (.*)$/.exec(line);
    if (call) {
      const kind = call[1];
      let rest = call[2];
      const name = rest.split(/\s+/)[0];
      rest = rest.slice(name.length).trim();
      let jq = null;
      if (kind !== 'show') {
        const m = /^('(?:[^']*)'|\S+)\s+(.*)$/.exec(rest);
        jq = m[1].replace(/^'|'$/g, '');
        rest = m[2];
      }
      let command = rest
        .replace(/\$tt\b/g, 'thinkthen')
        .replace(/\s*"\$@"/g, '')
        .replace(/\s+<<'[A-Z]+'\s*$/, '')
        .trim();
      if (jq) command += kind === 'showr' ? ` | jq -r '${jq}'` : ` | jq '${jq}'`;
      calls.push({ name, command, heredoc: body, redirect: /< (\S+)/.exec(rest)?.[1] });
      i += 1;
      continue;
    }
    i += 1;
  }
  return { files, calls };
}

// The recorded output, with the harness's trailing `exit N` line split off.
function recorded(dir, name) {
  const file = path.join(dir, name + '.txt');
  if (!exists(file)) return null;
  const rows = read(file).replace(/\n+$/, '').split('\n');
  let code = null;
  if (rows.length && /^exit \d+$/.test(rows[rows.length - 1])) {
    code = Number(rows.pop().slice(5));
  }
  return { output: rows.join('\n'), exit: code };
}

// The deck records one run that prints the whole raw result object. That
// object carries internal field names the site does not print, and the pages
// teach the same lesson with `jq .answer`. It is left out here, not edited.
const SKIP_RUNS = new Set(['decide-details']);

function bashCells() {
  const runSh = path.join(DECK, 'examples', 'run.sh');
  const outDir = path.join(DECK, 'examples', 'out');
  const { files, calls } = parseExamplesRun(read(runSh));
  const byFunction = {};
  for (const call of calls) {
    if (SKIP_RUNS.has(call.name)) continue;
    const fn = FUNCTIONS.find((f) => call.name === f.name || call.name.startsWith(f.name + '-'))
      || (call.name.startsWith('question-') ? FUNCTIONS.find((f) => f.name === 'question-file') : null);
    if (!fn) throw new Error(`no function owns the example ${call.name}`);
    const rec = recorded(outDir, call.name);
    if (!rec) throw new Error(`${call.name}: no recorded output`);
    const input = call.heredoc ?? (call.redirect ? files[call.redirect] : null);
    (byFunction[fn.name] ||= []).push({
      name: call.name,
      command: call.command,
      input,
      inputFile: call.heredoc ? null : call.redirect || null,
      output: rec.output,
      exit: rec.exit,
    });
  }
  return byFunction;
}

// ------------------------------------------------------------- drawn surfaces

// How a call to one function is written on one surface. A chunk that matches
// belongs to that function's cell.
function callPattern(surface, fn) {
  const n = fn.replace(/-/g, '_');
  const forms = {
    python: [`tt.${n}(`],
    polars: [`tt.${n}(`],
    typescript: [`tt.${n}(`],
    ruby: [`ThinkThen.${n}(`],
    r: [`tt_${n}(`],
    rust: [`tt.${n}(`, `Question::${n}(`, `Recognize::`, `Relate::`],
    c: [`thinkthen_${n}(`],
    duckdb: [`thinkthen_${n}(`],
    sqlite: [`thinkthen_${n}(`],
    postgresql: [`thinkthen_${n}(`],
  }[surface] || [];
  if (fn === 'question-file') {
    return { python: ['tt.question('], polars: ['tt.question('], typescript: ['tt.question('],
      ruby: ['ThinkThen.question('], r: ['tt_question('], rust: ['Question::decide('],
      c: ['thinkthen_question('], duckdb: ["'@"], sqlite: ["'@"], postgresql: ["'@"] }[surface] || [];
  }
  if (fn === 'recognize' && surface === 'rust') return ['Recognize::'];
  if (fn === 'relate' && surface === 'rust') return ['Relate::'];
  return forms;
}

const COMMENT = /^\s*(#|\/\/|--|\/\*|\*)/;

function chunksOf(code) {
  return code.split(/\n\s*\n/).map((c) => c.replace(/\s+$/, '')).filter(Boolean);
}

function chunkCalls(chunk, surface, fn) {
  const body = chunk.split('\n').filter((l) => !COMMENT.test(l)).join('\n');
  return callPattern(surface, fn).some((p) => body.includes(p));
}

// Every fenced block under a `## Heading` in a Markdown file.
function fencedBySection(text) {
  const sections = {};
  let heading = null;
  const lines = text.split('\n');
  for (let i = 0; i < lines.length; i += 1) {
    const h = /^## (.+)$/.exec(lines[i]);
    if (h) { heading = h[1].trim(); continue; }
    if (/^```/.test(lines[i])) {
      const lang = lines[i].slice(3).trim();
      const body = [];
      i += 1;
      while (i < lines.length && !/^```/.test(lines[i])) body.push(lines[i++]);
      if (heading) (sections[heading] ||= []).push({ lang, code: body.join('\n') });
    }
  }
  return sections;
}

function drawnCells() {
  const text = read(path.join(DECK, 'surfaces.md'));
  const sections = fencedBySection(text);
  const cells = {};
  const wholeBlocks = {};
  for (const surface of SURFACES) {
    if (surface.slug === 'shell') continue;
    const block = sections[surface.deckHeading]?.[0];
    if (!block) continue;
    // Split on the whole heading line. A bare `## R` also matches `## Ruby`.
    const install = /^Install: `(.+)`$/m.exec(
      text.split(`\n## ${surface.deckHeading}\n`)[1].split('\n```')[0]);
    wholeBlocks[surface.slug] = { lang: block.lang, code: block.code, install: install?.[1] || null };
    const chunks = chunksOf(block.code);
    const preamble = [];
    for (const chunk of chunks) {
      const owners = FUNCTIONS.filter((f) => chunkCalls(chunk, surface.slug, f.name));
      if (!owners.length) {
        if (!preamble.length || chunks.indexOf(chunk) < 2) preamble.push(chunk);
        continue;
      }
      for (const owner of owners) {
        const key = `${owner.name}|${surface.slug}`;
        if (cells[key]) continue;
        cells[key] = {
          lang: block.lang,
          code: [preamble.join('\n\n'), chunk].filter(Boolean).join('\n\n'),
        };
      }
    }
  }
  return { cells, wholeBlocks };
}

// recognize and relate are drawn on their own page in the deck.
function recognizeCells() {
  const text = read(path.join(DECK, 'recognize-surfaces.md'));
  const cells = {};
  const langOrder = ['python', 'typescript', 'ruby', 'r', 'rust', 'c'];
  const parts = text.split(/^## /m);
  const sections = [
    { fn: 'recognize', body: parts[0] },
    { fn: 'relate', body: parts.find((p) => p.startsWith('`relate`')) },
  ];
  for (const section of sections) {
    if (!section.body) continue;
    const blocks = [];
    const lines = section.body.split('\n');
    for (let i = 0; i < lines.length; i += 1) {
      if (/^```/.test(lines[i])) {
        const lang = lines[i].slice(3).trim();
        const body = [];
        i += 1;
        while (i < lines.length && !/^```/.test(lines[i])) body.push(lines[i++]);
        blocks.push({ lang, code: body.join('\n') });
      }
    }
    let n = 0;
    for (const block of blocks) {
      if (block.lang === 'sql') {
        // One comment line naming a database, then the statement under it.
        const rows = block.code.split('\n');
        for (let i = 0; i < rows.length; i += 1) {
          const hint = /^--\s*(DuckDB|SQLite|PostgreSQL)/i.exec(rows[i]);
          if (!hint || !rows[i + 1]) continue;
          const slug = hint[1].toLowerCase();
          const key = `${section.fn}|${slug}`;
          if (!cells[key]) cells[key] = { lang: 'sql', code: `${rows[i]}\n${rows[i + 1]}` };
        }
        continue;
      }
      const slug = langOrder[n];
      n += 1;
      if (slug) cells[`${section.fn}|${slug}`] = { lang: block.lang, code: block.code };
    }
  }
  return cells;
}

// ------------------------------------------------------------------- how-tos

function parseUsecasesRun(text) {
  const lines = text.split('\n');
  const files = {};
  const runs = {};
  let i = 0;
  while (i < lines.length) {
    const line = lines[i];
    const cat = /^cat > (\S+) <<'([A-Z]+)'/.exec(line);
    if (cat) {
      const body = [];
      i += 1;
      while (i < lines.length && lines[i] !== cat[2]) body.push(lines[i++]);
      files[cat[1]] = body.join('\n');
      i += 1;
      continue;
    }
    if (/^\{ /.test(line)) {
      const block = [];
      let row = line;
      while (true) {
        const end = /\} > "\$out\/([^"]+)\.txt"/.exec(row);
        if (end) {
          const name = end[1];
          block.push(row.slice(0, end.index));
          const command = block
            .join('\n')
            .replace(/^\{ /, '')
            .replace(/\btt /g, 'thinkthen ')
            .replace(/"\$here\/([^"]+)"/g, '$1')
            .split('\n')
            .filter((l) => !/echo "exit \$\?"/.test(l) && l.trim() !== '')
            .join('\n')
            .replace(/;\s*$/, '')
            .trimEnd();
          runs[name] = { command };
          break;
        }
        block.push(row);
        i += 1;
        if (i >= lines.length) break;
        row = lines[i];
      }
      i += 1;
      continue;
    }
    i += 1;
  }
  return { files, runs };
}

function howtoCells() {
  const runSh = path.join(DECK, 'usecases', 'run.sh');
  const outDir = path.join(DECK, 'usecases', 'out');
  const { files, runs } = parseUsecasesRun(read(runSh));
  const out = {};
  for (const howto of HOWTOS) {
    const steps = [];
    for (const name of howto.runs) {
      const run = runs[name];
      if (!run) throw new Error(`how-to ${howto.slug}: no run named ${name}`);
      const rec = recorded(outDir, name);
      if (!rec) throw new Error(`how-to ${howto.slug}: no recorded output for ${name}`);
      steps.push({ name, command: run.command, output: rec.output, exit: rec.exit });
    }
    out[howto.slug] = {
      status: 'run',
      source: 'deck usecases/run.sh and usecases/out/',
      input: howto.input ? files[howto.input] ?? null : null,
      inputFile: howto.input || null,
      steps,
    };
  }
  return out;
}

// ------------------------------------------------------------------- recipes

// The recipes follow the how-tos: the deck's recipes/run.sh holds the commands
// and the input files, and recipes/out/ holds what the recorded run printed.
function recipeCells() {
  const runSh = path.join(DECK, 'recipes', 'run.sh');
  const outDir = path.join(DECK, 'recipes', 'out');
  const { files, runs } = parseUsecasesRun(read(runSh));
  const out = {};
  for (const recipe of RECIPES) {
    const steps = recipe.runs.map((name) => {
      const run = runs[name];
      if (!run) throw new Error(`recipe ${recipe.slug}: no run named ${name}`);
      const rec = recorded(outDir, name);
      if (!rec) throw new Error(`recipe ${recipe.slug}: no recorded output for ${name}`);
      return { name, command: run.command, output: rec.output, exit: rec.exit };
    });
    const inputs = recipe.files.map((name) => {
      if (files[name] === undefined) throw new Error(`recipe ${recipe.slug}: run.sh writes no ${name}`);
      return { name, text: files[name] };
    });
    out[recipe.slug] = { status: 'run', source: 'deck recipes/run.sh and recipes/out/', inputs, steps };
  }
  return out;
}

// ---------------------------------------------------------------------- write

// The see sentences are written here, not in the deck. Keep them across a pull.
function keptSee() {
  const see = {};
  if (!exists(OUT)) return see;
  for (const file of fs.readdirSync(OUT).filter((f) => f.endsWith('__shell.json'))) {
    for (const run of JSON.parse(read(path.join(OUT, file))).runs || []) {
      if (run.see) see[run.name] = run.see;
    }
  }
  return see;
}

function main() {
  const see = keptSee();
  fs.rmSync(OUT, { recursive: true, force: true });
  fs.mkdirSync(OUT, { recursive: true });

  const bash = bashCells();
  const { cells: drawn, wholeBlocks } = drawnCells();
  const recognize = recognizeCells();
  const howtos = howtoCells();
  const recipes = recipeCells();

  const index = [];
  for (const fn of FUNCTIONS) {
    for (const surface of SURFACES) {
      const key = `${fn.name}|${surface.slug}`;
      let cell;
      if (surface.slug === 'shell') {
        const runs = bash[fn.name]?.map((run) => (see[run.name] ? { ...run, see: see[run.name] } : run));
        cell = runs
          ? { status: 'run', source: 'deck examples/run.sh and examples/out/', runs }
          : { status: 'planned', source: 'the command has no such function yet' };
      } else if (drawn[key] || recognize[key]) {
        const block = drawn[key] || recognize[key];
        cell = {
          status: 'drawn',
          source: recognize[key] ? 'deck recognize-surfaces.md' : 'deck surfaces.md',
          lang: block.lang,
          code: block.code,
        };
      } else {
        cell = { status: 'planned', source: 'no example drawn for this surface yet' };
      }
      cell.function = fn.name;
      cell.surface = surface.slug;
      fs.writeFileSync(path.join(OUT, `${fn.name}__${surface.slug}.json`),
        JSON.stringify(cell, null, 2) + '\n');
      index.push({ function: fn.name, surface: surface.slug, status: cell.status });
    }
  }

  fs.writeFileSync(path.join(OUT, '_surfaces.json'), JSON.stringify(wholeBlocks, null, 2) + '\n');
  fs.writeFileSync(path.join(OUT, '_howtos.json'), JSON.stringify(howtos, null, 2) + '\n');
  fs.writeFileSync(path.join(OUT, '_recipes.json'), JSON.stringify(recipes, null, 2) + '\n');
  fs.writeFileSync(path.join(OUT, '_index.json'), JSON.stringify(index, null, 2) + '\n');

  const counts = index.reduce((a, c) => ((a[c.status] = (a[c.status] || 0) + 1), a), {});
  console.log(`wrote ${index.length} cells:`, counts);
  console.log(`wrote ${Object.keys(howtos).length} how-tos, ${Object.keys(recipes).length} recipes and ${Object.keys(wholeBlocks).length} surface samples`);
}

main();
