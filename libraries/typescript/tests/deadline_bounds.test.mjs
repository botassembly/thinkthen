// The deadline's bounds, checked at the contract's one door: a NaN, an
// oversized budget, and every negative — including the contract's minus-one
// sentinel — reject with the usage kind instead of crashing the process or
// quietly disabling the deadline. `null` (and leaving the key out) is this
// host's one spelling of no deadline; zero stays a spent deadline. Before
// the fix a huge budget panicked in the unchecked conversion and took the
// Node process down, minus one silently meant "no deadline", and an
// explicit null read as a spent deadline.
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
  for (const held of [Number.NaN, Number.MAX_VALUE, -5, -1]) {
    const raised = await rejection(() =>
      tt.decide('Is this a complaint?', 'I want a refund', { deadlineMs: held }),
    );
    assert.ok(raised instanceof tt.ThinkThenError, `deadlineMs ${held} rejects`);
    assert.equal(raised.kind, 'usage', `deadlineMs ${held} is refused as usage`);
  }
});

test('null and an absent budget both mean no deadline', { skip: !offline }, async () => {
  const explicit = await tt.decide('Is this a complaint?', 'I want a refund', { deadlineMs: null });
  assert.equal(explicit, true);
  const absent = await tt.decide('Is this a complaint?', 'I want a refund');
  assert.equal(absent, true);
});

test('zero stays a spent deadline', { skip: !offline }, async () => {
  const raised = await rejection(() =>
    tt.decide('Is this a complaint?', 'I want a refund', { deadlineMs: 0 }),
  );
  assert.ok(raised instanceof tt.ThinkThenError, 'a spent deadline rejects');
  assert.equal(raised.kind, 'deadline');
});
