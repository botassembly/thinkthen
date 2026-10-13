#!/usr/bin/env python3
"""Check public package text against its reviewed source without contacting registries."""
import argparse
import email
import gzip
import io
import json
from pathlib import Path
import re
import tarfile
import tomllib
import xml.etree.ElementTree as ET
import zipfile

REPO = Path(__file__).resolve().parents[2]


class Refusal(ValueError):
    pass


def version(root=REPO):
    return tomllib.loads((root / 'crates/thinkthen/Cargo.toml').read_text())['package']['version']


def source(family, root=REPO):
    if family == 'crate':
        return root / 'crates/thinkthen/README-PUBLIC.md'
    if family in ('sqlite', 'postgresql', 'duckdb'):
        return root / 'databases' / family / 'README-PUBLIC.md'
    if family == 'flutter':
        return root / 'libraries/dart/flutter/README-PUBLIC.md'
    return root / 'libraries' / family / 'README-PUBLIC.md'


def check_text(data, release):
    text = data.decode('utf-8')
    rules = {
        'adr': r'\bADR[\s#:-]*\d+\b',
        'ticket': r'\bticket[\s#:-]*\d+\b',
        'internal-path': r'\bsdlc[/\\]',
        'internal-crate': r'\bthinkthen-(?:python|node|ruby|c|postgresql|sqlite|duckdb)\b(?!-\d)',
    }
    for reason, pattern in rules.items():
        if re.search(pattern, text, re.I):
            raise Refusal(reason)
    # Versioned ThinkThen artifacts, release URLs, install pins and explicit product claims.
    # Other packages' compiler, Rust, DuckDB and language-runtime versions are unrestricted.
    patterns = [
        r'\bThinkThen(?:\s+(?:version|release))?\s+v?(\d+\.\d+(?:\.\d+)?)',
        r'thinkthen(?:[-_][\w.]+)?(?:[-_@= ]+|\s+-v\s+)v?(\d+\.\d+\.\d+)',
        r'releases/download/v(\d+\.\d+\.\d+)',
        r'botassembly:thinkthen-jvm:(\d+\.\d+\.\d+)',
    ]
    for pattern in patterns:
        for match in re.finditer(pattern, text, re.I):
            found = match.group(1)
            if found != release and found != '.'.join(release.split('.')[:2]):
                raise Refusal('stale-version')


def check_readme(family, data, root=REPO):
    check_text(data, version(root))
    expected = source(family, root).read_bytes()
    if data != expected:
        raise Refusal('readme-source-mismatch: ' + family)


def members(data, zipped=False):
    if zipped:
        with zipfile.ZipFile(io.BytesIO(data)) as archive:
            return {name: archive.read(name) for name in archive.namelist() if not name.endswith('/')}
    with tarfile.open(fileobj=io.BytesIO(data), mode='r:*') as archive:
        return {entry.name.removeprefix('./'): archive.extractfile(entry).read()
                for entry in archive if entry.isfile()}


def unique_suffix(files, suffix):
    matches = [value for name, value in files.items() if name == suffix or name.endswith('/' + suffix)]
    if len(matches) != 1:
        raise Refusal('missing-or-repeated-member: ' + suffix)
    return matches[0]


def description(family, root=REPO):
    if family in ('python', 'crate'):
        path = 'libraries/python/pyproject.toml' if family == 'python' else 'crates/thinkthen/Cargo.toml'
        key = 'project' if family == 'python' else 'package'
        return tomllib.loads((root / path).read_text())[key]['description']
    if family in ('typescript', 'php'):
        path = 'libraries/typescript/package.json' if family == 'typescript' else 'libraries/php/composer.json'
        return json.loads((root / path).read_text())['description']
    if family in ('jvm', 'csharp'):
        path = 'libraries/jvm/pom.xml' if family == 'jvm' else 'libraries/csharp/Botassembly.ThinkThen.nuspec'
        document = ET.parse(root / path)
        return next(node.text.strip() for node in document.iter() if node.tag.rsplit('}', 1)[-1] == 'description')
    if family == 'dart':
        return re.search(r'(?m)^description: (.+)$', (root / 'libraries/dart/pubspec.yaml').read_text()).group(1)
    if family == 'ruby':
        return re.search(r'spec.description = "([^"]+)"', (root / 'libraries/ruby/thinkthen.gemspec').read_text()).group(1)
    if family == 'r':
        text = (root / 'libraries/r/thinkthen/DESCRIPTION').read_text()
        return re.search(r'(?m)^Description: (.*(?:\n[ \t]+[^\n]*)*)', text).group(1)
    return None


