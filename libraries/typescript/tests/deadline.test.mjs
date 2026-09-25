// The host's deadline rule (ADR 0041): null, a missing key, and -1 are no
// deadline, 0 is spent, and anything else is a whole number of milliseconds
// up to 4294967295000. Refusals send nothing.
import { test } from 'node:test';
import assert from 'node:assert/strict';

import { ask, child, startBackend, until } from './backend.mjs';

const refused = (written) =>
  `deadlineMs ${written} is not a deadline; use -1, null, or a whole number of milliseconds from 0 to 4294967295000`;

test('each deadline spelling runs, is spent, or is refused before a send', async (t) => {
  const backend = await startBackend(t);
  const { value } = await ask(backend, `
    const out = {};
    const spellings = { minusOne: -1, null: null, missing: undefined, zero: 0, minusTwo: -2, half: 0.5,
      over: 4294967296000, most: 4294967295000, nan: NaN, infinity: Infinity, huge: 1e300, max: Number.MAX_VALUE,
      tiny: 1e-7, yes: true, no: false, five: '5', list: [] };
    for (const [name, deadlineMs] of Object.entries(spellings)) {
      const options = deadlineMs === undefined ? {} : { deadlineMs };
      try { out[name] = await tt.decide('Refund?', name, options); }
      catch (error) { out[name] = [error.name, error.kind, error.message]; }
    }
    return out;`);
  const usage = (message) => ['ThinkThenError', 'usage', message];
  const hostRule = usage('options.deadlineMs is a number of milliseconds; no deadline is spelled null, left out, or -1');
  assert.deepEqual(value, {
    minusOne: true,
    null: true,
    missing: true,
    zero: ['ThinkThenError', 'deadline', 'the deadline of 0 s passed before the call answered'],
    minusTwo: usage(refused('-2')),
    half: usage(refused('0.5')),
    over: usage(refused('4294967296000')),
    most: true,
    nan: usage(refused('NaN')),
    infinity: usage(refused('Infinity')),
    huge: usage(refused('1e+300')),
    max: usage(refused('1.7976931348623157e+308')),
    tiny: usage(refused('1e-7')),
    yes: hostRule,
    no: hostRule,
    five: hostRule,
    list: hostRule,
  });
  assert.equal(await backend.count(), 4, 'only -1, null, missing, and the most send');
});

test('a deadline ends a call during a held send', async (t) => {
  const backend = await startBackend(t);
  const run = child(backend, "return tt.decide('Refund?', 'held', { deadlineMs: 300 });", { arm: 'arm/held' });
  assert.equal(await backend.wait(1), 1);
  const sent = performance.now();
  assert.ok(await until(() => run.lines.length > 0, 2000));
  const [{ at, value }] = run.lines;
  assert.equal(value.error.kind, 'deadline');
  assert.ok(at - sent <= 450, `rejected ${Math.round(at - sent)} ms after the send`);
  assert.equal(await backend.count(), 1);
});
