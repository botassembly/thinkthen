#!/usr/bin/env python3
"""Require a KDoc comment on every public Kotlin declaration.

Usage: kdoc_check.py SOURCE_DIRECTORY

Explicit API mode makes every public declaration start with `public`, so a
declaration line is one whose first word after its annotations is `public`.
The nearest earlier line that is not an annotation must close a KDoc comment,
or the comment must sit on the same line. Overrides inherit their docs.
"""
from pathlib import Path
import re
import sys

ANNOTATION = re.compile(r'^@[\w.]+(\(.*\))?$')
DECLARATION = re.compile(r'^(?:/\*\*.*\*/\s*)?(?:@[\w.]+(?:\([^)]*\))?\s+)*public\s')

missing = []
for path in sorted(Path(sys.argv[1]).rglob('*.kt')):
    lines = path.read_text().splitlines()
    for number, line in enumerate(lines):
        text = line.strip()
        if not DECLARATION.match(text):
            continue
        if text.startswith('/**'):
            continue
        previous = number - 1
        while previous >= 0 and ANNOTATION.match(lines[previous].strip()):
            previous -= 1
        if previous < 0 or not lines[previous].strip().endswith('*/'):
            missing.append(f'{path}:{number + 1}: {text[:100]}')
if missing:
    print('\n'.join(missing))
    raise SystemExit(f'{len(missing)} public Kotlin declarations have no KDoc')
print('Every public Kotlin declaration has KDoc')
