// The transport keeps raw Windows streams and sends no real recorder key.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { runWindowsExample } from './windows-example.mjs';

const scratch = fs.mkdtempSync(path.join(os.tmpdir(), 'windows-example-test-'));
try {
  fs.writeFileSync(path.join(scratch, 'run.ps1'), '& ./show.ps1\r\n');
  fs.writeFileSync(path.join(scratch, 'show.ps1'), "'refund'\r\n");
  const ssh = path.join(scratch, 'ssh');
  const requestFile = path.join(scratch, 'request.json');
  fs.writeFileSync(ssh, `#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import readline from 'node:readline';
const command=process.argv.at(-1).split(' ');
assert.deepEqual(command.slice(0,-1),['powershell.exe','-NoProfile','-NonInteractive','-ExecutionPolicy','Bypass','-EncodedCommand']);
assert.equal(Buffer.from(command.at(-1),'base64').toString('utf16le'),${JSON.stringify("& 'C:/Python path/O''Brien/python.exe' -I 'C:/source path/O''Brien/windows-example.py'; exit $LASTEXITCODE")});
process.stderr.write('Allocated port 43127 for remote forward to 127.0.0.1:12345\\n');
const reader = readline.createInterface({input:process.stdin});
const input = await new Promise(resolve => reader.once('line',resolve));
reader.close();
process.stdin.destroy();
fs.writeFileSync(${JSON.stringify(requestFile)}, input);
console.log(JSON.stringify({stdout:Buffer.from('refund\\r\\n').toString('base64'),stderr:Buffer.from('notice\\r\\n').toString('base64'),exit:1,timedOut:false,provenance:{os:'Windows'}}));
`, { mode: 0o700 });
  const configuration = { PATH: process.env.PATH, HOME: scratch, SITE_WINDOWS_HOST: 'fixture', SITE_WINDOWS_HELPER: "C:/source path/O'Brien/windows-example.py",
    SITE_WINDOWS_PYTHON: "C:/Python path/O'Brien/python.exe",
    SITE_WINDOWS_BINARY: 'C:/bin/thinkthen.exe', SITE_WINDOWS_BINARY_SHA256: 'a'.repeat(64), SITE_WINDOWS_BINARY_SOURCE: 'b'.repeat(40),
    SITE_WINDOWS_SSH: ssh, THINKTHEN_API_KEY: 'real-key-must-not-travel' };
  const env = { THINKTHEN_BACKEND: 'systemone', THINKTHEN_BASE_URL: 'http://127.0.0.1:12345', THINKTHEN_API_KEY: 'docs-loopback', OPENAI_API_KEY: 'another-real-key' };
  const result = await runWindowsExample({ id: 'start/ps', cwd: scratch, env, timeoutMs: 1000, configuration });
  assert.deepEqual(result.rawOut, Buffer.from('refund\r\n'));
  assert.deepEqual(result.rawErr, Buffer.from('notice\r\n'));
  assert.equal(result.exit, 1);
  assert.equal(result.timedOut, false);
  assert.equal(result.answers, null);
  const request = JSON.parse(fs.readFileSync(requestFile));
  assert.equal(request.recorderUrl, 'http://127.0.0.1:43127');
  assert.equal(Buffer.from(request.programs['run.ps1'].bytes, 'base64').toString(), '& ./show.ps1\r\n');
  assert(!JSON.stringify(request).includes('real-key'));
  await assert.rejects(runWindowsExample({ id: 'start/ps', cwd: scratch, env: { ...env, THINKTHEN_API_KEY: 'real-key' }, configuration }), /dummy key/);
  console.log('Windows capture transport preserves raw streams, exit and loopback-only inputs');
} finally {
  fs.rmSync(scratch, { recursive: true, force: true });
}
