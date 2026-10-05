// Transform only declared generated aliases; all consumers still see stubs.
import fs from 'node:fs';
import path from 'node:path';
import { builtPages } from '../src/lib/listed-pages.mjs';
import { ALIASES, routeFile, preserver, validateStub } from './redirect-contract.mjs';
const dist = path.join(process.cwd(), 'dist');
try {
  for (const p of builtPages(dist).filter(p => p.kind === 'stub')) {
    if (!Object.hasOwn(ALIASES, p.route.replace(/\/$/, ''))) throw new Error(`undeclared redirect stub: ${p.route}`);
  }
  // Validate the whole input before changing any generated file.
  const edits = Object.entries(ALIASES).map(([alias, fixed]) => {
    const file = routeFile(dist, alias);
    if (!fs.existsSync(file)) throw new Error(`alias compatibility: ${alias}: missing redirect stub`);
    const html = fs.readFileSync(file, 'utf8');
    const transformed = html.includes('<script>');
    const refresh = validateStub(html, alias, fixed, dist, transformed);
    return { file, html: transformed ? html : html.replace(refresh, preserver(fixed) + `<noscript>${refresh}</noscript>`) };
  });
  for (const { file, html } of edits) fs.writeFileSync(file, html);
  console.log(`fragment preservation: ${edits.length} direct aliases`);
} catch (e) { console.error(e.message); process.exitCode = 1; }
