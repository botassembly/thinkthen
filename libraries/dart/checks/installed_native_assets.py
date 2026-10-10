"""Install development packages and exercise SDK-owned Linux native assets offline."""
import fcntl
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

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
from native_assets import configure, package_members


def run(command, cwd, env, expected=None, timeout=180):
    result = subprocess.run(command, cwd=cwd, env=env, text=True, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, timeout=timeout)
    if expected:
        assert result.returncode != 0 and expected in result.stdout, result.stdout
    else:
        assert result.returncode == 0, result.stdout
    print(result.stdout[-2000:])
    return result


def main():
    dart, flutter, native, scratch, pub_cache = (Path(value).resolve() for value in sys.argv[1:])
    scratch.mkdir(parents=True, exist_ok=False)
    env = child_env(home=scratch / 'home', PATH='/usr/bin:/bin', LANG='C.UTF-8',
                    PUB_CACHE=str(pub_cache), FLUTTER_SUPPRESS_ANALYTICS='true',
                    CI='true')
    packages = scratch / 'packages'
    packages.mkdir()
    full = os.environ.get('THINKTHEN_TEST_PROFILE') == 'full'
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
        target = packages / name
        target.mkdir()
        with tarfile.open(archive) as content:
            content.extractall(target, filter='data')
        archives.append(archive)
        print(name, hashlib.sha256(archive.read_bytes()).hexdigest())
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
    run([str(dart), 'run', 'bin/parser_cases.dart', str(ROOT / 'specification/fixtures/types/corpus.json')], consumer, env)
    has_flutter = (packages / 'flutter').is_dir()
    if has_flutter:
        configure(packages / 'dart', native, [packages / 'flutter'])
        run([str(flutter), 'pub', 'get', '--offline'], packages / 'flutter', env)
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
        lock = None
        if mode == 'failed':
            folder = Path(usage_env['XDG_STATE_HOME']) / 'thinkthen'
            folder.mkdir(parents=True, mode=0o700)
            lock = (folder / '.lock').open('w')
            os.chmod(folder / '.lock', 0o600)
            fcntl.flock(lock, fcntl.LOCK_EX)
        if mode == 'disabled':
            usage_env.pop('HOME', None)
            usage_env.pop('XDG_STATE_HOME', None)
        try:
            run([str(dart), 'run', 'bin/main.dart', 'usage-' + mode], consumer, usage_env)
        finally:
            if lock:
                lock.close()
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
        run([sys.executable, str(ROOT / 'libraries/php/fixtures/complete_parity.py'), 'dart'], scratch, parity_env, timeout=None)
    # Run the compiled bundle after the source packages and build cache move away.
    packages.rename(scratch / 'packages-held')
    consumer.rename(scratch / 'consumer-held')
    run([str(scratch / 'dart-build/bundle/bin/main')], scratch / 'dart-build/bundle', env)
    (scratch / 'packages-held').rename(packages)
    (scratch / 'consumer-held').rename(consumer)
    if not has_flutter:
        run([sys.executable, str(ROOT / 'libraries/zig/Tests/guard.py'), str(native), *map(str, archives)], scratch, env)
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
        (app / 'test').mkdir()
        shutil.copyfile(ROOT / 'libraries/dart/checks/session_consumer/parity_test.dart', app / 'test/parity_test.dart')
        parity_source = (consumer / 'bin/parity_consumer.dart').read_text().replace('package:thinkthen_dart/thinkthen_dart.dart', 'package:thinkthen_flutter/thinkthen_flutter.dart')
        (app / 'lib/parity_consumer.dart').write_text(parity_source)
    (app / 'pubspec.yaml').write_text("name: thinkthen_flutter_example\npublish_to: none\nenvironment:\n  sdk: '>=3.10.0 <4.0.0'\ndependencies:\n  flutter:\n    sdk: flutter\n  thinkthen_flutter:\n    path: ../packages/flutter\ndependency_overrides:\n  thinkthen_dart:\n    path: ../packages/dart\nhooks:\n  user_defines:\n    thinkthen_dart:\n      offline: true\n      asset_cache: ../packages/dart/checks/scratch/native-assets/\n")
    if full:
        with (app / 'pubspec.yaml').open('a') as output:
            output.write('dev_dependencies:\n  flutter_test:\n    sdk: flutter\n')
    run([str(flutter), 'pub', 'get', '--offline'], app, env)
    if full:
        parity_env.update(THINKTHEN_FLUTTER_CONSUMER=str(app))
        run([sys.executable, str(ROOT / 'libraries/php/fixtures/complete_parity.py'), 'flutter'], scratch, parity_env, timeout=None)
    run([str(flutter), 'build', 'linux', '--release', '--no-pub'], app, env)
    bundle = app / 'build/linux/x64/release/bundle'
    assert list((bundle / 'lib').glob('*thinkthen*')), 'FLUTTER_BUNDLED_NATIVE_ASSET'
    standalone = scratch / 'flutter-bundle'
    shutil.copytree(bundle, standalone)
    app.rename(scratch / 'flutter-app-held')
    packages.rename(scratch / 'packages-held')
    run(['/usr/bin/xvfb-run', '-a', str(standalone / 'thinkthen_flutter_example')], standalone, env)
    (scratch / 'packages-held').rename(packages)
    run([sys.executable, str(ROOT / 'libraries/zig/Tests/guard.py'), str(native), *map(str, archives)], scratch, env)
    print('PASS: installed Dart bundle and Linux Flutter FFI app; supplied archives' if supplied else 'PASS: installed Dart bundle and Linux Flutter FFI app, cache miss, corrupt cache, loopback build fetch; no caller library path')


if __name__ == '__main__':
    main()
