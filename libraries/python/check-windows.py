#!/usr/bin/env python3
"""Build and test the installed Windows Python wheel using cached test packages."""
import argparse
import os
from pathlib import Path
import subprocess
import sys
import tempfile

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--packages', type=Path, required=True)
    args = parser.parse_args()
    if os.name != 'nt':
        parser.error('this gate needs native Windows')
    packages = args.packages.resolve()
    names = ('PATH', 'SystemRoot', 'SystemDrive', 'TEMP', 'TMP', 'INCLUDE', 'LIB', 'LIBPATH')
    env = {name: os.environ[name] for name in names if name in os.environ}
    env.update(CARGO_HOME=os.environ.get('CARGO_HOME', str(Path.home() / '.cargo')),
               RUSTUP_HOME=os.environ.get('RUSTUP_HOME', str(Path.home() / '.rustup')),
               CARGO_TARGET_DIR=str(REPO / 'target'), CARGO_NET_OFFLINE='true', CARGO_BUILD_JOBS='2',
               RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='')
    with tempfile.TemporaryDirectory(prefix='thinkthen-python-windows-') as temporary:
        scratch = Path(temporary)
        env.update(HOME=str(scratch / 'home'), APPDATA=str(scratch / 'Roaming'),
                   LOCALAPPDATA=str(scratch / 'Local'))
        for name in ('home', 'Roaming', 'Local'):
            (scratch / name).mkdir()
        env['RUSTFLAGS'] = (f'--remap-path-prefix={REPO.as_posix()}=/build/source '
                            f'--remap-path-prefix={env["CARGO_HOME"]}=/build/cargo '
                            f'--remap-path-prefix={Path.home().as_posix()}=/build/home')

        def run(*command, cwd=HERE):
            subprocess.run(command, cwd=cwd, env=env, check=True, timeout=1800)

        def venv(name, requirements):
            folder = scratch / name
            run(sys.executable, '-m', 'venv', str(folder))
            python = folder / 'Scripts/python.exe'
            run(str(python), '-m', 'pip', 'install', '--no-index', '--find-links',
                str(packages / name), '--require-hashes', '-r', str(HERE / requirements))
            return python

        python = venv('pandas3', 'requirements-dev.txt')
        env['PYO3_PYTHON'] = str(python)
        run('cargo', 'build', '--locked', '--offline', '--package', 'conformance-backend', cwd=REPO)
        run('sh', 'build-wheel.sh')
        wheels = list((HERE / 'target/release-wheel').glob('thinkthen-*-win_amd64.whl'))
        if len(wheels) != 1:
            raise RuntimeError('expected one checked win_amd64 release wheel')
        run(str(python), '-m', 'pip', 'install', '--no-index', '--no-deps', str(wheels[0]))
        run(str(python), '-c', 'import pathlib, sys, thinkthen; '
            'assert pathlib.Path(thinkthen.__file__).is_relative_to(sys.prefix)', cwd=scratch)
        # The production wheel must send nothing on refusal, then answer from
        # the fixture backend. Those tests count real loopback requests.
        env['PYTHONPATH'] = str(HERE / 'tests')
        run(str(python), '-m', 'pytest', '-q', '-p', 'no:cacheprovider',
            str(HERE / 'tests/test_call.py') + '::test_token_cap_variable_refuses_before_any_send',
            str(HERE / 'tests/test_stopping.py') + '::test_a_token_cancelled_before_the_call_sends_nothing',
            str(HERE / 'tests/test_surface.py') + '::test_the_module_functions_equal_an_explicit_engine', cwd=scratch)
        run('cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings')
        run('cargo', 'clippy', '--locked', '--offline', '--all-targets', '--features', 'probe', '--', '-D', 'warnings')
        run('cargo', 'test', '--locked', '--offline', '--no-default-features', '--lib')
        # Full tests use the existing test hooks. The released wheel above
        # contains none; build-wheel.sh checks that distinction.
        run('maturin', 'build', '--quiet', '--locked', '--offline', '--features', 'probe', '-o', str(scratch / 'probe'))
        probe, = (scratch / 'probe').glob('thinkthen-*.whl')
        run(str(python), '-m', 'pip', 'install', '--no-index', '--no-deps', '--force-reinstall', str(probe))
        run(str(python), '-m', 'mypy', '--strict', str(HERE / 'tests/type_contract.py'), cwd=scratch)
        run(str(python), '-m', 'pytest', '-q', '-rs', '-p', 'no:cacheprovider', '-m', 'not stress',
            str(HERE / 'tests'), cwd=scratch)
        older = venv('pandas2', 'requirements-pandas2.txt')
        run(str(older), '-m', 'pip', 'install', '--no-index', '--no-deps', str(probe))
        run(str(older), '-m', 'pytest', '-q', '-rs', '-p', 'no:cacheprovider', '-m', 'not stress',
            str(HERE / 'tests/test_pandas.py'), str(HERE / 'tests/test_secrecy.py'), cwd=scratch)


if __name__ == '__main__':
    main()
