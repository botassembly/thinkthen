// Pure contract fixtures qualify carriers only, never native parity.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { inspect } from 'node:util';
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
const { decode, AnswerId } = require('../_complete.js');
const builders = require('../_requests.js');
const fixture = JSON.parse(readFileSync(new URL('../../python/tests/fixtures/complete.json', import.meta.url), 'utf8'));

test('all ten private complete carriers preserve probabilities, locations and failures', () => {
  for (const row of fixture.results) {
    const result = decode(row.type, row.result);
    assert.deepEqual(JSON.parse(JSON.stringify(result)), row.result);
    assert.equal(result.answer_id, 'a'.repeat(64));
    assert.equal(result.meta.requests.length, 2);
    assert.equal(Object.isFrozen(result), true);
    assert.equal(inspect(result).includes('reported'), false);
    assert.equal(decode(row.type, {...row.result,index:0}).index, 0);
    assert.throws(()=>decode(row.type,{...row.result,index:-1}),/invalid/);
  }
  const candidate=decode('FindCandidate',{index:0,input:false,probability:.25,source:{file:'é.txt',first_line:2,last_line:2}});
  assert.equal(candidate.input,false);assert.equal(candidate.source.first_line,2);assert.equal(candidate.probability,.25);
  const decide = decode('DecideResult', fixture.results[0].result);
  assert.equal(decide.value, false);
  assert.equal(decide.question.true, null);
  const choose = decode('ChooseResult', fixture.results[1].result);
  assert.equal(choose.value, null);
  assert.equal(choose.answer.confidence, 0);
  assert.deepEqual(Object.keys(choose.answer.probabilities), ['b', 'a']);
  assert.deepEqual(choose.position.images, ['red.png', 'blue.png', 'red.png']);
  assert.equal(Object.hasOwn(choose.position, 'first'), false);
  const annotation = decode('AnnotateResult', fixture.results[7].result);
  assert.equal(annotation.answers.ok.value, null);
  assert.equal(annotation.answers.bad.failure.cause, 'missing_answer');
  const recognition = decode('RecognizeResult', fixture.results[8].result);
  assert.equal(recognition.answer.names[0].edges, null);
  assert.equal(recognition.value.entities[0].end, 2);
  const relation = decode('RelateResult', fixture.results[9].result);
  assert.equal(relation.value[0].source.file, 'é.txt');
  assert.equal(relation.value[0].target.file, 'é.txt');
  assert.equal(relation.answer.questions[1].target, null);
  assert.equal(relation.answer.questions[1].failure.cause, 'missing_answer');
  const empty = decode('RelateResult', fixture.empty.result);
  assert.equal(empty.meta.origin, null);
  assert.equal(empty.meta.cached, false);
  assert.equal(Object.hasOwn(empty.meta, 'answered_by'), false);
});

test('facts and terminal errors retain reported values and reject fabricated fields', () => {
  const facts = decode('Facts', fixture.facts);
  assert.equal(facts.call_id, 'b'.repeat(64));
  assert.equal(facts.estimated_cost_usd, '0.000000');
  assert.equal(Object.hasOwn(facts, 'command_ms'), false);
  for (const error of fixture.errors) assert.equal(Object.hasOwn(decode('CallError', error), 'facts'), false);
  const failed = decode('CallError', fixture.started_error);
  assert.equal(failed.attempts[0].sdk_request_id, 'c'.repeat(64));
  assert.equal(failed.attempts[0].server_ms, 0);
  for (const id of ['A'.repeat(64), 'a'.repeat(63), 0]) assert.throws(() => AnswerId(id), /invalid complete result/);
  for (const change of [
    { schema: 'thinkthen.result/1' }, { answer_id: 'a'.repeat(63) }, { value: 0 },
    { answer: { kind: 'yes_no', probability: true } }, { position: { file: 'x', first: 4 } },
  ]) assert.throws(() => decode('DecideResult', { ...fixture.results[0].result, ...change },true), /invalid complete result/);
  for (const change of [{ origin: 'proxy' }, { cached: false }, { answered_by: 'invented' }, { observations: [] }, { failed_questions: 1 }]) {
    assert.throws(() => decode('Meta', { ...fixture.results[0].result.meta, ...change }), /invalid complete result/);
  }
});

test('complete decisions retain authored readings and withhold their inspected content', () => {
  for (const value of ['private reading', { private: [false, null] }, ['private reading', 2], null]) {
    const original = { ...fixture.results[0].result, value, question: { verb: 'decide', text: 'Q', false: value } };
    const result = decode('DecideResult', original);
    assert.deepEqual(JSON.parse(JSON.stringify(result)), original);
    assert.equal(inspect(result).includes('private'), false);
    if (value !== null && typeof value === 'object') assert.equal(Object.isFrozen(result.value), true);
  }
});

