// The engine settings of ADR 0017 section 5, each in its own child. An engine
// starts from the environment, and each given option overrides one setting.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdirSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

import { ask, child, sleep, startBackend } from './backend.mjs';

// Answer files, beside the folder's one backend marker.
const files = (folder) =>
  readdirSync(folder, { recursive: true, withFileTypes: true }).filter((entry) => entry.isFile() && entry.name.endsWith('.json') && !entry.name.startsWith('.'));

test('an engine starts from the environment: THINKTHEN_CACHE holds its answers', async (t) => {
  const backend = await startBackend(t);
  const folderA = join(backend.folder, 'a');
  const scratch = join(backend.folder, 'scratch');
  mkdirSync(folderA);
  mkdirSync(scratch);
  const { value } = await ask(backend, `
    const engine = new tt.Engine({ throttle: 4, baseUrl: ${JSON.stringify(backend.base())} });
    return [await engine.decide('Refund?', 'same'), await engine.decide('Refund?', 'same')];`,
  { env: { THINKTHEN_CACHE: folderA, HOME: scratch, XDG_CACHE_HOME: scratch } });
  assert.deepEqual(value, [true, true]);
  assert.equal(await backend.count(), 1);
  assert.equal(files(folderA).length, 1, 'the answer lands in folder A');
  assert.deepEqual(readdirSync(scratch), [], 'nothing lands under the scratch cache home');
});

test('baseUrl, maxRequests, and cache each override their setting', async (t) => {
  const first = await startBackend(t);
  const second = await startBackend(t);
  const folder = join(first.folder, 'named');
  const { value } = await ask(first, `
    const out = {};
    // A cache folder belongs to one backend address, so this engine keeps none.
    out.second = await new tt.Engine({ baseUrl: ${JSON.stringify(second.base())}, cache: false }).decide('Refund?', 'elsewhere');
    try { await new tt.Engine({ maxRequests: 2 }).decide_many('Refund?', ['a', 'b', 'c']); } catch (error) { out.most = [error.kind, error.message]; }
    const named = new tt.Engine({ cache: ${JSON.stringify(folder)} });
    out.named = [await named.decide('Refund?', 'kept'), await named.decide('Refund?', 'kept')];
    const none = new tt.Engine({ cache: false });
    out.none = [await none.decide('Refund?', 'unkept'), await none.decide('Refund?', 'unkept')];
    return out;`);
  assert.deepEqual(value, {
    second: true,
    most: ['usage', 'this engine answers at most 2 records in one call'],
    named: [true, true],
    none: [true, true],
  });
  assert.equal(await second.count(), 1, 'baseUrl sends to the second backend');
  assert.equal(files(folder).length, 1, 'the named cache holds one answer');
  // A streaming call sends the records before the limit, then refuses (EngineBuilder::max_requests).
  assert.equal(await first.count(), 2 + 1 + 2, 'the limit sends two, the named cache one, and no cache two');
});

test('a refused setting throws usage from the constructor and sends nothing', async (t) => {
  const backend = await startBackend(t);
  const { value } = await ask(backend, `
    const out = [];
    for (const options of [{ cacheBytes: 0 }, { throttle: 33 }, { throttle: '4' }, { nope: 1 }, { cache: true }, 'fast']) {
      try { new tt.Engine(options); out.push('built'); } catch (error) { out.push([error.name, error.kind, error.message]); }
    }
    return out;`);
  const usage = (message) => ['ThinkThenError', 'usage', message];
  assert.deepEqual(value, [
    usage('a cache cap is a whole number of bytes above zero'),
    usage('a throttle is a whole number from 1 through 32'),
    usage('options.throttle is a whole number'),
    usage('new Engine takes no option nope'),
    usage('options.cache is false or a folder path'),
    usage('new Engine takes one options object'),
  ]);
  assert.equal(await backend.count(), 0);
});

test('the first explicit throttle holds for the process and the default engine', async (t) => {
  const backend = await startBackend(t);
  const run = child(backend, `
    new tt.Engine({ throttle: 4 });
    let second;
    try { new tt.Engine({ throttle: 8 }); } catch (error) { second = error.message; }
    line(second);
    return Promise.all([0, 1, 2, 3, 4].map((at) => tt.decide('Refund?', 'held ' + at)));`, { arm: 'arm/held' });
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
