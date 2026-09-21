// The eight verbs plus the bulk spelling, the audit trail, and the
// counters, offline against the null backend. The null rule is the stub's
// own: evidence naming a refund scores 0.97, "maybe" 0.55, anything else
// 0.03; a choose weighs its options' own text; a score spreads the
// evidence keyword over three levels; a tag weighs its labels' own text.

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

test('usage counts sends and reset clears them', async () => {
  tt.reset_usage();
  await tt.decide('Does the customer ask for a refund?', 'I want a refund for order 9');
  const one = tt.usage();
  assert.equal(one.requests, 1);
  assert.equal(one.cache_answers, 0);
  tt.reset_usage();
  assert.equal(tt.usage().requests, 0);
});

test('a question value is asked, not called, and never a string again', () => {
  const refund = tt.question({ decide: 'Refund?', threshold: [0.2, 0.8] });
  assert.throws(() => refund(), /asked, not called/);
  assert.equal(typeof refund, 'function');
});
