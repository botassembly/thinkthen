"""Run one healthy PostgreSQL/SystemOne soak from a verified installed closure.

Print one execution record path. Child stdout, stderr and checkpoints stay in the
new evidence directory; on failure, print at most thirty stderr lines.
"""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys

from build_targets import container_labels
from postgres_installed_prepare import digest, output

ROOT = Path(__file__).resolve().parents[1]


def clean_revision(source):
    if output('git', 'status', '--porcelain', '--untracked-files=all', cwd=source):
        raise ValueError('consumer and stress runner source must be clean')
    return output('git', 'rev-parse', 'HEAD', cwd=source)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('preparation', 'source', 'runner', 'loopback', 'evidence'):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    receipt_path = args.preparation.resolve()
    prepared = receipt_path.parent
    receipt = json.loads(receipt_path.read_text())
    image = receipt['postgres_image']
    if not re.fullmatch('sha256:[0-9a-f]{64}', image):
        raise ValueError('prepared PostgreSQL image must have an immutable identity')
    files = receipt['files']
    if {str(path.relative_to(prepared)) for path in prepared.rglob('*') if path.is_file()} != set(files) | {receipt_path.name}:
        raise ValueError('prepared file inventory changed')
    for name, expected in files.items():
        if Path(name).is_absolute() or '..' in Path(name).parts or digest(prepared / name) != expected:
            raise ValueError('prepared file checksum changed: ' + name)
    source, runner, loopback = (path.resolve() for path in (args.source, args.runner, args.loopback))
    evidence = args.evidence.absolute()
    package_revision = clean_revision(source)
    if package_revision != receipt['package_revision']:
        raise ValueError('consumer source differs from the prepared package source')
    runner_revision = clean_revision(runner)
    if args.loopback.is_symlink() or digest(loopback) != receipt['fixtures']['files']['loopback']:
        raise ValueError('loopback differs from the verified fixture producer')
    if evidence.exists() or evidence.is_symlink():
        raise ValueError('evidence output already exists')
    evidence = evidence.resolve()
    if any(evidence.is_relative_to(path) for path in (prepared, source, runner)):
        raise ValueError('evidence output overlaps a verified input')
    evidence.mkdir(parents=True)
    (evidence / 'home').mkdir()
    command = ['docker', 'run', '--pull=never', '--rm', '--network', 'none',
               *container_labels(ROOT), '-v', f'{prepared / "bundle/runtime"}:/opt/thinkthen-pg18:ro',
               '-v', f'{source}:/src:ro', '-v', f'{runner}:/runner:ro',
               '-v', f'{loopback}:/loopback:ro', '-v', f'{evidence}:/evidence',
               '-v', f'{ROOT / "scripts/postgres-soak-container"}:/launch:ro',
               image, 'sh', '/launch', str(os.getuid()), str(os.getgid())]
    with (evidence / 'stdout.log').open('w') as stdout, (evidence / 'stderr.log').open('w') as stderr:
        result = subprocess.run(command, stdout=stdout, stderr=stderr)
    execution = dict(launcher_revision=output('git', 'rev-parse', 'HEAD', cwd=ROOT),
                     stress_runner_revision=runner_revision, package_revision=package_revision,
                     preparation_sha256=digest(receipt_path), preparation_runner=receipt['runner_revision'],
                     native_producer=receipt['native']['revision'],
                     fixture_producer=receipt['fixtures']['built_revision'], image=image, exit_code=result.returncode)
    record = evidence / 'execution.json'
    record.write_text(json.dumps(execution, indent=2, sort_keys=True) + '\n')
    if result.returncode:
        print('\n'.join((evidence / 'stderr.log').read_text().splitlines()[-30:]), file=sys.stderr)
    print(record)
    return result.returncode


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, OSError) as error:
        sys.exit('PostgreSQL soak: ' + str(error))
