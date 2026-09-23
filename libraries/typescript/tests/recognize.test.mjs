// `recognize` and `relate`: the two functions whose results have no fixed
// size, offline against the recorded answers. The deck's calls
// (`recognize-surfaces.md`) run as written where the recordings cover
// them; the one place the deck and the recording disagree is pinned as a
// finding, not shimmed. The replay engine ignores call options, so the
// cancel and deadline kinds are the door's, proven for the other verbs in
// errors.test.mjs and bulk.test.mjs; offline, only the usage kind fires
// here, and the kind and retryable fields are asserted on it.

import { test } from 'node:test';
import assert from 'node:assert/strict';

import * as tt from '../index.mjs';

const MARIA = 'Maria Chen joined Northwind Freight in Chicago last spring.';
const OFFSET_TEXT = 'Le café 😀 Maria Chen arrived.';
const ALERTS = [
  'Checkout returns 500 at the payment step.',
  'Card charges are failing for every customer.',
  'The nightly export ran two hours late.',
  'The payments database ran out of disk space.',
];

function signal() {
  return new AbortController().signal;
}

test('the deck\'s recognize call runs with the recorded rules', async () => {
  const found = await tt.recognize(MARIA, {
    kinds: ['person', 'organization', 'place'],
    relations: { works_for: ['person', 'organization'], based_in: ['organization', 'place'] },
    signal: signal(),
  });
  assert.equal(found.entities.length, 3);
  assert.equal(found.entities[0].kind, 'person');
  assert.equal(found.entities[0].text, 'Maria Chen');
  assert.equal(found.entities[0].start, 0);
  assert.equal(found.entities[0].end, 10);
  assert.equal(found.entities[1].text, 'Northwind Freight');
  assert.equal(found.entities[2].text, 'Chicago');
  for (const entity of found.entities) {
    assert.equal(MARIA.slice(entity.start, entity.end), entity.text, 'the slice is the name');
  }
  assert.deepEqual(found.relations, [{ name: 'works_for', source: 1, target: 2, probability: 1 }]);
});

test('finding: the deck\'s call as written names located_in, which the recording does not cover', async () => {
  // The deck's TypeScript section asks `located_in` on this sentence; the
  // C01 recording covers `works_for` and `based_in`. The call cannot run
  // as written until the deck's rule name or the recording changes.
  await assert.rejects(
    () =>
      tt.recognize(MARIA, {
        kinds: ['person', 'organization', 'place'],
        relations: { works_for: ['person', 'organization'], located_in: ['*', 'place'] },
        signal: signal(),
      }),
    (raised) =>
      raised instanceof tt.ThinkThenError &&
      raised.kind === 'usage' &&
      raised.message.includes('located_in') &&
      raised.message.includes('works_for, based_in'),
  );
});

test('the any-kind end is the one-character string "*"', async () => {
  const found = await tt.recognize('The road from Hull to Leeds was closed.', {
    kinds: ['place'],
    relations: { located_in: ['*', 'place'] },
  });
  assert.deepEqual(
    found.entities.map((entity) => entity.text),
    ['Hull', 'Leeds'],
  );
  assert.deepEqual(found.relations, []);
});

test('a relation end naming a kind outside the asked kinds is a usage error', async () => {
  await assert.rejects(
    () =>
      tt.recognize('The road from Hull to Leeds was closed.', {
        kinds: ['place'],
        relations: { located_in: ['*', 'person'] },
      }),
    (raised) => raised instanceof tt.ThinkThenError && raised.kind === 'usage',
  );
});

test('offsets slice the name in UTF-16 units with an accent and an emoji before it', async () => {
  const found = await tt.recognize(OFFSET_TEXT, { kinds: ['person'] });
  const [entity] = found.entities;
  assert.equal(entity.text, 'Maria Chen');
  // The contract counts code points (10 to 20 here); JavaScript counts
  // UTF-16 units, and the emoji is two of them, so the surface answers
  // 11 to 21 and the slice is the name.
  assert.equal(entity.start, 11);
  assert.equal(entity.end, 21);
  assert.equal(OFFSET_TEXT.slice(entity.start, entity.end), 'Maria Chen');
});

test('relate runs as written and answers the recorded edges', async () => {
  const edges = await tt.relate(ALERTS, {
    relations: ['caused_by'],
    either: ['same_as'],
    signal: signal(),
  });
  assert.deepEqual(edges, [
    { name: 'same_as', source: 1, target: 2, probability: 0.61 },
    { name: 'caused_by', source: 1, target: 4, probability: 0.71 },
    { name: 'caused_by', source: 2, target: 1, probability: 0.65 },
    { name: 'caused_by', source: 2, target: 4, probability: 0.73 },
    { name: 'caused_by', source: 3, target: 4, probability: 0.55 },
  ]);
});

test('relate refuses more than 255 records with a usage error, and 255 reaches the engine', async () => {
  await assert.rejects(
    () => tt.relate(Array.from({ length: 256 }, (_, at) => `record ${at}`), { relations: ['caused_by'] }),
    (raised) =>
      raised instanceof tt.ThinkThenError &&
      raised.kind === 'usage' &&
      raised.message.includes('at most 255 records'),
  );
  await assert.rejects(
    () => tt.relate(Array.from({ length: 255 }, (_, at) => `record ${at}`), { relations: ['caused_by'] }),
    (raised) =>
      raised instanceof tt.ThinkThenError &&
      raised.kind === 'usage' &&
      !raised.message.includes('at most'),
  );
});

test('an unrecorded text is a usage error that is never a value', async () => {
  await assert.rejects(
    () => tt.recognize('a text the recordings never saw', { kinds: ['person'] }),
    (raised) =>
      raised instanceof tt.ThinkThenError &&
      raised.kind === 'usage' &&
      raised.retryable === false &&
      raised.name === 'ThinkThenError',
  );
});

test('the last object carries the inputs and the call options together', async () => {
  await assert.rejects(
    () => tt.recognize(MARIA, { kinds: ['person'], top: 3 }),
    (raised) => raised instanceof tt.ThinkThenError && raised.kind === 'usage' && raised.message.includes('options.top'),
  );
  await assert.rejects(
    () => tt.relate(ALERTS, { relations: ['caused_by'], kindField: '/kind' }),
    (raised) => raised instanceof tt.ThinkThenError && raised.kind === 'usage' && raised.message.includes('options.kindField'),
  );
});
