"""Resolve copied complete callers against extracted Dart and Flutter packages."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
from urllib.parse import unquote, urlparse

ROOT = Path(__file__).resolve().parents[3]
def resolution(project, name, expected):
    config = project / '.dart_tool/package_config.json'
    entry = next(row for row in json.loads(config.read_text())['packages'] if row['name'] == name)
    actual = Path(unquote(urlparse((config.parent.as_uri() + '/')).path)) / entry['rootUri']
    if entry['rootUri'].startswith('file:'):
        actual = Path(unquote(urlparse(entry['rootUri']).path))
    assert actual.resolve() == expected.resolve(), (name, actual, expected)


def prepare(package, native, consumer):
    package, native = package.resolve(strict=True), native.resolve(strict=True)
    assert consumer in ('dart', 'flutter')
    dart = os.environ['TT_DART']
    flutter = os.environ.get('TT_FLUTTER')
    app = package / 'checks/consumers/alpha'
    app.mkdir(parents=True)
    source = ROOT / 'libraries/dart/checks/consumers/alpha'
    shutil.copytree(source / 'bin', app / 'bin')
    shutil.copy2(source / 'pubspec.yaml', app / 'pubspec.yaml')
    env = dict(os.environ, FLUTTER_SUPPRESS_ANALYTICS='true', CI='true')
    subprocess.run([dart, 'pub', 'get', '--offline', '--directory', str(app)], env=env, check=True)
    
    resolution(app, 'thinkthen_dart', package)
    env.update(THINKTHEN_PARITY_PACKAGE=str(package), THINKTHEN_DART_CONSUMER=str(app),
               THINKTHEN_COMPLETE_LIBRARY=str(native / 'lib/libthinkthen.so'))
    if consumer == 'dart':
        binary = app / 'complete-native'
        subprocess.run([dart, 'compile', 'exe', str(app / 'bin/complete_native.dart'), '-o', str(binary)], env=env, check=True)
        env['THINKTHEN_DART_BINARY'] = str(binary)
    else:
        facade = package / 'flutter'
        (facade / 'lib/thinkthen_flutter.dart').resolve(strict=True)
        example = facade / 'example'
        tests = example / 'test'; tests.mkdir()
        for name in ('complete_constructed_test.dart', 'complete_native_test.dart'):
            shutil.copy2(ROOT / 'libraries/dart/flutter/example/test' / name, tests / name)
        for project in (facade, example):
            subprocess.run([flutter, 'pub', 'get', '--offline'], cwd=project, env=env, check=True)
            resolution(project, 'thinkthen_dart', package)
        resolution(example, 'thinkthen_flutter', facade)
        env['THINKTHEN_FLUTTER_CONSUMER'] = str(example)
    return env


if __name__ == "__main__":
    env = prepare(Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3])
    raise SystemExit(subprocess.run([sys.executable, str(ROOT / "libraries/php/fixtures/complete_parity.py"), sys.argv[3]], env=env).returncode)
