#!/usr/bin/env python3
"""Install job-owned macOS tools or consume the pinned Linux image's tool prefix."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys
import tarfile
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
SYSTEM_PATH = '/usr/bin:/bin:/usr/sbin:/sbin'


def run(argv, env, **kwargs):
    return subprocess.run(list(map(str, argv)), env=env, check=True, **kwargs)


def selected(manifest, groups, host):
    return {name: tool for name, tool in manifest['tools'].items()
            if host in tool['platforms'] and set(tool['groups']) & {'common', *groups}}


def bootstrap(manifest, root, env):
    binary = root / 'bootstrap/mise/bin/mise'
    if binary.is_file():
        return binary
    pin = manifest['bootstrap']
    arch = {'arm64': 'arm64', 'x86_64': 'x64'}[platform.machine()]
    url = 'https://github.com/jdx/mise/releases/download/v{0}/mise-v{0}-macos-{1}.tar.gz'.format(pin['version'], arch)
    archive = root / 'mise.tar.gz'
    root.mkdir(parents=True, exist_ok=True)
    with urllib.request.urlopen(url, timeout=60) as response, archive.open('wb') as output:
        shutil.copyfileobj(response, output)
    if hashlib.sha256(archive.read_bytes()).hexdigest() != pin['sha256'][arch]:
        raise RuntimeError('tool installer checksum differs from tools.json')
    # The pinned public archive contains only the mise directory.
    with tarfile.open(archive) as packed:
        for member in packed.getmembers():
            if Path(member.name).is_absolute() or '..' in Path(member.name).parts or member.issym() or member.islnk():
                raise RuntimeError('unsafe tool installer archive')
        packed.extractall(root / 'bootstrap')
    archive.unlink()
    return binary


def environment(root):
    # No login shell, inherited PATH or user package-manager configuration.
    env = {name: os.environ[name] for name in (
        'HOME', 'TMPDIR', 'LANG', 'LC_ALL', 'CARGO_HOME', 'CARGO_BUILD_JOBS') if name in os.environ}
    env.update(PATH=SYSTEM_PATH, MISE_DATA_DIR=str(root / 'mise'),
               MISE_CACHE_DIR=str(root / 'cache'), MISE_CONFIG_DIR=str(root / 'config'),
               MISE_GLOBAL_CONFIG_FILE=str(root / 'config/config.toml'),
               MISE_SYSTEM_CONFIG_FILE=str(root / 'config/system.toml'),
               MISE_USE_TOML='0', MISE_YES='1', MISE_CARGO_BINSTALL='0',
               RUSTUP_HOME=str(root / 'rustup'), UV_CACHE_DIR=str(root / 'uv-cache'),
               UV_PYTHON_INSTALL_DIR=str(root / 'python'))
    return env


def resolve(manifest, groups, root, check=False):
    host = platform.system().lower()
    requirements = selected(manifest, groups, host)
    env = environment(root)
    paths = [str(root / 'bin')]
    failures = []
    installer = None
    for name, tool in requirements.items():
        if host != 'darwin' or 'provider' not in tool or tool['provider'] == 'python-package':
            continue
        identity = tool['provider'] + '@' + tool['version']
        try:
            if not check:
                installer = installer or bootstrap(manifest, root, env)
                run([installer, 'install', identity], env)
            installer = installer or root / 'bootstrap/mise/bin/mise'
            folder = subprocess.check_output([str(installer), 'where', identity], env=env, text=True).strip()
            paths.append(str(Path(folder) / 'bin'))
            env['PATH'] = os.pathsep.join(paths + [SYSTEM_PATH])
        except (OSError, subprocess.CalledProcessError, RuntimeError) as error:
            failures.append(name + ': ' + str(error))
    env['PATH'] = os.pathsep.join(paths + [SYSTEM_PATH])
    if host == 'darwin' and not check and not failures:
        packages = dict(manifest['python_packages'])
        packages.update({tool['commands'][0]: tool['version'] for tool in requirements.values()
                         if tool.get('provider') == 'python-package'})
        try:
            run(['python3', '-m', 'pip', 'install', '--disable-pip-version-check',
                 *[name + '==' + version for name, version in packages.items()]], env)
        except subprocess.CalledProcessError as error:
            failures.append('Python tools: ' + str(error))
    missing = [command for tool in requirements.values() for command in tool['commands']
               if shutil.which(command, path=env['PATH']) is None]
    if missing or failures:
        raise RuntimeError('missing tools: ' + ', '.join(sorted(set(missing)))
                           + ('; installation failures: ' + '; '.join(failures) if failures else ''))
    return env


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest', type=Path, default=ROOT / 'sdlc/tools.json')
    parser.add_argument('--root', type=Path, help='job tool folder; Linux image supplies its declared prefix')
    parser.add_argument('--check', action='store_true', help='refuse missing tools without installation')
    parser.add_argument('--list', action='store_true', help='print selected manifest entries for image construction')
    parser.add_argument('groups', help='comma-separated gate or library names')
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    manifest = json.loads(args.manifest.read_text())
    groups = args.groups.split(',')
    if args.list:
        print(json.dumps(selected(manifest, groups, platform.system().lower()), indent=2))
        return 0
    root = (args.root or ROOT / '.tools').resolve()
    if platform.system() == 'Darwin' and ROOT not in root.parents:
        parser.error('macOS tools must live under this job checkout')
    try:
        env = resolve(manifest, groups, root, args.check)
    except (OSError, RuntimeError) as error:
        print('tools: ' + str(error), file=sys.stderr)
        return 2
    command = args.command
    if command[:1] == ['--']:
        command = command[1:]
    if command:
        child = dict(os.environ, **env)
        # Compiler overrides and tool locations must not bypass the resolver.
        for name in list(child):
            if name.startswith('THINKTHEN_') and name.endswith('_BIN'):
                del child[name]
        os.execvpe(command[0], command, child)
    print(env['PATH'])
    return 0


if __name__ == '__main__':
    sys.exit(main())
