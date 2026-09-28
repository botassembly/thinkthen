// Every verb in its host shape, the last-object rule, the input checks, and
// the error kinds, each against its own loopback backend. The generic arm
// gives the first option, level, or yes 0.9 and shares the rest.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { createServer } from 'node:http';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { createRequire } from 'node:module';

import { FAKE_KEY, ask, startBackend } from './backend.mjs';
const recordingDigest = (base, body) => createHash('sha256').update(`systemone\n${base}/systemone\n${body}`).digest('hex');
const native = createRequire(import.meta.url)('../loader.js');

test('named recognition and relation plans retain source, model and bounded answers', async (t) => {
  const backend = await captured(t);
  const recognition = '{"version":1,"recognize":{"kinds":{"person":"A person"},"relations":[{"name":"knows","source":"person","target":"person","reads":"knows","either":false}]},"threshold":0.95,"relation_threshold":0.65,"model":"fixture-recognize-model","profile":"recognize-calibration"}';
  const relation = '{"version":1,"relate":{"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}]},"threshold":0.95,"model":"fixture-relate-model","profile":"relate-calibration"}';
  const first = join(backend.folder, 'recognize.json');
  const second = join(backend.folder, 'relate.json');
  writeFileSync(first, recognition); writeFileSync(second, relation);
  assert.equal(JSON.parse(native.planFile(first, 'recognize')).ok, recognition, 'validated recognition source, including saved profile');
  assert.equal(JSON.parse(native.planFile(second, 'relate')).ok, relation, 'validated relation source, including saved profile');
  const { value, error } = await ask(backend, `
    const engine = new tt.Engine({ model: 'engine-default-0252', cache: false });
    const recognized = await engine.recognize('Ana Bob', { file: ${JSON.stringify(first)} });
    const related = await engine.relate([['Ana', 'person'], ['Acme', 'organization']], { file: ${JSON.stringify(second)} });
    const inlineNames = await engine.recognize('Ana Bob', { kinds: ['person'] });
    const inlineEdges = await engine.relate([['Ana', 'person'], ['Acme', 'organization']], { relations: ['works_for=person:organization'] });
    return { recognized, related, inlineNames: inlineNames.value.entities, inlineEdges: inlineEdges.value };`);
  assert.equal(error, undefined, JSON.stringify(error));
  assert.deepEqual(value.recognized.value, { entities: [], relations: [] }, '0.95 file cut drops the generic name');
  assert.deepEqual(value.related.value, [], '0.95 file cut drops the generic 0.9 edge');
  assert.equal(value.inlineNames.length, 1, 'the retained default cut admits the same name');
  assert.equal(value.inlineEdges.length, 1, 'the retained default cut admits the same edge');
  assert.equal(backend.bodies.length, 6, 'the two named plans send three requests before inline calls');
  assert.deepEqual(backend.bodies.slice(0, 3).map((body) => createHash('sha256').update(body).digest('hex')), [
    '500ade25b0ef5826b8823bca242b4540dd1d8d0a061efc37457011e3f33aae4b',
    '3745ebe527887293996e6cc7d1bc12e94b7fec31f2ed42ef6d4e3141da445f28',
    '526b75c58f1c5921c7b313c2c059c6622d8926ce167c129d9ee9b36fc259480e',
  ], 'independently pinned recognition-stage and relation request bytes');
  assert.deepEqual(backend.bodies.slice(0, 3).map((body) => JSON.parse(body).model),
    ['fixture-recognize-model', 'fixture-recognize-model', 'fixture-relate-model']);
  assert.deepEqual(value.recognized.details.map((detail) => detail.model), ['fixture-recognize-model', 'fixture-recognize-model', 'fixture-recognize-model']);
  assert.equal(value.related.details[0].question_sha256, '6a7c109d0897d85c579046930f83ec063b51b526e1155b42c092b627a12e6fda',
    'the logical pair digest is stable, not a whole-plan profile digest');
});

