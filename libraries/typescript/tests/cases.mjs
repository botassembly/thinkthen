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
  '18-annotate-two-groups': 'its set reads record parts through `on`, and a library record is one whole text',
  '18-find-second': 'the public find takes no `none` candidate',
  '19-find-none': 'the public find takes no `none` candidate',
  '25-defect-fault': 'no input makes the engine panic; the Rust unit test covers the binding guard',
  '30-local-question-file': 'this surface reads a question file only as an annotate set',
};

const digest = (url, request) => createHash('sha256').update(`systemone\n${url}\n${request}`).digest('hex');

function same(what, actual, expected) {
  if (!isDeepStrictEqual(actual, expected)) {
    throw new Error(`${what}: ${JSON.stringify(actual)} is not ${JSON.stringify(expected)}`);
  }
}

// Scalar-value offsets to UTF-16 units, so the expectation reads as JavaScript's.
function units(text, held) {
  const widths = [...text].map((ch) => ch.length);
  const at = (point) => widths.slice(0, point).reduce((sum, width) => sum + width, 0);
  return { ...held, start: at(held.start), end: at(held.end) };
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
    case '31-usage-rank-blank-question': return engine.rank(question.decide, one.evidence.split('\n').filter(Boolean));
    default: throw new Error(`no fault form for ${one.id}`);
  }
}

async function check(tt, one, origin, folder) {
  const base = `${origin}/${one.id === '21-backend-fault' ? 'arm/refuse' : `case/${one.id}`}/v1`;
  const engine = new tt.Engine({ baseUrl: base, cache: false });
  engine.base = base;
  if (one.expect.error) {
    const kind = await Promise.resolve().then(() => fault(tt, one, engine, folder)).then(() => 'resolved', (error) => error.kind);
    return same('kind', kind, one.expect.error.kind);
  }
  const success = one.expect.success;
  const served = `${base}/systemone`;
  const renamed = new Map(one.exchanges.map(({ request }) => [digest(CANONICAL, request), digest(served, request)]));
  const texts = one.exchanges.map((exchange) => exchange.evidence);
  const want = (at) => success.answers.find((answer) => answer.exchange === at);
  switch (success.kind) {
    case 'single': {
      same('value', await engine[one.verb](one.question, texts[0]), success.answers[0].bare);
      const details = await engine.details(one.question, texts[0]);
      const expected = success.answers[0].details;
      same('answer', details.answer, expected.answer);
      same('question_sha256', details.meta.question_sha256, expected.question_sha256);
      same('model', details.meta.model, expected.model);
      same('requests', details.meta.requests, expected.requests.map((held) => renamed.get(held)));
      for (const name of ['usage', 'requests_sent', 'cached']) same(name, details.meta[name], expected[name]);
      same('url', details.meta.url, served);
      if (success.counters) {
        const cached = new tt.Engine({ baseUrl: base, cache: join(folder, one.id) });
        for (let call = 0; call < success.counters.calls; call += 1) await cached.decide(one.question, texts[0]);
        const { requests_sent, cache_answers } = cached.usage();
        same('counters', { requests: requests_sent, cache_answers }, { requests: success.counters.requests, cache_answers: success.counters.cache_answers });
      }
      return undefined;
    }
    case 'decide_many':
      return same('values', await engine.decide_many(one.question, texts), texts.map((_, at) => want(at).bare));
    case 'filter':
      return same('kept', await engine.filter(one.question, texts), success.operation.indexes.map((at) => texts[at]));
    case 'rank': {
      const ranked = await engine.rank(one.question, texts);
      return same('ranking', ranked.map(({ index, probability }) => ({ index, probability })), success.operation.ranking);
    }
    case 'annotate': {
      const rows = await engine.annotate(one.question_set, texts);
      const expected = texts.map(() => ({}));
      for (const answer of success.answers) expected[answer.exchange][answer.name] = answer.bare;
      return same('rows', rows, expected);
    }
    case 'recognize': {
      const { kinds, relations } = one.question.recognize;
      const asked = { kinds, threshold: one.question.threshold, relationThreshold: one.question.relation_threshold };
      const found = await engine.recognize(one.text, relations ? { ...asked, relations } : asked);
      const bare = success.answers[0].bare;
      const shaped = { entities: bare.entities.map((held) => units(one.text, held)) };
      if (bare.relations) shaped.relations = bare.relations.map((held) => ({ ...held, source: units(one.text, held.source), target: units(one.text, held.target) }));
      for (const held of found.entities) same('slice', one.text.slice(held.start, held.end), held.name);
      return same('recognized', found, shaped);
    }
    case 'relate': {
      const rules = one.question.relate.relations;
      const edges = await engine.relate(one.entities, {
        relations: rules.map(({ name, source, target }) => `${name}=${source}:${target}`),
        either: rules.filter((rule) => rule.either).map((rule) => rule.name),
        threshold: one.question.threshold,
      });
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
