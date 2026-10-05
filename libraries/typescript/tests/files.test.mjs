import { test } from 'node:test';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { ask, startBackend } from './backend.mjs';

const documents = fileURLToPath(new URL('../../../specification/fixtures/files/documents', import.meta.url));

test('explicit reader reaches every verb and keeps original source coordinates', async (t) => {
  const backend = await startBackend(t);
  const { value, error } = await ask(backend, `
    const engine = new tt.Engine({cache:false});
    const source = ${JSON.stringify(documents)};
    const questions = [
      {decide:'Q?'}, {choose:'Q?',options:['policy','contract']},
      {tag:'Q?',labels:['refund','support']}, {score:'Q?',levels:['low','high']},
      {filter:'Q?'}, {rank:'Q?'}, {find:'Q?'},
      {annotate:{version:1,questions:{urgent:{decide:'Q?'}}}},
      {version:1,recognize:{kinds:{person:null}}},
      {version:1,relate:{relations:[{name:'supports',source:'*',target:'*'}]}}
    ];
    return Promise.all(questions.map(q => engine.files(q, source, {unit:'file'})));
  `, { arm: 'arm/full' });
  assert.equal(error, undefined);
  assert.equal(value.length, 10);
  for (const [at, call] of value.entries()) {
    assert.ok(call.facts.requests_sent > 0);
    const rows = at === 9 ? call.value.edges.flatMap(edge => [edge.source, edge.target]) :
      at === 6 ? [call.value] : call.value;
    assert.ok(rows.length > 0);
    for (const row of rows) {
      assert.equal(row.first_line, 1); assert.equal(row.last_line, 4);
      assert.ok(row.record.includes('\n')); assert.ok(row.file.endsWith('.txt'));
      assert.ok(Object.hasOwn(row, 'value'));
    }
  }
});

test('invalid source options and evidence mixtures send no requests', async (t) => {
  const backend = await startBackend(t);
  for (const [question, reader] of [[{decide:'Q?'},{unit:'file',window:2}],
      [{decide:'Q?',evidence:'old'},{unit:'file'}]]) {
    const reply = await ask(backend, `return tt.files(${JSON.stringify(question)}, ${JSON.stringify(documents)}, ${JSON.stringify(reader)});`);
    assert.equal(reply.error.kind, 'usage');
  }
  assert.equal(await backend.count(), 0);
});
