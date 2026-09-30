#!/usr/bin/env node
// The head every built page needs, and the files search engines and browsers
// read beside it. Fails the build when:
//   - a listed page lacks exactly one canonical link to its own address;
//   - an unlisted page (the 404 and search pages) has a canonical link, no
//     robots noindex, or the search index mark;
//   - a page lacks the icon links, the manifest or the two theme colours;
//   - an icon file is missing or the wrong size;
//   - sitemap.xml and the listed pages differ;
//   - robots.txt disagrees with NOINDEX;
//   - the home page's JSON-LD lacks WebSite with SearchAction or
//     SoftwareApplication;
//   - the search index holds any address that is not a listed page.

import fs from 'node:fs';
import path from 'node:path';
import zlib from 'node:zlib';
import { NOINDEX } from '../src/data/catalog.mjs';
import { SITE, builtPages } from '../src/lib/listed-pages.mjs';

const DIST = path.join(process.cwd(), 'dist');
const problems = [];

// ------------------------------------------------------------- icon sizes

const ICONS = { '/apple-touch-icon.png': 180, '/icon-192.png': 192, '/icon-512.png': 512 };
const FAVICON_SIZES = [16, 32, 48];

function pngSize(buf) {
  if (buf.toString('latin1', 1, 4) !== 'PNG') return null;
  return [buf.readUInt32BE(16), buf.readUInt32BE(20)];
}

function read(route) {
  const file = path.join(DIST, route);
  return fs.existsSync(file) ? fs.readFileSync(file) : null;
}

for (const [route, px] of Object.entries(ICONS)) {
  const buf = read(route);
  const size = buf && pngSize(buf);
  if (!size) problems.push(`${route}: missing or not a PNG`);
  else if (size[0] !== px || size[1] !== px) problems.push(`${route}: ${size.join('x')}, expected ${px}x${px}`);
}

const ico = read('/favicon.ico');
if (!ico || ico.readUInt16LE(2) !== 1) problems.push('/favicon.ico: missing or not an icon file');
else {
  const found = [];
  for (let i = 0; i < ico.readUInt16LE(4); i += 1) {
    const at = 6 + 16 * i;
    const image = ico.subarray(ico.readUInt32LE(at + 12), ico.readUInt32LE(at + 12) + ico.readUInt32LE(at + 8));
    const size = pngSize(image);
    if (!size || size[0] !== (ico[at] || 256)) problems.push(`/favicon.ico: entry ${i} does not match its PNG`);
    found.push(ico[at] || 256);
  }
  if (found.sort((a, b) => a - b).join() !== FAVICON_SIZES.join()) problems.push(`/favicon.ico: sizes ${found.join(', ')}, expected ${FAVICON_SIZES.join(', ')}`);
}

let manifest = null;
try { manifest = JSON.parse(read('/site.webmanifest')); } catch { problems.push('/site.webmanifest: missing or not JSON'); }
if (manifest) {
  const sizes = (manifest.icons || []).map((i) => i.sizes).sort().join();
  if (sizes !== '192x192,512x512') problems.push(`/site.webmanifest: icon sizes ${sizes}, expected 192x192 and 512x512`);
  for (const icon of manifest.icons || []) {
    const buf = read(icon.src);
    const size = buf && pngSize(buf);
    if (!size || `${size[0]}x${size[1]}` !== icon.sizes) problems.push(`/site.webmanifest: ${icon.src} is not a PNG of ${icon.sizes}`);
  }
}

// ------------------------------------------------------------------ heads

const HEAD = [
  ['the favicon.ico link', /<link rel="icon" href="\/favicon\.ico" sizes="16x16 32x32 48x48">/],
  ['the unconditional SVG icon', /<link rel="icon" href="\/brand\/thinkthen-mark-dark\.svg" type="image\/svg\+xml">/],
  ['the apple-touch icon', /<link rel="apple-touch-icon" href="\/apple-touch-icon\.png">/],
  ['the manifest link', /<link rel="manifest" href="\/site\.webmanifest">/],
  ['the light theme colour', /<meta name="theme-color" content="#faf8f1" media="\(prefers-color-scheme: light\)">/],
  ['the dark theme colour', /<meta name="theme-color" content="#0e1311" media="\(prefers-color-scheme: dark\)">/],
];

const pages = builtPages(DIST);
const listed = pages.filter((p) => p.kind === 'listed');
const unlisted = pages.filter((p) => p.kind === 'unlisted');
for (const route of ['/404.html', '/search/']) {
  if (!unlisted.some((p) => p.route === route)) problems.push(`${route}: missing, or not marked unlisted`);
}

