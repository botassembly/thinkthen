"""Collect installed routine managed consumers with their packaged native objects."""
import os
from pathlib import Path
import shutil
import sys
import tarfile
import zipfile

COLLECTIONS = tuple('installed-' + name for name in
                    ('csharp', 'dart', 'flutter', 'swift', 'objective-c'))


def archive(source, output, members):
    with tarfile.open(output, 'w:gz') as package:
        for name in sorted(members):
            package.add(source / name, arcname='./' + name)


def require_tools(language, env, missing):
    tools = {'csharp': ('/usr/bin/bwrap',), 'dart': ('node',),
             'flutter': ('node', '/usr/bin/xvfb-run'),
             'swift': ('node', 'bwrap', 'flock')}[language]
    overrides = {'csharp': (('dotnet', 'THINKTHEN_DOTNET'),),
                 'dart': (('dart', 'TT_DART'),),
                 'flutter': (('dart', 'TT_DART'), ('flutter', 'TT_FLUTTER')),
                 'swift': (('swift', 'THINKTHEN_SWIFT'),)}[language]
    for tool, key in overrides:
        selected = env.get(key) or shutil.which(tool, path=env.get('PATH'))
        if not selected and tool in ('dart', 'flutter'):
            selected = str(Path(env['HOME']) / '.local/opt/flutter/bin' / tool)
        tools += (selected or tool,)
    for tool in tools:
        if not shutil.which(tool, path=env.get('PATH')):
            raise missing(f'installed {language} needs {tool}')


