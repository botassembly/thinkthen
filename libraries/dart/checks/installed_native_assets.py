"""Install development packages and exercise SDK-owned Linux native assets offline."""
import base64
import os
import hashlib
import http.server
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import threading
import tempfile

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
from usage import run_usage
from native_assets import configure, package_members


def run(command, cwd, env, expected=None, timeout=180):
    result = subprocess.run(command, cwd=cwd, env=env, text=True, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, timeout=timeout)
    if expected:
        assert result.returncode != 0 and expected in result.stdout, result.stdout
    else:
        assert result.returncode == 0, result.stdout
    output = result.stdout if timeout is None else result.stdout[-2000:]
    print(output, end='' if output.endswith('\n') else '\n')
    return result


def run_prepared():
    prepared = Path(os.environ['THINKTHEN_PARITY_PREPARED']).resolve(strict=True) / 'consumer'
    member = os.environ['THINKTHEN_PARITY_CONSUMER']
    assert member in ('dart', 'flutter'), 'unknown prepared Dart consumer'
    settings = json.loads((prepared / 'parity.json').read_text())
    assert member in settings['members'], 'consumer was not prepared'
    env = dict(os.environ, TT_DART=settings['dart'], TT_FLUTTER=settings['flutter'],
               THINKTHEN_COMPLETE_LIBRARY=settings['native'])
    # Flutter writes compiler and test output; each member owns its project copy.
    with tempfile.TemporaryDirectory(prefix='thinkthen-dart-member-') as owned:
        scratch = Path(owned)
        shutil.copytree(prepared / 'packages', scratch / 'packages')
        project = settings['consumer'] if member == 'dart' else 'flutter-app'
        shutil.copytree(prepared / project, scratch / project,
                        ignore=shutil.ignore_patterns('build', 'flutter_build'))
        for config in (scratch / project / '.dart_tool').rglob('*.json'):
            config.write_text(config.read_text().replace(str(prepared), str(scratch)))
        env.update(THINKTHEN_PARITY_PACKAGE=str(scratch / 'packages/dart'),
                   THINKTHEN_DART_CONSUMER=str(scratch / project),
                   THINKTHEN_DART_BINARY=str(prepared / 'parity-build/bundle/bin/parity_consumer'),
                   THINKTHEN_FLUTTER_CONSUMER=str(scratch / project),
                   THINKTHEN_FLUTTER_BINARY=str(prepared / 'flutter-parity-bundle/thinkthen_flutter_example'))
        command = [sys.executable, str(ROOT / 'conformance/children/complete_parity.py'), member]
        if member == 'flutter':
            command = ['/usr/bin/xvfb-run', '-a', *command]
        run(command, scratch, env, timeout=None)


def save_prepared(scratch, dart, flutter, native, consumer, has_flutter):
    (scratch / 'parity.json').write_text(json.dumps({
        'dart': str(dart), 'flutter': str(flutter), 'native': str(native),
        'consumer': consumer.name, 'members': ['dart', 'flutter'] if has_flutter else ['dart']}))


