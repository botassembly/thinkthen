// Supply the existing checks with a disposable copy of public site files, without
// mounting the workspace, credentials, home, package caches or a Docker socket.
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
const [script, ...args] = process.argv.slice(2);
if (!['scripts/check-landing.mjs', 'scripts/social-card.mjs'].includes(script)) throw new Error('Unknown browser task');
const outputs = script === 'scripts/check-landing.mjs'
  ? ['landing-report.json', 'landing-geometry.json', 'landing-shots', 'compact-access-failure.json', 'compact-access-failure.png']
  : [];
// An absent output means this run did not produce it, not that the last run's
// file remains current. Clear only the browser check's owned evidence, before
// setup or launch can fail, then publish this invocation's scratch files.
for (const name of outputs) fs.rmSync(name, { recursive: true, force: true });
const reviewDirectory = outputs.length && process.env.SITE_LOOK_SHOTS
  ? path.dirname(process.env.SITE_LOOK_SHOTS) : null;
if (reviewDirectory) {
  // The checker writes these children beside SITE_LOOK_SHOTS; other files in
  // the caller's preview directory belong to the caller.
  for (const name of ['after', 'install', 'authored']) fs.rmSync(path.join(reviewDirectory, name), { recursive: true, force: true });
}
const scratch = fs.mkdtempSync(path.join(os.tmpdir(), 'thinkthen-browser-'));
try {
  const site = path.join(scratch, 'site');
  fs.mkdirSync(site);
  fs.mkdirSync(path.join(scratch, 'schemas'));
  fs.copyFileSync('../schemas/contract.json', path.join(scratch, 'schemas/contract.json'));
  // The retained installer check compares dist against this independent source.
  // Copy only this public sibling, never the repository or the user's home.
  if (script === 'scripts/check-landing.mjs') fs.copyFileSync('../install.sh', path.join(scratch, 'install.sh'));
  for (const entry of ['src', 'scripts', 'examples', 'dist', 'public', 'questions', 'product.config.mjs', 'package.json']) {
    if (fs.existsSync(entry)) fs.cpSync(entry, path.join(site, entry), { recursive: true });
  }
  fs.mkdirSync(path.join(site, 'node_modules'), { recursive: true });
  fs.cpSync('node_modules/yaml', path.join(site, 'node_modules/yaml'), { recursive: true });
  fs.writeFileSync(path.join(scratch, 'browser-request.json'), JSON.stringify({ script, args, shots: Boolean(process.env.SITE_LOOK_SHOTS), routes: process.env.SITE_REVIEW_ROUTES }));
  const result = spawnSync(process.env.BROWSER_BOX ?? 'browser-box', ['run', '--offline', '--out', scratch, 'scripts/browser-box-entry.mjs'], { stdio: 'inherit' });
  if (result.error) console.error(`browser-box launch failed: ${result.error.message}`);
  for (const name of outputs) if (fs.existsSync(path.join(site, name))) fs.cpSync(path.join(site, name), name, { recursive: true });
  if (reviewDirectory && fs.existsSync(path.join(scratch, 'review-shots'))) fs.cpSync(path.join(scratch, 'review-shots'), reviewDirectory, { recursive: true });
  if (script === 'scripts/social-card.mjs' && result.status === 0) fs.copyFileSync(path.join(site, 'public/social-card.png'), 'public/social-card.png');
  process.exitCode = result.status ?? 1;
} finally { fs.rmSync(scratch, { recursive: true, force: true }); }
