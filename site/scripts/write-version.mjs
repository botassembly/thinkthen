#!/usr/bin/env node
// Prove every deploy. The site writes the commit it was built from, and the
// deploy is only finished when thinkthen.dev/version.json carries that commit.
// A docs deploy can fail while the old build keeps serving. This is how you
// find out.

import fs from 'node:fs';
import { execSync } from 'node:child_process';

function git(args, fallback) {
  try {
    return execSync(`git ${args}`, { stdio: ['ignore', 'pipe', 'ignore'] }).toString().trim();
  } catch {
    return fallback;
  }
}

const commit = process.env.GITHUB_SHA || git('rev-parse HEAD', 'unknown');
const version = {
  commit,
  short: commit.slice(0, 7),
  branch: process.env.GITHUB_REF_NAME || git('rev-parse --abbrev-ref HEAD', 'unknown'),
  built: new Date().toISOString(),
};

fs.mkdirSync('public', { recursive: true });
fs.writeFileSync('public/version.json', JSON.stringify(version, null, 2) + '\n');
console.log(`version.json: ${version.short} on ${version.branch}`);
