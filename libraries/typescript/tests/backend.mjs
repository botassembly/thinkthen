// One loopback backend per test, and child Node processes that reach it.
// Each child gets PATH, a fresh cache, a scratch XDG_CACHE_HOME, and a fake
// key only beside a 127.0.0.1 address, and nothing else (ticket 0127). No test changes
// its own environment, because the default engine reads it once.
import { spawn } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { createInterface } from 'node:readline';
import { fileURLToPath } from 'node:url';

import { childEnv as cleanEnv } from '../../../conformance/children/children.mjs';

export const FAKE_KEY = 'fake-loopback-key';
export const INDEX = fileURLToPath(new URL('../index.mjs', import.meta.url));
export const sleep = (ms) => new Promise((done) => setTimeout(done, ms));

/** Start the conformance backend and close it when the test ends. */
export async function startBackend(t) {
  const proc = spawn(process.env.THINKTHEN_TEST_BACKEND, [], { stdio: ['pipe', 'pipe', 'inherit'], env: cleanEnv() });
  const lines = createInterface({ input: proc.stdout });
  const queue = [];
  const waiting = [];
  lines.on('line', (line) => (waiting.length ? waiting.shift()(line) : queue.push(line)));
  const next = () => (queue.length ? Promise.resolve(queue.shift()) : new Promise((done) => waiting.push(done)));
  const port = Number(await next());
  const folder = mkdtempSync(join(tmpdir(), 'thinkthen-ts-'));
  const backend = {
    port,
    folder,
    base: (arm = 'generic') => `http://127.0.0.1:${port}/${arm}/v1`,
    async count() {
      proc.stdin.write('count\n');
      return Number(await next());
    },
    async wait(n) {
      proc.stdin.write(`wait ${n}\n`);
      return Number((await next()).replace('wait ', ''));
    },
    release: () => proc.stdin.write('release\n'),
  };
  t.after(() => {
    backend.release();
    proc.stdin.end();
    rmSync(folder, { recursive: true, force: true });
  });
  return backend;
}

/** The child's whole environment: PATH, the fake key, and the loopback address. */
export function childEnv(backend, arm = 'generic', extra = {}) {
  const url = extra.THINKTHEN_BASE_URL ?? backend.base(arm);
  if (new URL(url).hostname !== '127.0.0.1') throw new Error(`refusing a backend that is not loopback: ${url}`);
  const cache = mkdtempSync(join(backend.folder, 'cache-'));
  return cleanEnv({ values: { THINKTHEN_API_KEY: FAKE_KEY, THINKTHEN_BASE_URL: url, THINKTHEN_CACHE: cache, XDG_CACHE_HOME: cache, ...extra } });
}

/** Start a child running BODY as the body of an async function with `tt`
 * in scope. It prints the returned value, or the error, as one JSON line. */
export function child(backend, body, { arm = 'generic', env = {}, stdin = false } = {}) {
  const code = `import * as tt from ${JSON.stringify(INDEX)};
const line = (value) => process.stdout.write(JSON.stringify(value === undefined ? null : value) + '\\n');
globalThis.line = line;
try { line({ value: await (async () => { ${body} })() }); }
catch (error) { line({ error: { name: error.name, kind: error.kind, message: error.message, retryable: error.retryable, text: String(error) } }); }`;
  const proc = spawn(process.execPath, ['--input-type=module', '-e', code], {
    env: childEnv(backend, arm, env),
    stdio: [stdin ? 'pipe' : 'ignore', 'pipe', 'inherit'],
  });
  const lines = [];
  createInterface({ input: proc.stdout }).on('line', (text) => lines.push({ at: performance.now(), value: JSON.parse(text) }));
  const exited = new Promise((done) => proc.on('exit', (code) => done({ at: performance.now(), code })));
  return { proc, lines, exited };
}

/** Run BODY in a child and return its one result: `{ value }` or `{ error }`. */
export async function ask(backend, body, options) {
  const run = child(backend, body, options);
  const { code } = await run.exited;
  if (code !== 0 || run.lines.length === 0) throw new Error(`the child exited ${code} with ${JSON.stringify(run.lines)}`);
  return run.lines.at(-1).value;
}

/** Wait until the predicate holds or the time runs out. */
export async function until(predicate, ms) {
  const end = performance.now() + ms;
  while (performance.now() < end && !predicate()) await sleep(5);
  return predicate();
}
