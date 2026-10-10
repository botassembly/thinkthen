"""Install one versioned SwiftPM dependency; no checkout or Rust tool is visible."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).resolve().parent
PACKAGE = HERE.parents[1]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
from backend import Backend

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--package', type=Path, required=True)
parser.add_argument('--out', type=Path, required=True)
args = parser.parse_args()
args.out.mkdir(parents=True, exist_ok=True)
work = Path(tempfile.mkdtemp(prefix='installed-', dir=args.out.resolve()))
package = work / 'package'
shutil.copytree(args.package, package)
version = tomllib.loads((ROOT / 'crates/thinkthen/Cargo.toml').read_text())['package']['version']
for name in ('home', 'cache', 'barrier', 'consumer/Sources/Owned', 'consumer/Sources/Parity'):
    (work / name).mkdir(parents=True, exist_ok=True)
shutil.copyfile(HERE / 'owned_consumer.swift', work / 'consumer/Sources/Owned/main.swift')
shutil.copyfile(PACKAGE / 'Tests/TypeCase/main.swift', work / 'consumer/Sources/Parity/main.swift')
(work / 'consumer/Package.swift').write_text('''// swift-tools-version: 6.0
import PackageDescription
let package = Package(name: "InstalledConsumer", dependencies: [
    .package(url: "file:///work/package", exact: "''' + version + '''")
], targets: [
    .executableTarget(name: "Owned", dependencies: [.product(name: "ThinkThen", package: "package")]),
    .executableTarget(name: "Parity", dependencies: [.product(name: "ThinkThen", package: "package")])
])
''')
env = child_env(home=work / 'home', PATH=os.environ.get('PATH', '/usr/bin:/bin'), LANG='C.UTF-8')
for command in (['git', 'init', '-q', str(package)], ['git', '-C', str(package), 'add', '.'],
                ['git', '-C', str(package), '-c', 'user.name=Package Fixture', '-c', 'user.email=fixture@example.invalid', 'commit', '-qm', 'Prepare installed package'],
                ['git', '-C', str(package), 'tag', version]):
    subprocess.run(command, env=env, check=True)
swift = Path(shutil.which('swift')).resolve()
toolchain = swift.parents[2]
base = ['bwrap', '--unshare-all', '--share-net', '--die-with-parent', '--ro-bind', '/usr', '/usr', '--symlink', 'usr/bin', '/bin',
        '--ro-bind', '/lib', '/lib', '--ro-bind', '/lib64', '/lib64', '--ro-bind', str(toolchain), '/swift',
        '--bind', str(work), '/work', '--proc', '/proc', '--dev', '/dev', '--tmpfs', '/tmp', '--chdir', '/work/consumer', '--']
env = child_env(home='/work/home', PATH='/swift/usr/bin:/usr/bin:/bin', LANG='C.UTF-8',
                SWIFTPM_MODULECACHE_OVERRIDE='/work/cache/modules', XDG_CACHE_HOME='/work/cache', THINKTHEN_API_KEY='tt-canary-294')
try:
    built = subprocess.run(base + ['/swift/usr/bin/swift', 'build', '--jobs', '2'], env=env, capture_output=True, timeout=300)
    (args.out / 'installed-build.log').write_bytes(built.stdout + built.stderr)
    assert built.returncode == 0, (built.stdout + built.stderr)[-5000:]
    resolution = json.loads((work / 'consumer/Package.resolved').read_text())
    assert resolution['pins'][0]['state']['version'] == version, resolution
    server = Backend(work / 'barrier')
    try:
        settings = json.dumps({'base_url': f'http://127.0.0.1:{server.server_port}/generic/v1', 'cache': False, 'max_retries': 0})
        result = subprocess.run(base + ['/work/consumer/.build/debug/Owned', settings, '/work/barrier'], env=env, capture_output=True, timeout=30)
        (args.out / 'installed-owned.log').write_bytes(result.stdout + result.stderr)
        assert result.returncode == 0 and b'OWNED_SWIFT_PASS' in result.stdout, (result.stdout, result.stderr)
        assert server.arrivals == ['hold-owned-swift', 'owned-swift', 'status-401'] and server.attempts == server.connections == 3, server.arrivals
        print(result.stdout.decode(), end='')
        asset = work / 'consumer/.build/debug/ThinkThen_ThinkThen.bundle/Native/x86_64-unknown-linux-gnu/libthinkthen.so'
        if not asset.exists():
            asset = next((work / 'consumer/.build/debug/ThinkThen_ThinkThen.bundle/Native').glob('*/libthinkthen.so'))
        hidden = asset.with_suffix('.missing')
        asset.rename(hidden)
        try:
            refused = subprocess.run(base + ['/work/consumer/.build/debug/Owned', settings, '/work/barrier'], env=env, capture_output=True, timeout=5)
            assert refused.returncode != 0 and b'ThinkThen native package could not be loaded' in refused.stderr, (refused.stdout, refused.stderr)
            assert server.attempts == 3, server.arrivals
            print('Missing bundled native asset refuses locally with zero extra requests')
        finally:
            hidden.rename(asset)
    finally:
        (work / 'barrier/release-hold-owned-swift').touch()
        server.close()
    # Retain the installed executable and its resource bundle for shared cases.
    installed = args.out / 'installed'
    shutil.copytree(work / 'consumer/.build/debug', installed, symlinks=False)
    print('Installed versioned SwiftPM dependency PASS; bundled native loading, three counted requests')
finally:
    shutil.rmtree(work)
