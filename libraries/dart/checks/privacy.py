"""Reject private workspace paths and planted key markers in shipped sources."""
from pathlib import Path
import re
import tempfile
import os
import subprocess
import sys
import tarfile
from native_assets import development, tracked_files

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'conformance/children'))
from native_guard import distribution_bytes

root = Path(__file__).resolve().parents[1]
blocked = re.compile(r'(?:/home/|/Users/)|PRIVATE_PATH_MARKER|PRIVATE_KEY_MARKER|sk-private-[A-Za-z0-9]+')


def check(paths):
    return [str(path) for path in paths if blocked.search(path.read_text(errors='replace'))]


def sources(folder):
    return [path for path in tracked_files(folder) if path.is_file()
            and path != Path(__file__) and path.suffix in {'.dart', '.py', '.sh', '.md', '.yaml', '.json'}]


def archives(paths):
    return [str(path) for path in paths if any(blocked.search(data.decode(errors='replace'))
                                             for data in distribution_bytes(path))]


selected = sources(root)
found = check(selected) + archives({Path(os.environ[name]) for name in
    ('THINKTHEN_ARTIFACT', 'THINKTHEN_DART_ARTIFACT', 'THINKTHEN_C_ARTIFACT') if os.environ.get(name)})
if found:
    raise SystemExit(f'PRIVATE_SOURCE_REFUSED {found}')
with tempfile.TemporaryDirectory() as directory:
    planted = Path(directory) / 'planted.txt'
    for marker in ('/home/private/file', '/Users/private/file', 'PRIVATE_KEY_MARKER', 'sk-private-SECRET123'):
        planted.write_text(marker)
        if not check([planted]):
            raise SystemExit(f'PRIVATE_PLANT_NOT_REFUSED {marker}')
    fixture = Path(directory) / 'checkout/libraries/dart'
    fixture.mkdir(parents=True)
    subprocess.run(['git', 'init', '-q', str(fixture.parents[1])], check=True)
    for relative in ('pubspec.yaml', 'flutter/pubspec.yaml', 'flutter/example/pubspec.yaml',
                     'checks/session_consumer/pubspec.yaml', 'lib/target/pubspec.yaml'):
        authored = fixture / relative
        authored.parent.mkdir(parents=True, exist_ok=True)
        authored.write_text('name: fixture\n')
        subprocess.run(['git', '-C', str(fixture), 'add', relative], check=True)
        authored.write_text('PRIVATE_KEY_MARKER')
        if str(authored) not in check(sources(fixture)):
            raise SystemExit('PRIVATE_TRACKED_SOURCE_PLANT_NOT_REFUSED')
        authored.write_text('name: fixture\n')
    definition = fixture / 'native-assets.json'
    definition.write_bytes((root / 'native-assets.json').read_bytes())
    subprocess.run(['git', '-C', str(fixture), 'add', 'native-assets.json'], check=True)
    for output in ('target/core-wave', 'target/core-wave-flutter', 'flutter/build', '.dart_tool', 'cache', 'untracked'):
        generated = fixture / output / 'pubspec.yaml'
        generated.parent.mkdir(parents=True, exist_ok=True)
        generated.write_text('PRIVATE_KEY_MARKER')
    if check(sources(fixture)):
        raise SystemExit('PRIVATE_WORKING_OUTPUT_CLASSIFIED_AS_SOURCE')
    # Archive membership remains authoritative even beneath an unrelated checkout.
    shipped = fixture / 'target/shipped.tar.gz'
    with tarfile.open(shipped, 'w:gz') as archive:
        archive.add(generated, arcname='pubspec.yaml')
    if not archives([shipped]):
        raise SystemExit('PRIVATE_ARCHIVE_PLANT_NOT_REFUSED')
    native = Path(directory) / 'libthinkthen.so'
    native.write_bytes(b'development fixture')
    copied = development(fixture, native, Path(directory) / 'owned')
    for item in tracked_files(fixture):
        if not (copied / item.relative_to(fixture)).is_file():
            raise SystemExit('DEVELOPMENT_TRACKED_INPUT_MISSING')
    if any((copied / output).exists() for output in ('target', 'cache', 'untracked', 'flutter/build', '.dart_tool')):
        raise SystemExit('DEVELOPMENT_COPIED_WORKING_OUTPUT')
print(f'PRIVATE_SOURCE_PASS {len(selected)} files, 4 planted negatives; tracked/archive scope passed')
