// Independent compatibility inventory: 42 historical aliases and 19 moved pages.
import fs from 'node:fs';
import path from 'node:path';
import { SITE, isStub } from '../src/lib/listed-pages.mjs';
export const ALIASES = Object.freeze({
  '/surfaces': '/install/', '/backends': '/install/backends/',
  '/tutorial': '/learn/tutorial/', '/recipes': '/how-tos/bash/',
  ...Object.fromEntries(['label-a-json-file', 'review-a-diff-by-what-it-does', 'lint-prose-for-hedging', 'fill-a-form-by-selection'].map(s => [`/how-tos/${s}`, `/how-tos/bash/${s}/`])),
  ...Object.fromEntries(['shell', 'python', 'polars', 'typescript', 'ruby', 'r', 'rust', 'c', 'duckdb', 'sqlite', 'postgresql'].map(s => [`/${s}`, `/install/${s}/`])),
  '/blog/code-that-understands': '/blog/introducing-thinkthen/',
  '/beatles-bench': '/learn/beatles-bench/',
  '/beatles-bench/the-data': '/learn/beatles-bench/',
  '/beatles-bench/run-it-for-free': '/learn/beatles-bench/',
  '/beatles-bench/every-language': '/install/',
  ...Object.fromEntries(['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate', 'audit', 'diff', 'blind-spots', 'rad'].map(s => [`/beatles-bench/${s}`, `/learn/beatles-bench/${s}/`])),
  '/beatles-bench/what-jev-knows': '/learn/beatles-bench/strings/',
  '/learn/beatles-bench/what-jev-knows': '/learn/beatles-bench/strings/',
  '/reference/annotate': '/functions/annotate/#edge-cases',
  '/learn/beatles-bench/bench-run': '/learn/beatles-bench/bench-field/',
  ...Object.fromEntries(['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate', 'question-file'].map(s => [`/reference/functions/${s}`, `/functions/${s}/`])),
  ...Object.fromEntries(['audit', 'diff', 'check', 'transform'].map(s => [`/reference/${s}`, `/functions/${s}/`])),
  ...Object.fromEntries(['answers', 'question-sets', 'recording'].map(s => [`/reference/${s}`, `/learn/${s}/`])),
  '/reference': '/functions/',
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
