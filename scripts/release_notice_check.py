"""Verify license inventories inside delivered packages without the source tree."""
import email
import hashlib
import io
import json
from pathlib import Path
import tarfile
import zipfile

NOTICE = 'THIRD-PARTY-NOTICES'


def declared_notices(files):
    """Read the notice paths promised by every delivered dependency index."""
    expected = set()
    for name, data in files.items():
        if NOTICE not in name.split('/') or not name.endswith('/index.json'): continue
        expected.add(name)
        prefix = name.removesuffix('index.json')
        index = json.loads(data)
        for group in ('crates', 'components'):
            for component, entry in index.get(group, {}).items():
                expected.update(prefix + component + '/' + relative for relative in entry['files'])
    if not expected: raise ValueError('package omits dependency notice inventory')
    return expected


def contents(path):
    if zipfile.is_zipfile(path):
        with zipfile.ZipFile(path) as archive:
            return {name: archive.read(name) for name in archive.namelist() if not name.endswith('/')}
    with tarfile.open(path) as archive:
        if Path(path).suffix == '.gem':
            raw = archive.extractfile('data.tar.gz').read()
            with tarfile.open(fileobj=io.BytesIO(raw)) as data:
                return {p.name.removeprefix('./'): data.extractfile(p).read() for p in data if p.isfile()}
        return {p.name.removeprefix('./'): archive.extractfile(p).read() for p in archive if p.isfile()}


def verify(path, expected=None):
    files = contents(path)
    indices = [name for name in files if NOTICE in name.split('/') and name.endswith('/index.json')]
    if not indices: raise ValueError('package omits dependency notices: ' + str(path))
    matched = False
    for name in indices:
        index = json.loads(files[name])
        if expected is not None and 'crates' in index and index != expected:
            raise ValueError('package notices differ from target dependencies: ' + str(path))
        matched |= index == expected
        prefix = name.removesuffix('index.json')
        for crate, entry in (index.get('crates') or index['components']).items():
            for relative, digest in entry['files'].items():
                if hashlib.sha256(files[prefix + crate + '/' + relative]).hexdigest() != digest:
                    raise ValueError('packaged license differs: ' + crate + '/' + relative)
    if expected is not None and not matched: raise ValueError('package omits target crate inventory')
    if Path(path).suffix == '.whl':
        metadata_path = next(p for p in files if p.endswith('.dist-info/METADATA'))
        metadata = email.message_from_bytes(files[metadata_path])
        index = json.loads(files[indices[0]])
        if metadata['License-Expression'] != index['license_expression']:
            raise ValueError('wheel license expression differs from bundled dependencies')
        prefix = metadata_path.removesuffix('METADATA') + 'licenses/'
        listed = {prefix + p for p in metadata.get_all('License-File', [])}
        if listed != {p for p in files if p.startswith(prefix)}:
            raise ValueError('wheel License-File metadata omits bundled terms')


def candidate(folder):
    """Compare final package inventories with the currently resolved target graphs."""
    from release_notices import bundle
    from release_notice_packages import package_files, runtime_files, native_notices
    target = json.loads((folder / 'build.json').read_text())['platform']
    produced = json.loads((folder / 'compiled-crates.json').read_text())
    if produced['target'] != target: raise ValueError('compiled target differs')
    for family, path in package_files(folder, target.startswith('windows')):
        expected = json.loads(bundle(target, family, compiled=produced['crates'])[NOTICE + '/index.json'])
        verify(path, expected)
        files = contents(path)
        prefix = '' if family == 'sqlite' or target.startswith('windows') else 'share/thinkthen-' + family + '/'
        for name, data in native_notices(family, target).items():
            if files.get(prefix + name) != data:
                raise ValueError('package native notices differ: ' + str(path))
    from release_jvm_notices import check as jvm_notices
    jvm_notices(folder)
    from release_r_dependencies import directories, terms, metadata
    from release_notices import ROOT
    for directory in directories(folder):
        inventory = json.loads((directory / 'inventory.json').read_text())
        for filename in inventory['install_order']:
            path = directory / filename
            verify(path)
            files = contents(path)
            for name, data in terms(path, ROOT).items():
                if files[metadata(path)['Package'] + '/' + name] != data:
                    raise ValueError('R dependency terms differ: ' + filename)
    if (folder / 'prerequisites/swift-runtime').is_dir():
        for name, data in runtime_files('swift-icu').items():
            if (folder / 'prerequisites/swift-runtime' / name).read_bytes() != data:
                raise ValueError('Swift ICU runtime terms differ')
    print('all package and runtime notices match: ' + target)


def package_notices(path):
    """Reuse delivered terms when registry assembly reuses the delivered engine."""
    files = contents(path)
    selected = {}
    for name, data in files.items():
        parts = name.split('/')
        if NOTICE in parts:
            selected['/'.join(parts[parts.index(NOTICE):])] = data
        elif parts[-1] == 'LICENSE': selected['LICENSE'] = data
    if not selected or 'LICENSE' not in selected: raise ValueError('missing delivered package terms')
    verify(path)
    return selected