test('named plans refuse unsafe sources and mixed inline options before sending', async (t) => {
  const backend = await captured(t);
  const marker = 'SYNTHETIC_PRIVATE_MARKER_0252';
  const paths = ['missing', 'unknown', 'wrong', 'large', 'utf8', 'malformed', 'relation-unknown'].map((name) => join(backend.folder, `${name}.json`));
  writeFileSync(paths[1], JSON.stringify({ version: 1, recognize: { kinds: { person: 'Person' } }, [marker]: 1 }));
  writeFileSync(paths[2], '{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person","reads":"knows"}]}}');
  writeFileSync(paths[3], Buffer.alloc(1_048_577, 120));
  writeFileSync(paths[4], Buffer.from([0xff]));
  writeFileSync(paths[5], '{"version":');
  writeFileSync(paths[6], JSON.stringify({ version: 1, relate: { relations: [{ name: 'knows', source: 'person', target: 'person' }] }, [marker]: 1 }));
  const { value, error } = await ask(backend, `
    const paths = ${JSON.stringify(paths)};
    const seen = [];
    for (const file of paths) {
      try { await tt.recognize('Ana', { file }); seen.push('accepted'); }
      catch (failure) { seen.push([failure.kind, failure.retryable, failure.message.includes(file) || failure.message.includes(${JSON.stringify(marker)})]); }
    }
    try { await tt.relate([['Ana', 'person']], { file: paths[6] }); seen.push('accepted'); }
    catch (failure) { seen.push([failure.kind, failure.retryable, failure.message.includes(paths[6]) || failure.message.includes(${JSON.stringify(marker)})]); }
    for (const call of [
      () => tt.recognize('Ana', { file: paths[2], kinds: ['person'] }),
      () => tt.relate([['Ana', 'person']], { file: paths[2], relations: ['knows=person:person'] }),
      () => tt.recognize('Ana', { file: '\\ud800' }),
      () => tt.relate([['Ana', 'person']], { file: 4 }),
    ]) {
      try { await call(); seen.push('accepted'); }
      catch (failure) { seen.push([failure.kind, failure.retryable]); }
    }
    return seen;`);
  assert.equal(error, undefined, JSON.stringify(error));
  assert.deepEqual(value, [['local', false, false], ['local', false, false], ['local', false, false],
    ['local', false, false], ['local', false, false], ['local', false, false], ['local', false, false],
    ['local', false, false], ['usage', false], ['usage', false],
    ['usage', false], ['usage', false]]);
  assert.deepEqual(backend.bodies, []);
});

test('a named rich question preserves source order and its captured request', async (t) => {
  const backend = await captured(t);
  const source = '{"choose":"Which?","options":{"2":["nested",{"flag":true}],"1":{"what":"first"},"other":null}}';
  const file = join(backend.folder, 'rich-question.json');
  writeFileSync(file, source);
  const { value, error } = await ask(backend, `
    const loaded = tt.questionFile(${JSON.stringify(file)});
    const result = await tt.choose(loaded, 'first');
    return { source: loaded.__spec, answer: result.value, request: result.details[0].requests[0] };`);
  assert.equal(error, undefined, JSON.stringify(error));
  assert.equal(value.source, source);
  const body = '{"state":"first","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"Which?","criteria":{"2":["nested",{"flag":true}],"1":{"what":"first"},"other":null}}}}';
  assert.deepEqual(backend.bodies, [body]);
  assert.equal(value.request, recordingDigest(backend.base(), body));
});

test('named question files refuse bounded local failures before any send', async (t) => {
  const backend = await captured(t);
  const marker = 'SYNTHETIC_PRIVATE_MARKER_0244';
  const files = ['missing.json', 'blank.json', 'large.json', 'utf8.json', 'unknown.json', 'choose.json'].map((name) => join(backend.folder, name));
  writeFileSync(files[1], '{"decide":"   "}');
  writeFileSync(files[2], Buffer.alloc(1_048_577, 120));
  writeFileSync(files[3], Buffer.from([0xff]));
  writeFileSync(files[4], JSON.stringify({ decide: 'Question?', [marker]: 1 }));
  writeFileSync(files[5], '{"choose":"Which?","options":["a","b"]}');
  const { value, error } = await ask(backend, `
    const files = ${JSON.stringify(files)};
    const observed = [];
    for (const file of files.slice(0, 5)) {
      try { tt.questionFile(file); observed.push('accepted'); }
      catch (failure) { observed.push([failure.kind, failure.retryable, failure.message.includes(file) || failure.message.includes(${JSON.stringify(marker)})]); }
    }
    try { tt.questionFile(42); observed.push('accepted'); }
    catch (failure) { observed.push([failure.kind, failure.retryable]); }
    try { await tt.decide(tt.questionFile(files[5]), 'text'); observed.push('accepted'); }
    catch (failure) { observed.push([failure.kind, failure.retryable]); }
    try { await tt.decide(tt.question({ decide: '   ' }), 'text'); observed.push('accepted'); }
    catch (failure) { observed.push([failure.kind, failure.retryable]); }
    return observed;`);
  assert.equal(error, undefined, JSON.stringify(error));
  assert.deepEqual(value, [
    ['local', false, false], ['local', false, false], ['local', false, false], ['local', false, false], ['local', false, false],
    ['usage', false], ['usage', false], ['usage', false],
  ]);
  assert.deepEqual(backend.bodies, []);
});

