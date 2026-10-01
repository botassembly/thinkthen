// The engine settings of ADR 0017 section 5, each in its own child. An engine
// starts from the environment, and each given option overrides one setting.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdirSync, readdirSync, readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { join } from 'node:path';

import { ask, child, sleep, startBackend, storedAnswers } from './backend.mjs';

test('an engine starts from the environment: THINKTHEN_CACHE holds its answers', async (t) => {
  const backend = await startBackend(t);
  const folderA = join(backend.folder, 'a');
  const scratch = join(backend.folder, 'scratch');
  mkdirSync(folderA);
  mkdirSync(scratch);
  const { value } = await ask(backend, `
    const engine = new tt.Engine({ throttle: 4, baseUrl: ${JSON.stringify(backend.base())} });
    return [(await engine.decide('Refund?', 'same')).value, (await engine.decide('Refund?', 'same')).value];`,
  { env: { THINKTHEN_CACHE: folderA, HOME: scratch, XDG_STATE_HOME: scratch } });
  assert.deepEqual(value, [true, true]);
  assert.equal(await backend.count(), 1);
  assert.equal((await ask(backend, `return new tt.Engine({ cache: false }).usage();`)).value.retries, 0);
  assert.equal(await storedAnswers(folderA), 1, 'the answer lands in folder A');
  // ADR 0113: the scratch home holds only the count-only usage totals, in its state folder (ticket 0360).
  const written = readdirSync(scratch, { recursive: true, withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => join(entry.parentPath, entry.name));
  assert.ok(written.length > 0 && written.every((path) => path.startsWith(join(scratch, 'thinkthen') + '/')), `only usage totals land under the scratch state home: ${written}`);
  assert.ok(!written.some((path) => readFileSync(path, 'utf8').includes('Refund')), 'the usage totals hold no question text');
});

test('baseUrl and a named cache override their setting', async (t) => {
  const first = await startBackend(t);
  const second = await startBackend(t);
  const folder = join(first.folder, 'named');
  const { value } = await ask(first, `
    const out = {};
    // A cache folder belongs to one backend address, so this engine keeps none.
    out.second = (await new tt.Engine({ baseUrl: ${JSON.stringify(second.base())}, cache: false }).decide('Refund?', 'elsewhere')).value;
    const named = new tt.Engine({ cache: ${JSON.stringify(folder)} });
    out.named = [(await named.decide('Refund?', 'kept')).value, (await named.decide('Refund?', 'kept')).value];
    return out;`);
  assert.deepEqual(value, {
    second: true,
    named: [true, true],
  });
  assert.equal(await second.count(), 1, 'baseUrl sends to the second backend');
  assert.equal(await storedAnswers(folder), 1, 'the named cache holds one answer');
  assert.equal(await first.count(), 1, 'the named cache sends once');
});

test('a refused setting throws usage from the constructor and sends nothing', async (t) => {
  const backend = await startBackend(t);
  const { value } = await ask(backend, `
    const out = [];
    for (const options of [{ timeoutSeconds: 0 }, { maxRetries: 1.5 }, { maxRequestBytes: 0 }, { throttle: 33 }, { throttle: '4' }, { nope: 1 }, { cache: true }, 'fast']) {
      try { new tt.Engine(options); out.push('built'); } catch (error) { out.push([error.name, error.kind, error.message]); }
    }
    return out;`);
  const usage = (message) => ['ThinkThenError', 'usage', message];
  assert.deepEqual(value, [
    usage('a timeout is a time above zero'),
    usage('options.maxRetries is a whole number'),
    usage('max_request_bytes is a whole number of at least 1'),
    usage('a throttle is a whole number from 1 through 32'),
    usage('options.throttle is a whole number'),
    usage('new Engine takes no option nope'),
    usage('options.cache is false or a folder path'),
    usage('new Engine takes one options object'),
  ]);
  assert.equal(await backend.count(), 0);
});

// The token cap reaches TypeScript through from_env; a regression if from_env stops reading the variable.
test('the token cap variable refuses a call before any request', async (t) => {
  const backend = await startBackend(t);
  const { error } = await ask(backend, `await new tt.Engine({ cache: false }).decide('Refund?', 'text');`,
    { env: { THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL: '10' } });
  assert.deepEqual([error?.name, error?.kind, error?.message], ['ThinkThenError', 'usage',
    "max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request"]);
  assert.equal(await backend.count(), 0);
});

test('an explicit engine batch outranks an invalid environment batch', async (t) => {
  const backend = await startBackend(t);
  const { value, error } = await ask(backend, `
    const engine = new tt.Engine({ batch: 1, cache: false });
    const first = await engine.decide_many('Refund?', ['one', 'two']);
    const second = await engine.decide_many('Refund?', ['three', 'four'], { batch: 'max' });
    return { first, second };`, { env: { THINKTHEN_BATCH: 'invalid' } });
  assert.equal(error, undefined, JSON.stringify(error));
  assert.deepEqual(value.first.value, [true, true]);
  assert.equal(value.first.facts.requests_sent, 2);
  assert.equal(value.second.facts.requests_sent, 1);
  assert.equal(await backend.count(), 3);
});

test('the first explicit throttle holds for the process and the default engine', async (t) => {
  const backend = await startBackend(t);
  const run = child(backend, `
    new tt.Engine({ throttle: 4 });
    let second;
    try { new tt.Engine({ throttle: 8 }); } catch (error) { second = error.message; }
    line(second);
    return Promise.all([0, 1, 2, 3, 4].map(async (at) => (await tt.decide('Refund?', 'held ' + at)).value));`, { arm: 'arm/held' });
  assert.equal(await backend.wait(4), 4);
  await sleep(300);
  assert.equal(await backend.count(), 4, 'the module-level calls follow throttle 4');
  backend.release();
  await run.exited;
  assert.deepEqual(run.lines.map((held) => held.value), [
    'throttle 4 is already active for this process; use throttle 4 or drop the throttle argument',
    { value: [true, true, true, true, true] },
  ]);
});

// Ticket 0291's P1 through the public plan, with no key: the exact plan object
// of the shared corpus. A batch of 0 refuses as usage with code 1, and neither
// reaches the backend.
test('plan previews P1 with no key and sends nothing', async (t) => {
  const backend = await startBackend(t);
  const corpus = JSON.parse(readFileSync(fileURLToPath(new URL('../../../specification/fixtures/types/corpus.json', import.meta.url)), 'utf8'));
  const p1 = corpus.cases.find((one) => one.name === 'plan-p1');
  const { question, input, settings } = p1.plan_input;
  assert.deepEqual(settings, {});
  const { value } = await ask(backend, `
    const out = [tt.plan(${JSON.stringify(question)}, ${JSON.stringify(input)})];
    try { tt.plan(${JSON.stringify(question)}, ${JSON.stringify(input)}, { batch: 0 }); } catch (error) { out.push([error.kind, error.code]); }
    try { tt.plan('Refund?', ['one'], { deadlineMs: 0 }); } catch (error) { out.push([error.kind, error.message]); }
    return out;`, { env: { THINKTHEN_API_KEY: undefined } });
  assert.deepEqual(value, [p1.response, ['usage', 1], ['usage', 'options.deadlineMs is not a plan key']]);
  assert.equal(await backend.count(), 0);
});

// A zero process cap refuses the first live send as usage, a spent deadline
// stops recognize and relate, and the helpers read codes and failures.
test('the zero cap and spent budgets send nothing; the helpers read codes and failures', async (t) => {
  const backend = await startBackend(t);
  const { value } = await ask(backend, `
    const out = [];
    try { await new tt.Engine({ maxRequestsTotal: 0, cache: false }).decide('Refund?', 'capped'); }
    catch (error) { out.push([error.kind, error.code, error.message.includes('process send budget')]); }
    for (const call of [() => tt.recognize('Ada Lovelace', { deadlineMs: 0 }),
      () => tt.relate([['First', 'alert'], ['Second', 'alert']], { relations: ['caused_by=alert:alert'], deadlineMs: 0 })]) {
      try { await call(); } catch (error) { out.push([error.kind, error.code]); }
    }
    const failure = { failed: { kind: 'backend', cause: 'missing_answer', surprise: 1 } };
    out.push(tt.failed(failure), [null, true, 'billing', 1.2, ['billing'], {}, { ...failure, other: 1 }].map(tt.failed));
    out.push([true, false, null].map(tt.outcome), [tt.YES, tt.NO, tt.UNSURE]);
    return out;`);
  assert.deepEqual(value, [['usage', 1, true], ['deadline', 3], ['deadline', 3],
    { kind: 'backend', cause: 'missing_answer', surprise: 1 }, [null, null, null, null, null, null, null], [1, 0, 2], [1, 0, 2]]);
  assert.equal(await backend.count(), 0);
});
