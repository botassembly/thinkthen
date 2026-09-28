// An AbortSignal settles a call at once, Node may exit after any settle, and
// a shared signal keeps no listener. The worker finishes a sent request.
import { test } from 'node:test';
import assert from 'node:assert/strict';

import { ask, child, sleep, startBackend, until } from './backend.mjs';

const ABORTED = 'the call was cancelled by its AbortSignal';

// The child aborts on its first line in, then closes its input.
const ABORT_ON_LINE = `
  const stop = new AbortController();
  process.stdin.once('data', () => { process.stdin.destroy(); stop.abort('stop'); });
  try { await CALL; return 'resolved'; }
  catch (error) { return { kind: error.kind, message: error.message, cause: error.cause }; }`;

/** Abort once the backend has read N requests; time the reject and the exit. */
async function aborted(t, call, n, engine = 'tt') {
  const backend = await startBackend(t);
  const body = ABORT_ON_LINE.replace('CALL', call.replace('ON', engine));
  const run = child(backend, body, { arm: 'arm/held', stdin: true });
  assert.equal(await backend.wait(n), n);
  const abortAt = performance.now();
  run.proc.stdin.write('abort\n');
  const exited = await Promise.race([run.exited, sleep(2000).then(() => null)]);
  backend.release();
  await sleep(500);
  const count = await backend.count();
  run.proc.kill();
  return { backend, run, abortAt, exited, count };
}

test('an abort settles a held single call at once, and Node exits', async (t) => {
  const { backend, run, abortAt, exited, count } = await aborted(t, "ON.decide('Refund?', 'held', { signal: stop.signal })", 1);
  const [{ at, value }] = run.lines;
  assert.deepEqual(value.value, { kind: 'cancelled', message: ABORTED, cause: 'stop' });
  assert.ok(at - abortAt <= 100, `rejected ${Math.round(at - abortAt)} ms after the abort`);
  assert.ok(exited && exited.at - abortAt <= 500, 'the child exits while the reply is held');
  assert.equal(count, 1);
  const next = await ask(backend, "return tt.decide('Refund?', 'next');");
  assert.equal(next.value.value, true, 'a detached worker leaks into no later call');
});

test('an abort settles a held batch at once, and the batch sends no more', async (t) => {
  const records = JSON.stringify(Array.from({ length: 200 }, (_, at) => `record ${at}`));
  // A 3 s timer keeps the child alive past the release, so a batch whose token never fired would send again.
  const call = `(setTimeout(() => {}, 3000), new tt.Engine({ throttle: 4 }).decide_many('Refund?', ${records}, { signal: stop.signal, batch: 2 }))`;
  const { run, abortAt, count } = await aborted(t, call, 4);
  const [{ at, value }] = run.lines;
  assert.equal(value.value.kind, 'cancelled');
  assert.ok(at - abortAt <= 100, `rejected ${Math.round(at - abortAt)} ms after the abort`);
  assert.equal(count, 4);
});

test('an observed completion retains the one held worker and its final account', async (t) => {
  const backend = await startBackend(t);
  const run = child(backend, `
    const stop = new AbortController();
    process.stdin.once('data', () => { process.stdin.destroy(); stop.abort('stop'); });
    try { await tt.decide_many('Refund?', ['first', 'second', 'third'], { signal: stop.signal, batch: 2 }); }
    catch (error) {
      line({ early: error.kind, hasFacts: Object.hasOwn(error, 'facts'), hasCompletion: !!error.completion });
      return error.completion.wait();
    }`, { arm: 'arm/held', stdin: true });
  assert.equal(await backend.wait(2), 2);
  run.proc.stdin.write('abort\n');
  assert.ok(await until(() => run.lines.length === 1, 1000), 'the early rejection is prompt');
  assert.deepEqual(run.lines[0].value, { early: 'cancelled', hasFacts: false, hasCompletion: true });
  backend.release();
  await run.exited;
  assert.equal(run.lines.length, 2);
  const report = run.lines[1].value.value;
  assert.equal(report.err.kind, 'cancelled');
  assert.equal(report.err.facts.requests_sent, 2);
  assert.equal(report.err.facts.records, 3);
  assert.equal(report.err.details.length, 3);
  assert.equal(await backend.count(), 2, 'the cancelled planner admits no later request');
});

test('a settled call lets Node exit, after a result and after an error', async (t) => {
  const backend = await startBackend(t);
  for (const [arm, call] of [
    ['generic', "return tt.decide('Refund?', 'x');"],
    ['arm/held', "return tt.decide('Refund?', 'x', { deadlineMs: 300 });"],
  ]) {
    const run = child(backend, call, { arm });
    const release = setTimeout(() => backend.release(), 2000);
    const kill = setTimeout(() => run.proc.kill(), 3000);
    const exited = await run.exited;
    clearTimeout(release);
    clearTimeout(kill);
    assert.equal(run.lines.length, 1, `${arm} settled`);
    assert.ok(exited.code === 0 && exited.at - run.lines[0].at <= 500, `${arm}: the child exits by itself`);
  }
});

async function sharedSignalLeavesNoListener(t, calls) {
  const backend = await startBackend(t);
  const { value } = await ask(backend, `
    const { getEventListeners } = await import('node:events');
    const shared = new AbortController();
    for (let at = 0; at < ${calls}; at += 1) await tt.decide('Refund?', 'same text', { signal: shared.signal });
    const afterCalls = getEventListeners(shared.signal, 'abort').length;
    const gone = AbortSignal.abort();
    await tt.decide('Refund?', 'x', { signal: gone }).catch(() => {});
    const midCall = new AbortController();
    const pending = tt.decide('Refund?', 'other text', { signal: midCall.signal }).catch(() => {});
    midCall.abort();
    await pending;
    return [afterCalls, getEventListeners(gone, 'abort').length, getEventListeners(midCall.signal, 'abort').length];`);
  assert.deepEqual(value, [0, 0, 0]);
}

test('settled and aborted shared-signal calls leave no listener behind', async (t) => {
  await sharedSignalLeavesNoListener(t, 3);
});

test('stress: two thousand shared-signal calls leave no listener behind', async (t) => {
  await sharedSignalLeavesNoListener(t, 2000);
});
