"""Resolve target dependencies and carry their complete license texts into packages."""
import hashlib
from functools import cache
import json
from pathlib import Path
import re
import subprocess
import tomllib

from release_patched import patched

ROOT = Path(__file__).resolve().parent.parent
TARGETS = {'linux-x86_64': 'x86_64-unknown-linux-gnu', 'linux-arm64': 'aarch64-unknown-linux-gnu',
           'macos-arm64': 'aarch64-apple-darwin', 'windows-x86_64': 'x86_64-pc-windows-msvc'}
SURFACES = {'command': 'thinkthen', 'python': 'thinkthen-python',
            'typescript': 'thinkthen-node', 'r': 'thinkthen-r'}
NOTICE = 'THIRD-PARTY-NOTICES'


def digest(data):
    return hashlib.sha256(data).hexdigest()


def locked(root):
    result = {p['name'] + '-' + p['version']: p['checksum']
              for p in tomllib.loads((root / 'Cargo.lock').read_text())['package'] if 'source' in p}
    result.update({name + '-' + entry['version']: entry['checksum'] for name, entry in patched(root).items()})
    return result


@cache
def metadata(root, target=None):
    args = ['cargo', 'metadata', '--offline', '--locked', '--format-version=1']
    if target: args += ['--filter-platform', target]
    return json.loads(subprocess.check_output(args, cwd=root))


def license_files(folder):
    return sorted(p for p in folder.rglob('*') if p.is_file() and
                  (re.match(r'(?i)^(licen[cs]e|copying|copyright|notice)(?:[._-]|$)', p.name)
                   or (p.name.upper() == 'AUTHORS' and p.read_text().startswith('LICENSE:'))))


def package_terms(folder, name, license_file=None, root=ROOT):
    paths = license_files(folder)
    if not paths:
        folder = root / 'licenses/upstream' / name
        paths = license_files(folder)
    if license_file:
        paths = sorted(set([*paths, folder / license_file]))
    if not paths: raise ValueError('missing license text: ' + name)
    return {str(path.relative_to(folder)): path.read_bytes() for path in paths}


def inputs(root):
    root = Path(root).resolve()
    members = tomllib.loads((root / 'Cargo.toml').read_text())['workspace']['members']
    manifests = {path for member in members for path in root.glob(member + '/Cargo.toml')}
    patched_files = [p for entry in patched(root).values() for p in entry['folder'].rglob('*') if p.is_file()]
    return {str(p.relative_to(root)): digest(p.read_bytes()) for p in
            [root / 'Cargo.lock', root / 'Cargo.toml', *sorted(manifests), *sorted(patched_files)]}


def refresh(root=ROOT):
    """Copy license texts from Cargo's checksum-verified source cache."""
    catalog = {}
    changes = patched(root)
    destination = root / 'licenses/crates'
    wanted = locked(root)
    selected = set().union(*(resolve(root, target, family) for target in TARGETS
                             for family in [*SURFACES, 'ffi']))
    wanted = {name: checksum for name, checksum in wanted.items() if name in selected}
    for package in metadata(root)['packages']:
        if package['source'] is None and package['name'] not in changes: continue
        name = package['name'] + '-' + package['version']
        if name not in wanted: continue
        folder = Path(package['manifest_path']).parent
        texts = package_terms(folder, name, package['license_file'], root)
        if not package['license']:
            raise ValueError('missing license declaration or text: ' + name)
        entry = {'checksum': wanted[name], 'license': package['license'].replace('/', ' OR '), 'files': {}}
        if package['name'] in changes: entry['patch_sha256'] = changes[package['name']]['patch_sha256']
        for relative, data in texts.items():
            target = destination / name / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
            entry['files'][relative] = digest(data)
        catalog[name] = entry
    if set(catalog) != set(wanted):
        raise ValueError('Cargo metadata did not include every locked source')
    kept = {destination / name / relative for name, entry in catalog.items() for relative in entry['files']}
    for path in destination.rglob('*'):
        if path.is_file() and path not in kept: path.unlink()
    (root / 'licenses/inputs.json').write_text(json.dumps(inputs(root), sort_keys=True, indent=2) + '\n')
    (root / 'licenses/cargo.json').write_text(json.dumps(catalog, sort_keys=True, indent=2) + '\n')
    check(root)


def check(root=ROOT):
    catalog = json.loads((root / 'licenses/cargo.json').read_text())
    checksums = locked(root)
    if (json.loads((root / 'licenses/inputs.json').read_text()) != inputs(root) or
            any(entry['checksum'] != checksums.get(name) for name, entry in catalog.items())):
        raise ValueError('crate notices are stale; run scripts/notices refresh after cargo fetch --locked')
    changes = patched(root)
    declarations = {p['name'] + '-' + p['version']: p['license'].replace('/', ' OR ')
                    for p in metadata(root)['packages'] if (p['source'] or p['name'] in changes) and p['license']}
    selected = set().union(*(resolve(root, target, family) for target in TARGETS
                             for family in [*SURFACES, 'ffi']))
    if set(catalog) != selected:
        raise ValueError('crate notice inventory differs from resolved target dependencies')
    expected = set()
    for name, entry in catalog.items():
        expected_patch = next((p['patch_sha256'] for n, p in changes.items() if n + '-' + p['version'] == name), None)
        if entry.get('patch_sha256') != expected_patch: raise ValueError('crate patch identity differs: ' + name)
        if entry['license'] != declarations.get(name):
            raise ValueError('crate license declaration differs: ' + name)
        for relative, checksum in entry['files'].items():
            path = root / 'licenses/crates' / name / relative
            expected.add(path)
            if digest(path.read_bytes()) != checksum:
                raise ValueError('crate license text differs: ' + name + '/' + relative)
    actual = {p for p in (root / 'licenses/crates').rglob('*') if p.is_file()}
    if actual != expected:
        raise ValueError('unlisted or stale crate license texts')
    for relative, entry in json.loads((root / 'licenses/runtime.json').read_text()).items():
        if digest((root / 'licenses/runtime' / relative).read_bytes()) != entry['sha256']:
            raise ValueError('runtime license text differs: ' + relative)
    return catalog


