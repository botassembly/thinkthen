// The result shapes the engine writes: `details` equals the command's
// `--details` document, and the three entry files export the same names.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';

import * as esm from '../index.mjs';
import { ask, childEnv, startBackend } from './backend.mjs';

test('details equals the command --details document for the same question and text', async (t) => {
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
    assert.deepEqual(value.value, JSON.parse(command.stdout));
  }
});

test('index.js, index.mjs, and index.d.ts export the same twenty names', () => {
  const cjs = Object.keys(createRequire(import.meta.url)('../index.js')).sort();
  const mjs = Object.keys(esm).filter((name) => name !== 'default').sort();
  const types = readFileSync(new URL('../index.d.ts', import.meta.url), 'utf8');
  const declared = [...types.matchAll(/^export (?:class|function|const) (\w+)/gm)].map((match) => match[1]);
  assert.equal(cjs.length, 20);
  assert.deepEqual(mjs, cjs);
  assert.deepEqual([...new Set(declared)].sort(), cjs);
});
