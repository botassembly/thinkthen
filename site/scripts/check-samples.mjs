#!/usr/bin/env node
// Check the rules in WRITING.md that a script can see, over every example
// and every code block in the articles. The build runs it.
//
//   - No line over 60 characters, apart from the exempt lines below.
//   - A library or SQL sample that examples/REPLAY lists has its saved
//     output beside it as <sample>.out. A library sample prints its answer.
//   - No example asks for --details, except on the annotate edge-case page
//     under examples/reference/annotate/ (checklist ruling of 2026-09-30).
//   - Each function page opens with one command example of 10 to 25
//     lines, script and output together (Ian, 2026-09-28).
//   - Code carries no comments.
//   - Every page and article starts with its goal: a `// Goal:` line in
//     an Astro page, and a `goal:` field or a `<!-- Goal: -->` comment in
//     an article.
//   - An example names each answer before it uses it: no assert or print
//     acts on a ThinkThen call, a SQL call carries an alias, a Bash script
//     names an exit code before it reads it, and no answer takes a generic
//     name such as `result` or `answer`.
//   - Every example file and every fence tag is a code language or a kind
//     in NO_CALL. A mistyped fence tag fails.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { namedAnswerProblems } from './named-answers.mjs';
import { CODE_FUNCTIONS } from '../src/data/catalog.mjs';
import { readReplayList, replayLine } from './binding-proofs.mjs';

const here = fileURLToPath(import.meta.url);
const site = path.resolve(path.dirname(here), '..');
const root = path.join(site, 'examples');
const bench = path.join(root, 'beatles', 'bench');

// The one page whose examples may ask for --details.
const DETAILS_PAGE = 'reference/annotate/';

const WIDTH = 60;
// The lines a function page's example may show, script and output together.
const EXAMPLE_LINES = { min: 10, max: 25 };

