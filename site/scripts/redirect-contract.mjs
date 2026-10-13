// Independent compatibility inventory: every old address the site has
// served, each pointing straight at its final page or anchor. 169 entries:
// historical aliases, moved pages, and the Start, Models, Databases, Recipes,
// Learn and Beatles Bench moves.
import fs from 'node:fs';
import path from 'node:path';
import { SITE, isStub } from '../src/lib/listed-pages.mjs';
export const ALIASES = Object.freeze({
  '/surfaces': '/start/', '/backends': '/models/',
  '/tutorial': '/start/tutorial/',
  '/how-tos': '/recipes/', '/how-tos/bash': '/languages/bash/#techniques', '/guides': '/recipes/',
  '/how-tos/set-aside-bad-records': '/languages/bash/set-aside-bad-records/',
  '/recipes/set-aside-bad-records': '/languages/bash/set-aside-bad-records/',
  '/recipes/ask-your-cache-with-duckdb': '/databases/duckdb/#ask-your-saved-answers',
  ...Object.fromEntries([...['triage-a-support-inbox', 'join-two-tables-by-meaning', 'code-open-ended-survey-answers', 'screen-studies-for-a-review', 'group-alerts-into-incidents', 'rank-the-inbound-leads', 'check-an-expense-against-the-policy', 'split-a-scanned-packet-into-documents', 'screen-a-post-before-it-goes-up', 'search-youtube-transcripts-by-meaning'], ...['label-a-json-file', 'review-a-diff-by-what-it-does', 'lint-prose-for-hedging', 'fill-a-form-by-selection']].map(s => [`/how-tos/${s}`, `/recipes/${s}/`])),
  ...Object.fromEntries(['label-a-json-file', 'review-a-diff-by-what-it-does', 'lint-prose-for-hedging', 'fill-a-form-by-selection'].map(s => [`/how-tos/bash/${s}`, `/recipes/${s}/`])),
  '/shell': '/languages/bash/',
  ...Object.fromEntries(['python', 'polars', 'typescript', 'ruby', 'r', 'rust', 'c'].map(s => [`/${s}`, `/languages/${s}/`])),
  ...Object.fromEntries(['duckdb', 'sqlite', 'postgresql'].map(s => [`/${s}`, `/databases/${s}/`])),
  ...Object.fromEntries(['duckdb', 'sqlite', 'postgresql'].map(s => [`/install/${s}`, `/databases/${s}/`])),
  '/install/shell': '/languages/bash/',
  ...Object.fromEntries(['python', 'pandas', 'polars', 'typescript', 'ruby', 'r', 'rust', 'c', 'cpp', 'objective-c', 'cobol', 'ada', 'java', 'kotlin', 'scala', 'csharp', 'go', 'swift', 'zig', 'php', 'dart'].map(s => [`/install/${s}`, `/languages/${s}/`])),
  ...Object.fromEntries(['if', 'case', 'not-sure', 'threshold-band', 'while-read', 'pipeline', 'batches', 'ci-gate', 'long-lived-loop', 'judge-paragraphs', 'agent-tool-guard', 'set-aside-bad-records'].map(s => [`/how-tos/bash/${s}`, `/languages/bash/${s}/`])),
  '/blog/code-that-understands': '/blog/introducing-thinkthen/',
  '/beatles-bench': '/learn/beatles-bench/',
  '/beatles-bench/the-data': '/learn/beatles-bench/',
  '/beatles-bench/run-it-for-free': '/learn/beatles-bench/',
  '/beatles-bench/every-language': '/languages/',
  ...Object.fromEntries(['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate', 'audit', 'diff', 'blind-spots', 'rad'].map(s => [`/beatles-bench/${s}`, `/learn/beatles-bench/#${s}`])),
  '/beatles-bench/what-jev-knows': '/learn/beatles-bench/#strings',
  '/learn/beatles-bench/what-jev-knows': '/learn/beatles-bench/#strings',
  '/reference/annotate': '/functions/annotate/#edge-cases',
  '/learn/beatles-bench/bench-run': '/learn/beatles-bench/#bench-field',
  ...Object.fromEntries(['strings', 'jev', 'runs-in', 'decide', 'choose', 'tag', 'score', 'score-bands', 'filter', 'rank', 'find', 'recognize', 'relate', 'question-file', 'annotate', 'bash', 'languages', 'data', 'blind-spots', 'rad', 'audit', 'diff', 'use-cases', 'shell-uses', 'data-uses', 'retrieval', 'competitors', 'bench-field', 'backends'].map(s => [`/learn/beatles-bench/${s}`, `/learn/beatles-bench/#${s}`])),
  ...Object.fromEntries(['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate', 'question-file'].map(s => [`/reference/functions/${s}`, `/functions/${s}/`])),
  ...Object.fromEntries(['audit', 'diff', 'transform'].map(s => [`/reference/${s}`, `/functions/${s}/`])),
  '/reference/check': '/models/check/',
  '/reference/answers': '/functions/answers/', '/reference/question-sets': '/functions/question-file/#question-sets',
  '/learn': '/start/', '/learn/answers': '/functions/answers/', '/learn/question-sets': '/functions/question-file/#question-sets',
  '/reference/recording': '/start/replay/',
  '/reference': '/functions/',
  '/install': '/start/', '/install/settings': '/start/settings/', '/install/configuration': '/start/configuration/',
  '/learn/tutorial': '/start/tutorial/', '/learn/functional': '/start/pay-once/',
  '/learn/caching': '/start/replay/', '/learn/recording': '/start/replay/',
  '/learn/files': '/start/files/', '/learn/overhead': '/start/speed/',
  '/install/backends': '/models/', '/functions/check': '/models/check/',
  ...Object.fromEntries(['typesafe', 'liquid', 'openai', 'perplexity', 'openrouter', 'ollama', 'local', 'system-one', 'other-servers'].map(s => [`/install/backends/${s}`, `/models/${s}/`])),
});
export const routeFile = (dist, route) => path.join(dist, route.replace(/^\//, ''), 'index.html');
export function preserver(fixed) {
  const literal = JSON.stringify(fixed).replace(/</g, '\\u003c').replace(/\u2028/g, '\\u2028').replace(/\u2029/g, '\\u2029');
  return `<script>(() => { const fixed = ${literal}; const at = window.location.href.indexOf("#"); const target = at < 0 ? fixed : fixed.split("#")[0] + window.location.href.slice(at); window.location.replace(target); })();</script>`;
}
export function validateStub(html, alias, fixed, dist, transformed = true) {
  const fail = message => { throw new Error(`alias compatibility: ${alias}: ${message}`); };
  if (!isStub(html)) fail('missing redirect stub');
  const refresh = [...html.matchAll(/<meta http-equiv="refresh" content="0;url=([^"]+)"\s*\/?\s*>/g)];
  const canonical = [...html.matchAll(/<link rel="canonical" href="([^"]+)"\s*\/?\s*>/g)];
  const fallback = [...html.matchAll(/<a href="([^"]+)"/g)];
  if (refresh.length !== 1 || canonical.length !== 1 || fallback.length !== 1) fail('unexpected template shape');
  const target = new URL(refresh[0][1], SITE + alias);
  if (target.origin !== SITE || target.username || target.password || target.pathname + target.search + target.hash !== fixed) fail('wrong fixed destination');
  if (canonical[0][1] !== target.href || fallback[0][1] !== fixed) fail('destinations disagree');
  const file = routeFile(dist, target.pathname);
  if (!fs.existsSync(file)) fail('missing destination');
  const destination = fs.readFileSync(file, 'utf8');
  if (isStub(destination)) fail('redirect chain or cycle');
  if (!/<main(?:\s|>)/.test(destination) || /<html[^>]* data-unlisted/.test(destination)) fail('destination is not listed content');
  if (target.hash) {
    const id = decodeURIComponent(target.hash.slice(1));
    const ids = [...destination.matchAll(/\s(?:id|name)="([^"]+)"/g)].map(m => m[1]);
    if (!ids.includes(id)) fail('missing destination anchor');
  }
  if (!/<meta name="robots" content="noindex"/.test(html) || /<main(?:\s|>)|data-pagefind-body/.test(html)) fail('stub publication shape');
  if (transformed) {
    if (!html.includes(preserver(fixed))) fail('missing fragment preserver');
    const without = html.replace(`<noscript>${refresh[0][0]}</noscript>`, '');
    if (isStub(without) || without === html || (html.match(/<script>/g) || []).length !== 1) fail('active refresh or unexpected script');
  } else if (/<script|<noscript/.test(html)) fail('unexpected untransformed shape');
  return refresh[0][0];
}

export const validateCompatibility = validateStub;