def main():
    if sys.argv[1:] == ['--prepared']:
        run_prepared()
        return
    dart, flutter, native, scratch, pub_cache = (Path(value).resolve() for value in sys.argv[1:])
    scratch.mkdir(parents=True, exist_ok=False)
    env = child_env(keep=('LLVM_PROFILE_FILE',), home=scratch / 'home', PATH='/usr/bin:/bin', LANG='C.UTF-8',
                    PUB_CACHE=str(pub_cache), FLUTTER_SUPPRESS_ANALYTICS='true',
                    CI='true')
    packages = scratch / 'packages'
    packages.mkdir()
    full = os.environ.get('THINKTHEN_TEST_PROFILE') == 'full'
    preparing = bool(os.environ.get('THINKTHEN_PARITY_PREPARE'))
    assert not preparing or full, 'preparation requires full profile'
    supplied = os.environ.get('THINKTHEN_ARTIFACT')
    archives = []
    if supplied:
        supplied = Path(supplied).resolve()
        flutter_archive = supplied if supplied.name.startswith('thinkthen-flutter-') else None
        dart_archive = Path(os.environ['THINKTHEN_DART_ARTIFACT']).resolve() if flutter_archive else supplied
        assert dart_archive.name.startswith('thinkthen-dart-'), 'expected Dart package archive'
        native_archive = native
        with tarfile.open(native_archive) as content:
            content.extractall(scratch / 'native', filter='data')
        native = scratch / 'native/lib/libthinkthen.so'
        run([sys.executable, str(ROOT / 'sdlc/scripts/check-c-exports.py'),
             str(scratch / 'native/include/thinkthen.h'), str(native)], scratch, env)
        sources = [('dart', dart_archive)] + ([('flutter', flutter_archive)] if flutter_archive else [])
    else:
        sources = [('dart', ROOT / 'libraries/dart'), ('flutter', ROOT / 'libraries/dart/flutter')]
    for name, source in sources:
        if supplied:
            archive = source
        else:
            staged = scratch / 'source' / name
            for member in package_members(source):
                target = staged / member
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(source / member, target)
            if name == 'dart':
                configure(staged, native, [])
            archive = scratch / f'thinkthen-{name}.tar.gz'
            with tarfile.open(archive, 'w:gz') as output:
                for member in sorted(package_members(staged)):
                    output.add(staged / member, arcname=member)
        # The shipping Flutter archive already owns its flutter/ package root.
        target = packages if supplied and name == 'flutter' else packages / name
        target.mkdir(exist_ok=target == packages)
        with tarfile.open(archive) as content:
            content.extractall(target, filter='data')
        archives.append(archive)
        print(name, hashlib.sha256(archive.read_bytes()).hexdigest())
    if supplied:
        declared = json.loads((packages / 'dart/native-assets.json').read_text())['assets']['linux_x64']
        assert declared['sha256'] == hashlib.sha256(native.read_bytes()).hexdigest(), 'supplied Dart/native asset checksum mismatch'
    cache = configure(packages / 'dart', native, [])
    definition = packages / 'dart/native-assets.json'
    manifest = json.loads(definition.read_text())
    asset = manifest['assets']['linux_x64']
    payload = native.read_bytes()
    cached = cache / asset['sha256'] / asset['file']
    consumer = scratch / 'consumer'
    (consumer / 'bin').mkdir(parents=True)
    shutil.copytree(ROOT / 'libraries/dart/checks/session_consumer/bin', consumer / 'bin', dirs_exist_ok=True)
    data_case = consumer / 'bin/data_cases.dart'
    data_case.write_text(data_case.read_text().replace('__CANONICAL_IMAGE__', base64.b64encode((ROOT / 'specification/fixtures/images/red.png').read_bytes()).decode()))
    spec = ("name: installed_native_assets\npublish_to: none\nenvironment:\n  sdk: '>=3.10.0 <4.0.0'\ndependencies:\n  thinkthen_dart:\n    path: ../packages/dart\nhooks:\n  user_defines:\n    thinkthen_dart:\n      offline: true\n      asset_cache: ../packages/dart/checks/scratch/native-assets/\n")
    (consumer / 'pubspec.yaml').write_text(spec)
    run([str(dart), 'pub', 'get', '--offline'], consumer, env)
    if os.environ.get('THINKTHEN_TEST_PROFILE') == 'smoke':
        env.update({name: os.environ[name] for name in ('THINKTHEN_BASE_URL', 'THINKTHEN_API_KEY',
            'THINKTHEN_CACHE', 'THINKTHEN_TEST_SMOKE_QUESTION', 'THINKTHEN_TEST_SMOKE_TEXT')})
        run([sys.executable, str(ROOT / 'conformance/children/native_guard.py'), str(native), *map(str, archives)], scratch, env)
        run([str(dart), 'run', 'bin/main.dart', 'smoke'], consumer, env)
        return
    member = os.environ.get('THINKTHEN_INSTALLED_CONSUMER', 'both')
    assert member in ('dart', 'flutter', 'both'), 'unknown routine consumer'
    assert member == 'both' or not full, 'split consumer requires routine profile'
    if member != 'flutter':
        run([str(dart), 'run', 'bin/parser_cases.dart', str(ROOT / 'specification/fixtures/types/corpus.json')], consumer, env)
    has_flutter = (packages / 'flutter').is_dir()
    if has_flutter:
        projects = [packages / 'flutter']
        if (packages / 'flutter/example/pubspec.yaml').is_file():
            projects.append(packages / 'flutter/example')
        configure(packages / 'dart', native, projects)
        run([str(flutter), 'pub', 'get', '--offline'], packages / 'flutter', env)
    if member != 'flutter':
        cached.rename(cached.with_suffix('.held'))
        run([str(dart), 'run', 'bin/main.dart'], consumer, env, 'cache miss in offline build')
        cached.write_bytes(b'corrupted native library')
        run([str(dart), 'run', 'bin/main.dart'], consumer, env, 'checksum mismatch')
        cached.with_suffix('.held').replace(cached)
        run([str(dart), 'run', 'bin/main.dart'], consumer, env)
        run([str(dart), 'run', 'bin/main.dart', 'stream-regressions'], consumer, env)
        # The same installed consumer owns tiny usage cases; it runs one local call.
        for mode in ('written', 'failed', 'disabled'):
            usage_env = dict(env, XDG_STATE_HOME=str(scratch / ('usage-' + mode)))
            if mode == 'disabled':
                usage_env.pop('HOME', None)
                usage_env.pop('XDG_STATE_HOME', None)
            run_usage([str(dart), 'run', 'bin/main.dart', 'usage-' + mode],
                      state=scratch / ('usage-' + mode) / 'thinkthen', mode=mode, env=usage_env,
                      execute=lambda command, *, env: run(command, consumer, env))
        if supplied:
            run([str(dart), 'build', 'cli', '-t', 'bin/main.dart', '-o', str(scratch / 'dart-build')], consumer, env)
        else:
            # A separate installed project obtains the same pinned bytes during its build.
            download_consumer = scratch / 'download-consumer'
            shutil.copytree(consumer / 'bin', download_consumer / 'bin')
            consumer = download_consumer
            download_spec = spec.replace('offline: true', 'offline: false').replace('      asset_cache: ../packages/dart/checks/scratch/native-assets/\n', '')
            (consumer / 'pubspec.yaml').write_text(download_spec)
            run([str(dart), 'pub', 'get', '--offline'], consumer, env)
            run([str(dart), 'run', 'bin/main.dart'], consumer, env, 'no approved download URL')
            requests = []
            class Handler(http.server.BaseHTTPRequestHandler):
                def do_GET(self):
                    requests.append(self.path)
                    self.send_response(200)
                    self.send_header('Content-Length', str(len(payload)))
                    self.send_header("Connection", "close")
                    self.end_headers()
                    self.wfile.write(payload)
                def log_message(self, *_):
                    pass
            server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), Handler)
            thread = threading.Thread(target=server.serve_forever)
            thread.start()
            try:
                asset['url'] = f'http://127.0.0.1:{server.server_port}/native-library'
                definition.write_text(json.dumps(manifest))
                # Invalidate prior hook output through the changed registered definition.
                run([str(dart), 'build', 'cli', '-t', 'bin/main.dart', '-o', str(scratch / 'dart-build')], consumer, env)
                assert requests == ['/native-library'], requests
            finally:
                server.shutdown()
                thread.join()
                server.server_close()
            definition.write_text(json.dumps({**manifest, 'assets': {'linux_x64': {**asset, 'url': None}}}))
    if full:
        run([str(dart), 'build', 'cli', '-t', 'bin/parity_consumer.dart', '-o', str(scratch / 'parity-build')], consumer, env)
        parity_env = dict(env, THINKTHEN_PARITY_PACKAGE=str(packages / 'dart'),
            THINKTHEN_DART_CONSUMER=str(consumer),
            THINKTHEN_DART_BINARY=str(scratch / 'parity-build/bundle/bin/parity_consumer'),
            THINKTHEN_COMPLETE_LIBRARY=str(native), TT_DART=str(dart), TT_FLUTTER=str(flutter))
        if not preparing:
            run([sys.executable, str(ROOT / 'conformance/children/complete_parity.py'), 'dart'], scratch, parity_env, timeout=None)
    if member != 'flutter':
        # Run the compiled bundle after the source packages and build cache move away.
        packages.rename(scratch / 'packages-held')
        consumer.rename(scratch / 'consumer-held')
        run([str(scratch / 'dart-build/bundle/bin/main')], scratch / 'dart-build/bundle', env)
        (scratch / 'packages-held').rename(packages)
        (scratch / 'consumer-held').rename(consumer)
    if not has_flutter:
        run([sys.executable, str(ROOT / 'conformance/children/native_guard.py'), str(native), *map(str, archives)], scratch, env)
        if preparing:
            save_prepared(scratch, dart, flutter, native, consumer, has_flutter)
        print('PASS: supplied Dart and native archives, standalone typed bundle; no source build')
        return
    app = scratch / 'flutter-app'
    shutil.copytree(ROOT / 'libraries/dart/flutter/example/linux', app / 'linux')
    (app / 'lib').mkdir()
    shutil.copyfile(consumer / 'bin/stream_cases.dart', app / 'lib/stream_cases.dart')
    shutil.copyfile(consumer / 'bin/data_cases.dart', app / 'lib/data_cases.dart')
    for streams in [app / 'lib/stream_cases.dart', app / 'lib/data_cases.dart']:
        streams.write_text(streams.read_text().replace('package:thinkthen_dart/thinkthen_dart.dart', 'package:thinkthen_flutter/thinkthen_flutter.dart'))
    source = (consumer / 'bin/main.dart').read_text().replace('package:thinkthen_dart/thinkthen_dart.dart', 'package:thinkthen_flutter/thinkthen_flutter.dart').replace('(dart)', '(flutter)')
    source = source.replace('Future<void> main(List<String> args) async {', 'Future<void> exercise([List<String> args = const []]) async {')
    source += '\nFuture<void> main() async {\n  try { await exercise(); await exercise(["stream-regressions"]); exit(0); } catch (error, stack) { stderr.writeln("$error\\n$stack"); exit(1); }\n}\n'
    (app / 'lib/main.dart').write_text(source)
    if full:
        parity_source = (consumer / 'bin/parity_consumer.dart').read_text().replace('package:thinkthen_dart/thinkthen_dart.dart', 'package:thinkthen_flutter/thinkthen_flutter.dart')
        parity_source = parity_source.replace('Future<void> main(List<String> args) async {',
                                              'Future<void> exercise(List<String> args) async {')
        parity_source += '\nFuture<void> main(List<String> args) async {\n  try { await exercise(args); exit(0); } catch (error, stack) { stderr.writeln("$error\\n$stack"); exit(1); }\n}\n'
        (app / 'lib/parity_consumer.dart').write_text(parity_source)
    (app / 'pubspec.yaml').write_text("name: thinkthen_flutter_example\npublish_to: none\nenvironment:\n  sdk: '>=3.10.0 <4.0.0'\ndependencies:\n  flutter:\n    sdk: flutter\n  thinkthen_flutter:\n    path: ../packages/flutter\ndependency_overrides:\n  thinkthen_dart:\n    path: ../packages/dart\nhooks:\n  user_defines:\n    thinkthen_dart:\n      offline: true\n      asset_cache: ../packages/dart/checks/scratch/native-assets/\n")
    run([str(flutter), 'pub', 'get', '--offline'], app, env)
    run([str(flutter), 'build', 'linux', '--release', '--no-pub'], app, env)
    bundle = app / 'build/linux/x64/release/bundle'
    assert list((bundle / 'lib').glob('*thinkthen*')), 'FLUTTER_BUNDLED_NATIVE_ASSET'
    standalone = scratch / 'flutter-bundle'
    shutil.copytree(bundle, standalone)
    app.rename(scratch / 'flutter-app-held')
    packages.rename(scratch / 'packages-held')
    run(['/usr/bin/xvfb-run', '-a', str(standalone / 'thinkthen_flutter_example')], standalone, env)
    (scratch / 'packages-held').rename(packages)
    if full:
        (scratch / 'flutter-app-held').rename(app)
        # Compile the real installed Flutter public caller once, not flutter test
        # once per fixture. Every fixture still starts its own process and home.
        run([str(flutter), 'build', 'linux', '--release', '--no-pub',
             '-t', 'lib/parity_consumer.dart'], app, env)
        parity_bundle = scratch / 'flutter-parity-bundle'
        shutil.copytree(bundle, parity_bundle)
        parity_env.update(THINKTHEN_FLUTTER_CONSUMER=str(app),
                          THINKTHEN_FLUTTER_BINARY=str(parity_bundle / 'thinkthen_flutter_example'))
        if not preparing:
            run(['/usr/bin/xvfb-run', '-a', sys.executable,
                 str(ROOT / 'conformance/children/complete_parity.py'), 'flutter'],
                scratch, parity_env, timeout=None)
    run([sys.executable, str(ROOT / 'conformance/children/native_guard.py'), str(native), *map(str, archives)], scratch, env)
    if preparing:
        save_prepared(scratch, dart, flutter, native, consumer, has_flutter)
    print('PASS: installed Dart bundle and Linux Flutter FFI app; supplied archives' if supplied else 'PASS: installed Dart bundle and Linux Flutter FFI app, cache miss, corrupt cache, loopback build fetch; no caller library path')


if __name__ == '__main__':
    main()
