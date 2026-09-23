// The deadline's bounds, checked at the contract's one door: a NaN, an
// oversized budget, every negative other than the sentinel, and any
// non-number a caller might pass reject with the usage kind instead of
// crashing the process or quietly disabling the deadline. One spelling
// everywhere (third review, item 21): `null`, an absent key, and the
// minus-one sentinel all mean no deadline; zero stays a spent deadline.
// Before the fix a huge budget panicked in the unchecked conversion and
// took the Node process down, and `true`, `"5"`, and `[]` were coerced
// into numbers (`Number([])` is zero — a silent spent deadline).
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
  for (const held of [Number.NaN, Number.MAX_VALUE, -5, -0.5, true, '5', []]) {
    const raised = await rejection(() =>
      tt.decide('Is this a complaint?', 'I want a refund', { deadlineMs: held }),
    );
    assert.ok(raised instanceof tt.ThinkThenError, `deadlineMs ${held} rejects`);
    assert.equal(raised.kind, 'usage', `deadlineMs ${held} is refused as usage`);
  }
});

test('null, an absent budget, and the sentinel all mean no deadline', { skip: !offline }, async () => {
  const explicit = await tt.decide('Is this a complaint?', 'I want a refund', { deadlineMs: null });
  assert.equal(explicit, true);
  const absent = await tt.decide('Is this a complaint?', 'I want a refund');
  assert.equal(absent, true);
  const sentinel = await tt.decide('Is this a complaint?', 'I want a refund', { deadlineMs: -1 });
  assert.equal(sentinel, true);
});

test('zero stays a spent deadline', { skip: !offline }, async () => {
  const raised = await rejection(() =>
    tt.decide('Is this a complaint?', 'I want a refund', { deadlineMs: 0 }),
  );
  assert.ok(raised instanceof tt.ThinkThenError, 'a spent deadline rejects');
  assert.equal(raised.kind, 'deadline');
});
