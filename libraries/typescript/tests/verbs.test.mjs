// Every verb in its host shape, the last-object rule, the input checks, and
// the error kinds, each against its own loopback backend. The generic arm
// gives the first option, level, or yes 0.9 and shares the rest.
import { test } from 'node:test';
import assert from 'node:assert/strict';

import { FAKE_KEY, ask, startBackend } from './backend.mjs';

test('each verb resolves its host shape, on the module and on an engine', async (t) => {
  const backend = await startBackend(t);
  const { value } = await ask(backend, `
    const band = tt.question({ decide: 'Does it ask for a refund?', threshold: [0.2, 0.8] });
    const shapes = (on) => Promise.all([
      on.decide('Does it ask for a refund?', 'I want a refund'),
      on.decide(band, 'I want a refund'),
      on.decide_many(band, ['one', 'two']),
      on.choose('Which team?', 'text', { options: ['billing', 'other'] }),
      on.score('How urgent?', 'text', { levels: ['low', 'mid', 'high'] }),
      on.tag({ tag: 'Which topics?', labels: ['billing', 'urgent'] }, 'text'),
      on.filter('Is it a complaint?', ['one', 'two']),
      on.rank('Is it urgent?', ['one', 'two', 'three'], { top: 2 }),
      on.find('Which line answers?', ['one', 'two']),
      on.annotate({ version: 1, questions: { refund: { decide: 'Refund?' }, team: { choose: 'Team?', options: ['a', 'b'] } } }, ['one']),
    ]);
    return { module: await shapes(tt), engine: await shapes(new tt.Engine({ cache: false })) };`);
  const expected = [
    true,
    true,
    [true, true],
    'billing',
    0.15,
    ['billing', 'urgent'],
    ['one', 'two'],
    [
      { index: 0, record: 'one', probability: 0.9 },
      { index: 1, record: 'two', probability: 0.9 },
    ],
    { index: 0, unit: 'one', probability: 0.9 },
    [{ refund: true, team: 'a' }],
  ];
  assert.deepEqual(value.module, expected);
  assert.deepEqual(value.engine, expected);
  assert.equal(await backend.count(), 27);
});

test('the slide sample runs as drawn', async (t) => {
  const backend = await startBackend(t);
  const { value } = await ask(backend, `
    const text = 'I want a refund for order 9';
    const message = 'I was charged twice and want a refund for order 9';
    const inbox = ['Where is my order?', 'I want a refund', 'Hello team', 'Escalate this', 'The refund never arrived', 'Weekly summary'];
    const options = ["billing", "shipping", "account"];
    const team = await tt.choose("Which team owns it?", text, { options });
    const labels = ["billing", "shipping", "urgent", "praise"];
    const topics = await tt.tag("Which topics?", message, { labels });
    const stop = new AbortController();
    const urgent = await tt.rank("Is this urgent?", inbox, { top: 5, signal: stop.signal });
    return { team, topics, urgent: urgent.length, first: urgent[0].record };`);
  // The generic arm says yes to every label; the deck's comment quotes a real backend.
  assert.deepEqual(value, {
    team: 'billing',
    topics: ['billing', 'shipping', 'urgent', 'praise'],
    urgent: 5,
    first: 'Where is my order?',
  });
});

