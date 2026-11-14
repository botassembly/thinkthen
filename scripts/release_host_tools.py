"""Prepare ordinary host runtimes before platform dependencies."""
import json
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import tomllib

from release_prepare_common import ROOT, TOOLS, PINS, fetch, run, prepared
from release_go import version as go_version, prepare as prepare_go, command as go_command

IMAGE = (ROOT / 'scripts/floors/Dockerfile').read_text()
HOST_PINS = {'node': re.search(r'node-v([0-9.]+)-linux-x64', IMAGE)[1],
             'go': go_version(),
             'ruby': re.search(r'ruby-([0-9.]+)\.tar.gz', IMAGE)[1]}


def host_tools():
    if not shutil.which('uv'):
        run(sys.executable, '-m', 'pip', 'install', '--disable-pip-version-check', 'uv==' + PINS['uv'])
    prepared('uv', 'python', 'install', '3.10', '3.13')
    required = ('cc', 'make', 'cmake', 'pkg-config', 'node', 'ruby')
    if any(not shutil.which(name) for name in required):
        if platform.system() == 'Darwin':
            run('brew', 'install', 'cmake', 'pkgconf', 'gettext', 'node@22', 'ruby@3.3')
        else:
            run('sudo', 'apt-get', 'update')
            run('sudo', 'apt-get', 'install', '-y', 'build-essential', 'cmake', 'ninja-build',
                'pkg-config', 'curl', 'libssl-dev', 'libyaml-dev', 'zlib1g-dev')
    host_languages()
    if platform.system() == 'Darwin': macos_swift()
    from release_rust import prepare as prepare_rust
    prepare_rust()
    prepared('cargo', 'fetch', '--locked')
    requirement = tomllib.loads((ROOT / 'sdk/python/pyproject.toml').read_text())['build-system']['requires'][0]
    prepared('uv', 'run', '--no-project', '--python', '3.10', '--with', requirement, '--', 'maturin', '--version')


def host_languages():
    system = 'darwin' if platform.system() == 'Darwin' else 'linux'
    arm = platform.machine() in ('arm64', 'aarch64')
    try:
        node_version = prepared('node', '-p', 'process.versions.node', capture_output=True, text=True).stdout.strip()
    except (FileNotFoundError, subprocess.CalledProcessError):
        node_version = ''
    if node_version != HOST_PINS['node']:
        name = f'node-v{HOST_PINS["node"]}-{system}-{"arm64" if arm else "x64"}'
        base = 'https://nodejs.org/dist/v' + HOST_PINS['node']
        filename = name + '.tar.gz'
        archive = fetch(base + '/' + filename, TOOLS / 'sources' / filename)
        with tarfile.open(archive) as source:
            source.extractall(TOOLS, filter='data')
        shutil.rmtree(TOOLS / 'node', ignore_errors=True)
        (TOOLS / name).rename(TOOLS / 'node')
    actual = prepared('node', '-p', 'process.versions.node', capture_output=True, text=True).stdout.strip()
    if actual != HOST_PINS['node']:
        raise ValueError('prepared Node version differs from the pinned release')
    prepare_go()
    try:
        prepared('ruby', '-e', 'exit(Gem::Version.new(RUBY_VERSION) >= Gem::Version.new("3.3") ? 0 : 1)', capture_output=True)
        return
    except (FileNotFoundError, subprocess.CalledProcessError):
        pass
    if system == 'darwin':
        run('brew', 'install', 'ruby@3.3')
        return
    run('sudo', 'apt-get', 'install', '-y', 'libssl-dev', 'libyaml-dev', 'zlib1g-dev')
    archive = fetch(f'https://cache.ruby-lang.org/pub/ruby/3.3/ruby-{HOST_PINS["ruby"]}.tar.gz', TOOLS / 'sources/ruby.tar.gz')
    with tempfile.TemporaryDirectory(prefix='thinkthen-ruby-prepare-') as temporary:
        with tarfile.open(archive) as source:
            source.extractall(temporary, filter='data')
        run('sh', '-eu', '-c', './configure --disable-install-doc --prefix="$1"; make -j2; make install',
            'ruby-prepare', TOOLS / 'ruby/runtime', cwd=Path(temporary) / ('ruby-' + HOST_PINS['ruby']))


