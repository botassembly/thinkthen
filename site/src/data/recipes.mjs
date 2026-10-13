// The closed public content contract of the measured pipeline recipes.
import { parseHtml, textContent } from '../lib/html.mjs';
export const hasDecimal = text => [...text].some(c => /\p{Decimal_Number}/u.test(c));
export const decodedText = text => textContent(parseHtml(text));
const recordSource = url => /^https:\/\/github\.com\/botassembly\/thinkthen\/blob\/(?:main|[a-f0-9]{40})\/sdlc\/records\/[a-z0-9-]+\.md$/.test(url);
export const unsafePublic = text => /\/(?:home|Users)\/|file:\/\/|thinkthen-exp|(?:workspace|repos)\//.test(text);
const FUNCTION_NAMES = ['decide','choose','tag','score','filter','rank','find','annotate','recognize','relate'];

// An evidence grid shows one row per method and cohort. Its cells read the
// measured entries `${method}-${cohort}-tp`, `-fp` and `-fn`.
export const GRID_COLUMNS = [['tp', 'Right'], ['fp', 'Wrong yes'], ['fn', 'Missed yes']];
export const gridRows = (node, evidence) => node.cohorts.flatMap((cohort) => node.methods.map((method) => ({
  method, cohort,
  cells: GRID_COLUMNS.map(([key]) => evidence.get(`${method}-${cohort}-${key}`)),
})));

export function recipeProblems(r) {
  const problems = [], add = (code, where) => problems.push(`recipe ${code}: ${r.slug}: ${where}`);
  for (const key of ['title','gain']) {
    if (typeof r[key] !== 'string' || !r[key]) add('metadata', key);
    else if (hasDecimal(decodedText(r[key]))) add('unsupported numerical prose', key);
  }
  if (!/^[a-z][a-z0-9-]+$/.test(r.slug || '') || r.example !== `site/examples/recipes/${r.slug}`) add('metadata','slug/example');
  if (!/^sdlc\/records\/[a-z0-9-]+\.md$/.test(r.sourceRecord || '')) add('source', 'source record');
  if (!Array.isArray(r.functions) || !r.functions.length || r.functions.some(f => !FUNCTION_NAMES.includes(f))) add('metadata', 'functions');
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
    if (measured && !recordSource(e.source || '')) add('source', e.id);
    if (!measured && (!/^files\/[a-z0-9.-]+$/.test(e.artifact || '') || !/^\/[a-z_]+$/.test(e.selector || '') || !/Controlled/.test(e.qualification || ''))) add('carrier-shape', e.id);
  }
  if (!Array.isArray(r.body)) add('carrier-shape','body');
  for (const [i,node] of (r.body || []).entries()) {
    const where = `body[${i}]`;
    if (node.kind === 'evidence-grid') {
      const lists = ['methods','cohorts'].every(k => Array.isArray(node[k]) && node[k].length && node[k].every(v => /^[a-z]+$/.test(v)));
      if (!lists || Object.keys(node).some(k => !['kind','methods','cohorts'].includes(k))) { add('carrier-shape',where); continue; }
      for (const row of gridRows(node, evidence)) if (row.cells.some(c => !c)) add('carrier-shape',`${where}/${row.method}-${row.cohort}`);
      continue;
    }
    const fields = { text:['kind','text'], heading:['kind','text'], link:['kind','text','href'], evidence:['kind','id'], artifact:['kind','file'] }[node.kind];
    if (!fields || Object.keys(node).some(k => !fields.includes(k)) || fields.some(k => typeof node[k] !== 'string')) { add('carrier-shape',where); continue; }
    if (['text','heading','link'].includes(node.kind)) {
      if (hasDecimal(decodedText(node.text))) add('unsupported numerical prose',where);
      if (/[<>]/.test(node.text)) add('carrier-shape',where);
    }
    if (node.kind === 'link' && !/^\/(?:functions|trust|start|models|databases|languages)\/[a-z0-9/-]*(?:#[a-z-]+)?$/.test(node.href)) add('carrier-shape',where);
    if (node.kind === 'evidence' && !evidence.has(node.id)) add('carrier-shape',where);
    if (node.kind === 'artifact' && !/^(?:files\/[a-z0-9.-]+|[12]-[a-z]+\.(?:sh|out))$/.test(node.file)) add('carrier-shape',where);
  }
  if (unsafePublic(JSON.stringify(r))) add('public-provenance','private locator');
  return problems;
}
export function evidenceText(e) {
  return e.evidenceClass === 'measured'
    ? `${e.value}; denominator ${e.denominator}; ${e.definition}; ${e.cohort}; fields with answers ${e.resolved}, fields without answers ${e.unresolved}; backend ${e.backend}, model ${e.model}; ${e.qualification}`
    : `${e.value}; denominator ${e.denominator}; ${e.scope}; ${e.qualification}`;
}
export function evidenceSource(r,e) {
  return e.evidenceClass === 'measured' ? e.source : `https://github.com/botassembly/thinkthen/blob/main/${r.example}/${e.artifact}`;
}
export const EVIDENCE_LABEL = { measured: 'Source', fixture: 'Controlled fixture' };
