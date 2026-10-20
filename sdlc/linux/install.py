#!/usr/bin/env python3
"""Install individual Linux image layers and share their manifest contract."""
import gzip
import hashlib
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
import urllib.request

ROOT = Path(__file__).resolve().parents[2]


def download(url, path, sha256=None):
    """Save HTTP representation bytes before checking a pinned content digest."""
    request = urllib.request.Request(url, headers={'Accept-Encoding': 'identity'})
    with tempfile.TemporaryDirectory(prefix='.' + path.name + '-', dir=path.parent) as temporary:
        staged = Path(temporary) / 'download'
        digest = hashlib.sha256()
        with urllib.request.urlopen(request, timeout=60) as response, staged.open('wb') as output:
            encoding = response.headers.get('Content-Encoding', 'identity').strip().lower()
            if encoding not in ('identity', 'gzip'):
                raise RuntimeError('unsupported download content encoding: ' + encoding)
            with gzip.GzipFile(fileobj=response) if encoding == 'gzip' else response as source:
                for part in iter(lambda: source.read(1024 * 1024), b''):
                    output.write(part)
                    digest.update(part)
        if sha256 is not None and digest.hexdigest() != sha256:
            raise RuntimeError('download checksum differs from tools.json')
        staged.replace(path)


def run(argv, env, **kwargs):
    return subprocess.run(list(map(str, argv)), env=env, check=True, **kwargs)


def load_manifest(path=None):
    """Read shared tool requirements and each language's own requirements."""
    source = Path(path) if path is not None else ROOT / 'sdlc/tools.json'
    manifest = json.loads(source.read_text())
    for fragment in sorted(source.with_suffix('').glob('*.json')):
        for section, entries in json.loads(fragment.read_text()).items():
            combined = manifest.setdefault(section, {})
            overlap = combined.keys() & entries.keys()
            if overlap:
                raise ValueError(f'{fragment}: duplicate {section}: {sorted(overlap)}')
            combined.update(entries)
    return manifest


def image_dockerfile(template, fragments):
    """Append new fragment layers after the established core layer order."""
    layers = []
    for relative in fragments:
        if 'COPY ' + relative + ' ' not in template:
            name = Path(relative).name
            layers += ['COPY ' + relative + ' /tmp/tools/' + name,
                       'RUN python3 /opt/thinkthen-image/sdlc/linux/install.py /tmp/tools.json /tmp/tools/' + name]
    return template.replace('# Additional fragment layers.', '\n'.join(layers))


def linux_requirements(manifest):
    """Install core requirements and every fragment's declared Linux provider."""
    core = selected(manifest, manifest['linux']['tool_groups'], 'linux')
    return {name: tool for name, tool in manifest['tools'].items()
            if name in core or ('linux' in tool['platforms']
                                and tool.get('linux_provider', tool.get('provider')))}


def image_environment(manifest, host=None):
    return ((host or platform.system().lower()) == 'linux'
            and (Path(manifest['linux_prefix']) / 'image-ready').is_file())


def surface_paths(manifest, host=None, registry=None):
    """Select prepared image surfaces; native hosts keep their full inventory."""
    host = host or platform.system().lower()
    arch = {'aarch64': 'arm64'}.get(platform.machine(), platform.machine())
    target = host + '-' + arch
    candidates = [f'libraries/{name}' for name, row in manifest['libraries'].items()
                  if target in row['check_platforms']]
    candidates += [f'databases/{name}' for name, targets in manifest['databases'].items()
                   if target in targets]
    if image_environment(manifest, host):
        registry = registry or ROOT / 'sdlc/surfaces.txt'
        registered = {fields[0] for line in registry.read_text().splitlines()
                      if (fields := line.split()) and not line.startswith('#')
                      and len(fields) > 1 and fields[1] in ('landed', 'feature')}
        candidates = [path for path in candidates if path in registered]
        available = linux_requirements(manifest)
        candidates = [path for path in candidates
                      if set(selected(manifest, [path.split('/')[1]], host)) <= available.keys()]
    return candidates


def selected(manifest, groups, host):
    groups = set(groups)
    if 'linux-image' in groups:
        return linux_requirements(manifest)
    if 'surfaces' in groups:
        if image_environment(manifest, host):
            groups.update(path.split('/')[1] for path in surface_paths(manifest, host))
        else:
            arch = {'aarch64': 'arm64'}.get(platform.machine(), platform.machine())
            target = host + '-' + arch
            groups.update(name for name, row in manifest['libraries'].items()
                          if target in row['check_platforms'])
            groups.update(manifest['databases'])
    return {name: tool for name, tool in manifest['tools'].items()
            if host in tool['platforms'] and set(tool['groups']) & {'common', *groups}}


def rust_toolchain(manifest, env):
    """Read the repository's Rust contract with its prepared Python."""
    tool = manifest['tools']['rust']
    source = ROOT / tool['toolchain_file']
    contract = json.loads(subprocess.check_output(['python3', '-c',
        'import json, sys, tomllib; '
        'print(json.dumps(tomllib.load(open(sys.argv[1], "rb"))["toolchain"]))',
        str(source)], env=env, text=True))
    if contract['channel'] != tool['version']:
        raise RuntimeError('Rust manifest version differs from rust-toolchain.toml')
    return contract, [*contract['components'], *tool.get('components', [])]


