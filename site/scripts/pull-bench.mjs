#!/usr/bin/env node
// Copy the Beatles Bench pages into src/pages/beatles-bench/ from one pinned
// commit of the bench.
//
//   BEATLES_BENCH=path/to/beatles-bench node scripts/pull-bench.mjs
//
// The bench owns every page but one. Its docs/README.md lists them in order,
// and its tests run every command on them against the committed answers. This
// script runs none of them. It refuses a checkout at any other commit than
// src/data/bench/PIN, or one with local changes.
//
// For each listed page it:
//   - moves the title and the first image into the front matter,
//   - copies each image it links into public/beatles-bench/img/,
//   - turns a link to another listed page into its site route,
//   - turns any other relative link into a GitHub link at the pinned commit,
//   - turns a thinkthen.dev link into a site link.
// It writes src/data/bench/pages.json and a manifest with the SHA-256 of every
// file it wrote. scripts/check-bench.mjs fails the build when a pulled file
// differs from the manifest.

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { execFileSync } from 'node:child_process';

const SITE = process.cwd();
const BENCH = process.env.BEATLES_BENCH;
const REPO = 'https://github.com/botassembly/beatles-bench';
const DATA = path.join(SITE, 'src', 'data', 'bench');
const PAGES = path.join(SITE, 'src', 'pages', 'beatles-bench');
const IMG = path.join(SITE, 'public', 'beatles-bench', 'img');
const ROUTE = '/beatles-bench/';

function fail(message) {
  console.error(`pull-bench: ${message}`);
  process.exit(1);
}

if (!BENCH) fail('set BEATLES_BENCH to a checkout of the bench');
const git = (...args) => execFileSync('git', ['-C', BENCH, ...args], { encoding: 'utf8' }).trim();
const pin = fs.readFileSync(path.join(DATA, 'PIN'), 'utf8').trim();
const head = git('rev-parse', 'HEAD');
if (head !== pin) fail(`the checkout is at ${head}, and src/data/bench/PIN names ${pin}`);
if (git('status', '--porcelain')) fail('the checkout has local changes');

// ------------------------------------------------------------------ the list

const list = fs.readFileSync(path.join(BENCH, 'docs', 'README.md'), 'utf8');
const listed = [...list.matchAll(/^\d+\. \[([^\]]+)\]\(([^)]+)\)$/gm)]
  .map(([, title, link]) => path.posix.normalize(path.posix.join('docs', link)));
if (!listed.length) fail('docs/README.md lists no pages');

// docs/beatles-bench.md -> '' (the section's front page)
// docs/NAME.md -> NAME
// examples/NN-NAME/README.md -> NAME
function slugOf(file) {
  if (file === 'docs/beatles-bench.md') return '';
  const doc = /^docs\/([a-z0-9-]+)\.md$/.exec(file);
  if (doc) return doc[1];
  const ex = /^examples\/\d\d-([a-z]+)\/README\.md$/.exec(file);
  if (ex) return ex[1];
  return fail(`no route for ${file}`);
}
const routes = new Map(listed.map((f) => [f, ROUTE + (slugOf(f) ? slugOf(f) + '/' : '')]));

// ------------------------------------------------------------------- a page

const written = [];
function write(file, body) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, body);
  written.push(file);
}

function rewrite(target, from, images, slug) {
  if (/^https:\/\/thinkthen\.dev\//.test(target)) return target.replace('https://thinkthen.dev', '');
  if (/^[a-z]+:/.test(target) || target.startsWith('#')) return target;
  const [rel, anchor] = target.split('#');
  const file = path.posix.normalize(path.posix.join(path.posix.dirname(from), rel));
  const hash = anchor ? `#${anchor}` : '';
  if (routes.has(file)) return routes.get(file) + hash;
  const onDisk = path.join(BENCH, file);
  if (!fs.existsSync(onDisk)) fail(`${from} links ${target}, and the bench has no ${file}`);
  if (/\.(png|jpe?g|svg)$/.test(file)) {
    // examples/NN-NAME/slide.png -> NAME.png. docs/slides/NAME.png keeps its name.
    const base = path.posix.basename(file);
    const name = base === 'slide.png' ? `${slug}.png` : base;
    images.push([onDisk, name]);
    return `/beatles-bench/img/${name}`;
  }
  const kind = fs.statSync(onDisk).isDirectory() ? 'tree' : 'blob';
  return `${REPO}/${kind}/${pin}/${file.replace(/\/$/, '')}${hash}`;
}

// A page or an image the bench dropped must not linger.
for (const f of fs.existsSync(PAGES) ? fs.readdirSync(PAGES) : []) if (f.endsWith('.md')) fs.rmSync(path.join(PAGES, f));
fs.rmSync(IMG, { recursive: true, force: true });

const pages = [];
for (const [order, file] of listed.entries()) {
  const slug = slugOf(file);
  let text = fs.readFileSync(path.join(BENCH, file), 'utf8');
  const title = /^# (.+)\n/.exec(text)?.[1];
  if (!title) fail(`${file} has no title`);
  text = text.replace(/^# .+\n+/, '');

  const images = [];
  // The first image is the slide. It moves to the top of the page.
  let slide = null;
  let tagline = null;
  text = text.replace(/^!\[([^\]]*)\]\(([^)]+)\)\n\n?/m, (_, alt, src) => {
    slide = rewrite(src, file, images, slug);
    tagline = alt.replace(/\.$/, '') === title.replace(/\.$/, '') ? null : alt.replace(/\.?$/, '.');
    return '';
  });

  // Every other link. A fenced block stays as it is.
  text = text.split(/(^```\w*\n[\s\S]*?^```$)/m).map((part, i) => (i % 2 ? part
    : part.replace(/(!?)\[([^\]]*)\]\(([^)\s]+)\)/g, (_, bang, label, target) =>
      `${bang}[${label}](${rewrite(target, file, images, slug)})`))).join('');

  for (const [from, name] of images) {
    const to = path.join(IMG, name);
    fs.mkdirSync(IMG, { recursive: true });
    fs.copyFileSync(from, to);
    written.push(to);
  }

  const front = {
    layout: '../../layouts/BenchPage.astro',
    title,
    tagline,
    slide,
    source: `${REPO}/blob/${pin}/${file}`,
    runsIn: file.startsWith('examples/') ? path.posix.dirname(file) : null,
  };
  const yaml = Object.entries(front)
    .map(([k, v]) => `${k}: ${v === null ? 'null' : JSON.stringify(v)}`).join('\n');
  write(path.join(PAGES, (slug || 'index') + '.md'), `---\n${yaml}\n---\n\n${text.trimEnd()}\n`);
  pages.push({ order: order + 1, slug, title, tagline, route: routes.get(file), source: file });
}

write(path.join(DATA, 'pages.json'), JSON.stringify({ pin, pages }, null, 2) + '\n');

const sha = (f) => crypto.createHash('sha256').update(fs.readFileSync(f)).digest('hex');
const manifest = Object.fromEntries(written.sort().map((f) => [path.relative(SITE, f).split(path.sep).join('/'), sha(f)]));
fs.writeFileSync(path.join(DATA, 'manifest.json'), JSON.stringify({ pin, files: manifest }, null, 2) + '\n');

console.log(`pulled ${pages.length} pages and ${written.length - pages.length - 1} images from the bench at ${pin.slice(0, 8)}`);
