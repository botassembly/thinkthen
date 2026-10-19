#!/usr/bin/env python3
"""Fill the Linux image prefix from the shared tool manifest."""
import json
import os
from pathlib import Path
import platform
import re
import shlex
import shutil
import subprocess
import sys
import tarfile
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from conformance.children.children import child_env
from sdlc.scripts.tools import download, rust_components

manifest = json.loads(Path(sys.argv[1]).read_text())
root = Path(manifest['linux_prefix'])
root.mkdir(parents=True, exist_ok=True)
arch = {'aarch64': 'arm64', 'x86_64': 'x64'}[platform.machine()]
pin = manifest['bootstrap']
archive = root / 'mise.tar.gz'
url = 'https://github.com/jdx/mise/releases/download/v{0}/mise-v{0}-linux-{1}.tar.gz'.format(pin['version'], arch)
download(url, archive, pin['linux_sha256'][arch])
with tarfile.open(archive) as packed:
    for member in packed.getmembers():
        if Path(member.name).is_absolute() or '..' in Path(member.name).parts or member.issym() or member.islnk():
            raise SystemExit('unsafe mise archive')
    packed.extractall(root / 'bootstrap')
archive.unlink()
mise = root / 'bootstrap/mise/bin/mise'
env = child_env(keep=('HOME',), PATH=str(mise.parent) + ':' + os.environ['PATH'], MISE_DATA_DIR=str(root / 'mise'), MISE_CACHE_DIR=str(root / 'cache'),
           MISE_CONFIG_DIR=str(root / 'config'), MISE_YES='1', MISE_JOBS='2',
           MISE_CARGO_BINSTALL='1', MISE_RUBY_COMPILE='0', MISE_PYTHON_COMPILE='0',
           RUSTUP_HOME=str(root / 'rustup'), CARGO_HOME=str(root / 'cargo'),
           CARGO_BUILD_JOBS='2', MAKEFLAGS='-j2', MAKE_OPTS='-j2', MAKEOPTS='-j2',
           CARGO_PROFILE_RELEASE_DEBUG='0')
(root / 'bin').mkdir()
(root / 'bin/mise').symlink_to(mise)
# cargo-binstall must exist before mise handles cargo:* providers. It selects
# upstream binaries first; the existing locked Cargo helper fallback follows below.
subprocess.run([str(mise), 'install', 'aqua:cargo-bins/cargo-binstall@' + manifest['linux']['binstall_version']], env=env, check=True)
resolved = json.loads(subprocess.check_output([str(mise), 'env', '--json', 'aqua:cargo-bins/cargo-binstall@' + manifest['linux']['binstall_version']], env=env))
env['PATH'] = resolved['PATH']


def swift_sdk(tool, env):
    # Swift spells the minor release's zero patch explicitly in archive names.
    version = tool['version'] + ('.0' if tool['version'].count('.') == 1 else '')
    suffix = '-aarch64' if arch == 'arm64' else ''
    name = 'swift-' + version + '-RELEASE-ubuntu24.04' + suffix
    url = ('https://download.swift.org/swift-' + version + '-release/ubuntu2404' + suffix +
           '/swift-' + version + '-RELEASE/' + name + '.tar.gz')
    keys = manifest['linux']['swift_signing_keys']
    destination = root / 'swift'
    destination.mkdir()
    with tempfile.TemporaryDirectory(prefix='thinkthen-swift-') as temporary:
        temporary = Path(temporary)
        for target, source in (('sdk.tar.gz', url), ('sdk.sig', url + '.sig'), ('keys.asc', keys['url'])):
            download(source, temporary / target, keys['sha256'] if target == 'keys.asc' else None)
        keyring = temporary / 'keyring'
        keyring.mkdir(mode=0o700)
        gpg = ['gpg', '--batch', '--homedir', str(keyring)]
        subprocess.run([*gpg, '--import', str(temporary / 'keys.asc')], env=env, check=True)
        subprocess.run([*gpg, '--verify', str(temporary / 'sdk.sig'), str(temporary / 'sdk.tar.gz')],
                       env=env, check=True)
        with tarfile.open(temporary / 'sdk.tar.gz') as packed:
            packed.extractall(destination, filter='data')
    binary = destination / name / 'usr/bin'
    target = ('aarch64' if arch == 'arm64' else 'x86_64') + '-unknown-linux-gnu'
    for command in tool['commands']:
        shown = subprocess.check_output([str(binary / command), '--version'], env=env, text=True)
        found = re.search(r'\bSwift version (\d+\.\d+(?:\.\d+)?)\b', shown)
        actual = found[1] if found else ''
        if actual.count('.') == 1:
            actual += '.0'
        if actual != version or 'Target: ' + target not in shown:
            raise SystemExit('Swift binary differs from the manifest release or architecture')
        print(shown, flush=True)
    return {'PATH': str(binary) + ':' + env['PATH']}


