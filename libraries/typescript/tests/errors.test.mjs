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

test('usage: an unknown key in the last object names the key', async () => {
  const held = await kindOf(() => tt.decide('Refund?', 'some text', { urgent: true }));
  assert.ok(held instanceof tt.ThinkThenError);
  assert.equal(held.kind, 'usage');
  assert.equal(held.retryable, false);
  assert.match(held.message, /options\.urgent/);
});

test('usage: labels are not a choose key', async () => {
  const held = await kindOf(() => tt.choose('Which team?', 'some text', { labels: ['billing'] }));
  assert.equal(held.kind, 'usage');
  assert.match(held.message, /options\.labels/);
});

test('usage: choose with a bare question string wants options last', async () => {
  const held = await kindOf(() => tt.choose('Which team owns it?', 'some text'));
  assert.equal(held.kind, 'usage');
  assert.match(held.message, /options/);
});

test('usage: tag with a bare question string wants labels last', async () => {
  const held = await kindOf(() => tt.tag('Which topics?', 'some text'));
  assert.equal(held.kind, 'usage');
  assert.match(held.message, /labels/);
});

test('usage: top is a positive whole number', async () => {
  const held = await kindOf(() => tt.rank('Is this urgent?', ['one', 'two'], { top: 0 }));
  assert.equal(held.kind, 'usage');
  assert.match(held.message, /options\.top/);
});

test('usage: a question value carries its own options', async () => {
  const held = await kindOf(() =>
    tt.choose({ choose: 'Which team?', options: ['billing'] }, 'some text', { options: ['shipping'] }),
  );
  assert.equal(held.kind, 'usage');
  assert.match(held.message, /options/);
});

test('usage: a question value carries its own levels (settled 2026-09-21)', async () => {
  const held = await kindOf(() =>
    tt.score({ score: 'How urgent?', levels: ['low', 'high'] }, 'some text', { levels: ['one', 'two'] }),
  );
  assert.equal(held.kind, 'usage');
  assert.match(held.message, /levels/);
});

test('a spent deadline is legal: zero rejects with the deadline kind naming the budget (settled 2026-09-21)', async () => {
  const held = await kindOf(() =>
    tt.decide('Does the customer ask for a refund?', 'I want a refund for order 9', { deadlineMs: 0 }),
  );
  assert.equal(held.kind, 'deadline');
  assert.match(held.message, /0/);
});
