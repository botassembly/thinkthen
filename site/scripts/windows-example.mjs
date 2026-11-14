// Native Windows leaf for docs capture and final-command qualification.
import { spawn, execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';
import { captureBackend } from './capture-backend.mjs';

const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const powershellString = (value) => `'${value.replaceAll("'", "''")}'`;
const required = (configuration, name) => {
  const value = configuration[name];
  if (!value || /[\r\n"]/.test(value)) throw new Error(`Set ${name} for native Windows examples`);
  return value;
};

// Stage this module's windows-example.py and scripts/release_windows_cache.py in
// their repository-relative locations. Binary source/hash come from its build,
// not from an installed release guessed by the capture runner.
export async function runWindowsExample({ id, cwd, env, timeoutMs = 300_000, configuration = process.env, native = false }) {
  const host = native ? null : required(configuration, 'SITE_WINDOWS_HOST');
  const helper = native ? fileURLToPath(new URL('windows-example.py', import.meta.url)) : required(configuration, 'SITE_WINDOWS_HELPER');
  const binary = required(configuration, 'SITE_WINDOWS_BINARY');
  const binaryHash = required(configuration, 'SITE_WINDOWS_BINARY_SHA256');
  const binarySource = required(configuration, 'SITE_WINDOWS_BINARY_SOURCE');
  if (!/^[a-f0-9]{64}$/.test(binaryHash) || !/^[a-f0-9]{40}$/.test(binarySource)) throw new Error('Windows binary needs its build source and SHA-256');
  const recorder = new URL(env.THINKTHEN_BASE_URL);
  if (recorder.protocol !== 'http:' || recorder.hostname !== '127.0.0.1' || !recorder.port || recorder.pathname !== '/') throw new Error('Windows examples require the existing loopback recorder URL without a path');
  if (env.THINKTHEN_BACKEND !== 'systemone' || env.THINKTHEN_API_KEY !== 'docs-loopback') throw new Error('Windows examples receive only the recorder backend and dummy key');
  const programs = Object.fromEntries(['run.ps1', 'show.ps1'].map((name) => {
    const bytes = fs.readFileSync(path.join(cwd, name));
    return [name, { bytes: bytes.toString('base64'), sha256: hash(bytes) }];
  }));
  const repository = fileURLToPath(new URL('../..', import.meta.url));
  const source = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repository, encoding: 'utf8' }).trim();
  const helperHash = hash(fs.readFileSync(new URL('windows-example.py', import.meta.url)));
  const cacheHelperHash = hash(fs.readFileSync(path.join(repository, 'scripts/release_windows_cache.py')));
  const python = configuration.SITE_WINDOWS_PYTHON ?? 'python';
  if (/[\r\n"]/.test(python)) throw new Error('Invalid Windows Python path');
  const script = `& ${powershellString(python)} -I ${powershellString(helper)}; exit $LASTEXITCODE`;
  const command = `powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -EncodedCommand ${Buffer.from(script, 'utf16le').toString('base64')}`;
  const args = ['-T', '-o', 'BatchMode=yes', '-o', 'ExitOnForwardFailure=yes', '-o', 'LogLevel=INFO', '-p', configuration.SITE_WINDOWS_PORT ?? '22', '-R', `127.0.0.1:0:127.0.0.1:${recorder.port}`, host, command];
  const nativeNames = new Set(['PATH', 'SYSTEMROOT', 'WINDIR', 'COMSPEC', 'PATHEXT', 'TEMP', 'TMP', 'USERPROFILE', 'APPDATA']);
  const nativeEnv = Object.fromEntries(Object.entries(configuration).filter(([key]) => nativeNames.has(key.toUpperCase())));
  const sshEnv = Object.fromEntries(['PATH', 'HOME', 'USER', 'LOGNAME', 'SSH_AUTH_SOCK', 'TMPDIR'].filter((key) => configuration[key] !== undefined).map((key) => [key, configuration[key]]));
  return await new Promise((resolve, reject) => {
    const child = native
      ? spawn(python, ['-I', helper], { env: nativeEnv, stdio: ['pipe', 'pipe', 'pipe'] })
      : spawn(configuration.SITE_WINDOWS_SSH ?? 'ssh', args, { env: sshEnv, stdio: ['pipe', 'pipe', 'pipe'] });
    const stdout = [];
    let stderr = '';
    let ready = native;
    const send = (recorderUrl) => child.stdin.write(JSON.stringify({ id, source, programs, binary, binaryHash, binarySource, helperHash, cacheHelperHash, timeoutMs,
      recorderUrl, model: env.THINKTHEN_MODEL ?? null }) + '\n');
    if (native) send(recorder.origin);
    let failure;
    const timer = setTimeout(() => {
      failure = new Error(ready ? 'Windows capture transport timed out' : 'Windows recorder forwarding did not become ready');
      child.kill('SIGKILL');
    }, timeoutMs + 60_000);
    child.stdout.on('data', (bytes) => stdout.push(bytes));
    child.stderr.on('data', (bytes) => {
      stderr = (stderr + bytes.toString('utf8')).slice(-16_384);
      const allocated = stderr.match(/Allocated port (\d+) for remote forward/);
      if (!ready && allocated) {
        ready = true;
        send(`http://127.0.0.1:${allocated[1]}`);
      }
    });
    child.stdin.on('error', () => { /* The child's terminal status owns a closed input pipe. */ });
    child.on('error', (error) => { failure = error; });
    child.on('close', (status) => {
      clearTimeout(timer);
      if (failure || status !== 0 || !ready) return reject(failure ?? new Error(`Windows capture transport failed (${status}): ${stderr.trim()}`));
      try {
        const result = JSON.parse(Buffer.concat(stdout).toString('utf8'));
        if (!Number.isInteger(result.exit) || typeof result.stdout !== 'string' || typeof result.stderr !== 'string') throw new Error('Invalid Windows capture response');
        const rawOut = Buffer.from(result.stdout, 'base64');
        const rawErr = Buffer.from(result.stderr, 'base64');
        resolve({ id, rawOut, rawErr, out: rawOut.toString('utf8').replace(/[ \t]+$/gm, '').replace(/\s+$/, '') + '\n',
          err: rawErr.toString('utf8').trimEnd() ? `${rawErr.toString('utf8').trimEnd()}\n` : '',
          exit: result.exit, timedOut: result.timedOut, answers: null, provenance: result.provenance });
      } catch (error) { reject(error); }
    });
  });
}

// Qualification consumes the recorded real cache; it never captures on a miss.
export async function qualifyWindowsExample({ binary, source, cache, evidence, configuration = process.env }) {
  const repository = fileURLToPath(new URL('../..', import.meta.url));
  const id = 'start/decide-branch-ps';
  const cwd = path.join(repository, 'site/examples', id);
  const expected = path.join(repository, 'site/examples/start/decide-branch');
  const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repository, encoding: 'utf8' }).trim();
  if (source !== head) throw new Error('Windows replay source differs from the candidate source');
  const capture = JSON.parse(fs.readFileSync(path.join(expected, 'capture.json'), 'utf8'));
  if (capture.backend !== 'systemone' || !(capture.answers > 0)) throw new Error('Windows replay requires a recorded real capture');
  const bytes = fs.readFileSync(cache); // Missing real data is a qualification failure.
  fs.mkdirSync(evidence, { recursive: true });
  const replay = path.join(evidence, 'replay-cache.json');
  fs.writeFileSync(replay, bytes, { mode: 0o600 });
  const recorder = await captureBackend({ file: replay, replayOnly: true });
  let result;
  try {
    result = await runWindowsExample({ id, cwd, native: true, configuration: {
      ...configuration, SITE_WINDOWS_BINARY: path.resolve(binary), SITE_WINDOWS_BINARY_SOURCE: source,
      SITE_WINDOWS_BINARY_SHA256: hash(fs.readFileSync(binary)),
    }, env: { THINKTHEN_BASE_URL: recorder.url, THINKTHEN_BACKEND: 'systemone', THINKTHEN_API_KEY: 'docs-loopback' } });
    fs.writeFileSync(path.join(evidence, 'stdout.txt'), result.rawOut);
    fs.writeFileSync(path.join(evidence, 'stderr.txt'), result.rawErr);
    const normalize = (value) => value.toString('utf8').replaceAll('\r\n', '\n');
    if (result.timedOut || result.exit !== Number(fs.readFileSync(path.join(expected, 'exit'), 'utf8')) ||
        normalize(result.rawOut) !== normalize(fs.readFileSync(path.join(expected, 'output.txt'))) ||
        normalize(result.rawErr) !== normalize(fs.readFileSync(path.join(expected, 'stderr.txt')))) {
      throw new Error('Windows replay output, exit or timeout differs from the recorded capture');
    }
    const usage = recorder.usage();
    if (recorder.failure() || recorder.served() < 1 || usage.calls !== 0 || usage.inputBytes !== 0 || usage.bytes !== 0) {
      throw new Error(`Windows replay failed: ${recorder.failure() ?? 'no cached answer served'}`);
    }
  } finally {
    await recorder.close();
    fs.writeFileSync(path.join(evidence, 'windows-example.json'), JSON.stringify({
      id, source, capture, cache_sha256: hash(bytes), exit: result?.exit, timedOut: result?.timedOut,
      provenance: result?.provenance, served: recorder.served(), ...recorder.usage(), failure: recorder.failure(),
    }, null, 2) + '\n');
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const { values } = parseArgs({ options: Object.fromEntries(['binary', 'source', 'cache', 'evidence'].map((name) => [name, { type: 'string' }])) });
  for (const name of ['binary', 'source', 'cache', 'evidence']) if (!values[name]) throw new Error(`Set --${name} for native Windows replay`);
  await qualifyWindowsExample(values);
}
