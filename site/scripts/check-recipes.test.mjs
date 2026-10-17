// Independent numerical and evidence-rendering controls; no catalog promotion.
import crypto from 'node:crypto';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { recipeProblems } from '../src/data/recipes.mjs';
import { renderedProblems, fixtureAssertionProblems } from './check-recipes.mjs';
import { ALIASES } from './redirect-contract.mjs';
const own=fs.mkdtempSync(path.join(os.tmpdir(),'thinkthen-recipe-check-'));
const cleanup=dir=>{ if(dir!==own)throw Error('cleanup refuses unowned path');fs.rmSync(dir,{recursive:true}); };
assert.throws(()=>cleanup(process.cwd()),/refuses unowned/);
const source='https://github.com/botassembly/thinkthen/blob/ea615c0e6a3412bb4716ad97b704a3f698be30cb/sdlc/records/2026-10-05-reviewed-experiment-planning.md';
const fixture={slug:'fixture-number',title:'A supported comparison',gain:'Keep measured support visible.',sourceCommit:'ea615c0e6a3412bb4716ad97b704a3f698be30cb',sourceRecord:'sdlc/records/2026-10-05-reviewed-experiment-planning.md',example:'site/examples/recipes/fixture-number',functions:['choose'],measured:[{id:'supported',evidenceClass:'measured',value:'503/626',denominator:'626',cohort:'Matched cohort: 849 fields on 144 documents',definition:'Exact-value precision',resolved:'849',unresolved:'0',backend:'TypeSafe',model:'unknown; scoped aggregate',qualification:'Tested choose confirmation only; equivalent decide performance was not established.',source}],fixtureAssertions:[],body:[{kind:'evidence',id:'supported'}]};
let count=0;
try {
  const file=path.join(own,'fixture.json'),checker=new URL('./check-recipes.mjs',import.meta.url).pathname;
  function check(r,exit,stderr='') {
    fs.writeFileSync(file,JSON.stringify([r]));
    const done=spawnSync(process.execPath,[checker,'--fixture',file],{encoding:'utf8',timeout:10000});
    assert.equal(done.status,exit,done.stderr);assert.equal(done.stderr,stderr);count++;
  }
  check(fixture,0);
  check({...fixture,body:[{kind:'text',text:'The method is 97.3% accurate.'}]},1,'recipe unsupported numerical prose: fixture-number: body[0]\n');
  check(fixture,0);
  check({...fixture,body:[{kind:'text',text:'Accuracy is ９７ percent.'}]},1,'recipe unsupported numerical prose: fixture-number: body[0]\n');
  check({...fixture,body:[{kind:'text',text:'Accuracy is &#57; percent.'}]},1,'recipe unsupported numerical prose: fixture-number: body[0]\n');
  const visible='503/626; denominator 626; Exact-value precision; Matched cohort: 849 fields on 144 documents; fields with answers 849, fields without answers 0; backend TypeSafe, model unknown; scoped aggregate; Tested choose confirmation only; equivalent decide performance was not established.';
  const html=`<main data-recipe="fixture-number"><h1>A supported comparison</h1><table data-recipe-evidence="supported" data-evidence-class="measured"><thead><tr><th>Evidence</th><th>Source</th></tr></thead><tbody><tr><td>${visible}</td><td><a href="${source}">Source</a></td></tr></tbody></table></main>`;
  const md=`# A supported comparison\n\n| Evidence | Source |\n| --- | --- |\n| ${visible} | [Source](${source}) |\n`;
  const llms='Source: https://thinkthen.dev/recipes/fixture-number/\n'+md;
  assert.deepEqual(renderedProblems(fixture,html,md,llms),[]);count++;
  for (const needle of ['503/626','denominator 626','Tested choose confirmation only;',source]) {
    assert.ok(renderedProblems(fixture,html.replace(needle,'altered'),md,llms).some(p=>p.includes('rendered-evidence')));count++;
  }
  assert.deepEqual(renderedProblems(fixture,html.replace('</main>','<p>The method is 97.3% accurate.</p></main>'),md,llms),['recipe unsupported numerical prose: fixture-number: rendered']);count++;
  for (const kind of ['md','llms']) { assert.ok(renderedProblems(fixture,html,kind==='md'?md.replace('503/626','lost'):md,kind==='llms'?llms.replace('503/626','lost'):llms).some(p=>p.includes('export-evidence')));count++; }
  const artifactDir=path.join(own,'files');fs.mkdirSync(artifactDir);
  const savedAssertion='{"population":8}\n';fs.writeFileSync(path.join(artifactDir,'harness.json'),savedAssertion);
  const artifactHash=crypto.createHash('sha256').update(savedAssertion).digest('hex');
  const control={...fixture,measured:[],fixtureAssertions:[{id:'controlled',evidenceClass:'fixture',value:'8',denominator:'8',scope:'Handwritten notes',qualification:'Controlled loopback fixture; no provider-quality measurement.',artifact:'files/harness.json',selector:'/population',sha256:artifactHash,artifactCommit:'1da71e8116cf89e33f2edc193b0c7ea3d19c32ff'}],body:[{kind:'evidence',id:'controlled'}]};
  assert.deepEqual(recipeProblems(control),[]);count++;
  assert.deepEqual(fixtureAssertionProblems(own,control.fixtureAssertions[0]),[]);count++;
  fs.writeFileSync(path.join(artifactDir,'harness.json'),'{"population":97}\n');
  assert.deepEqual(fixtureAssertionProblems(own,control.fixtureAssertions[0]),['recipe fixture assertion: controlled: value mismatch']);count++;
  fs.writeFileSync(path.join(artifactDir,'harness.json'),savedAssertion);
  const switched=structuredClone(control);switched.fixtureAssertions[0].evidenceClass='measured';assert.ok(recipeProblems(switched).includes('recipe evidence-class: fixture-number: controlled'));count++;
  const promoted=structuredClone(control);promoted.measured=[{...promoted.fixtureAssertions[0],evidenceClass:'measured'}];promoted.fixtureAssertions=[];assert.ok(recipeProblems(promoted).includes('recipe evidence-class: fixture-number: controlled'));count++;
  const replacement=structuredClone(fixture);replacement.body[0].value='97.3';assert.deepEqual(recipeProblems(replacement),['recipe carrier-shape: fixture-number: body[0]']);count++;
  const missing=structuredClone(fixture);delete missing.measured[0].unresolved;assert.ok(recipeProblems(missing).includes('recipe measurement-definition: fixture-number: supported/unresolved'));count++;
  const unpinned=structuredClone(fixture);unpinned.measured[0].source=source.replace(/[a-f0-9]{40}/,'main');assert.deepEqual(recipeProblems(unpinned),[]);count++;
  const cell=(key,value)=>({id:`rules-full-${key}`,evidenceClass:'measured',value,denominator:'10',cohort:'Ten fields',definition:key,resolved:'10',unresolved:'0',backend:'not applicable',model:'not applicable',qualification:'Fixture.',source});
  const grid={...fixture,measured:[cell('tp','7'),cell('fp','2'),cell('fn','1')],body:[{kind:'evidence-grid',methods:['rules'],cohorts:['full']}]};
  assert.deepEqual(recipeProblems(grid),[]);count++;
  assert.ok(recipeProblems({...grid,measured:grid.measured.slice(0,2)}).includes('recipe carrier-shape: fixture-number: body[0]/rules-full'));count++;
  const gridHtml=`<main data-recipe="fixture-number"><h1>A supported comparison</h1><table data-recipe-evidence-grid><tbody><tr data-grid-row="rules-full"><td>rules</td><td>Ten fields</td><td>7</td><td>2</td><td>1</td><td><a href="${source}">Source</a></td></tr></tbody></table></main>`;
  const gridMd=`| rules | Ten fields | 7 | 2 | 1 | [Source](${source}) |`;
  assert.deepEqual(renderedProblems(grid,gridHtml,gridMd,gridMd),[]);count++;
  assert.ok(renderedProblems(grid,gridHtml.replace('<td>7</td>','<td>8</td>'),gridMd,gridMd).some(p=>p.includes('rendered-evidence: fixture-number: grid rules-full')));count++;
  assert.equal(Object.keys(ALIASES).length,169);
  console.log(`recipe independent controls: ${count} passed; exact unsupported-literal exit/cause and supported HTML/Markdown/llms`);
} finally {cleanup(own);}
