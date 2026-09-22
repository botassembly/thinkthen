// The deadline's bounds, checked at the contract's one door: a NaN and an
// oversized budget reject with the usage kind instead of crashing the
// process; minus one milliseconds means no deadline; zero stays a spent
// deadline. Before the fix a huge budget panicked in the unchecked
// conversion and took the Node process down.
//
// Offline; check.sh's null section runs it.

import { test } from 'node:test';
import assert from 'node:assert/strict';

import * as tt from '../index.mjs';

const offline = process.env.ENGINE_NULL === '1';

async function rejection(call) {
  try {
    await call();
    return null;
  } catch (raised) {
    return raised;
  }
}

test('a hostile budget rejects with the usage kind', { skip: !offline }, async () => {
  for (const held of [Number.NaN, Number.MAX_VALUE, -5]) {
    const raised = await rejection(() =>
      tt.decide('Is this a complaint?', 'I want a refund', { deadlineMs: held }),
    );
    assert.ok(raised instanceof tt.ThinkThenError, `deadlineMs ${held} rejects`);
    assert.equal(raised.kind, 'usage', `deadlineMs ${held} is refused as usage`);
  }
});

test('minus one milliseconds means no deadline', { skip: !offline }, async () => {
  const answer = await tt.decide('Is this a complaint?', 'I want a refund', { deadlineMs: -1 });
  assert.equal(answer, true);
});

test('zero stays a spent deadline', { skip: !offline }, async () => {
  const raised = await rejection(() =>
    tt.decide('Is this a complaint?', 'I want a refund', { deadlineMs: 0 }),
  );
  assert.ok(raised instanceof tt.ThinkThenError, 'a spent deadline rejects');
  assert.equal(raised.kind, 'deadline');
});
