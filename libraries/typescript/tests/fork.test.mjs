// Ian's item 6 for TypeScript: a fork after the first call answers its own
// call.
//
// Node has no raw fork(); its two process-survival shapes are
// child_process.fork() — fork + exec, so the child is a fresh process —
// and worker_threads.Worker, which shares the process. Both run here
// after the parent's first call, under a 10 s watchdog, so a hang or a
// frozen event loop fails instead of hanging the suite.
//
// Run with: ENGINE_NULL=1 node --test tests/fork.test.mjs

import { fork } from 'node:child_process';
import { Worker } from 'node:worker_threads';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import assert from 'node:assert/strict';

import * as tt from '../index.mjs';

const BOUND_MS = 10_000;
const ASK = 'Does the customer ask for a refund?';

test('a child process forked after the first call answers its own call', async () => {
  const first = await tt.decide(ASK, 'I want a refund for order 9');
  assert.equal(first, true, "the parent's first call answers");

  const childPath = fileURLToPath(new URL('./fork_child.mjs', import.meta.url));
  const child = fork(childPath, [], { stdio: ['ignore', 'pipe', 'pipe', 'ipc'] });

  const outcome = await new Promise((resolve, reject) => {
    const timer = setTimeout(() => {
      child.kill('SIGKILL');
      reject(new Error(`the child hung past ${BOUND_MS} ms — a fork that does not answer`));
    }, BOUND_MS);
    child.on('message', (message) => {
      clearTimeout(timer);
      resolve(message);
    });
    child.on('exit', (code) => {
      if (code !== 0) {
        clearTimeout(timer);
        reject(new Error(`the child exited ${code} without answering`));
      }
    });
  });

  assert.deepEqual(outcome, { answered: true }, 'the child answered through the same addon');
});

test('a worker thread made after a call answers its own call', async () => {
  const first = await tt.decide(ASK, 'I want a refund for order 9');
  assert.equal(first, true, "the parent's first call answers");

  const workerPath = fileURLToPath(new URL('./fork_worker.mjs', import.meta.url));
  const worker = new Worker(workerPath);

  const outcome = await new Promise((resolve, reject) => {
    const timer = setTimeout(async () => {
      await worker.terminate();
      reject(new Error(`the worker hung past ${BOUND_MS} ms`));
    }, BOUND_MS);
    worker.on('message', (message) => {
      clearTimeout(timer);
      resolve(message);
    });
    worker.on('error', (error) => {
      clearTimeout(timer);
      reject(error);
    });
  });

  assert.deepEqual(outcome, { answered: true }, 'the worker answered through the same addon');
});
