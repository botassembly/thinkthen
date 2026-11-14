// Exercise the runner's public-output boundary with disposable command consumers.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import product from '../product.config.mjs';
const scratch = fs.mkdtempSync(path.join(os.tmpdir(), 'capture-runner-'));
const artifacts = fs.mkdtempSync(path.join(os.tmpdir(), 'capture-artifacts-'));
const site = path.join(scratch, 'site');
const write = (name, text) => { const file = path.join(site, name); fs.mkdirSync(path.dirname(file), { recursive: true }); fs.writeFileSync(file, text); };
const command = 'functions/decide/command';
const python = 'functions/decide/python';
// Keys are in the transport's canonical order. These are disposable fake bodies.
const request = { model: 'test', questions: { q1: { instructions: 'Refund?', type: 'noul' } }, state: 'Please refund my order.' };
const digest = createHash('sha256').update(JSON.stringify(request)).digest('hex');
const post = `node -e 'fetch(process.env.THINKTHEN_BASE_URL+"/systemone", {method:"POST",body:JSON.stringify(${JSON.stringify(request)})}).then(r=>{if(!r.ok)process.exitCode=1})'\n`;
let artifactIndex = 0;
const execute = (ids, limits = {}) => {
  const artifactDir = path.join(artifacts, String(++artifactIndex));
  const result = spawnSync(process.execPath, ['scripts/run-examples.mjs', '--replay', ...ids], { cwd: site, encoding: 'utf8', env: { PATH: process.env.PATH, SITE_CAPTURE_FILE: path.join(scratch, 'responses.json'), SITE_CAPTURE_ARTIFACT_DIR: artifactDir, ...limits } });
  return { ...result, artifactDir };
};
const run = (...ids) => execute(ids);
try {
  // Run the real branch programs with a failed CLI and a successful later tool.
  // A cache hit with an engine failure must not be hidden by shell control flow.
  const branches = ['start/decide-branch', 'decide-script', 'choose-script',
    'filter-script', 'find-script', 'codes-script', 'howto-ci-ask',
    'howto-labels-ask', 'howto-labels-bar', 'howto-labels-count',
    'howto-match-ask', 'howto-match-person', 'howto-screen-ask',
    'howto-screen-route', 'howto-triage-ask', 'howto-triage-person',
    'howto-triage-route'];
  const fakeBin = path.join(scratch, 'branch-bin');
  fs.mkdirSync(fakeBin);
  fs.writeFileSync(path.join(fakeBin, 'thinkthen'),
    '#!/bin/sh\nprintf "actual command failure\\n" >&2\nexit "$TEST_CLI_EXIT"\n', { mode: 0o755 });
  fs.writeFileSync(path.join(fakeBin, 'jq'), '#!/bin/sh\nexit 0\n', { mode: 0o755 });
  for (const id of branches) {
    const cwd = path.join(scratch, 'branches', id);
    fs.cpSync(path.resolve('examples', id), cwd, { recursive: true });
    for (const code of [2, 4, 5, 6, 7]) {
      const result = spawnSync('bash', ['run.sh'], { cwd, encoding: 'utf8',
        env: { ...process.env, PATH: `${fakeBin}:${process.env.PATH}`, TEST_CLI_EXIT: String(code) } });
      assert.equal(result.status, code, `${id} must retain operational failure ${code}`);
      assert.match(result.stderr, /actual command failure/);
      assert.doesNotMatch(result.stdout, /send to a person|send to the refunds queue|to do:/);
    }
  }
  for (const id of ['start/decide-branch', 'decide-script', 'choose-script',
    'filter-script', 'find-script', 'codes-script', 'howto-ci-ask']) {
    for (const code of [1, 3]) {
      const result = spawnSync('bash', ['run.sh'], {
        cwd: path.join(scratch, 'branches', id), encoding: 'utf8',
        env: { ...process.env, PATH: `${fakeBin}:${process.env.PATH}`, TEST_CLI_EXIT: String(code) } });
      assert.equal(result.status, 0, `${id} retains its reviewed no/not-sure branch`);
      if (['choose-script', 'find-script'].includes(id)) assert.equal(result.stdout, 'send to a person\n');
    }
  }
  for (const id of ['sdks/c/unsure', 'sdks/cpp/unsure',
    'sdks/csharp/first', 'sdks/csharp/unsure', 'sdks/zig/unsure',
    'functions/decide/c', 'functions/decide/cpp', 'functions/decide/zig',
    'functions/decide/typescript']) {
    const result = spawnSync('sh', ['run.sh'], {
      cwd: path.resolve('examples', id), encoding: 'utf8',
      env: { PATH: process.env.PATH, SITE_EXECUTE_ONLY: '1' },
    });
    assert.equal(result.status, 2, `${id} must refuse runtime compilation before tool/package setup`);
    assert.match(result.stderr, /Use a prepared/);
  }
  for (const name of ['scripts/run-examples.mjs', 'scripts/windows-example.mjs', 'scripts/bundle-runtimes.mjs', 'scripts/runtime-examples.py', 'scripts/capture-backend.mjs', 'src/lib/binary.mjs', 'src/lib/json-values.mjs', 'src/lib/examples.mjs', 'src/lib/prepared-examples.mjs', 'src/lib/xlsx.mjs']) write(name, fs.readFileSync(name, 'utf8'));
  write('product.config.mjs', 'export default { binary: "thinkthen", binDir: "../bin", versionArgs: ["--version"], exampleLocalCaptures: ["exit-interrupt", "version", "error-key-missing", "ans-field-piped", "ans-field-piped-missing", "ans-json-on-table"], exampleLocalCaptureCodes: { "error-key-missing": "usage.key", "ans-field-piped": "record.json", "ans-field-piped-missing": "record.missing", "ans-json-on-table": "usage.option" }, exampleLocalCaptureNotices: { "exit-interrupt": "thinkthen: cancelling; press Ctrl-C again to exit" }, exampleKeylessCaptures: ["error-key-missing"], exampleWithheldCaptures: ["backend-wire", "error-reply", "error-status"], exampleRoutineResults: { "routine-refusal": { exit: 1, stdout: "not sure\\n", stderr: "" } }, exampleEnv: { THINKTHEN_BACKEND: "stub" } };');
  write('src/data/site.json', '{"version":"0.2.0"}');
  fs.mkdirSync(path.join(scratch, 'bin'));
  fs.writeFileSync(path.join(scratch, 'bin/thinkthen'), '#!/bin/sh\nprintf "thinkthen 0.2.0\\n"\n', { mode: 0o755 });
  fs.writeFileSync(path.join(scratch, 'responses.json'), '{}');
  const withoutArtifacts = spawnSync(process.execPath, ['scripts/run-examples.mjs', '--replay', 'version'], { cwd: site, encoding: 'utf8', env: { PATH: process.env.PATH } });
  assert.match(withoutArtifacts.stderr, /require an absolute SITE_CAPTURE_ARTIFACT_DIR/);
  const insideCheckout = spawnSync(process.execPath, ['scripts/run-examples.mjs', '--replay', 'version'], { cwd: site, encoding: 'utf8', env: { PATH: process.env.PATH, SITE_CAPTURE_ARTIFACT_DIR: path.join(site, 'artifacts') } });
  assert.match(insideCheckout.stderr, /must be outside the checkout and empty/);
  write('examples/functions/decide/case.json', '{}');
  const fabricated = 'printf "true\\n"\nprintf "[true]" > "$SITE_ANSWERS_FILE"\n';
  const successful = post + fabricated;
  write(`examples/${command}/run.sh`, successful);
  write(`examples/${python}/run.sh`, successful);
  write(`.example-runs/${command}/answers.json`, '[true]');
  assert.match(run(python).stderr, /establish fresh capture answers/, 'saved routine baseline cannot authorize capture');
  write(`examples/${command}/run.sh`, fabricated);
  assert.match(run(command).stderr, /no successful capture transport field/, 'fabricated semantic arrays cannot authorize capture');
  for (const id of ['backend-wire', 'error-reply', 'error-status']) {
    write(`examples/${id}/run.sh`, 'touch executed\nprintf "manufactured HTTP 400\\n"\n');
    write(`examples/${id}/unverified`, 'No real backend output recorded.\n');
    const result = run(id);
    assert.notEqual(result.status, 0, 'a stand-in with an empty response store cannot publish');
    assert.match(result.stderr, /capture withheld for stand-in example/);
    assert.equal(fs.existsSync(path.join(site, `examples/${id}/executed`)), false, 'withheld consumers never execute');
    for (const name of ['output.txt', 'capture.json', 'ran']) assert.equal(fs.existsSync(path.join(site, `examples/${id}/${name}`)), false);
    assert.equal(fs.existsSync(path.join(site, `examples/${id}/unverified`)), true);
  }
  write('examples/withheld-parent/run.sh', 'touch executed\n');
  write('examples/withheld-parent/needs', 'backend-wire\n');
  assert.match(run('withheld-parent').stderr, /capture withheld for stand-in example/);
  assert.equal(fs.existsSync(path.join(site, 'examples/withheld-parent/executed')), false);
  assert.equal(fs.existsSync(path.join(site, 'examples/backend-wire/executed')), false);
  fs.writeFileSync(path.join(scratch, 'responses.json'), JSON.stringify({ [digest]: { request, response: { answers: { q1: { type: 'noul', noul: 0.99 } } } } }));
  for (const id of ['backend-wire', 'error-reply', 'error-status']) {
    write(`examples/${id}/run.sh`, post + 'printf "manufactured HTTP 400\\n"\n');
    const result = run(id);
    assert.doesNotMatch(result.stderr, /no successful capture transport field/, 'withheld consumers are refused before transport execution');
    assert.notEqual(result.status, 0, 'a stand-in cannot publish even when a saved answer is available');
    assert.match(result.stderr, /capture withheld for stand-in example/);
    for (const name of ['output.txt', 'capture.json', 'ran']) assert.equal(fs.existsSync(path.join(site, `examples/${id}/${name}`)), false);
    assert.equal(fs.existsSync(path.join(site, `examples/${id}/unverified`)), true);
  }
  write(`examples/${command}/run.sh`, post + 'printf "true\\n"\n');
  assert.match(run(command).stderr, /invalid fresh semantic answers/);
  assert.equal(fs.existsSync(path.join(site, `examples/${command}/output.txt`)), false, 'missing semantic answers never publish');
  write(`examples/${command}/run.sh`, successful);
  write(`examples/${python}/run.sh`, post + 'printf "false\\n"\nprintf "[false]" > "$SITE_ANSWERS_FILE"\n');
  assert.match(run(command, python).stderr, /answers differ from captured command case/);
  assert.equal(fs.existsSync(path.join(site, `examples/${python}/output.txt`)), false, 'differing answers never publish');
  write(`examples/${python}/run.sh`, successful);
  assert.equal(run(command, python).status, 0, 'fresh agreeing cells publish');
  assert.equal(fs.readFileSync(path.join(site, `examples/${python}/output.txt`), 'utf8'), 'true\n');
  const routine = (...ids) => spawnSync(process.execPath, ['scripts/run-examples.mjs', '--check', ...ids], { cwd: site, encoding: 'utf8', env: { PATH: process.env.PATH } });
  write('examples/routine-refusal/run.sh', 'printf "not sure\\n"\nexit 1\n');
  assert.equal(routine('routine-refusal').status, 0, 'the exact offline refusal branch is exercised');
  assert.match(run('routine-refusal').stderr, /exit 1, expected 0/, 'offline branch witness cannot authorize public capture');
  write('examples/routine-refusal/run.sh', 'printf "wrong branch\\n"\nexit 1\n');
  assert.match(routine('routine-refusal').stderr, /exact routine branch witness/, 'an arbitrary exit one does not prove the branch');
  write('examples/routine-refusal/run.sh', 'printf "not sure\\n"\nexit 2\n');
  assert.match(routine('routine-refusal').stderr, /exit 2, expected 1/, 'operational failure is not an uncertainty witness');
  write(`examples/${command}/run.sh`, fabricated);
  write(`examples/${python}/run.sh`, 'printf "false\\n"\nprintf "[false]" > "$SITE_ANSWERS_FILE"\n');
  assert.match(routine(python).stderr, /answers differ from routine command case/, 'a scoped routine run adds a fresh command baseline and rejects disagreement');
  write(`examples/${python}/run.sh`, fabricated);
  write(`examples/${python}/output.txt`, 'saved real answer differs\n');
  assert.equal(routine(python).status, 0, 'routine agreement never compares with saved real output');
  write(`examples/${python}/run.sh`, 'printf "true\\n"\n');
  assert.match(routine(python).stderr, /invalid fresh semantic answers/, 'routine cells require actual semantic JSON');
  write(`examples/${command}/run.sh`, successful);
  write(`examples/${python}/run.sh`, successful);
  write(`examples/${command}/run.sh`, `node -e 'fetch(process.env.THINKTHEN_BASE_URL+"/systemone", {method:"POST",body:"{"})'\n${successful}`);
  assert.match(run(command).stderr, /capture.*failed/i, 'a swallowed transport error still fails capture');
  write('examples/ordinary/run.sh', post + 'printf "backend output\\n"\n');
  write('examples/ordinary/unverified', 'No real backend output recorded.\n');
  const ordinaryReplay = execute(['ordinary'], { SITE_CAPTURE_MAX_CALLS: '99', SITE_CAPTURE_MAX_INPUT_BYTES: '1', SITE_CAPTURE_MAX_BYTES: '300000' });
  assert.equal(ordinaryReplay.status, 0, 'cached replies consume no paid input allowance');
  const replayManifest = JSON.parse(fs.readFileSync(path.join(ordinaryReplay.artifactDir, 'manifest.json')));
  assert.equal(replayManifest.inputBytes, 0);
  assert.equal(replayManifest.calls, 0);
  const invalidInputLimit = execute(['ordinary'], { SITE_CAPTURE_MAX_INPUT_BYTES: '0' });
  assert.notEqual(invalidInputLimit.status, 0);
  assert.match(invalidInputLimit.stderr, /capture limits must be positive integers/, 'runner passes the configured input limit to transport');
  assert.equal(JSON.parse(fs.readFileSync(path.join(site, 'examples/ordinary/capture.json'))).backend, 'systemone');
  assert.equal(JSON.parse(fs.readFileSync(path.join(site, 'examples/ordinary/capture.json'))).answers, 1);
  const backendMetadata = JSON.parse(fs.readFileSync(path.join(site, 'examples/ordinary/capture.json')));
  assert.deepEqual(Object.keys(backendMetadata).sort(), ['answers', 'backend', 'date'], 'public backend metadata contains no filesystem path');
  assert.equal(fs.existsSync(path.join(site, 'examples/ordinary/unverified')), false);
  assert.equal(fs.readFileSync(path.join(site, 'examples/ordinary/output.txt'), 'utf8'), 'backend output\n');
  // Captures retain terminal whitespace, blank lines and CRLF byte-for-byte,
  // both in the public files and in the durable handoff outside the checkout.
  const rawOutput = ' answer \t\r\n\n';
  const rawError = ' diagnostic \t\r\n\n';
  write('examples/exact-streams/run.sh', post + 'printf " answer \\t\\r\\n\\n"\nprintf " diagnostic \\t\\r\\n\\n" >&2\n');
  const exact = run('exact-streams');
  assert.equal(exact.status, 0, exact.stderr);
  for (const [name, expected] of [['output.txt', rawOutput], ['stderr.txt', rawError]]) {
    assert.deepEqual(fs.readFileSync(path.join(site, 'examples/exact-streams', name)), Buffer.from(expected));
    assert.deepEqual(fs.readFileSync(path.join(exact.artifactDir, 'site/examples/exact-streams', name)), Buffer.from(expected));
  }
  write('examples/zz-no-contact/run.sh', 'printf "fabricated backend output\\n"\n');
  assert.match(run('ordinary', 'zz-no-contact').stderr, /no successful capture transport field/, 'a previous example in the same session cannot supply transport field');
  assert.equal(fs.existsSync(path.join(site, 'examples/zz-no-contact/output.txt')), false);
  write('examples/version/run.sh', 'thinkthen --version\n');
  write('examples/version/unverified', 'No local run recorded.\n');
  assert.equal(run('version').status, 0, 'a reviewed pure local command needs no backend');
  const local = JSON.parse(fs.readFileSync(path.join(site, 'examples/version/capture.json')));
  assert.equal(local.backend, 'local');
  assert.equal(local.responses, undefined);
  assert.equal(fs.readFileSync(path.join(site, 'examples/version/output.txt'), 'utf8'), 'thinkthen 0.2.0\n');
  assert.equal(fs.existsSync(path.join(site, 'examples/version/unverified')), false);
  write('examples/version/run.sh', post + 'thinkthen --version\n');
  assert.match(run('version').stderr, /local example used the capture transport/);
  write('examples/error-key-missing/exit', '2\n');
  write('examples/error-key-missing/run.sh', `node -e 'if(Object.keys(process.env).some(k=>/(_API_KEY|_TOKEN|_SECRET|_PASSWORD)$/.test(k))) process.exit(9)'
printf 'usage.key: missing key\n' >&2
exit 2
`);
  assert.equal(run('error-key-missing').status, 0, 'missing-key capture strips even the injected recorder key');
  assert.equal(JSON.parse(fs.readFileSync(path.join(site, 'examples/error-key-missing/capture.json'))).backend, 'local');
  write('examples/error-key-missing/run.sh', 'printf "wrong.code\n" >&2\nexit 2\n');
  assert.match(run('error-key-missing').stderr, /local example did not report usage.key/);
  write('examples/error-key-missing/run.sh', 'printf "usage.key\n" >&2\n');
  assert.match(run('error-key-missing').stderr, /exit 0, expected 2/);
  write('examples/error-key-missing/run.sh', post + 'printf "usage.key\n" >&2\nexit 2\n');
  assert.match(run('error-key-missing').stderr, /local example used the capture transport/);
  for (const [id, code, wrong] of [['ans-field-piped', 'record.json', 'record.missing'], ['ans-field-piped-missing', 'record.missing', 'record.json']]) {
    write(`examples/${id}/exit`, '6\n');
    write(`examples/${id}/run.sh`, `printf '${code}\n'\nexit 6\n`);
    assert.equal(run(id).status, 0, `${id} requires its distinct reviewed record code`);
    write(`examples/${id}/run.sh`, `printf '${wrong}\n'\nexit 6\n`);
    assert.match(run(id).stderr, new RegExp(`local example did not report ${code}`));
  }
  write('examples/ans-json-on-table/exit', '2\n');
  write('examples/ans-json-on-table/run.sh', 'printf "usage.option: json: expected text output for table framing\\n" >&2\nexit 2\n');
  assert.equal(run('ans-json-on-table').status, 0, 'table JSON usage fails locally before backend setup');
  write('examples/ans-json-on-table/run.sh', 'printf "usage.question\\n" >&2\nexit 2\n');
  assert.match(run('ans-json-on-table').stderr, /local example did not report usage.option/);
  write('examples/ans-json-on-table/run.sh', post + 'printf "usage.option\\n" >&2\nexit 2\n');
  assert.match(run('ans-json-on-table').stderr, /local example used the capture transport/);
  write('examples/mixed-output-exists/exit', '2\n');
  write('examples/mixed-output-exists/run.sh', post + 'printf "usage.option: output already exists\\n" >&2\nexit 2\n');
  assert.equal(run('mixed-output-exists').status, 0, 'backend setup before a local usage failure remains a backend example');
  write('examples/mixed-output-exists/run.sh', 'printf "usage.option: output already exists\\n" >&2\nexit 2\n');
  assert.match(run('mixed-output-exists').stderr, /no successful capture transport field/);
  write('examples/exit-interrupt/exit', '130\n');
  write('examples/exit-interrupt/run.sh', 'printf "thinkthen: cancelling; press Ctrl-C again to exit\\n" >&2\nexit 130\n');
  assert.equal(run('exit-interrupt').status, 0, 'cancellation notice and exit130 need no error code');
  write('examples/exit-interrupt/run.sh', 'printf "local.cancelled\\n" >&2\nexit 130\n');
  assert.match(run('exit-interrupt').stderr, /local example did not report thinkthen: cancelling/);
  write('scripts/source-job.sh', fs.readFileSync('scripts/source-job.sh', 'utf8'));
  write('scripts/node-env.sh', ':\n');
  fs.mkdirSync(path.join(scratch, 'scripts'));
  fs.writeFileSync(path.join(scratch, 'scripts/native-tools-env'), 'test -z "${SITE_RUNTIME_BUNDLE:-}" || { touch build-env-entered; exit 99; }\n');
  const generator = path.join(scratch, 'scripts/jvm-generated');
  fs.writeFileSync(generator, 'first generator\n');
  const git = (...args) => {
    const result = spawnSync('git', ['-c', 'user.name=Test', '-c', 'user.email=test@example.invalid', ...args], { cwd: scratch, encoding: 'utf8' });
    assert.equal(result.status, 0, result.stderr);
    return result.stdout.trim();
  };
  git('init', '-q');
  git('add', '.');
  git('commit', '-qm', 'Initial fixture');
  const stagedSource = git('rev-parse', 'HEAD');
  const packages = path.join(artifacts, 'packages');
  fs.mkdirSync(packages);
  fs.mkdirSync(path.join(packages, 'python'));
  fs.writeFileSync(path.join(packages, 'python/immutable_probe.py'), 'value = "imported"\n');
  const quoteShell = (value) => "'" + value.replaceAll("'", "'\\''") + "'";
  const importCode = 'import sys; sys.path.insert(0, ' + JSON.stringify(path.join(packages, 'python')) + '); import immutable_probe; print(immutable_probe.value)';
  write('examples/python-import/run.sh', post + 'python3 -c ' + quoteShell(importCode) + '\n');
  const reuse = (prefix = packages) => spawnSync('sh', ['scripts/source-job.sh', 'routine', 'version'], { cwd: site, encoding: 'utf8', env: { ...process.env, SITE_PACKAGES: prefix } });
  for (const prefix of [packages, `${packages}-missing`])
    assert.match(reuse(prefix).stderr, /site-packages is incomplete/, 'explicit unmarked package prefixes never fall back to a CLI build');
  const localPackages = path.join(scratch, 'target/site-packages');
  fs.mkdirSync(localPackages, { recursive: true });
  const noPrefix = Object.fromEntries(Object.entries(process.env).filter(([name]) => name !== 'SITE_PACKAGES'));
  for (const mode of ['routine', 'capture', 'replay']) {
    const localPartial = spawnSync('sh', ['scripts/source-job.sh', mode, 'version'], { cwd: site, encoding: 'utf8', env: noPrefix });
    assert.match(localPartial.stderr, /site-packages is incomplete/, `${mode} never consumes an unfinished default package tree`);
  }
  const mockTools = path.join(scratch, 'mock-tools');
  fs.mkdirSync(mockTools);
  fs.writeFileSync(path.join(mockTools, 'cargo'), '#!/bin/sh\necho "CLI-only build reached" >&2\nexit 43\n', { mode: 0o755 });
  const cliOnly = (mode) => spawnSync('sh', ['scripts/source-job.sh', mode, 'version'], { cwd: site, encoding: 'utf8', env: { ...noPrefix, PATH: `${mockTools}:${process.env.PATH}` } });
  assert.equal(cliOnly('reference').status, 43, 'reference-only generation can build its CLI despite a partial local SDK tree');
  fs.rmdirSync(localPackages);
  assert.equal(cliOnly('routine').status, 43, 'a no-prefix CLI-only routine still reaches its CLI build');
  fs.writeFileSync(path.join(packages, 'source-commit'), `${stagedSource}\n`);
  write('examples/version/run.sh', 'thinkthen --version\n');
  git('add', 'site/examples/version/run.sh');
  git('commit', '-qm', 'Change only the consumer');
  assert.equal(reuse().status, 0, 'complete packages survive a runner-only change');
  for (const name of ['scripts/release_install.py', 'sdk/cpp/tests/check-installed', 'extensions/sqlite/tests/run.py']) {
    const destination = path.join(scratch, name);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, 'changed non-staging documentation checker\n');
    git('add', name);
  }
  git('commit', '-qm', 'Change reviewed non-staging helpers');
  assert.equal(reuse().status, 0, 'the three exact reviewed non-staging inputs preserve prefix reuse');
  assert.equal(fs.realpathSync(path.join(scratch, 'target/site-packages')), packages, 'old consumers see only the validated prefix');
  fs.unlinkSync(path.join(scratch, 'target/site-packages'));
  fs.mkdirSync(path.join(scratch, 'target/site-packages'));
  assert.match(reuse().stderr, /conflicts with the validated package prefix/, 'conflicting local packages are never overwritten');
  fs.writeFileSync(generator, 'changed generator\n');
  git('add', 'scripts/jvm-generated');
  git('commit', '-qm', 'Change a package generator');
  assert.match(reuse().stderr, /Prepared packages differ from this source/, 'package generator changes invalidate reuse before consumers run');
  write('examples/lang-go/show.go', 'package main\n');
  write('examples/lang-go/run.sh', 'touch compiler-executed\nexit 99\n');
  write('examples/lang-go/unverified', 'No real output recorded.\n');
  git('add', '.');
  git('commit', '-qm', 'Bind the shown compiled example');
  const preparedDir = path.join(artifacts, 'prepared');
  fs.mkdirSync(path.join(preparedDir, 'lang-go'), { recursive: true });
  const ready = path.join(preparedDir, 'lang-go/run.sh');
  fs.writeFileSync(ready, post + 'printf "ready program\\n"\n');
  fs.writeFileSync(path.join(preparedDir, 'source-commit'), git('rev-parse', 'HEAD') + '\n');
  const prepared = { source: git('rev-parse', 'HEAD'), ids: ['lang-go'], links: {}, files: Object.fromEntries(['lang-go/run.sh', 'source-commit'].map((name) => [name, createHash('sha256').update(fs.readFileSync(path.join(preparedDir, name))).digest('hex')])) };
  const savePrepared = () => fs.writeFileSync(path.join(preparedDir, 'manifest.json'), JSON.stringify(prepared));
  savePrepared();
  fs.mkdirSync(path.join(packages, 'bin'));
  fs.copyFileSync(path.join(scratch, 'bin/thinkthen'), path.join(packages, 'bin/thinkthen'));
  const runtimeRoots = {};
  const runtimeSlices = {
    java: ['bin/java', 'lib/modules', 'conf/security', 'release', 'bin/javac'],
    dotnet: ['dotnet', 'host/fxr/runtime.so', 'shared/runtime.so', 'sdk/compiler'],
    python: ['bin/python3', 'lib/python.py'], ruby: ['bin/ruby', 'lib/ruby.rb'],
    node: ['bin/node', 'bin/npm'], swift: ['libswiftCore.so'],
  };
  for (const [name, files] of Object.entries(runtimeSlices)) {
    const root = path.join(artifacts, 'runtime-sources', name);
    for (const file of files) {
      fs.mkdirSync(path.dirname(path.join(root, file)), { recursive: true });
      fs.writeFileSync(path.join(root, file), `literal ${name}/${file}\n`);
    }
    runtimeRoots[name] = { root, ...(name === 'swift' ? {} : { executable: path.join(root,
      name === 'dotnet' ? 'dotnet' : `bin/${name === 'java' ? 'java' : name === 'python' ? 'python3' : name}`) }) };
  }
  const rubyLoadPaths = ['lib/ruby/site_ruby', 'lib/ruby/3.3.0/architecture', 'lib/ruby/3.3.0']
    .map((directory) => path.join(runtimeRoots.ruby.root, directory));
  const fakeRuby = (paths) => {
    fs.writeFileSync(runtimeRoots.ruby.executable,
      `#!/bin/sh\nprintf '%s\\n' '${JSON.stringify(paths)}'\n`);
    fs.chmodSync(runtimeRoots.ruby.executable, 0o755);
  };
  fakeRuby(rubyLoadPaths);
  const runtimeFile = path.join(packages, 'runtime-roots.json');
  fs.writeFileSync(runtimeFile, JSON.stringify(runtimeRoots));
  const runtimeOut = path.join(artifacts, 'runtime-bundle');
  const makeRuntimeBundle = (out) => spawnSync(process.execPath, ['scripts/bundle-runtimes.mjs', out], {
    cwd: site, encoding: 'utf8', env: { PATH: process.env.PATH, SITE_PACKAGES: packages, SITE_PREPARED_EXAMPLES: preparedDir },
  });
  const selectedPython = spawnSync('python3', ['-c', 'import sys; print(sys.executable)'], { encoding: 'utf8' });
  assert.equal(selectedPython.status, 0, selectedPython.stderr);
  fs.writeFileSync(runtimeRoots.python.executable, '#!/bin/sh\nexec ' + quoteShell(selectedPython.stdout.trim()) + ' "$@"\n');
  fs.chmodSync(runtimeRoots.python.executable, 0o755);
  // These host launchers exercise bundle admission and replay, not portability.
  // Keep Node in its installed layout: macOS builds can use adjacent dylibs.
  fs.writeFileSync(runtimeRoots.node.executable, '#!/bin/sh\nexec ' + quoteShell(process.execPath) + ' "$@"\n');
  fs.chmodSync(runtimeRoots.node.executable, 0o755);
  const beforePrepared = fs.readFileSync(path.join(preparedDir, 'manifest.json'));
  const bundled = makeRuntimeBundle(runtimeOut);
  assert.equal(bundled.status, 0, bundled.stderr);
  const runtimeManifest = JSON.parse(fs.readFileSync(path.join(runtimeOut, 'manifest.json')));
  for (const [file, digest] of Object.entries(runtimeManifest.files)) assert.equal(createHash('sha256').update(fs.readFileSync(path.join(runtimeOut, file))).digest('hex'), digest);
  const runtimeEnvironment = JSON.parse(fs.readFileSync(path.join(runtimeOut, 'environment.json')));
  const expectedRubyLib = rubyLoadPaths.map((directory) => path.join(runtimeManifest.mapped.ruby,
    path.relative(runtimeRoots.ruby.root, directory))).join(path.delimiter);
  assert.equal(runtimeEnvironment.RUBYLIB, expectedRubyLib, 'preserve selected Ruby load-path order under copied root');
  assert.equal(product.exampleSetup({ shell: runtimeEnvironment }).RUBYLIB, expectedRubyLib);
  assert.equal(Object.hasOwn(product.exampleSetup({ shell: {} }), 'RUBYLIB'), false);
  fakeRuby(['/outside/runtime/lib']);
  assert.match(makeRuntimeBundle(path.join(artifacts, 'outside-ruby')).stderr, /Ruby load path is outside its runtime root/);
  fakeRuby(rubyLoadPaths);
  assert.equal(fs.existsSync(path.join(runtimeManifest.mapped.java, 'bin/javac')), false);
  assert.equal(fs.existsSync(path.join(runtimeManifest.mapped.dotnet, 'sdk')), false);
  assert.equal(fs.existsSync(path.join(runtimeManifest.mapped.node, 'bin/npm')), false);
  assert.equal(fs.readFileSync(path.join(packages, 'source-commit'), 'utf8'), `${stagedSource}\n`);
  assert.deepEqual(fs.readFileSync(path.join(preparedDir, 'manifest.json')), beforePrepared);
  const runtimeRun = (...ids) => spawnSync('sh', ['scripts/source-job.sh', 'replay', ...ids], {
    cwd: site, encoding: 'utf8', env: { PATH: process.env.PATH,
      SITE_RUNTIME_BUNDLE: runtimeOut, SITE_CAPTURE_FILE: path.join(scratch, 'responses.json'),
      SITE_CAPTURE_ARTIFACT_DIR: path.join(artifacts, String(++artifactIndex)) },
  });
  const copiedRun = runtimeRun('lang-go', 'version', 'python-import');
  assert.equal(copiedRun.status, 0, copiedRun.stderr);
  assert.equal(fs.existsSync(path.join(packages, 'python/__pycache__')), false, 'real imports never mutate the frozen SDK prefix');
  const copiedReplay = runtimeRun('lang-go', 'version', 'python-import');
  assert.equal(copiedReplay.status, 0, copiedReplay.stderr);
  assert.equal(fs.existsSync(path.join(scratch, 'build-env-entered')), false, 'runtime entry never sources build setup');
  assert.equal(fs.existsSync(path.join(site, 'examples/lang-go/compiler-executed')), false);
  const copiedNode = path.join(runtimeManifest.mapped.node, 'bin/node');
  const nodeBytes = fs.readFileSync(copiedNode);
  fs.appendFileSync(copiedNode, 'changed runtime');
  assert.match(runtimeRun('version').stderr, /Copied runtime bytes or links differ/);
  fs.writeFileSync(copiedNode, nodeBytes);
  const commandBytes = fs.readFileSync(path.join(packages, 'bin/thinkthen'));
  fs.appendFileSync(path.join(packages, 'bin/thinkthen'), 'changed command');
  assert.match(runtimeRun('version').stderr, /Copied package bytes or links differ/);
  fs.writeFileSync(path.join(packages, 'bin/thinkthen'), commandBytes);
  runtimeRoots.node.root = '/usr';
  fs.writeFileSync(runtimeFile, JSON.stringify(runtimeRoots));
  assert.match(makeRuntimeBundle(path.join(artifacts, 'broad-runtime')).stderr, /Refuse broad or missing runtime root/);
  write('examples/aaa-before-compiled/run.sh', post + 'touch request-started\n');
  assert.match(run('aaa-before-compiled', 'lang-go').stderr, /require a prepared executable bundle/);
  assert.equal(fs.existsSync(path.join(site, 'examples/aaa-before-compiled/request-started')), false, 'the entire selected plan is refused before its first backend example');
  assert.match(run('lang-go').stderr, /require a prepared executable bundle/);
  assert.equal(fs.existsSync(path.join(site, 'examples/lang-go/compiler-executed')), false, 'missing bundle never launches a compiler');
  const runPrepared = (...ids) => spawnSync(process.execPath, ['scripts/run-examples.mjs', '--replay', ...ids], { cwd: site, encoding: 'utf8', env: { PATH: process.env.PATH, SITE_PREPARED_EXAMPLES: preparedDir, SITE_CAPTURE_FILE: path.join(scratch, 'responses.json'), SITE_CAPTURE_ARTIFACT_DIR: path.join(artifacts, String(++artifactIndex)) } });
  assert.equal(runPrepared('lang-go', 'version').status, 0, 'ready entry runs while an interpreted/local runner stays unchanged');
  assert.equal(fs.existsSync(path.join(site, 'examples/lang-go/compiler-executed')), false);
  prepared.ids = []; savePrepared();
  assert.match(runPrepared('lang-go').stderr, /prepared executable is required/);
  assert.equal(fs.existsSync(path.join(site, 'examples/lang-go/compiler-executed')), false, 'a missing ready entry never falls back to a compiler');
  prepared.ids = ['lang-go']; savePrepared();
  write('examples/lang-go/show.go', 'changed shown source\n');
  assert.match(runPrepared('lang-go').stderr, /Prepared example source inputs differ/);
  write('examples/lang-go/show.go', 'package main\n');
  const shadow = path.join(preparedDir, 'lang-go/out/node_modules/thinkthen');
  fs.mkdirSync(shadow, { recursive: true });
  fs.writeFileSync(path.join(shadow, 'package.json'), '{"main":"index.js"}');
  fs.writeFileSync(path.join(shadow, 'index.js'), 'throw new Error("unrecorded shadow package")');
  assert.match(runPrepared('lang-go').stderr, /Prepared example inventory differs/, 'an unrecorded closer import cannot shadow a recorded dependency');
  fs.rmSync(path.join(preparedDir, 'lang-go/out'), { recursive: true });
  fs.writeFileSync(ready, 'tampered ready program\n');
  assert.match(runPrepared('lang-go').stderr, /Prepared example bytes differ/);
  write('examples/version/run.sh', 'printf "kept output\\n"\nprintf "kept stderr\\n" >&2\n');
  write('examples/version/unverified', 'No local run recorded.\n');
  write('examples/zz-retained-failure/run.sh', 'printf "failed output \\n\\n"\nprintf "actual failure\\n" >&2\nexit 7\n');
  write('examples/zz-retained-failure/unverified', 'No real backend output recorded.\n');
  const retained = run('version', 'zz-retained-failure');
  assert.equal(retained.status, 1, 'the original failed replay remains failed');
  assert.equal(fs.existsSync(path.join(site, 'examples/zz-retained-failure/output.txt')), false, 'a failed run is not promoted');
  fs.rmSync(site, { recursive: true, force: true }); // Simulate the next job removing its checkout.
  const manifest = JSON.parse(fs.readFileSync(path.join(retained.artifactDir, 'manifest.json')));
  assert.equal(manifest.exit, 1);
  assert.deepEqual(manifest.passed_ids, ['version']);
  assert.deepEqual(manifest.removed_notices, ['site/examples/version/unverified']);
  assert.equal(fs.readFileSync(path.join(retained.artifactDir, 'site/examples/version/output.txt'), 'utf8'), 'kept output\n');
  assert.equal(fs.readFileSync(path.join(retained.artifactDir, 'site/examples/version/stderr.txt'), 'utf8'), 'kept stderr\n');
  assert.equal(fs.readFileSync(path.join(retained.artifactDir, 'failures/zz-retained-failure/output.txt'), 'utf8'), 'failed output \n\n');
  assert.equal(fs.readFileSync(path.join(retained.artifactDir, 'failures/zz-retained-failure/stderr.txt'), 'utf8'), 'actual failure\n');
  assert.equal(manifest.failures[0].exit, 7);
  assert.match(manifest.failures[0].problems.join('; '), /exit 7, expected 0/);
  assert.equal(fs.existsSync(path.join(retained.artifactDir, 'site/examples/zz-retained-failure')), false);
  for (const [name, digest] of Object.entries(manifest.files)) assert.equal(createHash('sha256').update(fs.readFileSync(path.join(retained.artifactDir, name))).digest('hex'), digest);
  assert.equal(Object.keys(manifest.files).some((name) => /responses|cache/.test(name)), false);
  console.log('Capture runner controls pass: per-example transport field, truthful local provenance, fresh answers, agreement, fatal transport errors and durable results');
} finally {
  fs.rmSync(scratch, { recursive: true, force: true });
  fs.rmSync(artifacts, { recursive: true, force: true });
}
