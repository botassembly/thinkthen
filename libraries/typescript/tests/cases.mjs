// Run each shared case in conformance/cases.json through this package, on the
// backend's case arm, and report pass, fail, or not run with a reason. A
// child process with the fake key runs it; conformance.test.mjs asserts.
import { createHash } from 'node:crypto';
import { writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { isDeepStrictEqual } from 'node:util';

const CANONICAL = 'https://api.typesafe.ai/v1/systemone';

/** Cases this surface cannot express, each with its reason. */
const NOT_RUN = {
  '18-annotate-two-groups': 'its recording holds one request per group, and ADR 0111 section 5 packs a record\'s groups into one',
  '25-defect-fault': 'no input makes the engine panic; the Rust unit test covers the binding guard',
};

const digest = (url, request) => createHash('sha256').update(`systemone\n${url}\n${request}`).digest('hex');

// The end of the JSON value that starts at `at` in compact `text`.
function valueEnd(text, at) {
  let depth = 0;
  for (let i = at; i < text.length; i += 1) {
    const ch = text[i];
    if (ch === '"') {
      for (i += 1; text[i] !== '"'; i += 1) if (text[i] === '\\') i += 1;
      if (depth === 0) return i + 1;
    } else if (ch === '{' || ch === '[') depth += 1;
    else if (ch === '}' || ch === ']') {
      depth -= 1;
      if (depth === 0) return i + 1;
    } else if (depth === 0 && (ch === ',' || ch === '}')) return i;
  }
  return text.length;
}

// Each member of the compact JSON object at `at`, as its key and raw value
// bytes, so an object's key order survives where JSON.parse would sort it.
function rawMembers(text, at) {
  const members = [];
  let i = at + 1;
  while (text[i] !== '}') {
    const keyEnd = valueEnd(text, i);
    const key = JSON.parse(text.slice(i, keyEnd));
    const close = valueEnd(text, keyEnd + 1);
    members.push([key, text.slice(keyEnd + 1, close)]);
    i = text[close] === ',' ? close + 1 : close;
  }
  return members;
}

// Every question key of one request body, in wire order, by ADR 0111 section 2:
// the SHA-256 of the adapter, the URL, the model, the state and one question as
// the body carries them, joined by line feeds.
export function questionKeys(url, request) {
  const body = Object.fromEntries(rawMembers(request, 0));
  const head = ['systemone', url, body.model, body.state].join('\n');
  return rawMembers(body.questions, 0)
    .sort(([a], [b]) => Number(a.slice(1)) - Number(b.slice(1)))
    .map(([, question]) => createHash('sha256').update(`${head}\n${question}`).digest('hex'));
}

function same(what, actual, expected) {
  if (!isDeepStrictEqual(actual, expected)) {
    throw new Error(`${what}: ${JSON.stringify(actual)} is not ${JSON.stringify(expected)}`);
  }
}

// Scalar-value offsets to UTF-16 units, so the expectation reads as JavaScript's.
function units(text, held) {
  const widths = [...text].map((ch) => ch.length);
  const at = (point) => widths.slice(0, point).reduce((sum, width) => sum + width, 0);
  return { ...held, start: at(held.start), end: at(held.end), length: at(held.end) - at(held.start) };
}

/** Each fault case, as this surface raises its kind. */
function fault(tt, one, engine, folder) {
  const question = one.question;
  switch (one.id) {
    case '20-usage-fault': return engine.decide(question, 42);
    case '21-backend-fault': return engine.decide(question, 'any text');
    case '22-local-fault': {
      const file = join(folder, 'a-file');
      writeFileSync(file, 'not a folder');
      return new tt.Engine({ baseUrl: engine.base, cache: file }).decide(question, 'any text');
    }
    case '23-cancelled-fault': return engine.decide(question, 'any text', { signal: AbortSignal.abort() });
    case '24-deadline-fault': return engine.decide(question, 'any text', { deadlineMs: 0 });
    case '29-usage-json-text': return engine.decide(question, one.evidence);
    case '30-local-question-file': {
      const file = join(folder, 'question.json');
      writeFileSync(file, JSON.stringify(question));
      return engine.decide(tt.questionFile(file), one.evidence);
    }
    case '31-usage-rank-blank-question': return engine.rank(question.decide, one.evidence.split('\n').filter(Boolean));
    default: throw new Error(`no fault form for ${one.id}`);
  }
}

async function check(tt, one, origin, folder) {
  const base = `${origin}/${one.id === '21-backend-fault' ? 'arm/refuse' : `case/${one.id}`}/v1`;
  const engine = new tt.Engine({ baseUrl: base, cache: false });
  engine.base = base;
  if (one.expect.error) {
    let retryable;
    const kind = await Promise.resolve().then(() => fault(tt, one, engine, folder)).then(() => 'resolved', (error) => {
      retryable = error.retryable;
      return error.kind;
    });
    if (one.id === '30-local-question-file') {
      same('named-file retryable', retryable, false);
      same('named-file sends', engine.usage().requests_sent, 0);
    }
    return same('kind', kind, one.expect.error.kind);
  }
  const success = one.expect.success;
  const served = `${base}/systemone`;
  // Every row lists question keys, by ADR 0111.
  const renamed = new Map(one.exchanges.map(({ request }) => [digest(CANONICAL, request),
    questionKeys(served, request)]));
  const texts = one.exchanges.map((exchange) => exchange.evidence);
  const want = (at) => success.answers.find((answer) => answer.exchange === at);
  switch (success.kind) {
    case 'single': {
      same('value', (await engine[one.verb](one.question, texts[0])).value, success.answers[0].bare);
      const details = (await engine.details(one.question, texts[0])).value;
      const expected = success.answers[0].details;
      same('answer', details.answer, expected.answer);
      same('question_sha256', details.meta.question_sha256, expected.question_sha256);
      same('model', details.meta.model, expected.model);
      same('requests', details.meta.requests, expected.requests.flatMap((held) => renamed.get(held)));
      for (const name of ['usage', 'requests_sent', 'cached']) same(name, details.meta[name], expected[name]);
      same('url', details.meta.url, served);
      if (one.id === '01-decide-yes-captured') {
        const file = join(folder, 'captured-question.json');
        writeFileSync(file, JSON.stringify(one.question));
        const before = engine.usage().requests_sent;
        const loaded = await engine.details(tt.questionFile(file), texts[0]);
        same('named-file answer', loaded.value.answer, expected.answer);
        same('named-file digest', loaded.value.meta.requests, expected.requests.flatMap((held) => renamed.get(held)));
        same('named-file sends', engine.usage().requests_sent - before, 1);
      }
      if (success.counters) {
        const cached = new tt.Engine({ baseUrl: base, cache: join(folder, one.id) });
        for (let call = 0; call < success.counters.calls; call += 1) await cached.decide(one.question, texts[0]);
        const { requests_sent, cache_answers } = cached.usage();
        same('counters', { requests: requests_sent, cache_answers }, { requests: success.counters.requests, cache_answers: success.counters.cache_answers });
      }
      return undefined;
    }
    case 'decide_many':
      return same('values', (await engine.decide_many(one.question, texts, { batch: 1 })).value, texts.map((_, at) => want(at).bare));
    case 'filter':
      return same('kept', (await engine.filter(one.question, texts, { batch: 1 })).value, success.operation.indexes.map((at) => texts[at]));
    case 'rank': {
      const ranked = (await engine.rank(one.question, texts, { batch: 1 })).value;
      return same('ranking', ranked.map(({ index, probability }) => ({ index, probability })), success.operation.ranking);
    }
    case 'annotate': {
      const records = one.record ? [JSON.stringify(one.record)] : texts;
      const call = await engine.annotate(one.question_set, records, { batch: 1 });
      const rows = call.value;
      if (one.id === '17-annotate-partial' && !call.details.some((row) => row.member && row.failed)) {
        throw new Error('the failed annotate member has no distinct call detail');
      }
      const expected = records.map(() => ({}));
      for (const answer of success.answers) expected[one.record ? 0 : answer.exchange][answer.name] = answer.bare;
      // tt.failed reads the failure marker; null stays unresolved.
      const read = (row) => Object.fromEntries(Object.entries(row).map(([name, member]) => [name,
        tt.failed(member) ? `failed ${tt.failed(member).cause}` : member === null ? 'unresolved' : 'answered']));
      same('fields', rows.map(read), expected.map(read));
      return same('rows', rows, expected);
    }
    case 'find': {
      const { find, none, units: listed } = one.question;
      const found = (await engine.find(find, listed, { none })).value;
      const { selected, probabilities } = success.operation;
      const picked = probabilities.find((row) => row.index === selected);
      return same('found', found, selected === null ? null : { index: selected, unit: listed[selected], probability: picked.probability });
    }
    case 'recognize': {
      const { kinds, relations } = one.question.recognize;
      const asked = { kinds, threshold: one.question.threshold, relationThreshold: one.question.relation_threshold };
      const found = (await engine.recognize(one.text, relations ? { ...asked, relations } : asked)).value;
      const bare = success.answers[0].bare;
      const shaped = { entities: bare.entities.map((held) => units(one.text, held)) };
      if (bare.relations) shaped.relations = bare.relations.map((held) => ({ ...held, source: units(one.text, held.source), target: units(one.text, held.target) }));
      for (const held of found.entities) same('slice', one.text.slice(held.start, held.end), held.text);
      return same('recognized', found, shaped);
    }
    case 'relate': {
      const rules = one.question.relate.relations;
      const edges = (await engine.relate(one.entities, {
        relations: rules.map(({ name, source, target }) => `${name}=${source}:${target}`),
        either: rules.filter((rule) => rule.either).map((rule) => rule.name),
        threshold: one.question.threshold,
      })).value;
      return same('edges', edges, success.answers[0].bare);
    }
    default:
      throw new Error(`no runner for ${success.kind}`);
  }
}

/** Every case's outcome: `[id, 'pass' | 'fail' | 'not run', reason]`. */
export async function runCases(tt, cases, origin, folder) {
  const outcomes = [];
  for (const one of cases.cases) {
    if (NOT_RUN[one.id]) {
      outcomes.push([one.id, 'not run', NOT_RUN[one.id]]);
      continue;
    }
    try {
      await check(tt, one, origin, folder);
      outcomes.push([one.id, 'pass', '']);
    } catch (error) {
      outcomes.push([one.id, 'fail', error.message]);
    }
  }
  return outcomes;
}
