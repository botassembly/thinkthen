"""Put self-contained user documentation inside the delivered package formats."""
import base64
import gzip
import hashlib
import io
import json
from pathlib import Path
from runtime_floors import FLOORS
from release_python_dependencies import wheelhouse
import re
import tarfile
import zipfile

ROOT = Path(__file__).resolve().parent.parent
SOURCES = {
    'command': 'README.md',
    'python': 'sdk/python/README.md',
    'typescript': 'sdk/typescript/README.md',
    'go': 'sdk/go/README.md',
    'java': 'sdk/java/README.md',
    'kotlin': 'sdk/kotlin/README.md',
    'csharp': 'sdk/csharp/README.md',
    'r': 'sdk/r/README.md',
    'c': 'sdk/c/README.md',
    'cpp': 'sdk/cpp/README.md',
    'ruby': 'sdk/ruby/README.md',
    'zig': 'sdk/zig/README.md',
    'swift': 'sdk/swift/README.md',
    'sqlite': 'extensions/sqlite/README.md',
    'duckdb': 'databases/duckdb/README.md',
    'postgres': 'extensions/postgres/README.md',
}


def released_readme(name, *, root=ROOT):
    """Render the shared package guide without platform-specific offline steps."""
    source = SOURCES[name]
    body = (root / source).read_text(encoding='utf-8')
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
    version = json.loads((out / 'build.json').read_text(encoding='utf-8'))['version']
    requirements = json.loads((out / 'requirements.json').read_text(encoding='utf-8'))[name]
    python = 'python' if requirements.get('platform') == 'windows-x86_64' else 'python3'
    if name == 'python':
        dependencies = '../' + wheelhouse(out).relative_to(out).as_posix()
        install = f'{python} -m pip install --no-index --find-links {dependencies} ./{artifact.name}'
        body = body.replace('Run `python -m pip install thinkthen`.',
                            f'Install the bundled wheel:\n\n```sh\n{install}\n```\n')
        body = body.replace('python -m pip install thinkthen', install)
        body = body.replace('pip install thinkthen', install)
    body = body.replace('npm install thinkthen', f'npm install --offline ./{artifact.name}')
    support = (FLOORS['windows']['support'].rstrip('.') if requirements.get('platform') == 'windows-x86_64' else f'macOS {requirements["macos"]}' if 'macos' in requirements else
               f'Linux glibc >= {requirements["supported_glibc"]}')
    intro = (f'# ThinkThen {version}: {name}\n\n'
             f'{requirements["runtime"]}. {support}.\n\n'
             'Install the bundled file with the normal language tools. '
             'Contributors can use INSTALL.md for offline package checks. '
             'Keep the supplied runtime dependencies and their licenses beside the packages.\n\n')
    return (intro + body + '\n').encode()


def windows_steps(document, folder, addresses=None):
    """Write a README's one PowerShell block as a script and return its command.

    Only an address may change, for a download a check must serve locally.
    """
    block, = re.findall(r'```powershell\n(.*?)\n```', document.read_text(encoding='utf-8'), re.S)
    for address, local in (addresses or {}).items():
        if address not in block:
            raise ValueError(f'{document} does not name {address}')
        block = block.replace(address, local)
    script = folder / 'windows-steps.ps1'
    # Tools write progress to stderr, so Stop could end a good run. The trap catches a missing command.
    guard = 'if ($LASTEXITCODE) { exit $LASTEXITCODE }\n'
    steps = ''.join(line + '\n' + guard for line in block.splitlines())
    script.write_text("$ErrorActionPreference = 'Continue'\ntrap { Write-Output $_; exit 1 }\n" + steps + 'exit 0\n', encoding='utf-8')
    return 'powershell', '-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'Bypass', '-File', script


def tar_bytes(data, updates, remove=()):
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
                if key in remove:
                    continue
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
    for name in json.loads((out / 'requirements.json').read_text(encoding='utf-8')):
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
