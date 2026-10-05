// Public support declarations must cover the build authority independently.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export function omissions(authority, catalog, readme) {
  // Deliberately read the list without requiring build pins: an omitted public
  // declaration must fail even when a new version is still being prepared.
  const match = /^DUCKDB_VERSIONS="(v[0-9.]+(?: v[0-9.]+)*)"$/m.exec(authority);
  if (!match) throw new Error('malformed DuckDB version list');
  const declaration = /supportedVersions: \[([^\]]+)\]/.exec(catalog)?.[1] ?? '';
  return match[1].split(' ').flatMap((version) => [
    ...(!declaration.includes(`'${version}'`) ? [`catalog omits ${version}`] : []),
    ...(!readme.includes(`DuckDB ${version}`) && !readme.includes(`and ${version}`) ? [`DuckDB README omits ${version}`] : []),
  ]);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
  const errors = omissions(...['databases/duckdb/tools/version.env', 'site/src/data/catalog.mjs', 'databases/duckdb/README.md']
    .map((name) => fs.readFileSync(path.join(repo, name), 'utf8')));
  if (errors.length) { console.error(errors.join('\n')); process.exit(1); }
  console.log('DuckDB public versions cover the build list');
}
