"""Build and load the approved source hosts through the community template."""
from __future__ import annotations

import argparse
import fcntl
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import shutil

from inputs import authority, canonical, native_target, selected, versions

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parent.parent


def tool_environment():
    names = ('PATH', 'HOME', 'LANG', 'LC_ALL', 'TMPDIR', 'TEMP', 'CARGO_HOME', 'RUSTUP_HOME',
             'CARGO_TARGET_DIR', 'CARGO_BUILD_JOBS', 'CARGO_NET_OFFLINE', 'RUSTFLAGS',
             'CARGO_ENCODED_RUSTFLAGS', 'RUSTC_WRAPPER', 'RUSTUP_TOOLCHAIN', 'CC', 'CXX', 'AR',
             'CFLAGS', 'CXXFLAGS', 'LDFLAGS', 'VCPKG_BINARY_SOURCES', 'VCPKG_ROOT',
             'VCPKG_TOOLCHAIN_PATH', 'VCPKG_TARGET_TRIPLET', 'VCPKG_HOST_TRIPLET',
             'OPENSSL_ROOT_DIR', 'OPENSSL_DIR', 'OPENSSL_USE_STATIC_LIBS',
             'MACOSX_DEPLOYMENT_TARGET', 'SDKROOT', 'THINKTHEN_TOOLCHAINS',
             'THINKTHEN_DUCKDB_VERSION', 'THINKTHEN_DUCKDB_CACHE_LOCK_HELD',
             'THINKTHEN_HEAVY_LOCK', 'THINKTHEN_HEAVY_LOCK_HELD')
    env = {name: os.environ.get(name) for name in names if os.environ.get(name) is not None}
    env.update(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL='/dev/null', GIT_NO_REPLACE_OBJECTS='1',
               PYTHONDONTWRITEBYTECODE='1')
    return env


def hosts():
    pins = authority()
    requested = os.environ.get('THINKTHEN_DUCKDB_VERSION')
    if requested is not None and requested not in versions():
        raise ValueError(f'unsupported DuckDB version: {requested}')
    for version in versions():
        if os.environ.get('THINKTHEN_DUCKDB_VERSION', version) != version:
            continue
        kind = 'RELEASE' if version == pins['DUCKDB_ROUTINE_RELEASE'] else 'PREVIEW'
        prefix = 'DUCKDB_ROUTINE_' + kind
        values = [pins.get(prefix + suffix, '') for suffix in ('', '_SOURCE', '_TOOLS')]
        if not values[0] or any(re.fullmatch('[0-9a-f]{40}', value) is None for value in values[1:]):
            raise ValueError('the routine DuckDB source host pins are incomplete')
        yield tuple(values)


def prepare(folder, commit, repository, fetch, release=None):
    env = tool_environment()
    if fetch:
        folder.mkdir(parents=True, exist_ok=True)
        if not (folder / '.git').exists():
            subprocess.run(['git', 'init', '--quiet', str(folder)], env=env, check=True)
        subprocess.run(['git', '-C', str(folder), 'fetch', '--no-tags', '--depth', '1', repository, commit], env=env, check=True)
        if release:
            tag = 'refs/tags/' + release
            subprocess.run(['git', '-C', str(folder), 'fetch', '--no-tags', '--depth', '1', repository, tag + ':' + tag], env=env, check=True)
        subprocess.run(['git', '-C', str(folder), 'checkout', '--quiet', '--detach', commit], env=env, check=True)
    if release:
        observed = subprocess.check_output(['git', '-C', str(folder), 'rev-parse', release + '^{commit}'], env=env, text=True).strip()
        if observed != commit:
            raise ValueError('the released DuckDB tag differs from the approved source')
    from validate_inputs import archive_tree
    archive_tree.raw_git(folder, commit)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--fetch', action='store_true')
    args = parser.parse_args()
    cache = Path(os.environ.get('THINKTHEN_TOOLCHAINS', Path.home() / '.cache/thinkthen-toolchains')) / 'duckdb/community'
    lock = cache.parent.parent / '.duckdb-cache-mutation.lock'
    if args.fetch and os.environ.get('THINKTHEN_DUCKDB_CACHE_LOCK_HELD') != str(lock):
        return subprocess.call([sys.executable, str(ROOT / 'tools/cache_lock.py'), str(lock),
                                sys.executable, str(Path(__file__).resolve()), '--fetch'], env=tool_environment())
    lock.parent.mkdir(parents=True, exist_ok=True)
    with lock.open('a') as lease:
        if not args.fetch:
            fcntl.flock(lease, fcntl.LOCK_SH)
        try:
            return check_hosts(cache, args.fetch)
        except FileNotFoundError as error:
            print(f'not run: missing DuckDB source input: {error}', file=sys.stderr)
            return 77


