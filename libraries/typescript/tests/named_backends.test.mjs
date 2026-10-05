import assert from 'node:assert/strict';
import { test } from 'node:test';
import { readFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { ask, startBackend } from './backend.mjs';

const rows = JSON.parse(readFileSync(new URL('../../../conformance/binding-backends.json', import.meta.url))).backends;
const slots = ['generic_systemone', 'generic_decisions', 'generic_custom', 'capture_systemone', 'capture_decisions', 'capture_custom', 'other', 'non_post'];
const paths = (slot, n = 1) => ({ ...Object.fromEntries(slots.map((name) => [name, 0])), [slot]: n, overflow: false });

for (const row of rows) {
  test(`constructor ${row.name} selects its captured key, model, path and form`, async (t) => {
    const marker = `fake-binding-${row.name}`;
    const backend = await startBackend(t, { [row.name]: marker, unnamed: 'fake-unnamed' });
    const result = await ask(backend, `
      const engine = new tt.Engine({backend:${JSON.stringify(row.name)}, baseUrl:${JSON.stringify(backend.base('arm/full/capture'))}, cache:false});
      process.env[${JSON.stringify(row.key)}] = 'fake-later';
      const q = tt.question({decide:'Does it need attention?', true:{what:'yes', examples:['refund']}});
      return (await engine.decide(q, 'refund')).value;
    `, { env: { [row.key]: marker, THINKTHEN_API_KEY:'fake-unnamed', THINKTHEN_BASE_URL:backend.base('arm/status/500') } });
    assert.deepEqual(result, {value:true});
    assert.equal(await backend.count(), 1);
    assert.deepEqual(await backend.snapshot('paths'), paths(row.path));
    assert.deepEqual(await backend.snapshot('bearers'), { markers:{[row.name]:1,unnamed:0}, absent:0, unknown:0, overflow:false });
    const body = JSON.parse((await backend.snapshot('capture')).bodies[0]);
    assert.equal(body.model, row.model);
    assert.deepEqual(body.questions.q1.criteria.true, row.form === 'text' ? 'yes' : {what:'yes',examples:['refund']});
    if (row.form === 'both') assert.deepEqual(body.questions.q1.criteria.false, {});
    else assert.ok(!Object.hasOwn(body.questions.q1.criteria, 'false'));
  });
}

test('constructor backend errors remain usage failures without sends', async (t) => {
  const backend = await startBackend(t);
  for (const backendName of [1, true, [], {}, null, '', 'nowhere']) {
    const got = await ask(backend, `new tt.Engine({backend:${JSON.stringify(backendName)},cache:false});`);
    assert.equal(got.error.kind, 'usage');
    if (typeof backendName !== 'string') assert.equal(got.error.message, 'options.backend is a string');
    assert.ok(!JSON.stringify(got).includes('fake-loopback-key'));
  }
  assert.equal(await backend.count(), 0);
});

test('explicit backend selects configured routing over captured address and keeps setup limits', async (t) => {
  const backend = await startBackend(t, { local:'fake-local' });
  const folder = join(backend.folder, 'config');
  const config = join(folder, process.platform === 'darwin' ? 'Library/Application Support/thinkthen' : 'thinkthen');
  mkdirSync(config, {recursive:true});
  writeFileSync(join(config, 'config.json'), JSON.stringify({schema:'thinkthen.config/1', backends:{local:{
    url:backend.base('arm/full/capture'), model:'setup', key_env:'LOCAL_KEY', path:'judgements/v2/decide',
    usd_per_million_input:'1', usd_per_million_output:'2',
    profile:{schema:'thinkthen.backend-profile/1',name:'small',max_evidence_bytes:3},
  }}}));
  const env = {HOME:folder,XDG_CONFIG_HOME:folder,LOCAL_KEY:'fake-local'};
  const refused = await ask(backend, `return await new tt.Engine({backend:'local',cache:false}).decide('attention?', 'refund');`, {env});
  assert.equal(refused.error.kind, 'usage');
  assert.match(refused.error.message, /profile small/);
  assert.equal(await backend.count(), 0);
  const profile = join(folder, 'profile.json');
  writeFileSync(profile, JSON.stringify({schema:'thinkthen.backend-profile/1',name:'explicit',max_evidence_bytes:100}));
  const got = await ask(backend, `
    const engine = new tt.Engine({backend:'local',model:'override',profile:${JSON.stringify(profile)},cache:false});
    process.env.LOCAL_KEY='fake-later';
    return await engine.decide('attention?', 'refund');
  `, {env});
  assert.equal(got.value.value, true);
  assert.equal(got.value.facts.estimated_cost_usd, '0.000003');
  assert.equal(await backend.count(), 1);
  assert.deepEqual(await backend.snapshot('paths'), paths('capture_custom'));
  assert.deepEqual(await backend.snapshot('bearers'), {markers:{local:1},absent:0,unknown:0,overflow:false});
  assert.equal(JSON.parse((await backend.snapshot('capture')).bodies[0]).model, 'override');
});

test('a successful provider body at the wrong path fails the count expectation', async (t) => {
  const backend = await startBackend(t, {provider:'fake-provider'});
  const body = '{"state":"refund","model":"pplx-decider-v1-27b","questions":{"q1":{"type":"noul","description":"attention?"}}}';
  const response = await fetch(`${backend.base('arm/full/capture')}/systemone`, {method:'POST',body,headers:{Authorization:'Bearer fake-provider'}});
  assert.equal(response.status, 200);
  await response.text();
  assert.equal(await backend.count(), 1);
  assert.deepEqual(await backend.snapshot('capture'), {bodies:[body]});
  assert.deepEqual(await backend.snapshot('bearers'), {markers:{provider:1},absent:0,unknown:0,overflow:false});
  const actual = await backend.snapshot('paths');
  assert.throws(() => assert.deepEqual(actual, paths('capture_decisions'), 'posting_path_mismatch'), /posting_path_mismatch/);
});
