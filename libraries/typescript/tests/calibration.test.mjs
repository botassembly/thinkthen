import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { ask, startBackend } from './backend.mjs';

const fixture = JSON.parse(readFileSync(new URL('../../../conformance/calibration.json', import.meta.url)));

test('a saved profile keeps its digest and reports a runtime name mismatch', async (t) => {
  const backend = await startBackend(t);
  const profile = join(backend.folder, 'profile.json');
  writeFileSync(profile, JSON.stringify(fixture.runtime_profile));
  const { value } = await ask(backend, `
    const engine = new tt.Engine({ profile: ${JSON.stringify(profile)}, cache: false });
    const found = await engine.details(tt.question(${JSON.stringify(fixture.question)}), ${JSON.stringify(fixture.evidence)});
    return { digest: found.value.meta.question_sha256, warning: found.value.meta.profile_warning, model: found.value.meta.model };`);
  assert.deepEqual(value, {
    digest: fixture.question_sha256,
    warning: fixture.warning,
    model: fixture.model,
  });
  assert.equal(await backend.count(), 1);
});
