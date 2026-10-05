import assert from 'node:assert/strict';
import { omissions } from './check-duckdb-versions.mjs';
const versions = 'DUCKDB_VERSIONS="v1.5.5 v1.5.4"';
const catalog = "supportedVersions: ['v1.5.5', 'v1.5.4']";
const readme = 'DuckDB v1.5.5 and v1.5.4';
assert.deepEqual(omissions(versions, catalog, readme), []);
assert.deepEqual(omissions(versions, catalog.replace("'v1.5.4'", ''), readme), ['catalog omits v1.5.4']);
assert.deepEqual(omissions(versions, catalog, readme.replace('v1.5.4', '')), ['DuckDB README omits v1.5.4']);
assert.deepEqual(omissions(versions.replace('v1.5.4"', 'v1.5.4 v1.5.6"'), catalog, readme),
  ['catalog omits v1.5.6', 'DuckDB README omits v1.5.6']);
console.log('DuckDB declarations reject independent omissions');
