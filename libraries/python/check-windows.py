#!/usr/bin/env python3
"""Build and test the installed Windows Python wheel using cached test packages."""
import argparse
from contextlib import chdir
import os
from pathlib import Path
import subprocess
import shutil
import runpy
import sys
import tempfile

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
CAPTURE = runpy.run_path(str(REPO / 'sdlc/scripts/release-bounded.py'))['capture']


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
               RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='', PYTHONUTF8='1')
    with tempfile.TemporaryDirectory(prefix='thinkthen-python-windows-') as temporary:
        scratch = Path(temporary)
        env.update(HOME=str(scratch / 'home'), APPDATA=str(scratch / 'Roaming'),
                   LOCALAPPDATA=str(scratch / 'Local'))
        for name in ('home', 'Roaming', 'Local'):
            (scratch / name).mkdir()
        # Copy only fixtures and tests. No source package can satisfy imports.
        checked = scratch / 'libraries/python'
        checked.mkdir(parents=True)
        shutil.copytree(HERE / 'tests', checked / 'tests', ignore=shutil.ignore_patterns('__pycache__'))
        shutil.copyfile(HERE / 'examples.json', checked / 'examples.json')
        shutil.copytree(REPO / 'conformance', scratch / 'conformance',
                        ignore=shutil.ignore_patterns('target', '__pycache__'))
        shutil.copytree(REPO / 'specification/fixtures/batching', scratch / 'specification/fixtures/batching')
        shutil.copytree(REPO / 'specification/fixtures/files', scratch / 'specification/fixtures/files')
        shutil.copytree(REPO / 'specification/fixtures/question-file', scratch / 'specification/fixtures/question-file')
        for relative in ('site/examples/learn/python/files.py', 'site/examples/learn/python/files.py.out',
                         'site/recordings/thinkthen.jsonl'):
            destination = scratch / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(REPO / relative, destination)
        env['RUSTFLAGS'] = (f'--remap-path-prefix={REPO.as_posix()}=/build/source '
                            f'--remap-path-prefix={env["CARGO_HOME"]}=/build/cargo '
                            f'--remap-path-prefix={Path.home().as_posix()}=/build/home')

        def run(*command, cwd=HERE, extra=None):
            with chdir(cwd):
                result = CAPTURE(command, env=env | (extra or {}), text=True, timeout=1800)
            print(result.stdout, end='')
            print(result.stderr, end='', file=sys.stderr)
            result.check_returncode()

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
        installed = ('import pathlib, sys, thinkthen, thinkthen._thinkthen as native; '
                     'assert all(pathlib.Path(p).resolve().is_relative_to(pathlib.Path(sys.prefix).resolve()) '
                     'for p in (thinkthen.__file__, native.__file__))')
        run(str(python), '-c', installed, cwd=checked)
        # The production wheel must send nothing on refusal, then answer from
        # the fixture backend. Those tests count real loopback requests.
        env['PYTHONPATH'] = str(checked / 'tests')
        run(str(python), '-m', 'pytest', '-q', '-p', 'no:cacheprovider', '-m', 'not stress', 'tests', cwd=checked)
        run(str(python), '-m', 'mypy', '--strict', 'tests/native_types.py', cwd=checked)
        run('cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings')
        run('cargo', 'clippy', '--locked', '--offline', '--all-targets', '--features', 'probe', '--', '-D', 'warnings')
        run('cargo', 'test', '--locked', '--offline', '--no-default-features', '--lib')
        older = venv('pandas2', 'requirements-pandas2.txt')
        run(str(older), '-m', 'pip', 'install', '--no-index', '--no-deps', str(wheels[0]))
        run(str(older), '-c', installed, cwd=checked)
        run(str(older), '-m', 'pytest', '-q', '-p', 'no:cacheprovider', '-m', 'not stress',
            'tests/test_series_accessors.py', 'tests/test_call_boundaries.py', cwd=checked)


if __name__ == '__main__':
    main()
