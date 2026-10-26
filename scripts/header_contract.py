"""Check generated declarations once per clean source tree and compiler context."""
import fcntl
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent


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
    with (folder / (key + '.lock')).open('w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        if not dirty and stamp.is_file() and stamp.read_text() == key:
            return
        context = 'floor-target' if os.environ.get('THINKTHEN_FLOOR_BUILD') == '1' else 'host-target'
        # The generator embeds CARGO_MANIFEST_DIR, so its executable belongs to this root.
        target = folder / context / host / hashlib.sha256(os.fsencode(root)).hexdigest()
        # This is the existing sole generator, not another declaration parser.
        # Explicit host selection also supports a Windows cross-build caller.
        subprocess.run(['cargo', 'run', '--quiet', '--offline', '--locked', '--target', host,
                        '-p', 'thinkthen-ffi', '--example', 'headers', '--', '--check'],
                       cwd=root, check=True, stdout=sys.stderr,
                       env=dict(os.environ, CARGO_TARGET_DIR=str(target)))
        if not dirty:
            stamp.write_text(key)
