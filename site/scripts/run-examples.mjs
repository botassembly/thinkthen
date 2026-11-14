// Run examples with isolated homes and separate stdout/stderr.
// Routine mode (default/--check) uses the test backend and writes only .example-runs.
// --capture EXAMPLES... records real bodies once, reuses them across surfaces,
// and writes public output only after the cell passes. --replay uses saved bodies
// without network calls. Run command cells first to establish semantic answers.
// Capture caps: SITE_CAPTURE_MAX_CALLS (99), SITE_CAPTURE_MAX_INPUT_BYTES (10000), SITE_CAPTURE_MAX_BYTES (300000).
// SITE_CAPTURE_FILE selects the body-only recording; credentials remain in memory.
// Capture/replay require an empty absolute SITE_CAPTURE_ARTIFACT_DIR outside the checkout.
import { spawn, spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import product from '../product.config.mjs';
import site from '../src/data/site.json' with { type: 'json' };
import { binaryInfo } from '../src/lib/binary.mjs';
import { jsonValues } from '../src/lib/json-values.mjs';
import { captureBackend } from './capture-backend.mjs';
import { runWindowsExample } from './windows-example.mjs';
import { isDeepStrictEqual } from 'node:util';
import { createHash } from 'node:crypto';
import { needsPreparation } from '../src/lib/examples.mjs';
import { validatePreparedExamples } from '../src/lib/prepared-examples.mjs';

const args = process.argv.slice(2);
const capture = args.includes('--capture');
const replay = args.includes('--replay');
const check = !capture && !replay;
if (capture && replay) throw new Error('choose capture or replay');
if ((capture || replay) && !args.some((arg) => !arg.startsWith('--'))) throw new Error('capture and replay require explicit example IDs');
const only = args.filter((a) => !a.startsWith('--'));
const root = path.resolve('examples');
const runs = path.resolve('.example-runs');
const bin = path.resolve(product.binDir);
const today = new Date().toISOString().slice(0, 10);
let transport;
let artifactDir;
let manifest;
if (!check) {
  artifactDir = process.env.SITE_CAPTURE_ARTIFACT_DIR;
  if (!artifactDir || !path.isAbsolute(artifactDir)) throw new Error('capture and replay require an absolute SITE_CAPTURE_ARTIFACT_DIR outside the checkout');
  fs.mkdirSync(artifactDir, { recursive: true });
  artifactDir = fs.realpathSync(artifactDir);
  const relative = path.relative(fs.realpathSync('..'), artifactDir);
  if (!relative.startsWith(`..${path.sep}`) || fs.readdirSync(artifactDir).length) throw new Error('SITE_CAPTURE_ARTIFACT_DIR must be outside the checkout and empty');
  manifest = { source: spawnSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).stdout.trim() || null, mode: capture ? 'capture' : 'replay', passed_ids: [], failures: [], removed_notices: [], files: {} };
  process.on('exit', (code) => {
    try {
      manifest.exit = code;
      Object.assign(manifest, transport?.usage() ?? { calls: 0, bytes: 0 });
      fs.writeFileSync(path.join(artifactDir, 'manifest.json'), JSON.stringify(manifest, null, 2) + '\n');
    } catch {
      console.error('Could not retain capture/replay artifacts');
      process.exitCode = code || 1;
    }
  });
}
function retainExample(id, result, problems, removedNotice) {
  const success = problems.length === 0;
  const prefix = `${success ? 'site/examples' : 'failures'}/${id}`;
  const files = success
    ? Object.fromEntries(['output.txt', 'stderr.txt', 'exit', 'ran', 'capture.json'].map((name) => [name, fs.readFileSync(path.join(root, id, name))]))
    : { 'output.txt': result.rawOut, 'stderr.txt': result.rawErr, exit: `${result.exit}\n`, ...(result.answers === null ? {} : { 'answers.json': result.answers }) };
  fs.mkdirSync(path.join(artifactDir, prefix), { recursive: true });
  for (const [name, value] of Object.entries(files)) {
    const bytes = Buffer.isBuffer(value) ? value : Buffer.from(value);
    fs.writeFileSync(path.join(artifactDir, prefix, name), bytes);
    manifest.files[`${prefix}/${name}`] = createHash('sha256').update(bytes).digest('hex');
  }
  if (result.provenance) {
    const name = `private/windows/${id}.json`;
    const bytes = Buffer.from(JSON.stringify(result.provenance, null, 2) + '\n');
    fs.mkdirSync(path.dirname(path.join(artifactDir, name)), { recursive: true });
    fs.writeFileSync(path.join(artifactDir, name), bytes);
    manifest.files[name] = createHash('sha256').update(bytes).digest('hex');
  }
  if (success) {
    manifest.passed_ids.push(id);
    if (removedNotice) manifest.removed_notices.push(`site/examples/${id}/unverified`);
  } else manifest.failures.push({ id, exit: result.exit, problems });
  fs.writeFileSync(path.join(artifactDir, 'manifest.json'), JSON.stringify(manifest, null, 2) + '\n');
}
if (!fs.existsSync(path.join(bin, product.binary))) {
  console.error(`missing ${path.join(bin, product.binary)}: build the product first (cargo build --release)`);
  process.exit(1);
}
const discover = (dir, prefix = '') => fs.readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
  if (!entry.isDirectory()) return [];
  const id = prefix ? `${prefix}/${entry.name}` : entry.name;
  const folder = path.join(dir, entry.name);
  return ['run.sh', 'run.ps1'].some((name) => fs.existsSync(path.join(folder, name))) ? [id] : discover(folder, id);
});
const ids = discover(root).filter((id) => !only.length || only.includes(id)).sort();
for (const id of only) if (!ids.includes(id)) throw new Error(`example ${id}: no runnable runner`);
if (!check && ids.some(needsPreparation) && !process.env.SITE_PREPARED_EXAMPLES) throw new Error('capture and replay require a prepared executable bundle; compilation is disabled');
let preparedDir;
let preparedIds = new Set();
if (process.env.SITE_PREPARED_EXAMPLES) {
  const prepared = validatePreparedExamples(process.env.SITE_PREPARED_EXAMPLES);
  preparedDir = prepared.directory;
  preparedIds = new Set(prepared.ids);
  for (const id of ids.filter(needsPreparation)) {
    if (!preparedIds.has(id) || !fs.existsSync(path.join(preparedDir, id, 'run.sh'))) throw new Error(`${id}: prepared executable is required; compilation is disabled`);
  }
}
if (check) for (const id of [...ids].filter((id) => id.startsWith('functions/'))) {
  const command = `functions/${id.split('/')[1]}/command`;
  if (!fs.existsSync(path.join(root, command, 'run.sh'))) throw new Error(`${id}: no runnable command baseline`);
  if (!ids.includes(command)) ids.push(command);
}
if (capture || replay) for (const id of ids.filter((id) => id.startsWith('functions/'))) {
  const command = `functions/${id.split('/')[1]}/command`;
  if (!ids.includes(command)) throw new Error(`${id}: include ${command} to establish fresh capture answers`);
}
const file = (id, f) => (fs.existsSync(path.join(root, id, f)) ? fs.readFileSync(path.join(root, id, f), 'utf8') : null);
const list = (text) => (text ?? '').split('\n').map((l) => l.trim()).filter((l) => l && !l.startsWith('#'));