// Each exempt entry names a kind of line that cannot break, and says why.
const EXEMPT = [
  {
    name: 'one JSON string',
    why: 'JSON has no way to break a string. A pretty-printed string, or a key and its string, stays on one line.',
    test: (line, file) => /\.(json|out)$/.test(file) && /^\s*("[^"]*":\s*)?"(?:[^"\\]|\\.)*",?$/.test(line),
  },
  {
    name: 'one JSON Lines record',
    why: 'A JSON Lines record is one line by definition.',
    test: (line) => /^\{\s?".*\}$/.test(line),
  },
  {
    name: 'a record filter printed whole',
    why: 'filter prints each record it keeps byte for byte.',
    test: (line, file) => file === 'how-tos/join-two-tables-by-meaning/1-join.out',
  },
  {
    name: 'the annotate clash refusal',
    why: 'thinkthen annotate prints its name-clash refusal as one line, and the page quotes it exactly.',
    test: (line, file) => file.startsWith('reference/annotate/') && file.endsWith('.out') && /^thinkthen: the record already holds `[a-z0-9_]+`, so that question cannot be appended$/.test(line),
  },
  {
    name: 'a question key',
    why: 'A question key is 64 hex characters, and cache unused reads and prints one a line.',
    test: (line, file) => file.startsWith('learn/caching/') && /^(unused )?[0-9a-f]{64}$/.test(line),
  },
  {
    name: 'the replay miss',
    why: 'thinkthen prints a replay miss as one line, and the page quotes the key sentence exactly.',
    test: (line, file) => file === 'reference/answer-cache/3-miss.out' && /^thinkthen: .*the replay folder holds no answer for question `[0-9a-f]{64}`; the key is the SHA-256 of the adapter, address, model, shared state and question as sent$/.test(line),
  },
  {
    name: 'the diff warning',
    why: 'thinkthen diff prints its warning as one line.',
    test: (line, file) => file.startsWith('beatles/diff/') && file.endsWith('.out') && line.startsWith('thinkthen: diff: warning:'),
  },
  {
    name: 'a backend diagnostic',
    why: 'thinkthen prints each warning or refusal as one line on standard error.',
    test: (line, file) => file.startsWith('install/backends/') && file.endsWith('.out') && line.startsWith('thinkthen: '),
  },
];

const COMMENT = {
  '.sh': /^\s*#(?!!)/,
  '.py': /^\s*#/,
  '.rb': /^\s*#/,
  '.R': /^\s*#/,
  '.ts': /^\s*(\/\/|\/\*)/,
  '.rs': /^\s*(\/\/|\/\*)/,
  '.c': /^\s*(\/\/|\/\*)/,
  '.cpp': /^\s*(\/\/|\/\*)/,
  '.m': /^\s*(\/\/|\/\*)/,
  '.cob': /^\s*\*>/,
  '.adb': /^\s*--/,
  '.java': /^\s*(\/\/|\/\*)/,
  '.kt': /^\s*(\/\/|\/\*)/,
  '.scala': /^\s*(\/\/|\/\*)/,
  '.cs': /^\s*(\/\/|\/\*)/,
  '.csproj': /^\s*<!--/,
  '.go': /^\s*(\/\/|\/\*)/,
  '.swift': /^\s*(\/\/(?! swift-tools-version)|\/\*)/,
  '.zig': /^\s*\/\//,
  '.php': /^\s*(\/\/|#(?!\[)|\/\*)/,
  '.dart': /^\s*(\/\/|\/\*)/,
  '.sql': /^\s*--/,
};

// Named answers (Ian, 2026-09-26). An example keeps each ThinkThen answer
// in a variable named for its meaning, then asserts on that name. The rule
// lives in named-answers.mjs, which the builder's lint rung shares.
//
// NO_CALL lists the kinds that hold no ThinkThen call: file extensions in
// the examples and fence tags in the articles. The check skips them. Every
// other kind goes to namedAnswerProblems, which throws on a language it
// does not accept. That throw fails the build, so a mistyped fence tag or
// a new kind of file never passes unread.
export const NO_CALL = new Set([
  '.json', '.jsonl', '.out', '.txt', '.exit', '.diff', '.jq', '.csproj', '.zon', '.yaml', '.toml',
  'text', 'json', 'console', 'output',
]);
const FILE_LANGUAGE = { '.sh': 'bash', '.py': 'python', '.ts': 'typescript', '.rb': 'ruby', '.R': 'r', '.rs': 'rust', '.c': 'c', '.cpp': 'cpp', '.m': 'objective-c', '.cob': 'cobol', '.adb': 'ada', '.java': 'java', '.kt': 'kotlin', '.scala': 'scala', '.cs': 'csharp', '.go': 'go', '.swift': 'swift', '.zig': 'zig', '.php': 'php', '.dart': 'dart', '.sql': 'sql' };
const FENCE_EXT = { bash: '.sh', sh: '.sh', python: '.py', ts: '.ts', typescript: '.ts', ruby: '.rb', r: '.R', rust: '.rs', c: '.c', cpp: '.cpp', 'objective-c': '.m', objc: '.m', cobol: '.cob', ada: '.adb', java: '.java', kotlin: '.kt', scala: '.scala', csharp: '.cs', cs: '.cs', go: '.go', swift: '.swift', zig: '.zig', php: '.php', dart: '.dart', sql: '.sql', json: '.json' };

function namedAnswers(label, text, kind, language, offset = 0) {
  if (NO_CALL.has(kind)) return [];
  try {
    return namedAnswerProblems(text, language).map((p) => `${label}:${offset + p.line}: ${p.message}`);
  } catch (error) {
    if (!(error instanceof TypeError) || !error.message.startsWith('unknown language ')) throw error;
    return [`${label}:${offset + 1}: ${kind} is not a kind check-samples knows. ${error.message} Kinds that hold no ThinkThen call: ${[...NO_CALL].join(', ')}.`];
  }
}

function walk(dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    // Recorded wire bodies are machine data checked by the replay smoke run,
    // not a code block a reader copies from the page.
    if (e.isDirectory()) return p === bench || ['recording', 'proposed'].includes(e.name) ? [] : walk(p);
    return [p];
  });
}

const used = new Map(EXEMPT.map((e) => [e.name, 0]));

function checkLines(label, rel, lines, ext) {
  const found = [];
  lines.forEach((line, i) => {
    const at = `${label}:${i + 1}`;
    if ([...line].length > WIDTH) {
      const exempt = EXEMPT.find((e) => e.test(line, rel));
      if (exempt) used.set(exempt.name, used.get(exempt.name) + 1);
      else found.push(`${at}: ${[...line].length} characters, over ${WIDTH}`);
    }
    if (COMMENT[ext]?.test(line)) found.push(`${at}: a comment. The sentence above the block says it.`);
  });
  return found;
}

// A C assert never calls a function whose result or side effect the
// sample uses later, since the call goes when NDEBUG is set. The check
// catches the side effect: an assert that passes an address with & as a
// function argument.
export function cAssertProblems(label, text, offset = 0) {
  return [...text.matchAll(/\bassert\(([^;]*?)\);/gs)]
    .filter((m) => /[(,]\s*&\s*\w/.test(m[1]))
    .map((m) => `${label}:${offset + text.slice(0, m.index).split('\n').length}: an assert that writes through &. Assign the call first, then assert the result.`);
}

// The code blocks in an article follow the same rules as the examples.
export function articleProblems(name, text) {
  const found = [];
  const lines = text.split('\n');
  let tag = null;
  let start = 0;
  lines.forEach((line, i) => {
    const fence = /^```\s*(\S*)/.exec(line);
    if (fence && tag === null) { tag = fence[1].toLowerCase() || 'text'; start = i + 1; return; }
    if (line.startsWith('```') && tag !== null) {
      const ext = Object.hasOwn(FENCE_EXT, tag) ? FENCE_EXT[tag] : '';
      const block = lines.slice(start, i);
      block.forEach((l, j) => {
        if ([...l].length > WIDTH && !EXEMPT.some((e) => e.test(l, ext === '.json' ? 'x.json' : `x${ext}`))) {
          found.push(`src/articles/${name}:${start + j + 1}: ${[...l].length} characters, over ${WIDTH}`);
        }
        if (COMMENT[ext]?.test(l)) found.push(`src/articles/${name}:${start + j + 1}: a comment in a code block`);
      });
      found.push(...namedAnswers(`src/articles/${name}`, block.join('\n'), tag, tag, start));
      if (ext === '.c') found.push(...cAssertProblems(`src/articles/${name}`, block.join('\n'), start));
      tag = null;
    }
  });
  return found;
}

function main() {
  const problems = [];
  for (const file of walk(root)) {
    const rel = path.relative(root, file);
    if (['SKIP', 'REPLAY', 'bindings-proof.json', 'beatles/bench-pin', 'beatles/deck-pin', 'beatles/folders.json'].includes(rel)) continue;
    const ext = path.extname(file);
    const text = fs.readFileSync(file, 'utf8').replace(/\n+$/, '');
    problems.push(...checkLines(`examples/${rel}`, rel, text.split('\n'), ext));
    problems.push(...namedAnswers(`examples/${rel}`, text, ext || '(no extension)', FILE_LANGUAGE[ext] ?? ext));
    if (ext === '.c') problems.push(...cAssertProblems(`examples/${rel}`, text));
    if (ext === '.sh' && /--details\b/.test(text) && !rel.startsWith(DETAILS_PAGE)) problems.push(`examples/${rel}: asks for --details. Only examples under examples/${DETAILS_PAGE} may.`);
  }

  for (const rel of new Set(readReplayList(root).map((line) => replayLine(line).rel))) {
    if (!fs.existsSync(path.join(root, `${rel}.out`))) problems.push(`examples/${rel}: no saved output. Every replayed sample prints its answer, and its page shows examples/${rel}.out under it.`);
  }

  const lineCount = (file) => (fs.existsSync(file) ? fs.readFileSync(file, 'utf8').replace(/\n+$/, '').split('\n').length : 0);
  for (const fn of CODE_FUNCTIONS) {
    const dir = path.join(root, 'functions', fn.name);
    const first = fs.readdirSync(dir).filter((n) => n.endsWith('.sh')).sort()[0];
    if (!first) { problems.push(`examples/functions/${fn.name}: no command example. Every function page opens with one.`); continue; }
    const script = path.join(dir, first);
    const shown = lineCount(script) + lineCount(script.replace(/\.sh$/, '.out'));
    if (shown < EXAMPLE_LINES.min || shown > EXAMPLE_LINES.max) problems.push(`examples/functions/${fn.name}/${first}: the page's example shows ${shown} lines of script and output. Keep it from ${EXAMPLE_LINES.min} to ${EXAMPLE_LINES.max}.`);
  }

  const articles = path.join(site, 'src', 'articles');
  for (const name of fs.readdirSync(articles).filter((n) => n.endsWith('.md'))) {
    problems.push(...articleProblems(name, fs.readFileSync(path.join(articles, name), 'utf8')));
  }

  // Every page says what it must communicate, before anything else.
  function astroPages(dir) {
    return fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
      const p = path.join(dir, e.name);
      return e.isDirectory() ? astroPages(p) : p.endsWith('.astro') ? [p] : [];
    });
  }
  for (const file of astroPages(path.join(site, 'src', 'pages'))) {
    const second = fs.readFileSync(file, 'utf8').split('\n')[1] || '';
    if (!second.startsWith('// Goal: ')) problems.push(`${path.relative(site, file)}: no goal. Start the front matter with // Goal: and one sentence.`);
  }
  for (const name of fs.readdirSync(articles).filter((n) => n.endsWith('.md'))) {
    const text = fs.readFileSync(path.join(articles, name), 'utf8');
    if (!/^(---\n(?:.*\n)*?goal: .+\n|<!-- Goal: .+ -->\n)/.test(text)) problems.push(`src/articles/${name}: no goal. Add a goal: field or start with <!-- Goal: -->.`);
  }

  if (problems.length) {
    console.error(`check-samples: ${problems.length} problems\n  ${problems.join('\n  ')}`);
    process.exit(1);
  }
  console.log(`check-samples: every example keeps the rules. Exempt long lines: ${[...used].map(([n, c]) => `${c} ${n}`).join(', ')}.`);
}

// The table test imports articleProblems. Only a direct run checks the site.
if (process.argv[1] && fs.realpathSync(process.argv[1]) === fs.realpathSync(here)) main();
