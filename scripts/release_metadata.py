"""Check package manifests against the Cargo workspace release version."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile
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
    source_versions(root, expected)


def source_versions(root, expected):
    from runtime_floors import check_sources
    check_sources(root)
    for name in ('extensions/postgres/Makefile', 'extensions/postgres/package.sh'):
        versions = re.findall(r'thinkthen--([0-9.]+)\.sql', (root / name).read_text())
        if not versions or set(versions) != {expected}:
            raise ValueError(name + ': PostgreSQL SQL version differs from Cargo.toml')
    for name in ('sdk/c/CMakeLists.txt', 'sdk/cpp/CMakeLists.txt', 'databases/duckdb/CMakeLists.txt'):
        text = (root / name).read_text()
        declaration = re.search(r'(?im)^project\(([^\n]+)\)', text)
        version_field = re.search(r'\bVERSION\s+("[^"]+"|\S+)', declaration[1])
        # DuckDB's host supplies its extension version; it declares no project version.
        if version_field is None:
            if name.startswith('sdk/'): raise ValueError(name + ': missing package version')
            continue
        # Let CMake evaluate the real version derivation, including Cargo.toml reads.
        with tempfile.TemporaryDirectory(prefix='thinkthen-cmake-version-') as temporary:
            script = Path(temporary) / 'version.cmake'
            script.write_text('set(CMAKE_CURRENT_SOURCE_DIR "${PACKAGE_SOURCE_DIR}")\n' +
                              text[:declaration.start()] + '\nfile(WRITE "${OUTPUT}" ' + version_field[1] + ')\n')
            output = Path(temporary) / 'version'
            subprocess.run(['cmake', '-DPACKAGE_SOURCE_DIR=' + str((root / name).parent),
                            '-DOUTPUT=' + str(output), '-P', str(script)], check=True, capture_output=True)
            if output.read_text() != expected:
                raise ValueError(name + ': CMake package version differs from Cargo.toml')
    name = 'scripts/sqlite-package'
    sqlite = re.search(r'sqlite="\$THINKTHEN_SQLITE_ROOT/sqlite-([0-9.]+)"', (root / name).read_text())
    prepared = re.search(r"for release in '([0-9.]+) ", (root / 'scripts/sqlite-prepare').read_text())
    if sqlite is None or sqlite[1] != prepared[1]:
        raise ValueError(name + ': SQLite package does not use the prepared runtime floor')
    name = 'databases/duckdb/community-versions.json'
    versions = json.loads((root / name).read_text())
    selected = re.search(r'/duckdb-([0-9.]+)', (root / 'databases/duckdb/build.sh').read_text())[1]
    if selected not in versions:
        raise ValueError(name + ': DuckDB package runtime has no community revision')
    for release, pins in versions.items():
        if not re.fullmatch(r'[0-9]+\.[0-9]+\.[0-9]+', release) or set(pins) != {'duckdb', 'extension-ci-tools'}:
            raise ValueError(name + ': invalid DuckDB version declaration')
        if any(not re.fullmatch(r'[0-9a-f]{40}', pin) for pin in pins.values()):
            raise ValueError(name + ': invalid upstream revision')
    indexed = subprocess.check_output(['git', 'ls-files', '--stage', 'duckdb', 'extension-ci-tools'], cwd=root, text=True)
    pins = {line.split('\t')[1]: line.split()[1] for line in indexed.splitlines()}
    if pins != versions[selected]:
        raise ValueError(name + ': DuckDB runtime revisions differ from source gitlinks')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--version', action='store_true')
    arguments = parser.parse_args()
    if arguments.version:
        print(version())
    else:
        check()
        print('release: package versions agree with Cargo.toml')
