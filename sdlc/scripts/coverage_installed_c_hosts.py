"""Run installed routine C door hosts with retained native coverage objects."""
import os
from pathlib import Path
import shutil
import tarfile

COLLECTIONS = tuple('installed-' + name for name in ('cpp', 'go', 'zig', 'ada', 'php', 'cobol'))


def archive(source, output, members):
    with tarfile.open(output, 'w:gz') as package:
        for name in members:
            package.add(source / name, arcname='./' + name)


def prepare(context):
    root, run = context['root'], context['run']
    command, missing = context['command'], context['missing_tool']
    env = dict(context['env'])
    overrides = ('THINKTHEN_CMAKE_BIN', 'THINKTHEN_GO_BIN', 'THINKTHEN_ZIG',
                 'THINKTHEN_PHP_BIN', 'THINKTHEN_COMPOSER_BIN')
    env.update({key: os.environ[key] for key in overrides if key in os.environ})
    tools = {'cmake': 'THINKTHEN_CMAKE_BIN', 'go': 'THINKTHEN_GO_BIN',
             'zig': 'THINKTHEN_ZIG', 'php8.3': 'THINKTHEN_PHP_BIN',
             'composer': 'THINKTHEN_COMPOSER_BIN'}
    for tool in ('c++', 'cc', 'ar', 'ld', 'nm', 'objcopy', 'gprbuild', 'gnatmake', 'cobc',
                 'clang', 'node', 'flock', 'python3', 'gofmt', *tools):
        selected = env.get(tools.get(tool, ''), tool)
        phar = tool == 'composer' and Path(selected).suffix == '.phar' and Path(selected).is_file()
        if not phar and not shutil.which(selected, path=env.get('PATH')):
            raise missing(f'installed C door hosts need {tool}: {selected}')
    for tool in ('cmake', 'go', 'zig', 'php8.3'):
        if tools[tool] in env:
            env['PATH'] = str(Path(env[tools[tool]]).resolve().parent) + os.pathsep + env['PATH']
    # Host builds must use the profile runtime in the supplied native assets;
    # only the driver's Rust native build receives Rust instrumentation flags.
    env.pop('CARGO_ENCODED_RUSTFLAGS', None)
    env.pop('RUSTFLAGS', None)
    env.update(CARGO_TARGET_DIR=str(run / 'target-tools'), THINKTHEN_TEST_PROFILE='routine',
               THINKTHEN_FULL_COMPILER='1', GOCACHE=str(run / 'go-cache'),
               THINKTHEN_BACKEND_BIN=str(run / 'target-tools/debug/conformance-backend'))
    stage = run / 'stage-c-hosts'
    native = stage / 'native'
    (native / 'include').mkdir(parents=True)
    (native / 'lib').mkdir()
    shutil.copy2(root / 'libraries/c/include/thinkthen.h', native / 'include/thinkthen.h')
    shutil.copy2(context['c_shared'], native / 'lib/libthinkthen.so')
    command(['sh', str(root / 'libraries/c/localize.sh'),
             str(context['c_target'] / 'debug/libthinkthen_c.a'), str(native / 'lib/libthinkthen.a')],
            root, {**env, 'THINKTHEN_COVERAGE_INITIALIZER': str(context['initializer'])},
            stage / 'localize.log')
    native_archive = stage / 'c.tar.gz'
    archive(native, native_archive, ('include', 'lib'))
    cpp_archive = stage / 'cpp.tar.gz'
    archive(root / 'libraries/cpp', cpp_archive, ('include', 'cmake', 'CMakeLists.txt', 'LICENSE', 'README.md'))
    go = stage / 'go'
    command(['python3', str(root / 'libraries/go/fixtures/package_owned.py'),
             str(root / 'libraries/go'), str(native), str(go)], root, env, stage / 'package-go.log')
    go_archive = stage / 'go.tar.gz'
    archive(go, go_archive, tuple(path.name for path in go.iterdir()))
    zig = stage / 'zig'
    zig.mkdir()
    for name in ('src', 'examples', 'build.zig', 'build.zig.zon', 'LICENSE', 'README.md'):
        source = root / 'libraries/zig' / name
        if source.is_dir():
            shutil.copytree(source, zig / name)
        else:
            shutil.copy2(source, zig / name)
    shutil.copytree(native, zig / 'native/x86_64-unknown-linux-gnu')
    zig_archive = stage / 'zig.tar.gz'
    archive(zig, zig_archive, tuple(path.name for path in zig.iterdir()))
    ada = stage / 'ada'
    ada.mkdir()
    for name in ('src', 'examples', 'thinkthen.gpr', 'LICENSE', 'README.md'):
        source = root / 'libraries/ada' / name
        if source.is_dir():
            shutil.copytree(source, ada / name)
        else:
            shutil.copy2(source, ada / name)
    shutil.copytree(native, ada / 'native')
    ada_archive = stage / 'ada.tar.gz'
    archive(ada, ada_archive, tuple(path.name for path in ada.iterdir()))
    settings = {
        'cpp': {'THINKTHEN_ARTIFACT': str(cpp_archive), 'THINKTHEN_C_ARTIFACT': str(native_archive)},
        'go': {'THINKTHEN_ARTIFACT': str(go_archive), 'THINKTHEN_C_ARTIFACT': str(native_archive)},
        'zig': {'THINKTHEN_ARTIFACT': str(zig_archive)},
        'ada': {'THINKTHEN_ARTIFACT': str(ada_archive),
                'THINKTHEN_C_LIBRARY': str(native / 'lib/libthinkthen.so'),
                'THINKTHEN_C_STATIC_LIBRARY': str(native / 'lib/libthinkthen.a')},
        'php': {'THINKTHEN_COMPLETE_LIBRARY': str(native / 'lib/libthinkthen.so')},
        'cobol': {'THINKTHEN_C_LIBRARY': str(native / 'lib/libthinkthen.so'),
                  'THINKTHEN_C_HEADER': str(native / 'include/thinkthen.h')},
    }
    consumers = {
        'cpp': ('owned-shared/owned_consumer', 'owned-static/owned_consumer',
                'owned-shared/session_consumer', 'install/lib/libthinkthen.so.0'),
        'go': ('plan-consumer', 'owned-consumer', 'feed-consumer', 'usage-consumer', 'shared-consumer'),
        'zig': ('session', 'example', 'consumer'),
        'ada': ('native_session', 'ada-consumer'),
    }
    collections = []
    for name in COLLECTIONS:
        language = name.removeprefix('installed-')
        retained = run / name / 'installed'
        objects = [retained / path for path in consumers.get(language, ())]
        if language in ('php', 'cobol'):
            objects = [native / 'lib/libthinkthen.so']
        def runner(folder, language=language, retained=retained):
            command(['sh', './check.sh'], root / 'libraries' / language,
                    {**env, **settings[language], 'THINKTHEN_COVERAGE_STAGE': str(retained)},
                    folder / 'check.log', timeout=1800)
        collections.append({'name': name, 'objects': objects, 'runner': runner})
    return collections
