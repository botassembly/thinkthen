// Native Windows leaf for run-examples.mjs. The caller owns the VM and recorder.
import { spawn, execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

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
export async function runWindowsExample({ id, cwd, env, timeoutMs = 300_000, configuration = process.env }) {
  const host = required(configuration, 'SITE_WINDOWS_HOST');
  const helper = required(configuration, 'SITE_WINDOWS_HELPER');
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
  const sshEnv = Object.fromEntries(['PATH', 'HOME', 'USER', 'LOGNAME', 'SSH_AUTH_SOCK', 'TMPDIR'].filter((key) => configuration[key] !== undefined).map((key) => [key, configuration[key]]));
  return await new Promise((resolve, reject) => {
    const child = spawn(configuration.SITE_WINDOWS_SSH ?? 'ssh', args, { env: sshEnv, stdio: ['pipe', 'pipe', 'pipe'] });
    const stdout = [];
    let stderr = '';
    let ready = false;
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
        child.stdin.write(JSON.stringify({ id, source, programs, binary, binaryHash, binarySource, helperHash, cacheHelperHash, timeoutMs,
          recorderUrl: `http://127.0.0.1:${allocated[1]}`, model: env.THINKTHEN_MODEL ?? null }) + '\n');
      }
    });
    child.stdin.on('error', () => { /* SSH's terminal status owns a closed input pipe. */ });
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
