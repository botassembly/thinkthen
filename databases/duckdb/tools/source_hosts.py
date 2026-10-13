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

from inputs import authority
from harness import child_env, stored

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parent.parent


def hosts():
    pins = authority()
    for kind in ('RELEASE', 'PREVIEW'):
        prefix = 'DUCKDB_ROUTINE_' + kind
        values = [pins.get(prefix + suffix, '') for suffix in ('', '_SOURCE', '_TOOLS')]
        if not values[0] or any(re.fullmatch('[0-9a-f]{40}', value) is None for value in values[1:]):
            raise ValueError('the routine DuckDB source host pins are incomplete')
        yield tuple(values)


def prepare(folder, commit, repository, fetch, release=None):
    if fetch:
        folder.mkdir(parents=True, exist_ok=True)
        if not (folder / '.git').exists():
            subprocess.run(['git', 'init', '--quiet', str(folder)], check=True)
        subprocess.run(['git', '-C', str(folder), 'fetch', '--no-tags', '--depth', '1', repository, commit], check=True)
        if release:
            tag = 'refs/tags/' + release
            subprocess.run(['git', '-C', str(folder), 'fetch', '--no-tags', '--depth', '1', repository, tag + ':' + tag], check=True)
        subprocess.run(['git', '-C', str(folder), 'checkout', '--quiet', '--detach', commit], check=True)
    if release:
        observed = subprocess.check_output(['git', '-C', str(folder), 'rev-parse', release + '^{commit}'], text=True).strip()
        if observed != commit:
            raise ValueError('the released DuckDB tag differs from the approved source')
    from validate_inputs import archive_tree
    archive_tree.raw_git(folder, commit)


def cli_runner(cli, extension):
    def run(statements, base):
        results = []
        with tempfile.TemporaryDirectory(prefix='thinkthen-duckdb-source-') as folder:
            env = child_env(base, Path(folder))
            for statement in statements:
                load = "LOAD '" + str(extension).replace("'", "''") + "'; "
                done = subprocess.run([str(cli), '-unsigned', '-json', '-c', load + statement],
                                      env=env, capture_output=True, text=True, timeout=60, check=False)
                if done.returncode:
                    results.append({'error': done.stderr})
                else:
                    import json
                    rows = json.loads(done.stdout)
                    results.append({'rows': [list(row.values()) for row in rows]})
            stored(env.get('THINKTHEN_CACHE', ''))
        return results
    return run


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--fetch', action='store_true')
    args = parser.parse_args()
    cache = Path(os.environ.get('THINKTHEN_TOOLCHAINS', Path.home() / '.cache/thinkthen-toolchains')) / 'duckdb/community'
    lock = cache.parent.parent / '.duckdb-cache-mutation.lock'
    if args.fetch and os.environ.get('THINKTHEN_DUCKDB_CACHE_LOCK_HELD') != str(lock):
        return subprocess.call([sys.executable, str(ROOT / 'tools/cache_lock.py'), str(lock),
                                sys.executable, str(Path(__file__).resolve()), '--fetch'])
    lock.parent.mkdir(parents=True, exist_ok=True)
    with lock.open('a') as lease:
        if not args.fetch:
            fcntl.flock(lease, fcntl.LOCK_SH)
        return check_hosts(cache, args.fetch)


def check_hosts(cache, fetch):
    for version, source_commit, tools_commit in hosts():
        inputs = cache / version
        prepare(inputs / 'source', source_commit, 'https://github.com/duckdb/duckdb.git', fetch,
                version if version == authority()['DUCKDB_ROUTINE_RELEASE'] else None)
        prepare(inputs / 'tools', tools_commit, 'https://github.com/duckdb/extension-ci-tools.git', fetch)
        if fetch:
            continue
        build = ROOT / 'build/community' / version
        build.mkdir(parents=True, exist_ok=True)
        for name, target in [('extension-ci-tools', inputs / 'tools'), ('databases', ROOT.parent)]:
            link = build / name
            if not link.exists():
                link.symlink_to(target, target_is_directory=True)
        command = ['make', '-f', str(REPO / 'Makefile'), 'PROJ_DIR=' + str(build) + '/',
                   'DUCKDB_SRCDIR=' + str(inputs / 'source'), 'EXT_CONFIG=' + str(REPO / 'extension_config.cmake'),
                   'GEN=ninja', 'BUILD_EXTENSION_TEST_DEPS=none', 'DISABLE_SANITIZER=1',
                   'EXT_RELEASE_FLAGS=-DBUILD_UNITTESTS=ON',
                   'TESTS_BASE_DIRECTORY=' + str(ROOT / 'community/test/sql') + '/']
        env = os.environ.copy()
        env['CMAKE_BUILD_PARALLEL_LEVEL'] = '2'
        subprocess.run(command + ['release'], cwd=build, env=env, check=True)
        with tempfile.TemporaryDirectory(prefix='thinkthen-duckdb-sql-') as folder:
            test_env = child_env('http://127.0.0.1:9/generic/v1', Path(folder), keyless=True)
            subprocess.run(command + ['test_release'], cwd=build, env=test_env, check=True)
        cli = build / 'build/release/duckdb'
        extension = build / 'build/release/extension/thinkthen/thinkthen.duckdb_extension'
        from named_options import check
        check(cli_runner(cli, extension))
        print(f'DuckDB {version} at {source_commit}: community SQL and installed named options pass', flush=True)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
