// Check the actual rendered table consumer, not copies of CLI help strings.
import fs from 'node:fs';
import path from 'node:path';
import { functions } from '../src/lib/function-surfaces.mjs';
import { functionOptions, functionOptionsMarkdown, sharedOptions, sharedOptionsMarkdown } from '../src/lib/function-options.mjs';
import { bannedWords, proseOnly } from '../src/lib/docs-rules.mjs';
import { binaryInfo } from '../src/lib/binary.mjs';

const info = binaryInfo();
console.log(`Help tables: ${info.line}, build ${info.commit ?? 'not reported'}`);
let failed = false;
for (const name of [...functions, 'shared']) {
  const rows = name === 'shared' ? sharedOptions() : functionOptions(name);
  const markdown = name === 'shared' ? sharedOptionsMarkdown() : functionOptionsMarkdown(name);
  const route = name === 'shared' ? '/reference/options/' : `/functions/${name}/`;
  const findings = bannedWords(proseOnly(markdown), route);
  const valid = rows.length > 0 && findings.length === 0;
  console.log(`${valid ? 'PASS' : 'FAIL'} ${name}: ${rows.length} options/arguments, ${findings.length} banned words`);
  if (!valid) {
    failed = true;
    console.error(JSON.stringify({ name, rows, findings }, null, 2));
  }
}
// These explicit consumer expectations catch missing command arguments/flags,
// independently of how the help-row helper builds its arrays.
const required = {
  audit: ['<ROWS>', '--manifest', '--key', '--bins', '--help'],
  diff: ['<LEFT>', '--manifest', '--left-manifest', '--right', '--right-manifest', '--question-map', '--threshold', '--low', '--high', '--help'],
  recover: ['<FILE>', '--help'],
};
const rendered = process.argv.includes('--rendered');
const html = rendered ? fs.readFileSync('dist/reference/options/index.html', 'utf8') : '';
const twinName = rendered ? html.match(/data-copy-markdown="([^"]+)"/)?.[1] : null;
if (rendered && !twinName) throw new Error('Options page has no Markdown twin');
const twin = rendered ? fs.readFileSync(path.join('dist', twinName.replace(/^\//, '')), 'utf8') : '';
const codeText = (text) => text.replaceAll('&lt;', '<').replaceAll('&gt;', '>').replaceAll('&amp;', '&');
for (const [name, tokens] of Object.entries(required)) {
  const markdown = functionOptionsMarkdown(name, 'command');
  const table = rendered ? html.match(new RegExp(`<table[^>]*data-command-options="${name}"[^>]*>([\\s\\S]*?)</table>`))?.[1] : '';
  const codes = table ? [...table.matchAll(/<code[^>]*>(.*?)<\/code>/g)].map((match) => codeText(match[1])) : [];
  const section = twin.split(`\n### ${name}\n`)[1]?.split(/\n##?# /)[0] ?? '';
  const findings = bannedWords(proseOnly(markdown), '/reference/options/');
  const valid = findings.length === 0 && tokens.every((token) => markdown.includes(token)
    && (!rendered || (codes.some((code) => code.includes(token)) && section.includes(token))))
    && (name !== 'audit' || (markdown.includes('Default: 10') && (!rendered || table?.includes('Default: 10'))));
  console.log(`${valid ? 'PASS' : 'FAIL'} ${name}: complete current help${rendered ? ', HTML and Markdown' : ''}`);
  if (!valid) { failed = true; console.error(JSON.stringify({ name, tokens, findings, tableFound: !!table })); }
}
if (failed) process.exitCode = 1;
