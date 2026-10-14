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
from backend import Backend, QUOTED_STATE

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--package', type=Path, required=True)
parser.add_argument('--out', type=Path, required=True)
args = parser.parse_args()
args.out.mkdir(parents=True, exist_ok=True)
work = Path(tempfile.mkdtemp(prefix='installed-', dir=args.out.resolve()))
package = work / 'package'
shutil.copytree(args.package, package)
apple = sys.platform == 'darwin'
version = tomllib.loads((ROOT / 'crates/thinkthen/Cargo.toml').read_text())['package']['version']
for name in ('home', 'cache', 'barrier', 'consumer/Sources/Owned', 'consumer/Sources/Parity', 'consumer/Sources/Example'):
    (work / name).mkdir(parents=True, exist_ok=True)
shutil.copyfile(HERE / 'owned_consumer.swift', work / 'consumer/Sources/Owned/main.swift')
shutil.copyfile(PACKAGE / 'Tests/TypeCase/main.swift', work / 'consumer/Sources/Parity/main.swift')
shutil.copyfile(PACKAGE / 'Examples/main.swift', work / 'consumer/Sources/Example/main.swift')
(work / 'consumer/Package.swift').write_text('''// swift-tools-version: 6.0
import PackageDescription
let package = Package(name: "InstalledConsumer", platforms: [.macOS(.v15)], dependencies: [
    .package(url: "''' + (package.as_uri() if apple else 'file:///work/package') + '''", exact: "''' + version + '''")
], targets: [
    .executableTarget(name: "Owned", dependencies: [.product(name: "ThinkThen", package: "package")]),
    .executableTarget(name: "Parity", dependencies: [.product(name: "ThinkThen", package: "package")]),
    .executableTarget(name: "Example", dependencies: [.product(name: "ThinkThen", package: "package")])
])
''')
env = child_env(home=work / 'home', PATH=os.environ.get('PATH', '/usr/bin:/bin'), LANG='C.UTF-8')
for command in (['git', 'init', '-q', str(package)], ['git', '-C', str(package), 'add', '.'],
                ['git', '-C', str(package), '-c', 'user.name=Package Fixture', '-c', 'user.email=fixture@example.invalid', 'commit', '-qm', 'Prepare installed package'],
                ['git', '-C', str(package), 'tag', version]):
    subprocess.run(command, env=env, check=True)
(work / 'empty-tool').write_bytes(b'')
swift = Path(shutil.which('swift')).resolve()
toolchain = swift.parents[2]
base = [] if apple else ['bwrap', '--unshare-all', '--share-net', '--die-with-parent', '--ro-bind', '/usr', '/usr', '--symlink', 'usr/bin', '/bin',
        '--ro-bind', '/lib', '/lib', '--ro-bind', '/lib64', '/lib64', '--ro-bind', str(toolchain), '/swift',
        '--ro-bind', str(work / 'empty-tool'), '/usr/bin/cargo', '--ro-bind', str(work / 'empty-tool'), '/usr/bin/rustc',
        '--bind', str(work), '/work', '--proc', '/proc', '--dev', '/dev', '--tmpfs', '/tmp', '--chdir', '/work/consumer', '--']
if os.environ.get('LLVM_PROFILE_FILE') and not apple:
    observation = Path(os.environ['LLVM_PROFILE_FILE']).parent.absolute()
    profiles = observation.resolve(strict=True)
    boundary = base.index('--')
    base[boundary:boundary] = ['--bind', str(profiles), str(observation)]
location = str(work) if apple else '/work'
env = child_env(keep=('LLVM_PROFILE_FILE', 'GITHUB_ACTIONS', 'RUNNER_ENVIRONMENT'), home=location + '/home', PATH='/usr/bin:/bin' if apple else '/swift/usr/bin:/usr/bin:/bin', LANG='C.UTF-8',
                SWIFTPM_MODULECACHE_OVERRIDE=location + '/cache/modules', XDG_CACHE_HOME=location + '/cache', THINKTHEN_API_KEY='tt-canary-294')
