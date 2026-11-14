// The site checks that run on the built site:
//   front matter is complete, no two files make one page, the word list holds, prose never
//   types the version, the README's install lines match, no broken internal link or anchor,
//   every page has an incoming link, the agent and host files exist (with no raw component
//   tag in a Markdown twin or llms-full.txt), every old address
//   (and its Markdown twin) still resolves, and each redirect lands on a built page.
// The other checks run earlier in npm run build: examples, generated reference, astro warnings.
//   node scripts/check.mjs             the build check
//   node scripts/check.mjs --publish   also fail on addresses the publishing step must serve
//                                      (product.hostFiles), once they are placed in dist/
import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import YAML from 'yaml';
import product from '../product.config.mjs';
import site from '../src/data/site.json' with { type: 'json' };
import { bannedWords, commandLines, proseOnly } from '../src/lib/docs-rules.mjs';
import { menuPages } from '../src/lib/sitemap.mjs';
import redirects from '../src/data/redirects.json' with { type: 'json' };
import { pages, pageMarkdown, splitCode } from './pages.mjs';
import { EXAMPLES, inputFiles, readExample, matchesExampleInputDownload } from '../src/lib/examples.mjs';

const publish = process.argv.includes('--publish');
const dist = path.resolve('dist');
const all = pages();
const errors = [];
const notices = [];

const helpTables = spawnSync(process.execPath, ['scripts/help-tables.mjs', '--rendered'], { encoding: 'utf8' });
if (helpTables.status !== 0) errors.push(helpTables.stderr.trim() || helpTables.stdout.trim() || 'Rendered help table check did not run');

const pythonExamples = spawnSync('python3', ['scripts/check-python-examples.py'], { encoding: 'utf8' });
if (pythonExamples.status !== 0) errors.push(pythonExamples.stderr.trim() || 'Python example check did not run');

// Front matter.
const FIELDS = ['title', 'description', 'goal', 'reader', 'kind', 'verified'];
for (const p of all) for (const f of FIELDS) if (!p.data[f]) errors.push(`${p.file}: front matter lacks ${f}`);

// Two source files that make the same page: Astro keeps one and drops the other quietly.
const byRoute = new Map();
for (const p of all) byRoute.set(p.route, [...(byRoute.get(p.route) ?? []), p.file]);
for (const [route, files] of byRoute) if (files.length > 1) errors.push(`${route}: made by ${files.length} files; keep one: ${files.map((f) => path.relative('.', f)).join(', ')}`);

// Current authored prose and table descriptions follow the one banned list.
// Code spellings and exact captured output are excluded from this prose rule.
for (const p of all) {
  const markdown = pageMarkdown(p);
  // The two dated release posts retain their historical opening version.
  // All docs pages and the Blog index follow rule 4 (decision recorded on main b8f7d504b).
  const releasePost = ['/blog/introducing-thinkthen/', '/blog/let-your-shell-branch-on-meaning/'].includes(p.route) && p.data.date;
  const lead = markdown.split(/\n\s*\n/)[1] ?? '';
  if (!releasePost && lead.includes(site.version)) errors.push(`${p.route}: lead names the version`);

  for (const finding of bannedWords(proseOnly(markdown), p.route))
    errors.push(`${p.route}: banned word "${finding.word}". ${finding.use}`);
  for (const finding of commandLines(markdown))
    errors.push(`${p.route}: command line has ${finding.length} characters (72 maximum): ${finding.line}`);
  if (/\bstub\b/i.test(`${p.data.title}\n${p.data.description}\n${markdown}`)) errors.push(`${p.route}: page names the stub`);
}
for (const expected of menuPages) {
  const p = all.find((page) => page.route === expected.route);
  if (!p) { errors.push(`${expected.route}: sitemap page missing`); continue; }
  const name = expected.slug.startsWith('sdks/') && !['duckdb', 'sqlite', 'postgresql', 'sql'].includes(expected.slug.split('/')[1])
    ? `${expected.label} SDK` : expected.route === '/' ? product.name : expected.label;
  if (p.data.title !== name) errors.push(`${expected.route}: title must be ${name}, got ${p.data.title}`);
  const fn = expected.route.match(/^\/functions\/([^/]+)\/$/)?.[1];
  if (fn) {
    const tables = [...p.body.matchAll(/<FunctionOptions\s+name="([^"]+)"\s*\/>/g)].map((match) => match[1]);
    if (tables.length !== 1 || tables[0] !== fn) errors.push(`${expected.route}: require exactly its own help-derived FunctionOptions table`);
  }
}

