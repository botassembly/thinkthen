// The same immutable prepared-program validation serves capture and bundling.
import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';

export function validatePreparedExamples(directory) {
  if (!path.isAbsolute(directory)) throw new Error('Use an absolute prepared example bundle');
  const preparedDir = fs.realpathSync(directory);
  if (!path.relative(fs.realpathSync('..'), preparedDir).startsWith(`..${path.sep}`)) throw new Error('Prepared examples must be outside the checkout');
  const prepared = JSON.parse(fs.readFileSync(path.join(preparedDir, 'manifest.json')));
  const source = fs.readFileSync(path.join(preparedDir, 'source-commit'), 'utf8').trim();
  const unchanged = spawnSync('git', ['diff', '--quiet', source, '--', '.', ...['output.txt', 'stderr.txt', 'exit', 'ran', 'capture.json', 'unverified'].map((name) => `:(exclude)site/examples/**/${name}`)], { cwd: path.resolve('..') });
  if (prepared.source !== source || unchanged.status !== 0) throw new Error('Prepared example source inputs differ');
  const inventory = (dir, prefix = '') => fs.readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const name = prefix + entry.name;
    if (name === 'manifest.json') return [];
    return entry.isDirectory() ? inventory(path.join(dir, entry.name), name + '/') : [name];
  });
  if (!isDeepStrictEqual(inventory(preparedDir).sort(), [...Object.keys(prepared.files), ...Object.keys(prepared.links)].sort())) throw new Error('Prepared example inventory differs');
  for (const [name, digest] of Object.entries(prepared.files)) {
    const file = path.resolve(preparedDir, name);
    if (!file.startsWith(preparedDir + path.sep) || createHash('sha256').update(fs.readFileSync(file)).digest('hex') !== digest) throw new Error('Prepared example bytes differ');
  }
  for (const [name, target] of Object.entries(prepared.links)) if (fs.readlinkSync(path.join(preparedDir, name)) !== target) throw new Error('Prepared example link differs');
  return { ...prepared, directory: preparedDir };
}
