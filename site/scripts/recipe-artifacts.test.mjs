import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {RECIPE_PAGES} from '../src/data/catalog.mjs';
const root=fs.mkdtempSync(path.join(os.tmpdir(),'thinkthen-recipe-metadata-'));
const cleanup=p=>{if(p!==root)throw Error('cleanup refuses unowned path');fs.rmSync(p,{recursive:true});};
assert.throws(()=>cleanup(process.cwd()),/refuses unowned/);
let cases=0;
try{
 fs.mkdirSync(path.join(root,'site/examples/recipes'),{recursive:true});
 fs.cpSync(new URL('../examples/recipes/rules-propose-model-confirms',import.meta.url),path.join(root,'site/examples/recipes/rules-propose-model-confirms'),{recursive:true});
 fs.mkdirSync(path.join(root,'sdlc/issues'),{recursive:true});fs.mkdirSync(path.join(root,'sdlc/planning'),{recursive:true});
 for(const r of RECIPE_PAGES)fs.copyFileSync(new URL('../../'+r.issue,import.meta.url),path.join(root,r.issue));
 fs.copyFileSync(new URL('../../sdlc/planning/milestones.md',import.meta.url),path.join(root,'sdlc/planning/milestones.md'));
 const {sourceProblems}=await import('./check-recipes.mjs');
 const check=()=>sourceProblems(RECIPE_PAGES,root);
 assert.deepEqual(check(),[]);cases++;
 const base=path.join(root,RECIPE_PAGES[0].example);
 for(const file of ['files/question.json','files/key.jsonl','files/recording/thinkthen.jsonl','files/audit.jsonl','files/provenance.json']){
  const p=path.join(base,file),saved=fs.readFileSync(p);fs.unlinkSync(p);
  assert.ok(check().includes(`recipe missing-artifact: rules-propose-model-confirms: ${file}`));cases++;
  fs.writeFileSync(p,saved);assert.deepEqual(check(),[]);cases++;
 }
 for(const file of ['files/cases.jsonl','files/key.jsonl','files/recording/thinkthen.jsonl']){
  const p=path.join(base,file),saved=fs.readFileSync(p);fs.appendFileSync(p,'\n');
  assert.ok(check().includes(`recipe hash mismatch: rules-propose-model-confirms: ${file}`));cases++;
  fs.writeFileSync(p,saved);assert.deepEqual(check(),[]);cases++;
 }
 const issue=path.join(root,RECIPE_PAGES[3].issue),saved=fs.readFileSync(issue);
 fs.unlinkSync(issue);assert.ok(check().includes('recipe scope: navigate-many-documents: missing issue'));cases++;
 fs.writeFileSync(issue,saved);
 const milestone=path.join(root,'sdlc/planning/milestones.md'),m=fs.readFileSync(milestone,'utf8');
 fs.writeFileSync(milestone,m.replaceAll('ask-your-cache-with-duckdb','omitted'));
 assert.ok(check().includes('recipe scope: ask-your-cache-with-duckdb: issue/milestone disposition'));cases++;
 fs.writeFileSync(milestone,m);assert.deepEqual(check(),[]);cases++;
 const clone=structuredClone(RECIPE_PAGES);clone.push({...clone[2],slug:'qualify'});
 assert.ok(sourceProblems(clone,root).includes('recipe disposition: verify/qualify must remain shared'));cases++;
 console.log(`recipe metadata/artifact controls: ${cases} passed, required issues/slugs/milestones and actual byte omission/mutation`);
}finally{cleanup(root);}
