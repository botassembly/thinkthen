#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { builtPages } from '../src/lib/listed-pages.mjs';
import { parseHtml, findElement, textContent, hasClass } from '../src/lib/html.mjs';
import { postLineProblems } from '../src/lib/post-line.mjs';

// Literal paragraph exceptions follow the accepted design's individual scopes.
export const ALLOWED = [
  {
    "text": "The cut applies to the probability of yes on decide and each record under filter, the winning option’s probability on choose, each label’s independent yes probability under tag, each name’s printed strength under recognize, and each relation edge’s yes probability under recognize or relate. The threshold contract defines every boundary.",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/recognize/",
      "/functions/relate/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain the cross-function cut contract in each complete function reference."
  },
  {
    "text": "The framing flags are mutually exclusive. The tool never guesses a framing from a filename. --input FILE reads the same input from a file. An empty document is a usage error. An empty line or JSONL stream exits 0 with no output and no request. Invalid UTF-8 in a record is exit 5.",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/find/",
      "/functions/rank/",
      "/functions/recognize/",
      "/functions/relate/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain framing and empty/invalid input rules in each complete function reference."
  },
  {
    "text": "Repeat --input FILE to read files in argument order. Decide, filter, rank and annotate also accept files after their question or question set. These positional files cannot accompany --input. Choose, score and tag keep positional labels and levels.",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/rank/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain repeated-file and positional-input rules where those inputs apply."
  },
  {
    "text": "--window N joins physical text lines within each file. It keeps internal line feeds and removes the final record ending. A window never crosses a file boundary. Empty files send nothing. Windows refuse JSON and table framing, --field and saved on pointers. Each joined item keeps the record byte limit.",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/rank/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain the window boundary contract where window input applies."
  },
  {
    "text": "Detailed text and document results carry position with file, first and last. Each file starts at line one. Stdin uses a null file. Table rows carry no position.",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/rank/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain detailed position provenance where line positions apply."
  },
  {
    "text": "Both require a header. Every cell becomes a JSON string, including an empty cell and text that looks like a number, a boolean, a null, an array, or an object. Header order becomes key order. A header name may hold whitespace, but it may not be blank, hold a control character, or exactly duplicate another. One UTF-8 byte order mark is ignored at the start of the first header name and nowhere else. Every data row must carry the header's field count. A header with no data rows is a successful empty dataset and sends no request. Empty input is exit 2. Every output from CSV or TSV input is JSON Lines.",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/rank/",
      "/functions/recognize/",
      "/functions/relate/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain the table cell/header contract where CSV and TSV input apply."
  },
  {
    "text": "Every result is compact and sits on one line. --details prints the full result object in place of the bare value. The result specification lists its fields.",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/find/",
      "/functions/rank/",
      "/functions/recognize/",
      "/functions/relate/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain compact result and details semantics in each complete function reference."
  },
  {
    "text": "In record mode the exit code reports the run and never one record's answer. A completed run exits 0 unless a code above says otherwise. A valid answer on standard output can sit beside a code other than 0. Every exit code lists the codes of all the functions.",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/find/",
      "/functions/rank/",
      "/functions/recognize/",
      "/functions/relate/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain record-mode exit semantics in each complete function reference."
  },
  {
    "text": "With no backend named, the key comes from THINKTHEN_API_KEY. --backend, THINKTHEN_BACKEND or the configuration file's backend names a backend. A named backend reads only its own key variables, and the first one that is not blank wins:",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/find/",
      "/functions/rank/",
      "/functions/recognize/",
      "/functions/relate/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain backend/key selection in each complete function reference."
  },
  {
    "text": "The command line comes first, then the environment, then the configuration file. The first of these that names a backend or an address decides. So THINKTHEN_BASE_URL in the shell outranks the configuration file's backend, and the key then comes from THINKTHEN_API_KEY. With both --backend and --url, the backend's key variables apply at that address.",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/find/",
      "/functions/rank/",
      "/functions/recognize/",
      "/functions/relate/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain backend/address precedence in each complete function reference."
  },
  {
    "text": "With the key unset or blank, a request to localhost, 127.0.0.1 or [::1] goes out with no key and no Authorization header. At any other address, the command exits 4. The Configuration page lists every variable ThinkThen reads, and the Backends page gives the rest of the key rules.",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/find/",
      "/functions/rank/",
      "/functions/recognize/",
      "/functions/relate/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain keyless loopback and hosted failure boundaries in each complete function reference."
  },
  {
    "text": "One whole request carries at most about 64,000 tokens, the questions included. The evidence in one request must fit in about 32,000 tokens. TypeSafe published both numbers on 2026-09-19 for its hosted service. They belong to the backend and not to the tool. Evidence past the token limit is refused, and the command exits 4. The records specification lists them.",
    "routes": [
      "/functions/annotate/",
      "/functions/choose/",
      "/functions/decide/",
      "/functions/filter/",
      "/functions/find/",
      "/functions/rank/",
      "/functions/recognize/",
      "/functions/relate/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain the sourced hosted request/evidence limits in each complete function reference."
  },
  {
    "text": "--jobs N runs from 1 to 32 and defaults to 8. It acts in record mode. On one document it is a usage error. TypeSafe documents a limit of 1,200 requests a minute. The records specification holds the measured rates. --jobs caps the requests in flight. It sets no limit on requests a minute. The rate depends on how fast replies come back. On short records, the default of 8 can pass that limit. In a review, throttle 4 sent 1,519 requests a minute to TypeSafe. Against a local test server with 100 ms replies, --jobs 3 sent 1,285 a minute. Set requests_per_minute on the backend, or THINKTHEN_REQUESTS_PER_MINUTE, to stay under the limit.",
    "routes": [
      "/functions/choose/",
      "/functions/decide/",
      "/functions/recognize/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain the jobs contract with its document-mode restriction on applicable functions."
  },
  {
    "text": "A cut of 0 is refused. A percent such as 90, a band, and a number that is not finite are all usage errors before any request.",
    "routes": [
      "/functions/choose/",
      "/functions/filter/",
      "/functions/recognize/",
      "/functions/relate/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain invalid cut boundaries on the functions accepting a single cut."
  },
  {
    "text": "Without record framing, several named files produce one JSON line per document: input_file names the file and value carries the answer. Details add input_file to the full result. A completed run exits 0, including false or null answers. A later failure keeps completed output and returns its error code. Several documents refuse --raw and --quiet.",
    "routes": [
      "/functions/choose/",
      "/functions/decide/",
      "/functions/score/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain the multiple-document envelope and partial-failure contract where it applies."
  },
  {
    "text": "The default is 0.5. --threshold T takes a number above 0 and at most 1. A value at or above T passes, and a value under T fails.",
    "routes": [
      "/functions/decide/",
      "/functions/filter/",
      "/functions/recognize/",
      "/functions/relate/",
      "/functions/tag/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain the default single-cut boundary on applicable functions."
  },
  {
    "text": "--jobs N runs from 1 to 32 and defaults to 8. It acts in record mode. TypeSafe documents a limit of 1,200 requests a minute. The records specification holds the measured rates. --jobs caps the requests in flight. It sets no limit on requests a minute. The rate depends on how fast replies come back. On short records, the default of 8 can pass that limit. In a review, throttle 4 sent 1,519 requests a minute to TypeSafe. Against a local test server with 100 ms replies, --jobs 3 sent 1,285 a minute. Set requests_per_minute on the backend, or THINKTHEN_REQUESTS_PER_MINUTE, to stay under the limit.",
    "routes": [
      "/functions/filter/",
      "/functions/rank/"
    ],
    "source": "src/pages/functions/[name]/index.astro",
    "reason": "Retain the jobs contract on filter and rank without a document-mode restriction."
  },
  {
    "text": "OpenAI announced its Decisions API on 2026-09-29 in its DevDay 2026 recap. OpenAI has published no address, schema or price, so ThinkThen cannot call it. The OpenAI Decisions API page says what ThinkThen knows.",
    "routes": [
      "/install/ada/",
      "/install/c/",
      "/install/cobol/",
      "/install/cpp/",
      "/install/csharp/",
      "/install/dart/",
      "/install/go/",
      "/install/java/",
      "/install/kotlin/",
      "/install/objective-c/",
      "/install/php/",
      "/install/python/",
      "/install/r/",
      "/install/ruby/",
      "/install/rust/",
      "/install/scala/",
      "/install/shell/",
      "/install/swift/",
      "/install/typescript/",
      "/install/zig/"
    ],
    "source": "src/data/catalog.mjs: OPENAI_ANNOUNCED",
    "reason": "Retain the shared installation announcement from its single catalog source."
  },
  {
    "text": "The typed value wins, then the environment, then the question file, then the configuration file, then the built-in default. A setting skips the tiers it has no home in.",
    "routes": [
      "/install/configuration/",
      "/install/settings/"
    ],
    "source": "specification/settings.md",
    "reason": "Retain the settings precedence contract on the settings table and configuration guide."
  },
  {
    "text": "Door(settingsJson) takes the settings as JSON. \"record\" writes a recording to a folder. \"replay\" answers from that recording with no connection. The Settings page lists every setting.",
    "routes": [
      "/install/kotlin/",
      "/install/scala/"
    ],
    "source": "src/data/catalog.mjs: JVM settings",
    "reason": "Retain the shared JVM settings constructor instructions for Kotlin and Scala."
  },
  {
    "text": "tt.Engine takes each setting as a keyword, as in Python. record= writes a recording to a folder. replay= answers from that recording with no connection. The Settings page lists every setting.",
    "routes": [
      "/install/pandas/",
      "/install/polars/"
    ],
    "source": "src/data/catalog.mjs: frame settings",
    "reason": "Retain shared Python frame-engine settings instructions for pandas and Polars."
  }
];

const elements = node => (node.kids || []).filter(k => k.tag !== '#text');
const normalized = node => textContent(node).replace(/\s+/g, ' ').trim();
function visit(node, test, acc = []) {
  if (test(node)) acc.push(node);
  for (const child of node.kids || []) visit(child, test, acc);
  return acc;
}
export function paragraphInventory(pages) {
  const paragraphs = new Map();
  for (const page of pages.filter(p => p.kind !== 'stub')) {
    const main = findElement(parseHtml(page.html), n => n.tag === 'main');
    if (!main) continue;
    function collect(node) {
      if (['pre', 'script', 'style', 'button'].includes(node.tag)) return;
      if (node.tag === 'p') {
        const text = normalized(node);
        if (text.split(/\s+/).length >= 25) {
          if (!paragraphs.has(text)) paragraphs.set(text, new Set());
          paragraphs.get(text).add(page.route);
        }
        return;
      }
      for (const child of node.kids || []) collect(child);
    }
    collect(main);
  }
  return paragraphs;
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

export function layoutProblems(pages, articles = [], allowed = ALLOWED) {
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
  const paragraphs = paragraphInventory(pages);
  const exceptions = new Map();
  for (const entry of allowed) {
    if (exceptions.has(entry.text)) fail(entry.source, 'allowed-duplicate');
    exceptions.set(entry.text, entry);
    const routes = [...(paragraphs.get(entry.text) || [])].sort();
    if (!entry.source || !entry.reason || !entry.text || entry.text !== entry.text.replace(/\s+/g, ' ').trim() || entry.text.split(/\s+/).length < 25 || entry.routes.length < 2 || new Set(entry.routes).size !== entry.routes.length || JSON.stringify(entry.routes) !== JSON.stringify([...entry.routes].sort())) fail(entry.source, 'allowed-invalid');
    if (!routes.length) fail(entry.source, 'allowed-unused');
    else if (JSON.stringify(routes) !== JSON.stringify(entry.routes)) fail(entry.source, 'allowed-routes');
  }
  for (const [text, routes] of paragraphs) {
    if (routes.size < 2) continue;
    if (!exceptions.has(text)) fail([...routes].sort().join(', '), `duplicate-paragraph ${text}`);
  }
  return problems;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const pages = builtPages(path.resolve(process.argv[2] || 'dist'));
  const articles = readArticles(path.resolve(process.argv[3] || 'src/articles'));
  const problems = layoutProblems(pages, articles);
  if (problems.length) { console.error(`check-layout: ${problems.length} problems\n  ${problems.join('\n  ')}`); process.exit(1); }
  console.log(`check-layout: ${pages.filter(p => p.kind !== 'stub').length} pages, ${articles.length} source posts, grids and scoped paragraphs passed`);
}
