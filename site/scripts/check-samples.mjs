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
//   - An example names each answer before it uses it: no assert or print
//     acts on a ThinkThen call, a SQL call carries an alias, a Bash script
//     names an exit code before it reads it, and no answer takes a generic
//     name such as `result` or `answer`.

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

// Named answers (Ian, 2026-09-26). An example keeps each ThinkThen answer
// in a variable named for its meaning, then asserts on that name.
const FNS = 'decide|choose|score|tag|filter|rank|find|annotate|recognize|relate';
const CALL = {
  '.py': new RegExp(`\\btt\\s*\\.(${FNS})\\(`),
  '.ts': new RegExp(`\\btt\\s*\\.(${FNS})\\(`),
  '.rs': new RegExp(`\\btt\\s*\\.(${FNS})\\(`),
  '.rb': new RegExp(`\\bThinkThen\\.(${FNS})\\(`),
  '.R': new RegExp(`\\btt_(${FNS})\\(`),
  '.c': new RegExp(`\\bthinkthen_(call|${FNS})\\(`),
  '.sh': new RegExp(`\\bthinkthen\\s+(${FNS})\\b`),
};
const SQL_CALL = new RegExp(`\\bthinkthen_(${FNS}|probability|relations)\\s*\\(`, 'g');
const GENERIC = new Set([
  'result', 'results', 'answer', 'answers', 'output', 'outputs', 'out',
  'res', 'ret', 'response', 'responses', 'value', 'values', 'data', 'found',
  'kept', 'ranked', 'pick', 'picks', 'hits', 'matches', 'scores', 'tags',
  'edges', 'forms', 'tmp', 'x', 'y', 'n', 'r', 'v',
]);
const ASSIGN = {
  '.py': /^\s*([A-Za-z_]\w*)\s*=(?!=)/,
  '.rb': /^\s*([A-Za-z_]\w*)\s*=(?!=)/,
  '.R': /^\s*([A-Za-z_.][\w.]*)\s*(?:<-|=(?!=))/,
  '.ts': /^\s*(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*=/,
  '.rs': /^\s*(?:let\s+(?:mut\s+)?([A-Za-z_]\w*)\s*(?::[^=]+)?=|for\s+\(?([\w, ]+?)\)?\s+in\b)/,
  '.c': /^\s*(?:const\s+)?[A-Za-z_]\w*\s*\*?\s*([A-Za-z_]\w*)\s*(?:=|;)/,
};
const ASSERT = {
  '.py': /^\s*assert\b/,
  '.ts': /^\s*assert(\.\w+)?\(/,
  '.rb': /\braise\s+(unless|if)\b/,
  '.R': /^\s*stopifnot\(/,
  '.rs': /^\s*(debug_)?assert(_eq|_ne)?!\(/,
  '.c': /^\s*assert\(/,
};
const SQL_WORDS = new Set(['from', 'where', 'order', 'group', 'is', 'and', 'or', 'not', 'in', 'desc', 'asc', 'limit', 'on', 'join', 'union', 'having', 'select', 'with']);

// Blank out string literals, so brackets and words inside them do not count.
function unquote(text, ext) {
  const single = ['.py', '.rb', '.R', '.ts', '.sql'].includes(ext);
  let out = '';
  let quote = null;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (quote) {
      if (c === '\\' && quote === '"') { i += 1; continue; }
      if (c === quote) { out += c; quote = null; }
      else if (c === '\n') out += c;
      continue;
    }
    if (c === '"' || (single && c === "'")) quote = c;
    out += c;
  }
  return out;
}

// The lines from `start` to the end of the statement that begins there.
function statement(lines, start, ext) {
  let depth = 0;
  for (let j = start; j < lines.length; j++) {
    const line = lines[j];
    depth += (line.match(/[([]/g) || []).length - (line.match(/[)\]]/g) || []).length;
    if (ext === '.rb') depth += (line.match(/\bdo\b/g) || []).length - (line.match(/^\s*end\b/g) || []).length;
    const tail = line.trimEnd();
    const more = ['.ts', '.rs', '.c'].includes(ext)
      ? !tail.endsWith(';') && !tail.endsWith('{') && !tail.endsWith('}')
      : /(\\|[+,=.]|<-|\|\||&&)$/.test(tail);
    if (depth <= 0 && !more) return lines.slice(start, j + 1).join('\n');
  }
  return lines.slice(start).join('\n');
}

function generic(name) {
  return GENERIC.has(name.toLowerCase());
}

function namedAnswers(label, text, ext) {
  const found = [];
  if (ext === '.sql') {
    const bare = unquote(text, ext);
    for (const m of bare.matchAll(SQL_CALL)) {
      let depth = 0;
      let i = m.index + m[0].length - 1;
      for (; i < bare.length; i++) {
        if (bare[i] === '(') depth += 1;
        else if (bare[i] === ')' && --depth === 0) break;
      }
      // A call wrapped in another function, such as unnest(), closes that first.
      const wraps = (/((?:\w+\s*\(\s*)+)$/.exec(bare.slice(0, m.index))?.[1].match(/\(/g) || []).length;
      let rest = bare.slice(i + 1);
      for (let w = 0; w < wraps; w++) rest = rest.replace(/^\s*\)/, '');
      const alias = /^\s*(?:AS\s+)?([A-Za-z_]\w*)/i.exec(rest);
      const line = bare.slice(0, m.index).split('\n').length;
      if (!alias || SQL_WORDS.has(alias[1].toLowerCase())) {
        found.push(`${label}:${line}: thinkthen_${m[1]} has no name. Give its answer an alias named for its meaning, and filter or sort on that name.`);
      } else if (generic(alias[1])) {
        found.push(`${label}:${line}: the answer is named ${alias[1]}. Name it for its meaning.`);
      }
    }
    return found;
  }
  const call = CALL[ext];
  if (!call) return found;
  if (ext === '.sh') {
    const lines = [];
    let until = null;
    for (const line of text.split('\n')) {
      if (until) { lines.push(''); if (line.trim() === until) until = null; continue; }
      const doc = /<<-?\s*'?(\w+)'?/.exec(line);
      if (doc) until = doc[1];
      lines.push(line.replace(/'[^']*'/g, "''"));
    }
    lines.forEach((line, i) => {
      const at = `${label}:${i + 1}`;
      const capture = /^\s*([A-Za-z_]\w*)=\$\(/.exec(line);
      const code = /^\s*([A-Za-z_]\w*)=\$\?/.exec(line);
      const fn = /^\s*([A-Za-z_]\w*)\s*\(\)\s*\{/.exec(line);
      if (capture || fn) {
        let depth = 0;
        let j = i;
        for (; j < lines.length; j++) {
          depth += (lines[j].match(fn ? /\{/g : /\(/g) || []).length - (lines[j].match(fn ? /\}/g : /\)/g) || []).length;
          if (depth <= 0) break;
        }
        const name = (capture || fn)[1];
        if (call.test(lines.slice(i, j + 1).join('\n')) && generic(name)) found.push(`${at}: the answer is named ${name}. Name it for its meaning.`);
      }
      if (code && generic(code[1])) found.push(`${at}: the exit code is named ${code[1]}. Name it for its meaning.`);
      if (/^\s*(case|test|if|while|\[)\b.*\$\?/.test(line) || /^\s*\[\[?\s.*\$\?/.test(line)) {
        found.push(`${at}: reads $? directly. Name the exit code first, such as refund_code=$?.`);
      }
      if (/^\s*(test|\[\[?|echo|printf)\b/.test(line) && /\$\(/.test(line)) {
        const span = statement(lines, i, '.py');
        if (call.test(span)) found.push(`${at}: an assert or print acts on a thinkthen call. Name the answer first.`);
      }
    });
    return found;
  }
  const lines = unquote(text, ext).split('\n');
  const assign = ASSIGN[ext];
  const assert = ASSERT[ext];
  lines.forEach((line, i) => {
    const at = `${label}:${i + 1}`;
    if ((assert.test(line) || PRINT.test(line)) && call.test(statement(lines, i, ext))) {
      found.push(`${at}: an assert or print acts on a ThinkThen call. Name the answer first, then assert on the name.`);
    }
    const a = assign.exec(line);
    if (a && call.test(statement(lines, i, ext))) {
      for (const name of (a[1] || a[2]).split(',').map((s) => s.trim()).filter(Boolean)) {
        if (generic(name)) found.push(`${at}: the answer is named ${name}. Name it for its meaning.`);
      }
    }
    if (ext === '.c' && call.test(line)) {
      for (const [, name] of statement(lines, i, ext).matchAll(/&\s*([A-Za-z_]\w*)/g)) {
        if (generic(name)) found.push(`${at}: the answer is named ${name}. Name it for its meaning.`);
      }
    }
  });
  return found;
}

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
  problems.push(...namedAnswers(`examples/${rel}`, text, ext));
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
      problems.push(...namedAnswers(`src/articles/${name}:${start}+`, block.join('\n'), ext));
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