// Capture the listener's exact body bytes while answering by the generic rule.
async function captured(t) {
  const bodies = [];
  const server = createServer(async (request, reply) => {
    const chunks = [];
    for await (const chunk of request) chunks.push(chunk);
    const body = Buffer.concat(chunks).toString('utf8');
    bodies.push(body);
    const asked = JSON.parse(body);
    const answers = Object.fromEntries(Object.entries(asked.questions).map(([name, question]) => {
      if (question.type === 'noul') return [name, { type: 'noul', noul: 0.9 }];
      const keys = Array.isArray(question.criteria) ? question.criteria.map((_, at) => String(at)) : Object.keys(question.criteria);
      const probabilities = Object.fromEntries(keys.map((key, at) => [key, at === 0 ? 0.9 : 0.1 / (keys.length - 1)]));
      return [name, { type: question.type, probabilities }];
    }));
    reply.writeHead(200, { 'content-type': 'application/json' });
    reply.end(JSON.stringify({ model: asked.model, answers, usage: { input_tokens: 3, output_tokens: 2 } }));
  });
  await new Promise((done) => server.listen(0, '127.0.0.1', done));
  const folder = mkdtempSync(join(tmpdir(), 'thinkthen-ts-capture-'));
  t.after(() => { server.closeAllConnections(); server.close(); rmSync(folder, { recursive: true, force: true }); });
  return { bodies, port: server.address().port, folder, base: () => `http://127.0.0.1:${server.address().port}/v1` };
}

test('packed and batch-one calls expose exact bodies, ordered details, and final facts', async (t) => {
  const backend = await captured(t);
  const { value, error } = await ask(backend, `
    const engine = new tt.Engine({ cache: false });
    const records = ['first', 'first', 'third'];
    const packed = await engine.decide_many('Refund?', records);
    const frozen = [Object.isFrozen(packed), Object.isFrozen(packed.facts), Object.isFrozen(packed.details), Object.isFrozen(packed.details[0]), Object.isFrozen(packed.details[0].requests)];
    const singles = await engine.decide_many('Refund?', records, { batch: 1 });
    const contextual = await engine.decide_many('Refund?', records, { context: 'Shared note.' });
    const ranked = await engine.rank('Refund?', records, { top: 1 });
    return { packed, singles, contextual, ranked, frozen };`);
  assert.equal(error, undefined, JSON.stringify({ error, bodies: backend.bodies }));
  assert.deepEqual(value.packed.value, [true, true, true]);
  assert.equal(value.packed.facts.records, 3);
  assert.equal(value.packed.facts.requests_sent, 1);
  assert.equal(value.packed.facts.input_tokens, 3);
  assert.deepEqual(value.frozen, [true, true, true, true, true]);
  assert.equal(value.packed.details.length, 3);
  assert.equal(value.singles.facts.requests_sent, 3);
  assert.deepEqual(value.singles.value, value.packed.value);
  assert.equal(value.contextual.facts.requests_sent, 1);
  assert.equal(value.ranked.value.length, 1);
  assert.equal(value.ranked.facts.records, 3);
  assert.equal(value.ranked.details.length, 3);
  assert.equal(backend.bodies.length, 6);
  assert.equal(Object.keys(JSON.parse(backend.bodies[0]).questions).length, 2, 'the duplicate record shares one packed question');
  for (const body of backend.bodies.slice(1, 4)) assert.equal(Object.keys(JSON.parse(body).questions).length, 1);
  assert.notEqual(backend.bodies[0], backend.bodies[4]);
  const digest = recordingDigest(backend.base(), backend.bodies[0]);
  assert.deepEqual(value.packed.details.map((row) => row.requests), [[digest], [digest], [digest]]);
  assert.deepEqual(value.packed.details.map((row) => row.index), [0, 1, 2]);
});

