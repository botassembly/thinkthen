#!/usr/bin/env node
// Check the rules in WRITING.md that a script can see, over every example
// and every code block in the articles. The build runs it.
//
//   - No line over 60 characters, apart from the exempt lines below.
//   - A library sample asserts. It never prints.
//   - A script that asks for --details shows it whole: it ends in jq .
//   - Code carries no comments.
//   - Every page and article starts with its goal: a `// Goal:` line in
//     an Astro page, and a `goal:` field or a `<!-- Goal: -->` comment in
//     an article.

import fs from 'node:fs';
import path from 'node:path';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const root = path.join(site, 'examples');
const bench = path.join(root, 'beatles', 'bench');
const WIDTH = 60;

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
    name: 'the diff warning',
    why: 'thinkthen diff prints its warning as one line.',
    test: (line, file) => file.startsWith('beatles/diff/') && file.endsWith('.out') && line.startsWith('thinkthen: diff: warning:'),
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
  '.sql': /^\s*--/,
};
const PRINT = /\b(print\(|console\.log\(|puts\b|println!|printf\(|cat\()/;
const LIBRARY = new Set(['.py', '.rb', '.R', '.ts', '.rs', '.c']);

function walk(dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) return p === bench ? [] : walk(p);
    return [p];
  });
}

const problems = [];
const used = new Map(EXEMPT.map((e) => [e.name, 0]));

function checkLines(label, rel, lines, ext) {
  lines.forEach((line, i) => {
    const at = `${label}:${i + 1}`;
    if ([...line].length > WIDTH) {
      const exempt = EXEMPT.find((e) => e.test(line, rel));
      if (exempt) used.set(exempt.name, used.get(exempt.name) + 1);
      else problems.push(`${at}: ${[...line].length} characters, over ${WIDTH}`);
    }
    if (COMMENT[ext]?.test(line)) problems.push(`${at}: a comment. The sentence above the block says it.`);
    if (LIBRARY.has(ext) && PRINT.test(line)) problems.push(`${at}: a print. Assert the answer instead.`);
  });
}

for (const file of walk(root)) {
  const rel = path.relative(root, file);
  if (['SKIP', 'beatles/BENCH', 'beatles/folders.json'].includes(rel)) continue;
  const ext = path.extname(file);
  const text = fs.readFileSync(file, 'utf8').replace(/\n+$/, '');
  checkLines(`examples/${rel}`, rel, text.split('\n'), ext);
  if (ext === '.sh' && /--details\b/.test(text)) {
    const last = text.split('\n').at(-1).trim();
    if (last !== 'jq .') problems.push(`examples/${rel}: asks for --details and does not end in jq . Show the details whole.`);
  }
}

// The code blocks in the articles follow the same rules.
const articles = path.join(site, 'src', 'articles');
for (const name of fs.readdirSync(articles).filter((n) => n.endsWith('.md'))) {
  const lines = fs.readFileSync(path.join(articles, name), 'utf8').split('\n');
  let lang = null;
  let start = 0;
  lines.forEach((line, i) => {
    const fence = /^```(\w*)/.exec(line);
    if (fence && lang === null) { lang = fence[1] || 'text'; start = i + 1; return; }
    if (line.startsWith('```') && lang !== null) {
      const ext = { bash: '.sh', sh: '.sh', python: '.py', ts: '.ts', typescript: '.ts', sql: '.sql', json: '.json' }[lang] || '';
      const block = lines.slice(start, i);
      block.forEach((l, j) => {
        if ([...l].length > WIDTH && !EXEMPT.some((e) => e.test(l, ext === '.json' ? 'x.json' : `x${ext}`))) {
          problems.push(`src/articles/${name}:${start + j + 1}: ${[...l].length} characters, over ${WIDTH}`);
        }
        if (COMMENT[ext]?.test(l)) problems.push(`src/articles/${name}:${start + j + 1}: a comment in a code block`);
        if (LIBRARY.has(ext) && PRINT.test(l)) problems.push(`src/articles/${name}:${start + j + 1}: a print. Assert the answer instead.`);
      });
      lang = null;
    }
  });
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
