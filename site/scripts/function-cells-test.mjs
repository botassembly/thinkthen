// Current docs-rule rejection controls and shared example-reader boundaries.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { expandFunctionGroups } from '../src/lib/function-variants.mjs';
import { functions, surfaces, cellId, cellRoute } from '../src/lib/function-surfaces.mjs';
import { displayOutput, selectQuestionCall, capturedStreams, inlineCode, exampleInputUrl, matchesExampleInputDownload, EXAMPLES } from '../src/lib/examples.mjs';
import { commandOptions } from '../src/lib/binary.mjs';
import { bannedWords, commandLines, proseOnly, markdownShape } from '../src/lib/docs-rules.mjs';
import { menuPages } from '../src/lib/sitemap.mjs';
import product from '../product.config.mjs';

// New docs checks reject the old words and long commands without changing
// API spellings, copied output, or a legitimate CSV row's name.
for (const language of ['go', 'java', 'kotlin', 'python', 'typescript']) {
  const id = `sdk-${language}-failure`;
  assert.ok(product.exampleLocalCaptures.includes(id));
  assert.equal(product.exampleLocalCaptureCodes[id], 'record.empty');
}
for (const id of ['sdk-pandas-failure', 'sdk-polars-failure', ...['duckdb', 'sqlite', 'postgresql'].map((db) => `sdks/${db}/failed`)]) assert.equal(product.exampleLocalCaptures.includes(id), false);
assert.equal(menuPages.length, 45);
assert.equal(new Set(menuPages.map(({ route }) => route)).size, 45);
assert.equal(bannedWords('Read the evidence field.', '/functions/decide/')[0].word, 'evidence');
assert.equal(bannedWords('Choose a backend.', '/configuration/').length, 0);
assert.equal(bannedWords('Choose a backend.', '/functions/decide/').length, 1);
assert.equal(bannedWords('The CSV row holds a message.', '/command-line/file-types/').length, 0);
assert.equal(bannedWords('The row is the answer.', '/functions/decide/').length, 1);
assert.equal(bannedWords('## Judge a CSV file\n\nEach failed row leaves its cell empty.', '/functions/decide/').length, 0);
assert.equal(bannedWords('Answer every row of a CSV file.', '/quickstart/').length, 0);
assert.equal(bannedWords('CSV files hold text; each row gets an answer.', '/command-line/file-types/').length, 0);
assert.equal(bannedWords('## CSV files\n\nRead a CSV file.\n\n## Plain text\n\nEach row is an answer.', '/functions/decide/').length, 1);
assert.equal(bannedWords('CSV files are supported.\n\nEach row is an answer.', '/functions/decide/').length, 1);
assert.equal(bannedWords('CSV files are supported.\n## Plain text\nEach row is an answer.', '/functions/decide/').length, 1);
assert.equal(bannedWords('CSV files are supported.\n  \nEach row is an answer.', '/functions/decide/').length, 1);
assert.equal(bannedWords('## CSV files\n  ### Plain text\nEach row is an answer.', '/functions/decide/').length, 1);
assert.equal(bannedWords('  ## CSV files\nEach row gets an answer.', '/functions/decide/').length, 0);
assert.equal(bannedWords('## A row is an answer\nCSV files are supported.', '/functions/decide/').length, 1);
assert.equal(bannedWords('## Every row of a CSV file\nName the column.', '/functions/decide/').length, 0);
for (const text of [
  '## CSV files\n\n##\n\nEach row is an answer.',
  'CSV files are supported.\r\n\r\nEach row is an answer.',
  '## CSV files\n\nPlain text\n----------\n\nEach row is an answer.',
  '## CSV files\n\n> ## Plain text\nEach row is an answer.',
  '## CSV files\n\n- ## Plain text\nEach row is an answer.',
]) assert.equal(bannedWords(text, '/functions/decide/').length, 1, text);
assert.equal(bannedWords('CSV files\n---------\nEach row gets an answer.', '/functions/decide/').length, 0);
assert.equal(bannedWords('## CSV files\r\n\r\nEach row gets an answer.', '/functions/decide/').length, 0);
assert.equal(bannedWords('- File types: answer every row of a CSV file.', '/quickstart/').length, 0);
assert.equal(bannedWords('## CSV files\n\n- Each row is an answer.', '/quickstart/').length, 1);
assert.equal(bannedWords('> Each row of a CSV file gets an answer.', '/quickstart/').length, 0);
assert.equal(bannedWords('## CSV files\n\n> Each row is an answer.', '/quickstart/').length, 1);
assert.equal(bannedWords(proseOnly('Call `evidence`:\n\n```sh\necho record\n```'), '/functions/decide/').length, 0);
assert.equal(bannedWords(proseOnly('| Option | Description |\n| --field | Read the evidence. |'), '/functions/decide/').length, 1);
assert.equal(commandLines('```sh\n' + 'x'.repeat(72) + '\n```').length, 0);
assert.equal(commandLines('```sh\n' + 'x'.repeat(73) + '\n```')[0].length, 73);
assert.equal(commandLines('```text\n' + 'x'.repeat(73) + '\n```').length, 0);

