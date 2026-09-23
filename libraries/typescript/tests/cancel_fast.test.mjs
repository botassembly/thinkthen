// Cancel on a fast backend: an AbortSignal stops a null-backend batch
// within about a tick.
//
// The starvation this pins, found by the adversarial review and fixed in
// the stand-in: before the fix the poll ran only when the wait channel
// idled, so a fast backend starved it and the stop waited for the whole
// batch (8.48 s on a three-million-record null batch). Here a
// two-million-record null batch is aborted about one second in and must
// reject within 500 ms of the abort — the batch runs about 9.7 s deaf, so
// the threshold separates the two behaviors. The clock starts at the abort
// itself: on a loaded host the timer fires late, and a clock from the
// start of the call blamed that on the stop (surfaces-review-5). Offline;
// check.sh's null section runs it.

import { test } from 'node:test';
import assert from 'node:assert/strict';

import * as tt from '../index.mjs';

test(
  'a fast backend hears an AbortSignal within a tick',
  { skip: !(process.env.THINKTHEN_NULL === '1' || process.env.ENGINE_NULL === '1') },
  async () => {
    const records = Array.from({ length: 2_000_000 }, (_, at) => `record ${at}`);
    const controller = new AbortController();
    let aborted = null;
    const timer = setTimeout(() => {
      aborted = process.hrtime.bigint();
      controller.abort();
    }, 1000);
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
    assert.ok(held instanceof tt.ThinkThenError, 'the call rejects');
    assert.equal(held.kind, 'cancelled');
    assert.ok(aborted !== null, 'the batch was still running when the abort fired');
    const took = Number(process.hrtime.bigint() - aborted) / 1e6;
    assert.ok(
      took < 500,
      `the stop landed ${took.toFixed(0)} ms after the abort; the deaf batch runs about 9.7 s`,
    );
  },
);