def rust_components(manifest, env, install=False):
    contract, components = rust_toolchain(manifest, env)
    version = contract['channel']
    if install:
        run(['rustup', 'toolchain', 'install', version, '--profile', contract['profile'],
             *[arg for component in components for arg in ('--component', component)]], env)
    installed = subprocess.check_output(['rustup', 'component', 'list', '--toolchain', version,
                                        '--installed'], env=env, text=True).splitlines()
    missing = [component for component in components
               if not any(line == component or line.startswith(component.removesuffix('-preview') + '-')
                          for line in installed)]
    if missing:
        raise RuntimeError('Rust components missing: ' + ', '.join(missing))


def swift_sdk(tool, env, manifest, root, arch):
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


def main():
    manifest = load_manifest(sys.argv[1])
    root = Path(manifest['linux_prefix'])
    root.mkdir(parents=True, exist_ok=True)
    arch = {'aarch64': 'arm64', 'x86_64': 'x64'}[platform.machine()]
    mise = root / 'bootstrap/mise/bin/mise'
    env = dict(HOME=os.environ.get('HOME', '/root'), PATH=str(root / 'bin') + ':' + str(mise.parent) + ':' + os.environ['PATH'],
               MISE_DATA_DIR=str(root / 'mise'), MISE_CACHE_DIR=str(root / 'cache'),
               MISE_CONFIG_DIR=str(root / 'config'), MISE_YES='1', MISE_JOBS='2',
               MISE_CARGO_BINSTALL='1', MISE_RUBY_COMPILE='0', MISE_PYTHON_COMPILE='0',
               RUSTUP_HOME=str(root / 'rustup'), CARGO_HOME=str(root / 'cargo'),
               CARGO_BUILD_JOBS='2', MAKEFLAGS='-j2', MAKE_OPTS='-j2', MAKEOPTS='-j2',
               CARGO_PROFILE_RELEASE_DEBUG='0')
    if sys.argv[2] == 'finish':
        (root / 'image-ready').touch()
        # Skip already-readable files so Docker does not copy earlier layers.
        for path in (root, *root.rglob('*')):
            if path.is_symlink():
                continue
            mode = path.stat().st_mode & 0o7777
            readable = mode | 0o444 | (0o111 if path.is_dir() or mode & 0o111 else 0)
            if readable != mode:
                path.chmod(readable)
        return
    if sys.argv[2] == 'bootstrap':
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
        (root / 'bin').mkdir()
        (root / 'bin/mise').symlink_to(mise)
        identity = 'aqua:cargo-bins/cargo-binstall@' + manifest['linux']['binstall_version']
        subprocess.run([str(mise), 'install', identity], env=env, check=True)
        resolved = json.loads(subprocess.check_output([str(mise), 'env', '--json', identity], env=env))
        executable = shutil.which('cargo-binstall', path=resolved['PATH'])
        if executable is None:
            raise SystemExit('installed provider lacks cargo-binstall')
        (root / 'bin/cargo-binstall').symlink_to(executable)
        return
    fragment = json.loads(Path(sys.argv[2]).read_text())
    selected_requirements = linux_requirements(manifest)
    requirements = {name: tool for name, tool in fragment.get('tools', {}).items()
                    if name in selected_requirements}
    for name, tool in requirements.items():
        if 'linux' not in tool['platforms']:
            continue
        provider = tool.get('linux_provider', tool.get('provider'))
        if name == 'swift':
            provider = name
        if provider == 'ruby-bundled':
            destination = root / tool.get('linux_directory', 'bin')
            ruby = (destination / 'ruby').resolve()
            for command in tool['commands']:
                executable = ruby.with_name(command)
                if not executable.is_file():
                    raise SystemExit('installed provider lacks ' + command)
                (destination / command).symlink_to(executable)
            continue
        if provider is None or provider == 'python-package':
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
            resolved = {'PATH': str(binaries / 'bin') + ':' + env['PATH']}
        elif name == 'swift':
            resolved = swift_sdk(tool, env, manifest, root, arch)
        else:
            subprocess.run([str(mise), 'install', identity], env=env, check=True)
            resolved = json.loads(subprocess.check_output([str(mise), 'env', '--json', identity], env=env))
        paths = resolved['PATH'].split(os.pathsep)
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
        env.update(resolved)
        env['PATH'] = str(root / 'bin') + ':' + env['PATH']
        if name == 'rust':
            rust_components(manifest, env, install=True)
    packages = dict(manifest['python_packages']) if 'python' in requirements else {}
    packages.update({tool['commands'][0]: tool['version'] for tool in requirements.values()
                     if tool.get('provider') == 'python-package'})
    if packages:
        python = root / 'bin/python3'
        subprocess.run([str(python), '-m', 'pip', 'install', '--disable-pip-version-check',
                        *[name + '==' + version for name, version in packages.items()]], env=env, check=True)
        scripts = subprocess.check_output([str(python), '-c', 'import sysconfig; print(sysconfig.get_path("scripts"))'],
                                          env=env, text=True).strip()
        for tool in requirements.values():
            if tool.get('provider') == 'python-package':
                for command in tool['commands']:
                    (root / 'bin' / command).symlink_to(Path(scripts) / command)


if __name__ == '__main__':
    main()
