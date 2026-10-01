#!/usr/bin/env node
// Fail the build when a built page's prose uses a word the site has
// retired. WRITING.md, "Pages", gives the words: "not sure" for the third
// answer, plain words for a wrong yes and a missed yes, and no status word.
//
// Prose is the page's title, its meta description and the text of its
// body. Code, scripts and styles are dropped with their contents, then
// every tag. Machine words such as `unsure` live in code and pass.
//
// ALLOWED admits a hit the rules cannot tell apart. Each entry names the
// page, the word, a few words beside the hit, and the reason. Every entry
// must still match a hit.

import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { builtPages } from '../src/lib/listed-pages.mjs';

const GAP = String.raw`(?:\s|&[#\w]+;)+`;
const word = (w) => w.split(' ').join(GAP);

export const RULES = [
  ['unsure', 'say "not sure"'],
  ['unresolved', 'say "not sure"'],
  ['false positive', 'say "wrong yes"'],
  ['false negative', 'say "missed yes"'],
  ['coming soon', 'a status word'],
  ['alpha', 'a status word'],
  ['beta', 'a status word'],
  ['planned', 'a status word'],
  ['plan for 0.1', 'a status word'],
  ['preview', 'a status word; "Plan preview" and "Prune preview" are setting names'],
].map(([w, why]) => {
  const body = word(w).replace('.', String.raw`\.`);
  return { word: w, why, re: new RegExp(String.raw`(?<![&\w])${body}(?!\w)`, 'gi') };
});

// The setting names keep their capital, so only "Plan preview" and
// "Prune preview" pass.
const SETTING_NAME = new RegExp(String.raw`\b(?:Plan|Prune)${GAP}$`);

const ALLOWED = [
  ['/install/settings/', 'planned', 'whole-input records', 'Settings renders specification/settings.md word for word. "planned" there means the requests a run would send.'],
];

export function prose(html) {
  const title = /<title>([\s\S]*?)<\/title>/i.exec(html)?.[1] ?? '';
  const description = /<meta name="description" content="([^"]*)"/i.exec(html)?.[1] ?? '';
  const body = /<body\b[^>]*>([\s\S]*)<\/body>/i.exec(html)?.[1] ?? html;
  const text = body
    .replace(/<(pre|code|script|style)\b[\s\S]*?<\/\1>/gi, ' ')
    .replace(/<[^>]*>/g, ' ');
  return [title, description, text].join(' \n ').replace(/\s+/g, ' ');
}

export function wordHits(html) {
  const text = prose(html);
  const hits = [];
  for (const rule of RULES) {
    for (const m of text.matchAll(rule.re)) {
      if (rule.word === 'preview' && SETTING_NAME.test(text.slice(0, m.index))) continue;
      const context = text.slice(Math.max(0, m.index - 60), m.index + m[0].length + 60).trim();
      hits.push({ word: rule.word, why: rule.why, context });
    }
  }
  return hits;
}

function main() {
  const dist = path.resolve(process.argv[2] || 'dist');
  const pages = builtPages(dist).filter((p) => p.kind !== 'stub');
  const faults = [];
  const used = new Set();
  let allowed = 0;
  for (const page of pages) {
    for (const hit of wordHits(page.html)) {
      const i = ALLOWED.findIndex(([route, w, near]) => route === page.route && w === hit.word && hit.context.includes(near));
      if (i >= 0) {
        used.add(i);
        allowed += 1;
      } else {
        faults.push(`${page.route} uses "${hit.word}" (${hit.why}): "${hit.context}"`);
      }
    }
  }
  ALLOWED.forEach(([route, w, near], i) => {
    if (!used.has(i)) faults.push(`ALLOWED names "${w}" near "${near}" on ${route}, and no hit matches it`);
  });
  if (faults.length) {
    for (const f of faults) console.error(`check-words: ${f}`);
    process.exit(1);
  }
  console.log(`check-words: ${pages.length} pages, no retired word in prose, ${allowed} allowed`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) main();