// ---- The isolated environment ----------------------------------------------------------
// Variables never passed to an example, whatever the shell holds.
const SECRET = /(_API_KEY|_TOKEN|_SECRET|_PASSWORD)$/;
const KNOWN_KEYS = ['NCBI_API_KEY', 'S2_API_KEY', 'NCI_API_KEY', 'ONCOKB_TOKEN', 'OPENFDA_API_KEY', 'UMLS_API_KEY', 'DISGENET_API_KEY', 'ALPHAGENOME_API_KEY', 'ORCID_ACCESS_TOKEN', 'THINKTHEN_API_KEY', 'OPENAI_API_KEY'];
// Variables passed through from the shell.
const PASS = ['HTTPS_PROXY', 'https_proxy', 'HTTP_PROXY', 'http_proxy', 'NO_PROXY', 'no_proxy', 'SSL_CERT_FILE'];

const sandbox = fs.mkdtempSync(path.join(os.tmpdir(), `${product.binary}-examples-`));
const dirs = { home: path.join(sandbox, 'home'), cache: path.join(sandbox, 'cache'), config: path.join(sandbox, 'config'), tmp: path.join(sandbox, 'tmp') };
for (const d of Object.values(dirs)) fs.mkdirSync(d, { recursive: true, mode: 0o700 });
const cleanup = () => {
  // Go and other tools write read-only folders; make them writable before removing.
  spawnSync('chmod', ['-R', 'u+w', sandbox]);
  fs.rmSync(sandbox, { recursive: true, force: true });
};
process.on('exit', cleanup);
for (const s of ['SIGINT', 'SIGTERM']) process.on(s, () => process.exit(130));