const codeShape = markdownShape('# C++\n\n```cpp\nstd::vector<T> xs;\n```');
assert.equal(codeShape.literals[0].text, 'std::vector<T> xs;');
assert.equal(markdownShape('Output\n\n```text\n<NA> *literal* [value](not-a-link)\n```').literals[0].text, '<NA> *literal* [value](not-a-link)');
assert.notDeepEqual(markdownShape('```text\nprint("a  b")\n```'), markdownShape('```text\nprint("a b")\n```'));
assert.notDeepEqual(markdownShape('```text\na\tb\n```'), markdownShape('```text\na b\n```'));
assert.notDeepEqual(markdownShape('```text\na\n\nb\n```'), markdownShape('```text\na\nb\n```'));
assert.equal(bannedWords(proseOnly('````text\n```\nrecord\n```\n````'), '/functions/decide/').length, 0);
assert.equal(bannedWords(proseOnly('~~~~text\n```\nrecord\n```\n~~~~'), '/functions/decide/').length, 0);

const intoHelp = { short: '', flag: '--into', value: '<INTO>', description: 'JSON Lines field or table column', env: '', default: '', values: '' };
const optionGroups = commandOptions([
  { name: 'decide', options: [{ ...intoHelp, flag: '--text' }, intoHelp] },
  { name: 'choose', options: [{ ...intoHelp }] },
  { name: 'run', options: [{ ...intoHelp, description: 'Table column' }] },
]);
assert.deepEqual(optionGroups.map((option) => [option.flag, option.description, option.commands]), [
  ['--into', intoHelp.description, ['decide', 'choose']],
  ['--into', 'Table column', ['run']],
  ['--text', intoHelp.description, ['decide']],
]);
const preciseJson = String.raw`{"id":900719925474099312345,"p":1.2300e-09,"text":"quoted \"words\" stay together"}`;
assert.equal(displayOutput(preciseJson).text, preciseJson);
assert.match(displayOutput(preciseJson).text, /1\.2300e-09/);
assert.equal(JSON.parse(displayOutput(preciseJson).text).text, JSON.parse(preciseJson).text);
assert.deepEqual(JSON.parse(displayOutput('{"a":[{},[],null,true]}').text), { a: [{}, [], null, true] });
assert.equal(displayOutput('{"a":true}\n{"a":false}').text, '{"a":true}\n{"a":false}');
assert.equal(displayOutput('diagnostic '.repeat(20)).wrapped, false);
assert.equal(displayOutput('diagnostic '.repeat(20)).text, 'diagnostic '.repeat(20));
const selected = selectQuestionCall('import sdk\nconst isRefund = ask();\nprint(isRefund);', { anchor: 'const isRefund = ask();', before: 0, after: 1 });
assert.equal(selected.text, 'const isRefund = ask();\nprint(isRefund);');
assert.equal(selected.partial, true);
const tabbed = selectQuestionCall('setup();\n\task();', { anchor: '\task();', before: 1, after: 0 });
assert.equal(tabbed.line, 1);
assert.equal(tabbed.anchor, '\task();');
assert.equal(tabbed.renderedAnchor, '  ask();');
assert.throws(() => selectQuestionCall('setup();\n\tchangedAsk();', { anchor: '\task();', before: 1, after: 0 }), /exactly once/);

