// Owned HTML/source fixtures call the real checker in a separate process.
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
const root = fs.mkdtempSync(path.join(os.tmpdir(), 'thinkthen-layout-'));
const dist = path.join(root, 'dist');
const articles = path.join(root, 'articles');
const checker = new URL('./check-layout.mjs', import.meta.url).href;
const listed = new URL('../src/lib/listed-pages.mjs', import.meta.url).href;
const write = (file, text) => { fs.mkdirSync(path.dirname(file), { recursive: true }); fs.writeFileSync(file, text); };
let cases = 0;
function run(pages, sources = [], expected = 0, marker, drafts = false) {
  fs.rmSync(dist, { recursive: true, force: true }); fs.rmSync(articles, { recursive: true, force: true });
  fs.mkdirSync(dist); fs.mkdirSync(articles);
  for (const [route, html] of Object.entries(pages)) write(path.join(dist, route, 'index.html'), html);
  for (const [i, source] of sources.entries()) write(path.join(articles, `${i}.md`), source);
  const code = `import {layoutProblems,readArticles} from ${JSON.stringify(checker)}; import {builtPages} from ${JSON.stringify(listed)}; const p=layoutProblems(builtPages(${JSON.stringify(dist)}),readArticles(${JSON.stringify(articles)})); if(p.length){console.error(p.join('\\n'));process.exit(1);}`;
  const result = spawnSync(process.execPath, ['--input-type=module', '-e', code], { encoding: 'utf8', timeout: 10000, env: { PATH: process.env.PATH, THINKTHEN_DRAFTS: drafts ? '1' : '0' } });
  assert.equal(result.status, expected, result.stderr);
  if (marker) assert.ok(result.stderr.includes(marker), result.stderr);
  cases += 1;
}
const grid = count => `<main><div class="grid">\n<!-- <i>not a card</i> -->${Array.from({length:count}, () => '<a><b>Title</b><span>Nested text</span></a>').join('\n')}</div></main>`;
const source = (slug, date, line = 'Summary.', extra = '') => `---\nslug: ${slug}\ntitle: "${slug}"\ndate: "${date}"\n${extra}\n${line === undefined ? '' : 'line: ' + JSON.stringify(line)}\n---\nBody`;
const row = (slug, date, line = 'Summary.', draft = false) => `<li><time datetime="${date}">${date}</time> <a href="/blog/${slug}/">${slug}${draft ? ' (draft)' : ''}</a> <span>${line}</span></li>`;
const blog = rows => `<main><ol class="blog-posts">${rows}</ol></main>`;
try {
  for (const count of [2,3,4,6,9]) run({a:grid(count)});
  for (const count of [0,1,5,7,8,10]) run({a:grid(count)}, [], 1, 'grid-count');
  run({blog:grid(2)}, [], 1, 'blog-grid');
  run({blog:blog(row('a','2026-10-01'))}, [source('a','2026-10-01')]);
  run({blog:blog(row('a','2026-10-01'))}, [source('a','2026-10-01').replace('line: "Summary."\n','')], 1, 'post-line-missing');
  for (const [line, marker] of [['','post-line-missing'],['   ','post-line-missing'],['x'.repeat(101),'post-line-long'],['a\nb','post-line-newline'],[42,'post-line-missing']]) run({blog:blog(row('a','2026-10-01'))}, [source('a','2026-10-01',line)], 1, marker);
  run({blog:blog(row('a','2026-10-01','x'.repeat(100)))}, [source('a','2026-10-01','x'.repeat(100))]);
  run({blog:blog(row('a','2026-10-01','😀'.repeat(100)))}, [source('a','2026-10-01','😀'.repeat(100))]);
  run({blog:blog(row('a','2026-10-01'))}, [source('a','2026-10-01').replace('"Summary."', "'Summary.'")], 1, 'post-line-missing');
  run({blog:blog(row('a','2026-10-01'))}, [source('a','2026-10-01'),source('draft','2026-09-01','', 'draft: true')], 1, 'post-line-missing');
  const two = [source('a','2026-10-01'),source('b','2026-10-02')];
  run({blog:blog(row('b','2026-10-02')+row('a','2026-10-01'))}, two);
  run({blog:blog(row('a','2026-10-01')+row('b','2026-10-02'))}, two, 1, 'blog-order');
  run({blog:blog(row('a','2026-10-01'))}, two, 1, 'blog-posts');
  run({blog:blog(row('a','2026-10-01','Wrong blurb.'))}, [source('a','2026-10-01')], 1, 'blog-line');
  run({blog:blog(row('a','2026-10-01').replace('>a</a>','>Wrong</a>'))}, [source('a','2026-10-01')], 1, 'blog-title');
  run({blog:blog(row('a','2026-10-01').replace('datetime="2026-10-01"','datetime="2026-09-01"'))}, [source('a','2026-10-01')], 1, 'blog-date');
  run({blog:blog(row('a','2026-10-01')+row('d','2026-09-01','Summary.',true))}, [source('a','2026-10-01'),source('d','2026-09-01','Summary.','draft: true')], 0, undefined, true);
  run({blog:blog(row('a','2026-10-01')+row('b','2026-10-01'))}, [source('b','2026-10-01'),source('a','2026-10-01')]);
  console.log(`layout checker plants: ${cases} cases passed`);
} finally { fs.rmSync(root, { recursive: true, force: true }); }
