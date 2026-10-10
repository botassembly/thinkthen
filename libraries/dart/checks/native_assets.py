"""Prepare owned development projects for the same build hook used by pub callers."""
import hashlib
import json
from pathlib import Path
import shutil
import sys


def package_members(source):
    source = Path(source)
    members = {name for name in ('pubspec.yaml', 'README.md', 'LICENSE', 'CHANGELOG.md')
               if (source / name).is_file()}
    if (source / 'native-assets.json').is_file():
        members.add('native-assets.json')
    for folder in ('lib', 'hook', 'linux'):
        members.update(str(path.relative_to(source)) for path in (source / folder).rglob('*') if path.is_file())
    return members


def configure(package, native, projects):
    package, native = Path(package).resolve(), Path(native).resolve()
    definition = package / 'native-assets.json'
    manifest = json.loads(definition.read_text())
    asset = manifest['assets']['linux_x64']
    payload = native.read_bytes()
    digest = hashlib.sha256(payload).hexdigest()
    if manifest['distribution'] == 'development-only':
        # Only an owned package copy may describe the selected development fixture.
        asset.update(sha256=digest, url=None)
        definition.write_text(json.dumps(manifest, indent=2) + '\n')
    else:
        assert digest == asset['sha256'], 'NATIVE_FIXTURE_IDENTITY'
    cache = package / 'checks/scratch/native-assets'
    cached = cache / asset['sha256'] / asset['file']
    cached.parent.mkdir(parents=True, exist_ok=True)
    cached.write_bytes(payload)
    for project in projects:
        project = Path(project).resolve()
        spec = project / 'pubspec.yaml'
        text = spec.read_text()
        text = text.split('\nhooks:', 1)[0]
        spec.write_text(text + '\nhooks:\n  user_defines:\n    thinkthen_dart:\n'
                        '      offline: true\n      asset_cache: ' + json.dumps(str(cache) + '/') + '\n')
        if project != package:
            (project / 'pubspec_overrides.yaml').write_text(
                'dependency_overrides:\n  thinkthen_dart:\n    path: ' + json.dumps(str(package)) + '\n')
    return cache


def development(source, native, destination):
    """Copy the retained gates without copying build output or changing repository files."""
    source, destination = Path(source).resolve(), Path(destination).resolve()
    package = destination / 'libraries/dart'
    shutil.copytree(source, package, ignore=shutil.ignore_patterns(
        '.dart_tool', 'build', 'scratch', 'logs', '__pycache__', 'pubspec_overrides.yaml'))
    # Keep existing fixture paths valid in the owned tree; all non-Dart inputs remain read-only.
    repo = source.parents[1]
    for item in repo.iterdir():
        if item.name not in {'libraries', '.git'}:
            (destination / item.name).symlink_to(item, target_is_directory=item.is_dir())
    for item in (repo / 'libraries').iterdir():
        if item.name != 'dart':
            (destination / 'libraries' / item.name).symlink_to(item, target_is_directory=item.is_dir())
    projects = [package, package / 'checks/consumers/alpha', package / 'checks/consumers/bravo',
                package / 'flutter', package / 'flutter/example']
    configure(package, native, projects)
    for folder in ('checks/logs', 'flutter/logs', 'flutter/scratch'):
        (package / folder).mkdir(parents=True, exist_ok=True)
    scratch = package / 'checks/scratch'
    shutil.copyfile(native, scratch / 'libthinkthen.so')
    return package


if __name__ == '__main__':
    if sys.argv[1] == 'development':
        print(development(*sys.argv[2:]))
    else:
        configure(sys.argv[2], sys.argv[3], sys.argv[4:])
