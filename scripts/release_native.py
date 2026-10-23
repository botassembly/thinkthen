"""Build the release Rust surfaces together, then reuse their immutable outputs."""
from contextlib import contextmanager
import fcntl
import hashlib
import json
import os
from pathlib import Path
import shutil
import sys
import tempfile

from native_platform import current

PACKAGES = ('thinkthen', 'thinkthen-ffi', 'thinkthen-python', 'thinkthen-node', 'thinkthen-r')


def source_inputs(root, run):
    root = Path(root).resolve()
    metadata = json.loads(run('cargo', 'metadata', '--offline', '--locked', '--format-version=1').stdout)
    packages = {item['id']: item for item in metadata['packages']}
    nodes = {item['id']: item for item in metadata['resolve']['nodes']}
    pending = [item['id'] for item in packages.values() if item['name'] in PACKAGES]
    visited = set()
    folders = set()
    while pending:
        identity = pending.pop()
        if identity in visited: continue
        visited.add(identity)
        item = packages[identity]
        if item['source'] is None:
            folders.add(str(Path(item['manifest_path']).resolve().parent.relative_to(root)))
        pending.extend(dependency['pkg'] for dependency in nodes[identity]['deps']
                       if any(kind['kind'] != 'dev' for kind in dependency['dep_kinds']))
    names = run('git', 'ls-files', '--', *sorted(folders), 'Cargo.toml', 'Cargo.lock',
                'rust-toolchain.toml', '.cargo').stdout.splitlines()
    return {name: hashlib.sha256((root / name).read_bytes()).hexdigest() for name in names
            if '/tests/' not in name and not name.endswith('.md')}


def stage_outputs(source, destination, inputs, artifacts):
    with tempfile.TemporaryDirectory(prefix='publish-', dir=destination.parent) as temporary:
        stage = Path(temporary) / 'artifacts'
        stage.mkdir()
        for name in artifacts:
            shutil.copy2(source / name, stage / name)
        sums = {name: hashlib.sha256((stage / name).read_bytes()).hexdigest() for name in artifacts}
        (stage / 'identity.json').write_text(json.dumps({'inputs': inputs, 'artifacts': sums}, sort_keys=True, indent=2) + '\n')
        if destination.exists(): shutil.rmtree(destination)
        stage.replace(destination)


def complete(folder, inputs, artifacts):
    try:
        manifest = json.loads((folder / 'identity.json').read_text())
        return manifest['inputs'] == inputs and set(manifest['artifacts']) == set(artifacts) and all(
            hashlib.sha256((folder / name).read_bytes()).hexdigest() == manifest['artifacts'][name] for name in artifacts)
    except (OSError, ValueError, KeyError):
        return False


def publish(source, destination):
    temporary = destination.with_suffix(destination.suffix + '.partial')
    shutil.copy2(source, temporary)
    temporary.replace(destination)


@contextmanager
def prepare(root, run, native_run):
    host = current()
    cache = Path(run('scripts/native-prepare', '--path').stdout.strip())
    inputs = source_inputs(root, run)
    for name in ('scripts/release_native.py', 'scripts/runtime-floor-build', 'scripts/floors/Dockerfile'):
        inputs[name] = hashlib.sha256((root / name).read_bytes()).hexdigest()
    inputs['ffi'] = cache.name
    inputs['target'] = host['rust']
    inputs['R'] = native_run('R', '--version').stdout.splitlines()[0]
    inputs['packages'] = list(PACKAGES)
    inputs['profile'] = ['--release', '--offline', '--locked', 'PYO3_NO_PYTHON=1']
    digest = hashlib.sha256(json.dumps(inputs, sort_keys=True).encode()).hexdigest()
    output = cache / 'release' / digest
    output.mkdir(parents=True, exist_ok=True)
    payload = output / 'artifacts'
    artifacts = ('thinkthen', 'libthinkthen_ffi.' + host['suffix'], 'libthinkthen_ffi.a',
                 'lib_native.' + host['suffix'], 'libthinkthen_node.' + host['suffix'], 'libthinkthen_r.a')
    with (output / 'build.lock').open('w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        if not complete(payload, inputs, artifacts):
            target = output / 'target'
            arguments = ['cargo', 'build', '--release', '--offline', '--locked', '--target', host['rust'],
                         '--message-format=json-render-diagnostics']
            for package in PACKAGES:
                arguments += ['-p', package]
            workers = str(max(1, int(float(os.environ.get('PM_JOB_CORES', '2')))))
            env = dict(os.environ, CARGO_TARGET_DIR=str(target), PYO3_NO_PYTHON='1',
                       CARGO_BUILD_JOBS=os.environ.get('CARGO_BUILD_JOBS', workers))
            result = native_run(*arguments, env=env)
            messages = output / 'artifacts.json'
            messages.write_text(result.stdout)
            selected = target / host['rust'] / 'release'
            for artifact in ('libthinkthen_ffi.' + host['suffix'], 'libthinkthen_r.a'):
                run(sys.executable, 'scripts/package-profile', messages, selected / artifact, 'release')
            stage_outputs(selected, payload, inputs, artifacts)
            print('candidate: shared Rust release build', flush=True)
        else:
            print('candidate: shared Rust release cache hit', flush=True)
        # The same lock and atomic file publication protect the routine producer.
        with (cache / (host['rust'] + '.lock')).open('w') as host_lock:
            fcntl.flock(host_lock, fcntl.LOCK_EX)
            destination = cache / host['rid']
            destination.mkdir(parents=True, exist_ok=True)
            for name in ('libthinkthen_ffi.' + host['suffix'], 'libthinkthen_ffi.a'):
                if not (destination / name).exists():
                    publish(payload / name, destination / name)
            alias = cache / 'jvm' / host['jvm']
            alias.parent.mkdir(exist_ok=True)
            if not alias.exists() and not alias.is_symlink():
                alias.symlink_to(Path('..') / host['rid'], target_is_directory=True)
            publish(root / 'crates/thinkthen-ffi/include/thinkthen.h', cache / 'thinkthen.h')
        yield cache, payload