def prepare(context):
    root, run = context['root'], context['run']
    command, missing = context['command'], context['missing_tool']
    env = dict(context['env'])
    for name in ('THINKTHEN_DOTNET', 'THINKTHEN_SWIFT', 'TT_DART', 'TT_FLUTTER', 'PUB_CACHE'):
        if name in os.environ:
            env[name] = os.environ[name]
    env.pop('CARGO_ENCODED_RUSTFLAGS', None)
    env.pop('RUSTFLAGS', None)
    env.update(CARGO_TARGET_DIR=str(run / 'target-tools'), THINKTHEN_FULL_COMPILER='1',
               THINKTHEN_TEST_PROFILE='routine',
               THINKTHEN_BACKEND_BIN=str(run / 'target-tools/debug/conformance-backend'))
    stage = run / 'stage-managed'
    stage.mkdir()
    native = stage / 'native'
    (native / 'include').mkdir(parents=True)
    (native / 'lib').mkdir()
    shutil.copy2(root / 'libraries/c/include/thinkthen.h', native / 'include/thinkthen.h')
    shutil.copy2(context['c_shared'], native / 'lib/libthinkthen.so')
    (native / 'lib/libthinkthen.so.0').symlink_to('libthinkthen.so')
    c_archive = stage / 'c.tar.gz'
    archive(native, c_archive, ('include', 'lib'))
    collections = []
    for name in COLLECTIONS:
        language = name.removeprefix('installed-')
        retained = run / name / 'installed'
        objects = [retained / 'libthinkthen.so']
        def runner(folder, *, language=language, retained=retained):
            if language == 'objective-c':
                # This door requires an assembled Apple XCFramework and macOS execution.
                raise missing('installed Objective-C Foundation needs macOS and a matching instrumented Apple package')
            if sys.platform != 'linux':
                raise missing(f'installed {language} coverage needs the matching Linux native package')
            require_tools(language, env, missing)
            settings = {**env, 'THINKTHEN_COVERAGE_STAGE': str(retained)}
            if language == 'csharp':
                tree = stage / 'csharp-tree'
                (tree / 'libraries').mkdir(parents=True)
                for source in root.iterdir():
                    if source.name not in ('libraries', '.git', 'target'):
                        (tree / source.name).symlink_to(source, target_is_directory=source.is_dir())
                for source in (root / 'libraries').iterdir():
                    if source.name != 'csharp':
                        (tree / 'libraries' / source.name).symlink_to(source, target_is_directory=source.is_dir())
                package = tree / 'libraries/csharp'
                shutil.copytree(root / 'libraries/csharp', package,
                                ignore=shutil.ignore_patterns('target', 'bin', 'obj', '__pycache__'))
                settings['THINKTHEN_NATIVE_ASSET'] = str(context['c_shared'])
                command(['sh', 'check.sh'], package, settings, folder / 'check.log', timeout=1800)
                retained.mkdir()
                packed = list((package / 'target/scratch/managed').glob('*.nupkg'))
                if len(packed) != 1:
                    raise RuntimeError('installed C# expected one NuGet package')
                with zipfile.ZipFile(packed[0]) as bundle:
                    (retained / 'libthinkthen.so').write_bytes(bundle.read('runtimes/linux-x64/native/libthinkthen.so'))
            elif language in ('dart', 'flutter'):
                sys.path.insert(0, str(root / 'libraries/dart/checks'))
                from native_assets import configure, package_members
                dart = stage / ('dart-package-' + language)
                for member in package_members(root / 'libraries/dart'):
                    target = dart / member
                    target.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copy2(root / 'libraries/dart' / member, target)
                configure(dart, context['c_shared'], [])
                dart_archive = stage / ('thinkthen-dart-' + language + '.tar.gz')
                archive(dart, dart_archive, package_members(dart))
                settings.update(THINKTHEN_ARTIFACT=str(dart_archive),
                                THINKTHEN_C_ARTIFACT=str(c_archive),
                                THINKTHEN_INSTALLED_CONSUMER=language)
                if language == 'flutter':
                    flutter_archive = stage / 'thinkthen-flutter-coverage.tar.gz'
                    with tarfile.open(flutter_archive, 'w:gz') as packed:
                        source = root / 'libraries/dart/flutter'
                        for member in sorted(package_members(source)):
                            packed.add(source / member, arcname='flutter/' + member)
                    settings.update(THINKTHEN_ARTIFACT=str(flutter_archive),
                                    THINKTHEN_DART_ARTIFACT=str(dart_archive))
                command(['sh', 'check.sh'], root / 'libraries/dart', settings,
                        folder / 'check.log', timeout=1800)
                bundle = 'dart-build/bundle' if language == 'dart' else 'flutter-bundle'
                shutil.copy2(retained / 'consumer' / bundle / 'lib/libthinkthen.so', retained / 'libthinkthen.so')
            else:
                swift = env.get('THINKTHEN_SWIFT', shutil.which('swift', path=env.get('PATH')))
                if not swift:
                    raise missing('installed Swift needs swift')
                tools = stage / 'swift-tools'
                tools.mkdir()
                (tools / 'swift').symlink_to(Path(swift).resolve())
                settings['PATH'] = str(tools) + os.pathsep + env['PATH']
                package = stage / 'swift-package'
                command(['python3', str(root / 'libraries/swift/Tests/fixtures/package_local.py'),
                         '--native', str(context['c_shared']), '--out', str(package)],
                        root, settings, folder / 'package.log')
                settings.update(THINKTHEN_ARTIFACT=str(stage / 'thinkthen-swift-0.2.0.tar.gz'),
                                THINKTHEN_C_ARTIFACT=str(c_archive))
                command(['sh', 'check.sh'], root / 'libraries/swift', settings,
                        folder / 'check.log', timeout=1800)
                assets = list((retained / 'installed/ThinkThen_ThinkThen.bundle/Native').glob('*/libthinkthen.so'))
                if len(assets) != 1:
                    raise RuntimeError('installed Swift expected one bundled native asset')
                shutil.copy2(assets[0], retained / 'libthinkthen.so')
            if (retained / 'libthinkthen.so').read_bytes() != context['c_shared'].read_bytes():
                raise RuntimeError(f'installed {language} loaded a different native asset')
        collections.append({'name': name, 'objects': objects, 'runner': runner})
    return collections
