"""Check generated declarations once per clean source tree and compiler context."""
import contextlib
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent


@contextlib.contextmanager
def locked(path):
    with path.open('a+b') as lock:
        if os.name == 'nt':
            import errno
            import msvcrt
            if lock.tell() == 0:
                lock.write(b'\0')
                lock.flush()
            lock.seek(0)
            while True:
                try:
                    msvcrt.locking(lock.fileno(), msvcrt.LK_LOCK, 1)
                    break
                except OSError as error:
                    # LK_LOCK gives up after about ten seconds with EDEADLOCK, which
                    # Windows defines as EDEADLK; keep waiting.
                    if error.errno not in (errno.EACCES, errno.EDEADLK):
                        raise
        else:
            import fcntl
            fcntl.flock(lock, fcntl.LOCK_EX)
        yield


def check(root=ROOT):
    root = Path(root).resolve()
    def output(*arguments):
        return subprocess.check_output(arguments, cwd=root, text=True).strip()

    compiler = output('rustc', '-vV')
    host = compiler.split('host: ')[1].splitlines()[0]
    cargo = Path(os.environ.get('CARGO_HOME', Path.home() / '.cargo'))
    inputs = {
        'root': str(root),
        'tree': output('git', 'rev-parse', 'HEAD^{tree}'),
        'compiler': compiler,
        'configuration': {name: hashlib.sha256((cargo / name).read_bytes()).hexdigest()
                          for name in ('config', 'config.toml') if (cargo / name).is_file()},
        'environment': {name: value for name, value in os.environ.items()
                        if name.startswith(('RUST', 'CARGO_BUILD_', 'CARGO_TARGET_', 'CARGO_ENCODED_RUSTFLAGS'))},
    }
    key = hashlib.sha256(json.dumps(inputs, sort_keys=True).encode()).hexdigest()
    dirty = bool(output('git', 'status', '--porcelain', '--untracked-files=normal'))
    folder = Path.home() / '.local/share/thinkthen-build-tools/header-contract'
    folder.mkdir(parents=True, exist_ok=True)
    stamp = folder / (key + '.passed')
    with locked(folder / (key + '.lock')):
        if not dirty and stamp.is_file() and stamp.read_text() == key:
            return
        context = 'floor-target' if os.environ.get('THINKTHEN_FLOOR_BUILD') == '1' else 'host-target'
        # One build per machine and context; checkouts share it. The older per-checkout
        # folders sat under folder / context / host, so this layout keeps clear of them.
        target = folder / 'build' / context / host
        if os.name == 'nt':
            # MSVC also needs room for Cargo's target triple and object paths.
            target = folder / hashlib.sha256(json.dumps([context, host]).encode()).hexdigest()
        target.mkdir(parents=True, exist_ok=True)
        environment = dict(os.environ, CARGO_TARGET_DIR=str(target))
        built = target / 'source-tree'
        with locked(target / 'build.lock'):
            # Cargo judges freshness by file times, so a checkout made before another
            # checkout's build looks fresh even when its sources differ. Compile the
            # workspace crates afresh unless this folder last built this same clean tree.
            tree = None if dirty else inputs['tree']
            if tree is None or not built.is_file() or built.read_text() != tree:
                built.unlink(missing_ok=True)
                members = json.loads(output('cargo', 'metadata', '--offline', '--locked', '--no-deps',
                                            '--format-version', '1'))['packages']
                clean = ['cargo', 'clean', '--offline', '--target', host]
                for member in members:
                    clean += ['-p', member['name']]
                subprocess.run(clean, cwd=root, check=True, stdout=sys.stderr, env=environment)
            # This is the existing sole generator, not another declaration parser.
            # Explicit host selection also supports a Windows cross-build caller.
            # Cargo gives the generator this checkout as CARGO_MANIFEST_DIR at run time.
            subprocess.run(['cargo', 'run', '--quiet', '--offline', '--locked', '--target', host,
                            '-p', 'thinkthen-ffi', '--example', 'headers', '--', '--check'],
                           cwd=root, check=True, stdout=sys.stderr, env=environment)
            if tree is not None:
                built.write_text(tree)
        if not dirty:
            stamp.write_text(key)
