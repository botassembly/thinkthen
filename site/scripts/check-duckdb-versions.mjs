// Public support declarations must cover the build authority independently.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export function omissions(authority, catalog, readme) {
  // Deliberately read the list without requiring build pins: an omitted public
  // declaration must fail even when a new version is still being prepared.
  const match = /^DUCKDB_VERSIONS="(v[0-9.A-Za-z-]+(?: v[0-9.A-Za-z-]+)*)"$/m.exec(authority);
  if (!match) throw new Error('malformed DuckDB version list');
  const declaration = /slug:\s*['"]duckdb['"][^{}]*versions:\s*\{\s*runtime:\s*\[([^\]]+)\]/.exec(catalog)?.[1] ?? '';
  const entries = declaration.replace(/'(?:\\.|[^'\\])*'|"(?:\\.|[^"\\])*"|\/\/[^\n]*|\/\*[\s\S]*?\*\//g,
    (token) => token.startsWith('/') ? ' ' : token).split(',')
    .map((entry) => /^\s*(['"])(v[0-9.A-Za-z-]+)\1\s*$/.exec(entry)?.[2]);
  return match[1].split(' ').flatMap((version) => {
    // Public prose uses release numbers; build metadata retains exact source tags.
    const display = /^v(\d+\.\d+(?:\.\d+)?)(?:-[A-Za-z0-9-]+)?$/.exec(version)?.[1];
    const labels = [version, ...(display ? [display] : [])];
    const documented = labels.some((label) => {
      const escaped = label.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      return new RegExp(`(?:DuckDB |and )${escaped}(?![\\w-]|\\.\\d)`).test(readme);
    });
    return [
      ...(!entries.includes(version) ? [`catalog omits ${version}`] : []),
      ...(!documented ? [`DuckDB README omits ${version}`] : []),
    ];
  });
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
  const errors = omissions(...['databases/duckdb/tools/version.env', 'site/src/data/catalog/duckdb.mjs', 'databases/duckdb/README.md']
    .map((name) => fs.readFileSync(path.join(repo, name), 'utf8')));
  if (errors.length) { console.error(errors.join('\n')); process.exit(1); }
  console.log('DuckDB public versions cover the build list');
}
