// Legacy details retain the command's answer fields and original metadata.
// The three entry files expose the same declared runtime names.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';

import * as esm from '../index.mjs';
import { ask, childEnv, startBackend } from './backend.mjs';

test('legacy scalar details retain the command answer and original metadata', async (t) => {
  const backend = await startBackend(t);
  for (const [spec, argv] of [
    [{ decide: 'Does it ask for a refund?' }, ['decide', 'Does it ask for a refund?']],
    [{ score: 'How urgent?', levels: ['low', 'mid', 'high'] }, ['score', 'How urgent?', 'low', 'mid', 'high']],
  ]) {
    const { value } = await ask(backend, `return tt.details(${JSON.stringify(spec)}, 'the same text');`);
    const command = spawnSync(process.env.THINKTHEN_TEST_COMMAND, [...argv, '--details'], {
      env: childEnv(backend),
      input: 'the same text',
      encoding: 'utf8',
    });
    assert.equal(command.status, 0, command.stderr);
    const document = JSON.parse(command.stdout);
    // Scalar SDK text is not a file read, so its result carries no reader
    // position. The command reads one stdin document, and records.md settles
    // its position as lines one to one of a null file.
    assert.deepEqual(document.position, { file: null, first: 1, last: 1 });
    assert.equal(Object.hasOwn(value.value, 'position'), false);
    const { schema, answer_id, position, meta, ...answer } = document;
    const { schema: legacySchema, meta: legacyMeta, ...legacyAnswer } = value.value;
    assert.equal(schema, 'thinkthen.result/2');
    assert.equal(legacySchema, 'thinkthen.result/1');
    assert.match(answer_id, /^[0-9a-f]{64}$/);
    assert.deepEqual(legacyAnswer, answer);
    // Complete native calls and the CLI carry v2 provenance; the retained
    // details entrypoint keeps its original metadata shape.
    const { attempts, origin, question_sources, observations, answered_by, ...originalMeta } = meta;
    assert.deepEqual(legacyMeta, originalMeta);
  }
});

test('index.js, index.mjs, and index.d.ts export the same declared names', () => {
  const cjs = Object.keys(createRequire(import.meta.url)('../index.js')).sort();
  const mjs = Object.keys(esm).filter((name) => name !== 'default').sort();
  const types = readFileSync(new URL('../index.d.ts', import.meta.url), 'utf8');
  const declared = [...types.matchAll(/^export (?:(?:class|function|const) |\* as )(\w+)/gm)].map((match) => match[1]);
  assert.deepEqual(mjs, cjs);
  assert.deepEqual([...new Set(declared)].sort(), cjs);
});
