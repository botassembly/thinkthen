"""Install development packages and exercise SDK-owned Linux native assets offline."""
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


def run(command, cwd, env, expected=None):
    result = subprocess.run(command, cwd=cwd, env=env, text=True, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, timeout=180)
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
    for name, source in [('dart', ROOT / 'libraries/dart'), ('flutter', ROOT / 'libraries/dart/flutter')]:
        archive = scratch / f'thinkthen-{name}.tar.gz'
        with tarfile.open(archive, 'w:gz') as output:
            for path in sorted(source.rglob('*')):
                relative = path.relative_to(source)
                if any(part in {'.dart_tool', 'build', 'scratch', 'checks', 'flutter', 'example', 'logs'} for part in relative.parts):
                    continue
                if path.is_file() and (relative.parts[0] in {'lib', 'hook', 'linux'} or str(relative) in {'native-assets.json', 'pubspec.yaml', 'README.md', 'LICENSE', 'CHANGELOG.md'}):
                    output.add(path, arcname=str(relative))
        target = packages / name
        target.mkdir()
        with tarfile.open(archive) as content:
            content.extractall(target, filter='data')
        print(name, hashlib.sha256(archive.read_bytes()).hexdigest())
    definition = packages / 'dart/native-assets.json'
    manifest = json.loads(definition.read_text())
    asset = manifest['assets']['linux_x64']
    payload = native.read_bytes()
    assert hashlib.sha256(payload).hexdigest() == asset['sha256'], 'NATIVE_FIXTURE_IDENTITY'
    cache = scratch / 'asset-cache'
    cached = cache / asset['sha256'] / asset['file']
    cached.parent.mkdir(parents=True)
    cached.write_bytes(payload)
    consumer = scratch / 'consumer'
    (consumer / 'bin').mkdir(parents=True)
    shutil.copyfile(ROOT / 'libraries/dart/checks/session_consumer/bin/main.dart', consumer / 'bin/main.dart')
    spec = ("name: installed_native_assets\npublish_to: none\nenvironment:\n  sdk: '>=3.10.0 <4.0.0'\ndependencies:\n  thinkthen_dart:\n    path: ../packages/dart\nhooks:\n  user_defines:\n    thinkthen_dart:\n      offline: true\n      asset_cache: ../asset-cache/\n")
    (consumer / 'pubspec.yaml').write_text(spec)
    run([str(dart), 'pub', 'get', '--offline'], consumer, env)
    cached.rename(cached.with_suffix('.held'))
    run([str(dart), 'run', 'bin/main.dart'], consumer, env, 'cache miss in offline build')
    cached.write_bytes(b'corrupted native library')
    run([str(dart), 'run', 'bin/main.dart'], consumer, env, 'checksum mismatch')
    cached.with_suffix('.held').replace(cached)
    run([str(dart), 'run', 'bin/main.dart'], consumer, env)
    # A separate installed project obtains the same pinned bytes during its build.
    download_consumer = scratch / 'download-consumer'
    shutil.copytree(consumer / 'bin', download_consumer / 'bin')
    consumer = download_consumer
    download_spec = spec.replace('offline: true', 'offline: false').replace('      asset_cache: ../asset-cache/\n', '')
    (consumer / 'pubspec.yaml').write_text(download_spec)
    run([str(dart), 'pub', 'get', '--offline'], consumer, env)
    run([str(dart), 'run', 'bin/main.dart'], consumer, env, 'no approved download URL')
    requests = []
    class Handler(http.server.BaseHTTPRequestHandler):
        def do_GET(self):
            requests.append(self.path)
            self.send_response(200)
            self.send_header('Content-Length', str(len(payload)))
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
    # Run the compiled bundle after the source packages and build cache move away.
    packages.rename(scratch / 'packages-held')
    consumer.rename(scratch / 'consumer-held')
    run([str(scratch / 'dart-build/bundle/bin/main')], scratch / 'dart-build/bundle', env)
    (scratch / 'packages-held').rename(packages)
    (scratch / 'consumer-held').rename(consumer)
    app = scratch / 'flutter-app'
    shutil.copytree(ROOT / 'libraries/dart/flutter/example/linux', app / 'linux')
    (app / 'lib').mkdir()
    source = (consumer / 'bin/main.dart').read_text().replace('package:thinkthen_dart/thinkthen_session.dart', 'package:thinkthen_flutter/thinkthen_session_flutter.dart').replace('(dart)', '(flutter)')
    source = source.replace('Future<void> main(List<String> args) async {', 'Future<void> exercise() async {')
    source += '\nFuture<void> main() async {\n  try { await exercise(); exit(0); } catch (error, stack) { stderr.writeln("$error\\n$stack"); exit(1); }\n}\n'
    (app / 'lib/main.dart').write_text(source)
    (app / 'pubspec.yaml').write_text("name: thinkthen_flutter_example\npublish_to: none\nenvironment:\n  sdk: '>=3.10.0 <4.0.0'\ndependencies:\n  flutter:\n    sdk: flutter\n  thinkthen_flutter:\n    path: ../packages/flutter\ndependency_overrides:\n  thinkthen_dart:\n    path: ../packages/dart\nhooks:\n  user_defines:\n    thinkthen_dart:\n      offline: true\n      asset_cache: ../asset-cache/\n")
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
    run([sys.executable, str(ROOT / 'libraries/zig/Tests/guard.py'), str(native), str(scratch / 'thinkthen-dart.tar.gz'), str(scratch / 'thinkthen-flutter.tar.gz')], scratch, env)
    print('PASS: installed Dart bundle and Linux Flutter FFI app, cache miss, corrupt cache, loopback build fetch; no caller library path')


if __name__ == '__main__':
    main()
