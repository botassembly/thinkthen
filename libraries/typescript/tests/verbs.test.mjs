// The eight verbs plus the bulk spelling, the audit trail, and the
// counters, offline against the null backend. The null rule is the stub's
// own: evidence naming a refund scores 0.97, "maybe" 0.55, anything else
// 0.03; a choose weighs its options' own text; a score spreads the
// evidence keyword over three levels; a tag weighs its labels' own text.
//
// The stand-in's one synthesized partial failure (conformance case 74)
// fires only under the test-only `ENGINE_SYNTHETIC_PARTIAL` opt-in, which
// the door reads when it builds the engine on the first call.
process.env.ENGINE_SYNTHETIC_PARTIAL = '1';

import { test } from 'node:test';
import assert from 'node:assert/strict';

import * as tt from '../index.mjs';

test('decide: a plain string takes the default cut', async () => {
  assert.equal(await tt.decide('Does the customer ask for a refund?', 'I want a refund for order 9'), true);
  assert.equal(await tt.decide('Does the customer ask for a refund?', 'Just saying hi'), false);
});

test('decide: a band gives three values, null inside', async () => {
  const refund = tt.question({
    decide: 'Does the customer ask for a refund?',
    threshold: [0.2, 0.8],
  });
  assert.equal(await tt.decide(refund, 'I want a refund for order 9'), true);
  assert.equal(await tt.decide(refund, 'maybe later'), null);
  assert.equal(await tt.decide(refund, 'Just saying hi'), false);
});

test('decide_many: decide in bulk, answers in input order', async () => {
  const refund = tt.question({
    decide: 'Does the customer ask for a refund?',
    threshold: [0.2, 0.8],
  });
  const held = await tt.decide_many(refund, [
    'I want a refund for order 9',
    'Just saying hi',
    'maybe later',
  ]);
  assert.deepEqual(held, [true, false, null]);
});

test('choose: the option whose own text carries the keyword wins', async () => {
  const pick = await tt.choose(
    { choose: 'Which team owns this?', options: ['the refund team', 'somewhere else'] },
    'Whatever the evidence says',
  );
  assert.equal(pick, 'the refund team');
});

test('score: the specification position on the levels', async () => {
  const value = await tt.score(
    { score: 'How strong is the refund claim?', levels: ['low', 'mid', 'high'] },
    'maybe later',
  );
  // The maybe arm spreads 0.20/0.55/0.25 over low/mid/high: 1.05.
  assert.ok(Math.abs(value - 1.05) < 1e-9);
});

test('tag: the labels whose own text carries the keyword hold', async () => {
  const held = await tt.tag(
    { tag: 'Name what applies', labels: ['a refund label', 'nothing here'] },
    'Whatever the evidence says',
  );
  assert.deepEqual(held, ['a refund label']);
});

test('filter: keeps the records that reached the mark, in order', async () => {
  const kept = await tt.filter('Is this a complaint?', [
    'Just saying hi',
    'I want a refund for order 9',
    'Please refund my broken mug',
  ]);
  assert.deepEqual(kept, ['I want a refund for order 9', 'Please refund my broken mug']);
});

test('rank: most likely yes first, ties in input order', async () => {
  const ranked = await tt.rank('Is this a complaint?', [
    'Just saying hi',
    'maybe later',
    'I want a refund for order 9',
  ]);
  assert.deepEqual(ranked.map((held) => held.record), [
    'I want a refund for order 9',
    'maybe later',
    'Just saying hi',
  ]);
  assert.ok(ranked[0].probability >= ranked[1].probability);
  assert.ok(ranked[1].probability >= ranked[2].probability);
});

test('find: picks a unit and reports its probability', async () => {
  const found = await tt.find('Which unit answers best?', [
    'one plain unit',
    'another plain unit',
  ]);
  assert.ok(['one plain unit', 'another plain unit'].includes(found.unit));
  assert.equal(found.index, found.unit === null ? null : ['one plain unit', 'another plain unit'].indexOf(found.unit));
  assert.equal(typeof found.probability, 'number');
});