// With product.readmeInstallLines, the repository README repeats every install line of site.json
// exactly. (A README rendered from site.json by scripts/readme.mjs is checked there instead.)
const readme = path.resolve('../README.md');
if (product.readmeInstallLines && fs.existsSync(readme)) {
  const text = fs.readFileSync(readme, 'utf8');
  for (const [k, line] of Object.entries(site.install ?? {})) if (typeof line === 'string' && !text.includes(line)) errors.push(`README.md: install line install.${k} differs from src/data/site.json`);
}

// Example inputs: every file an example reads from its own folder shows on each page that runs
// the example, through <ExampleFile>, a shown script or output that holds it, or a link to a
// page that shows it. A reader who lands on one page can then rerun what the page shows.
const content = (id, f) => {
  const b = fs.readFileSync(path.join(EXAMPLES, id, f));
  return /\.xlsx$/i.test(f) ? `xlsx:${b.toString('base64')}` : b.toString('utf8').trimEnd();
};
const attr = (tag, name) => tag.match(new RegExp(`\\b${name}="([^"]+)"`))?.[1];
const shows = new Map();
const runsOn = new Map();
for (const p of all) {
  const files = new Set([...p.body.matchAll(/<ExampleFile\b[^>]*>/g)].map((m) => content(attr(m[0], 'id'), attr(m[0], 'file'))));
  const ids = [...new Set([...p.body.matchAll(/<Example\b[^>]*>/g)].map((m) => attr(m[0], 'id')))];
  const texts = ids.map((id) => { const ex = readExample(id); return `${ex.show}\n${ex.output ?? ''}`; });
  shows.set(p.route, (c) => files.has(c) || (!c.startsWith('xlsx:') && texts.some((t) => t.includes(c))));
  runsOn.set(p, ids);
}
for (const [p, ids] of runsOn) {
  const downloads = new Set([...p.body.matchAll(/\]\((\/[^)#\s]*)/g)].map((m) => m[1]));
  const links = new Set([...downloads].map((url) => (url.endsWith('/') ? url : `${url}/`)));
  for (const id of ids) {
    for (const f of inputFiles(id)) {
      const c = content(id, f);
      if (shows.get(p.route)(c) || [...links].some((r) => shows.get(r)?.(c)) || matchesExampleInputDownload(id, f, downloads, dist)) continue;
      errors.push(`${path.relative('.', p.file)}: example ${id} reads ${f}, which the page neither shows nor links to a page that shows; add <ExampleFile id="${id}" file="${f}" />`);
    }
  }
}

// Links and anchors, read from main content and the reader's surface navigation.
const fileFor = (target) => {
  const clean = decodeURIComponent(target.split('#')[0].split('?')[0]) || '/';
  const p = path.join(dist, clean);
  if (fs.existsSync(p) && fs.statSync(p).isFile()) return p;
  if (fs.existsSync(path.join(p, 'index.html'))) return path.join(p, 'index.html');
  return null;
};
const exists = (target) => fileFor(target) !== null;
const isRedirect = (file) => /<meta http-equiv="refresh"/i.test(fs.readFileSync(file, 'utf8').slice(0, 2000));
const idCache = new Map();
const ids = (file) => {
  if (!idCache.has(file)) idCache.set(file, new Set([...fs.readFileSync(file, 'utf8').matchAll(/\sid="([^"]+)"/g)].map((m) => m[1])));
  return idCache.get(file);
};
const anchorOk = (file, hash) => !hash || ids(file).has(decodeURIComponent(hash));
const incoming = new Map(all.map((p) => [p.route, 0]));
let anchors = 0;
for (const p of all) {
  const file = path.join(dist, p.route, 'index.html');
  if (!fs.existsSync(file)) { errors.push(`${p.route}: not built`); continue; }
  const html = fs.readFileSync(file, 'utf8');
  // The canonical menu and header/footer are real incoming navigation too.
  // Match anchor elements only, never hidden data or stylesheet/script URLs.
  const navigation = [...html.matchAll(/<(main|nav|header|footer)\b[^>]*>[\s\S]*?<\/\1>/gi)].map(([block]) => block).join('');
  for (const [, raw] of navigation.matchAll(/<a\b[^>]*\bhref="([^"]+)"/g)) {
    const href = raw.replaceAll('&amp;', '&');
    if (/^(https?:|mailto:)/.test(href)) continue;
    const url = new URL(href, `https://x${p.route}`);
    const target = url.pathname;
    const hash = url.hash.slice(1);
    const tfile = href.startsWith('#') ? file : fileFor(target);
    if (!tfile) { errors.push(`${p.route}: broken link ${href}`); continue; }
    if (hash && tfile.endsWith('.html')) {
      anchors++;
      if (!anchorOk(tfile, hash)) errors.push(`${p.route}: link ${href} names an anchor its page lacks`);
    }
    const r = target.endsWith('/') ? target : `${target}/`;
    if (!href.startsWith('#') && incoming.has(r) && r !== p.route) incoming.set(r, incoming.get(r) + 1);
  }
}
for (const [route, n] of incoming) if (n === 0 && route !== '/') errors.push(`${route}: no page links here`);

// Agent files.
const llms = path.join(dist, 'llms.txt');
if (!fs.existsSync(llms) || !fs.existsSync(path.join(dist, 'llms-full.txt'))) errors.push('dist: llms.txt or llms-full.txt missing');
else {
  const text = fs.readFileSync(llms, 'utf8');
  const full = fs.readFileSync(path.join(dist, 'llms-full.txt'), 'utf8');
  for (const p of pages({ include404: true })) {
    if (!fs.existsSync(path.join(dist, p.twin))) errors.push(`${p.twin}: Markdown twin missing`);
    if (!text.includes(p.twin)) errors.push(`llms.txt: does not list ${p.twin}`);
    if (!text.includes(p.data.description)) errors.push(`llms.txt: lacks summary for ${p.route}`);
    if (!full.includes(`${pageMarkdown(p).split('\n', 1)[0]}\n`)) errors.push(`llms-full.txt: lacks title for ${p.route}`);
    const twinBody = fs.readFileSync(path.join(dist, p.twin), 'utf8').replace(/^---\n[\s\S]*?\n---\n+/, '').trim();
    if (twinBody !== pageMarkdown(p).trim()) errors.push(`${p.twin}: body differs from the page source`);
    const fullBlock = full.split(`<page url="${new URL(p.route, product.site).href}">\n\n`)[1]?.split('\n\n</page>')[0].trim();
    if (fullBlock !== twinBody) errors.push(`llms-full.txt: body differs from ${p.twin}`);
    const html = fs.readFileSync(path.join(dist, p.route, 'index.html'), 'utf8');
    if (!html.includes(`data-copy-markdown="${p.twin}"`)) errors.push(`${p.route}: copy button missing`);
    if (!html.includes(`href="${p.twin}"`)) errors.push(`${p.route}: raw Markdown link missing`);
  }
  // A raw component tag outside code means an agent reads a tag where the HTML page shows content.
  const mdFiles = [];
  const walk = (d) => { for (const f of fs.readdirSync(d, { withFileTypes: true })) { const p = path.join(d, f.name); if (f.isDirectory()) walk(p); else if (f.name.endsWith('.md')) mdFiles.push(p); } };
  walk(dist);
  for (const f of [...mdFiles, path.join(dist, 'llms-full.txt')]) {
    const body = fs.readFileSync(f, 'utf8').replace(/^---\n[\s\S]*?\n---\n+/, '');
    const prose = splitCode(body).filter((_, i) => i % 2 === 0).join('');
    const tag = prose.match(/<[A-Z][A-Za-z]+[ />]/);
    if (tag) errors.push(`${path.relative(dist, f)}: holds the raw component tag ${tag[0].trim()}; give it a Markdown form in scripts/pages.mjs`);
  }
}

// Inspect built HTML independently of the source-page list, including the 404 layout.
function checkCopyControls(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) { checkCopyControls(file); continue; }
    if (!file.endsWith('.html')) continue;
    const html = fs.readFileSync(file, 'utf8');
    // Astro's old-address redirect documents have no page body. Their destinations are checked below.
    if (/http-equiv="refresh"/i.test(html)) continue;
    const twin = html.match(/data-copy-markdown="([^"]+)"/)?.[1];
    if (!twin || !fs.existsSync(path.join(dist, twin))) errors.push(`${path.relative(dist, file)}: missing copy control or Markdown twin`);
  }
}
checkCopyControls(dist);

// Redirects: each lands on a built page, not on another redirect or a missing address.
for (const [from, to] of Object.entries(redirects)) {
  const [toPath, hash] = to.split('#');
  const file = fileFor(toPath);
  if (!file) errors.push(`redirect ${from} -> ${to}: the target is not a built page`);
  else if (file.endsWith('.html') && isRedirect(file)) errors.push(`redirect ${from} -> ${to}: the target is itself a redirect; point at the page`);
  else if (hash && !anchorOk(file, hash)) errors.push(`redirect ${from} -> ${to}: the target page has no anchor #${hash}`);
}

// Old addresses: each still serves a page, a redirect or a file. A page's Markdown twin
// (/a/b/ -> /a/b.md) was served too, so each twin must still resolve.
// Addresses in product.hostFiles are files the publishing step places; see site/README.md.
const hostFiles = product.hostFiles ?? {};
const twinOf = (a) => (a === '/' ? '/index.md' : `${a.replace(/\/$/, '')}.md`);
const old = fs.readFileSync(path.resolve('src/data/old-addresses.txt'), 'utf8').split('\n').map((l) => l.trim()).filter((l) => l && !l.startsWith('#'));
for (const a of old) {
  if (hostFiles[a] !== undefined) {
    if (!exists(a)) (publish ? errors : notices).push(`${a}: the build does not make this old address. ${hostFiles[a]}`);
    continue;
  }
  if (!exists(a)) errors.push(`${a}: old address no longer resolves; add it to src/data/redirects.json`);
  if (a.endsWith('/') && !exists(twinOf(a))) errors.push(`${twinOf(a)}: the Markdown twin of old address ${a} no longer resolves; scripts/agent-files.mjs writes one for each redirect`);
}

// Host files: sitemaps, robots.txt, version.json and the social card.
for (const f of ['sitemap-index.xml', 'sitemap.xml', 'robots.txt', 'version.json', 'social-card.png', 'install.sh', 'install.ps1']) if (!fs.existsSync(path.join(dist, f))) errors.push(`dist: ${f} missing`);
if (fs.existsSync(path.join(dist, 'robots.txt')) && !fs.readFileSync(path.join(dist, 'robots.txt'), 'utf8').includes(`Sitemap: ${new URL('/sitemap.xml', product.site).href}`)) errors.push('dist/robots.txt: does not name the sitemap');
if (fs.existsSync(path.join(dist, 'version.json'))) {
  const v = JSON.parse(fs.readFileSync(path.join(dist, 'version.json'), 'utf8'));
  if (v.version !== site.version) errors.push(`dist/version.json: version ${v.version}, site.json ${site.version}`);
}
const card = path.join(dist, 'social-card.png');
if (fs.existsSync(card)) {
  const b = fs.readFileSync(card);
  if (b.readUInt32BE(16) !== 1200 || b.readUInt32BE(20) !== 630) errors.push(`public/social-card.png: ${b.readUInt32BE(16)}x${b.readUInt32BE(20)}, not 1200x630`);
}
for (const p of all) {
  const file = path.join(dist, p.route, 'index.html');
  if (!fs.existsSync(file)) continue;
  const head = fs.readFileSync(file, 'utf8').split('</head>')[0];
  if (!/property="og:image"/.test(head) || !/name="twitter:image"/.test(head)) errors.push(`${p.route}: lacks og:image or twitter:image`);
}

if (notices.length) console.log(`publishing must serve:\n${notices.join('\n')}`);
fs.writeFileSync('content-report.json', JSON.stringify({ pages: all.length, redirects: Object.keys(redirects).length, oldAddresses: old.length, errors, notices }, null, 2) + '\n');
if (errors.length) {
  console.error(`${errors.length} check failure(s):\n` + errors.slice(0, 40).join('\n'));
  if (errors.length > 40) console.error('First 40 shown; all findings are in content-report.json');
  process.exit(1);
}

console.log(`checks passed: ${all.length} pages, ${anchors} anchors, ${Object.keys(redirects).length} redirects, ${old.length} old addresses`);
