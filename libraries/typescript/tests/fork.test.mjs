// A child process and a worker thread started after the first call each
// answer their own call. A worker shares the process's default engine.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';

import { ask, startBackend } from './backend.mjs';

const script = (name) => JSON.stringify(fileURLToPath(new URL(name, import.meta.url)));

test('a forked child and a worker thread answer after the first call', async (t) => {
  const backend = await startBackend(t);
  const answered = await ask(backend, `
    const { fork } = await import('node:child_process');
    const { Worker } = await import('node:worker_threads');
    const { once } = await import('node:events');
    const first = await tt.decide('Does the customer ask for a refund?', 'parent text');
    const before = tt.usage().requests_sent;
    const forked = fork(${script('fork_child.mjs')}, { execArgv: [] });
    const [fromChild] = await once(forked, 'message');
    const worker = new Worker(${script('fork_worker.mjs')}, { execArgv: [] });
    const [fromWorker] = await once(worker, 'message');
    await worker.terminate();
    return { first, fromChild, fromWorker, workerSends: tt.usage().requests_sent - before };`);
  assert.deepEqual(answered.value, { first: true, fromChild: { answered: true }, fromWorker: { answered: true }, workerSends: 1 }, JSON.stringify(answered));
  assert.equal(await backend.count(), 3);
});
