// The pages a search engine may list: every built index.html that is not a
// redirect stub and not marked unlisted by Base.astro. write-sitemap.mjs and
// check-head.mjs share this list, so the sitemap and its check cannot drift.

import fs from 'node:fs';
import path from 'node:path';

export const SITE = 'https://thinkthen.dev';

export const isStub = (html) => /<meta http-equiv="refresh"/.test(html);
export const isUnlisted = (html) => /<html[^>]* data-unlisted/.test(html);

function walk(dir, acc = []) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) walk(full, acc);
    else if (entry.name.endsWith('.html')) acc.push(full);
  }
  return acc;
}

// Every built HTML file, with its address and its kind.
export function builtPages(dist) {
  return walk(dist).sort().map((file) => {
    const html = fs.readFileSync(file, 'utf8');
    const rel = path.relative(dist, file).split(path.sep).join('/');
    const route = rel === 'index.html' ? '/' : rel.endsWith('/index.html') ? `/${rel.slice(0, -'index.html'.length)}` : `/${rel}`;
    const kind = isStub(html) ? 'stub' : isUnlisted(html) ? 'unlisted' : rel.endsWith('index.html') ? 'listed' : 'other';
    return { file, route, html, kind };
  });
}

export const listedUrls = (dist) => builtPages(dist).filter((p) => p.kind === 'listed').map((p) => SITE + p.route);
