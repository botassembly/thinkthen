// Environment-neutral recipe selection and closed public content contract.
import { parseHtml, textContent } from '../lib/html.mjs';
export const RECIPE_SCOPE = Object.freeze({
  'rules-propose-model-confirms': '2026-10-03-draft-function-extract.md',
  'link-records': '2026-10-03-draft-function-link.md',
  'verify-a-claim': '2026-10-03-draft-function-verify.md',
  'navigate-many-documents': '2026-10-03-draft-function-navigate.md',
  'search-transcripts': '2026-10-05-recipe-search-transcripts.md',
  'ask-your-cache-with-duckdb': '2026-10-05-recipe-ask-your-cache-with-duckdb.md',
});
export function recipeSelection(catalog, preview = false) {
  const published = catalog.filter(r => !r.draft);
  return { index: published.length > 0 || preview, navigation: published.length > 0,
    pages: catalog.filter(r => !r.draft || preview), draftIndex: !published.length };
}
export const hasDecimal = text => [...text].some(c => /\p{Decimal_Number}/u.test(c));
export const decodedText = text => textContent(parseHtml(text));
const pinned = url => /^https:\/\/github\.com\/botassembly\/thinkthen\/blob\/[a-f0-9]{40}\/sdlc\/records\/[a-z0-9-]+\.md$/.test(url);
export const unsafePublic = text => /\/(?:home|Users)\/|file:\/\/|thinkthen-exp|(?:workspace|repos)\//.test(text);
export function recipeProblems(r) {
  const problems = [], add = (code, where) => problems.push(`recipe ${code}: ${r.slug}: ${where}`);
  for (const key of ['title','goal','job','wait']) {
    if (typeof r[key] !== 'string' || !r[key]) add('metadata', key);
    else if (hasDecimal(decodedText(r[key]))) add('unsupported numerical prose', key);
  }
  if (!/^[a-z][a-z0-9-]+$/.test(r.slug || '') || r.example !== `site/examples/recipes/${r.slug}`) add('metadata','slug/example');
  if (typeof r.draft !== 'boolean' || !['waiting','ready'].includes(r.publication)) add('metadata', 'disposition');
  if (!r.draft && r.publication !== 'ready') add('publication-evidence', 'not ready');
  if (!/^[a-f0-9]{40}$/.test(r.sourceCommit || '') || !/^sdlc\/records\/[a-z0-9-]+\.md$/.test(r.sourceRecord || '')) add('source-pin', 'owning record');
  if (!r.issue || r.owner !== 'Queue owner' || !Array.isArray(r.functions) || !r.functions.length || r.functions.some(f => !['decide','choose','tag','score','filter','rank','find','annotate','recognize','relate'].includes(f))) add('metadata', 'owner/functions');
  const evidence = new Map();
  for (const e of [...(r.measured || []), ...(r.fixtureAssertions || [])]) {
    if (!e.id || evidence.has(e.id)) add('carrier-shape', 'evidence identity');
    evidence.set(e.id, e);
    const measured = (r.measured || []).includes(e);
    if (measured && (Object.hasOwn(e,'artifact') || Object.hasOwn(e,'selector') || /controlled.loopback/i.test(e.qualification || ''))) add('evidence-class',e.id);
    if (e.evidenceClass !== (measured ? 'measured' : 'fixture')) add('evidence-class', e.id);
    for (const field of measured ? ['value','denominator','cohort','definition','resolved','unresolved','backend','model','qualification','source'] : ['value','denominator','scope','qualification','artifact','selector']) {
      if (typeof e[field] !== 'string' || !e[field]) add('measurement-definition', `${e.id}/${field}`);
    }
    if (measured && !pinned(e.source || '')) add('source-pin', e.id);
    if (!measured && (!/^files\/[a-z0-9.-]+$/.test(e.artifact || '') || !/^\/[a-z_]+$/.test(e.selector || '') || !/Controlled/.test(e.qualification || ''))) add('carrier-shape', e.id);
  }
  if (!Array.isArray(r.body)) add('carrier-shape','body');
  for (const [i,node] of (r.body || []).entries()) {
    const where = `body[${i}]`;
    const fields = { text:['kind','text'], heading:['kind','text'], link:['kind','text','href'], evidence:['kind','id'], artifact:['kind','file'] }[node.kind];
    if (!fields || Object.keys(node).some(k => !fields.includes(k)) || fields.some(k => typeof node[k] !== 'string')) { add('carrier-shape',where); continue; }
    if (['text','heading','link'].includes(node.kind)) {
      if (hasDecimal(decodedText(node.text))) add('unsupported numerical prose',where);
      if (/[<>]/.test(node.text)) add('carrier-shape',where);
    }
    if (node.kind === 'link' && !/^\/(?:functions|learn|trust)\/[a-z0-9/-]*(?:#[a-z-]+)?$/.test(node.href)) add('carrier-shape',where);
    if (node.kind === 'evidence' && !evidence.has(node.id)) add('carrier-shape',where);
    if (node.kind === 'artifact' && !/^(?:files\/[a-z0-9.-]+|[12]-[a-z]+\.(?:sh|out))$/.test(node.file)) add('carrier-shape',where);
  }
  if (unsafePublic(JSON.stringify(r))) add('public-provenance','private locator');
  return problems;
}
export function scopeProblems(catalog) {
  const out = [], expected = Object.keys(RECIPE_SCOPE);
  if (catalog.some(r => r.slug === 'qualify')) out.push('recipe disposition: verify/qualify must remain shared');
  if (catalog.length !== expected.length || new Set(catalog.map(r => r.slug)).size !== expected.length || expected.some(slug => !catalog.some(r => r.slug === slug && r.issue === 'sdlc/issues/'+RECIPE_SCOPE[slug]))) out.push('recipe scope: required slug/owning issue');
  return out;
}
export function evidenceText(e) {
  return e.evidenceClass === 'measured'
    ? `${e.value}; denominator ${e.denominator}; ${e.definition}; ${e.cohort}; fields with answers ${e.resolved}, fields without answers ${e.unresolved}; backend ${e.backend}, model ${e.model}; ${e.qualification}`
    : `${e.value}; denominator ${e.denominator}; ${e.scope}; ${e.qualification}`;
}
export function evidenceSource(r,e) {
  return e.evidenceClass === 'measured' ? e.source : `https://github.com/botassembly/thinkthen/blob/${r.sourceCommit}/${r.example}/${e.artifact}`;
}
