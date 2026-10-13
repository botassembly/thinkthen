#!/usr/bin/env python3
"""Check packed public text against its authoritative source without contacting registries."""
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
OPENING = 'ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.'


class Refusal(ValueError):
    pass


def version(root=REPO):
    return tomllib.loads((root / 'crates/thinkthen/Cargo.toml').read_text())['package']['version']


def source(family, root=REPO):
    if family in ('command', 'packagist'):
        return root / 'README.md'
    if family == 'crate':
        return root / 'crates/thinkthen/README.md'
    if family in ('typescript', 'go', 'ruby'):
        return root / 'libraries' / family / 'README.md'
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
        'internal-project': r'\b(?:thinkthen[-_]core|thinkthen[-_]native|thinkthen[-_]conformance)\b',
        'internal-crate': r'\bthinkthen-(?:python|node|ruby|c|postgresql|sqlite|duckdb)\b(?!-\d)',
    }
    for reason, pattern in rules.items():
        if re.search(pattern, text, re.I):
            raise Refusal(reason)
    names = Path.home() / '.config/thinkthen/private-names.txt'
    if names.is_file():
        for name in names.read_text().splitlines():
            if name.strip() and re.search(r'(?<!\w)' + re.escape(name.strip()) + r'(?!\w)', text, re.I):
                raise Refusal('private-name')
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
    if len(data.decode().splitlines()) >= 40:
        raise Refusal('readme-line-limit')
    expected = source(family, root).read_bytes()
    if data != expected:
        raise Refusal('readme-source-mismatch: ' + family)
    if not data.decode().startswith('# ') or data.decode().split('\n\n')[1:2] != [OPENING]:
        raise Refusal('shared-opening: ' + family)


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
    if family == 'packagist':
        return json.loads((root / 'composer.json').read_text())['description']
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
        metadata = email.message_from_bytes(unique_suffix(files, 'METADATA') if path.suffix == '.whl' else unique_suffix(files, 'PKG-INFO'))
        check_readme(family, metadata.get_payload(decode=True), root)
        check_description(family, metadata['Summary'], root)
    elif family == 'ruby':
        package = members(files['data.tar.gz'])
        check_readme(family, unique_suffix(package, 'README.md'), root)
        metadata = gzip.decompress(files['metadata.gz']).decode()
        # Gem emits description as a YAML scalar, potentially folded by its serializer.
        text = re.search(r'(?m)^description: (.*(?:\n  [^\n]*)*)', metadata).group(1)
        text = ' '.join(text.split())
        if text.startswith("'") and text.endswith("'"):
            text = text[1:-1].replace("''", "'")
        elif text.startswith('"'):
            text = json.loads(text)
        check_description(family, text, root)
    elif family == 'csharp' and path.suffix != '.nupkg':
        check_readme(family, unique_suffix(files, 'README.md'), root)
        nested = [name for name in files if name.endswith('.nupkg')]
        if len(nested) != 1:
            raise Refusal('missing-or-repeated-nupkg')
        check_nupkg(files[nested[0]], root)
    else:
        prefix = {'crate': 'thinkthen-' + version(root) + '/', 'r': 'thinkthen/', 'typescript': 'package/', 'flutter': 'flutter/'}.get(family, '')
        name = prefix + 'README.md'
        if name not in files:
            raise Refusal('missing-or-repeated-member: ' + name)
        check_readme(family, files[name], root)
        if family == 'typescript':
            check_description(family, json.loads(unique_suffix(files, 'package.json'))['description'], root)
        elif family == 'crate':
            check_description(family, tomllib.loads(files[prefix + 'Cargo.toml'].decode())['package']['description'], root)
        elif family == 'php':
            check_description(family, json.loads(unique_suffix(files, 'composer.json'))['description'], root)
        elif family == 'jvm':
            document = ET.fromstring(unique_suffix(files, 'pom.xml'))
            check_description(family, next(node.text.strip() for node in document.iter() if node.tag.rsplit('}', 1)[-1] == 'description'), root)
        elif family == 'dart':
            text = unique_suffix(files, 'pubspec.yaml').decode()
            check_description(family, re.search(r'(?m)^description: (.+)$', text).group(1), root)
        elif family == 'r':
            text = files[prefix + 'DESCRIPTION'].decode()
            check_description(family, re.search(r'(?m)^Description: (.*(?:\n[ \t]+[^\n]*)*)', text).group(1), root)
        elif family == 'csharp':
            check_nupkg(path.read_bytes(), root)


def check_nupkg(data, root):
    files = members(data, True)
    check_readme('csharp', unique_suffix(files, 'README.md'), root)
    document = ET.fromstring(unique_suffix(files, 'Botassembly.ThinkThen.nuspec'))
    text = next(node.text.strip() for node in document.iter() if node.tag.rsplit('}', 1)[-1] == 'description')
    check_description('csharp', text, root)


PUBLIC_FAMILIES = ('command', 'crate', 'python', 'typescript', 'ruby', 'go', 'cpp',
                   'c', 'csharp', 'jvm', 'swift', 'zig', 'php', 'dart', 'flutter',
                   'ada', 'objective-c', 'cobol', 'r', 'sqlite', 'postgresql', 'duckdb', 'packagist')


def check_sources(root=REPO):
    for family in PUBLIC_FAMILIES:
        check_readme(family, source(family, root).read_bytes(), root)
        text = description(family, root)
        if text is not None:
            check_description(family, text, root)


def archive_family(path):
    name = path.name
    if name.endswith('.crate'): return 'crate'
    if name.endswith('.whl'): return 'python'
    if name.endswith('.gem'): return 'ruby'
    if name.endswith('.nupkg'): return 'csharp'
    if name.startswith('thinkthen_'): return 'r'
    for family in PUBLIC_FAMILIES:
        if name.startswith('thinkthen-' + family + '-') or (family == 'postgresql' and name.startswith('thinkthen-postgresql16-')):
            return family
    if name.endswith('.tgz'): return 'typescript'
    if re.match(r'thinkthen-\d', name): return 'command'
    raise Refusal('unknown-package-family')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('family', choices=PUBLIC_FAMILIES + ('auto', 'sources'))
    parser.add_argument('packages', type=Path, nargs='*')
    args = parser.parse_args()
    if args.family == 'sources':
        check_sources()
        print('registry-readmes: checked public sources (drafts; no approval implied)')
        return
    if not args.packages:
        parser.error('package paths are required')
    for package in args.packages:
        family = archive_family(package) if args.family == 'auto' else args.family
        check_package(family, package)
        print('registry-readmes: checked ' + package.name)


if __name__ == '__main__':
    try:
        main()
    except (Refusal, OSError, KeyError, StopIteration, AttributeError) as error:
        raise SystemExit('registry-readmes: ' + str(error))
