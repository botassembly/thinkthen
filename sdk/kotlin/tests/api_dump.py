#!/usr/bin/env python3
"""Compare the Kotlin library's public JVM interface with its saved dump.

Usage: api_dump.py JAVAP JAR DUMP [--update-api]

The dump lists every public class and member that compiled callers can link
against, including inline functions' @PublishedApi entry points and the
`$default` bridges for default arguments. Kotlin-internal code lives in the
`dev.thinkthen.kotlin.internal` package and in `$`-mangled member names, so
it stays out. Any difference fails, because a removed or changed line breaks
code compiled against an earlier release. With --update-api the new dump is
written and printed between markers instead.
"""
import difflib
from pathlib import Path
import re
import subprocess
import sys
import zipfile

PACKAGE = 'dev/thinkthen/kotlin/'


def public_class(entry):
    """Keep named public classes; skip internals, lambdas and generated serializers."""
    if not entry.startswith(PACKAGE) or not entry.endswith('.class'):
        return False
    name = entry[len(PACKAGE):-len('.class')]
    if '/' in name:
        return False
    nested = name.split('$')[1:]
    return all(part[:1].isupper() for part in nested)


def member_name(line):
    match = re.search(r'([\w$<>]+)\(', line)
    if match:
        return match.group(1)
    return line.rstrip(';').split()[-1]


def dump(javap, jar):
    with zipfile.ZipFile(jar) as archive:
        names = sorted(entry[:-len('.class')].replace('/', '.') for entry in archive.namelist() if public_class(entry))
    output = subprocess.run([javap, '-public', '-cp', jar, *names], check=True, text=True, capture_output=True).stdout
    blocks, header, members = [], None, []
    for line in output.splitlines():
        if line.startswith('Compiled from'):
            continue
        if header is None:
            if line.endswith('{'):
                header, members = line, []
            continue
        if line == '}':
            if header.startswith('public '):
                blocks.append('\n'.join([header] + sorted(members) + ['}']))
            header = None
            continue
        name = member_name(line.strip())
        if '$' in name and not name.endswith('$default'):
            continue
        members.append(line)
    return '\n\n'.join(sorted(blocks)) + '\n'


def main():
    javap, jar, saved = sys.argv[1:4]
    update = sys.argv[4:] == ['--update-api']
    actual = dump(javap, jar)
    path = Path(saved)
    if update:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(actual)
        print('BEGIN_KOTLIN_API\n' + actual + 'END_KOTLIN_API')
        return
    expected = path.read_text() if path.exists() else ''
    if actual != expected:
        sys.stdout.writelines(difflib.unified_diff(expected.splitlines(True), actual.splitlines(True), str(path), 'built jar'))
        raise SystemExit('Kotlin public interface changed; review the difference, then rerun with --update-api and commit the dump')
    print('Kotlin public interface matches ' + saved)


main()