if (process.env.SITE_EXAMPLE_CONFIG) {
  if (!product.exampleConfig) {
    console.error(`SITE_EXAMPLE_CONFIG is set, but ${product.name} names no exampleConfig in product.config.mjs`);
    process.exit(1);
  }
  const to = path.join(dirs.config, product.exampleConfig);
  fs.mkdirSync(path.dirname(to), { recursive: true });
  fs.copyFileSync(path.resolve(process.env.SITE_EXAMPLE_CONFIG), to);
}

const env = {
  PATH: `${bin}:${process.env.PATH}`,
  HOME: dirs.home,
  XDG_CACHE_HOME: dirs.cache,
  XDG_CONFIG_HOME: dirs.config,
  XDG_DATA_HOME: path.join(dirs.home, '.local/share'),
  TMPDIR: dirs.tmp,
  LANG: 'C.UTF-8',
  TZ: 'UTC',
  NO_COLOR: '1',
  TERM: 'dumb',
  COLUMNS: '100',
  PYTHONDONTWRITEBYTECODE: '1',
  ...Object.fromEntries(PASS.filter((k) => process.env[k] !== undefined).map((k) => [k, process.env[k]])),
  ...(product.exampleSetup?.({ ...dirs, sandbox, bin, shell: process.env }) ?? {}),
  ...(product.exampleEnv ?? {}),
  ...(preparedDir ? { SITE_EXECUTE_ONLY: '1' } : {}),
};
for (const k of Object.keys(env)) if (SECRET.test(k) || KNOWN_KEYS.includes(k)) delete env[k];

if (capture || replay) {
  if (capture && !process.env.THINKTHEN_API_KEY) throw new Error('capture needs THINKTHEN_API_KEY');
  transport = await captureBackend({ file: path.resolve(process.env.SITE_CAPTURE_FILE ?? 'examples/captured-responses.json'), key: process.env.THINKTHEN_API_KEY, replayOnly: replay, maxCalls: Number(process.env.SITE_CAPTURE_MAX_CALLS ?? 99), maxInputBytes: Number(process.env.SITE_CAPTURE_MAX_INPUT_BYTES ?? 10000), maxBytes: Number(process.env.SITE_CAPTURE_MAX_BYTES ?? 300000) });
  env.THINKTHEN_BACKEND = 'systemone';
  env.THINKTHEN_BASE_URL = transport.url;
  env.THINKTHEN_API_KEY = 'docs-loopback';
}

// ---- The version the site states must be the version of the program it runs -------------
const info = binaryInfo(env);
if (info.version !== site.version) {
  console.error(`src/data/site.json version is ${site.version}, but ${product.binary} ${product.versionArgs.join(' ')} prints ${info.version}: build the program from this commit or update site.json`);
  process.exit(1);
}

