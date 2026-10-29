"""Release orchestration; package building and installed checks stay shared."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile

from release_home import checked_home

ROOT = Path(__file__).resolve().parent.parent
TARGETS = {
    'linux-x86_64': 'ubuntu-24.04',
    'linux-arm64': 'ubuntu-24.04-arm',
    'macos-arm64': 'macos-15',
    # The shared Windows recipe builds on Linux; installs run on Windows below.
    'windows-x86_64': 'ubuntu-24.04',
}


def run(*args, **kwargs):
    kwargs.setdefault('cwd', ROOT)
    return subprocess.run(list(map(str, args)), check=True, **kwargs)


def output(*args):
    return run(*args, capture_output=True, text=True).stdout.strip()


def prepared(*args, **options):
    return run('sh', '-eu', '-c', '. scripts/native-tools-env; exec "$@"',
               'release', *args, **options)


from release_metadata import version


def resolve(tag):
    if not re.fullmatch(r'v[0-9]+\.[0-9]+\.[0-9]+(?:-[A-Za-z0-9.-]+)?', tag):
        raise ValueError('select a version tag, such as v0.2.0')
    sha = output('git', 'rev-parse', 'HEAD')
    if tag != 'v' + version():
        raise ValueError('selected release version and Cargo workspace version must match')
    print(f'sha={sha}\nversion={version()}\ntag={tag}')
    print('matrix=' + json.dumps({'include': [
        {'target': target, 'runner': runner} for target, runner in TARGETS.items()
    ]}, separators=(',', ':')))


def validate(package, target, sha):
    info = json.loads((package / 'build.json').read_text())
    if (info['platform'], info['version'], info['revision']) != (target, version(), sha):
        raise ValueError(f'{package}: package target, version or revision differs')
    checksums(package)
    if not (package / 'INSTALL.md').is_file():
        raise ValueError(f'{package}: missing install guide')


def checksums(package):
    listed = set()
    for line in (package / 'SHA256SUMS').read_text().splitlines():
        digest, name = line.split('  ', 1)
        path = package / name
        if Path(name).is_absolute() or '..' in Path(name).parts or name in listed:
            raise ValueError(f'invalid checksum path: {name}')
        if path.is_symlink() or hashlib.sha256(path.read_bytes()).hexdigest() != digest:
            raise ValueError(f'checksum differs: {name}')
        listed.add(name)
    actual = {p.relative_to(package).as_posix() for p in package.rglob('*')
              if p.is_file() and p != package / 'SHA256SUMS'}
    if listed != actual:
        raise ValueError(f'{package}: incomplete checksum inventory')


def build(target, destination, sha):
    if output('git', 'rev-parse', 'HEAD') != sha:
        raise ValueError('checkout differs from the resolved release revision')
    destination.mkdir(parents=True, exist_ok=True)
    candidate = ['scripts/release-candidate']
    if target == 'windows-x86_64':
        candidate.append('--windows')
    prepared(*candidate, destination / 'candidate', env=dict(os.environ, CARGO_NET_OFFLINE='true'))
    package = destination / 'candidate' / version() / target
    pack(target, package, destination, sha)


def pack(target, package, destination, sha):
    validate(package, target, sha)
    destination.mkdir(parents=True, exist_ok=True)
    # The candidate entry runs all host installed checks before returning.
    # Windows additionally runs every SDK on its native runner before bundling.
    archive = destination / f'thinkthen-{version()}-{target}-packages.tar.gz'
    with tarfile.open(archive, 'w:gz', format=tarfile.GNU_FORMAT) as bundle:
        bundle.add(package, arcname=f'{version()}/{target}')
    print(archive)


def unpack(archive, scratch, target, sha):
    from release_retry import archive_revision
    sha = archive_revision(target, archive, sha)
    with tarfile.open(archive) as bundle:
        bundle.extractall(scratch, filter='data')
    package = scratch / version() / target
    validate(package, target, sha)
    return package


def check_archive(target, archive, sha):
    with tempfile.TemporaryDirectory(prefix='thinkthen-release-check-') as scratch:
        package = unpack(archive, Path(scratch), target, sha)
        if target == 'windows-x86_64':
            from release_windows_host import environment
            run('python3', 'scripts/release-check', package, env=environment())
        else:
            wheel = next((package / 'prerequisites/duckdb').glob('*.whl'))
            abi = wheel.name.split('-')[2][2:]
            interpreter = prepared('uv', 'python', 'find', '--offline', f'{abi[0]}.{abi[1:]}',
                                   capture_output=True, text=True).stdout.strip()
            python = prepared('uv', 'python', 'find', '--offline', '3.10',
                              capture_output=True, text=True).stdout.strip()
            env = dict(os.environ, THINKTHEN_CHECK_PYTHON=python, THINKTHEN_CHECK_DUCKDB_PYTHON=interpreter)
            prepared('python3', 'scripts/release-check', package, env=env)


def collect(source, destination, sha, targets):
    destination.mkdir(parents=True, exist_ok=False)
    platforms = {}
    for target in targets:
        name = f'thinkthen-{version()}-{target}-packages.tar.gz'
        matches = list(source.rglob(name))
        if len(matches) != 1:
            raise ValueError(f'expect one checked archive for {target}')
        with tempfile.TemporaryDirectory(prefix='thinkthen-release-bundle-') as scratch:
            package = unpack(matches[0], Path(scratch), target, sha)
            platforms[target] = json.loads((package / 'requirements.json').read_text())
        shutil.copy2(matches[0], destination / name)
    from release_retry import selected
    reuse = selected(sha)
    if reuse is not None:
        (destination / 'reuse.json').write_text(json.dumps(reuse, sort_keys=True, indent=2) + '\n')
    from release_install import release_note
    (destination / 'INSTALL.md').write_text(release_note(version(), platforms))
    for name in ('install.sh', 'install.ps1'):
        shutil.copy2(ROOT / name, destination / name)
    sums = [f'{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.name}\n'
            for p in sorted(destination.iterdir())]
    (destination / 'SHA256SUMS').write_text(''.join(sums))
    print(f'release: bundled {len(targets)} checked platform archives')


def draft(source, tag, sha, dry_run):
    if tag != 'v' + version():
        raise ValueError('draft version differs from the qualified release')
    files = sorted(source.iterdir())
    expected = {'INSTALL.md', 'SHA256SUMS', 'install.sh', 'install.ps1'} | {
        f'thinkthen-{version()}-{target}-packages.tar.gz' for target in TARGETS}
    from release_retry import selected
    reuse = selected(sha)
    if reuse is not None:
        if not dry_run or json.loads((source / 'reuse.json').read_text()) != reuse:
            raise ValueError('a reduced retry requires its provenance and draft dry-run')
        expected.add('reuse.json')
    registry = source / f'thinkthen-{version()}-registries.tar.gz'
    with tarfile.open(registry) as archive:
        info = json.load(archive.extractfile('source.json'))
        if info != {'revision': sha, 'version': version()}:
            raise ValueError('registry archive differs from the qualified release')
        expected.add(registry.name)
        expected.update(Path(p.name).name for p in archive.getmembers()
                        if p.isfile() and Path(p.name).parent == Path('github'))
    if {p.name for p in files} != expected:
        raise ValueError('draft requires every platform, checksums and install guide')
    checksums(source)
    args = ['gh', 'release', 'create', 'review-' + tag, '--draft', '--target', sha,
            '--title', f'ThinkThen {version()}', '--notes-file', str(source / 'INSTALL.md'),
            *map(str, files)]
    if dry_run:
        print(json.dumps(args))
    else:
        run(*args)


def main():
    if os.environ.get('GITHUB_ACTIONS') == 'true':
        os.environ.setdefault('PM_JOB', 'release-' + os.environ['GITHUB_RUN_ID'])
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    commands.add_parser('resolve').add_argument('tag')
    commands.add_parser('prepare-registry')
    prep = commands.add_parser('prepare')
    prep.add_argument('target', choices=TARGETS)
    for name, path in [('build', 'destination'), ('check-archive', 'archive')]:
        command = commands.add_parser(name)
        command.add_argument('target', choices=TARGETS)
        command.add_argument(path, type=Path)
        command.add_argument('sha')
    command = commands.add_parser('pack')
    command.add_argument('target', choices=TARGETS)
    command.add_argument('package', type=Path)
    command.add_argument('destination', type=Path)
    command.add_argument('sha')
    command = commands.add_parser('bundle')
    command.add_argument('source', type=Path)
    command.add_argument('destination', type=Path)
    command.add_argument('sha')
    command.add_argument('--targets', nargs='+', choices=TARGETS, default=list(TARGETS))
    command = commands.add_parser('draft')
    command.add_argument('source', type=Path)
    command.add_argument('tag')
    command.add_argument('sha')
    command.add_argument('--dry-run', action='store_true')
    command = commands.add_parser('registry')
    command.add_argument('source', type=Path)
    command.add_argument('destination', type=Path)
    command.add_argument('sha')
    command = commands.add_parser('attach')
    command.add_argument('registry', type=Path)
    command.add_argument('release', type=Path)
    command.add_argument('sha')
    args = parser.parse_args()
    if args.command in ('prepare', 'prepare-registry', 'build'):
        checked_home()
    if args.command == 'resolve': resolve(args.tag)
    elif args.command == 'prepare-registry':
        from release_registry_prepare import prepare
        prepare()
    elif args.command == 'prepare':
        from release_workflow_prepare import prepare
        prepare(args.target)
    elif args.command == 'build': build(args.target, args.destination.resolve(), args.sha)
    elif args.command == 'check-archive': check_archive(args.target, args.archive.resolve(), args.sha)
    elif args.command == 'pack': pack(args.target, args.package.resolve(), args.destination.resolve(), args.sha)
    elif args.command == 'bundle': collect(args.source.resolve(), args.destination.resolve(), args.sha, args.targets)
    elif args.command == 'registry':
        from release_registry import stage
        stage(args.source.resolve(), args.destination.resolve(), args.sha)
    elif args.command == 'attach':
        from release_registry import attach
        attach(args.registry.resolve(), args.release.resolve(), args.sha)
    else: draft(args.source.resolve(), args.tag, args.sha, args.dry_run)
