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
    required = ('cc', 'make', 'cmake', 'pkg-config', 'node', 'go', 'ruby')
    if any(not shutil.which(name) for name in required):
        if platform.system() == 'Darwin':
            run('brew', 'install', 'cmake', 'pkgconf', 'gettext', 'node@22', 'go', 'ruby@3.3')
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
        node_major = int(prepared('node', '-p', 'process.versions.node.split(".")[0]', capture_output=True, text=True).stdout)
    except (FileNotFoundError, subprocess.CalledProcessError):
        node_major = 0
    if node_major < 22:
        name = f'node-v{HOST_PINS["node"]}-{system}-{"arm64" if arm else "x64"}'
        base = 'https://nodejs.org/dist/v' + HOST_PINS['node']
        filename = name + '.tar.gz'
        archive = fetch(base + '/' + filename, TOOLS / 'sources' / filename)
        with tarfile.open(archive) as source:
            source.extractall(TOOLS, filter='data')
        (TOOLS / name).rename(TOOLS / 'node')
    if system == 'linux':
        prepare_go()
    else:
        try:
            go_version = re.search(r'go([0-9]+)\.([0-9]+)', prepared('go', 'version', capture_output=True, text=True).stdout)
            go_ok = go_version and tuple(map(int, go_version.groups())) >= (1, 22)
        except (FileNotFoundError, subprocess.CalledProcessError):
            go_ok = False
        if not go_ok:
            filename = f'go{HOST_PINS["go"]}.{system}-{"arm64" if arm else "amd64"}.tar.gz'
            with tarfile.open(fetch('https://go.dev/dl/' + filename, TOOLS / 'sources' / filename)) as source:
                source.extractall(TOOLS, filter='data')
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
    compiler = (Path(go_command()) if platform.system() == 'Linux' else
                Path(os.environ.get('THINKTHEN_GO_HOME', TOOLS / ('go' + HOST_PINS['go']))) / 'bin/go')
    cache = TOOLS / 'staticcheck' / PINS['staticcheck']
    environment = dict(os.environ, GOTOOLCHAIN='local', CGO_ENABLED='0',
                       GOPROXY='https://proxy.golang.org', GOSUMDB='sum.golang.org',
                       GOPRIVATE='', GONOSUMDB='', GOBIN=str(Path.home() / '.local/bin'),
                       GOPATH=str(cache), GOMODCACHE=str(cache / 'modules'))
    actual = subprocess.check_output([compiler, 'env', 'GOVERSION'], env=environment, text=True).strip()
    if actual != 'go' + HOST_PINS['go']:
        raise ValueError('prepare staticcheck with the pinned Go floor compiler')
    Path(environment['GOBIN']).mkdir(parents=True, exist_ok=True)
    run(compiler, 'install', 'honnef.co/go/tools/cmd/staticcheck@' + PINS['staticcheck'], env=environment)
    binary = Path(environment['GOBIN']) / 'staticcheck'
    metadata = subprocess.check_output([compiler, 'version', '-m', binary], env=environment, text=True)
    if '\tmod\thonnef.co/go/tools\t' + PINS['staticcheck'] + '\t' not in metadata:
        raise ValueError('prepared staticcheck differs from the pinned module')
    run(binary, '-version', env=environment)
