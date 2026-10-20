#!/usr/bin/env python3
"""Check feature folders against the source modules of their Cargo consumer."""
from pathlib import Path
import re
import sys
import tomllib

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'build-tools/results'))
from sources import tracked_files


def module_source(source):
    """Mask Rust comments and strings, retaining only path attribute values."""
    output = list(source)
    position = 0
    opening = re.compile(r"//|/\*|(?:br|r)(#*)\"|b?'(?:\\.|[^'\\\n])'|\"")
    while match := opening.search(source, position):
        start = match.start()
        position = match.end()
        token = match[0]
        if token == '//':
            end = source.find('\n', position)
            position = len(source) if end < 0 else end
        elif token == '/*':
            depth = 1
            while depth and position < len(source):
                nested = re.search(r'/\*|\*/', source[position:])
                if nested is None:
                    position = len(source)
                    break
                position += nested.end()
                depth += 1 if nested[0] == '/*' else -1
        elif token.endswith("'"):
            pass
        elif token != '"':
            closing = '"' + match[1]
            end = source.find(closing, position)
            position = len(source) if end < 0 else end + len(closing)
        else:
            while position < len(source):
                character = source[position]
                position += 1
                if character == '\\':
                    position += 1
                elif character == '"':
                    break
            prefix = ''.join(output[:start])
            if re.search(r'#\[path\s*=\s*$', prefix):
                continue
        output[start:position] = ['\n' if char == '\n' else ' '
                                  for char in source[start:position]]
    return ''.join(output)


def inventory(root, surface, tracked):
    base = root / surface
    files = {base / 'README.md', base / 'check.sh'}
    consumer = base / 'consumer'
    if not any(path.is_relative_to(consumer) for path in tracked):
        return files
    manifest = consumer / 'Cargo.toml'
    if manifest not in tracked:
        raise ValueError(f'feature consumer manifest is not tracked: {manifest}')
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
        if source not in tracked:
            raise ValueError(f'feature consumer source is not tracked: {source}')
        text = module_source(source.read_text())
        files.add(source)
        for match in re.finditer(r'(?m)^[ \t]*(?:#\[path\s*=\s*"([^"]+)"\]\s*)?(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', text):
            if match[1]:
                module = source.parent / match[1]
            else:
                directory = source.parent if source.name in ('main.rs', 'lib.rs', 'mod.rs') else source.with_suffix('')
                module = directory / (match[2] + '.rs')
                if module not in tracked:
                    module = directory / match[2] / 'mod.rs'
            pending.append(module)
    return {path.resolve() for path in files}


def check(root, surface):
    root = root.resolve()
    tracked = set(tracked_files(root))
    expected = inventory(root, surface, tracked)
    actual = {path for path in tracked if path.is_relative_to(root / surface) and path.is_file()}
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
