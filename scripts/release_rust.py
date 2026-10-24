"""Prepare Rust through a local mirror containing only hash-pinned archives."""
import hashlib
import os
import platform
from pathlib import Path
import tomllib
from urllib.parse import urlparse

from native_platform import current
from release_prepare_common import ROOT, TOOLS, fetch, prepared, run


def prepare(target=None):
    config = tomllib.loads((ROOT / 'rust-toolchain.toml').read_text())['toolchain']
    channel = config['channel']
    host = 'x86_64-pc-windows-msvc' if platform.system() == 'Windows' else current()['rust']
    execute = run if platform.system() == 'Windows' else prepared
    installed = execute('rustup', 'toolchain', 'list', capture_output=True, text=True).stdout
    if channel + '-' + host in installed:
        components = execute('rustup', 'component', 'list', '--installed', '--toolchain', channel,
                              capture_output=True, text=True).stdout.splitlines()
        needed = ['cargo-' + host, 'rustc-' + host, 'rust-std-' + host]
        needed += [name.removesuffix('-preview') + '-' + host for name in config['components']]
        if target: needed.append('rust-std-' + target)
        if all(name in components for name in needed): return
    mirror = TOOLS / 'rust-dist'
    url = f'https://static.rust-lang.org/dist/channel-rust-{channel}.toml'
    manifest = fetch(url, mirror / urlparse(url).path.lstrip('/'))
    manifest.with_name(manifest.name + '.sha256').write_text(hashlib.sha256(manifest.read_bytes()).hexdigest() + '  ' + manifest.name + '\n')
    metadata = tomllib.loads(manifest.read_text())
    packages = [(name, host) for name in ('cargo', 'rustc', 'rust-std', 'clippy-preview', 'rustfmt-preview', 'llvm-tools-preview')]
    if target: packages.append(('rust-std', target))
    for name, target_platform in packages:
        archive = metadata['pkg'][name]['target'][target_platform]['xz_url']
        fetch(archive, mirror / urlparse(archive).path.lstrip('/'))
    env = dict(os.environ, RUSTUP_DIST_SERVER=mirror.as_uri())
    args = ['rustup', 'toolchain', 'install', channel, '--profile', 'minimal', '--no-self-update']
    for component in config['components']: args += ['--component', component]
    if target: args += ['--target', target]
    execute(*args, env=env)
