import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {ALIASES,routeFile,preserver} from './redirect-contract.mjs';
import {RECIPE_PAGES} from '../src/data/catalog.mjs';
import {recipeSelection} from '../src/data/recipes.mjs';
const root=fs.mkdtempSync(path.join(os.tmpdir(),'thinkthen-recipe-transform-'));
const cleanup=p=>{if(p!==root)throw Error('cleanup refuses unowned path');fs.rmSync(p,{recursive:true});};
assert.throws(()=>cleanup(process.cwd()),/refuses unowned/);
fs.symlinkSync(new URL('../../specification',import.meta.url).pathname,path.join(root,'specification'));
const script=new URL('./preserve-redirect-fragments.mjs',import.meta.url).pathname;
const put=(p,s)=>{fs.mkdirSync(path.dirname(p),{recursive:true});fs.writeFileSync(p,s);};
const stub=f=>`<meta http-equiv="refresh" content="0;url=${f}"><link rel="canonical" href="https://thinkthen.dev${f}"><meta name="robots" content="noindex"><a href="${f}">Continue</a>`;
const index='<link rel="canonical" href="https://thinkthen.dev/recipes/"><main data-draft><p>Draft catalog: waiting.</p></main>';
let cases=0;
try{
 for(const preview of [false,true]){
  const state=recipeSelection(RECIPE_PAGES,preview);
  const recipeIndex=state.draftIndex?index:index.replace(' data-draft','').replace('Draft catalog: waiting.','Published recipes.');
  const cwd=path.join(root,preview?'preview':'normal'),dist=path.join(cwd,'dist');
  for(const [a,f] of Object.entries(ALIASES)){
   put(routeFile(dist,new URL(f,'https://thinkthen.dev').pathname),'<main><h2 id="edge-cases">Edges</h2></main>');
   put(routeFile(dist,a),a==='/recipes'&&state.index?recipeIndex:stub(f));
  }
  const run=()=>spawnSync(process.execPath,[script],{cwd,encoding:'utf8',env:{...process.env,THINKTHEN_DRAFTS:preview?'1':'0'}});
  const alias=routeFile(dist,'/surfaces');fs.unlinkSync(alias);
  const before=fs.readFileSync(routeFile(dist,'/backends'),'utf8');
  let done=run();assert.equal(done.status,1);assert.equal(done.stderr,'alias compatibility: /surfaces: missing redirect stub\n');assert.equal(fs.readFileSync(routeFile(dist,'/backends'),'utf8'),before);cases++;
  put(alias,stub('/install/'));done=run();assert.equal(done.status,0,done.stderr);cases++;
  assert.equal(fs.readFileSync(routeFile(dist,'/recipes'),'utf8'),state.index?recipeIndex:stub('/how-tos/bash/').replace('<meta http-equiv="refresh" content="0;url=/how-tos/bash/">',preserver('/how-tos/bash/')+'<noscript><meta http-equiv="refresh" content="0;url=/how-tos/bash/"></noscript>'));cases++;
  for(const [a,f]of Object.entries(ALIASES))if(a!=='/recipes'||!state.index)assert.ok(fs.readFileSync(routeFile(dist,a),'utf8').includes(preserver(f)));cases++;
  if(state.index){put(routeFile(dist,'/recipes'),stub('/how-tos/bash/'));done=run();assert.equal(done.status,1);assert.match(done.stderr,/expected recipe index, found redirect/);cases++;}
 }
 console.log(`recipe fragment transformer: ${cases} actual controls; validates complete input before editing, preview index unchanged, every other alias preserved`);
}finally{cleanup(root);}
