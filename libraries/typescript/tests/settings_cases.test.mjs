// The shared engine-setting corpus through the TypeScript option names.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, mkdirSync, readdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { ask, startBackend } from './backend.mjs';

const corpus = JSON.parse(readFileSync(fileURLToPath(new URL('../../../conformance/settings.json', import.meta.url))));
assert.equal(corpus.schema, 'thinkthen.settings-cases/1');
const names = { timeout: 'timeoutSeconds', max_retries: 'maxRetries', max_requests: 'maxRequests',
  max_request_bytes: 'maxRequestBytes', model: 'model', profile: 'profile', record: 'record', replay: 'replay', cache: 'cache' };

for (const entry of corpus.cases) {
  test(`shared setting: ${entry.id}`, async (t) => {
    const backend = await startBackend(t);
    const folder = join(backend.folder, 'recording');
    mkdirSync(folder);
    const profile = join(backend.folder, 'profile.json');
    if (entry.profile) writeFileSync(profile, JSON.stringify(entry.profile));
    for (const step of entry.steps) {
      const options = Object.fromEntries(Object.entries(step.settings).map(([key, value]) =>
        [names[key], value === '$FOLDER' ? folder : value === '$PROFILE' ? profile : value]));
      assert.equal(Object.keys(options).length, Object.keys(step.settings).length);
      const body = `
        const engine = new tt.Engine(${JSON.stringify(options)});
        ${step.verb === 'relate'
          ? `return (await engine.relate(${JSON.stringify(entry.entities)}, { relations: [${JSON.stringify(entry.relation)}] })).length;`
          : step.verb === 'decide_many'
          ? `return await engine.decide_many(${JSON.stringify(corpus.question)}, ${JSON.stringify(step.records)});`
          : step.model
            ? `const details = await engine.details(${JSON.stringify(corpus.question)}, ${JSON.stringify(step.text)}); return { value: details.value, model: details.meta.model };`
            : `return await engine.decide(${JSON.stringify(corpus.question)}, ${JSON.stringify(step.text)});`}`;
      const result = await ask(backend, body, { arm: entry.arm.replace(/\/v1$/, ''), env: { THINKTHEN_CACHE: folder } });
      if (step.error) assert.equal(result.error?.kind, step.error, entry.id);
      else if (step.verb === 'relate') assert.equal(result.value, step.edges, entry.id);
      else if (step.model) assert.deepEqual(result.value, { value: step.value, model: step.model }, entry.id);
      else assert.equal(result.value, step.value, entry.id);
      assert.equal(await backend.count(), step.count, entry.id);
    }
    if (entry.entries !== undefined) {
      const count = readdirSync(folder).filter((name) => name.endsWith('.json') && !name.startsWith('.')).length;
      assert.equal(count, entry.entries, entry.id);
    }
  });
}
