// Writes one answers page per question-list entry marked `answers_page: true`,
// and the answers index that lists them. Pages are rebuilt, never edited.
//
// An entry names its examples in `examples`, a list of {id, says}: one command per box, each
// with the line above it that says what it does. `example` (one id) still works. `explain`
// maps each output field the answer relies on to its meaning; the page's "Reading the output"
// table shows the value of each field from the examples' real output, and the generator fails
// when a field is not in that output.
import fs from 'node:fs';
import path from 'node:path';
import YAML from 'yaml';
import site from '../src/data/site.json' with { type: 'json' };
import { ranOn } from '../src/lib/verified.mjs';
import { inputFiles, readExample, EXAMPLES, capturedStreams, inlineCode } from '../src/lib/examples.mjs';
import { jsonValues } from '../src/lib/json-values.mjs';
import { linkTerms } from '../src/lib/glossary-links.mjs';

const qdir = path.resolve('questions');
const out = path.resolve('src/content/docs/answers');
// The question list writes the version as {version}; site.json holds it.
const withFacts = (v) => (typeof v === 'string' ? v.replaceAll('{version}', site.version) : v);
const questions = fs.readdirSync(qdir).filter((f) => f.endsWith('.yaml')).flatMap((f) =>
  (YAML.parse(fs.readFileSync(path.join(qdir, f), 'utf8')) ?? []).map((q) => ({ ...Object.fromEntries(Object.entries(q).map(([k, v]) => [k, withFacts(v)])), area: f.replace(/\.yaml$/, '') })),
);
const seen = new Set();
for (const q of questions) {
  if (seen.has(q.id)) throw new Error(`questions: duplicate id ${q.id}`);
  seen.add(q.id);
}
const answered = questions.filter((q) => q.answers_page).sort((a, b) => a.rank - b.rank);
for (const q of answered) {
  q.examples = q.examples ?? (q.example ? [{ id: q.example }] : []);
  q.example = q.examples[0]?.id;
  q.captured = q.examples.every((example) => !readExample(example.id).unverified);
}

// What each function does, in one clause, for the opening paragraph.
const FUNCTIONS = {
  decide: 'answers a yes-or-no question about each record',
  filter: 'keeps the records that answer yes',
  choose: 'picks one answer from a list of choices',
  tag: 'assigns every label that applies',
  score: 'places each record on ordered levels',
  rank: 'orders records on ordered levels',
  find: 'finds the line that answers a question',
  recognize: 'finds names of the kinds you list',
  relate: 'finds relations between names',
  annotate: 'answers several saved questions about each record',
  run: 'runs a saved entry of a toolbox',
};
const functionOf = (q) => {
  for (const e of q.examples) {
    const m = readExample(e.id).show.match(/\bthinkthen (\w+)/);
    if (m && FUNCTIONS[m[1]]) return m[1];
  }
  return null;
};

// The value of one field in the examples' output: a JSON path, a table column, or a line.
const show = (v) => (typeof v === 'string' ? v : JSON.stringify(v));
const pathValues = (values, key) => values.map((v) => key.split('.').reduce((o, k) => (o && typeof o === 'object' && k in o ? o[k] : undefined), v)).filter((v) => v !== undefined);
const exampleOutputs = (q) => q.examples.flatMap(({ id }) => {
  const example = readExample(id);
  return capturedStreams(example);
});

function fieldValue(outputs, key) {
  const values = outputs.flatMap((o) => jsonValues(o));
  const found = pathValues(values, key);
  if (found.length) {
    const distinct = [...new Set(found.map(show))];
    return distinct.length > 3 ? `${distinct.slice(0, 3).join(', ')}, …` : distinct.join(', ');
  }
  for (const o of outputs) {
    const lines = o.split('\n');
    const head = lines[0] ?? '';
    const sep = head.includes('\t') ? '\t' : head.includes(',') ? ',' : null;
    const unq = (c) => c.replace(/^"(.*)"$/, '$1');
    const cols = sep ? head.split(sep).map(unq) : [];
    const i = cols.indexOf(key);
    if (i >= 0) {
      const col = [...new Set(lines.slice(1).filter(Boolean).map((l) => unq(l.split(sep)[i] ?? '')))];
      return col.length > 3 ? `${col.slice(0, 3).join(', ')}, …` : col.join(', ');
    }
  }
  for (const o of outputs) {
    const line = o.split('\n').find((l) => l.includes(key));
    if (line) return line.length > 60 ? `${line.slice(0, 57)}…` : line;
  }
  return null;
}
// Inventory the complete class before writing pages, so one stale explanation does not hide the next.
const missingFields = [];
for (const q of answered.filter((q) => q.captured)) {
  const outputs = exampleOutputs(q);
  for (const key of Object.keys(q.explain ?? {})) {
    if (fieldValue(outputs, key) === null) missingFields.push(`questions/${q.area}.yaml: ${q.id} explains ${key}, which its examples' output does not hold`);
  }
}
if (missingFields.length) throw new Error(missingFields.join('\n'));
fs.rmSync(out, { recursive: true, force: true });
fs.mkdirSync(out, { recursive: true });

const code = (s) => inlineCode(s).replace(/\|/g, '\\|');

