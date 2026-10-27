"""Check package manifests against the Cargo workspace release version."""
import argparse
import json
from pathlib import Path
import re
import tomllib
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parent.parent


def version(root=ROOT):
    return tomllib.loads((root / 'Cargo.toml').read_text())['workspace']['package']['version']


def source_tags(tag):
    """Return the root and nested Go module tags for one release."""
    return tag, 'sdk/go/thinkthen/' + tag


def check(root=ROOT):
    expected = version(root)

    def equal(path, actual):
        if actual != expected:
            raise ValueError(f'{path}: package version {actual!r} differs from Cargo.toml {expected}')

    workspace = tomllib.loads((root / 'Cargo.toml').read_text())['workspace']
    for member in workspace['members']:
        path = root / member / 'Cargo.toml'
        data = tomllib.loads(path.read_text())
        declared = data['package']['version']
        equal(path.relative_to(root), expected if declared == {'workspace': True} else declared)
        for section in ('dependencies', 'build-dependencies'):
            for name, dependency in data.get(section, {}).items():
                if isinstance(dependency, dict) and dependency.get('workspace'):
                    dependency = workspace['dependencies'][name]
                if isinstance(dependency, dict) and 'path' in dependency:
                    if data['package'].get('publish') is not False or 'version' in dependency:
                        equal(f'{path.relative_to(root)} dependency {name}', dependency.get('version'))
    path = root / 'sdk/python/pyproject.toml'
    equal(path.relative_to(root), tomllib.loads(path.read_text())['project']['version'])
    for name in ('package.json', 'package-lock.json'):
        path = root / 'sdk/typescript' / name
        data = json.loads(path.read_text())
        equal(path.relative_to(root), data['version'])
        if name == 'package-lock.json':
            equal(str(path.relative_to(root)) + ' root package', data['packages']['']['version'])
    for path in (root / 'sdk').rglob('pom.xml'):
        if 'target' in path.parts:
            continue
        for element in ET.parse(path).iter():
            fields = {child.tag.rsplit('}', 1)[-1]: child.text for child in element}
            if fields.get('groupId') == 'io.github.botassembly' and 'version' in fields:
                equal(path.relative_to(root), expected if fields['version'] == '${project.version}' else fields['version'])
    for path in (root / 'sdk/csharp').rglob('*.csproj'):
        if any(part in ('obj', 'bin') for part in path.parts):
            continue
        for element in ET.parse(path).iter():
            if element.tag == 'Version':
                equal(path.relative_to(root), element.text)
            if element.tag == 'PackageReference' and element.get('Include') == 'Botassembly.ThinkThen':
                equal(path.relative_to(root), element.get('Version'))
    patterns = {
        'sdk/ruby/lib/thinkthen/version.rb': r'^\s*VERSION = "([^"]+)"',
        'sdk/r/DESCRIPTION': r'^Version: (.+)$',
        'sdk/zig/build.zig.zon': r'^\s*\.version = "([^"]+)"',
        'extensions/postgres/thinkthen.control': r"^default_version = '([^']+)'",
    }
    for name, pattern in patterns.items():
        found = re.search(pattern, (root / name).read_text(), re.MULTILINE)
        equal(name, found[1] if found else None)
    # CMake derives PROJECT_VERSION from the workspace manifest during configuration.
    c_template = (root / 'sdk/c/cmake/thinkthen-c.pc.in').read_text()
    if re.search(r'^Version: (.+)$', c_template, re.MULTILINE)[1] != '@PROJECT_VERSION@':
        raise ValueError('C pkg-config version must use its configured workspace version')
    if not (root / f'extensions/postgres/thinkthen--{expected}.sql').is_file():
        raise ValueError('PostgreSQL SQL filename differs from the release version')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--version', action='store_true')
    arguments = parser.parse_args()
    if arguments.version:
        print(version())
    else:
        check()
        print('release: package versions agree with Cargo.toml')