// Each refusal happens before the crossing or before any send.
const REFUSALS = [
  ["tt.score('How urgent?', 'x')", 'score takes its levels in the last object: { levels }'],
  ["tt.choose('Which team?', 'x')", 'choose takes its options in the last object: { options }'],
  ["tt.tag('Which topics?', 'x')", 'tag takes its labels in the last object: { labels }'],
  ["tt.decide('Refund?', 'x', { urgent: true })", 'options.urgent is not a decide key'],
  ["tt.choose('Which team?', 'x', { labels: ['a'] })", 'options.labels is not a choose key'],
  ["tt.rank('Urgent?', ['a', 'b'], { top: 0 })", 'options.top is a positive whole number'],
  ["tt.find('Which?', ['a', 'b'], { none: 'yes' })", 'options.none is true or false'],
  [
    "tt.score({ score: 'How urgent?', levels: ['low', 'high'] }, 'x', { levels: ['a', 'b'] })",
    'score: a question value carries its own levels; the last object holds call options and top',
  ],
  ["tt.rank({ decide: 'Refund?', threshold: 0.9 }, ['a', 'b'])", 'rank takes a decide question with no threshold'],
  ["tt.filter(tt.question({ decide: 'Refund?', threshold: [0.2, 0.8] }), ['a'])", 'filter does not take a banded question'],
  ["tt.choose(tt.question({ score: 'How urgent?', levels: ['low', 'high'] }), 'x')", 'choose does not take a score question'],
  ["tt.decide_many('Refund?', ['a', 'b', '\\uD800c'])", 'record 2 holds a lone surrogate'],
  ["tt.decide_many('Refund?', ['a', 7])", 'record 1 is text'],
  ["tt.decide('   ', 'x')", "the question file's `decide`: a question is text, not white space"],
  ["tt.question({ decide: 'Refund?' })()", 'a question value is asked, not called'],
];

test('usage refusals name their fault and send nothing', async (t) => {
  const backend = await startBackend(t);
  const { value } = await ask(backend, `
    const out = [];
    for (const call of ${JSON.stringify(REFUSALS.map(([call]) => call))}) {
      try { await eval(call); out.push(null); } catch (error) { out.push([error.name, error.kind, error.retryable, error.message]); }
    }
    return out;`);
  assert.deepEqual(
    value,
    REFUSALS.map(([, message]) => ['ThinkThenError', 'usage', false, message]),
  );
  assert.equal(await backend.count(), 0);
});

test('a refused request and a dead address are backend failures that do not retry', async (t) => {
  const backend = await startBackend(t);
  const refused = await ask(backend, "return tt.decide('Refund?', 'x');", { arm: 'arm/refuse' });
  assert.deepEqual(refused.error.kind, 'backend');
  assert.equal(refused.error.retryable, false);
  assert.match(refused.error.message, /^the backend answered with status 422/);
  const dead = await ask(backend, "return tt.decide('Refund?', 'x');", { env: { THINKTHEN_BASE_URL: 'http://127.0.0.1:1/v1' } });
  assert.deepEqual([dead.error.kind, dead.error.retryable, dead.error.message], ['backend', false, 'the backend refused the connection']);
});

test('no rejection repeats the key or the address credentials', async (t) => {
  const backend = await startBackend(t);
  const secret = `http://user:pw-secret@127.0.0.1:${backend.port}/arm/refuse/v1`;
  const { value } = await ask(backend, `
    const out = [];
    const calls = [
      () => tt.decide('Refund?', 'x'),
      () => tt.decide_many('Refund?', ['x', 'y']),
      () => tt.annotate('/no/such/set.json', ['x']),
      () => new tt.Engine({ baseUrl: ${JSON.stringify(secret)} }).decide('Refund?', 'x'),
      () => new tt.Engine({ nope: 1 }),
      () => tt.decide('Refund?', 'x', { deadlineMs: 0 }),
    ];
    for (const call of calls) {
      try { await call(); out.push('resolved'); } catch (error) { out.push(error.message + ' ' + String(error) + ' ' + JSON.stringify(error)); }
    }
    return out;`, { arm: 'arm/refuse' });
  assert.equal(value.length, 6);
  for (const text of value) {
    assert.notEqual(text, 'resolved');
    assert.ok(!text.includes(FAKE_KEY) && !text.includes('pw-secret'), text);
  }
});

test('the helper refuses a backend that is not loopback', async (t) => {
  const backend = await startBackend(t);
  await assert.rejects(
    ask(backend, 'return 1;', { env: { THINKTHEN_BASE_URL: 'http://localhost.example/v1' } }),
    /refusing a backend that is not loopback/,
  );
});
