// The installer walk's one README question, using saved real answers only.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';
import { spawn } from 'node:child_process';
import { performance } from 'node:perf_hooks';
import { captureBackend } from '../site/scripts/capture-backend.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const output = path.resolve(process.argv[2]);
const readme = fs.readFileSync(path.join(root, 'README.md'), 'utf8');
const section = readme.split('## Ask a question\n')[1]?.split('\n## ')[0];
const command = section?.match(/```sh\n([\s\S]*?)\n```/)?.[1];
const expected = section?.match(/```text\n([\s\S]*?)\n```/)?.[1] + '\n';
assert(command && expected !== 'undefined\n', 'README first question/output not found');
fs.writeFileSync(path.join(output, 'answer-command.sh'), command + '\n');
const cache = path.join(output, 'recorded-responses.json');
const hash = () => createHash('sha256').update(fs.readFileSync(cache)).digest('hex');
const before = hash();
const recorder = await captureBackend({ file: cache, replayOnly: true });
let result;
const started = performance.now();
try {
  // The Python parent supplies a fresh HOME and a key-free OS/tool allowlist.
  const env = { ...process.env, THINKTHEN_BASE_URL: recorder.url, THINKTHEN_API_KEY: 'docs-loopback' };
  result = await new Promise((resolve, reject) => {
    const child = spawn('sh', ['-c', command], { env, detached: true });
    const stdout = []; const stderr = [];
    child.stdout.on('data', (data) => stdout.push(data));
    child.stderr.on('data', (data) => stderr.push(data));
    const timeout = setTimeout(() => process.kill(-child.pid, 'SIGKILL'), 120_000);
    child.once('error', (error) => { clearTimeout(timeout); reject(error); });
    child.once('close', (exit, signal) => {
      clearTimeout(timeout);
      resolve({ exit, signal, stdout: Buffer.concat(stdout), stderr: Buffer.concat(stderr) });
    });
  });
  fs.writeFileSync(path.join(output, 'answer.stdout'), result.stdout);
  fs.writeFileSync(path.join(output, 'answer.stderr'), result.stderr);
  assert.equal(recorder.failure(), null, 'strict replay failed; no live fallback');
  assert.equal(result.exit, 0, 'README question exit');
  assert.deepEqual(result.stdout, Buffer.from(expected), 'README answer bytes');
  assert.equal(result.stderr.length, 0, 'README question stderr');
  assert(recorder.served() > 0, 'the installed command must use the saved real answer');
} finally {
  await recorder.close();
  const usage = recorder.usage();
  fs.writeFileSync(path.join(output, 'answer.json'), JSON.stringify({
    exit: result?.exit, signal: result?.signal, seconds: (performance.now() - started) / 1000,
    served: recorder.served(), usage, failure: recorder.failure(), cache_sha256: hash(),
  }, null, 2) + '\n');
  assert.deepEqual(usage, { calls: 0, bytes: 0 }, 'replay must never pay');
  assert.equal(hash(), before, 'replay must preserve the cache');
}
