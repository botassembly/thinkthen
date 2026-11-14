// Exercise transport boundaries without any paid service or credential.
import assert from 'node:assert/strict';
import http from 'node:http';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { captureBackend } from './capture-backend.mjs';
import { createHash } from 'node:crypto';
const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'docs-capture-'));
let calls = 0;
let upstreamMode = 'valid';
const upstream = http.createServer(async (req, res) => {
  assert.equal(req.url, '/v1/systemone');
  assert.equal(req.headers.authorization, 'Bearer private-test-only');
  let body = ''; for await (const chunk of req) body += chunk;
  assert.equal(Object.keys(JSON.parse(body).questions).length, 1);
  calls++;
  if (upstreamMode === 'failed') { res.writeHead(503); res.end('temporary failure'); return; }
  if (upstreamMode === 'invalid') { res.end('private-test-only invalid JSON'); return; }
  if (upstreamMode === 'large-valid') { res.end(JSON.stringify({ answers: { q1: { type: 'noul', noul: 0.99 } }, padding: 'x'.repeat(11000) })); return; }
  if (upstreamMode === 'large') { res.end('x'.repeat(1000)); return; }
  res.end(JSON.stringify({ answers: { q1: { type: 'noul', noul: 0.99 } } }));
});
await new Promise((resolve) => upstream.listen(0, '127.0.0.1', resolve));
const options = { file: path.join(dir, 'responses.json'), key: 'private-test-only', upstream: `http://127.0.0.1:${upstream.address().port}/v1`, maxCalls: 1 };
const post = (proxy, question) => fetch(proxy.url + '/systemone', { method: 'POST', body: JSON.stringify({ state: 'Please refund my order.', model: 'test', questions: question }) });
let proxy;
try {
  const previous = process.cwd();
  process.chdir(dir);
  try {
    for (const file of [undefined, null, '', '   ', 7]) {
      await assert.rejects(captureBackend({ ...options, file, replayOnly: true }), /explicit cache file/);
    }
    assert.deepEqual(fs.readdirSync(dir), [], 'invalid cache selection creates no files');
  } finally { process.chdir(previous); }
  proxy = await captureBackend(options);
  assert.equal((await post(proxy, { q9: { type: 'noul', instructions: 'Refund?' } })).status, 200);
  assert.deepEqual((await (await post(proxy, { q3: { instructions: 'Refund?', type: 'noul' } })).json()).answers.q3, { type: 'noul', noul: 0.99 });
  assert.equal(calls, 1, 'equivalent questions are paid once, independent of wire name and key order');
  assert.equal((await post(proxy, { q1: { type: 'noul', instructions: 'Different?' } })).status, 502);
  assert.equal(calls, 1, 'call cap blocks an upstream request');
  assert.doesNotMatch(fs.readFileSync(options.file, 'utf8'), /private-test-only|authorization/i);
  await proxy.close();
  const originalCache = fs.readFileSync(options.file);
  fs.writeFileSync(`${options.file}.miss.json`, 'old body', { mode: 0o644 });
  proxy = await captureBackend({ ...options, replayOnly: true });
  assert.equal((await post(proxy, { q1: { type: 'noul', instructions: 'Refund?' } })).status, 200);
  assert.equal((await post(proxy, { q1: { type: 'noul', instructions: 'Missing?' } })).status, 502);
  assert.equal(calls, 1, 'replay never reaches upstream');
  assert.deepEqual(proxy.usage(), { calls: 0, bytes: 0, inputBytes: 0 });
  assert.equal(proxy.failure(), 'saved response is missing');
  assert.deepEqual(fs.readFileSync(options.file), originalCache, 'miss never changes saved answers');
  const missFile = `${options.file}.miss.json`;
  const missText = fs.readFileSync(missFile, 'utf8');
  const miss = JSON.parse(missText);
  assert.deepEqual(miss.request, { model: 'test', questions: { q1: { instructions: 'Missing?', type: 'noul' } }, state: 'Please refund my order.' });
  assert.equal(miss.digest, createHash('sha256').update(JSON.stringify(miss.request)).digest('hex'));
  assert.deepEqual(Object.keys(miss).sort(), ['digest', 'request']);
  assert.doesNotMatch(missText, /private-test-only|authorization/i);
  assert.equal(fs.statSync(missFile).mode & 0o777, 0o600);
  await proxy.close();
  proxy = await captureBackend({ ...options, file: path.join(dir, 'tiny.json'), maxBytes: 2 });
  assert.equal((await post(proxy, { q1: {} })).status, 502);
  assert.equal(calls, 1, 'byte cap blocks an upstream request');
  await proxy.close();
  upstreamMode = 'large-valid';
  const inputRequest = JSON.stringify({ model: 'test', questions: { q1: { instructions: 'Budget?', type: 'noul' } }, state: 'Please refund my order.' });
  const inputLimit = Buffer.byteLength(inputRequest) + 1;
  proxy = await captureBackend({ ...options, file: path.join(dir, 'input.json'), maxCalls: 5, maxInputBytes: inputLimit });
  const beforeInput = calls;
  assert.equal((await post(proxy, { q1: { type: 'noul', instructions: 'Budget?' } })).status, 200);
  assert.ok(proxy.usage().bytes > 10000, 'response has a separate total allowance');
  assert.equal(proxy.usage().inputBytes, inputLimit - 1);
  assert.ok(fs.existsSync(path.join(dir, 'input.json')), 'large valid paid reply is retained');
  assert.equal((await post(proxy, { renamed: { instructions: 'Budget?', type: 'noul' } })).status, 200);
  assert.equal(proxy.usage().inputBytes, inputLimit - 1, 'cache hit consumes no input allowance');
  assert.equal((await post(proxy, { q1: { type: 'noul', instructions: 'Another?' } })).status, 502);
  assert.equal(calls, beforeInput + 1, 'cumulative input admission refuses before another paid request');
  await proxy.close();
  for (const maxInputBytes of [0, -1, NaN, 1.5]) {
    await assert.rejects(captureBackend({ ...options, maxInputBytes }), /positive integers/);
  }
  upstreamMode = 'failed';
  proxy = await captureBackend({ ...options, file: path.join(dir, 'failed.json'), maxCalls: 5 });
  const beforeFailure = calls;
  assert.equal((await post(proxy, { q1: { type: 'noul', instructions: 'Retry?' } })).status, 502);
  assert.equal((await post(proxy, { q1: { type: 'noul', instructions: 'Retry?' } })).status, 502);
  assert.equal(calls, beforeFailure + 1, 'upstream failure stops the capture rather than paying for an engine retry');
  assert.match(proxy.failure(), /HTTP 503/);
  await proxy.close();
  upstreamMode = 'invalid';
  proxy = await captureBackend({ ...options, file: path.join(dir, 'invalid.json') });
  assert.equal((await post(proxy, { q1: {} })).status, 502);
  assert.equal(proxy.failure(), 'docs capture failed', 'parser errors never disclose upstream body text');
  assert.equal(fs.existsSync(path.join(dir, 'invalid.json')), false);
  await proxy.close();
  upstreamMode = 'large';
  proxy = await captureBackend({ ...options, file: path.join(dir, 'large.json'), maxBytes: 400 });
  assert.equal((await post(proxy, { q1: {} })).status, 502);
  assert.match(proxy.failure(), /byte cap/);
  assert.equal(fs.existsSync(path.join(dir, 'large.json')), false, 'over-budget responses are never saved');
  console.log('Capture controls pass: deduplication, wire remapping, replay misses, call and byte caps, credential exclusion');
} finally {
  if (proxy) await proxy.close();
  await new Promise((resolve) => upstream.close(resolve));
  fs.rmSync(dir, { recursive: true, force: true });
}