// ---- Running one example ---------------------------------------------------------------
function run(id) {
  return new Promise((resolve) => {
    const cwd = path.join(root, id);
    // detached: the example gets its own process group, so a timeout kills every process it started.
    const answerPath = path.join(fs.mkdtempSync(path.join(dirs.tmp, 'answers-')), 'answers.json');
    const exampleEnv = { ...env, SITE_ANSWERS_FILE: answerPath, ...(id.startsWith('functions/') ? { SITE_CASE_FILE: path.join(root, 'functions', id.split('/')[1], 'case.json') } : {}) };
    if ((product.exampleKeylessCaptures ?? []).includes(id)) {
      for (const key of Object.keys(exampleEnv)) if (SECRET.test(key) || KNOWN_KEYS.includes(key)) delete exampleEnv[key];
    }
    if (fs.existsSync(path.join(cwd, 'run.ps1'))) {
      runWindowsExample({ id, cwd, env: exampleEnv, timeoutMs: product.exampleTimeoutMs ?? 120000 })
        .then(resolve, (error) => resolve({ id, rawOut: Buffer.alloc(0),
          rawErr: Buffer.from(`Windows capture adapter: ${error.message}\n`),
          out: '', err: `Windows capture adapter: ${error.message}\n`,
          exit: 124, timedOut: false, answers: null }));
      return;
    }
    const entry = preparedDir && preparedIds.has(id) ? path.join(preparedDir, id, 'run.sh') : 'run.sh';
    const child = spawn('bash', ['-o', 'pipefail', entry], { cwd, env: exampleEnv, detached: true, stdio: ['ignore', 'pipe', 'pipe'] });
    let out = '';
    let err = '';
    const rawOut = [];
    const rawErr = [];
    child.stderr.on('data', (b) => { rawErr.push(b); err += b; });
    let timedOut = false;
    child.stdout.on('data', (b) => { rawOut.push(b); out += b; });
    const killGroup = () => { try { process.kill(-child.pid, 'SIGKILL'); } catch {} };
    const timer = setTimeout(() => { timedOut = true; killGroup(); }, product.exampleTimeoutMs ?? 120000);
    child.on('close', (code) => {
      clearTimeout(timer);
      killGroup();
      const answers = fs.existsSync(answerPath) ? fs.readFileSync(answerPath, 'utf8') : null;
      resolve({ id, rawOut: Buffer.concat(rawOut), rawErr: Buffer.concat(rawErr), err: err.trimEnd() ? `${err.trimEnd()}\n` : '', answers, out: out.replace(/[ \t]+$/gm, '').replace(/\s+$/, '') + '\n', exit: timedOut ? 124 : (code ?? 124), timedOut });
    });
  });
}

// Problems a run's output shows whatever was saved before.
function outputProblems(id, r) {
  const p = [];
  if (r.timedOut) p.push(`timed out after ${(product.exampleTimeoutMs ?? 120000) / 1000} s`);
  if ((r.out + r.err).includes('jq: error')) p.push('output holds "jq: error"');
  if (product.exampleFailedStatus) {
    const statuses = jsonValues(r.out).filter((v) => v && typeof v === 'object' && !Array.isArray(v) && typeof v.status === 'string').map((v) => v.status);
    const expected = list(file(id, 'expect-status'));
    if (statuses.includes('failed') && !expected.includes('failed')) p.push('a reply has status "failed"; fix the example, or add an expect-status file that names failed when the page shows a failure');
    for (const s of expected) if (!statuses.includes(s)) p.push(`expect-status names "${s}", but no reply has that status`);
  }
  return p;
}

