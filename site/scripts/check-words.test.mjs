#!/usr/bin/env node
// Edge cases for the prose word check in check-words.mjs. Each row gives
// a page's HTML and the words the check must find in its prose.

import { wordHits, RULES } from './check-words.mjs';

const page = (body, head = '') => `<html><head>${head}</head><body>${body}</body></html>`;

const CASES = [
  ...RULES.map((r) => [`${r.word} in a paragraph`, page(`<p>It is ${r.word} here.</p>`), [r.word]]),
  ['a table cell', page('<table><tr><td>beta</td></tr></table>'), ['beta']],
  ['a title', page('<p>ok</p>', '<title>Alpha build</title>'), ['alpha']],
  ['a meta description', page('<p>ok</p>', '<meta name="description" content="When unsure, ask.">'), ['unsure']],
  ['an entity as the gap', page('<p>Coming&nbsp;soon.</p>'), ['coming soon']],
  ['a line break as the gap', page('<p>a false\n  positive</p>'), ['false positive']],
  ['pre', page('<pre data-lang="output">{"answer":"unsure","beta":1}</pre>'), []],
  ['code', page('<p>The label <code>unsure</code> counts.</p>'), []],
  ['a class name', page('<span class="exit unsure">3 not sure</span>'), []],
  ['a link address', page('<a href="/how-tos/2-unsure/">not sure</a>'), []],
  ['a script', page('<script>const planned = 1;</script>'), []],
  ['Plan preview', page('<li>Plan preview</li>'), []],
  ['Prune preview', page('<li>Prune&nbsp;preview</li>'), []],
  ['a bare preview', page('<p>a different preview</p>'), ['preview']],
  ['alphabet and betas', page('<p>The alphabet holds betas and unsureness.</p>'), []],
  ['an entity named beta', page('<p>&beta; is a letter.</p>'), []],
];

const failures = [];
for (const [name, html, expected] of CASES) {
  const got = wordHits(html).map((h) => h.word);
  if (JSON.stringify(got) !== JSON.stringify(expected)) {
    failures.push(`${name}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(got)}`);
  }
}

if (failures.length) {
  console.error(`check-words: ${failures.length} of ${CASES.length} cases fail\n  ${failures.join('\n  ')}`);
  process.exit(1);
}
console.log(`check-words: all ${CASES.length} cases keep the rule.`);