assert.throws(() => selectQuestionCall('ask();', { anchor: 'oldAsk();', before: 0, after: 0 }), /exactly once/);
assert.throws(() => selectQuestionCall('ask();\nask();', { anchor: 'ask();', before: 0, after: 0 }), /exactly once/);
assert.throws(() => selectQuestionCall('ask();', { anchor: 'ask();', before: 1, after: 0 }), /leaves the source/);
assert.throws(() => selectQuestionCall('ask();', { anchor: 'ask();', before: 0, after: 99 }), /0–12/);
// Error-reference verification must accept diagnostics in either real stream.
const witness = (output, stderr, code) => capturedStreams({ output, stderr }).join('\n').includes(code);
assert.equal(witness('', 'thinkthen: usage.threshold: invalid threshold', 'usage.threshold'), true);
assert.equal(witness('thinkthen: record.empty: empty record', '', 'record.empty'), true);
assert.equal(witness('ordinary output', 'another diagnostic', 'usage.threshold'), false);
assert.equal(inlineCode('ordinary diagnostic'), '`ordinary diagnostic`');
assert.equal(inlineCode('expected `systemone`'), '`` expected `systemone` ``');
assert.equal(inlineCode('a ``nested`` value'), '``` a ``nested`` value ```');
const checker = path.resolve('scripts/check-function-cells.mjs');
const scratch = fs.mkdtempSync(path.join(os.tmpdir(), 'function-cell-check-'));
const write = (file, text) => { fs.mkdirSync(path.dirname(file), { recursive: true }); fs.writeFileSync(file, text); };
const run = (args = []) => spawnSync(process.execPath, [checker, ...args], { cwd: scratch, encoding: 'utf8' });
try {
  const download = exampleInputUrl('error-reply', 'server.py');
  const published = path.join(scratch, 'dist', download);
  const linked = new Set([download]);
  assert.equal(matchesExampleInputDownload('error-reply', 'server.py', linked, path.join(scratch, 'dist')), false, 'missing download');
  write(published, fs.readFileSync(path.join(EXAMPLES, 'error-reply', 'server.py')));
  assert.equal(matchesExampleInputDownload('error-reply', 'server.py', linked, path.join(scratch, 'dist')), true, 'exact linked input');
  assert.equal(matchesExampleInputDownload('error-reply', 'server.py', new Set(), path.join(scratch, 'dist')), false, 'unlinked input');
  assert.equal(matchesExampleInputDownload('error-status', 'server.py', linked, path.join(scratch, 'dist')), false, 'another example input');
  write(published, 'different bytes');
  assert.equal(matchesExampleInputDownload('error-reply', 'server.py', linked, path.join(scratch, 'dist')), false, 'changed input');
  assert.throws(() => exampleInputUrl('../error-reply', 'server.py'), /canonical/);
  assert.throws(() => exampleInputUrl('error-reply', '../server.py'), /canonical/);
  for (const fn of functions) write(path.join(scratch, 'examples', 'functions', fn, 'case.json'), '{}\n');
  for (const fn of functions) for (const {id} of surfaces) {
    const cell = cellId(fn, id);
    write(path.join(scratch, 'examples', cell, 'show.sh'), 'is_refund=true\n');
    write(path.join(scratch, 'examples', cell, 'call.json'), JSON.stringify({ anchor: 'is_refund=true', before: 0, after: 0 }));
    write(path.join(scratch, 'examples', cell, 'run.sh'), '#!/bin/sh\n');
    write(path.join(scratch, '.example-runs', cell, 'answers.json'), '[true,false,null]\n');
  }
  assert.equal(run(['--fresh']).status, 0, 'complete fresh matrix');
  write(path.join(scratch, 'examples', cellId('decide', 'python'), 'unverified'), 'Real output has not been captured.\n');
  assert.equal(run(['--fresh']).status, 0, 'uncaptured output does not skip routine agreement');
  const previous = process.cwd();
  process.chdir(scratch);
  try {
    const expanded = expandFunctionGroups('<LanguageGroup id="first-call"><LanguageVariant surface="command"><Example id="functions/decide/command" wrap /></LanguageVariant></LanguageGroup>');
    assert.equal([...expanded.matchAll(/<LanguageVariant /g)].length, surfaces.length);
    assert.equal(expandFunctionGroups(expanded), expanded, 'expansion does not duplicate authored variants');
    assert.match(expanded, /functions\/decide\/python/);
  } finally { process.chdir(previous); }
  const removed = path.join(scratch, 'examples', cellId('decide', 'python'), 'run.sh');
  const waiting = path.join(scratch, 'examples', cellId('decide', 'python'), 'unverified');
  fs.unlinkSync(waiting);
  fs.unlinkSync(removed);
  assert.match(run().stderr, /missing run.sh/);
  write(removed, '#!/bin/sh\n');
  write(waiting, 'Real output has not been captured.\n');
  const answer = path.join(scratch, '.example-runs', cellId('decide', 'python'), 'answers.json');
  write(answer, '[false,false,null]\n');
  assert.match(run(['--fresh']).stderr, /answers differ/);
  write(answer, '[true,false,null]\n');
  const shown = path.join(scratch, 'examples', cellId('decide', 'python'), 'show.sh');
  for (const declaration of ['row = client.decide()', 'const result = client.decide()', 'let value = client.decide()', 'for row in client.decide_each():', 'for (Row<Boolean> row : client.decideEach()) {', 'for row := range answers {', 'for (answers) |row| {', 'SELECT answer AS result FROM messages']) {
    write(shown, declaration);
    assert.match(run().stderr, /name the answer for its meaning/, declaration);
  }
  fs.unlinkSync(shown);
  write(path.join(scratch, 'examples', cellId('decide', 'python'), 'show.sql'), 'SELECT answer FROM messages');
  assert.match(run().stderr, /alias the answer column/);
  console.log('Function matrix controls pass: missing runnable cell, changed fresh answer, generic names and SQL aliases are rejected');
} finally { fs.rmSync(scratch, { recursive: true, force: true }); }