// Everything wrong with one run.
function judge(id, r) {
  const problems = outputProblems(id, r);
  const routine = check ? product.exampleRoutineResults?.[id] : undefined;
  const want = routine?.exit ?? Number((file(id, 'exit') ?? '0').trim());
  if (!check) {
    if (r.exit !== want) problems.push(`exit ${r.exit}, expected ${want}; fix the example or explicitly review a changed documented exit`);
    return problems;
  }
  if (r.exit !== want) problems.push(`exit ${r.exit}, expected ${want}`);
  if (routine && (!r.rawOut.equals(Buffer.from(routine.stdout)) || !r.rawErr.equals(Buffer.from(routine.stderr)))) problems.push('output differs from the exact routine branch witness');
  return problems;
}

// ---- Order -----------------------------------------------------------------------------
const needs = new Map();
const errors = [];
const addNeeds = (id, from) => {
  if (needs.has(id)) return;
  if (!['run.sh', 'run.ps1'].some((name) => fs.existsSync(path.join(root, id, name)))) { errors.push(`${from}: needs ${id}, which is not an example`); return; }
  if (!ids.includes(id)) ids.push(id); // a named example still runs what it needs
  needs.set(id, list(file(id, 'needs')));
  for (const n of needs.get(id)) addNeeds(n, id);
};
for (const id of [...ids]) addNeeds(id);
if (errors.length) { console.error(errors.join('\n')); process.exit(1); }
// Dependencies are part of the execution set too. Refuse the entire plan before
// any example can ask the recorder or reach a build-capable fallback.
if (!check) for (const id of ids.filter(needsPreparation)) {
  if (!preparedDir || !preparedIds.has(id) || !fs.existsSync(path.join(preparedDir, id, 'run.sh'))) throw new Error(`${id}: prepared executable is required; compilation is disabled`);
}

if (capture || replay) {
  const withheld = ids.filter((id) => (product.exampleWithheldCaptures ?? []).includes(id));
  if (withheld.length) throw new Error(`${withheld.join(', ')}: capture withheld for stand-in example`);
}
const first = new Set();
const visiting = new Set();
const addFirst = (id) => {
  if (first.has(id)) return;
  if (visiting.has(id)) { console.error(`examples: needs files form a cycle through ${id}`); process.exit(1); }
  visiting.add(id);
  for (const n of needs.get(id)) addFirst(n);
  first.add(id);
};
const script = (id) => fs.readdirSync(path.join(root, id)).filter((f) => f === 'run.sh' || f === 'run.ps1' || f.startsWith('show.')).map((f) => file(id, f)).join('\n');
for (const id of ids) for (const n of needs.get(id)) addFirst(n);
if (product.exampleFirst) for (const id of ids) if (new RegExp(product.exampleFirst).test(script(id))) addFirst(id);

