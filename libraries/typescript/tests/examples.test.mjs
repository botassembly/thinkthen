// Every example in examples.json answers as the file says. Each runs with
// `tt` bound to an engine on the example's backend arm.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

import { ask, startBackend } from './backend.mjs';

const FILE = new URL('../examples.json', import.meta.url);

test('every function example answers as the file says', async (t) => {
  const backend = await startBackend(t);
  const { examples } = JSON.parse(readFileSync(FILE, 'utf8'));
  const { value } = await ask(backend, `
    const out = {};
    for (const [name, { arm, ts }] of Object.entries(${JSON.stringify(examples)})) {
      const engine = new tt.Client({ base_url: ${JSON.stringify(`http://127.0.0.1:${backend.port}/`)} + arm + '/v1', cache: false });
      try {out[name] = JSON.stringify(await new Function('tt', 'return ' + ts)(engine));}
      finally {engine.close();}
    }
    return out;`);
  assert.equal(Object.keys(examples).length, 10);
  for (const [name, { expected }] of Object.entries(examples)) assert.equal(value[name], expected, name);
});
