// Check the built documentation against the shipped command and function contract.
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import {spawnSync} from 'node:child_process';
import product from '../product.config.mjs';
import site from '../src/data/site.json' with {type:'json'};
import redirects from '../src/data/redirects.json' with {type:'json'};
import { readExample } from '../src/lib/examples.mjs';
const arg=(name,fallback)=>process.argv[process.argv.indexOf(name)+1] ?? fallback;
const dist=path.resolve(process.argv.includes('--dist')?arg('--dist'):'dist');
const binary=path.resolve(process.argv.includes('--binary')?arg('--binary'):path.join(product.binDir,product.binary));
const names=JSON.parse(fs.readFileSync('../schemas/contract.json','utf8')).functions.map(f=>f.name);
const decode=s=>s.replace(/<\/(?:th|td|tr|p|div|h[1-6])>/g,' ').replace(/<[^>]*>/g,'').replace(/&(?:amp|lt|gt|quot|apos|#\d+|#x[\da-f]+);/gi,e=>({'&amp;':'&','&lt;':'<','&gt;':'>','&quot;':'"','&apos;':"'"}[e]??String.fromCodePoint(e[2].toLowerCase()==='x'?parseInt(e.slice(3,-1),16):Number(e.slice(2,-1)))));
const plain=s=>decode(s).replace(/\s+/g,' ').trim();
const failures=[];
const check=(name,fn)=>{try{const errors=fn();if(errors.length){failures.push(name);console.error(`FAIL ${name}: ${errors.length} gap(s)\n${errors.slice(0,8).join('\n')}`);}else console.log(`PASS ${name}`);}catch(e){failures.push(name);console.error(`FAIL ${name}: ${e.message}`)}};
const walk=dir=>fs.readdirSync(dir,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(path.join(dir,e.name)):[path.join(dir,e.name)]);
check('Markdown alternate metadata',()=>{
 const errors=[];
 for(const file of walk(dist).filter(f=>f.endsWith('.html'))){
  const html=fs.readFileSync(file,'utf8');
  const links=[...(html.match(/<head\b[\s\S]*?<\/head>/i)?.[0]??html.split(/<body\b/i)[0]).matchAll(/<link\b[^>]*>/gi)].map(m=>m[0]).filter(t=>/rel="alternate"/.test(t)&&/type="text\/markdown"/.test(t));
  const href=links[0]?.match(/href="([^"]+)"/)?.[1];
  const expected=html.match(/data-copy-markdown="([^"]+)"/)?.[1] ?? (path.relative(dist,file).replace(/(?:\/)?index\.html$/,'').replace(/\.html$/,'')+'.md');
  const twin=href&&new URL(href,'https://docs.invalid').pathname;
  if(links.length!==1 || !twin || twin!==('/'+expected).replace(/^\/\//,'/') || !fs.existsSync(path.join(dist,twin)))errors.push(`${path.relative(dist,file)}: missing/incorrect alternate to existing ${expected}`);
 }
 return errors;
});
check('Every language lists all ten functions',()=>{
 const errors=[];
 for(const [surface,language] of Object.entries(site.languages)){
  const route=redirects[language.page]??language.page;
  let file=path.join(dist,route,'index.html');let html=fs.readFileSync(file,'utf8');
  if(fs.existsSync(path.join(dist,language.page,'functions/index.html'))){file=path.join(dist,language.page,'functions/index.html');html=fs.readFileSync(file,'utf8');}
  const tables=[...html.matchAll(/<table\b[\s\S]*?<\/table>/g)].map(m=>m[0]).filter(t=>/^Function\b/.test(plain(t.match(/<thead[\s\S]*?<\/thead>/)?.[0]??'')));
  const rows=tables.flatMap(t=>[...t.matchAll(/<tr\b[^>]*>[\s\S]*?<\/tr>/g)].slice(1).map(m=>plain(m[0].match(/<td\b[^>]*>([\s\S]*?)<\/td>/)?.[1]??'')));
  const code=[...html.matchAll(/<pre\b[\s\S]*?<\/pre>/g)].map(([block])=>plain(block)).join('\n');
  const cells=new Set([...html.matchAll(/data-example="functions\/([^/]+)\/([^"]+)"/g)].filter(([, , id])=>id===surface).map(([, name])=>name));
  const missing=names.filter(n=>tables.length
   ? !new RegExp(`(?:^|[\\W_])${n}(?=$|[\\W_])`).test(rows.join(' '))
   : !cells.has(n)&&!new RegExp(`(?:\\btt_|\\bclient\\.)${n}(?:_each)?\\s*\\(`).test(code));
  if(missing.length)errors.push(`${language.name}: missing ${missing.join(', ')}`);
  if(['/languages/cpp/','/languages/rust/'].includes(route))for(const name of names){
   const row=tables.flatMap(t=>[...t.matchAll(/<tr\b[^>]*>[\s\S]*?<\/tr>/g)].map(m=>m[0])).find(r=>plain(r.match(/<td\b[^>]*>([\s\S]*?)<\/td>/)?.[1]??'')===name);
   if(!row?.includes(`href="/functions/${name}/"`))errors.push(`${language.name}: ${name} lacks its function reference link`);
  }
 }
 return errors;
});
const sandbox=fs.mkdtempSync(path.join(os.tmpdir(),'site-text-proof-'));
try {
 const env={PATH:process.env.PATH,HOME:sandbox,XDG_CONFIG_HOME:path.join(sandbox,'config'),XDG_CACHE_HOME:path.join(sandbox,'cache'),LANG:'C.UTF-8',NO_COLOR:'1'};
 const run=(args,input)=>{const r=spawnSync(binary,args,{input,env,encoding:'utf8'});if(r.status!==0)throw Error(`command failed (${r.status}): ${r.error?.message ?? r.stderr}`);return r.stdout;};
 check('Grouped text examples and descriptions match the command',()=>{
  const errors=[];
  for(const [name,args] of [
   ['tag',['--label','billing','--label','shipping','Which topics appear?']],
   ['recognize',['--kind','person','--kind','organization']],
   ['relate',['--rule','works_for','person','organization']],
  ]){
   const base=[name,'--backend','stub',...args];
   const pool=Array.from({length:96},(_,i)=>i<48?`Message ${i}.`:`Morgan ${i} works for Example Group and requests a refund.`);
   const rows=run([...base,'--from','lines','--json'],pool.join('\n')).trim().split('\n').map(JSON.parse);
   const yes=rows.findIndex(r=>r.answer?.length>0), no=rows.findIndex(r=>r.answer?.length===0);
   if(yes<0||no<0)throw Error(`${name}: fixture pool must contain nonempty and empty results`);
   const inputs=[pool[yes],pool[no],pool[yes]];
   const expected=inputs.map((input,i)=>{
    const one=run([...base,'--from','text'],input);if(!/^[^\n]*\n$/.test(one))throw Error(`${name}: individual text result is not one line: ${JSON.stringify(one)}`);
    return `${i+1}\t${one}`;
   }).join('');
   const actual=run([...base,'--from','lines'],inputs.join('\n'));
   if(actual!==expected || actual.split('\n')[1]!=='2\t')errors.push(`${name}: grouped lines lost an empty record boundary`);
   console.log(`${name} actual nonempty/empty/nonempty: ${JSON.stringify(actual)}`);
   const html=fs.readFileSync(path.join(dist,'functions',name,'index.html'),'utf8');
   const text=plain(html);
   if(!/one line (?:per|for each) record|each record[^.]*one line/i.test(text)||!/tab-separated|separated by tabs/i.test(text)||text.includes('[position]'))errors.push(`${name}: description does not teach actual tab-separated record grouping`);
   const id=`functions/${name}/command`;
   const example=readExample(id);
   const block=[...html.matchAll(/<div\b[^>]*>/g)].find(([tag])=>tag.includes(`data-example="${id}"`))?.[0];
   if(!fs.existsSync(path.resolve('examples',id,'run.sh'))||!block)errors.push(`${name}: built page lacks runnable command matrix source ${id}`);
   if(example.unverified){
    if(!block?.includes('data-unverified'))errors.push(`${name}: uncaptured matrix example must withhold public output`);
   }else if(!example.output||block?.includes('data-unverified')||!text.includes(plain(example.output)))errors.push(`${name}: built page lacks the actual captured matrix output ${id}`);
  }
  const data=plain(fs.readFileSync(path.join(dist,'your-data/index.html'),'utf8'));
  if(!/one line per record/i.test(data)||!/tab-separated|separated by tabs/i.test(data)||data.includes('[position]'))errors.push('your-data: grouped text description must match unmarked tab-separated output');
  return errors;
 });
} finally {fs.rmSync(sandbox,{recursive:true,force:true});}
if(failures.length)process.exit(1);