def macos_swift():
    minimum = re.search(r'swift-tools-version:\s*([0-9.]+)', (ROOT / 'sdk/swift/Package.swift').read_text())[1]
    actual = prepared('swift', '--version', capture_output=True, text=True).stdout
    installed = re.search(r'Swift version ([0-9.]+)', actual)[1]
    if tuple(map(int, installed.split('.'))) >= tuple(map(int, minimum.split('.'))):
        return
    install_macos_swift()


def install_macos_swift():
    recipe = (ROOT / 'scripts/floors/prepare-swift').read_text()
    version = re.search(r'swift/([0-9.]+)-ubuntu', recipe)[1]
    parent = TOOLS / 'swift'
    parent.mkdir(parents=True, exist_ok=True)
    destination = parent / (version + '-macos')
    if not (destination / 'usr/bin/swift').exists():
        name = f'swift-{version}-RELEASE-osx.pkg'
        archive = fetch(f'https://download.swift.org/swift-{version}-release/xcode/swift-{version}-RELEASE/{name}', TOOLS / 'sources' / name)
        run('pkgutil', '--check-signature', archive)
        with tempfile.TemporaryDirectory(prefix='swift-', dir=parent) as temporary:
            expanded = Path(temporary) / 'expanded'
            run('pkgutil', '--expand-full', archive, expanded)
            toolchain = next(expanded.rglob('usr/bin/swift')).parents[2]
            toolchain.replace(destination)
    selected = parent / 'macos.partial'
    selected.symlink_to(destination.name, target_is_directory=True)
    selected.replace(parent / 'macos')
    commands = parent / 'macos-commands'
    commands.mkdir(exist_ok=True)
    # Expose Swift alone; the toolchain's clang does not select Apple's SDK.
    for name in ('swift', 'swiftc'):
        command = commands / name
        temporary = command.with_suffix('.partial')
        temporary.write_text('#!/bin/sh\nexec "' + str(parent / 'macos/usr/bin' / name) + '" "$@"\n')
        temporary.chmod(0o755)
        temporary.replace(command)


def source_tools():
    """Prepare the pinned Go checker with the selected floor compiler."""
    compiler = Path(go_command())
    cache = TOOLS / 'staticcheck' / PINS['staticcheck']
    environment = dict(os.environ, GOTOOLCHAIN='local', CGO_ENABLED='0',
                       PATH=str(compiler.parent) + os.pathsep + os.environ.get('PATH', ''),
                       GOPROXY='https://proxy.golang.org', GOSUMDB='sum.golang.org',
                       GOPRIVATE='', GONOSUMDB='', GONOPROXY='', GOINSECURE='',
                       GOFLAGS='', GOWORK='off', GOBIN=str(Path.home() / '.local/bin'),
                       GOPATH=str(cache), GOMODCACHE=str(cache / 'modules'))
    actual = subprocess.check_output([compiler, 'env', 'GOVERSION'], env=environment, text=True).strip()
    if actual != 'go' + HOST_PINS['go']:
        raise ValueError('prepare staticcheck with the pinned Go floor compiler')
    Path(environment['GOBIN']).mkdir(parents=True, exist_ok=True)
    if PINS['staticcheck'] == 'v0.8.2':
        prepare_staticcheck_variant(compiler, environment)
    else:
        run(compiler, 'install', 'honnef.co/go/tools/cmd/staticcheck@' + PINS['staticcheck'], env=environment)
    binary = Path(environment['GOBIN']) / 'staticcheck'
    verify_source_checker(binary)
    run(binary, '-version', env=environment)


STATICCHECK_VARIANT = 'thinkthen.dev/build/staticcheck-without-sa4023'


def verify_source_checker(binary):
    """Require honest upstream identity, including the one authorized entry variant."""
    metadata = subprocess.check_output([go_command(), 'version', '-m', binary], text=True)
    rows = [line.split() for line in metadata.splitlines()]
    pin = PINS['staticcheck']
    if pin == 'v0.8.2':
        main = any(row[:2] == ['mod', STATICCHECK_VARIANT] for row in rows)
        upstream = any(row[:3] == ['dep', 'honnef.co/go/tools', pin]
                       and len(row) == 4 and row[3].startswith('h1:') for row in rows)
        if not main or not upstream:
            raise ValueError('prepare the declared Staticcheck v0.8.2 entry variant and verified upstream dependency')
    elif not any(row[:3] == ['mod', 'honnef.co/go/tools', pin] for row in rows):
        raise ValueError('prepared staticcheck differs from the pinned module')


