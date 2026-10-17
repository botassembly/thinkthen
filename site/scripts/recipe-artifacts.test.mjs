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
 for(const r of RECIPE_PAGES) fs.cpSync(new URL('../'+r.example.replace(/^site\//,''),import.meta.url),path.join(root,r.example),{recursive:true});
 const {sourceProblems}=await import('./check-recipes.mjs');
 const check=()=>sourceProblems(RECIPE_PAGES,root);
 assert.deepEqual(check(),[]);cases++;
 const base=path.join(root,RECIPE_PAGES[0].example);
 for(const file of ['files/question.json','files/key.jsonl','files/recording/thinkthen.jsonl','files/results.jsonl']){
  const p=path.join(base,file),saved=fs.readFileSync(p);fs.unlinkSync(p);
  assert.ok(check().includes(`recipe missing-artifact: rules-propose-model-confirms: ${file}`));cases++;
  fs.writeFileSync(p,saved);assert.deepEqual(check(),[]);cases++;
 }
 for(const [slug,file] of [['verify-a-claim','files/recording/thinkthen.jsonl']]) {
  const p=path.join(root,'site/examples/recipes',slug,file),saved=fs.readFileSync(p);fs.unlinkSync(p);
  assert.ok(check().includes(`recipe missing-artifact: ${slug}: ${file}`));cases++;
  fs.writeFileSync(p,saved);assert.deepEqual(check(),[]);cases++;
 }
 console.log(`recipe metadata/artifact controls: ${cases} passed, missing runnable files`);
}finally{cleanup(root);}
