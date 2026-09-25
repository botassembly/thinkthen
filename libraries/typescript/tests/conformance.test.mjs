// The shared cases through this package. Every case reports pass, fail, or
// not run with its reason, the three counts sum to the file's count, and any
// failure names its case. THINKTHEN_TEST_CASES names another copy of the file.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

import { ask, startBackend } from './backend.mjs';

const FILE = process.env.THINKTHEN_TEST_CASES ?? fileURLToPath(new URL('../../../conformance/cases.json', import.meta.url));
const RUNNER = fileURLToPath(new URL('./cases.mjs', import.meta.url));

test('every shared case passes or says why it does not run', async (t) => {
  const backend = await startBackend(t);
  const cases = JSON.parse(readFileSync(FILE, 'utf8'));
  const { value, error } = await ask(backend, `
    const { runCases } = await import(${JSON.stringify(RUNNER)});
    const { readFileSync } = await import('node:fs');
    return runCases(tt, JSON.parse(readFileSync(${JSON.stringify(FILE)}, 'utf8')), ${JSON.stringify(`http://127.0.0.1:${backend.port}`)}, ${JSON.stringify(backend.folder)});`);
  assert.equal(error, undefined, JSON.stringify(error));
  const counts = { pass: 0, fail: 0, 'not run': 0 };
  for (const [id, status, why] of value) {
    counts[status] += 1;
    t.diagnostic(`${status} ${id}${why ? `: ${why}` : ''}`);
  }
  t.diagnostic(`pass ${counts.pass}, fail ${counts.fail}, not run ${counts['not run']}, of ${cases.case_count}`);
  assert.equal(counts.pass + counts.fail + counts['not run'], cases.case_count);
  assert.equal(cases.cases.length, cases.case_count);
  assert.deepEqual(value.filter(([, status]) => status === 'fail'), []);
});
