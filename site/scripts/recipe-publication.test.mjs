// Independent visibility fixtures for the recipe index, its navigation and
// each recipe's exports.
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import { HOWTOS, SHELL_JOBS } from '../src/data/catalog.mjs';
import { distProblems } from './check-recipes.mjs';
const root=fs.mkdtempSync(path.join(os.tmpdir(),'thinkthen-recipe-visibility-'));
const cleanup=p=>{if(p!==root)throw Error('cleanup refuses unowned path');fs.rmSync(p,{recursive:true});};
assert.throws(()=>cleanup(process.cwd()),/refuses unowned/);
const recipe={slug:'fixture-published',body:[],measured:[],fixtureAssertions:[]};
const catalog=[recipe];
const put=(p,s)=>{fs.mkdirSync(path.dirname(p),{recursive:true});fs.writeFileSync(p,s);};
let count=0;
try {
  const dist=path.join(root,'dist');
  const nav='<a href="/recipes/">Recipes</a>';
  const slugs=[recipe.slug,...[...HOWTOS,...SHELL_JOBS].map(j=>j.slug)];
  const index=`<header>${nav}</header><main>${slugs.map(s=>`<a href="/recipes/${s}/">${s}</a>`).join('')}</main><footer>${nav}</footer>`;
  put(path.join(dist,'index.html'),`<header>${nav}</header><main>Home</main><footer>${nav}</footer>`);
  put(path.join(dist,'recipes/index.html'),index);put(path.join(dist,'recipes.md'),'# Recipes\n');
  const md='# Recipe\nA recipe.';
  put(path.join(dist,`recipes/${recipe.slug}/index.html`),`<main data-recipe="${recipe.slug}"><h1>Recipe</h1><p>A recipe.</p></main>`);
  put(path.join(dist,`recipes/${recipe.slug}.md`),md);
  put(path.join(dist,'llms-full.txt'),`Source: https://thinkthen.dev/recipes/${recipe.slug}/\n${md}\n---\n`);
  put(path.join(dist,'llms.txt'),`https://thinkthen.dev/recipes/${recipe.slug}.md\n`);
  put(path.join(dist,'sitemap.xml'),`https://thinkthen.dev/recipes/${recipe.slug}/\n`);
  assert.deepEqual(distProblems(dist,catalog),[]);count++;
  put(path.join(dist,'recipes/index.html'),index.replace(`href="/recipes/${recipe.slug}/"`,'href="/missing/"'));
  assert.ok(distProblems(dist,catalog).includes(`recipe visibility: index membership ${recipe.slug}`));count++;
  put(path.join(dist,'recipes/index.html'),index.replace('</main>','<p>Pick one of 3 jobs.</p></main>'));
  assert.ok(distProblems(dist,catalog).includes('recipe unsupported numerical prose: index: rendered'));count++;
  put(path.join(dist,'recipes/index.html'),index);
  put(path.join(dist,'index.html'),'<header></header><main>Home</main><footer></footer>');
  assert.ok(distProblems(dist,catalog).includes('recipe visibility: index.html: footer navigation'));count++;
  put(path.join(dist,'index.html'),`<header>${nav}</header><main>Home</main><footer>${nav}</footer>`);
  const file=path.join(dist,`recipes/${recipe.slug}.md`);
  fs.unlinkSync(file);assert.ok(distProblems(dist,catalog).includes(`recipe missing-artifact: ${recipe.slug}: HTML/Markdown twin`));count++;
  put(file,md);
  put(path.join(dist,'recipes/stray/index.html'),'<main>Stray</main>');
  assert.ok(distProblems(dist,catalog).includes('recipe visibility: unknown stray'));count++;
  console.log(`recipe publication visibility: ${count} index, navigation, export and stray-page controls passed`);
} finally {cleanup(root);}
