// The engine settings of ADR 0017 section 5, each in its own child. An engine
// starts from the environment, and each given option overrides one setting.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdirSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

import { ask, child, sleep, startBackend } from './backend.mjs';

// Answer files, beside the folder's one backend marker.
const files = (folder) =>
  readdirSync(folder, { recursive: true, withFileTypes: true }).filter((entry) => entry.isFile() && entry.name.endsWith('.json') && !entry.name.startsWith('.'));

test('an engine starts from the environment: THINKTHEN_CACHE holds its answers', async (t) => {
  const backend = await startBackend(t);
  const folderA = join(backend.folder, 'a');
  const scratch = join(backend.folder, 'scratch');
  mkdirSync(folderA);
  mkdirSync(scratch);
  const { value } = await ask(backend, `
    const engine = new tt.Engine({ throttle: 4, baseUrl: ${JSON.stringify(backend.base())} });
    return [(await engine.decide('Refund?', 'same')).value, (await engine.decide('Refund?', 'same')).value];`,
  { env: { THINKTHEN_CACHE: folderA, HOME: scratch, XDG_CACHE_HOME: scratch } });
  assert.deepEqual(value, [true, true]);
  assert.equal(await backend.count(), 1);
  assert.equal((await ask(backend, `return new tt.Engine({ cache: false }).usage();`)).value.retries, 0);
  assert.equal(files(folderA).length, 1, 'the answer lands in folder A');
  assert.deepEqual(readdirSync(scratch), [], 'nothing lands under the scratch cache home');
});

test('baseUrl and a named cache override their setting', async (t) => {
  const first = await startBackend(t);
  const second = await startBackend(t);
  const folder = join(first.folder, 'named');
  const { value } = await ask(first, `
    const out = {};
    // A cache folder belongs to one backend address, so this engine keeps none.
    out.second = (await new tt.Engine({ baseUrl: ${JSON.stringify(second.base())}, cache: false }).decide('Refund?', 'elsewhere')).value;
    const named = new tt.Engine({ cache: ${JSON.stringify(folder)} });
    out.named = [(await named.decide('Refund?', 'kept')).value, (await named.decide('Refund?', 'kept')).value];
    return out;`);
  assert.deepEqual(value, {
    second: true,
    named: [true, true],
  });
  assert.equal(await second.count(), 1, 'baseUrl sends to the second backend');
  assert.equal(files(folder).length, 1, 'the named cache holds one answer');
  assert.equal(await first.count(), 1, 'the named cache sends once');
});

test('a refused setting throws usage from the constructor and sends nothing', async (t) => {
  const backend = await startBackend(t);
  const { value } = await ask(backend, `
    const out = [];
    for (const options of [{ timeoutSeconds: 0 }, { maxRetries: 1.5 }, { maxRequestBytes: 0 }, { throttle: 33 }, { throttle: '4' }, { nope: 1 }, { cache: true }, 'fast']) {
      try { new tt.Engine(options); out.push('built'); } catch (error) { out.push([error.name, error.kind, error.message]); }
    }
    return out;`);
  const usage = (message) => ['ThinkThenError', 'usage', message];
  assert.deepEqual(value, [
    usage('a timeout is a time above zero'),
    usage('options.maxRetries is a whole number'),
    usage('max_request_bytes is a whole number of at least 1'),
    usage('a throttle is a whole number from 1 through 32'),
    usage('options.throttle is a whole number'),
    usage('new Engine takes no option nope'),
    usage('options.cache is false or a folder path'),
    usage('new Engine takes one options object'),
  ]);
  assert.equal(await backend.count(), 0);
});

// The token cap reaches TypeScript through from_env; a regression if from_env stops reading the variable.
test('the token cap variable refuses a call before any request', async (t) => {
  const backend = await startBackend(t);
  const { error } = await ask(backend, `await new tt.Engine({ cache: false }).decide('Refund?', 'text');`,
    { env: { THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL: '10' } });
  assert.deepEqual([error?.name, error?.kind, error?.message], ['ThinkThenError', 'usage',
    "max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request"]);
  assert.equal(await backend.count(), 0);
});

test('an explicit engine batch outranks an invalid environment batch', async (t) => {
  const backend = await startBackend(t);
  const { value, error } = await ask(backend, `
    const engine = new tt.Engine({ batch: 1, cache: false });
    const first = await engine.decide_many('Refund?', ['one', 'two']);
    const second = await engine.decide_many('Refund?', ['three', 'four'], { batch: 'max' });
    return { first, second };`, { env: { THINKTHEN_BATCH: 'invalid' } });
  assert.equal(error, undefined, JSON.stringify(error));
  assert.deepEqual(value.first.value, [true, true]);
  assert.equal(value.first.facts.requests_sent, 2);
  assert.equal(value.second.facts.requests_sent, 1);
  assert.equal(await backend.count(), 3);
});

test('the first explicit throttle holds for the process and the default engine', async (t) => {
  const backend = await startBackend(t);
  const run = child(backend, `
    new tt.Engine({ throttle: 4 });
    let second;
    try { new tt.Engine({ throttle: 8 }); } catch (error) { second = error.message; }
    line(second);
    return Promise.all([0, 1, 2, 3, 4].map(async (at) => (await tt.decide('Refund?', 'held ' + at)).value));`, { arm: 'arm/held' });
  assert.equal(await backend.wait(4), 4);
  await sleep(300);
  assert.equal(await backend.count(), 4, 'the module-level calls follow throttle 4');
  backend.release();
  await run.exited;
  assert.deepEqual(run.lines.map((held) => held.value), [
    'throttle 4 is already active for this process; use throttle 4 or drop the throttle argument',
    { value: [true, true, true, true, true] },
  ]);
});
