// Closed source, artifact and actual export checks for the measured recipes.
import fs from 'node:fs';
import path from 'node:path';
import { pathToFileURL, fileURLToPath } from 'node:url';
import { RECIPE_PAGES, HOWTOS, SHELL_JOBS } from '../src/data/catalog.mjs';
import { recipeProblems, evidenceText, evidenceSource, hasDecimal, gridRows, GRID_COLUMNS, EVIDENCE_LABEL } from '../src/data/recipes.mjs';
import { parseHtml, findElement, textContent } from '../src/lib/html.mjs';
import { routeFile } from './redirect-contract.mjs';
import { isStub } from '../src/lib/listed-pages.mjs';
// Reader recipes and shell jobs also live under /recipes/.
const JOBS = [...HOWTOS, ...SHELL_JOBS].map((j) => j.slug);
const ROOT = fileURLToPath(new URL('../../',import.meta.url));
const raw = node => node.tag === '#text' ? node.text : (node.kids || []).map(raw).join('');
const tidy = text => text.replace(/\s+/g,' ').trim();
const walk = (n, fn) => { fn(n); for (const k of n.kids || []) walk(k,fn); };
export function fixtureAssertionProblems(base,e) {
  try {
    let value=JSON.parse(fs.readFileSync(path.join(base,e.artifact)));
    for (const part of e.selector.slice(1).split('/')) value=value?.[part];
    return String(value)===e.value ? [] : [`recipe fixture assertion: ${e.id}: value mismatch`];
  } catch { return [`recipe fixture assertion: ${e.id}: missing/malformed artifact`]; }
}
export function artifactProblems(r, root = ROOT) {
  const base=path.join(root,r.example), problems=[];
  const required = new Set(r.body.filter(n=>n.kind==='artifact').map(n=>n.file));
  if (r.slug==='rules-propose-model-confirms' || r.slug==='verify-a-claim') {
    for (const file of ['files/question.json','files/cases.jsonl','files/key.jsonl','files/recording/thinkthen.jsonl','1-ask.sh','1-ask.out','2-audit.sh','2-audit.out']) required.add(file);
  }
  if (r.slug==='rules-propose-model-confirms') required.add('files/results.jsonl');
  for (const file of required) if (!fs.existsSync(path.join(base,file))) problems.push(`recipe missing-artifact: ${r.slug}: ${file}`);
  for (const e of r.fixtureAssertions) problems.push(...fixtureAssertionProblems(base,e));
  return problems;
}
export function renderedProblems(r,html,markdown,llms,root=ROOT) {
  const out=[],add=(code,where)=>out.push(`recipe ${code}: ${r.slug}: ${where}`);
  const main=findElement(parseHtml(html),n=>n.tag==='main');
  if (!main || main.attrs['data-recipe']!==r.slug) return [`recipe rendered: ${r.slug}: missing main`];
  const evidence=new Map([...r.measured,...r.fixtureAssertions].map(e=>[e.id,e]));
  const seen=new Set(),artifacts=new Set();
  function inspect(n) {
    if (Object.hasOwn(n.attrs || {},'data-recipe-evidence')) {
      const id=n.attrs['data-recipe-evidence'],e=evidence.get(id);
      const label=EVIDENCE_LABEL[e?.evidenceClass];
      const a=findElement(n,k=>k.tag==='a');
      const cells=[]; walk(n,k=>{if(k.tag==='td')cells.push(k);});
      if (!e || seen.has(id) || n.attrs['data-evidence-class']!==e.evidenceClass || cells.length!==2 || tidy(textContent(cells[0]))!==tidy(evidenceText(e)) || tidy(textContent(cells[1]))!==label || a?.attrs.href!==evidenceSource(r,e)) add('rendered-evidence',id);
      if (e) for (const [kind,exported] of [['Markdown',markdown],['llms',llms]]) if (!exported.includes(evidenceText(e)) || !exported.includes(evidenceSource(r,e))) add('export-evidence',`${id}/${kind}`);
      seen.add(id);return;
    }
    if (Object.hasOwn(n.attrs || {},'data-recipe-evidence-grid')) {
      const grid=r.body.find(b=>b.kind==='evidence-grid');
      const want=grid?gridRows(grid,evidence):[];
      const got=[];walk(n,k=>{if(Object.hasOwn(k.attrs||{},'data-grid-row'))got.push(k);});
      if (!grid || got.length!==want.length) add('rendered-evidence','grid rows');
      for (const [i,row] of want.entries()) {
        const tr=got[i],cells=[];if(tr)walk(tr,k=>{if(k.tag==='td')cells.push(tidy(textContent(k)));});
        const a=tr&&findElement(tr,k=>k.tag==='a');
        const expect=[row.method,row.cells[0].cohort,...row.cells.map(c=>c.value),'Source'];
        if (!tr || cells.join('|')!==expect.join('|') || a?.attrs.href!==row.cells[0].source) add('rendered-evidence',`grid ${row.method}-${row.cohort}`);
        for (const [kind,exported] of [['Markdown',markdown],['llms',llms]]) if (!row.cells.every(c=>exported.includes(c.value)) || !exported.includes(row.cells[0].source)) add('export-evidence',`grid ${row.method}-${row.cohort}/${kind}`);
      }
      seen.add('grid');return;
    }
    if (Object.hasOwn(n.attrs || {},'data-recipe-artifact')) {
      const file=n.attrs['data-recipe-artifact'];
      if (!r.body.some(b=>b.kind==='artifact'&&b.file===file) || artifacts.has(file)) { add('artifact-shape',file);return; }
      const pre=findElement(n,k=>k.tag==='pre');
      const exact=fs.readFileSync(path.join(root,r.example,file),'utf8');
      if (!pre || tidy(raw(pre))!==tidy(exact) || !markdown.includes(exact.trim()) || !llms.includes(exact.trim())) add('rendered-artifact',file);
      artifacts.add(file);return;
    }
    if (n.tag==='#text' && hasDecimal(n.text)) add('unsupported numerical prose','rendered');
    for (const k of n.kids || []) inspect(k);
  }
  inspect(main);
  for (const node of r.body) {
    if (node.kind==='evidence'&&!seen.has(node.id)) add('rendered-evidence',`missing ${node.id}`);
    if (node.kind==='evidence-grid'&&!seen.has('grid')) add('rendered-evidence','missing grid');
    if (node.kind==='artifact'&&!artifacts.has(node.file)) add('rendered-artifact',`missing ${node.file}`);
  }
  return out;
}
export function sourceProblems(catalog=RECIPE_PAGES,root=ROOT) {
  const out=[];
  for (const r of catalog) out.push(...recipeProblems(r),...artifactProblems(r,root));
  return out;
}
export function distProblems(dist,catalog=RECIPE_PAGES,root=ROOT) {
  const out=[];
  const llms=fs.readFileSync(path.join(dist,'llms-full.txt'),'utf8');
  const short=fs.readFileSync(path.join(dist,'llms.txt'),'utf8');
  const sitemap=fs.readFileSync(path.join(dist,'sitemap.xml'),'utf8');
  const expected=new Set([...catalog.map(r=>r.slug),...JOBS]);
  const folder=path.join(dist,'recipes');
  if (fs.existsSync(folder)) for (const entry of fs.readdirSync(folder,{withFileTypes:true})) if (entry.isDirectory()&&!expected.has(entry.name)) {
    const page=path.join(folder,entry.name,'index.html');
    if (!fs.existsSync(page)||!isStub(fs.readFileSync(page,'utf8'))) out.push(`recipe visibility: unknown ${entry.name}`);
  }
  for (const r of catalog) {
    const htmlPath=routeFile(dist,'/recipes/'+r.slug),mdPath=path.join(dist,'recipes',r.slug+'.md');
    const url=`https://thinkthen.dev/recipes/${r.slug}/`;
    if (!fs.existsSync(htmlPath)||!fs.existsSync(mdPath)) { out.push(`recipe missing-artifact: ${r.slug}: HTML/Markdown twin`);continue; }
    const start=llms.indexOf('Source: '+url+'\n');
    const block=start<0?'':llms.slice(start,llms.indexOf('\n---\n',start)>0?llms.indexOf('\n---\n',start):undefined);
    out.push(...renderedProblems(r,fs.readFileSync(htmlPath,'utf8'),fs.readFileSync(mdPath,'utf8'),block,root));
    if (!short.includes(url.slice(0,-1)+'.md')||!sitemap.includes(url)) out.push(`recipe visibility: ${r.slug}: missing listings`);
  }
  for (const name of ['index.html','recipes/index.html']) if (fs.existsSync(path.join(dist,name))) {
    const tree=parseHtml(fs.readFileSync(path.join(dist,name),'utf8'));
    for (const tag of ['header','footer']) {
      const region=findElement(tree,n=>n.tag===tag);let links=0;
      walk(region || {kids:[]},n=>{ if(n.tag==='a'&&n.attrs.href==='/recipes/') links++; });
      if (links!==1) out.push(`recipe visibility: ${name}: ${tag} navigation`);
    }
  }
  const index=routeFile(dist,'/recipes');
  if (!fs.existsSync(index)) out.push('recipe visibility: index');
  else {
    const body=fs.readFileSync(index,'utf8');
    const main=findElement(parseHtml(body),n=>n.tag==='main');
    if (!main) out.push('recipe visibility: index');
    else if (hasDecimal(textContent(main))) out.push('recipe unsupported numerical prose: index: rendered');
    for (const slug of expected) if (!body.includes(`href="/recipes/${slug}/"`)) out.push(`recipe visibility: index membership ${slug}`);
    if (!fs.existsSync(path.join(dist,'recipes.md'))) out.push('recipe visibility: index twin');
  }
  return out;
}
if (process.argv[1] && import.meta.url===pathToFileURL(process.argv[1]).href) {
  const args=process.argv.slice(2), fixture=args.indexOf('--fixture'), di=args.indexOf('--dist');
  const catalog=fixture>=0?JSON.parse(fs.readFileSync(args[fixture+1],'utf8')):RECIPE_PAGES;
  const problems=sourceProblems(catalog,ROOT);
  if (di>=0) problems.push(...distProblems(path.resolve(args[di+1]),catalog));
  if (problems.length) {for(const p of problems) console.error(p);process.exitCode=1;}
  else console.log(`recipe check: ${catalog.length} source entries${di>=0?', rendered HTML/Markdown/llms and visibility':''}`);
}
