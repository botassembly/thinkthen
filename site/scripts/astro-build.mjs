// Runs astro build and fails on any warning, so a page problem never ships quietly.
import { spawnSync } from 'node:child_process';
const r = spawnSync('npx', ['astro', 'build'], { encoding: 'utf8', env: { ...process.env, FORCE_COLOR: '0' } });
process.stdout.write(r.stdout);
process.stderr.write(r.stderr);
const warnings = (r.stdout + r.stderr).split('\n').filter((l) => /\[WARN\]|\bwarn(ing)?\b:/i.test(l));
if (r.status !== 0) process.exit(r.status ?? 1);
if (warnings.length) {
  console.error(`\nastro build printed ${warnings.length} warning(s):\n${warnings.join('\n')}`);
  process.exit(1);
}