test('runtime-label many calls preserve ordered descriptions and bare versus null score levels', async (t) => {
  const backend = await captured(t);
  const { value, error } = await ask(backend, `
    const records = ['first', 'second'];
    const choices = await tt.choose_many('Which?', records, { options: { 2: ['nested', { flag: true }], 1: { what: 'first' }, other: null } });
    const scoresBare = await tt.score_many('How?', records, { levels: ['low', 'high'] });
    const scoresNull = await tt.score_many('How?', records, { levels: { low: null, high: 'High.' } });
    const tags = await tt.tag_many('Which?', records, { labels: { red: { nested: [1, false] }, blue: ['list', null] } });
    const meaning = await tt.decide(tt.question({ decide: 'Refund?', true: null, false: { nested: ['no', true] } }), 'first');
    const nullChoices = await tt.choose_many({ choose: 'Which?', options: ['a', 'b'], threshold: 0.95 }, records);
    const emptyTags = await tt.tag_many({ tag: 'Which?', labels: ['red', 'blue'], threshold: 0.95 }, records);
    const recognized = await tt.recognize('Maria', { kinds: { person: { what: 'A person', other: [true, 2] } } });
    return { choices, scoresBare, scoresNull, tags, meaning, nullChoices, emptyTags, recognized };`);
  assert.equal(error, undefined, JSON.stringify({ error, bodies: backend.bodies }));
  assert.deepEqual(value.choices.value, ['1', '1']);
  assert.deepEqual(value.scoresBare.value, [0.1, 0.1]);
  assert.deepEqual(value.scoresNull.value, [0.1, 0.1]);
  assert.deepEqual(value.tags.value, [['red', 'blue'], ['red', 'blue']]);
  assert.equal(value.meaning.value, true);
  assert.deepEqual(value.nullChoices.value, [null, null]);
  assert.deepEqual(value.emptyTags.value, [[], []]);
  assert.equal(value.choices.facts.records, 2);
  assert.equal(value.choices.facts.requests_sent, 1);
  const [choices, bare, described, tags, meaning] = backend.bodies.map((body) => JSON.parse(body));
  assert.deepEqual(Object.keys(Object.values(choices.questions)[0].criteria), ['1', '2', 'other']);
  assert.deepEqual(Object.values(choices.questions)[0].criteria['2'], ['nested', { flag: true }]);
  assert.ok(Array.isArray(Object.values(bare.questions)[0].criteria));
  assert.deepEqual(Object.values(described.questions)[0].criteria, [{}, 'High.']);
  assert.match(backend.bodies[3], /nested/);
  assert.equal(Object.values(meaning.questions)[0].criteria.true, null);
  assert.deepEqual(Object.values(meaning.questions)[0].criteria.false, { nested: ['no', true] });
  assert.deepEqual(value.choices.details.map((row) => row.requests), [[recordingDigest(backend.base(), backend.bodies[0])], [recordingDigest(backend.base(), backend.bodies[0])]]);
  assert.equal(Object.values(JSON.parse(backend.bodies[7]).questions)[0].type, 'choice');
  assert.ok(value.recognized.details.some((row) => row.requests.includes(recordingDigest(backend.base(), backend.bodies[7]))));
});

