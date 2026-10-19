"""Build the outside consumer with its dependency's declared Rust compiler."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tomllib


def toml_value(value):
    """Encode the parsed consumer manifest's scalar, array and inline tables."""
    if isinstance(value, dict):
        return '{' + ', '.join(json.dumps(k) + ' = ' + toml_value(v)
                               for k, v in value.items()) + '}'
    if isinstance(value, list):
        return '[' + ', '.join(map(toml_value, value)) + ']'
    return json.dumps(value)


def main():
    root = Path(sys.argv[1])
    project = root / 'libraries/rust/consumer'
    crate = root / 'crates/thinkthen'
    if len(sys.argv) > 2:
        installed = Path(sys.argv[2])
        crate = installed / 'crate'
        shutil.copytree(project, installed / 'consumer', dirs_exist_ok=True,
                        ignore=shutil.ignore_patterns('target'))
        project = installed / 'consumer'
        manifest = tomllib.loads((project / 'Cargo.toml').read_text())
        manifest['dependencies']['thinkthen']['path'] = str(crate)
        manifest['dev-dependencies']['conformance-backend']['path'] = str(root / 'conformance/backend')
        (project / 'Cargo.toml').write_text('\n'.join(
            json.dumps(key) + ' = ' + toml_value(value)
            for key, value in manifest.items()) + '\n')
    package = tomllib.loads((crate / 'Cargo.toml').read_text())['package']
    compiler = package['rust-version']
    if isinstance(compiler, dict) and compiler.get('workspace'):
        compiler = tomllib.loads((root / 'Cargo.toml').read_text())['workspace']['package']['rust-version']
    target = Path(os.environ.get('CARGO_TARGET_DIR', str(root / 'libraries/rust/target')))
    binary = target / 'debug/thinkthen-rust-consumer'
    binary.parent.mkdir(parents=True, exist_ok=True)
    # The retained shared fixture finds its backend through CARGO_TARGET_DIR.
    backend = os.environ.get('THINKTHEN_TEST_BACKEND')
    if backend:
        os.environ['CARGO_TARGET_DIR'] = str(Path(backend).parents[1])
    sys.path.insert(0, str(root / 'conformance/children'))
    from native_fixture import run
    compiler_env = {'RUSTUP_TOOLCHAIN': compiler}
    profile = os.environ.get('THINKTHEN_TEST_PROFILE', 'routine')
    if profile != 'stress' and run('rust', [str(binary)], root,
                                 rust_manifest=project / 'Cargo.toml',
                                 extra_env=compiler_env):
        return True
    if len(sys.argv) > 2:
        command = ['cargo', 'test', '--locked', '--offline', '--manifest-path',
                   str(project / 'Cargo.toml'), '--target-dir', str(target)]
        subprocess.run([*command, '--test', 'sessions', '--test', 'settled', '--', '--test-threads=1'],
                       env={**os.environ, **compiler_env}, check=True)
        if profile == 'stress':
            subprocess.run([*command, '--features', 'session-stress', '--test', 'sessions',
                            'repeated_session_contention_never_loses_wake_or_settlement',
                            '--', '--exact', '--test-threads=1'],
                           env={**os.environ, **compiler_env}, check=True)
    return False


if __name__ == '__main__':
    sys.exit(main())
