"""Prepare one owned PostgreSQL fixture package using verified existing assets."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import sys

from build_targets import container_labels
from release_prepare_common import run
from native_test_identity import identity as fixture_inputs
from release_images import image as prepared_image

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def output(*command, cwd):
    return subprocess.check_output(command, cwd=cwd, text=True).strip()


def main():
    parser = argparse.ArgumentParser(description=__doc__ + ' Prints one receipt path; failures show at most 30 child log lines.')
    parser.add_argument('source', type=Path, help='clean landed SDK source checkout')
    parser.add_argument('destination', type=Path, help='new owned output directory')
    parser.add_argument('native_producer', type=Path, help='original native pair producer manifest')
    parser.add_argument('--cargo-home', type=Path, help='original producer Cargo configuration')
    parser.add_argument('--floor-image', help='prepared compiler image identity')
    args = parser.parse_args()
    if args.cargo_home is not None:
        os.environ['CARGO_HOME'] = str(args.cargo_home.resolve())
    if args.floor_image is not None:
        os.environ['THINKTHEN_FLOOR_IMAGE'] = args.floor_image
    source = args.source.resolve()
    destination = args.destination.absolute()
    if destination.exists() or destination.is_symlink():
        raise ValueError('output already exists')
    if not (source / 'scripts/postgres-package').is_file():
        raise ValueError('source lacks the PostgreSQL package command')
    revision = output('git', 'rev-parse', 'HEAD', cwd=source)
    if output('git', 'status', '--porcelain', '--untracked-files=all', cwd=source):
        raise ValueError('source checkout must be clean')
    # Package staging globs direct SQL source files; ignored build directories
    # may remain, but unrecorded files beside those inputs cannot enter a receipt.
    if output('git', 'ls-files', '--others', '--ignored', '--exclude-standard', '--',
              ':(glob)extensions/postgres/*', cwd=source):
        raise ValueError('source package inputs must be clean')
    sys.path.insert(0, str(source / 'scripts'))
    pair = runpy.run_path(str(source / 'scripts/native-pair'))
    metadata = runpy.run_path(str(source / 'scripts/native-prepare'))
    cache = metadata['cache_path']()
    producer = pair['load'](args.native_producer)
    if producer['native_key'] != cache.name or producer['floor_image'] != prepared_image('floor'):
        raise ValueError('native producer differs from the verified compatible pair')
    pair['verify'](producer, cache)
    header = source / 'crates/thinkthen-ffi/include/thinkthen.h'
    if digest(header) != producer['assets']['thinkthen.h']:
        raise ValueError('source header differs from the verified native producer')
    registered = dict(producer, revision=revision)
    fixtures = runpy.run_path(str(source / 'scripts/native-test-assets'))
    folder, inputs = fixture_inputs(source, cache, source / 'scripts/native-test-assets')
    fixture_identity = fixtures['verify'](folder, inputs)
    environment = dict(os.environ, THINKTHEN_SQL_TEST_ARCHIVE=str(folder / 'libthinkthen_ffi.a'),
                       THINKTHEN_SQL_ARCHIVE=str(cache / 'linux-x64/libthinkthen_ffi.a'))
    image = output('docker', 'image', 'inspect', environment.get('THINKTHEN_POSTGRES_TEST_IMAGE',
                   'thinkthen-postgres-build:18.6'), '--format', '{{.Id}}', cwd=source)
    destination.mkdir()
    try:
        run(ROOT / 'scripts/postgres-package-link', source, destination / 'built',
            folder / 'libthinkthen_ffi.a', 'ON', env=environment, cwd=source)
        run('docker', 'run', '--pull=never', '--rm', '--network', 'none',
                        *container_labels(ROOT), '-v', f'{source}:/src:ro',
                        '-v', f'{destination}:/work',
                        '-v', f'{ROOT / "scripts/postgres-install-runtime"}:/prepare:ro',
                        image, 'sh', '/prepare', '/src', '/work', str(os.getuid()), str(os.getgid()),
            cwd=source)
        for name in ('installed/lib/thinkthen_postgres.so', 'bundle/runtime/bin/pg_config',
                     'bundle/runtime/bin/initdb', 'bundle/runtime/bin/pg_ctl'):
            if not (destination / name).is_file():
                raise ValueError('installed output missing: ' + name)
        shutil.copy2(args.native_producer, destination / 'native-producer.json')
        shutil.copy2(folder / 'identity.json', destination / 'fixture-producer.json')
        receipt = dict(runner_revision=output('git', 'rev-parse', 'HEAD', cwd=ROOT),
                       package_revision=revision, native=producer, fixtures=fixture_identity,
                       consumer_mapping=registered, postgres_image=image,
                       files={str(path.relative_to(destination)): digest(path)
                              for path in sorted(destination.rglob('*')) if path.is_file()})
        receipt_path = destination / 'preparation.json'
        receipt_path.write_text(json.dumps(receipt, indent=2, sort_keys=True) + '\n')
        print(receipt_path)
    except BaseException:
        try:
            shutil.rmtree(destination)
        except OSError as error:
            print(f"Owned PostgreSQL staging could not be removed: {destination}: {error}", file=sys.stderr)
        raise


if __name__ == '__main__':
    try:
        main()
    except subprocess.CalledProcessError as error:
        sys.exit(error.returncode)
    except (ValueError, OSError) as error:
        sys.exit('PostgreSQL preparation: ' + str(error))
