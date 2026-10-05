// Independent published/draft output fixtures; test input never promotes catalog entries.
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import { distProblems } from './check-recipes.mjs';
const root=fs.mkdtempSync(path.join(os.tmpdir(),'thinkthen-recipe-visibility-'));
const cleanup=p=>{if(p!==root)throw Error('cleanup refuses unowned path');fs.rmSync(p,{recursive:true});};
assert.throws(()=>cleanup(process.cwd()),/refuses unowned/);
const published={slug:'fixture-published',draft:false,body:[],measured:[],fixtureAssertions:[]};
const draft={slug:'fixture-draft',draft:true,body:[],measured:[],fixtureAssertions:[]};
const catalog=[published,draft];
const put=(p,s)=>{fs.mkdirSync(path.dirname(p),{recursive:true});fs.writeFileSync(p,s);};
let count=0;
try {
  for(const preview of [false,true]) {
    const dist=path.join(root,preview?'preview':'normal');
    const pages=preview?catalog:[published];
    const nav='<a href="/recipes/">Recipes</a>';
    const index=`<link rel="canonical" href="https://thinkthen.dev/recipes/"><header>${nav}</header><main>${pages.map(r=>`<a href="/recipes/${r.slug}/">${r.slug}</a>`).join('')}</main><footer>${nav}</footer>`;
    put(path.join(dist,'index.html'),`<header>${nav}</header><main>Home</main><footer>${nav}</footer>`);
    put(path.join(dist,'recipes/index.html'),index);put(path.join(dist,'recipes.md'),'# Recipes');
    let llms='',short='',sitemap='';
    for(const r of pages) {
      const md='# Recipe\n'+(r.draft?'Draft: waiting.':'Published recipe.');
      put(path.join(dist,`recipes/${r.slug}/index.html`),`<main data-recipe="${r.slug}"${r.draft?' data-draft':''}><h1>Recipe</h1><p>${r.draft?'Draft: waiting.':'Published recipe.'}</p></main>`);
      put(path.join(dist,`recipes/${r.slug}.md`),md);
      llms+=`Source: https://thinkthen.dev/recipes/${r.slug}/\n${md}\n---\n`;
      short+=`https://thinkthen.dev/recipes/${r.slug}.md\n`;sitemap+=`https://thinkthen.dev/recipes/${r.slug}/\n`;
    }
    put(path.join(dist,'llms-full.txt'),llms);put(path.join(dist,'llms.txt'),short);put(path.join(dist,'sitemap.xml'),sitemap);
    assert.deepEqual(distProblems(dist,catalog,preview),[]);count++;
    put(path.join(dist,'recipes/index.html'),index.replace('href="/recipes/fixture-published/"','href="/missing/"'));
    assert.ok(distProblems(dist,catalog,preview).includes('recipe visibility: index membership fixture-published'));count++;
    put(path.join(dist,'recipes/index.html'),index);
    put(path.join(dist,'index.html'),'<header></header><main>Home</main><footer></footer>');
    assert.ok(distProblems(dist,catalog,preview).includes('recipe visibility: index.html: footer navigation'));count++;
    put(path.join(dist,'index.html'),`<header>${nav}</header><main>Home</main><footer>${nav}</footer>`);
    const file=path.join(dist,'recipes/fixture-published.md'),md=fs.readFileSync(file,'utf8');
    fs.unlinkSync(file);assert.ok(distProblems(dist,catalog,preview).includes('recipe missing-artifact: fixture-published: HTML/Markdown twin'));count++;
    put(file,md);
    if(!preview) {
      put(path.join(dist,'recipes/fixture-draft/index.html'),'<main data-draft>Leaked draft</main>');
      assert.ok(distProblems(dist,catalog,preview).includes('recipe visibility: fixture-draft: excluded output'));count++;
    }
  }
  console.log(`recipe publication visibility: ${count} independent published/preview and omission controls passed`);
} finally {cleanup(root);}
