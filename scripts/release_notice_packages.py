"""Insert notices into the final independently installable package files."""
import base64
import gzip
import hashlib
import io
import json
from pathlib import Path
import re
import tarfile
import zipfile

from release_notices import ROOT, NOTICE, bundle
from release_readme import tar_bytes


def zip_update(path, updates, replace_prefix=None):
    with zipfile.ZipFile(path) as source:
        infos = source.infolist()
        files = {p.filename: source.read(p) for p in infos}
    if replace_prefix:
        infos = [p for p in infos if not p.filename.startswith(replace_prefix)]
        files = {name: data for name, data in files.items() if not name.startswith(replace_prefix)}
    files.update(updates)
    if path.suffix == '.whl':
        record = next(p for p in files if p.endswith('.dist-info/RECORD'))
        files[record] = ''.join(f'{p},sha256={base64.urlsafe_b64encode(hashlib.sha256(data).digest()).decode().rstrip("=")},{len(data)}\n'
                               for p, data in sorted(files.items()) if p != record).encode() + f'{record},,\n'.encode()
    if any(p.endswith('/MD5') for p in files):
        table = next(p for p in files if p.endswith('/MD5'))
        prefix = table.removesuffix('MD5')
        files[table] = ''.join(f'{hashlib.md5(data).hexdigest()}  {p[len(prefix):]}\n'
                               for p, data in sorted(files.items()) if p.startswith(prefix) and p != table).encode()
    temporary = path.with_suffix('.partial')
    with zipfile.ZipFile(temporary, 'w', zipfile.ZIP_DEFLATED) as target:
        for info in infos:
            target.writestr(info, files.pop(info.filename))
        for name, data in sorted(files.items()): target.writestr(name, data)
    temporary.replace(path)


def gem_update(path, updates):
    with tarfile.open(path) as source:
        files = {p.name: source.extractfile(p).read() for p in source if p.isfile()}
    files['data.tar.gz'] = tar_bytes(files['data.tar.gz'], updates.copy())
    metadata = gzip.decompress(files['metadata.gz']).decode()
    # JSON strings are YAML scalars, including paths containing punctuation.
    listed = re.search(r'(?ms)^files:\n(.*?)(?=^[^ -]|\Z)', metadata)
    existing = set(re.findall(r'^- (.+)$', listed[1], re.M)) if listed else set()
    additions = ''.join('- ' + json.dumps(name) + '\n' for name in sorted(updates) if name not in existing)
    metadata = re.sub(r'(?m)^files:\n', 'files:\n' + additions, metadata, count=1)
    files['metadata.gz'] = gzip.compress(metadata.encode())
    checksums = '---\n'
    for algorithm in ('SHA256', 'SHA512'):
        checksums += algorithm + ':\n'
        for name in ('metadata.gz', 'data.tar.gz'):
            checksums += f'  {name}: {hashlib.new(algorithm.lower(), files[name]).hexdigest()}\n'
    files['checksums.yaml.gz'] = gzip.compress(checksums.encode())
    temporary = path.with_suffix('.partial')
    with tarfile.open(temporary, 'w', format=tarfile.GNU_FORMAT) as target:
        for name, data in files.items():
            member = tarfile.TarInfo(name)
            member.size, member.mode = len(data), 0o644
            target.addfile(member, io.BytesIO(data))
    temporary.replace(path)


def apply(path, contents, prefix=''):
    updates = {prefix + name: data for name, data in contents.items()}
    if path.suffix == '.gem': gem_update(path, updates)
    elif zipfile.is_zipfile(path): zip_update(path, updates)
    else: path.write_bytes(tar_bytes(path.read_bytes(), updates))


def wheel_files(contents, metadata):
    index = json.loads(contents[NOTICE + '/index.json'])
    text = re.sub(r'(?m)^License-(?:Expression|File):[^\n]*\n', '', metadata)
    headers, body = text.split('\n\n', 1)
    headers += '\nLicense-Expression: ' + index['license_expression']
    headers += ''.join('\nLicense-File: ' + name for name in sorted(contents))
    return headers + '\n\n' + body


