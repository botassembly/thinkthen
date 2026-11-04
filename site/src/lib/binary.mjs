// The version and build commit the built program reports, read with product.versionArgs.
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import product from '../../product.config.mjs';

export function binaryInfo(env = { ...process.env, NO_COLOR: '1' }) {
  const bin = path.resolve(product.binDir, product.binary);
  const r = spawnSync(bin, product.versionArgs, { encoding: 'utf8', env, timeout: 30000 });
  const text = `${r.stdout ?? ''}${r.stderr ?? ''}`;
  const version = text.match(/\b(\d+\.\d+\.\d+(?:-[\w.]+)?)\b/)?.[1];
  if (r.status !== 0 || !version) throw new Error(`${bin} ${product.versionArgs.join(' ')} printed no version:\n${text}`);
  const commit = text.match(/^\s*commit:\s*([0-9a-f]{7,40})\b/im)?.[1] ?? null;
  return { version, commit, line: text.split('\n')[0].trim() };
}

// Command-specific help can describe the same flag differently. Merge only
// identical help, so a reference row cannot grant one command another's scope.
export function commandOptions(commands) {
  const groups = new Map();
  for (const command of commands.filter((command) => !command.hidden)) for (const option of command.options) {
    if (option.flag === '--help') continue;
    const key = JSON.stringify([option.short, option.flag, option.value, option.description, option.env, option.default, option.values]);
    if (!groups.has(key)) groups.set(key, { ...option, commands: [] });
    groups.get(key).commands.push(command.name);
  }
  return [...groups.values()].sort((a, b) => a.flag.localeCompare(b.flag));
}
