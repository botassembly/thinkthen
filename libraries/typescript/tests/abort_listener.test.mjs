// The AbortSignal bookkeeping: one shared signal gains no listener per
// call. Before the fix every call left its `abort` listener behind (the
// `once` option only removes it when the abort fires), so a server sharing
// one shutdown signal leaked one listener per call.
//
// Offline; check.sh's null section runs it.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { getEventListeners } from 'node:events';

import * as tt from '../index.mjs';

const offline = process.env.ENGINE_NULL === '1';

test('one shared AbortSignal gains no listener per call', { skip: !offline }, async () => {
  const controller = new AbortController();
  assert.equal(getEventListeners(controller.signal, 'abort').length, 0, 'the signal starts clean');
  for (let call = 1; call <= 25; call += 1) {
    await tt.decide('Is this a complaint?', 'I want a refund', { signal: controller.signal });
    assert.equal(
      getEventListeners(controller.signal, 'abort').length,
      0,
      `call ${call} left a listener on the shared signal`,
    );
  }
});

test('a pre-aborted signal adds no listener', { skip: !offline }, async () => {
  const controller = new AbortController();
  controller.abort();
  try {
    await tt.decide('Is this a complaint?', 'I want a refund', { signal: controller.signal });
  } catch {
    // What the engine does with a pre-fired token is the engine's rule;
    // this test pins the listener bookkeeping alone.
  }
  assert.equal(getEventListeners(controller.signal, 'abort').length, 0);
});

test('an abort that fires mid-call leaves no listener behind', { skip: !offline }, async () => {
  const controller = new AbortController();
  const records = Array.from({ length: 100_000 }, (_, at) => `record ${at}`);
  const timer = setTimeout(() => controller.abort(), 20);
  let held = null;
  try {
    await tt.decide_many('Is this a complaint?', records, { signal: controller.signal });
  } catch (raised) {
    held = raised;
  } finally {
    clearTimeout(timer);
  }
  assert.ok(held instanceof tt.ThinkThenError, 'the aborted call rejects');
  assert.equal(held.kind, 'cancelled');
  assert.equal(
    getEventListeners(controller.signal, 'abort').length,
    0,
    'the fired listener is gone',
  );
});
