// Calls run on their own worker threads, never on Node's libuv pool or the
// JavaScript thread. The engine's public throttle holds requests in flight.
import { test } from 'node:test';
import assert from 'node:assert/strict';

import { child, sleep, startBackend } from './backend.mjs';

test('five calls under throttle 4 hold four requests and leave the libuv pool free', async (t) => {
  const backend = await startBackend(t);
  const run = child(backend, `
    const engine = new tt.Engine({ throttle: 4 });
    const calls = [0, 1, 2, 3, 4].map((at) => engine.decide('Refund?', 'held ' + at));
    const { readFile } = await import('node:fs/promises');
    setTimeout(async () => { await readFile(${JSON.stringify(new URL(import.meta.url).pathname)}); line('read'); }, 50);
    return Promise.all(calls);`, { arm: 'arm/held', env: { UV_THREADPOOL_SIZE: '2' } });
  assert.equal(await backend.wait(4), 4);
  await sleep(300);
  assert.equal(await backend.count(), 4, 'the throttle holds the fifth request');
  assert.deepEqual(run.lines.map((held) => held.value), ['read'], 'a file read resolves during the hold');
  backend.release();
  await run.exited;
  assert.deepEqual(run.lines.at(-1).value, { value: [true, true, true, true, true] });
  assert.equal(await backend.count(), 5);
});

test('a running batch leaves the event loop free', async (t) => {
  const backend = await startBackend(t);
  const run = child(backend, `
    const records = Array.from({ length: 96 }, (_, at) => 'record ' + at);
    const batch = tt.decide_many('Refund?', records);
    const began = performance.now();
    for (let tick = 0; tick < 30; tick += 1) await new Promise((done) => setTimeout(done, 10));
    const drift = performance.now() - began - 300;
    return { drift, answers: (await batch).length };`, { arm: 'arm/delay/100' });
  await run.exited;
  const { value } = run.lines.at(-1).value;
  assert.equal(value.answers, 96);
  assert.ok(value.drift < 250, `the timers drifted ${Math.round(value.drift)} ms`);
});
