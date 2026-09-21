// The six error kinds map to ThinkThenError with the retry signal. The
// usage and malformed-reply arms run offline; the refused address points
// at a port nothing owns; the cancelled and deadline arms need the wire
// and live in bulk.test.mjs.

import { test } from 'node:test';
import assert from 'node:assert/strict';

import * as tt from '../index.mjs';

async function kindOf(run) {
  try {
    await run();
  } catch (held) {
    return held;
  }
  return null;
}

test('usage: filter refuses a band', async () => {
  const held = await kindOf(() =>
    tt.filter(
      tt.question({ decide: 'Refund?', threshold: [0.2, 0.8] }),
      ['one record'],
    ),
  );
  assert.ok(held instanceof tt.ThinkThenError);
  assert.equal(held.kind, 'usage');
  assert.equal(held.retryable, false);
});

test('usage: rank refuses a threshold', async () => {
  const held = await kindOf(() =>
    tt.rank(tt.question({ decide: 'Refund?', threshold: 0.9 }), ['one record']),
  );
  assert.equal(held.kind, 'usage');
});

test('usage: a blank question', async () => {
  const held = await kindOf(() => tt.decide('   ', 'some text'));
  assert.equal(held.kind, 'usage');
});

test('backend: a malformed reply is not retryable', { skip: process.env.THEN_TS_DEAD === '1' }, async () => {
  const held = await kindOf(() => tt.decide('Refund?', 'this evidence is malformed'));
  assert.equal(held.kind, 'backend');
  assert.equal(held.retryable, false);
  assert.match(held.message, /422|malformed/);
});

test('backend: a refused address is not retryable', { skip: process.env.ENGINE_NULL === '1' }, async () => {
  // The addon reads the address once per process, so check.sh runs this
  // suite a second time in a child pointed at a dead port, with only this
  // test unskipped and THEN_TS_DEAD set.
  const held = await kindOf(() => tt.decide('Refund?', 'I want a refund'));
  if (process.env.THEN_TS_DEAD !== '1') return; // only the dead-address child asserts
  assert.equal(held.kind, 'backend');
  assert.equal(held.retryable, false);
});

test('a failure never reads as a value', async () => {
  const held = await kindOf(() => tt.decide('Refund?', 'this evidence is malformed'));
  assert.ok(held instanceof Error);
  assert.equal(held.name, 'ThinkThenError');
});
