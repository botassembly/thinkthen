// Cancel on a fast backend: an AbortSignal stops a null-backend batch
// within about a tick.
//
// The starvation this pins, found by the adversarial review and fixed in
// the stand-in: before the fix the poll ran only when the wait channel
// idled, so a fast backend starved it and the stop waited for the whole
// batch (8.48 s on a three-million-record null batch). Here a
// two-million-record null batch is aborted about one second in and must
// reject within 1.5 s — the batch runs about 9.7 s deaf, so the threshold
// separates the two behaviors. Offline; check.sh's null section runs it.

import { test } from 'node:test';
import assert from 'node:assert/strict';

import * as tt from '../index.mjs';

test(
  'a fast backend hears an AbortSignal within a tick',
  { skip: process.env.ENGINE_NULL !== '1' },
  async () => {
    const records = Array.from({ length: 2_000_000 }, (_, at) => `record ${at}`);
    const controller = new AbortController();
    const started = process.hrtime.bigint();
    const timer = setTimeout(() => controller.abort(), 1000);
    let held = null;
    try {
      await tt.decide_many('Is this a complaint?', records, {
        signal: controller.signal,
      });
    } catch (raised) {
      held = raised;
    } finally {
      clearTimeout(timer);
    }
    const took = Number(process.hrtime.bigint() - started) / 1e6;
    assert.ok(held instanceof tt.ThinkThenError, 'the call rejects');
    assert.equal(held.kind, 'cancelled');
    assert.ok(
      took < 1500,
      `the stop landed at ${took.toFixed(0)} ms; the deaf batch runs about 9.7 s`,
    );
  },
);