def prepare_staticcheck_variant(compiler, environment):
    """Omit only the crashing SA4023 registration; never alter analyzer libraries."""
    # v0.8.2's SA4023 nilness result indexes a missing return fact on Go 1.27
    # generic methods. This pin therefore lacks its impossible interface/nil
    # comparison diagnostic; go vet and every other analyzer remain enabled.
    module = 'honnef.co/go/tools'
    pin = PINS['staticcheck']
    downloaded = json.loads(subprocess.check_output(
        [compiler, 'mod', 'download', '-json', module + '@' + pin], env=environment, text=True))
    if (downloaded.get('Path'), downloaded.get('Version')) != (module, pin):
        raise ValueError('Staticcheck download identity differs from its pin')
    if not all(downloaded.get(key, '').startswith('h1:') for key in ('Sum', 'GoModSum')):
        raise ValueError('Staticcheck download lacks verified module checksums')
    entry = (Path(downloaded['Dir']) / 'cmd/staticcheck/staticcheck.go').read_text()
    statement = '\tcmd.AddAnalyzers(staticcheck.Analyzers...)\n'
    if entry.count(statement) != 1:
        raise ValueError('Staticcheck entry point no longer has the one expected registration statement')
    replacement = """\tfor _, analyzer := range staticcheck.Analyzers {
\t\tif analyzer.Analyzer.Name != "SA4023" {
\t\t\tcmd.AddAnalyzers(analyzer)
\t\t}
\t}
"""
    with tempfile.TemporaryDirectory(prefix='staticcheck-entry-') as temporary:
        folder = Path(temporary)
        (folder / 'main.go').write_text(entry.replace(statement, replacement))
        (folder / 'go.mod').write_text('module ' + STATICCHECK_VARIANT
                                     + '\n\ngo 1.27\n\nrequire ' + module + ' ' + pin + '\n')
        (folder / 'go.sum').write_text(module + ' ' + pin + ' ' + downloaded['Sum'] + '\n'
                                     + module + ' ' + pin + '/go.mod ' + downloaded['GoModSum'] + '\n')
        run(compiler, 'mod', 'tidy', cwd=folder, env=environment)
        run(compiler, 'mod', 'verify', cwd=folder, env=environment)
        binary = Path(environment['GOBIN']) / 'staticcheck'
        run(compiler, 'build', '-mod=readonly', '-o', binary, '.', cwd=folder, env=environment)
        upstream = folder / 'upstream'
        upstream.mkdir()
        run(compiler, 'install', module + '/cmd/staticcheck@' + pin,
            env=dict(environment, GOBIN=str(upstream)))
        expected = subprocess.check_output([upstream / 'staticcheck', '-list-checks'], text=True)
        actual = subprocess.check_output([binary, '-list-checks'], text=True)
        expected_checks, actual_checks = set(expected.splitlines()), set(actual.splitlines())
        removed = expected_checks - actual_checks
        if actual_checks - expected_checks or len(removed) != 1 or not next(iter(removed)).startswith('SA4023 '):
            raise ValueError('Staticcheck variant registry differs by more than SA4023')
        sentinel = folder / 'sentinel'
        sentinel.mkdir()
        (sentinel / 'go.mod').write_text('module checker-sentinel\n\ngo 1.27\n')
        (sentinel / 'sentinel.go').write_text('package sentinel\nimport "regexp"\nvar _ = regexp.MustCompile("[")\n')
        result = subprocess.run([binary, '.'], cwd=sentinel, env=environment, text=True, capture_output=True)
        if result.returncode == 0 or 'SA1000' not in result.stdout + result.stderr:
            raise ValueError('Staticcheck variant did not report the retained SA1000 sentinel: '
                             + result.stdout + result.stderr)
    print('Staticcheck v0.8.2 entry variant: registry differs only by SA4023; SA1000 sentinel refused', flush=True)
