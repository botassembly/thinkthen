#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { builtPages } from '../src/lib/listed-pages.mjs';
import { parseHtml, findElement, textContent, hasClass } from '../src/lib/html.mjs';
import { postLineProblems } from '../src/lib/post-line.mjs';

const elements = node => (node.kids || []).filter(k => k.tag !== '#text');
const normalized = node => textContent(node).replace(/\s+/g, ' ').trim();
function visit(node, test, acc = []) {
  if (test(node)) acc.push(node);
  for (const child of node.kids || []) visit(child, test, acc);
  return acc;
}
export function readArticles(dir) {
  return fs.readdirSync(dir).filter(f => f.endsWith('.md')).sort().map(file => {
    const source = fs.readFileSync(path.join(dir, file), 'utf8');
    const front = source.match(/^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/)?.[1] || '';
    const field = name => front.match(new RegExp(`^${name}: *(.*)$`, 'm'))?.[1]?.trim();
    const scalar = value => value?.startsWith('"') ? JSON.parse(value) : value?.replace(/^'|'$/g, '');
    let line;
    const raw = field('line');
    try { if (/^".*"$/.test(raw || '')) line = JSON.parse(raw); } catch { /* Validator reports invalid values. */ }
    return { slug: scalar(field('slug')), title: scalar(field('title')), date: scalar(field('date')), draft: field('draft') === 'true', line };
  }).filter(a => a.slug);
}

export function layoutProblems(pages, articles = []) {
  const problems = [];
  const fail = (route, code) => problems.push(`${route}: ${code}`);
  for (const article of articles) for (const code of postLineProblems(article)) fail(`/blog/${article.slug}/`, code);
  for (const page of pages.filter(p => p.kind !== 'stub')) {
    const tree = parseHtml(page.html);
    const grids = visit(tree, n => hasClass(n, 'grid'));
    grids.forEach((grid, index) => {
      const count = elements(grid).length;
      if (![2, 3, 4, 6, 9].includes(count)) fail(page.route, `grid-count grid=${index} count=${count}`);
    });
    if (page.route !== '/blog/') continue;
    if (grids.length) fail(page.route, 'blog-grid');
    const list = findElement(tree, n => n.tag === 'ol' && hasClass(n, 'blog-posts'));
    if (!list) fail(page.route, 'blog-list');
    const entries = elements(list || {}).filter(n => n.tag === 'li');
    const expected = articles.filter(a => process.env.THINKTHEN_DRAFTS === '1' || !a.draft)
      .sort((a, b) => b.date.localeCompare(a.date) || a.slug.localeCompare(b.slug));
    const hrefs = entries.map(n => findElement(n, k => k.tag === 'a')?.attrs.href);
    if (entries.length !== expected.length || expected.some(a => !hrefs.includes(`/blog/${a.slug}/`))) fail(page.route, 'blog-posts');
    if (hrefs.length === expected.length && expected.some((a, i) => hrefs[i] !== `/blog/${a.slug}/`)) fail(page.route, 'blog-order');
    for (const entry of entries) {
      const link = findElement(entry, n => n.tag === 'a');
      const post = expected.find(a => link?.attrs.href === `/blog/${a.slug}/`);
      if (!post) continue;
      const time = findElement(entry, n => n.tag === 'time');
      if (time?.attrs.datetime !== post.date || normalized(time || {}) !== post.date) fail(page.route, 'blog-date');
      if (normalized(link) !== post.title + (post.draft ? ' (draft)' : '')) fail(page.route, 'blog-title');
      if (normalized(findElement(entry, n => n.tag === 'span') || {}) !== post.line) fail(page.route, 'blog-line');
    }
  }
  return problems;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const pages = builtPages(path.resolve(process.argv[2] || 'dist'));
  const articles = readArticles(path.resolve(process.argv[3] || 'src/articles'));
  const problems = layoutProblems(pages, articles);
  if (problems.length) { console.error(`check-layout: ${problems.length} problems\n  ${problems.join('\n  ')}`); process.exit(1); }
  console.log(`check-layout: ${pages.filter(p => p.kind !== 'stub').length} pages, ${articles.length} source posts, grids and blog list passed`);
}
