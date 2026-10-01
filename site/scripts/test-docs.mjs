#!/usr/bin/env node
// Run every doc test before a release: build thinkthen from this working
// tree, run the site build, replay every language sample and every SQL
// sample, and check the proof file strictly. Every sample replays from a
// saved recording, so the run needs no key and sends no request.
//
//   npm run test-docs
//
// The run strips every THINKTHEN_ variable and every key, token or secret
// from the environment first. It takes no options, so no --allow-missing can
// reach a runner, and a missing toolchain fails the run. The runners rewrite
// examples/bindings-proof.json. Commit it when every sample passed.

import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { leakedVariables } from './binding-proofs.mjs';

const site = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const repo = path.resolve(site, '..');

if (process.argv.length > 2) {
  console.error('test-docs: takes no options. A release runs every doc test with every toolchain.');
  process.exit(2);
}

const secret = (name) => name.startsWith('THINKTHEN_') || /(KEY|TOKEN|SECRET)$/.test(name) || leakedVariables({ [name]: '' }).length > 0;
const env = Object.fromEntries(Object.entries(process.env).filter(([name]) => !secret(name)));
const stripped = Object.keys(process.env).filter(secret);
if (stripped.length) console.log(`test-docs: unset ${stripped.sort().join(', ')} for the run.`);

const target = env.CARGO_TARGET_DIR ? path.resolve(repo, env.CARGO_TARGET_DIR) : path.join(repo, 'target');
const bin = path.join(target, 'release', 'thinkthen');

const steps = [
  ['cargo', ['build', '--release', '--locked', '--bin', 'thinkthen'], repo, {}],
  // smoke.mjs runs the command this step names. The binding runners refuse
  // any THINKTHEN_ variable, so only this step sees it.
  ['npm', ['run', 'build'], site, { THINKTHEN_BIN: bin }],
  ['node', ['scripts/smoke-bindings.mjs'], site, {}],
  ['node', ['scripts/smoke-sql.mjs'], site, {}],
  ['node', ['scripts/check-binding-proofs.mjs', '--strict'], site, {}],
];

for (const [cmd, args, cwd, extra] of steps) {
  const shown = [cmd, ...args].join(' ');
  console.log(`test-docs: ${shown}`);
  const done = spawnSync(cmd, args, { cwd, env: { ...env, ...extra }, stdio: 'inherit' });
  if (done.status !== 0) {
    console.error(`test-docs: ${shown} failed${done.error ? `: ${done.error.message}` : ` with exit ${done.status ?? done.signal}`}.`);
    process.exit(1);
  }
}
console.log('test-docs: every doc test passed. Commit examples/bindings-proof.json if it changed.');
