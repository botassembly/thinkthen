import assert from 'node:assert/strict';
import { omissions } from './check-duckdb-versions.mjs';
const versions = 'DUCKDB_VERSIONS="v1.5.6 v2.0-cyanoptera"';
const catalog = "supportedVersions: ['v1.5.6', 'v2.0-cyanoptera']";
const readme = 'DuckDB v1.5.6 and v2.0-cyanoptera';
assert.deepEqual(omissions(versions, catalog, readme), []);
assert.deepEqual(omissions(versions, catalog.replace("'v2.0-cyanoptera'", ''), readme), ['catalog omits v2.0-cyanoptera']);
assert.deepEqual(omissions(versions, catalog, readme.replace('v2.0-cyanoptera', '')), ['DuckDB README omits v2.0-cyanoptera']);
assert.deepEqual(omissions(versions.replace('v2.0-cyanoptera"', 'v2.0-cyanoptera v9.9.9"'), catalog, readme),
  ['catalog omits v9.9.9', 'DuckDB README omits v9.9.9']);
assert.deepEqual(omissions(versions, catalog, 'DuckDB 1.5.6 and 2.0'), []);
assert.deepEqual(omissions(versions, catalog, 'DuckDB 1.5.60 and 2.01'),
  ['DuckDB README omits v1.5.6', 'DuckDB README omits v2.0-cyanoptera']);
assert.deepEqual(omissions(versions, catalog, 'DuckDB 1.5.6 and 2.0.'), []);
assert.deepEqual(omissions(versions, catalog.replace("'v2.0-cyanoptera'", "'2.0'"), readme),
  ['catalog omits v2.0-cyanoptera']);
console.log('DuckDB declarations reject independent omissions');