def check_hosts(cache, fetch):
    for version, source_commit, tools_commit in hosts():
        inputs = cache / version
        if not fetch and not (inputs / 'source/.git').exists():
            print(f'not run: DuckDB {version} source inputs are missing; run tools/setup.sh --fetch', file=sys.stderr)
            return 77
        prepare(inputs / 'source', source_commit, 'https://github.com/duckdb/duckdb.git', fetch,
                version if version == authority()['DUCKDB_ROUTINE_RELEASE'] else None)
        prepare(inputs / 'tools', tools_commit, 'https://github.com/duckdb/extension-ci-tools.git', fetch)
        if fetch:
            continue
        build = ROOT / 'build/community' / version
        build.mkdir(parents=True, exist_ok=True)
        for name, target in [('extension-ci-tools', inputs / 'tools'), ('databases', ROOT.parent),
                             ('vcpkg.json', REPO / 'vcpkg.json')]:
            link = build / name
            if not link.exists():
                link.symlink_to(target, target_is_directory=target.is_dir())
        command = ['make', '-f', str(REPO / 'Makefile'), 'PROJ_DIR=' + str(build) + '/',
                   'DUCKDB_SRCDIR=' + str(inputs / 'source'), 'EXT_CONFIG=' + str(REPO / 'extension_config.cmake'),
                   'GEN=ninja', 'BUILD_EXTENSION_TEST_DEPS=none', 'DISABLE_SANITIZER=1',
                   'EXT_RELEASE_FLAGS=-DBUILD_UNITTESTS=ON',
                   'TESTS_BASE_DIRECTORY=' + str(ROOT / 'community/test/sql') + '/']
        env = tool_environment()
        env['CMAKE_BUILD_PARALLEL_LEVEL'] = '2'
        admission = []
        lock = env.get('THINKTHEN_HEAVY_LOCK', '/tmp/thinkthen-heavy.lock')
        if env.get('THINKTHEN_HEAVY_LOCK_HELD') != lock:
            admission = [sys.executable, str(REPO / 'sdlc/scripts/build-admission.py'), '--lock', lock, '--']
        subprocess.run(admission + command + ['release'], cwd=build, env=env, check=True)
        from harness import child_env
        with tempfile.TemporaryDirectory(prefix='thinkthen-duckdb-sql-') as folder:
            test_env = child_env('http://127.0.0.1:9/generic/v1', Path(folder), keyless=True)
            subprocess.run(command + ['test_release'], cwd=build, env=test_env, check=True)
        extension = build / 'build/release/extension/thinkthen/thinkthen.duckdb_extension'
        stage(version, extension)
        print(f'DuckDB {version} at {source_commit}: source build and community SQL pass', flush=True)
    return 0


def stage(version, extension):
    identity = selected(version)
    sys.path.insert(0, str(ROOT / 'cpp'))
    from verify_footer import verify
    release = next(line.split('"')[1] for line in (REPO / 'crates/thinkthen/Cargo.toml').read_text().splitlines()
                   if line.startswith('version = '))
    verify(extension, native_target(), identity['metadata_version'], release)
    output = canonical(version)
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(dir=output.parent, delete=False) as temporary:
        staged = Path(temporary.name)
    try:
        shutil.copyfile(extension, staged)
        staged.chmod(0o644)
        staged.replace(output)
    finally:
        staged.unlink(missing_ok=True)


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except ValueError as error:
        print(f'inputs: {error}', file=sys.stderr)
        raise SystemExit(2) from error
