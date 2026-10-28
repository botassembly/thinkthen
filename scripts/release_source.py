"""Stage release-only source additions using the checked native packages."""
from pathlib import Path
import shutil
import tarfile

from release_workflow import ROOT, version
from release_readme import released_readme

RIDS = {'linux-x86_64': 'linux-x64', 'linux-arm64': 'linux-arm64',
        'macos-arm64': 'osx-arm64', 'windows-x86_64': 'win-x64'}
GO = {'linux-x86_64': ('linux', 'amd64'), 'linux-arm64': ('linux', 'arm64'),
      'macos-arm64': ('darwin', 'arm64'), 'windows-x86_64': ('windows', 'amd64')}


def stage(libraries, destination, notices):
    """Return additions for an off-main commit, never mutate the source checkout."""
    from header_contract import check
    check(ROOT)
    go = destination / 'sdk/go/thinkthen'
    shutil.copytree(ROOT / 'sdk/go/thinkthen', go, ignore=shutil.ignore_patterns('native', 'library.go', 'tests'))
    (go / 'README.md').write_bytes(released_readme('go'))
    # The tag overlays main, so replace its one-platform declaration as well.
    (go / 'library.go').write_text('package thinkthen\n')
    shutil.copy2(ROOT / 'crates/thinkthen-ffi/include/thinkthen.h', go / 'thinkthen.h')
    shutil.copy2(ROOT / 'LICENSE', go / 'LICENSE')
    swift = destination / 'sdk/swift'
    shutil.copytree(ROOT / 'sdk/swift', swift, ignore=shutil.ignore_patterns('Native', '.build', 'tests', 'generate-codes.py'))
    (swift / 'README.md').write_bytes(released_readme('swift'))
    (swift / 'VERSION').write_text(version() + '\n')
    shutil.copy2(ROOT / 'LICENSE', swift / 'LICENSE')
    (swift / 'Sources/CThinkThen/include').mkdir(parents=True, exist_ok=True)
    shutil.copy2(ROOT / 'crates/thinkthen-ffi/include/thinkthen.h',
                 swift / 'Sources/CThinkThen/include/thinkthen.h')
    import subprocess
    import sys
    subprocess.run([sys.executable, str(ROOT / 'sdk/swift/generate-codes.py'),
                    str(ROOT / 'schemas/contract.json'),
                    str(swift / 'Sources/ThinkThen/ErrorCode.swift')], check=True)
    stage_go_libraries(libraries, go)
    for target, library in libraries.items():
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



def stage_go_libraries(libraries, go):
    """Embed platform assets and keep unsupported targets buildable."""
    (go / 'library.go').write_text('package thinkthen\n')
    for target, library in libraries.items():
        system, arch = GO[target]
        relative = f'native/{system}-{arch}/{library.name}'
        asset = go / relative
        asset.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(library, asset)
        (go / f'library_{system}_{arch}.go').write_text(
            f'//go:build !thinkthen_system && {system} && {arch}\n\npackage thinkthen\n\nimport _ "embed"\n\n'
            f'//go:embed {relative}\nvar library []byte\n')
    (go / 'native/dependencies.txt').write_text(''.join(
        f'{target}: {library.name}\n' for target, library in libraries.items()))
    (go / 'library_unsupported.go').write_text(
        '//go:build !thinkthen_system && !(' + ' || '.join(
            f'({system} && {arch})' for system, arch in GO.values()) + ')\n\n'
        'package thinkthen\n\nvar library []byte\n')


def archive_source(destination, output):
    with tarfile.open(output, 'w:gz') as archive:
        for path in sorted(destination.iterdir()):
            archive.add(path, arcname=path.name)
