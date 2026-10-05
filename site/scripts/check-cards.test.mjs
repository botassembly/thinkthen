// Independent head/image fixtures invoke the executable checker.
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
const root = fs.mkdtempSync(path.join(os.tmpdir(), 'thinkthen-cards-'));
const checker = new URL('./check-cards.mjs', import.meta.url).pathname;
const image = path.join(root, 'og.png');
let cases = 0;
const title = 'A &amp; B &quot;quoted&quot; &#39;apostrophe&#39;';
const description = 'A &#x26; B &quot;quoted&quot; &apos;apostrophe&apos;';
const meta = (name, content) => `<meta ${name.startsWith('og:') ? 'property' : 'name'}="${name}" content="${content}">`;
const head = (unlisted = false) => `<html${unlisted ? ' data-unlisted' : ''}><head><title>${title}</title>${unlisted ? '' : '<link rel="canonical" href="https://thinkthen.dev/">'}${[['description',description],['og:title',description],['og:description',title],['og:url',unlisted ? 'https://thinkthen.dev/search/' : 'https://thinkthen.dev/'],['og:image','https://thinkthen.dev/og.png'],['og:image:alt','Card text'],['twitter:title',title],['twitter:description',description],['twitter:card','summary_large_image'],['twitter:image','https://thinkthen.dev/og.png'],['twitter:image:alt','Card text']].map(([n,v])=>meta(n,v)).join('')}</head><main>Body</main></html>`;
const png = (width = 1200, height = 630) => { const b = Buffer.alloc(24); Buffer.from([137,80,78,71,13,10,26,10]).copy(b); b.write('IHDR',12); b.writeUInt32BE(width,16); b.writeUInt32BE(height,20); return b; };
function run(html, expected = 0, marker, unlisted = false, notFound = false) {
  const file = notFound ? path.join(root, '404.html') : unlisted ? path.join(root,'search/index.html') : path.join(root,'index.html');
  fs.mkdirSync(path.dirname(file),{recursive:true}); fs.writeFileSync(file,html);
  const result = spawnSync(process.execPath,[checker,root],{encoding:'utf8',timeout:10000});
  assert.equal(result.status, expected, result.stderr);
  if (marker) assert.ok(result.stderr.includes(marker), result.stderr);
  fs.unlinkSync(file); cases += 1;
}
try {
  fs.writeFileSync(image,png()); run(head());
  for (const [name,code] of [['og:title','og:title-equality'],['og:description','og:description-equality'],['og:url','og:url-equality'],['twitter:title','twitter:title-equality'],['twitter:description','twitter:description-equality']]) {
    const attr = name.startsWith('og:') ? 'property' : 'name';
    run(head().replace(new RegExp(`<meta ${attr}="${name}" content="[^"]*">`),meta(name,'wrong')),1,code); run(head());
  }
  for (const name of ['og:image:alt','twitter:image:alt']) {
    run(head().replace(meta(name,'Card text'),''),1,`${name}-count`); run(head());
    run(head().replace(meta(name,'Card text'),meta(name,'')),1,`${name}-empty`);
  }
  run(head().replace('</head>',meta('og:title',title)+'</head>'),1,'og:title-count');
  run(head().replace(meta('description',description),''),1,'description-count');
  run(head().replace(`<title>${title}</title>`,''),1,'title-count');
  run(head().replace(meta('twitter:image','https://thinkthen.dev/og.png'),meta('twitter:image','https://thinkthen.dev/missing.png')),1,'image-equality');
  run(head().replaceAll('https://thinkthen.dev/og.png','https://other.example/og.png'),1,'og:image-address');
  run(head().replace('summary_large_image','summary'),1,'twitter:card-value');
  fs.unlinkSync(image); run(head(),1,'og:image-missing');
  fs.writeFileSync(image,Buffer.from('PNG')); run(head(),1,'og:image-png-size');
  fs.writeFileSync(image,png(100,630)); run(head(),1,'og:image-png-size');
  fs.writeFileSync(image,Buffer.alloc(24)); run(head(),1,'og:image-png-size');
  fs.writeFileSync(image,png()); run(head());
  run(head(true),0,undefined,true);
  run(head(true).replace('https://thinkthen.dev/search/', 'https://thinkthen.dev/404.html'),0,undefined,true,true);
  run(head(true).replace('https://thinkthen.dev/search/', 'https://thinkthen.dev/404/'),1,'og:url-equality',true,true);
  run(head(true).replace(meta('og:url','https://thinkthen.dev/search/'),meta('og:url','https://thinkthen.dev/')),1,'og:url-equality',true);
  run('<html><head><meta http-equiv="refresh" content="0;url=/"></head></html>');
  console.log(`card checker plants: ${cases} cases passed`);
} finally { fs.rmSync(root,{recursive:true,force:true}); }
