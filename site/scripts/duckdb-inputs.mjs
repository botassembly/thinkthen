// Read the tracked version authority as bounded literal assignments.
import fs from 'node:fs';
import os from 'node:os';

export function readDuckDBVersions(file) {
  const raw = fs.readFileSync(file);
  if (raw.length > 16384 || [...raw].some((byte) => byte > 127)) throw new Error('invalid DuckDB input authority');
  const pins = new Map();
  for (const line of raw.toString('ascii').split(/\r?\n/)) {
    if (!line || line.startsWith('#')) continue;
    const match = /^(DUCKDB_[A-Z0-9_]+)=(?:"([A-Za-z0-9 ._-]+)"|([A-Za-z0-9_./:-]+))$/.exec(line);
    if (!match || pins.has(match[1])) throw new Error('malformed or duplicate DuckDB input assignment');
    pins.set(match[1], match[2] ?? match[3]);
  }
  const versions = (pins.get('DUCKDB_VERSIONS') ?? '').split(' ');
  if (versions.length < 1 || versions.length > 8 || new Set(versions).size !== versions.length ||
      versions.some((v) => !/^v[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}$/.test(v))) {
    throw new Error('unknown, malformed or duplicate DuckDB supported versions');
  }
  return versions;
}

export function nativeDuckDBTarget(kernel = os.platform(), chip = os.arch()) {
  const identity = {
    'linux:x64': ['x86_64-unknown-linux-gnu', 'linux_amd64'],
    'linux:arm64': ['aarch64-unknown-linux-gnu', 'linux_arm64'],
    'darwin:arm64': ['aarch64-apple-darwin', 'osx_arm64'],
    'darwin:x64': ['x86_64-apple-darwin', 'osx_amd64'],
  }[`${kernel}:${chip}`];
  if (!identity) throw new Error('no pinned native DuckDB target');
  return identity;
}

export function duckDBArtifact(repo, version, target) {
  const file = `${repo}/databases/duckdb/build/artifacts/cpp/${version}/${target}/thinkthen.duckdb_extension`;
  const meta = fs.lstatSync(file);
  if (!meta.isFile() || meta.isSymbolicLink()) throw new Error('canonical DuckDB artifact is not a regular file');
  return file;
}