def wheel_notices(path, contents):
    with zipfile.ZipFile(path) as source:
        metadata = next(p for p in source.namelist() if p.endswith('.dist-info/METADATA'))
        value = wheel_files(contents, source.read(metadata).decode()).encode()
    prefix = metadata.removesuffix('METADATA') + 'licenses/'
    zip_update(path, {metadata: value, **{prefix + name: data for name, data in contents.items()}}, prefix)


def package_files(out, windows=False):
    requirements = json.loads((out / 'requirements.json').read_text())
    for family in requirements:
        folder = out if family == 'command' else out / family
        default = '*.zip' if windows else '*.tar.gz'
        pattern = {'command': 'thinkthen-*.zip' if windows else 'thinkthen-*.tar.gz',
                   'python': '*.whl', 'typescript': '*.tgz', 'r': 'thinkthen_*',
                   'csharp': '*.nupkg', 'ruby': '*.gem', 'java': 'thinkthen-java-*.jar',
                   'kotlin': 'thinkthen-kotlin-*.jar'}.get(family, default)
        artifact, = folder.glob(pattern)
        yield family, artifact
        if family in ('java', 'kotlin'):
            for dependency in folder.glob('dependencies/thinkthen-java-*.jar'):
                yield 'java', dependency


def runtime_files(name, root=ROOT):
    source = root / 'licenses/runtime' / name
    if not source.is_dir(): raise ValueError('missing runtime notices: ' + name)
    entries = {str(p.relative_to(source)): p.read_bytes() for p in source.rglob('*') if p.is_file()}
    index = {'runtime': name, 'components': {name: {'files': {path: hashlib.sha256(data).hexdigest()
                                                          for path, data in entries.items()}}}}
    return {NOTICE + '/index.json': (json.dumps(index, sort_keys=True, indent=2) + '\n').encode(),
            **{NOTICE + '/' + name + '/' + path: data for path, data in entries.items()}}


def distribute(out, target, root=ROOT):
    produced = json.loads((out / 'compiled-crates.json').read_text())
    if produced['target'] != target: raise ValueError('compiled notice target differs')
    cached = {}
    for family, artifact in package_files(out, target.startswith('windows')):
        key = family if family in ('command', 'python', 'typescript', 'r') else 'ffi'
        if key not in cached: cached[key] = bundle(target, family, root, produced['crates'])
        contents = cached[key].copy()
        for surface, name, source in [('sqlite', 'jansson', 'licenses/native/jansson-2.14.1/LICENSE'),
                                      ('cpp', 'nlohmann-json', 'sdk/cpp/JSON-LICENSE')]:
            if family == surface: contents[NOTICE + '/' + name + '/LICENSE'] = (root / source).read_bytes()
        if family == 'python':
            wheel_notices(artifact, contents)
            continue
        prefix = {'typescript': 'package/', 'r': 'thinkthen/', 'java': 'META-INF/',
                  'kotlin': 'META-INF/'}.get(family, '')
        if family in ('c', 'cpp') and not target.startswith('windows'):
            prefix = 'share/thinkthen-' + family + '/'
        if family == 'zig' and not target.startswith('windows'): prefix = 'thinkthen/'
        apply(artifact, contents, prefix)
    from release_jvm_notices import distribute as jvm_notices
    jvm_notices(out, root)
    from release_r_dependencies import directories, terms, metadata, refresh
    for folder in directories(out):
        inventory = json.loads((folder / 'inventory.json').read_text())
        for filename in inventory['install_order']:
            path = folder / filename
            apply(path, terms(path, root), metadata(path)['Package'] + '/')
        refresh(folder)
    swift = out / 'prerequisites/swift-runtime'
    if swift.is_dir():
        for name, data in runtime_files('swift-icu', root).items():
            path = swift / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
