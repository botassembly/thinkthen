#!/usr/bin/env python3
"""Check prepared hosted inputs without acquiring tools or building outputs."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import runpy
import shlex
import shutil
import subprocess
import sys
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import SECRET  # noqa: E402


class Preflight:
    def __init__(self):
        self.missing = []

    def check(self, label, operation):
        try:
            operation()
        except (OSError, ValueError, RuntimeError, ImportError, subprocess.SubprocessError, AssertionError) as error:
            self.missing.append(f'{label}: {error}')

    def file(self, path, executable=False):
        path = Path(path)
        def verify():
            if not path.is_file() or (executable and not os.access(path, os.X_OK)):
                raise ValueError('missing' if not executable else 'missing executable')
        self.check(str(path), verify)

    def command(self, args, expected=None, cwd=ROOT, env=None):
        # Reuse the shared credential-name policy. Its blanket THINKTHEN_
        # namespace rule covers ordinary settings too, so inspect their suffix.
        secrets = {value for source in (os.environ, env or {}) for name, value in source.items()
                   if value and SECRET.search(name, len('THINKTHEN_') if name.upper().startswith('THINKTHEN_') else 0)}
        def diagnostic(value):
            if isinstance(value, bytes):
                value = value.decode(errors='replace')
            for secret in sorted(secrets, key=len, reverse=True):
                value = value.replace(secret, '[redacted]')
            # Redact before cutting: a secret spanning a cut must leave no fragment.
            if len(value) > 4096:
                value = value[:2048] + '\n[truncated]\n' + value[-2048:]
            return value.strip()

        def verify():
            try:
                result = subprocess.run(list(map(str, args)), cwd=cwd, capture_output=True,
                                        text=True, errors='replace', timeout=60, check=False, env=env)
            except subprocess.TimeoutExpired as error:
                reason, streams = 'timed out after 60 seconds', (error.stdout, error.stderr)
            else:
                if result.returncode:
                    reason = f'exit {result.returncode}'
                elif expected and not re.search(expected, result.stdout + result.stderr, re.M):
                    reason = 'version or prepared input differs from pin'
                else:
                    return
                streams = (result.stdout, result.stderr)
            output = '\n'.join(diagnostic(part) for part in streams if part)
            raise ValueError(reason + ('\n' + output if output else ''))
        self.check(diagnostic(shlex.join(list(map(str, args)))), verify)

    def tools(self, names):
        for name in names:
            self.check(name, lambda name=name: require(shutil.which(name), 'missing from PATH'))

    def hashes(self, manifest, folder):
        try:
            lines = manifest.read_text().splitlines()
        except OSError as error:
            self.missing.append(f'{manifest}: {error}')
            return
        for line in lines:
            if not line or line.startswith('#'):
                continue
            fields = line.split()
            if len(fields) != 2 or re.fullmatch(r'[0-9a-f]{64}', fields[0]) is None:
                self.missing.append(f'{manifest}: malformed SHA256 entry')
                continue
            digest, name = fields
            path = folder / name.lstrip('*')
            self.check(str(path), lambda path=path, digest=digest: require(
                hashlib.sha256(path.read_bytes()).hexdigest() == digest, 'SHA256 differs from pin'))

    def finish(self):
        for message in dict.fromkeys(self.missing):
            print('hosted-preflight: ' + message, file=sys.stderr)
        if not self.missing:
            print('hosted-preflight: all prepared inputs are ready')
        return int(bool(self.missing))


def require(condition, message):
    if not condition:
        raise ValueError(message)


def shell_values(script, names, env=None):
    command = '. "$1"\n' + '\n'.join('printf "%s\\n" "$' + name + '"' for name in names)
    output = subprocess.check_output(['sh', '-ec', command, 'preflight', str(script)],
                                     cwd=ROOT, env=env, text=True)
    return output.splitlines()


def rust(check, coverage=False):
    pin = tomllib.loads((ROOT / 'rust-toolchain.toml').read_text())['toolchain']
    check.tools(('cargo', 'rustc', 'rustup', 'git', 'python3', 'node', 'npm', 'tar'))
    node_pin = (ROOT / 'libraries/typescript/node.sha256').read_text().split()[1].split('-')[1]
    check.command(['node', '--version'], '^' + re.escape(node_pin) + r'\s*$')
    check.command(['rustc', '--version'], '^rustc ' + re.escape(pin['channel']) + r'\b')
    for component in pin['components']:
        check.command(['rustup', 'component', 'list', '--installed'], '^' + component + '-', ROOT)
    if coverage:
        def llvm():
            sysroot = subprocess.check_output(['rustc', '--print', 'sysroot'], text=True).strip()
            host = subprocess.check_output(['rustc', '-vV'], text=True).split('host: ')[1].splitlines()[0]
            for name in ('llvm-cov', 'llvm-profdata'):
                check.file(Path(sysroot) / 'lib/rustlib' / host / 'bin' / name, True)
        check.check('Rust coverage tools', llvm)


def cargo_cache(check, manifests):
    for manifest in manifests:
        check.command(['cargo', 'metadata', '--locked', '--offline', '--format-version', '1',
                       '--manifest-path', manifest])


def managed(check):
    tools = runpy.run_path(str(ROOT / 'sdlc/scripts/release-language-tools.py'))
    check.check('stable selected JVM', tools['check_jdk'])
    jdk = Path(os.environ.get('THINKTHEN_JDK_HOME', '/nonexistent'))
    for binary in ('java', 'javac', 'jar'):
        check.file(jdk / 'bin' / binary, True)
    env = tools['managed_env'](jdk)
    for name in ('dotnet', 'kotlin', 'scala'):
        check.check('selected ' + name + ' SDK', lambda name=name:
                    tools['sdk_path'](name, Path('/nonexistent'), False, env=env))


def native_inputs(check, target, cache, built_hosts=True, source_only=False, installed_files=False):
    ruby = ROOT / 'libraries/ruby/toolchain.env'
    minors = shell_values(ruby, ['RUBY_MINORS'])[0].split()
    for minor in minors:
        env = dict(os.environ, THINKTHEN_RUBY_MINOR=minor)
        version, digest, yaml, yaml512 = shell_values(ruby, ['RUBY_VERSION', 'RUBY_SHA256', 'YAML_SHA256', 'YAML_SHA512'], env)
        suffix = '-' + os.uname().machine + '-darwin' if 'apple' in target else ''
        prefix = cache / 'ruby' / (version + suffix)
        check.check('Ruby ' + version + ' stamp', lambda prefix=prefix, digest=digest, yaml=yaml, yaml512=yaml512:
                    require((prefix / 'thinkthen-toolchain.stamp').read_text().splitlines()[:3] ==
                            ['ruby ' + digest, 'yaml ' + yaml, 'yaml512 ' + yaml512], 'stamp differs from toolchain.env'))
        check.command([prefix / 'bin/ruby', '-rpsych', '-rzlib', '-ropenssl', '-rminitest', '-rrbs', '-e', 'puts RUBY_VERSION'], '^' + re.escape(version) + r'\s*$')
        rbs = shell_values(ruby, ['RUBY_RBS_VERSION'], env)[0]
        check.command([prefix / 'bin/rbs', '_' + rbs + '_', '--version'], re.escape(rbs))
    if not source_only:
        check.hashes(ROOT / 'databases/sqlite/amalgamation.sha256', cache / 'sqlite-amalgamation-3500000')
        # runtime_ready verifies executable hosts, their headers and libclang, not
        # only the downloaded packages. It never fetches or starts a server.
        for major in (16, 17, 18):
            artifact = ''
            if installed_files:
                version = tomllib.loads((ROOT / 'crates/thinkthen/Cargo.toml').read_text())['package']['version']
                artifact = 'THINKTHEN_ARTIFACT=' + shlex.quote(str(ROOT / 'platform' /
                    f'thinkthen-postgresql{major}-{version}-{target}.tar.gz')) + '; '
            check.command(['sh', '-ec', artifact + f'PG_MAJOR={major}; . ../../sdlc/scripts/scratch.sh; . ./runtime.sh; runtime_ready'],
                          cwd=ROOT / 'databases/postgresql')
        if 'apple' in target:
            check.hashes(ROOT / 'databases/sqlite/amalgamation-3490000.sha256', cache / 'sqlite-amalgamation-3490000')
        else:
            arch = 'arm64' if target.startswith('aarch64') else 'amd64'
            for line in (ROOT / 'databases/postgresql/runtime.sha256').read_text().splitlines():
                if line and not line.startswith('#') and line.split()[1].endswith('_' + arch + '.deb'):
                    digest, name = line.split()
                    path = cache / 'postgresql' / name
                    check.check(str(path), lambda path=path, digest=digest: require(
                        hashlib.sha256(path.read_bytes()).hexdigest() == digest, 'SHA256 differs from pin'))
    sys.path.insert(0, str(ROOT / 'databases/duckdb/tools'))
    from inputs import selected, versions
    from source_hosts import prepare
    for version in versions():
        identity = selected(version, target)
        folder = cache / 'duckdb/community' / version
        for kind in ('source', 'tools'):
            check.check(f'DuckDB {version} {kind}', lambda kind=kind, folder=folder, identity=identity:
                        prepare(folder / kind, identity[kind + '_commit'], '', False))
        if not built_hosts:
            continue
        from harness import child_env
        with tempfile.TemporaryDirectory(prefix='thinkthen-duckdb-preflight-') as temporary:
            # Match setup's prepared-host probe without extension acquisition or caller state.
            code = "import duckdb; con=duckdb.connect(config={'autoinstall_known_extensions':'false'}); "
            code += "print(con.execute('PRAGMA version').fetchone()[1])"
            check.command([folder / 'venv/bin/python', '-c', code], identity['source_commit'][:10],
                          env=child_env('http://127.0.0.1:9/v1', Path(temporary), keyless=True))
        cli = ROOT / 'databases/duckdb/build/community' / version / 'build/release/duckdb'
        check.command([cli, '-init', '/dev/null', '-noheader', '-list', '-c',
                       'SELECT source_id FROM pragma_version()'], identity['source_commit'][:8])


def installed(check, target, cache, installed_files=False):
    check.tools(('cc', 'c++', 'cmake', 'make', 'ninja', 'pkg-config', 'uv', 'sha256sum'))
    check.command(['uv', '--version'], r'^uv 0\.9\.17\b')
    check.check('native prepared inputs', lambda: native_inputs(check, target, cache, installed_files=installed_files))
    generators = Path(os.environ.get('RUNNER_TEMP', '/nonexistent')) / 'thinkthen-generators'
    # Read formatter pins from their existing bootstrap rather than keep a second authority.
    bootstrap = (ROOT / 'sdlc/scripts/release-workflow').read_text()
    for binary, expression, path in (
            ('ruff', r"'ruff==([^']+)'", generators / 'bin/ruff'),
            ('prettier', r"'prettier@([^']+)'", generators / 'node_modules/.bin/prettier')):
        version = re.search(expression, bootstrap)[1]
        check.command([path, '--version'], re.escape(version))
        check.check(binary + ' PATH handoff', lambda binary=binary, path=path:
                    require(shutil.which(binary) == str(path), 'prepared formatter missing from PATH'))
    check.command([generators / 'python/bin/python', '-c', 'import numpy, pandas, pyarrow'])
    # Match the interpreter the installed Python check chooses. A populated
    # venv for another Python minor does not prove its offline wheel cache.
    def python_cache():
        python = subprocess.check_output(
            ['sh', '-ec', '. libraries/python/host.sh; python_host'], cwd=ROOT, text=True).strip()
        require(python, 'no supported installed Python host')
        offline_caches(check, python)
    check.check('installed offline caches', python_cache)
    check.command(['bash', '-c', 'test "${BASH_VERSINFO[0]}" -ge 5'])
    if target == 'x86_64-unknown-linux-gnu':
        node = cache / 'node-v22.22.3-linux-x64'
        check.hashes(ROOT / 'libraries/typescript/node.sha256', cache / 'archives')
        check.command([node / 'bin/node', '--version'], r'^v22\.22\.3\s*$')
        managed(check)
        check.file('/usr/bin/bwrap', True)
        check.tools(('bwrap', 'flock', 'gprbuild', 'gnatmake', 'cobc', 'swift', 'zig', 'go', 'gofmt', 'composer', 'php8.3', 'R', 'Rscript', 'psql', 'dpkg-deb'))
        consumer_tools(check)
        check.hashes(ROOT / 'libraries/r/tools/pins.sha256', cache / 'archives')
        code = re.search(r"Rscript --vanilla -e '([^']+)'", (ROOT / 'libraries/r/check.sh').read_text())[1]
        check.command(['Rscript', '--vanilla', '-e', '.libPaths(c(' + json.dumps(str(cache / 'r-library')) + ', .libPaths())); ' + code])
    if 'apple' in target:
        check.tools(('swift', 'xcrun', 'lipo', 'brew'))
        check.command([os.environ.get('THINKTHEN_PYTHON', 'python3'), '-c',
                       'import sqlite3, jsonschema; sqlite3.connect(":memory:").enable_load_extension(True)'])


def consumer_tools(check):
    bootstrap = (ROOT / 'sdlc/scripts/release-workflow').read_text()
    tools = runpy.run_path(str(ROOT / 'sdlc/scripts/release-language-tools.py'))
    for name, version in tools['PACKAGES'].items():
        check.check('Ubuntu package ' + name, lambda name=name, version=version:
                    require(tools['package_version'](name) == version, 'installed package differs from pin'))
    for binary, package in tools['OWNERS'].items():
        found = shutil.which(binary) or '/nonexistent'
        check.command(['dpkg-query', '-S', found], '^' + re.escape(package) + ':')
    for operation in ('go-cpp-tools', 'swift-zig-tools', 'php-dart-tools', 'ada-objc-cobol-tools', 'managed-tools'):
        block = re.split(r'\n[a-z][a-z-]*\)\n', bootstrap.split('\n' + operation + ')\n', 1)[1])[0]
        for words in re.findall(r'for tool in ([^;]+); do', block):
            check.tools(shlex.split(words))
    for tool, args, expression in (
            ('go', ['version'], r'go version go([0-9.]+) linux/amd64'),
            ('zig', ['version'], r'Zig ([0-9.]+) archive'),
            ('swift', ['--version'], r'Swift version ([0-9.]+) \(swift-'),
            ('php8.3', ['-v'], r'PHP ([0-9.]+) executable')):
        version = re.search(expression, bootstrap)[1]
        check.command([tool, *args], re.escape(version))
    check.command(['php8.3', '-n', '-d', 'extension=ffi', '-d', 'ffi.enable=1', '-r',
                   'exit(class_exists("FFI") ? 0 : 1);'])
    dart = os.environ.get('TT_DART', 'dart')
    dart_version = re.search(r'dart-archive/channels/stable/release/([0-9.]+)/', bootstrap)[1]
    check.command([dart, '--version'], 'Dart SDK version: ' + re.escape(dart_version) + r' \(stable\)')
    pub_cache = Path(os.environ.get('PUB_CACHE', '/nonexistent'))
    check.file(pub_cache / 'hosted/pub.dev/ffi-2.2.0/pubspec.yaml')
    digest = re.search(r"= ([0-9a-f]{64}) \] \|\|\n\t\tfail 'cached ffi", bootstrap)[1]
    check.check('Dart ffi cache hash', lambda: require(
        (pub_cache / 'hosted-hashes/pub.dev/ffi-2.2.0.sha256').read_text().strip() == digest,
        'cached ffi differs from bootstrap pin'))
    dart_cache(check, dart)


def dart_cache(check, dart):
    # Use pub itself to verify the full locked native-assets closure offline.
    with tempfile.TemporaryDirectory(prefix='thinkthen-dart-preflight-') as folder:
        work = Path(folder)
        for name in ('pubspec.yaml', 'pubspec.lock'):
            shutil.copyfile(ROOT / 'libraries/dart' / name, work / name)
        check.command([dart, 'pub', 'get', '--offline', '--enforce-lockfile'], cwd=work)


def offline_caches(check, python):
    # Exercise the same offline installers in owned scratch, never in the checkout.
    # A listed cache entry alone does not prove the frozen consumer can install it.
    with tempfile.TemporaryDirectory(prefix='thinkthen-preflight-') as folder:
        work = Path(folder)
        for name in ('package.json', 'package-lock.json'):
            shutil.copyfile(ROOT / 'libraries/typescript' / name, work / name)
        check.command(['npm', 'ci', '--offline', '--ignore-scripts', '--no-audit', '--no-fund',
                       '--prefix', work])
        check.command(['uv', 'pip', 'install', '--offline', '--require-hashes', '--python', python,
                       '--only-binary', ':all:', '--target', work / 'python', '-r', ROOT / 'libraries/python/requirements-dev.txt'])


def downloaded_inputs(check, target):
    version = tomllib.loads((ROOT / 'crates/thinkthen/Cargo.toml').read_text())['package']['version']
    windows = target.endswith('windows-msvc')
    extension = '.zip' if windows else '.tar.gz'
    command_target = target.replace('-gnu', '-musl')
    expected = [f'thinkthen-c-{version}-{target}{extension}',
                f'thinkthen-{version}-{command_target}{extension}', 'thinkthen-first-run.tar.gz']
    if not windows:
        expected += [f'thinkthen-{family}-{version}-{target}.tar.gz' for family in
                     ('sqlite', 'duckdb', 'postgresql16', 'postgresql17', 'postgresql18')]
    for name in expected:
        check.file(ROOT / 'platform' / name)
        check.file(ROOT / 'platform' / (name + '.sha256'))
    directories = ['platform', 'crate-dist'] + ([] if windows else ['npm-dist'])
    if target == 'x86_64-unknown-linux-gnu':
        directories += ['r-dist', 'provenance']
    for directory in directories:
        folder = ROOT / directory
        check.check(directory, lambda folder=folder: require(
            folder.is_dir() and any(folder.iterdir()), 'prepared download is empty or missing'))
        for manifest in folder.glob('*.sha256'):
            check.hashes(manifest, folder)
    check.file(ROOT / 'crate-dist' / f'thinkthen-{version}.crate')
    if not windows:
        check.file(ROOT / 'npm-dist' / f'thinkthen-{version}.tgz')



def apple_input(check, local_only):
    # The rehearsal creates this report after bootstrap-only qualification.
    # Normal coverage still downloads the exact candidate report before preflight.
    if local_only:
        return
    path = Path(os.environ.get('RUNNER_TEMP', '/nonexistent')) / 'thinkthen-apple-coverage/totals.json'
    def identity():
        payload = json.loads(path.read_text())
        require(payload.get('source') == os.environ.get('SHA') and payload.get('platform_collection') == 'apple',
                'Apple coverage input must match the candidate SHA and Apple platform')
    check.check('same-SHA Apple coverage input', identity)


def dart_locks(check):
    pub_cache = Path(os.environ.get('PUB_CACHE', '/nonexistent'))
    for project in ('libraries/dart', 'libraries/dart/checks/session_consumer', 'libraries/dart/flutter/example'):
        check.file(ROOT / project / '.dart_tool/package_config.json')
        def locked_packages(project=project):
            import yaml
            lock = yaml.safe_load((ROOT / project / 'pubspec.lock').read_text())['packages']
            for name, package in lock.items():
                if package['source'] != 'hosted':
                    continue
                folder = name + '-' + package['version']
                check.file(pub_cache / 'hosted/pub.dev' / folder / 'pubspec.yaml')
                digest = package['description']['sha256']
                stamp = pub_cache / 'hosted-hashes/pub.dev' / (folder + '.sha256')
                check.check(str(stamp), lambda stamp=stamp, digest=digest: require(
                    stamp.read_text().strip() == digest, 'pub cache hash differs from frozen lock'))
        check.check(project + ' frozen Dart inputs', locked_packages)


def qualify(check, stage, target):
    coverage = stage in ('coverage', 'apple-coverage')
    rust(check, coverage)
    cache = Path(os.environ.get('THINKTHEN_TOOLCHAINS', Path.home() / '.cache/thinkthen-toolchains'))
    if stage == 'apple-coverage':
        check.tools(('cc', 'swift', 'xcrun', 'lipo'))
        check.command([sys.executable, '-c', 'from jsonschema import Draft202012Validator'])
        cargo_cache(check, ['Cargo.toml', 'libraries/c/Cargo.toml'])
        return
    if stage == 'smoke':
        downloaded_inputs(check, target)
    windows = target.endswith('windows-msvc')
    manifests = ['Cargo.toml', 'libraries/c/Cargo.toml', 'libraries/python/Cargo.toml', 'libraries/typescript/Cargo.toml']
    if not windows:
        manifests += ['databases/sqlite/Cargo.toml', 'databases/duckdb/Cargo.toml',
                      'crates/thinkthen-host/Cargo.toml', 'databases/duckdb/bridge/Cargo.toml',
                      'databases/postgresql/Cargo.toml', 'libraries/ruby/Cargo.toml', 'libraries/polars/consumer/Cargo.toml']
    cargo_cache(check, manifests)
    if stage == 'container':
        check.tools(('cc', 'c++', 'make', 'cmake', 'ninja', 'cargo-pgrx', 'rpm2archive'))
        check.command(['maturin', '--version'], r'^maturin 1\.15\.0\s*$')
        native_inputs(check, target, cache, built_hosts=False, source_only=True)
        check.file(Path(os.environ.get('LIBCLANG_PATH', '/nonexistent')) / 'libclang.so')
        check.file(Path(os.environ.get('THINKTHEN_RUBY_OPENSSL', '/nonexistent')) / 'include/openssl/ssl.h')
        for major in (16, 17, 18):
            postgres = Path('/work/home/pg/usr') / ('pgsql-' + str(major))
            for binary in ('pg_config', 'postgres', 'initdb', 'pg_ctl', 'psql'):
                check.file(postgres / 'bin' / binary, True)
            check.file(postgres / 'include/server/postgres.h')
        check.command([os.environ.get('PYO3_PYTHON', '/nonexistent'), '-c', 'import sys; assert sys.version_info[:2] == (3, 10)'])
        return
    check.command(['python3', '-c', 'from jsonschema import Draft202012Validator; import maturin'])
    bootstrap = (ROOT / 'sdlc/scripts/release-workflow').read_text()
    maturin = re.search(r"'maturin==([^']+)'", bootstrap)[1]
    check.command(['maturin', '--version'], '^maturin ' + re.escape(maturin) + r'\s*$')
    if windows:
        check.tools(('cl.exe', 'lib.exe', 'link.exe'))
        return
    check.tools(('cc', 'nm', 'jq'))
    if stage == 'build' and 'linux' in target:
        check.tools(('docker',))
        check.command(['docker', 'info'])
        if target == 'x86_64-unknown-linux-gnu':
            managed(check)
        return
    if stage == 'build':
        check.tools(('cc', 'c++', 'make', 'pkg-config'))
        native_inputs(check, target, cache, built_hosts=False)
        return
    installed(check, target, cache, stage == 'smoke')
    if stage in ('core', 'functional', 'coverage'):
        install = (ROOT / 'sdlc/scripts/install').read_text()
        check.tools(shlex.split(re.search(r'for tool in ([^;]+); do', install)[1]))
        check.tools(('ld', 'objcopy', 'ar', 'mustmatch', 'script', 'cargo-nextest', 'cargo-pgrx', 'cbindgen', 'musl-gcc', 'xvfb-run', 'xauth', 'clang'))
        pins = dict(re.findall(r'^([A-Z_]+)=([^\n]+)$', install, re.M))
        check.command(['python3', '-c', 'import sys; assert sys.version_info >= (3, 12)'])
        check.command(['jq', '--version'], '^' + re.escape(pins['JQ_VERSION']) + r'\s*$')
        check.command(['gcc', '--version'], r'\b(?:gcc|GCC)\b')
        check.command(['swift', 'format', '--help'])
        check.command(['sh', '-ec', '. libraries/ruby/toolchain.env; ruby_source_host'])
        check.tools(('prlimit',))
        check.check('nonroot Linux routine host', lambda: require(os.getuid() != 0, 'RLIMIT_NPROC needs a nonroot user'))
        check.command(['cargo', 'deny', '--version'], '^cargo-deny ' + re.escape(pins['DENY_VERSION']) + r'\s*$')
        check.command(['cargo', 'public-api', '--version'], '^cargo-public-api ' + re.escape(pins['PUBLIC_API_VERSION']) + r'\s*$')
        check.command(['rustc', '+' + pins['PUBLIC_API_TOOLCHAIN'], '--version'], re.escape(pins['PUBLIC_API_RUSTC_DATE']))
        check.command(['python3', '-c', 'import yaml; print(yaml.__version__)'], '^' + re.escape(pins['PYYAML_VERSION']) + r'\s*$')
        advisory = Path(os.environ.get('CARGO_HOME', str(Path.home() / '.cargo'))) / 'advisory-dbs'
        databases = list(advisory.glob('*/.git'))
        check.check('cargo-deny advisory cache', lambda: require(databases, 'fetched advisory database missing'))
        for database in databases:
            check.command(['git', '-C', database.parent, 'rev-parse', '--verify', 'HEAD'])
        generator = runpy.run_path(str(ROOT / 'sdlc/scripts/generate-c-header.py'))
        check.check('pinned cbindgen', generator['check_tool'])
        workflow = (ROOT / '.github/workflows/gate.yml').read_text()
        fetched = re.search(r'for manifest in ([^;]+); do', workflow)[1].replace(chr(92) + chr(10), '')
        cargo_cache(check, shlex.split(fetched))
        work = Path(os.environ.get('RELEASE_WORK', '/nonexistent'))
        check.check('hosted npm cache handoff', lambda: require(
            os.environ.get('npm_config_cache') == str(work / 'npm-cache'),
            'offline npm cache must survive the qualification home change'))
        check.command([work / 'python/bin/python', '-c', 'import numpy, pandas, pyarrow'])
        check.file(work / 'typescript/node_modules/.bin/tsc', True)
        for key, binary in (('THINKTHEN_JDK_HOME', 'javac'), ('THINKTHEN_KOTLIN_HOME', 'kotlinc'), ('THINKTHEN_SCALA_HOME', 'scalac')):
            expected = Path(os.environ.get(key, '/nonexistent')) / 'bin' / binary
            check.check(binary + ' nested PATH', lambda binary=binary, expected=expected:
                        require(Path(shutil.which(binary) or '/nonexistent').resolve() == expected.resolve(), 'selected JVM tool missing from nested PATH'))
        for key in ('TT_DART', 'TT_FLUTTER'):
            check.file(os.environ.get(key, '/nonexistent'), True)
        check.file(Path(os.environ.get('LIBCLANG_PATH', '/nonexistent')) / 'libclang.so.1')
        flutter = Path.home() / '.local/opt/flutter'
        workflow = (ROOT / '.github/workflows/gate.yml').read_text()
        source = re.search(r'test "\$\(git -C "\$HOME/.local/opt/flutter" rev-parse HEAD\)" = ([0-9a-f]{40})', workflow)[1]
        check.command(['git', '-C', flutter, 'rev-parse', 'HEAD'], '^' + source + r'\s*$')
        for asset in ('bin/cache/dart-sdk/bin/dart', 'bin/cache/artifacts/engine/linux-x64', 'packages/flutter_tools/.dart_tool/package_config.json'):
            path = flutter / asset
            check.check(str(path), lambda path=path: require(path.exists(), 'prepared Flutter asset missing'))
        check.check('frozen Dart inputs', lambda: dart_locks(check))
        if coverage:
            apple_input(check, os.environ.get('PREFLIGHT_ONLY') == 'true')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('stage', choices=('core', 'functional', 'coverage', 'build', 'smoke', 'apple-coverage', 'container'))
    parser.add_argument('target')
    args = parser.parse_args()
    os.environ.update(RUSTUP_AUTO_INSTALL='0', CARGO_NET_OFFLINE='true', HOMEBREW_NO_AUTO_UPDATE='1')
    check = Preflight()
    check.check('prepared host inputs', lambda: qualify(check, args.stage, args.target))
    return check.finish()


if __name__ == '__main__':
    sys.dont_write_bytecode = True
    sys.exit(main())
