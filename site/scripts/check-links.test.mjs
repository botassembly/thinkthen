// Outside-in scratch built-site cases exercise the real checker and fixed inventory.
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { ALIASES, routeFile, preserver } from './redirect-contract.mjs';
const root = fs.mkdtempSync(path.join(os.tmpdir(), 'thinkthen-links-'));
const dist = path.join(root, 'dist');
fs.symlinkSync(new URL('../../specification', import.meta.url).pathname, path.join(root, 'specification'));
const checker = new URL('./check-links.mjs', import.meta.url).pathname;
const write = (file, text) => { fs.mkdirSync(path.dirname(file), { recursive: true }); fs.writeFileSync(file, text); };
const stub = fixed => `<!doctype html><title>Redirect</title>${preserver(fixed)}<noscript><meta http-equiv="refresh" content="0;url=${fixed}"></noscript><meta name="robots" content="noindex"><link rel="canonical" href="https://thinkthen.dev${fixed}"><body><a href="${fixed}">Continue</a></body>`;
try {
  for (const [alias, fixed] of Object.entries(ALIASES)) {
    write(routeFile(dist, new URL(fixed, 'https://thinkthen.dev').pathname), '<main><h1 id="flags">Flags</h1><h2 id="edge-cases">Edges</h2></main>');
    write(routeFile(dist, alias), stub(fixed));
  }
  const page = routeFile(dist, '/fixture/');
  function run(link, expected, marker) {
    write(page, `<main><a href="${link}">Read</a></main>`);
    const result = spawnSync(process.execPath, [checker], { cwd: root, encoding: 'utf8', timeout: 10000 });
    assert.equal(result.status, expected, result.stderr);
    if (marker) assert.match(result.stderr, marker);
  }
  run('/functions/decide/#flags', 0);
  run('/reference/functions/decide/', 1, /redirect target:/);
  run('https://thinkthen.dev/reference/functions/decide/?x=1#flags', 1, /redirect target:/);
  run('../reference/functions/decide/', 1, /redirect target:/);
  run('/functions/decide/#missing', 1, /missing anchor/);
  const alias = routeFile(dist, '/surfaces'); const original = fs.readFileSync(alias, 'utf8');
  fs.unlinkSync(alias); run('/functions/decide/#flags', 1, /alias compatibility: \/surfaces: missing redirect stub/);
  write(alias, original);
  const dest = routeFile(dist, '/install/'); const canonical = fs.readFileSync(dest, 'utf8');
  write(dest, stub('/functions/decide/')); run('/functions/decide/#flags', 1, /redirect chain or cycle/); write(dest, canonical);
  for (const [mutate, marker] of [
    [s => s.replace('<noscript>', '').replace('</noscript>', ''), /active refresh/],
    [s => s.replace('content="noindex"', 'content="index"'), /stub publication shape/],
    [s => s.replace('rel="canonical" href="https://thinkthen.dev/install/"', 'rel="canonical" href="https://thinkthen.dev/functions/"'), /destinations disagree/],
    [s => s.replace(preserver('/install/'), ''), /missing fragment preserver/],
  ]) { write(alias, mutate(original)); run('/functions/decide/#flags', 1, marker); }
  write(alias, original); run('/functions/decide/#flags', 0);
  console.log('link checker plants: 12 cases passed');
} finally { fs.rmSync(root, { recursive: true, force: true }); }