test('named request builders retain JSON occurrences and owned ordered images', () => {
  const inputs = { records: [false, null, { id: 1 }, { id: 1 }], context: { context: [] } };
  const specs = {
    decide: { decide: ['Q', { active: false }], false: null, on: ['/body'] },
    choose: { choose: 'Q', options: { b: null, a: { nested: [false] } } },
    tag: { tag: 'Q', labels: ['a'] }, score: { score: 'Q', levels: ['low', 'high'] },
    filter: { decide: 'Q' }, rank: { score: 'Q', levels: ['low', 'high'] }, find: { find: 'Q', none: true },
    annotate: { version: 1, questions: { a: { decide: 'Q', false: null } } },
    recognize: { version: 1, recognize: { kinds: { person: { nested: [false] } } } },
    relate: { version: 1, relate: { relations: [{ name: 'knows', source: '*', target: '*' }] } },
  };
  for (const [verb, spec] of Object.entries(specs)) {
    const request = builders[verb](spec, inputs);
    assert.deepEqual(request.question, spec);
    assert.deepEqual(request.input, inputs);
  }
  const data = new Uint8Array([1, 2, 3]);
  const image = { data, name: 'red.png' };
  const source = { images: [image, { data: new Uint8Array([4]) }, image], text: null };
  const request = builders.decide(specs.decide, source);
  data[0] = 99;
  request.input.images[0].data[0] = 88;
  assert.deepEqual([...request.input.images[0].data], [1, 2, 3]);
  assert.deepEqual([...request.input.images[2].data], [1, 2, 3]);
  assert.equal(request.input.text, null);
  for (const verb of ['tag', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate']) {
    assert.throws(() => builders[verb](specs[verb], source), /text-only/);
    assert.throws(() => builders[verb](specs[verb], { paths: ['red.png'], unit: 'file', media: 'image' }), /text-only/);
  }
  assert.throws(() => builders.find(specs.score, { units: ['a', 'b'] }), /wrong question kind/);
  assert.throws(() => builders.decide(specs.decide, { paths: ['x'], unit: 'window' }), /window/);
  assert.deepEqual(builders.rank({ path: 'question.json' }, inputs).question, { path: 'question.json' });
});

test('private inputs retain the shared valid question grammar and refuse wrong field types', () => {
  const corpus = JSON.parse(readFileSync(new URL('../../../specification/fixtures/question-file/corpus.json', import.meta.url), 'utf8'));
  for (const row of corpus.cases.filter(row => row.valid)) {
    const kind = row.verb === 'relate' ? 'RelationSpec' : row.verb[0].toUpperCase() + row.verb.slice(1) + 'Spec';
    assert.deepEqual(decode(kind, row.file), row.file);
  }
  for (const [kind, body] of [
    ['DecideSpec', { decide: false }], ['ChooseSpec', { choose: 'Q', options: { a: true } }],
    ['ChooseSpec', { choose: 'Q', options: ['a','b'], threshold: 0 }],
    ['QuestionSet', { version: 1, questions: { ready: { decide: 'Q', profile: 'other' } } }],
  ]) assert.throws(() => decode(kind, body), /invalid complete result/);
  assert.deepEqual(decode('DecideSpec', { decide: {}, on: '/body', false: null }), { decide: {}, on: '/body', false: null });
});

test('rank members retain ordered typed judgments and refuse recursive children', () => {
  const judgment=fixture.results[0].result;
  const child=Object.fromEntries(['schema','answer_id','question','answer','meta'].map(k=>[k,structuredClone(judgment[k])]));
  Object.assign(child,{value:3,threshold:null});child.meta.usage={input_tokens:2};
  const parent={...structuredClone(fixture.results[5].result),question:child.question,answer:child.answer,question_name:'saved',members:[{name:'saved',result:child}]};
  const ranked=decode('RankResult',parent);
  assert.equal(ranked.members[0].result.value,3);assert.equal(ranked.members[0].result.meta.usage.output_tokens,undefined);
  assert.deepEqual(JSON.parse(JSON.stringify(ranked)),parent);
  for(const change of [{value:0},{input:false},{members:[]},{question:{verb:'score',text:'Q',levels:['x']}}]) {
    const invalid=structuredClone(parent);Object.assign(invalid.members[0].result,change);
    assert.throws(()=>decode('RankResult',invalid),/invalid/);
  }
  for(const members of [[],null,[{name:'saved'}]]) assert.throws(()=>decode('RankResult',{...parent,members}),/invalid/);
  assert.throws(()=>decode('Usage',{}),/invalid/);
});

test('current facts preserve request measurements and persistence', () => {
  for (const raw of fixture.observed_facts) {
    const facts = decode('Facts', raw);
    assert.equal(facts.largest_request_bytes, raw.largest_request_bytes);
    assert.equal(facts.largest_request_estimated_input_tokens, raw.largest_request_estimated_input_tokens);
    assert.equal(facts.token_estimate_method, raw.token_estimate_method);
    assert.deepEqual(facts.usage_persistence, raw.usage_persistence);
    assert.deepEqual(JSON.parse(JSON.stringify(facts)), raw);
  }
});

test('output extensions round trip owned JSON while requests and alternative identities stay strict', () => {
  const extension = {null:null,false:false,zero:0,array:[null,false,0],object:JSON.parse('{"nested":[],"__proto__":{"inert":true}}')};
  for (const fixtureRow of [...fixture.results,...fixture.native_members]) {
    const raw = structuredClone(fixtureRow.result);
    raw.proxy = null; raw.extension = structuredClone(extension); raw.meta.extension = structuredClone(extension);
    if (fixtureRow.type === 'AnnotateResult') for (const entry of Object.values(raw.answers)) entry.extension = structuredClone(extension);
    if (fixtureRow.type === 'RelateResult') for (const entry of raw.answer.questions) entry.extension = structuredClone(extension);
    const result = decode(fixtureRow.type, raw, true);
    assert.deepEqual(JSON.parse(JSON.stringify(result)), raw);
    raw.extension.array.push('mutated');
    assert.deepEqual(result.extension, extension);
    assert.equal(inspect(result).includes('extension'), false);
  }
  for (const [kind, entry] of [['AnnotationEntry',fixture.results[7].result.answers.ok],['RelationEntry',fixture.results[9].result.answer.questions[0]]]) {
    assert.throws(() => decode(kind,{...entry,failure_id:'b'.repeat(64)},true),/invalid/);
  }
  assert.throws(() => decode('DecideSpec',{decide:'Q',proxy:null}),/invalid/);
  assert.throws(() => decode('DecideResult',{...fixture.results[0].result,answer:{kind:'new',probability:.5}},true),/invalid/);
});
