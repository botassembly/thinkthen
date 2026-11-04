// Check the actual rendered table consumer, not copies of CLI help strings.
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
if (failed) process.exitCode = 1;