try:
    profile_link = []
    if apple and os.environ.get('THINKTHEN_COVERAGE_INITIALIZER'):
        profile_link = ['-Xlinker', os.environ['THINKTHEN_COVERAGE_INITIALIZER'],
                        '-Xlinker', '-u', '-Xlinker', '___llvm_profile_runtime']
    try:
        built = subprocess.run([sys.executable, str(ROOT / 'sdlc/scripts/build-admission.py'), '--lock', '/tmp/thinkthen-heavy.lock', '--'] + base + [str(swift) if apple else '/swift/usr/bin/swift', 'build', '--jobs', '2'] + profile_link, cwd=work / 'consumer', env=env, capture_output=True, timeout=300)
    except subprocess.TimeoutExpired as failure:
        (args.out / 'installed-build.log').write_bytes((failure.stdout or b'') + (failure.stderr or b''))
        raise
    (args.out / 'installed-build.log').write_bytes(built.stdout + built.stderr)
    assert built.returncode == 0, (built.stdout + built.stderr)[-5000:]
    resolution = json.loads((work / 'consumer/Package.resolved').read_text())
    assert resolution['pins'][0]['state']['version'] == version, resolution
    server = Backend(work / 'barrier', key='tt-canary-294', packed_questions={'Is it?'}, hold_first=True)
    try:
        settings = json.dumps({'base_url': f'http://127.0.0.1:{server.server_port}/generic/v1', 'cache': False, 'max_retries': 0})
        result = subprocess.run(base + [location + '/consumer/.build/debug/Owned', settings, location + '/barrier'], env=env, capture_output=True, timeout=30)
        (args.out / 'installed-owned.log').write_bytes(result.stdout + result.stderr)
        assert result.returncode == 0 and b'OWNED_SWIFT_PASS' in result.stdout, (result.stdout, result.stderr)
        assert server.arrivals[:8] == ['hold-owned-swift', 'owned-swift', QUOTED_STATE, 'unsure', 'uncertain-choice', 'authored-null', 'hold-explicit-swift', 'status-401'] and server.attempts == server.connections == 10, server.arrivals
        print(result.stdout.decode(), end='')
        if not apple:
            asset = work / 'consumer/.build/debug/ThinkThen_ThinkThen.bundle/Native/x86_64-unknown-linux-gnu/libthinkthen.so'
            if not asset.exists():
                asset = next((work / 'consumer/.build/debug/ThinkThen_ThinkThen.bundle/Native').glob('*/libthinkthen.so'))
            hidden = asset.with_suffix('.missing')
            asset.rename(hidden)
            try:
                refused = subprocess.run(base + ['/work/consumer/.build/debug/Owned', settings, '/work/barrier'], env=env, capture_output=True, timeout=5)
                assert refused.returncode != 0 and b'ThinkThen native package could not be loaded' in refused.stderr, (refused.stdout, refused.stderr)
                assert server.attempts == 10, server.arrivals
                print('Missing bundled native asset refuses locally with zero extra requests')
            finally:
                hidden.rename(asset)
    finally:
        (work / 'barrier/release-hold-owned-swift').touch()
        (work / 'barrier/release-hold-explicit-swift').touch()
        server.close()
    examples = Backend(key='tt-canary-examples')
    try:
        example_env = env | {'THINKTHEN_API_KEY': 'tt-canary-examples', 'THINKTHEN_BASE_URL': f'http://127.0.0.1:{examples.server_port}/generic/v1'}
        replayed = subprocess.run(base + [location + '/consumer/.build/debug/Example'], env=example_env, capture_output=True, timeout=30)
        (args.out / 'installed-example.log').write_bytes(replayed.stdout + replayed.stderr)
        assert replayed.returncode == 0, (replayed.stdout, replayed.stderr)
        assert len(replayed.stdout.splitlines()) >= 10 and examples.attempts == 10, (examples.attempts, examples.arrivals)
        expected_options = ['billing', 'shipping', 'account', 'fraud', 'returns']
        expected_kinds = ['person', 'organization', 'location', 'product', 'event']
        criteria = [list(question.get('criteria', {})) for request in examples.requests for question in request['questions'].values()]
        assert any([key for key in keys if key in expected_options] == expected_options for keys in criteria), criteria
        assert any([key for key in keys if key in expected_kinds] == expected_kinds for keys in criteria), criteria
        authored = [question['instructions'] for request in examples.requests for question in request['questions'].values() if any('Is ' + key + '?' in question.get('instructions', '') for key in expected_options)]
        assert len(authored) == 5 and all('Is ' + key + '?' in text for key, text in zip(expected_options, authored)), authored
        print('SWIFT_EXAMPLE_PASS ten typed answer calls and direct edge names')
    finally:
        examples.close()
    # Retain the installed executable and its resource bundle for shared cases.
    installed = args.out / 'installed'
    shutil.copytree(work / 'consumer/.build/debug', installed, symlinks=False)
    print('Installed versioned SwiftPM dependency PASS; bundled native loading, ten counted requests')
finally:
    shutil.rmtree(work)