test('each verb resolves its host shape, on the module and on an engine', async (t) => {
  const backend = await startBackend(t);
  const { value } = await ask(backend, `
    const band = tt.question({ decide: 'Does it ask for a refund?', threshold: [0.2, 0.8] });
    const shapes = async (on) => (await Promise.all([
      on.decide('Does it ask for a refund?', 'I want a refund'),
      on.decide(band, 'I want a refund'),
      on.decide_many(band, ['one', 'two'], { batch: 1 }),
      on.choose('Which team?', 'text', { options: ['billing', 'other'] }),
      on.score('How urgent?', 'text', { levels: ['low', 'mid', 'high'] }),
      on.tag({ tag: 'Which topics?', labels: ['billing', 'urgent'] }, 'text'),
      on.filter('Is it a complaint?', ['one', 'two'], { batch: 1 }),
      on.rank('Is it urgent?', ['one', 'two', 'three'], { top: 2, batch: 1 }),
      on.find('Which line answers?', ['one', 'two']),
      on.annotate({ version: 1, questions: { refund: { decide: 'Refund?' }, team: { choose: 'Team?', options: ['a', 'b'] } } }, ['one'], { batch: 1 }),
    ])).map((call) => call.value);
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

// ADR 0056: a found name carries text in place of name, and relate reads it as the name.
test('relate reads what recognize found', async (t) => {
  const backend = await startBackend(t);
  const { value } = await ask(backend, `
    const relations = ['knows=person:person'];
    const found = (await tt.recognize('Maria Chen arrived.', { kinds: ['person'] })).value.entities;
    const forms = [found, found.map((one) => ({ name: one.text, text: 'not this', kind: one.kind })),
      [['Maria Chen', 'person'], ['arrived.', 'person']]];
    const edges = await Promise.all(forms.map((form) => tt.relate(form, { relations })));
    return edges.map((each) => each.value.map((edge) => [edge.source.name, edge.target.name]));`);
  const both = [['Maria Chen', 'arrived.'], ['arrived.', 'Maria Chen']];
  assert.deepEqual(value, [both, both, both]);
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
    return { team: team.value, topics: topics.value, urgent: urgent.value.length, first: urgent.value[0].record };`);
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
  ["tt.decide('Refund?', 'x', { batch: null })", 'options.batch is not a decide key'],
  ["tt.choose('Which?', 'x', { options: ['a'], context: null })", 'options.context is not a choose key'],
  ["tt.score('How?', 'x', { levels: ['a'], batch: 1 })", 'options.batch is not a score key'],
  ["tt.tag('Which?', 'x', { labels: ['a'], context: 'note' })", 'options.context is not a tag key'],
  ["tt.details('Refund?', 'x', { batch: 'max' })", 'options.batch is not a details key'],
  ["tt.find('Which?', ['a'], { batch: 1 })", 'options.batch is not a find key'],
  ["tt.recognize('x', { context: 'note' })", 'options.context is not a recognize key'],
  ["tt.relate([], { batch: 1 })", 'options.batch is not a relate key'],
  ["tt.annotate({ version: 1, questions: { a: { decide: 'Refund?' } } }, ['x'], { context: null })", 'options.context is not a annotate key'],
  ["tt.decide_many('Refund?', ['x'], { batch: null })", 'options.batch is max or a positive whole number'],
  ["tt.decide_many('Refund?', ['x'], { batch: undefined })", 'options.batch is max or a positive whole number'],
  ["tt.filter('Refund?', ['x'], { context: null })", 'options.context is nonblank text'],
  ["tt.filter('Refund?', ['x'], { context: undefined })", 'options.context is nonblank text'],
  ["new tt.Engine({ batch: null })", 'options.batch is max or a positive whole number'],
  ["tt.question({ decide: 'Refund?', true: { nested: undefined } })", 'the question is JSON'],
  ["tt.question({ decide: 'Refund?', true: [undefined] })", 'the question is JSON'],
  ["tt.question({ decide: 'Refund?', true: { nested: 1n } })", 'the question is JSON'],
  ["tt.question({ decide: 'Refund?', true: new Map([['x', 'y']]) })", 'the question is JSON'],
  ["tt.question({ decide: 'Refund?', true: (() => { const x = {}; x.self = x; return x; })() })", 'the question is JSON without a cycle'],
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
  const unreported = await ask(backend, "return (await tt.decide('Refund?', 'plain')).facts;");
  assert.equal(unreported.value.input_tokens, undefined, 'missing provider usage stays absent');
  assert.equal(unreported.value.output_tokens, undefined);
  const refused = await ask(backend, "return tt.decide('Refund?', 'x');", { arm: 'arm/refuse' });
  assert.deepEqual(refused.error.kind, 'backend');
  assert.equal(refused.error.retryable, false);
  assert.match(refused.error.message, /^the backend answered with status 422/);
  assert.equal(refused.error.facts.requests_sent, 1);
  assert.equal(refused.error.facts.records, 0);
  assert.deepEqual(refused.error.details, []);
  const dead = await ask(backend, "return tt.decide('Refund?', 'x');", { env: { THINKTHEN_BASE_URL: 'http://127.0.0.1:1/v1' } });
  assert.deepEqual([dead.error.kind, dead.error.retryable, dead.error.message], ['backend', false, 'the backend refused the connection']);
  assert.equal(dead.error.facts.requests_sent, 1);
  const unstarted = await ask(backend, "return tt.decide('   ', 'x');");
  assert.equal(unstarted.error.kind, 'usage');
  assert.equal(unstarted.error.facts, undefined, 'a worker start does not imply account start');
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
