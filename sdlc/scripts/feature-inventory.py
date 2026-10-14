#!/usr/bin/env python3
"""Check feature folders against the source modules of their Cargo consumer."""
from pathlib import Path
import re
import subprocess
import sys
import tomllib


def inventory(root, surface):
    base = root / surface
    files = {base / 'README.md', base / 'check.sh'}
    consumer = base / 'consumer'
    if not consumer.exists():
        return files
    manifest = consumer / 'Cargo.toml'
    definition = tomllib.loads(manifest.read_text())
    if not definition.get('package', {}).get('publish') is False:
        raise ValueError('feature consumer must be unpublished')
    files.update((manifest, consumer / 'Cargo.lock'))
    pending = [consumer / target.get('path', 'src/main.rs')
               for target in definition.get('bin', [{}])]
    while pending:
        source = pending.pop().resolve()
        if source in files:
            continue
        text = source.read_text()
        files.add(source)
        for match in re.finditer(r'(?:#\[path\s*=\s*"([^"]+)"\]\s*)?\bmod\s+(\w+)\s*;', text):
            if match[1]:
                module = source.parent / match[1]
            else:
                directory = source.parent if source.name in ('main.rs', 'lib.rs', 'mod.rs') else source.with_suffix('')
                module = directory / (match[2] + '.rs')
                if not module.exists():
                    module = directory / match[2] / 'mod.rs'
            pending.append(module)
    return {path.resolve() for path in files}


def check(root, surface):
    expected = inventory(root, surface)
    names = subprocess.check_output(['git', 'ls-files', '-co', '--exclude-standard', '--', surface], cwd=root, text=True)
    actual = {(root / name).resolve() for name in names.splitlines()}
    extra, missing = actual - expected, expected - actual
    # A consumer can share a fixture from another installed library.
    missing = {path for path in missing if path.is_relative_to(root / surface)}
    if extra or missing:
        raise ValueError(f'{surface} inventory differs: missing={sorted(map(str, missing))}, extra={sorted(map(str, extra))}')


if __name__ == '__main__':
    try:
        check(Path.cwd().resolve(), sys.argv[1])
    except (ValueError, OSError) as error:
        print(f'surfaces: {error}', file=sys.stderr)
        sys.exit(1)