// ---- Run -------------------------------------------------------------------------------
const failures = [];
const failed = new Set();
let ran = 0;
const capturedAnswers = new Map();
async function one(id) {
  const blocked = needs.get(id).filter((n) => failed.has(n));
  if (blocked.length) { failed.add(id); failures.push(`${id}: not run; it needs ${blocked.join(', ')}, which failed`); return; }
  const servedBefore = transport?.served() ?? 0;
  const local = !id.startsWith('functions/') && (product.exampleLocalCaptures ?? []).includes(id);
  let r;
  let problems;
  // Live sources limit the rate of keyless requests; product.exampleRetries reruns a failed example after a pause.
  for (let attempt = 0; ; attempt++) {
    r = await run(id);
    problems = judge(id, r);
    if (!problems.length || attempt >= (product.exampleRetries ?? 0)) break;
    console.log(`retry ${id}: ${problems[0]}`);
    await new Promise((ok) => setTimeout(ok, product.exampleRetryDelayMs ?? 15000));
  }
  ran++;
  if (transport?.failure()) problems.push(transport.failure());
  const served = (transport?.served() ?? 0) - servedBefore;
  if (!check) {
    if ((product.exampleWithheldCaptures ?? []).includes(id)) problems.push('capture withheld for stand-in example');
    if (!local && served === 0) problems.push('no successful capture transport field for this example');
    if (local && served !== 0) problems.push('local example used the capture transport');
    const code = product.exampleLocalCaptureCodes?.[id];
    if (local && code && !(r.out + r.err).includes(code)) problems.push(`local example did not report ${code}`);
    const notice = product.exampleLocalCaptureNotices?.[id];
    if (local && notice && !(r.out + r.err).includes(notice)) problems.push(`local example did not report ${notice}`);
  }
  if (id.startsWith('functions/')) {
    const fn = id.split('/')[1];
    let answers;
    try {
      answers = JSON.parse(r.answers);
      if (!Array.isArray(answers) || answers.length === 0) throw new Error();
    } catch { problems.push('missing or invalid fresh semantic answers'); }
    if (id.endsWith('/command')) {
      if (!problems.length) capturedAnswers.set(fn, answers);
    } else if (!capturedAnswers.has(fn)) {
      problems.push('no successful command answers from this session');
    } else if (!isDeepStrictEqual(answers, capturedAnswers.get(fn))) {
      problems.push(check ? 'answers differ from routine command case' : 'answers differ from captured command case');
    }
  }
  const removedNotice = !check && !problems.length && fs.existsSync(path.join(root, id, 'unverified'));
  if (check) {
    fs.mkdirSync(path.join(runs, id), { recursive: true });
    fs.writeFileSync(path.join(runs, id, 'output.txt'), r.rawOut);
    fs.writeFileSync(path.join(runs, id, 'stderr.txt'), r.rawErr);
    if (r.answers !== null) fs.writeFileSync(path.join(runs, id, 'answers.json'), r.answers);
    fs.writeFileSync(path.join(runs, id, 'exit'), `${r.exit}\n`);
  } else if (!check && !problems.length) {
    // A failed run never replaces the saved one.
    fs.writeFileSync(path.join(root, id, 'output.txt'), r.rawOut);
    fs.writeFileSync(path.join(root, id, 'stderr.txt'), r.rawErr);
    fs.writeFileSync(path.join(root, id, 'exit'), `${r.exit}\n`);
    fs.writeFileSync(path.join(root, id, 'ran'), `${today}\n`);
    const provenance = local ? { backend: 'local', date: today } : { backend: 'systemone', answers: served, date: today };
    fs.writeFileSync(path.join(root, id, 'capture.json'), JSON.stringify(provenance) + '\n');
    if (fs.existsSync(path.join(root, id, 'unverified'))) fs.unlinkSync(path.join(root, id, 'unverified'));
  }
  if (!check) {
    try { retainExample(id, r, problems, removedNotice); }
    catch { problems.push('could not retain capture/replay artifacts'); }
  }
  if (problems.length) {
    failed.add(id);
    failures.push(`${id}: ${problems.join('; ')}\n${r.out.slice(0, 400)}`);
  }
  console.log(`${problems.length ? 'FAIL' : 'ok  '} ${r.exit === 0 ? '      ' : `exit ${String(r.exit).padEnd(2)}`} ${id}`);
}

if (check) fs.rmSync(runs, { recursive: true, force: true });
const commands = ids.filter((id) => id.startsWith('functions/') && id.endsWith('/command'));
for (const id of commands) await one(id);
for (const id of first) if (!commands.includes(id)) await one(id);
const rest = ids.filter((id) => !first.has(id) && !commands.includes(id));
let next = 0;
async function worker() {
  while (next < rest.length) {
    if ((capture || replay) && failures.length) break;
    await one(rest[next++]);
  }
}
await Promise.all(Array.from({ length: (capture || replay) ? 1 : (product.exampleWorkers ?? 4) }, worker));
if (transport) { await transport.close(); console.log('Capture transport:', transport.usage()); }
if (failures.length) {
  console.error(`\n${failures.length} example(s) failed:\n` + failures.join('\n'));
  process.exit(1);
}
console.log(`${ran} example(s) ${check ? 'checked' : 'ran'} with ${product.binary} ${info.version}${info.commit ? ` (commit ${info.commit.slice(0, 12)})` : ''}`);
