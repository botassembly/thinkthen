"""Stage release-only source additions using the checked native packages."""
from pathlib import Path
import shutil
import tarfile

from release_workflow import ROOT, version

RIDS = {'linux-x86_64': 'linux-x64', 'linux-arm64': 'linux-arm64',
        'macos-arm64': 'osx-arm64', 'windows-x86_64': 'win-x64'}
GO = {'linux-x86_64': ('linux', 'amd64'), 'linux-arm64': ('linux', 'arm64'),
      'macos-arm64': ('darwin', 'arm64'), 'windows-x86_64': ('windows', 'amd64')}


def stage(libraries, destination, notices):
    """Return additions for an off-main commit, never mutate the source checkout."""
    go = destination / 'sdk/go'
    shutil.copytree(ROOT / 'sdk/go', go, ignore=shutil.ignore_patterns('native', 'library.go', 'tests'))
    # The tag overlays main, so replace its one-platform declaration as well.
    (go / 'library.go').write_text('package thinkthen\n')
    shutil.copy2(ROOT / 'crates/thinkthen-ffi/include/thinkthen.h', go / 'thinkthen.h')
    shutil.copy2(ROOT / 'LICENSE', go / 'LICENSE')
    swift = destination / 'sdk/swift'
    shutil.copytree(ROOT / 'sdk/swift', swift, ignore=shutil.ignore_patterns('Native', '.build'))
    shutil.copy2(ROOT / 'crates/thinkthen-ffi/include/thinkthen.h',
                 swift / 'Sources/CThinkThen/include/thinkthen.h')
    for target, library in libraries.items():
        system, arch = GO[target]
        relative = f'native/{system}-{arch}/{library.name}'
        asset = go / relative
        asset.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(library, asset)
        (go / f'library_{system}_{arch}.go').write_text(
            f'//go:build {system} && {arch}\n\npackage thinkthen\n\nimport _ "embed"\n\n'
            f'//go:embed {relative}\nvar library []byte\n')
        asset = swift / 'Sources/ThinkThen/Native' / RIDS[target] / library.name
        asset.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(library, asset)
    from release_notices import combine
    contents = combine(notices)
    for folder in (go, swift):
        for name, data in contents.items():
            path = folder / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
    shutil.copy2(swift / 'Package.swift', destination / 'Package.swift')


def archive_source(destination, output):
    with tarfile.open(output, 'w:gz') as archive:
        for path in sorted(destination.iterdir()):
            archive.add(path, arcname=path.name)
