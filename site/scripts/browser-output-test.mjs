// Feature: each browser check publishes only its own reports and screenshots.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const wrapper = fileURLToPath(new URL('./browser-box-job.mjs', import.meta.url));
const scratch = fs.mkdtempSync(path.join(os.tmpdir(), 'browser-output-test-'));
try {
  const site = path.join(scratch, 'site');
  const screenshots = path.join(scratch, 'preview', 'screenshots');
  const write = (name, text) => {
    const file = path.join(site, name);
    fs.mkdirSync(path.dirname(file), { recursive: true });
    fs.writeFileSync(file, text);
  };
  write('../schemas/contract.json', '{}');
  write('../install.sh', '#!/bin/sh\nprintf source-installer\n');
  write('../private-canary', 'not a public browser input');
  write('dist/install.sh', 'different generated installer');
  write('node_modules/yaml/package.json', '{}');
  write('scripts/check-landing.mjs', '// The fake browser never executes the checker.');
  write('public/keep.txt', 'public asset');
  write('keep.txt', 'unowned output');
  write('landing-report.json', '{"old":"PASS"}');
  write('landing-geometry.json', '[{"old":"failure"}]');
  write('landing-shots/after/old-failure.png', 'old screenshot');
  write('compact-access-failure.json', '{"old":"failure"}');
  write('compact-access-failure.png', 'old screenshot');
  fs.mkdirSync(path.join(screenshots, 'after'), { recursive: true });
  fs.writeFileSync(path.join(screenshots, 'after/old-failure.png'), 'old external screenshot');
  fs.writeFileSync(path.join(screenshots, 'keep.txt'), 'unowned external output');
  const browser = path.join(scratch, 'fake-browser');
  fs.writeFileSync(browser, `#!${process.execPath}
import fs from 'node:fs';
import path from 'node:path';
const out = process.argv[process.argv.indexOf('--out') + 1];
const request = JSON.parse(fs.readFileSync(path.join(out, 'browser-request.json')));
for (const name of ['landing-report.json', 'landing-geometry.json', 'landing-shots', 'compact-access-failure.json', 'compact-access-failure.png']) {
  if (fs.existsSync(name)) throw new Error('Previous browser output remains before launch: ' + name);
}
if (fs.readFileSync(path.join(out, 'install.sh'), 'utf8') !== ${JSON.stringify('#!/bin/sh\nprintf source-installer\n')}) throw new Error('Independent installer source missing or changed');
if (fs.readFileSync(path.join(out, 'install.sh')).equals(fs.readFileSync(path.join(out, 'site/dist/install.sh')))) throw new Error('Installer source was taken from generated dist');
if (fs.existsSync(path.join(out, 'private-canary'))) throw new Error('Unapproved repository sibling was copied');
const mode = request.args[0];
const write = (name, text) => {
  const file = path.join(out, name);
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, text);
};
if (mode === 'early-failure') process.exit(9);
write('site/landing-report.json', JSON.stringify({ current: mode }));
if (mode === 'failure') {
  write('site/landing-geometry.json', '[{"current":"failure"}]');
  write('site/compact-access-failure.json', '{"current":"failure"}');
  write('site/compact-access-failure.png', 'current compact failure');
  write('review-shots/after/failure.png', 'current failure');
  process.exit(7);
}
write('review-shots/after/success.png', 'current success');
write('review-shots/install/success.png', 'current installer');
`);
  fs.chmodSync(browser, 0o755);
  const run = (mode, expected) => {
    const result = spawnSync(process.execPath, [wrapper, 'scripts/check-landing.mjs', mode], {
      cwd: site,
      env: { PATH: process.env.PATH, BROWSER_BOX: browser, SITE_LOOK_SHOTS: path.join(screenshots, 'after') },
      encoding: 'utf8',
    });
    assert.equal(result.status, expected, `${mode}: ${result.error ?? result.stderr}`);
  };
  run('failure', 7);
  assert.deepEqual(JSON.parse(fs.readFileSync(path.join(site, 'landing-geometry.json'))), [{ current: 'failure' }]);
  assert.equal(fs.readFileSync(path.join(screenshots, 'after/failure.png'), 'utf8'), 'current failure');
  assert.equal(fs.existsSync(path.join(screenshots, 'after/old-failure.png')), false, 'old external screenshot survived');
  assert.equal(fs.readFileSync(path.join(site, 'compact-access-failure.png'), 'utf8'), 'current compact failure');
  run('PASS', 0);
  assert.deepEqual(JSON.parse(fs.readFileSync(path.join(site, 'landing-report.json'))), { current: 'PASS' });
  for (const name of ['landing-geometry.json', 'compact-access-failure.json', 'compact-access-failure.png', 'landing-shots/after/failure.png']) {
    assert.equal(fs.existsSync(path.join(site, name)), false, `${name} survived a passing run`);
  }
  assert.equal(fs.existsSync(path.join(screenshots, 'after/failure.png')), false, 'external failure screenshot survived PASS');
  assert.equal(fs.readFileSync(path.join(screenshots, 'after/success.png'), 'utf8'), 'current success');
  assert.equal(fs.readFileSync(path.join(screenshots, 'install/success.png'), 'utf8'), 'current installer');
  fs.mkdirSync(path.join(screenshots, 'authored'), { recursive: true });
  fs.writeFileSync(path.join(screenshots, 'authored/old-page.png'), 'old authored screenshot');
  run('early-failure', 9);
  for (const name of ['landing-report.json', 'landing-geometry.json', 'landing-shots']) {
    assert.equal(fs.existsSync(path.join(site, name)), false, `${name} survived an early failure`);
  }
  for (const name of ['after', 'install', 'authored']) {
    assert.equal(fs.existsSync(path.join(screenshots, name)), false, `${name} survived an early failure outside the site`);
  }
  assert.equal(fs.readFileSync(path.join(screenshots, 'keep.txt'), 'utf8'), 'unowned external output');
  assert.equal(fs.readFileSync(path.join(site, 'keep.txt'), 'utf8'), 'unowned output');
  assert.equal(fs.readFileSync(path.join(site, 'public/keep.txt'), 'utf8'), 'public asset');
  console.log('Browser outputs: failure → PASS → early failure replaces owned evidence and preserves exit status');
} finally {
  fs.rmSync(scratch, { recursive: true, force: true });
}
