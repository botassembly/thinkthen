// Fail the build when a built page shows a code block with no colour.
//
// src/lib/code.mjs draws every block. It marks each pre with data-lang: a
// language, "output" for what a command printed, or "text" for a .txt
// file. This check reads every built page and fails when:
//
// - a pre has no data-lang, as a raw Markdown fence would;
// - a pre has data-lang="text" and did not come from a .txt file;
// - a pre keeps Shiki's tabindex or an inline style;
// - an output or text pre carries token colour;
// - a pre in a language has no coloured token and EXPECTED_PLAIN does not
//   name it;
// - an EXPECTED_PLAIN entry no longer names a plain block.

import fs from 'node:fs';
import path from 'node:path';

const dist = path.resolve(process.argv[2] || 'dist');

const LANGS = new Set(['bash', 'json', 'jsonl', 'jq', 'diff', 'python', 'typescript', 'ruby', 'r', 'rust', 'c', 'sql']);

// Blocks whose code holds no token the theme colours, keyed by page and
// the block's first line, with the reason. Every block on the site has a
// coloured token today, since shell command names take colour, so the list
// is empty. An entry must name a block that is still plain.
const EXPECTED_PLAIN = new Map([]);

const pages = [];
const walk = (dir) => {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p);
    else if (e.name.endsWith('.html')) pages.push(p);
  }
};
walk(dist);

const decode = (s) => s.replaceAll('&quot;', '"').replaceAll('&lt;', '<').replaceAll('&gt;', '>')
  .replaceAll('&#x3C;', '<').replaceAll('&amp;', '&');
const attr = (tag, name) => {
  const m = new RegExp(`\\s${name}(?:="([^"]*)")?(?=[\\s>])`).exec(tag);
  return m ? decode(m[1] ?? '') : null;
};

const faults = [];
const counts = { coloured: 0, output: 0, text: 0, plain: 0 };
const seenPlain = new Set();

for (const file of pages) {
  const page = path.relative(dist, file);
  const html = fs.readFileSync(file, 'utf8');
  for (const m of html.matchAll(/(<pre\b[^>]*>)([\s\S]*?)<\/pre>/g)) {
    const [, tag, body] = m;
    const lang = attr(tag, 'data-lang');
    const first = (attr(tag, 'data-copy') ?? decode(body.replace(/<[^>]+>/g, ''))).split('\n')[0].trim();
    const where = `${page}: "${first.slice(0, 60)}"`;
    const coloured = /style="[^"]*color:var\(--code-(?!ink\))/.test(body);
    if (attr(tag, 'tabindex') !== null) faults.push(`${where} keeps a tabindex`);
    if (attr(tag, 'style') !== null) faults.push(`${where} keeps an inline style`);
    if (lang === null) {
      faults.push(`${where} has no data-lang. Draw it with src/lib/code.mjs, not a Markdown fence.`);
    } else if (lang === 'output' || lang === 'text') {
      if (lang === 'text' && !(attr(tag, 'data-file') ?? '').endsWith('.txt')) faults.push(`${where} shows as text and is not a .txt file`);
      if (/style=/.test(body)) faults.push(`${where} is ${lang} and carries token colour`);
      counts[lang] += 1;
    } else if (!LANGS.has(lang)) {
      faults.push(`${where} names an unknown language "${lang}"`);
    } else if (coloured) {
      counts.coloured += 1;
    } else if (EXPECTED_PLAIN.has(`${page} ${first}`)) {
      seenPlain.add(`${page} ${first}`);
      counts.plain += 1;
    } else {
      faults.push(`${where} is ${lang} and has no coloured token`);
    }
  }
}
for (const key of EXPECTED_PLAIN.keys()) {
  if (!seenPlain.has(key)) faults.push(`EXPECTED_PLAIN names ${key}, and no plain block matches it`);
}

if (faults.length) {
  for (const f of faults) console.error(`check-code: ${f}`);
  process.exit(1);
}
console.log(`check-code: ${pages.length} pages. ${counts.coloured} coloured blocks, ${counts.output} output, ${counts.text} text, ${counts.plain} expected plain`);
