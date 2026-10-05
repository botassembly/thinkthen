"""Read bounded, literal DuckDB input assignments; never execute version.env."""
from __future__ import annotations

import os
import platform as host
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
PLATFORMS = {
    'x86_64-unknown-linux-gnu': 'linux_amd64',
    'aarch64-unknown-linux-gnu': 'linux_arm64',
    'aarch64-apple-darwin': 'osx_arm64',
    'x86_64-apple-darwin': 'osx_amd64',
}
HOSTS = {
    ('Linux', 'x86_64'): 'x86_64-unknown-linux-gnu',
    ('Linux', 'aarch64'): 'aarch64-unknown-linux-gnu',
    ('Darwin', 'arm64'): 'aarch64-apple-darwin',
    ('Darwin', 'x86_64'): 'x86_64-apple-darwin',
}


def pin_fields(pins: dict[str, str], version: str, platform: str) -> dict[str, str]:
    prefix = 'DUCKDB_' + version.upper().replace('.', '_')
    fields = {'source_commit': prefix + '_CPP_SOURCE_COMMIT', 'requirements': prefix + '_REQUIREMENTS'}
    fields.update({name: prefix + '_' + platform.upper() + '_' + name.upper()
                   for name in ('cli_zip_sha256', 'cli_sha256', 'static_zip_sha256', 'manifest')})
    result = {name: pins.get(key, '') for name, key in fields.items()}
    for name, value in result.items():
        pattern = r'requirements(?:-v[0-9.]+)?\.txt' if name == 'requirements' else r'archive-sha256(?:-[A-Za-z0-9.-]+)?\.txt' if name == 'manifest' else (
            r'[0-9a-f]{40}' if name == 'source_commit' else r'[0-9a-f]{64}')
        if re.fullmatch(pattern, value) is None:
            raise ValueError(f'missing or malformed DuckDB {name} for {version}/{platform}')
    return result


def authority(path: Path = HERE / 'version.env') -> dict[str, str]:
    raw = path.read_bytes()
    if len(raw) > 16384:
        raise ValueError('DuckDB input authority exceeds 16384 bytes')
    pins = {}
    for line in raw.decode('ascii').splitlines():
        if not line or line.startswith('#'):
            continue
        match = re.fullmatch(r'(DUCKDB_[A-Z0-9_]+)=(?:"([A-Za-z0-9 ._-]+)"|([A-Za-z0-9_./:-]+))', line)
        if not match or match[1] in pins:
            raise ValueError('malformed or duplicate DuckDB input assignment')
        pins[match[1]] = match[2] if match[2] is not None else match[3]
    versions = pins.get('DUCKDB_VERSIONS', '').split(' ')
    if not 1 <= len(versions) <= 8 or len(set(versions)) != len(versions) or any(
        re.fullmatch(r'v[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}', v) is None for v in versions
    ):
        raise ValueError('unknown, malformed or duplicate DuckDB supported versions')
    # Every declared entry needs a complete pin set before any selector resolves paths.
    for version in versions:
        for platform in PLATFORMS.values():
            pin_fields(pins, version, platform)
    return pins


def versions(path: Path = HERE / 'version.env') -> list[str]:
    return authority(path)['DUCKDB_VERSIONS'].split(' ')


def native_target() -> str:
    target = HOSTS.get((host.system(), host.machine()))
    if target is None:
        raise LookupError(f'no pinned DuckDB C++ inputs for {host.system()}:{host.machine()}')
    return target


def selected(version: str | None = None, target: str | None = None,
             path: Path = HERE / 'version.env') -> dict[str, str]:
    pins = authority(path)
    supported = pins['DUCKDB_VERSIONS'].split(' ')
    version = version if version is not None else os.environ.get('THINKTHEN_DUCKDB_VERSION', supported[0])
    if version not in supported:
        raise ValueError(f'unsupported DuckDB version: {version}')
    target = target if target is not None else native_target()
    if target not in PLATFORMS:
        raise ValueError(f'unsupported DuckDB target: {target}')
    platform = PLATFORMS[target]
    result = pin_fields(pins, version, platform)
    asset_platform = platform.replace('_', '-')
    result.update(version=version, target=target, platform=platform,
                  cli_asset=f'duckdb_cli-{asset_platform}.zip', static_asset=f'static-libs-{asset_platform}.zip')
    return result


def canonical(version: str | None = None, target: str | None = None) -> Path:
    identity = selected(version, target)
    return HERE.parent / 'build/artifacts/cpp' / identity['version'] / identity['target'] / 'thinkthen.duckdb_extension'


if __name__ == '__main__':
    try:
        action = sys.argv[1]
        if action == 'versions' and len(sys.argv) == 2:
            print(' '.join(versions()))
        elif action == 'select' and len(sys.argv) == 4:
            item = selected(sys.argv[2] or None, sys.argv[3])
            print(' '.join(item[name] for name in ('version', 'target', 'platform', 'cli_asset', 'cli_zip_sha256',
                  'cli_sha256', 'static_asset', 'static_zip_sha256', 'source_commit', 'manifest', 'requirements')))
        else:
            raise ValueError('usage: inputs.py versions | select VERSION TARGET')
    except LookupError as error:
        raise SystemExit(f'setup: {error}') from error
    except (OSError, ValueError) as error:
        print(f'inputs: {error}', file=sys.stderr)
        sys.exit(2)