def resolve(root, target, family):
    data = metadata(root, TARGETS[target])
    packages = {p['id']: p for p in data['packages']}
    nodes = {p['id']: p for p in data['resolve']['nodes']}
    start = SURFACES.get(family, 'thinkthen-ffi')
    pending = [p['id'] for p in data['packages'] if p['name'] == start]
    if len(pending) != 1: raise ValueError('missing notice root: ' + start)
    visited, selected = set(), set()
    changes = patched(root)
    while pending:
        identity = pending.pop()
        if identity in visited: continue
        visited.add(identity)
        package = packages[identity]
        if package['source'] is not None or package['name'] in changes:
            selected.add(package['name'] + '-' + package['version'])
        pending.extend(dep['pkg'] for dep in nodes[identity]['deps']
                       if any(kind['kind'] is None for kind in dep['dep_kinds'])
                       and not any('proc-macro' in target['kind'] for target in packages[dep['pkg']]['targets']))
    return sorted(selected)


def bundle(target, family, root=ROOT, compiled=None):
    catalog = check(root)
    names = resolve(root, target, family)
    if compiled is not None: names = sorted(set(names) & set(compiled))
    entries = {name: catalog[name] for name in names}
    index = {'target': target, 'package': SURFACES.get(family, 'thinkthen-ffi'),
             'license_expression': license_expression(entries), 'crates': entries}
    files = {'LICENSE': (root / 'LICENSE').read_bytes(),
             NOTICE + '/index.json': (json.dumps(index, sort_keys=True, indent=2) + '\n').encode()}
    for name, entry in entries.items():
        for relative in entry['files']:
            files[NOTICE + '/' + name + '/' + relative] = (root / 'licenses/crates' / name / relative).read_bytes()
    return files


def license_expression(entries):
    expressions = sorted({'MIT', *(entry['license'] for entry in entries.values())})
    return ' AND '.join('(' + term + ')' if ' ' in term else term for term in expressions)


def source_bundle(vendor, root=ROOT):
    """A source package redistributes all vendored crates, including build tools."""
    entries, files = {}, {'LICENSE': (root / 'LICENSE').read_bytes()}
    changes = patched(root)
    manifests = [*sorted(vendor.glob('*/Cargo.toml')), *(entry['folder'] / 'Cargo.toml' for entry in changes.values())]
    for manifest in manifests:
        package = tomllib.loads(manifest.read_text())['package']
        name = package['name'] + '-' + package['version']
        texts = package_terms(manifest.parent, name, package.get('license-file'), root)
        checksum = changes[package['name']]['checksum'] if package['name'] in changes else json.loads((manifest.parent / '.cargo-checksum.json').read_text())['package']
        entry = {'checksum': checksum,
                 'license': package['license'].replace('/', ' OR '),
                 'files': {relative: digest(data) for relative, data in texts.items()}}
        if package['name'] in changes: entry['patch_sha256'] = changes[package['name']]['patch_sha256']
        entries[name] = entry
        files.update({NOTICE + '/' + name + '/' + relative: data for relative, data in texts.items()})
    if not entries: raise ValueError('source package has no vendored dependencies')
    index = {'target': 'source', 'package': 'thinkthen-r',
             'license_expression': license_expression(entries), 'crates': entries}
    files[NOTICE + '/index.json'] = (json.dumps(index, sort_keys=True, indent=2) + '\n').encode()
    return files


def combine(bundles):
    """Keep each resolved target's inventory when one package carries several engines."""
    files = {}
    for target, contents in bundles.items():
        for name, data in contents.items():
            name = NOTICE + '/' + target + name[len(NOTICE):] if name.startswith(NOTICE) else name
            if name in files and files[name] != data: raise ValueError('conflicting license text: ' + name)
            files[name] = data
    return files


def compiled_names(target, messages, root=ROOT):
    """Compare notice resolution with the shared producer's target library outputs."""
    selected = set().union(*(resolve(root, target, family) for family in [*SURFACES, 'ffi']))
    data = metadata(root, TARGETS[target])
    changes = patched(root)
    identities = {p['id']: p['name'] + '-' + p['version'] for p in data['packages'] if p['source'] or p['name'] in changes}
    actual = set()
    for line in Path(messages).read_text().splitlines():
        item = json.loads(line)
        if item.get('reason') != 'compiler-artifact' or item['package_id'] not in identities: continue
        if any('/' + TARGETS[target] + '/' in filename for filename in item['filenames']):
            actual.add(identities[item['package_id']])
    if not actual or actual - selected:
        raise ValueError(f'notice graph differs from compiled target libraries: missing={sorted(actual-selected)}, extra={sorted(selected-actual)}')
    return sorted(actual)


def compiled(target, messages, root=ROOT):
    names = compiled_names(target, messages, root)
    selected = set().union(*(json.loads(bundle(target, family, root, names)[NOTICE + '/index.json'])['crates']
                             for family in [*SURFACES, 'ffi']))
    if selected != set(names): raise ValueError('notices omit compiled target crates')
    print(f'target notices match {len(names)} compiled crates: {target}')
