// Bulk on the wire: one crossing at the process width, an AbortSignal
// that stops a batch with the stub frozen, the deadline's own kind, the
// event loop staying free, and one connection reused. Runs only when
// THEN_TS_WIRE_URL names the stub; check.sh starts it.

import { test } from 'node:test';
import assert from 'node:assert/strict';

import * as tt from '../index.mjs';

const wireUrl = process.env.THEN_TS_WIRE_URL;
const stub = wireUrl ? new URL(wireUrl).port : null;

async function stats() {
  const held = await fetch(`http://127.0.0.1:${stub}/v1/stats`);
  return held.json();
}

async function reset() {
  await fetch(`http://127.0.0.1:${stub}/v1/reset`, { method: 'POST' });
}

async function kindOf(run) {
  try {
    await run();
  } catch (held) {
    return held;
  }
  return null;
}

test('a bulk call crosses once and holds the width', { skip: !wireUrl }, async () => {
  await reset();
  const records = Array.from({ length: 64 }, (_, at) =>
    at % 2 === 0 ? `I want a refund for order ${at}` : `plain message ${at}`,
  );
  const held = await tt.decide_many('Does the customer ask for a refund?', records);
  assert.equal(held.length, 64);
  assert.equal(held[0], true);
  assert.equal(held[1], false);
  const after = await stats();
  assert.equal(after.requests, 64);
  assert.ok(after.max_in_flight <= 32, `width held at or under 32, saw ${after.max_in_flight}`);
});

test('an AbortSignal stops a batch: nothing served after the return', { skip: !wireUrl }, async () => {
  await reset();
  const controller = new AbortController();
  const records = Array.from({ length: 64 }, (_, at) => `I want a refund for order ${at}`);
  const started = process.hrtime.bigint();
  setTimeout(() => controller.abort(), 350);
  const held = await kindOf(() =>
    tt.decide_many('Does the customer ask for a refund?', records, { signal: controller.signal }),
  );
  const took = Number(process.hrtime.bigint() - started) / 1e6;
  assert.ok(held instanceof tt.ThinkThenError, 'the call rejects');
  assert.equal(held.kind, 'cancelled');
  assert.ok(took < 1500, `returned promptly, took ${took.toFixed(0)} ms`);
  const atReturn = await stats();
  await new Promise((done) => setTimeout(done, 1200));
  const settled = await stats();
  assert.equal(settled.requests, atReturn.requests, 'the stub served nothing after the return');
  assert.ok(settled.requests <= 64);
});

test('a deadline returns its own kind and freezes the stub', { skip: !wireUrl }, async () => {
  await reset();
  const records = Array.from({ length: 64 }, (_, at) => `I want a refund for order ${at}`);
  const held = await kindOf(() =>
    tt.decide_many('Does the customer ask for a refund?', records, { deadlineMs: 450 }),
  );
  assert.ok(held instanceof tt.ThinkThenError);
  assert.equal(held.kind, 'deadline');
  assert.ok(held.retryable === true, 'a fresh budget may answer');
  assert.match(held.message, /deadline/i, 'the message names the limit');
  const atReturn = await stats();
  await new Promise((done) => setTimeout(done, 900));
  assert.equal((await stats()).requests, atReturn.requests);
});

test('the event loop stays free during a bulk run', { skip: !wireUrl }, async () => {
  // Timer drift over fixed 10 ms ticks, measured once before the run and
  // once during it. A blocked loop shows hundreds of milliseconds of
  // drift; a free one stays within scheduling noise.
  const driftOver = async (ticks) => {
    const start = process.hrtime.bigint();
    const marks = [];
    await new Promise((done) => {
      const step = () => {
        marks.push(Number(process.hrtime.bigint() - start) / 1e6);
        if (marks.length < ticks) setTimeout(step, 10);
        else done();
      };
      setTimeout(step, 10);
    });
    return marks[marks.length - 1] - 10 * (marks.length - 1);
  };

  const idle = await driftOver(20);
  assert.ok(idle < 250, `idle baseline drift ${idle.toFixed(1)} ms`);

  const records = Array.from({ length: 96 }, () => 'I want a refund for order 9');
  const running = tt.decide_many('Does the customer ask for a refund?', records);
  const during = await driftOver(30);
  await running;
  assert.ok(during < 250, `the loop stayed free: ${during.toFixed(1)} ms drift over 30 ticks`);
});

test('sequential calls reuse one connection', { skip: !wireUrl }, async () => {
  await reset();
  for (let at = 0; at < 20; at += 1) {
    await tt.decide('Does the customer ask for a refund?', `I want a refund for order ${at}`);
  }
  const after = await stats();
  assert.equal(after.requests, 20);
  assert.ok(after.connections <= 3, `connections stayed low, saw ${after.connections}`);
});
