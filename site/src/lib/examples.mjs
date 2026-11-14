// One home for reading an example from examples/<id>/.
// run.sh is what the build runs; show.<ext> (if present) is what the page shows.
import fs from 'node:fs';
import path from 'node:path';
import { xlsxMarkdown } from './xlsx.mjs';

const LANGS = { sh: 'sh', ps1: 'powershell', md: 'markdown', jsonl: 'json', txt: 'text', log: 'text', tsv: 'text', csproj: 'xml', mod: 'text', jq: 'text', py: 'python', ts: 'ts', js: 'js', sql: 'sql', go: 'go', java: 'java', kt: 'kotlin', cs: 'csharp', r: 'r', R: 'r', c: 'c', cpp: 'cpp', rb: 'ruby', swift: 'swift', zig: 'zig', rs: 'rust', json: 'json', csv: 'csv', yaml: 'yaml', toml: 'toml' };

export const EXAMPLES = path.resolve(process.cwd(), 'examples');
export const needsPreparation = (id) => ['c', 'cpp', 'csharp', 'go', 'java', 'kotlin', 'rust', 'swift', 'ts', 'zig'].includes(exampleSource(id).lang);

export function exampleSource(id) {
  const dir = path.join(EXAMPLES, id);
  const names = fs.readdirSync(dir);
  const runner = names.includes('run.sh') ? 'run.sh' : names.includes('run.ps1') ? 'run.ps1' : null;
  if (!runner && !names.includes('unverified')) throw new Error(`example ${id}: missing runner`);
  const file = names.find((name) => name.startsWith('show.')) ?? runner;
  if (!file) throw new Error(`example ${id}: missing shown source`);
  return { dir, lang: LANGS[file.split('.').pop()] ?? 'text',
    show: fs.readFileSync(path.join(dir, file), 'utf8').replace(/^#!.*\n/, '').trimEnd() };
}

export function readExample(id) {
  const { dir, lang, show } = exampleSource(id);
  const read = (f) => (fs.existsSync(path.join(dir, f)) ? fs.readFileSync(path.join(dir, f), 'utf8') : null);
  const output = read('output.txt');
  if (output !== null && read('capture.json') === null) throw new Error(`example ${id}: public output requires a real capture`);
  if (output === null && read('unverified') === null) throw new Error(`example ${id}: no output.txt; run npm run examples`);
  const selection = read('call.json');
  const call = selection === null ? null : selectQuestionCall(show, JSON.parse(selection), id);
  return {
    id,
    show,
    call,
    lang,
    stderr: read('stderr.txt') ?? '',
    output: output ?? '',
    exit: Number((read('exit') ?? '0').trim()),
    unverified: read('unverified') !== null,
    unverifiedReason: (read('unverified') ?? '').trim(),
    ran: (read('ran') ?? '').trim(),
  };
}

// Both streams are reader-visible field; diagnostics often appear only on stderr.
export const capturedStreams = (example) => [example.output, example.stderr];
// Character references survive HTML's CR/CRLF preprocessing without changing
// a literal ampersand sequence already present in captured text.
export const outputHtml = (text) => text.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;').replaceAll('\r', '&#13;');
export const outputNotice = (text) => text === '' ? '(nothing printed)' : /^(?:\r?\n)$/.test(text) ? '(an empty line)' : '';
export const outputStreams = (example) => example.unverified && !example.output && !example.stderr ? []
  : [['stdout', example.output], ...(example.stderr ? [['stderr', example.stderr]] : [])];

// A reviewed literal source anchor selects a contiguous excerpt, never inferred code.
export function selectQuestionCall(show, selection, id = 'example') {
  const invalid = (why) => { throw new Error(`${id}/call.json: ${why}`); };
  if (!selection || typeof selection !== 'object' || Array.isArray(selection)
      || Object.keys(selection).sort().join(',') !== 'after,anchor,before') invalid('require anchor, before and after only');
  const { anchor, before, after } = selection;
  if (typeof anchor !== 'string' || !anchor.trim() || /[\r\n]/.test(anchor)) invalid('anchor must be one nonempty exact source line');
  if (![before, after].every((count) => Number.isInteger(count) && count >= 0 && count <= 12)
      || before + after > 12) invalid('excerpt context must be 0–12 lines in total');
  const lines = show.split('\n');
  const matches = lines.flatMap((line, index) => line === anchor ? [index] : []);
  if (matches.length !== 1) invalid(`anchor must match exactly once; found ${matches.length}`);
  const start = matches[0] - before;
  const end = matches[0] + after + 1;
  if (start < 0 || end > lines.length) invalid('excerpt context leaves the source');
  const text = lines.slice(start, end).join('\n');
  // astro-expressive-code/components/Code.astro expands each tab to two
  // spaces by default. Bind the validated source line to that rendered line.
  return { anchor, text, line: before, renderedAnchor: anchor.replace(/\t/g, '  '), partial: text !== show };
}

// The label of an example marked unverified: the day of its last run when it has output, then the
// reason, written for readers (what the command needs and what they see without it).
export function unverifiedLabel(ex) {
  return ex.output && ex.ran ? `Output from a run on ${ex.ran}. ${ex.unverifiedReason}` : ex.unverifiedReason;
}

// Display and Markdown retain the exact captured streams. Trimming belongs
// in the shown command, never in the renderer (docs rules 6 and 15).
export const displayOutput = (text) => ({ text, wrapped: false });
export const trimOutput = (text) => text;

// Inline diagnostics can themselves name values with literal backticks.
export function inlineCode(text) {
  const longest = Math.max(0, ...[...text.matchAll(/`+/g)].map((match) => match[0].length));
  const fence = '`'.repeat(longest + 1);
  const pad = longest ? ' ' : '';
  return `${fence}${pad}${text}${pad}${fence}`;
}

// A Markdown code fence longer than any backtick run in the text, so output that holds
// its own fences (Markdown replies) stays one block.
export function fenced(text, lang = '') {
  const longest = Math.max(2, ...[...text.matchAll(/`+/g)].map((m) => m[0].length));
  const fence = '`'.repeat(longest + 1);
  return `${fence}${lang}\n${text}${text && !text.endsWith('\n') ? '\n' : ''}${fence}`;
}

export function exampleMarkdown(id) {
  const ex = readExample(id);
  const streams = outputStreams(ex).map(([, text]) => `${outputNotice(text)}${outputNotice(text) ? '\n\n' : ''}${fenced(text, 'text')}`);
  return `${fenced(ex.show, ex.lang)}\n\n${ex.unverified ? unverifiedLabel(ex) : 'Output'}\n\n${streams.join('\n\n')}\n`;
}

// The files of the example folder that make the build run it, not the files the example reads.
const HARNESS = /^(run\.(?:sh|ps1)|show\..+|output\.txt|stderr\.txt|answers\.json|capture\.json|call\.json|normalize\.sh|exit|ran|unverified|needs|expect-status|exact|build-only)$/;

// examples/<id>/build-only lists files that point the build at the packages staged from this
// commit, one `FILE: reason` per line. A reader writes those files differently, with the
// published package, so pages need not show them.
export function buildOnly(id) {
  const f = path.join(EXAMPLES, id, 'build-only');
  if (!fs.existsSync(f)) return new Map();
  return new Map(fs.readFileSync(f, 'utf8').split('\n').filter((l) => l.trim()).map((l) => {
    const [file, ...why] = l.split(':');
    if (!why.join(':').trim()) throw new Error(`examples/${id}/build-only: ${file} gives no reason`);
    if (!fs.existsSync(path.join(EXAMPLES, id, file.trim()))) throw new Error(`examples/${id}/build-only: ${file} does not exist`);
    return [file.trim(), why.join(':').trim()];
  }));
}

// Only this explicitly published input kind has a download route. IDs cannot
// select another directory, and a linked download must retain the exact bytes.
export function exampleInputUrl(id, file) {
  if (typeof id !== 'string' || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(id) || file !== 'server.py') {
    throw new Error('Example download requires a canonical example ID and server.py');
  }
  return `/example-inputs/${id}/${file}`;
}
export function matchesExampleInputDownload(id, file, links, dist) {
  try {
    const url = exampleInputUrl(id, file);
    if (!links.has(url)) return false;
    const source = path.join(EXAMPLES, id, file);
    const published = path.join(dist, url);
    if (!fs.lstatSync(source).isFile() || !fs.lstatSync(published).isFile()) return false;
    if (!fs.realpathSync(source).startsWith(fs.realpathSync(EXAMPLES) + path.sep)
        || !fs.realpathSync(published).startsWith(fs.realpathSync(dist) + path.sep)) return false;
    return fs.readFileSync(source).equals(fs.readFileSync(published));
  } catch { return false; }
}

// The files an example reads from its own folder, as paths relative to it (subfolders included).
export function inputFiles(id) {
  const dir = path.join(EXAMPLES, id);
  const out = [];
  const walk = (rel) => {
    for (const f of fs.readdirSync(path.join(dir, rel), { withFileTypes: true })) {
      const r = rel ? `${rel}/${f.name}` : f.name;
      if (f.isDirectory()) walk(r);
      else if (rel || !HARNESS.test(f.name)) out.push(r);
    }
  };
  walk('');
  const skip = buildOnly(id);
  return out.filter((f) => !skip.has(f)).sort();
}

export const isSpreadsheet = (file) => /\.xlsx$/i.test(file);
export const fileLang = (file, lang) => lang ?? LANGS[file.split('.').pop()] ?? 'text';

// One input file as Markdown: a titled code block, or a table per worksheet for a spreadsheet.
export function fileMarkdown(id, file, lang) {
  const p = path.join(EXAMPLES, id, file);
  if (isSpreadsheet(file)) return xlsxMarkdown(p, file);
  return '```' + fileLang(file, lang) + ' title="' + file + '"\n' + fs.readFileSync(p, 'utf8').trimEnd() + '\n```';
}

// MDX that shows an example's input files and then the example, for generated pages.
// The page must import Example, and ExampleFile when the example reads files.
export function exampleWithFiles(id, lines) {
  const files = inputFiles(id);
  const list = files.map((f) => `\`${f}\``);
  const lead = files.length ? `The example reads ${list.length > 1 ? `${list.slice(0, -1).join(', ')} and ${list.at(-1)}` : list[0]}:\n\n${files.map((f) => `<ExampleFile id="${id}" file="${f}" />`).join('\n\n')}\n\n` : '';
  return `${lead}<Example id="${id}"${lines ? ` lines={${lines}}` : ''} />`;
}