def check_description(family, text, root=REPO):
    check_text(text.encode(), version(root))
    if text != description(family, root):
        raise Refusal('description-source-mismatch: ' + family)


def check_package(family, path, root=REPO):
    files = members(path.read_bytes(), path.suffix in ('.whl', '.nupkg', '.zip'))
    if family == 'python':
        metadata = email.message_from_bytes(unique_suffix(files, 'METADATA'))
        check_readme(family, metadata.get_payload().encode(), root)
        check_description(family, metadata['Summary'], root)
    elif family == 'ruby':
        package = members(files['data.tar.gz'])
        check_readme(family, unique_suffix(package, 'README.md'), root)
        metadata = gzip.decompress(files['metadata.gz']).decode()
        # Gem emits description as a YAML scalar, potentially folded by its serializer.
        text = re.search(r'(?m)^description: (.*(?:\n  [^\n]*)*)', metadata).group(1)
        check_description(family, ' '.join(text.split()), root)
    elif family == 'csharp' and path.suffix != '.nupkg':
        check_readme(family, unique_suffix(files, 'README.md'), root)
        nested = [name for name in files if name.endswith('.nupkg')]
        if len(nested) != 1:
            raise Refusal('missing-or-repeated-nupkg')
        check_nupkg(files[nested[0]], root)
    else:
        check_readme(family, unique_suffix(files, 'README.md'), root)
        if family == 'typescript':
            check_description(family, json.loads(unique_suffix(files, 'package.json'))['description'], root)
        elif family == 'crate':
            check_description(family, tomllib.loads(unique_suffix(files, 'Cargo.toml').decode())['package']['description'], root)
        elif family == 'php':
            check_description(family, json.loads(unique_suffix(files, 'composer.json'))['description'], root)
        elif family == 'jvm':
            document = ET.fromstring(unique_suffix(files, 'pom.xml'))
            check_description(family, next(node.text.strip() for node in document.iter() if node.tag.rsplit('}', 1)[-1] == 'description'), root)
        elif family == 'dart':
            text = unique_suffix(files, 'pubspec.yaml').decode()
            check_description(family, re.search(r'(?m)^description: (.+)$', text).group(1), root)
        elif family == 'r':
            text = unique_suffix(files, 'DESCRIPTION').decode()
            check_description(family, re.search(r'(?m)^Description: (.*(?:\n[ \t]+[^\n]*)*)', text).group(1), root)
        elif family == 'csharp':
            check_nupkg(path.read_bytes(), root)


def check_nupkg(data, root):
    files = members(data, True)
    check_readme('csharp', unique_suffix(files, 'README.md'), root)
    document = ET.fromstring(unique_suffix(files, 'Botassembly.ThinkThen.nuspec'))
    text = next(node.text.strip() for node in document.iter() if node.tag.rsplit('}', 1)[-1] == 'description')
    check_description('csharp', text, root)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('family')
    parser.add_argument('packages', type=Path, nargs='+')
    args = parser.parse_args()
    for package in args.packages:
        check_package(args.family, package)
        print('registry-readmes: checked ' + package.name)


if __name__ == '__main__':
    try:
        main()
    except (Refusal, OSError, KeyError, StopIteration, AttributeError) as error:
        raise SystemExit('registry-readmes: ' + str(error))
