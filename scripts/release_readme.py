"""Put self-contained user documentation inside the delivered package formats."""
import base64
import gzip
import hashlib
import io
import json
from pathlib import Path
from runtime_floors import FLOORS
import re
import tarfile
import zipfile

ROOT = Path(__file__).resolve().parent.parent
SOURCES = {
    'command': ('README.md', '## Backends'),
    'python': ('sdk/python/README.md', '### Packaging and proof after approval'),
    'typescript': ('sdk/typescript/README.md', None),
    'go': ('sdk/go/README.md', '## Local staged packages'),
    'java': ('sdk/java/README.md', None),
    'kotlin': ('sdk/kotlin/README.md', None),
    'csharp': ('sdk/csharp/README.md', None),
    'r': ('sdk/r/README.md', '## Package'),
    'c': ('sdk/c/README.md', None),
    'cpp': ('sdk/cpp/README.md', '## Package proof'),
    'ruby': ('sdk/ruby/README.md', '## Native dependency and packaging choice'),
    'zig': ('sdk/zig/README.md', None),
    'swift': ('sdk/swift/README.md', '## Package and supported hosts'),
    'sqlite': ('extensions/sqlite/README.md', '## Native implementation boundary and proof'),
    'duckdb': ('databases/duckdb/README.md', '## Native execution and build boundary'),
    'postgres': ('extensions/postgres/README.md', '## Native implementation boundary'),
}


def released_readme(name, *, root=ROOT):
    """Render the shared package guide without platform-specific offline steps."""
    source, boundary = SOURCES[name]
    body = (root / source).read_text()
    if boundary:
        body = body.split('\n' + boundary, 1)[0]
    # Process prose only: C++ lambda captures also contain Markdown-like brackets.
    parts = re.split(r'(^```[^\n]*\n.*?^```[ \t]*$)', body, flags=re.M | re.S)
    for index in range(0, len(parts), 2):
        prose = re.sub(r'\[([^\]]+)\]\((?!https?://)[^)]+\)', r'\1', parts[index])
        parts[index] = '\n\n'.join(p for p in prose.split('\n\n')
                                  if not re.search(r'\b(?:API|PACKAGING)\.md\b|\bexamples/|\bscripts/',
                                                   re.sub(r'https?://[^)\s]+', '', p)))
    body = ''.join(parts)
    return (body.rstrip() + '\n').encode()


def readme(name, out, artifact, *, root=ROOT):
    body = released_readme(name, root=root).decode()
    version = json.loads((out / 'build.json').read_text())['version']
    requirements = json.loads((out / 'requirements.json').read_text())[name]
    python = 'python' if requirements.get('platform') == 'windows-x86_64' else 'python3'
    body = body.replace('pip install thinkthen', f'{python} -m pip install --no-index ./{artifact.name}')
    body = body.replace('npm install thinkthen', f'npm install --offline ./{artifact.name}')
    support = (FLOORS['windows']['support'].rstrip('.') if requirements.get('platform') == 'windows-x86_64' else f'macOS {requirements["macos"]}' if 'macos' in requirements else
               f'Linux glibc >= {requirements["supported_glibc"]}')
    intro = (f'# ThinkThen {version}: {name}\n\n'
             f'{requirements["runtime"]}. {support}.\n\n'
             'Install the bundled file with the normal language tools. '
             'INSTALL.md gives offline installation and stub example commands. '
             'Keep the supplied runtime dependencies and their licenses beside the packages.\n\n')
    return (intro + body + '\n').encode()


def tar_bytes(data, updates):
    """Change documentation only; retain every other member's bytes and mode."""
    output = io.BytesIO()
    with tarfile.open(fileobj=io.BytesIO(data)) as source:
        members = source.getmembers()
        existing = {p.name.removeprefix('./'): p for p in members}
        # R binary formats use MD5 or SHA256 (digest, size, name) tables.
        for key in list(updates):
            if '/' not in key:
                continue
            root, relative = key.split('/', 1)
            for algorithm in ('MD5', 'SHA256'):
                table = root + '/' + algorithm
                if table not in existing:
                    continue
                lines = updates.get(table, source.extractfile(existing[table]).read()).decode().splitlines()
                rows = [line for line in lines if line.split(maxsplit=2 if algorithm == 'SHA256' else 1)[-1].lstrip('*') != relative]
                digest = hashlib.new(algorithm.lower(), updates[key]).hexdigest()
                value = f'{digest} {len(updates[key])} {relative}' if algorithm == 'SHA256' else f'{digest}  {relative}'
                updates[table] = ('\n'.join([*rows, value]) + '\n').encode()
        with tarfile.open(fileobj=output, mode='w:gz', format=tarfile.GNU_FORMAT) as target:
            for member in members:
                key = member.name.removeprefix('./')
                member.pax_headers = {}
                if key in updates:
                    data = updates.pop(key)
                    member.size = len(data)
                    target.addfile(member, io.BytesIO(data))
                else:
                    target.addfile(member, source.extractfile(member) if member.isfile() else None)
            for name, data in updates.items():
                member = tarfile.TarInfo(name)
                member.size, member.mode = len(data), 0o644
                target.addfile(member, io.BytesIO(data))
    return output.getvalue()