// One command per box. Each box's input files show once, before the first box that reads them.
function examplesMarkdown(q) {
  const shownFiles = new Set();
  return q.examples.map((e, i) => {
    const files = inputFiles(e.id).filter((f) => {
      const key = `${f}\n${fs.readFileSync(path.join(EXAMPLES, e.id, f), 'utf8')}`;
      if (shownFiles.has(key)) return false;
      shownFiles.add(key);
      return true;
    });
    const list = files.map((f) => `\`${f}\``);
    const lead = files.length ? `${i ? 'This command reads' : 'The example reads'} ${list.length > 1 ? `${list.slice(0, -1).join(', ')} and ${list.at(-1)}` : list[0]}:\n\n${files.map((f) => `<ExampleFile id="${e.id}" file="${f}" />`).join('\n\n')}\n\n` : '';
    const says = e.says ? `${e.says.trim()}\n\n` : '';
    const lines = e.lines ?? (i === 0 ? q.lines : undefined);
    return `${lead}${says}<Example id="${e.id}"${lines ? ` lines={${lines}}` : ''} />`;
  }).join('\n\n');
}

// An answer is verified on the day its example last ran (examples/<id>/ran), not on a typed date.
for (const q of answered) q.verified = q.captured ? q.examples.map((e) => ranOn(e.id)).sort().at(-1) : new Date().toISOString().slice(0, 10);
const lastRun = answered.map((q) => q.verified).sort().at(-1) ?? new Date().toISOString().slice(0, 10);
const fm = (o) => `---\n${YAML.stringify(o).trim()}\n---\n`;
for (const q of answered) {
  for (const k of ['question', 'answer', 'why', 'limits', 'page', 'reference']) if (!q[k]) throw new Error(`questions/${q.area}.yaml: ${q.id} lacks ${k}`);
  if (!q.examples.length) throw new Error(`questions/${q.area}.yaml: ${q.id} names no example`);
  if (!q.explain) throw new Error(`questions/${q.area}.yaml: ${q.id} lacks explain, the fields its answer relies on`);
  const fn = functionOf(q);
  const context = `${site.product} asks a language model plain-language questions about your records and returns one typed answer per record.${fn ? ` This answer uses [\`thinkthen ${fn}\`](/functions/${fn === 'run' ? '' : fn}/), which ${FUNCTIONS[fn]}.` : ''}`.replace('/functions//', '/toolboxes/');
  const outputs = exampleOutputs(q);
  const rows = Object.entries(q.captured ? q.explain : {}).map(([key, meaning]) => {
    const value = fieldValue(outputs, key);
    if (value === null) throw new Error(`questions/${q.area}.yaml: ${q.id} explains ${key}, which its examples' output does not hold`);
    return `| ${code(key)} | ${code(value)} | ${String(meaning).trim().replace(/\n/g, ' ').replace(/\|/g, '\\|')} |`;
  });
  const body = [
    "import Example from '../../../components/Example.astro';",
    "import ExampleFile from '../../../components/ExampleFile.astro';",
    '',
    `${q.answer.trim()} ${context}`,
    '',
    '## Example',
    '',
    examplesMarkdown(q),
    '',
    ...(q.captured ? ['## Reading the output', '', '| Look for | In this run | Meaning |', '| --- | --- | --- |', ...rows] : []),
    '',
    '## Why it works',
    '',
    q.why.trim(),
    '',
    '## Limits',
    '',
    q.limits.trim(),
    '',
    '## Next',
    '',
    // An entry whose main page is its own answers page links only its reference.
    ...(q.page === `/answers/${q.id}/` ? [] : [`- [${q.page_title ?? 'Main guide'}](${q.page})`]),
    `- [${q.reference_title ?? 'Reference'}](${q.reference})`,
    '',
    ...(q.captured ? [`Verified on ${q.verified} with ${site.product} ${site.version}.`] : []),
    '',
  ].join('\n');
  fs.writeFileSync(
    path.join(out, `${q.id}.mdx`),
    fm({ title: q.question, description: q.answer.trim().split('\n')[0].slice(0, 300), goal: `Answer: ${q.question}`, reader: q.reader ?? 'Someone who searched for this question', kind: 'answer', verified: q.verified, sidebar: { hidden: true }, pagefind: true }) + '\n' + linkTerms(body, `/answers/${q.id}/`).trimEnd() + '\n',
  );
}
const index = [
  `Short answers for ${site.product} ${site.version}, which asks a language model plain-language questions about your records. Each page answers one question with a command, its real output, a table that reads the output, why it works and its limits.`,
  '',
  ...[...new Set(answered.map((q) => q.area))].flatMap((area) => [
    `## ${area[0].toUpperCase()}${area.slice(1).replace(/-/g, ' ')}`,
    '',
    ...answered.filter((q) => q.area === area).map((q) => `- [${q.question}](/answers/${q.id}/)`),
    '',
  ]),
].join('\n');
fs.writeFileSync(
  path.join(out, 'index.mdx'),
  fm({ title: 'Answers', description: `Short answers to common ${site.product} questions, each with a command and its real output.`, goal: 'List every answers page so a reader or agent can find one question.', reader: 'Anyone with one specific question', kind: 'index', verified: lastRun }) + '\n' + index,
);
console.log(`answers: ${answered.length} pages from ${questions.length} questions`);
