// The shared cases through this package. Every case reports pass, fail, or
// not run with its reason, the three counts sum to the file's count, and any
// failure names its case. THINKTHEN_TEST_CASES names another copy of the file.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { isAbsolute } from 'node:path';
import { fileURLToPath } from 'node:url';

import { ask, startBackend } from './backend.mjs';

const FILE = process.env.THINKTHEN_TEST_CASES ?? fileURLToPath(new URL('../../../conformance/cases.json', import.meta.url));
const RUNNER = fileURLToPath(new URL('./cases.mjs', import.meta.url));

test('every shared case passes or says why it does not run', async (t) => {
  const cases = JSON.parse(readFileSync(FILE, 'utf8'));
  const all = cases.cases;
  assert.equal(all.length, cases.case_count);
  const available = new Set(all.map(({ id }) => id));
  assert.equal(available.size, all.length, 'shared case IDs are unique');
  const path = process.env.THINKTHEN_CONFORMANCE_IDS;
  let selected = null;
  if (path !== undefined) {
    assert.ok(isAbsolute(path), 'THINKTHEN_CONFORMANCE_IDS takes an absolute path');
    const ids = readFileSync(path, 'utf8').split(/\r?\n/).map((line) => line.trim())
      .filter((id) => id && !id.startsWith('#'));
    selected = new Set(ids);
    assert.ok(ids.length > 0, 'the selected case list is empty');
    assert.equal(selected.size, ids.length, 'selected case IDs are unique');
    for (const id of selected) assert.ok(available.has(id), `selected case ${id} is absent`);
  }
  const chosen = selected === null ? all : all.filter(({ id }) => selected.has(id));
  const backend = await startBackend(t);
  const { value, error } = await ask(backend, `
    const { runCases } = await import(${JSON.stringify(RUNNER)});
    const { readFileSync } = await import('node:fs');
    const document = JSON.parse(readFileSync(${JSON.stringify(FILE)}, 'utf8'));
    const selected = ${JSON.stringify(selected === null ? null : [...selected])};
    if (selected !== null) document.cases = document.cases.filter(({ id }) => selected.includes(id));
    return runCases(tt, document, ${JSON.stringify(`http://127.0.0.1:${backend.port}`)}, ${JSON.stringify(backend.folder)});`);
  assert.equal(error, undefined, JSON.stringify(error));
  const counts = { pass: 0, fail: 0, 'not run': 0 };
  for (const [id, status, why] of value) {
    counts[status] += 1;
    t.diagnostic(`${status} ${id}${why ? `: ${why}` : ''}`);
  }
  t.diagnostic(`total=${cases.case_count} selected=${chosen.length} pass=${counts.pass} fail=${counts.fail} not_run=${counts['not run']} unselected=${cases.case_count - chosen.length}`);
  assert.equal(counts.pass + counts.fail + counts['not run'], chosen.length);
  assert.deepEqual(value.filter(([, status]) => status === 'fail'), []);
});