for name, tool in sorted(manifest['tools'].items(), key=lambda entry: entry[0] != 'rust'):
    if 'linux' not in tool['platforms']:
        continue
    provider = tool.get('linux_provider', tool.get('provider'))
    if name in ('swift', 'flutter'):
        provider = name
    if name == 'duckdb':
        provider = 'aqua:duckdb/duckdb'
        lines = Path('/tmp/duckdb-version.env').read_text().splitlines()
        tool = dict(tool, version=next(line.split('=', 1)[1].removeprefix('v') for line in lines
                                       if line.startswith('DUCKDB_ROUTINE_RELEASE=')))
    if provider is None or provider in ('python-package', 'ruby-bundled'):
        continue
    if provider == 'rustup':
        subprocess.run([str(root / 'bin/rustup'), 'toolchain', 'install', tool['version'],
                        '--profile', 'minimal'], env=env, check=True)
        continue
    identity = provider + '@' + tool['version']
    if provider.startswith('cargo:'):
        binaries = root / 'cargo-tools'
        acquired = subprocess.run(['cargo-binstall', '--no-confirm', '--strategies', 'crate-meta-data',
                                   '--root', str(binaries), '--version', '=' + tool['version'], provider[6:]], env=env)
        if acquired.returncode:
            # Existing helper acquisition; the compiler itself comes from the
            # pinned official Rust binary distribution installed above.
            subprocess.run(['cargo', 'install', '--locked', '--root', str(binaries),
                            '--version', '=' + tool['version'], provider[6:]], env=env, check=True)
        selected = {'PATH': str(binaries / 'bin') + ':' + env['PATH']}
    elif name == 'swift':
        selected = swift_sdk(tool, env)
    else:
        acquired = subprocess.run([str(mise), 'install', identity], env=env)
        if acquired.returncode and name == 'flutter':
            # This is the framework source, paired with downloaded engine and
            # Dart compiler binaries. Never build a compiler from source.
            framework = root / 'flutter'
            subprocess.run(['git', 'init', str(framework)], env=env, check=True)
            subprocess.run(['git', '-C', str(framework), 'remote', 'add', 'origin',
                            'https://github.com/flutter/flutter.git'], env=env, check=True)
            subprocess.run(['git', '-C', str(framework), 'fetch', '--depth=1', 'origin',
                            tool['source_commit']], env=env, check=True)
            subprocess.run(['git', '-C', str(framework), 'checkout', '--detach', 'FETCH_HEAD'], env=env, check=True)
            head = subprocess.check_output(['git', '-C', str(framework), 'rev-parse', 'HEAD'], env=env, text=True).strip()
            if head != tool['source_commit']:
                raise SystemExit('Flutter framework differs from tools.json')
            subprocess.run([str(framework / 'bin/flutter'), 'precache', '--linux'], env=env, check=True)
            selected = {'PATH': str(framework / 'bin') + ':' + env['PATH']}
        else:
            acquired.check_returncode()
            selected = json.loads(subprocess.check_output([str(mise), 'env', '--json', identity], env=env))
    paths = selected['PATH'].split(os.pathsep)
    destination = root / tool.get('linux_directory', 'bin')
    destination.mkdir(parents=True, exist_ok=True)
    for command in tool['commands']:
        executable = shutil.which(command, path=os.pathsep.join(paths))
        if executable is None:
            raise SystemExit('installed provider lacks ' + command)
        target = destination / command
        if not target.exists():
            if command == 'npm':
                # npm's launcher locates its JS relative to argv[0]; a
                # prefix symlink changes that path. Execute its actual path.
                target.write_text('#!/bin/sh\nexec ' + shlex.quote(executable) + ' "$@"\n')
                target.chmod(0o755)
            else:
                target.symlink_to(executable)
    env.update(selected)
    env['PATH'] = str(root / 'bin') + ':' + env['PATH']
    if name == 'rust':
        rust_components(manifest, env, install=True)
python = root / 'bin/python3'
packages = dict(manifest['python_packages'])
packages.update({tool['commands'][0]: tool['version'] for tool in manifest['tools'].values()
                 if tool.get('provider') == 'python-package'})
subprocess.run([str(python), '-m', 'pip', 'install', '--disable-pip-version-check',
                *[name + '==' + version for name, version in packages.items()]], env=env, check=True)
scripts = subprocess.check_output([str(python), '-c', 'import sysconfig; print(sysconfig.get_path("scripts"))'], env=env, text=True).strip()
for name in manifest['tools'].values():
    if name.get('provider') == 'python-package':
        for command in name['commands']:
            (root / 'bin' / command).symlink_to(Path(scripts) / command)
# The second Ruby carries its own bundled RBS and must not replace the default.
for name in ('ruby', 'ruby34'):
    tool = manifest['tools'][name]
    ruby = root / tool.get('linux_directory', 'bin') / 'ruby'
    gem_bin = Path(subprocess.check_output([str(ruby), '-rrubygems', '-e', 'print Gem.bindir'], env=env, text=True))
    for command in ('rbs',):
        target = ruby.parent / command
        if not target.exists():
            target.symlink_to(gem_bin / command)
# Keep the binary toolchain readable by the host user's unprivileged container.
subprocess.run(['chmod', '-R', 'a+rX', str(root)], env=env, check=True)

# Missing commands and wrong versions must fail image preparation before checks.
resolver = '/opt/thinkthen-image/sdlc/scripts/tools.py'
groups = {group for tool in manifest['tools'].values() for group in tool['groups']} - {'ruby34'}
subprocess.run([str(python), resolver, '--manifest', sys.argv[1], ','.join(sorted(groups))],
               env=env, check=True)
subprocess.run([str(python), resolver, '--manifest', sys.argv[1], 'ruby34'], env=env, check=True)
