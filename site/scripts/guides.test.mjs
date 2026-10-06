// Render independent lessons through the real site; fixtures never enter its catalog.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import zlib from 'node:zlib';
import { guideProblems } from '../src/lib/guide.mjs';

const metadata = {
  slug: 'test-lesson', title: 'One question', blurb: 'Ask one bounded question.',
  goal: 'Read an answer.', draft: false, prerequisites: ['Install ThinkThen.'],
  limits: ['Check your own data.'], video: 'https://www.youtube.com/watch?v=YbzrlpAyCV4',
  blog: '/blog/introducing-thinkthen/',
};
assert.deepEqual(guideProblems(metadata), []);
for (const field of ['video', 'blog']) {
  for (const value of [undefined, 'https://example.com/placeholder']) {
    assert.ok(guideProblems({ ...metadata, [field]: value }).some(p => p.startsWith(`guide ${field} requires`)));
  }
}
assert.deepEqual(guideProblems({ ...metadata, draft: true, video: undefined, blog: undefined }), []);

const site = fileURLToPath(new URL('../', import.meta.url));
const root = fs.mkdtempSync(path.join(os.tmpdir(), 'thinkthen-guides-'));
const cleanup = p => {
  if (p !== root) throw Error('cleanup refuses unowned path');
  fs.rmSync(p, { recursive: true });
};
assert.throws(() => cleanup(site), /refuses unowned path/);
const work = path.join(root, 'site');
const read = file => fs.readFileSync(path.join(work, 'dist', file), 'utf8');
const run = (cmd, args, preview = false) => {
  const done = spawnSync(cmd, args, {
    cwd: work, env: { PATH: process.env.PATH, HOME: process.env.HOME, LANG: 'C.UTF-8', ASTRO_TELEMETRY_DISABLED: '1',
      ...(preview ? { THINKTHEN_DRAFTS: '1' } : {}) }, encoding: 'utf8',
  });
  assert.equal(done.status, 0, `${cmd} ${args.join(' ')}\n${done.stdout}\n${done.stderr}`);
};
try {
  fs.mkdirSync(work);
  fs.cpSync(path.join(site, 'src'), path.join(work, 'src'), { recursive: true });
  for (const name of ['scripts', 'examples', 'public', 'node_modules']) {
    fs.symlinkSync(path.join(site, name), path.join(work, name));
  }
  for (const name of ['specification', 'databases']) {
    fs.symlinkSync(path.join(site, '..', name), path.join(root, name));
  }
  fs.copyFileSync(path.join(site, 'package.json'), path.join(work, 'package.json'));
  const config = fs.readFileSync(path.join(site, 'astro.config.mjs'), 'utf8');
  fs.writeFileSync(path.join(work, 'astro.config.mjs'), config.replace("output: 'static',", "output: 'static', cacheDir: './.cache/astro',"));
  fs.mkdirSync(path.join(work, 'src/guides'), { recursive: true });
  for (const name of ['complete', 'draft']) {
    fs.copyFileSync(new URL(`fixtures/guides/${name}.md`, import.meta.url), path.join(work, `src/guides/${name}.md`));
  }
  for (const preview of [false, true]) {
    run(process.execPath, [path.join(site, 'node_modules/astro/astro.js'), 'build'], preview);
    run(process.execPath, [path.join(site, 'scripts/emit-md.mjs')]);
    run(process.execPath, [path.join(site, 'scripts/write-sitemap.mjs')]);
    const index = read('guides/index.html');
    const lesson = read('guides/fixture-complete/index.html');
    assert.match(index, /href="\/guides\/fixture-complete\/"/);
    assert.doesNotMatch(index, /fixture-draft/);
    for (const href of ['https://www.youtube.com/watch?v=YbzrlpAyCV4', '/blog/introducing-thinkthen/']) {
      assert.ok(lesson.includes(`href="${href}"`), `actual metadata link ${href}`);
      assert.ok(read('guides/fixture-complete.md').includes(`](${href})`));
    }
    assert.match(lesson, /Prerequisites/);
    assert.match(lesson, /This fixture checks presentation, not model quality\./);
    assert.match(read('guides/fixture-complete.md'), /Does the customer ask for a refund/);
    assert.match(lesson, /data-pagefind-body/);
    assert.match(lesson, /href="\/guides\/" aria-current="page"/);
    assert.match(read('sitemap.xml'), /https:\/\/thinkthen.dev\/guides\/fixture-complete\//);
    for (const file of ['sitemap.xml', 'llms.txt', 'llms-full.txt']) assert.doesNotMatch(read(file), /fixture-draft/);
    const draftFile = path.join(work, 'dist/guides/fixture-draft/index.html');
    assert.equal(fs.existsSync(draftFile), preview);
    if (preview) {
      const draft = read('guides/fixture-draft/index.html');
      assert.match(draft, /Draft: this lesson is incomplete/);
      assert.match(draft, /data-unlisted/);
      assert.doesNotMatch(draft, /data-pagefind-body|rel="canonical"/);
    }
    run(process.execPath, [path.join(site, 'node_modules/pagefind/lib/runner/bin.cjs'), '--site', 'dist', '--exclude-selectors', 'button']);
    const fragmentDir = path.join(work, 'dist/pagefind/fragment');
    const fragments = fs.readdirSync(fragmentDir).map(file =>
      zlib.gunzipSync(fs.readFileSync(path.join(fragmentDir, file))).toString()
    ).join('\n');
    assert.match(fragments, /\/guides\/fixture-complete\//);
    assert.doesNotMatch(fragments, /fixture-draft/);
  }
  console.log('guides: real lesson media, examples, navigation, exports, search and hidden drafts passed');
} finally { cleanup(root); }
