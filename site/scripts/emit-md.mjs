#!/usr/bin/env node
// A Markdown twin of every page, at the same address with .md added, plus
// llms.txt. Both are read from the built pages, so a twin cannot drift from
// the page it twins.

import fs from 'node:fs';
import path from 'node:path';

const DIST = path.join(process.cwd(), 'dist');
const VOID = new Set(['br', 'img', 'meta', 'link', 'input', 'hr', 'source', 'area', 'col']);
const SKIP = new Set(['button', 'script', 'style', 'svg']);
// The Beatles Bench side list and phone menu repeat on every page. The
// Settings page's find box and its no-match line work only in a browser.
const SKIP_CLASS = ['bench-side', 'bench-menu', 'settings-filter', 'settings-none'];

// ------------------------------------------------------------------ the tree

function parse(html) {
  const root = { tag: '#root', attrs: {}, kids: [] };
  const stack = [root];
  const tagRe = /<(\/?)([a-zA-Z][a-zA-Z0-9-]*)((?:"[^"]*"|'[^']*'|[^>"'])*)>/g;
  let at = 0;
  let m;
  while ((m = tagRe.exec(html))) {
    if (m.index > at) text(stack, html.slice(at, m.index));
    at = tagRe.lastIndex;
    const [, closing, tag, rawAttrs] = m;
    const name = tag.toLowerCase();
    if (closing) {
      for (let i = stack.length - 1; i > 0; i -= 1) {
        if (stack[i].tag === name) { stack.length = i; break; }
      }
      continue;
    }
    const node = { tag: name, attrs: attrsOf(rawAttrs), kids: [] };
    stack[stack.length - 1].kids.push(node);
    if (!VOID.has(name) && !/\/\s*$/.test(rawAttrs)) stack.push(node);
  }
  if (at < html.length) text(stack, html.slice(at));
  return root;
}

function text(stack, raw) {
  if (!raw) return;
  stack[stack.length - 1].kids.push({ tag: '#text', text: decode(raw) });
}

function attrsOf(raw) {
  const out = {};
  const re = /([a-zA-Z_:][-a-zA-Z0-9_:.]*)(?:\s*=\s*("([^"]*)"|'([^']*)'|([^\s"'>]+)))?/g;
  let m;
  while ((m = re.exec(raw))) out[m[1].toLowerCase()] = decode(m[3] ?? m[4] ?? m[5] ?? '');
  return out;
}

function decode(s) {
  return s
    .replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&quot;/g, '"')
    .replace(/&#x([0-9a-f]+);/gi, (_, h) => String.fromCodePoint(parseInt(h, 16)))
    .replace(/&#([0-9]+);/g, (_, d) => String.fromCodePoint(Number(d))).replace(/&nbsp;/g, ' ')
    .replace(/&middot;/g, '·').replace(/&amp;/g, '&');
}

function find(node, test) {
  if (test(node)) return node;
  for (const kid of node.kids || []) {
    const hit = find(kid, test);
    if (hit) return hit;
  }
  return null;
}

// ------------------------------------------------------------------- render

const classOf = (n) => (n.attrs?.class || '').split(/\s+/);
const skipped = (n) => classOf(n).some((c) => SKIP_CLASS.includes(c));

function inline(node) {
  if (node.tag === '#text') return node.text.replace(/\s+/g, ' ');
  if (SKIP.has(node.tag)) return '';
  const inner = (node.kids || []).map(inline).join('');
  switch (node.tag) {
    case 'code': return inner.trim() ? '`' + inner.trim() + '`' : '';
    case 'b': case 'strong': return inner.trim() ? '**' + inner.trim() + '**' : '';
    case 'em': case 'i': return inner.trim() ? '*' + inner.trim() + '*' : '';
    case 'a': {
      const href = node.attrs.href || '';
      return inner.trim() ? `[${inner.trim()}](${href})` : '';
    }
    case 'br': return '\n';
    default: return inner;
  }
}

// Text exactly as it stands, for a code block.
function raw(node) {
  if (node.tag === '#text') return node.text;
  if (SKIP.has(node.tag) || skipped(node)) return '';
  return (node.kids || []).map(raw).join('');
}

function tidy(s) {
  return s.replace(/[ \t]+/g, ' ').replace(/ ([,.;:)])/g, '$1').replace(/\( /g, '(').trim();
}

function blocks(node, out) {
  for (const kid of node.kids || []) {
    if (kid.tag === '#text') continue;
    if (SKIP.has(kid.tag) || skipped(kid)) continue;
    const cls = classOf(kid);

    if (cls.includes('tabs')) continue;
    if (kid.tag === 'h1') { out.push('# ' + tidy(inline(kid))); continue; }
    if (kid.tag === 'h2') { out.push('## ' + tidy(inline(kid))); continue; }
    if (kid.tag === 'h3') { out.push('### ' + tidy(inline(kid))); continue; }
    if (kid.tag === 'p') {
      const line = tidy(inline(kid));
      if (line) out.push(line);
      continue;
    }
    if (kid.tag === 'ul' || kid.tag === 'ol') {
      const rows = (kid.kids || []).filter((k) => k.tag === 'li')
        .map((li, i) => (kid.tag === 'ol' ? `${i + 1}. ` : '- ') + tidy(inline(li)));
      if (rows.length) out.push(rows.join('\n'));
      continue;
    }
    if (kid.tag === 'pre') {
      const body = raw(kid).replace(/`/g, '');
      out.push('```\n' + body.replace(/\n+$/, '') + '\n```');
      continue;
    }
    if (kid.tag === 'table') {
      const rows = [];
      const walk = (n) => {
        for (const k of n.kids || []) {
          if (k.tag === 'tr') {
            rows.push((k.kids || []).filter((c) => c.tag === 'th' || c.tag === 'td')
              .map((c) => tidy(inline(c)).replace(/\|/g, '\\|')));
          } else walk(k);
        }
      };
      walk(kid);
      if (rows.length) {
        const width = Math.max(...rows.map((r) => r.length));
        const pad = (r) => r.concat(Array(width - r.length).fill(''));
        const lines = ['| ' + pad(rows[0]).join(' | ') + ' |',
          '| ' + Array(width).fill('---').join(' | ') + ' |'];
        for (const r of rows.slice(1)) lines.push('| ' + pad(r).join(' | ') + ' |');
        out.push(lines.join('\n'));
      }
      continue;
    }
    if (kid.tag === 'dl') {
      const rows = [];
      const walk = (n) => {
        for (const k of n.kids || []) {
          if (k.tag === 'dt') rows.push('- **' + tidy(inline(k)) + '**:');
          else if (k.tag === 'dd' && rows.length) rows[rows.length - 1] += ' ' + tidy(inline(k));
          else walk(k);
        }
      };
      walk(kid);
      if (rows.length) out.push(rows.join('\n'));
      continue;
    }
    if (kid.tag === 'div' && cls.includes('note')) {
      out.push('> ' + tidy(inline(kid)));
      continue;
    }
    if (kid.tag === 'div' && cls.includes('caption')) {
      const parts = (kid.kids || [])
        .filter((k) => k.tag !== '#text' && !SKIP.has(k.tag))
        .map((k) => tidy(inline(k)))
        .filter(Boolean);
      if (parts.length) out.push('*' + parts.join('. ') + '*');
      continue;
    }
    if (kid.tag === 'div' && (cls.includes('tile') || cls.includes('fact') || cls.includes('outcome'))) {
      out.push('- ' + tidy(inline(kid)));
      continue;
    }
    if (kid.tag === 'a' && cls.includes('tile')) {
      const part = (tag) => tidy(inline(find(kid, (n) => n.tag === tag) || { tag: '#text', text: '' }));
      out.push(`- [${part('b').replace(/\*/g, '')}](${kid.attrs.href || ''}): ${part('span')}`);
      continue;
    }
    blocks(kid, out);
  }
  return out;
}

// --------------------------------------------------------------------- walk

function pages(dir, acc = []) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) pages(full, acc);
    else if (entry.name === 'index.html') acc.push(full);
  }
  return acc;
}

const found = pages(DIST).sort();
const list = [];

for (const file of found) {
  const html = fs.readFileSync(file, 'utf8');
  const route = '/' + path.relative(DIST, path.dirname(file)).split(path.sep).filter(Boolean).join('/');
  const url = route === '/' ? '/' : route + '/';
  const tree = parse(html);
  const main = find(tree, (n) => n.tag === 'main');
  // A moved page leaves a redirect behind. It gets no twin.
  if (!main && /http-equiv="refresh"/.test(html)) continue;
  // The search page is never listed, so it gets no twin either.
  if (/<html[^>]* data-unlisted/.test(html)) continue;
  if (!main) throw new Error(`${file}: no main`);
  const titleNode = find(tree, (n) => n.tag === 'title');
  const title = titleNode ? tidy(inline(titleNode)) : url;
  const body = blocks(main, []).join('\n\n').replace(/\n{3,}/g, '\n\n');
  const target = route === '/' ? path.join(DIST, 'index.md') : path.join(DIST, route.slice(1) + '.md');
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, body + '\n');
  const first = body.split('\n').find((l) => l && !l.startsWith('#')) || '';
  list.push({ url, md: route === '/' ? '/index.md' : route + '.md', title, blurb: first.slice(0, 160) });
}

const llms = [
  '# ThinkThen',
  '',
  '> ThinkThen: code that knows what you mean. Simple functions that give your software the judgment to handle whatever comes its way. Ten functions, in your scripts, your programs, and your queries.',
  '',
  'Every page on this site has a Markdown twin at the same address with .md added.',
  '',
  '## Pages',
  '',
  ...list.map((p) => `- [${p.title}](https://thinkthen.dev${p.md}): ${p.blurb}`),
  '',
];
fs.writeFileSync(path.join(DIST, 'llms.txt'), llms.join('\n'));

console.log(`wrote ${list.length} Markdown twins and llms.txt`);
