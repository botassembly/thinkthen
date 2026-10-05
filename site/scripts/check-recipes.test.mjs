// Independent numerical controls and four visibility states; no catalog promotion.
import crypto from 'node:crypto';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { recipeProblems, recipeSelection } from '../src/data/recipes.mjs';
import { renderedProblems, fixtureAssertionProblems } from './check-recipes.mjs';
import { ALIASES, preserver, validateCompatibility } from './redirect-contract.mjs';
const own=fs.mkdtempSync(path.join(os.tmpdir(),'thinkthen-recipe-check-'));
const cleanup=dir=>{ if(dir!==own)throw Error('cleanup refuses unowned path');fs.rmSync(dir,{recursive:true}); };
assert.throws(()=>cleanup(process.cwd()),/refuses unowned/);
const source='https://github.com/botassembly/thinkthen/blob/ea615c0e6a3412bb4716ad97b704a3f698be30cb/sdlc/records/2026-10-05-reviewed-experiment-planning.md';
const fixture={slug:'fixture-number',title:'A supported comparison',goal:'Inspect a matched comparison.',job:'Keep measured support visible.',wait:'Waiting for publication.',draft:true,publication:'waiting',owner:'Queue owner',issue:'sdlc/issues/fixture.md',sourceCommit:'ea615c0e6a3412bb4716ad97b704a3f698be30cb',sourceRecord:'sdlc/records/2026-10-05-reviewed-experiment-planning.md',example:'site/examples/recipes/fixture-number',functions:['choose'],measured:[{id:'supported',evidenceClass:'measured',value:'503/626',denominator:'626',cohort:'Matched cohort: 849 fields on 144 documents',definition:'Exact-value precision',resolved:'849',unresolved:'0',backend:'TypeSafe',model:'unknown; scoped aggregate',qualification:'Tested choose confirmation only; equivalent decide performance was not established.',source}],fixtureAssertions:[],body:[{kind:'evidence',id:'supported'}]};
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
  const html=`<main data-recipe="fixture-number" data-draft><h1>A supported comparison</h1><p>Draft: a supported comparison awaits publication.</p><table data-recipe-evidence="supported" data-evidence-class="measured"><thead><tr><th>Evidence</th><th>Source</th></tr></thead><tbody><tr><td>${visible}</td><td><a href="${source}">Owning record</a></td></tr></tbody></table></main>`;
  const md=`# A supported comparison\n\nDraft: a supported comparison awaits publication.\n\n| Evidence | Source |\n| --- | --- |\n| ${visible} | [Owning record](${source}) |\n`;
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
  assert.deepEqual(fixtureAssertionProblems(own,control.fixtureAssertions[0]),['recipe fixture assertion: controlled: value/hash mismatch']);count++;
  fs.writeFileSync(path.join(artifactDir,'harness.json'),savedAssertion);
  const switched=structuredClone(control);switched.fixtureAssertions[0].evidenceClass='measured';assert.ok(recipeProblems(switched).includes('recipe evidence-class: fixture-number: controlled'));count++;
  const promoted=structuredClone(control);promoted.measured=[{...promoted.fixtureAssertions[0],evidenceClass:'measured'}];promoted.fixtureAssertions=[];assert.ok(recipeProblems(promoted).includes('recipe evidence-class: fixture-number: controlled'));count++;
  const replacement=structuredClone(fixture);replacement.body[0].value='97.3';assert.deepEqual(recipeProblems(replacement),['recipe carrier-shape: fixture-number: body[0]']);count++;
  const missing=structuredClone(fixture);delete missing.measured[0].unresolved;assert.ok(recipeProblems(missing).includes('recipe measurement-definition: fixture-number: supported/unresolved'));count++;
  const unpinned=structuredClone(fixture);unpinned.measured[0].source=source.replace(/[a-f0-9]{40}/,'main');assert.ok(recipeProblems(unpinned).includes('recipe source-pin: fixture-number: supported'));count++;
  const drafts=[{slug:'draft',draft:true}],mixed=[{slug:'published',draft:false},{slug:'draft',draft:true}];
  for(const [catalog,preview,index,navigation,draftIndex,pages] of [[drafts,false,false,false,false,[]],[drafts,true,true,false,true,['draft']],[mixed,false,true,true,false,['published']],[mixed,true,true,true,true,['published','draft']]]) {
    const selected=recipeSelection(catalog,preview);assert.equal(selected.index,index);assert.equal(selected.navigation,navigation);assert.equal(selected.draftIndex,draftIndex);assert.deepEqual(selected.pages.map(r=>r.slug),pages);count++;
    if(index) {
      const h=`<link rel="canonical" href="https://thinkthen.dev/recipes/"><main${draftIndex?' data-draft':''}><p>${draftIndex?'Draft catalog:':'Published catalog'}</p></main>`;
      assert.equal(validateCompatibility(h,'/recipes',ALIASES['/recipes'],own,true,selected),null);
      assert.throws(()=>validateCompatibility(h.replace('<main','<meta http-equiv="refresh"><main'),'/recipes',ALIASES['/recipes'],own,true,selected),/found redirect/);count++;
    }
  }
  assert.equal(Object.keys(ALIASES).length,61);
  console.log(`recipe independent controls: ${count} passed; exact unsupported-literal exit/cause, supported HTML/Markdown/llms and four selection states`);
} finally {cleanup(own);}
