#!/usr/bin/env python3
"""Shared dependency policy for the existing lint and registry rungs."""
import argparse
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
MODES = ('ticket', 'checkpoint', 'release')


def context_mode(checks, ticket, requested=None):
    """Only an identified routine ticket may select the relaxed advisory rule."""
    if requested is not None and requested not in MODES:
        raise ValueError('unknown advisory mode: ' + requested)
    if requested in ('checkpoint', 'release'):
        return requested
    if ticket == '0691' or set(checks) - {'lint', 'test'} or not ticket:
        return 'checkpoint'
    return 'ticket'


def dependency_checks(mode):
    if mode not in MODES:
        raise ValueError('unknown advisory mode: ' + mode)
    if mode != 'ticket':
        return ['advisories', 'bans', 'licenses'], None
    base = subprocess.check_output(
        ['git', 'merge-base', 'HEAD', 'refs/remotes/origin/main'], cwd=ROOT, text=True).strip()
    changed = subprocess.check_output(
        ['git', 'diff', '--name-only', '-z', base, '--', 'Cargo.lock', 'Cargo.toml',
         ':(glob)**/Cargo.lock', ':(glob)**/Cargo.toml'], cwd=ROOT)
    return (['advisories', 'bans', 'licenses'], None) if changed else (['bans', 'licenses'], base)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest-path')
    parser.add_argument('--config')
    parser.add_argument('--sources', action='store_true')
    args = parser.parse_args()
    try:
        checks, skipped = dependency_checks(os.environ.get('THINKTHEN_ADVISORY_MODE', 'checkpoint'))
    except (ValueError, subprocess.CalledProcessError) as error:
        print('dependency check: ' + str(error), file=sys.stderr)
        return 2
    if skipped and args.manifest_path is None:
        print('lint: skip advisories; tracked Cargo locks and manifests unchanged since merge base ' + skipped,
              flush=True)
    command = ['cargo', 'deny', '--offline']
    if args.manifest_path:
        command += ['--manifest-path', args.manifest_path]
    command += ['check']
    if args.config:
        command += ['--config', args.config]
    return subprocess.run([*command, *checks, *(['sources'] if args.sources else [])], cwd=ROOT).returncode


if __name__ == '__main__':
    sys.exit(main())