test('details: probability, value, model, digest, sends', async () => {
  const held = await tt.details('Does the customer ask for a refund?', 'I want a refund for order 9');
  assert.equal(held.value, true);
  assert.ok(Math.abs(held.probability - 0.97) < 1e-9);
  assert.equal(typeof held.model, 'string');
  assert.match(held.digest, /^[0-9a-f]{64}$/);
  assert.equal(held.sends, 1);
});

test('details: the requests list and the failure count (0053, 0054)', async () => {
  const held = await tt.details('Does the customer ask for a refund?', 'I want a refund for order 9');
  assert.ok(Array.isArray(held.requests), 'requests is always an array');
  assert.equal(held.requests.length, 1, 'one logical request, one digest');
  assert.match(held.requests[0], /^[0-9a-f]{64}$/);
  assert.equal(held.failed_questions, 0, 'always present, including zero');
});

test('details: nearest is the level on a score question, null elsewhere (settled 2026-09-21)', async () => {
  const scored = await tt.details(
    { score: 'How strong is the refund claim?', levels: ['low', 'mid', 'high'] },
    'maybe later',
  );
  assert.equal(scored.nearest, 'mid');
  const decided = await tt.details('Does the customer ask for a refund?', 'I want a refund for order 9');
  assert.equal(decided.nearest, null, 'null on every non-score verb');
});

test('annotate: a failed question carries the ruled marker, never null (0054)', async () => {
  const set = JSON.stringify({
    version: 1,
    questions: {
      refund: { decide: 'Is this a refund request?', threshold: 0.5 },
      topic: { decide: 'Is this a billing problem?', threshold: 0.5 },
    },
  });
  const rows = await tt.annotate(set, ['order 4471: charged twice, please refund']);
  assert.deepEqual(rows[0].topic, {
    failed: { kind: 'backend', cause: 'missing_answer' },
  });
  assert.equal(rows[0].refund, true, 'the good answer rides beside the failure');
  const clean = await tt.annotate(set, ['I want a refund for order 4471']);
  assert.equal(clean[0].topic, true);
});

test('usage counts sends', async () => {
  // No reset exists (ruling 4): the test takes the difference across one
  // send; a caller who wants fresh counters builds a new engine.
  const before = tt.usage().requests;
  await tt.decide('Does the customer ask for a refund?', 'I want a refund for order 9');
  const one = tt.usage();
  assert.equal(one.requests - before, 1);
  assert.equal(one.cache_answers, 0);
});

test('a question value is asked, not called, and never a string again', () => {
  const refund = tt.question({ decide: 'Refund?', threshold: [0.2, 0.8] });
  assert.throws(() => refund(), /asked, not called/);
  assert.equal(typeof refund, 'function');
});

test('choose: the ruled shape, options in the last object', async () => {
  const pick = await tt.choose('Which team owns this?', 'Whatever the evidence says', {
    options: ['the refund team', 'somewhere else'],
  });
  assert.equal(pick, 'the refund team');
});

test('tag: the ruled shape, labels in the last object', async () => {
  const held = await tt.tag('Name what applies', 'Whatever the evidence says', {
    labels: ['a refund label', 'nothing here'],
  });
  assert.deepEqual(held, ['a refund label']);
});

test('rank: top holds the first n of the ordered result', async () => {
  const records = ['Just saying hi', 'maybe later', 'I want a refund for order 9'];
  const all = await tt.rank('Is this a complaint?', records);
  const two = await tt.rank('Is this a complaint?', records, { top: 2 });
  assert.equal(all.length, 3);
  assert.deepEqual(two, all.slice(0, 2));
  assert.equal(two.length, 2);
});

test('rank: top rides with a question value too', async () => {
  const held = await tt.rank(tt.question({ decide: 'Is this a complaint?' }), ['one refund', 'nothing', 'maybe'], {
    top: 1,
  });
  assert.equal(held.length, 1);
  assert.equal(held[0].record, 'one refund');
});