for (const page of pages.filter((p) => p.kind !== 'stub')) {
  const head = page.html.slice(0, page.html.indexOf('</head>'));
  for (const [name, re] of HEAD) if (!re.test(head)) problems.push(`${page.route}: no ${name}`);
  const canon = [...head.matchAll(/<link rel="canonical" href="([^"]*)">/g)].map((m) => m[1]);
  const pagefind = /<main[^>]* data-pagefind-body/.test(page.html);
  if (page.kind === 'unlisted') {
    if (canon.length) problems.push(`${page.route}: unlisted, yet has a canonical link`);
    if (!/<meta name="robots" content="noindex">/.test(head)) problems.push(`${page.route}: unlisted, yet has no robots noindex`);
    if (pagefind) problems.push(`${page.route}: unlisted, yet marked for the search index`);
    if (/type="text\/markdown"/.test(head)) problems.push(`${page.route}: unlisted, yet links a Markdown twin`);
  } else if (page.kind === 'listed') {
    if (canon.length !== 1) problems.push(`${page.route}: ${canon.length} canonical links`);
    else if (canon[0] !== SITE + page.route) problems.push(`${page.route}: canonical is ${canon[0]}`);
    if (!pagefind) problems.push(`${page.route}: main is not marked for the search index`);
  } else {
    problems.push(`${page.route}: a built page that is neither listed, unlisted nor a redirect`);
  }
}

// ---------------------------------------------------------------- sitemap

const want = new Set(listed.map((p) => SITE + p.route));
const sitemap = read('/sitemap.xml');
if (!sitemap) problems.push('/sitemap.xml: missing');
else {
  const got = new Set([...sitemap.toString().matchAll(/<loc>([^<]*)<\/loc>/g)].map((m) => m[1]));
  for (const u of want) if (!got.has(u)) problems.push(`/sitemap.xml: misses ${u}`);
  for (const u of got) if (!want.has(u)) problems.push(`/sitemap.xml: lists ${u}, which is not a listed page`);
}

// ----------------------------------------------------------------- robots

const robots = read('/robots.txt')?.toString();
const robotsWant = NOINDEX
  ? 'User-agent: *\nDisallow: /\n'
  : `User-agent: *\nAllow: /\n\nSitemap: ${SITE}/sitemap.xml\n`;
if (robots !== robotsWant) problems.push(`/robots.txt: does not match NOINDEX = ${NOINDEX}`);

// ---------------------------------------------------------------- JSON-LD

const home = listed.find((p) => p.route === '/');
const ld = home && home.html.match(/<script type="application\/ld\+json">([\s\S]*?)<\/script>/);
let items = [];
try { items = ld ? [].concat(JSON.parse(ld[1])) : []; } catch { problems.push('/: JSON-LD does not parse'); }
const site = items.find((i) => i['@type'] === 'WebSite');
if (!site || site.potentialAction?.['@type'] !== 'SearchAction' || !String(site.potentialAction?.target?.urlTemplate).startsWith(`${SITE}/search/?q=`)) {
  problems.push('/: JSON-LD has no WebSite with a SearchAction on /search/');
}
const app = items.find((i) => i['@type'] === 'SoftwareApplication');
if (!app || app.offers?.priceCurrency !== 'USD') problems.push('/: JSON-LD has no SoftwareApplication with an Offer in USD');
for (const page of listed.filter((p) => p !== home)) {
  if (page.html.includes('application/ld+json')) problems.push(`${page.route}: JSON-LD belongs on the home page only`);
}

// ----------------------------------------------------------- search index

const FRAGMENTS = path.join(DIST, 'pagefind/fragment');
if (!fs.existsSync(path.join(DIST, 'pagefind/pagefind-ui.js')) || !fs.existsSync(FRAGMENTS)) problems.push('/pagefind/: missing');
else {
  const wantRoutes = new Set(listed.map((p) => p.route));
  const indexed = new Set(fs.readdirSync(FRAGMENTS).map((f) => {
    const text = zlib.gunzipSync(fs.readFileSync(path.join(FRAGMENTS, f))).toString('utf8');
    return JSON.parse(text.replace(/^pagefind_dcd/, '')).url;
  }));
  for (const r of indexed) if (!wantRoutes.has(r)) problems.push(`search index holds ${r}, which is not a listed page`);
  for (const r of wantRoutes) if (!indexed.has(r)) problems.push(`search index misses ${r}`);
}

if (problems.length) {
  console.error(`head check: ${problems.length} problems`);
  for (const p of problems) console.error('  ' + p);
  process.exit(1);
}

console.log(`check-head: ${listed.length} listed pages, each with one canonical link, in the sitemap and the search index; ${unlisted.length} unlisted pages; icons, manifest, theme colours, robots.txt (NOINDEX = ${NOINDEX}) and JSON-LD hold`);
