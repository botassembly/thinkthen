#!/usr/bin/env python3
"""Derive shipping files from package definitions and the release target matrix.

Compiled JVM members come from compiler output. Installed public consumers remain
independent: matching this inventory alone does not establish a usable package.
"""
import argparse
import fnmatch
import json
from pathlib import Path
import re
import sys
import xml.etree.ElementTree as ET

REPO = Path(__file__).resolve().parents[2]


def targets():
    matrix = (REPO / '.github/workflows/release.yml').read_text().split('  build:', 1)[1].split('    defaults:', 1)[0]
    values = re.findall(r'^\s+- \{target: ([\w-]+), runner:', matrix, re.M)
    if not values or len(values) != len(set(values)):
        raise ValueError('release target matrix is empty or repeated')
    result = []
    for target in values:
        cpu = target.split('-')[0]
        arch = {'x86_64': 'x64', 'aarch64': 'arm64'}[cpu]
        if '-linux-' in target:
            platform, native = 'linux', ['libthinkthen.so', 'libthinkthen.so.0']
        elif target.endswith('-apple-darwin'):
            platform, native = 'darwin', ['libthinkthen.dylib', 'libthinkthen.0.dylib']
        elif '-windows-' in target:
            platform, native = 'win32', ['thinkthen.dll']
        else:
            raise ValueError(f'unknown native platform: {target}')
        result.append(dict(target=target, platform=platform, arch=arch, libraries=native))
    return result


def npm_definition():
    return json.loads((REPO / 'libraries/typescript/package.json').read_text())


def npm_inventory(target=None):
    definition = npm_definition()
    assert definition['thinkthen']['nativeTargets'] == 'release'
    template = definition['thinkthen']['nativeFile']
    assets = {entry['target']: template.format(**entry) for entry in targets()}
    if target is not None and target not in assets:
        raise ValueError(f'unsupported npm target: {target}')
    fixed = [name for name in definition['files'] if '*' not in name]
    patterns = [name for name in definition['files'] if '*' in name]
    assert patterns == ['thinkthen-*.node'], 'unexpected npm file pattern'
    files = sorted(set(fixed + ['package.json', 'README.md'] +
                       ([assets[target]] if target else list(assets.values()))))
    return dict(files=files, native=assets)


def jvm_definition(pom=None):
    project = ET.fromstring(pom or (REPO / 'libraries/jvm/pom.xml').read_bytes())
    props = project.find('{*}properties')
    return {entry.tag.rsplit('}', 1)[-1]: entry.text.strip() for entry in props}


def jvm_inventory(out=None, pom=None):
    props = jvm_definition(pom)
    assert props['thinkthen.native.targets'] == 'release'
    jars = {kind: f'thinkthen-{kind}.jar' for kind in props['thinkthen.packages'].split()}
    native = {}
    for entry in targets():
        maven = dict(entry, platform={'darwin': 'osx', 'win32': 'win'}.get(entry['platform'], entry['platform']))
        classifier = props['thinkthen.native.classifier'].format(**maven)
        prefix = props['thinkthen.native.resource'].format(**entry)
        native[classifier] = dict(target=entry['target'], files=[prefix + name for name in entry['libraries']])
    result = dict(jars=jars, files=sorted(props['thinkthen.package.files'].split() + list(jars.values())), native=native)
    if out:
        result['members'] = {kind: sorted(path.relative_to(Path(out) / 'classes' / kind).as_posix()
                                         for path in (Path(out) / 'classes' / kind).rglob('*') if path.is_file() and not any(fnmatch.fnmatch(path.name, pattern) for pattern in props['thinkthen.package.exclude'].split()))
                             for kind in jars}
    return result


def check_npm(package, target=None):
    inventory = npm_inventory(target)
    expected = inventory['files']
    if target:
        expected = sorted(set(expected) | {name for name in inventory['native'].values() if (package / name).is_file()})
    actual = sorted(path.relative_to(package).as_posix() for path in package.rglob('*') if path.is_file())
    if actual != expected:
        raise ValueError(f'npm inventory differs: missing={set(expected)-set(actual)}, extra={set(actual)-set(expected)}')
    for path in expected:
        if not (package / path).stat().st_size:
            raise ValueError(f'empty npm product: {path}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('package', choices=['npm', 'jvm'])
    parser.add_argument('--target')
    parser.add_argument('--platform')
    parser.add_argument('--out', type=Path)
    parser.add_argument('--field')
    parser.add_argument('--kind')
    parser.add_argument('--write', action='store_true')
    parser.add_argument('--check', type=Path)
    args = parser.parse_args()
    if args.package == 'npm':
        if args.platform:
            args.target = next(entry['target'] for entry in targets() if entry['platform'] + '-' + entry['arch'] == args.platform)
        inventory = npm_inventory(args.target)
        generated = json.dumps({entry['platform'] + '-' + entry['arch']: inventory['native'][entry['target']]
                                for entry in targets()}, indent=2) + '\n'
        platform_file = REPO / 'libraries/typescript/native-platforms.json'
        if args.write:
            platform_file.write_text(generated)
        elif platform_file.read_text() != generated:
            raise ValueError('stale npm native-platforms.json; run package-inventory.py npm --write')
        if args.check:
            check_npm(args.check, args.target)
    else:
        inventory = jvm_inventory(args.out)
    if args.field:
        value = inventory[args.field]
        if args.kind:
            value = value[args.kind]
        print('\n'.join(value if isinstance(value, list) else value.values()))
    else:
        print(json.dumps(inventory, indent=2))


if __name__ == '__main__':
    main()