def update_zip(path, name, document):
    with zipfile.ZipFile(path) as source:
        infos = source.infolist()
        files = {p.filename: source.read(p) for p in infos}
    files[name] = document
    if path.suffix == '.whl':
        metadata = next(p for p in files if p.endswith('.dist-info/METADATA'))
        files[metadata] = files[metadata].split(b'\n\n', 1)[0] + b'\n\n' + document
        record = next(p for p in files if p.endswith('.dist-info/RECORD'))
        files[record] = ''.join(f'{p},sha256={base64.urlsafe_b64encode(hashlib.sha256(data).digest()).decode().rstrip("=")},{len(data)}\n'
                                for p, data in sorted(files.items()) if p != record).encode() + f'{record},,\n'.encode()
    temporary = path.with_suffix('.tmp')
    with zipfile.ZipFile(temporary, 'w', compression=zipfile.ZIP_DEFLATED) as target:
        for info in infos:
            target.writestr(info, files.pop(info.filename))
        for name, data in files.items():
            target.writestr(name, data)
    temporary.replace(path)


def update_gem(path, document):
    with tarfile.open(path) as source:
        files = {p.name: source.extractfile(p).read() for p in source if p.isfile()}
    files['data.tar.gz'] = tar_bytes(files['data.tar.gz'], {'README.md': document})
    checksums = '---\n'
    for algorithm in ('SHA256', 'SHA512'):
        checksums += algorithm + ':\n'
        for name in ('metadata.gz', 'data.tar.gz'):
            checksums += f'  {name}: {hashlib.new(algorithm.lower(), files[name]).hexdigest()}\n'
    files['checksums.yaml.gz'] = gzip.compress(checksums.encode())
    output = io.BytesIO()
    with tarfile.open(fileobj=output, mode='w', format=tarfile.GNU_FORMAT) as target:
        for name, data in files.items():
            member = tarfile.TarInfo(name)
            member.size, member.mode = len(data), 0o644
            target.addfile(member, io.BytesIO(data))
    path.write_bytes(output.getvalue())


def package_readmes(out):
    for name in json.loads((out / 'requirements.json').read_text()):
        folder = out if name == 'command' else out / name
        pattern = {'command': 'thinkthen-*.tar.gz', 'python': '*.whl', 'typescript': '*.tgz',
                   'r': 'thinkthen_*', 'csharp': '*.nupkg', 'ruby': '*.gem',
                   'java': 'thinkthen-java-*.jar', 'kotlin': 'thinkthen-kotlin-*.jar'}.get(name, '*.tar.gz')
        artifact, = folder.glob(pattern)
        document = readme(name, out, artifact)
        if name in ('python', 'java', 'kotlin', 'csharp'):
            if name == 'python':
                with zipfile.ZipFile(artifact) as wheel:
                    member = next(p.rsplit('/', 1)[0] + '/README.md' for p in wheel.namelist() if p.endswith('.dist-info/METADATA'))
            else:
                member = 'META-INF/README.md' if name in ('java', 'kotlin') else 'README.md'
            update_zip(artifact, member, document)
        elif name == 'ruby':
            update_gem(artifact, document)
        else:
            member = {'typescript': 'package/README.md', 'r': 'thinkthen/README.md',
                      'c': 'share/thinkthen-c/README.md', 'cpp': 'share/thinkthen-cpp/README.md',
                      'zig': 'thinkthen/README.md'}.get(name, 'README.md')
            artifact.write_bytes(tar_bytes(artifact.read_bytes(), {member: document}))
        if name != 'command':
            (folder / 'README.md').write_bytes(document)
