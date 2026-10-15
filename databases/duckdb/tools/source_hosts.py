"""Build and load the approved source hosts through the community template."""
from __future__ import annotations

import argparse
import json
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


def cli_runner(cli, extension):
    from harness import child_env, stored
    def run(statements, base):
        results = []
        with tempfile.TemporaryDirectory(prefix='thinkthen-duckdb-source-') as folder:
            env = child_env(base, Path(folder))
            # Delimit both streams while DuckDB keeps one connection and its settings.
            marker = r'^thinkthen-statement-end$'
            boundary = ('.print thinkthen-statement-end\n.output /dev/stderr\n'
                        '.print thinkthen-statement-end\n.output stdout\n')
            scripts = []
            for index, statement in enumerate(statements):
                display = Path(folder) / f'columns-{index}'
                query = statement.lstrip().upper().startswith(('SELECT', 'WITH'))
                scripts.append(f'.mode duckbox\n.output {display}\n' +
                               statement.rstrip().rstrip(';') + ';\n.output stdout\n.mode json\n' +
                               ('.last\n' if query else '') + boundary)
            script = ''.join(scripts)
            load = "LOAD '" + str(extension).replace("'", "''") + "';"
            done = subprocess.run([str(cli), '-unsigned', '-init', '/dev/null', '-batch',
                                   '-cmd', load], input=script, env=env,
                                  capture_output=True, text=True, timeout=60, check=False)
            outputs = re.split(marker, done.stdout, flags=re.MULTILINE)
            errors = re.split(marker, done.stderr, flags=re.MULTILINE)
            if len(outputs) != len(statements) + 1 or len(errors) != len(outputs):
                raise AssertionError(f'the CLI did not finish every statement: {done.stderr}')
            for index, (output, error) in enumerate(zip(outputs[:-1], errors[:-1])):
                if error.strip():
                    results.append({'error': error.strip()})
                else:
                    display = (Path(folder) / f'columns-{index}').read_text().splitlines()
                    types = [item.strip() for item in display[2].split('│')[1:-1]] if display else []
                    # JSON mode repeats column names and quotes uint64; retain SQL column types.
                    rows = json.loads(output, object_pairs_hook=JsonPairs) if output.strip() else []
                    results.append({'rows': [[cli_value(value, kind) for (_, value), kind
                                              in zip(row, types)] for row in rows]})
            stored(env.get('THINKTHEN_CACHE', ''))
        return results
    return run


class JsonPairs(list):
    """Keep repeated SQL column names distinct in the shell's JSON output."""


def cli_value(value, kind=''):
    if value is None:
        return None
    if kind.endswith('[]'):
        return [cli_value(item, kind[:-2]) for item in value]
    if kind == 'json':
        return json.dumps(cli_value(value), separators=(',', ':'), ensure_ascii=False)
    if kind in ('uint64', 'int128', 'uint128'):
        return int(value)
    if isinstance(value, JsonPairs):
        return {key: cli_value(item) for key, item in value}
    if isinstance(value, list):
        return [cli_value(item) for item in value]
    return value


def check_hosts(cache, fetch):
    if not fetch:
        prepare_backend()
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
        if (inputs / 'venv/bin/cmake').exists():
            env['PATH'] = str(inputs / 'venv/bin') + os.pathsep + env.get('PATH', '')
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
        from named_options import check as named_check
        from native_records import check as records_check
        run = cli_runner(build / 'build/release/duckdb', extension)
        named_check(run)
        records_check(run)
        stage(version, extension)
        print(f'DuckDB {version} at {source_commit}: community SQL, named options and native records pass', flush=True)
    return 0


def prepare_backend():
    """Build this checkout's fixture and keep its Cargo-selected executable alive.

    The caller owns CARGO_TARGET_DIR until all source-host checks return. Read
    Cargo's artifact path rather than guessing a profile, host or executable suffix.
    """
    env = tool_environment()
    lock = env.get('THINKTHEN_HEAVY_LOCK', '/tmp/thinkthen-heavy.lock')
    admission = []
    if env.get('THINKTHEN_HEAVY_LOCK_HELD') != lock:
        admission = [sys.executable, str(REPO / 'sdlc/scripts/build-admission.py'), '--lock', lock, '--']
    done = subprocess.run(admission + ['cargo', 'build', '--locked', '--offline',
                          '--package', 'conformance-backend', '--bin', 'conformance-backend',
                          '--message-format=json'], cwd=REPO, env=env,
                          stdout=subprocess.PIPE, text=True, check=True)
    executables = [Path(item['executable']) for line in done.stdout.splitlines()
                   if (item := json.loads(line)).get('reason') == 'compiler-artifact'
                   and item.get('target', {}).get('name') == 'conformance-backend'
                   and item.get('executable')]
    if len(executables) != 1 or not executables[0].is_file() or not os.access(executables[0], os.X_OK):
        raise ValueError('Cargo did not build one executable loopback backend')
    import harness
    harness.BACKEND = str(executables[0])
    return executables[0]


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
