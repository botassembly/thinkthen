"""Derive offline R installation order and terms from binary package metadata."""
import email
import hashlib
import json
from pathlib import Path
import re
import shutil
import tarfile
import zipfile


def fields(data):
    return dict(email.message_from_bytes(data))


def names(metadata, keys=('Depends', 'Imports', 'LinkingTo')):
    return {name for key in keys for name in
            re.findall(r'(?:^|,)\s*([A-Za-z][A-Za-z0-9.]*)', metadata.get(key, ''))}


def contents(path):
    if zipfile.is_zipfile(path):
        with zipfile.ZipFile(path) as archive:
            return {name: archive.read(name) for name in archive.namelist() if not name.endswith('/')}
    with tarfile.open(path) as archive:
        return {p.name.removeprefix('./'): archive.extractfile(p).read() for p in archive if p.isfile()}


def metadata(path):
    files = contents(path)
    description, = [name for name in files if name.count('/') == 1 and name.endswith('/DESCRIPTION')]
    result = fields(files[description])
    if 'Built' not in result:
        raise ValueError('R prerequisite is not an installed binary: ' + str(path))
    return result


def inventory(description, archives, r_version, standard, suggests=False):
    """Refuse incomplete closures; standard names come from the target R library."""
    available = {}
    for path in archives:
        info = metadata(path)
        name = info['Package']
        if name in available:
            raise ValueError('Duplicate R binary: ' + name)
        available[name] = (Path(path), info)
    required = names(fields(Path(description).read_bytes()), ('Imports', 'Suggests') if suggests else ('Imports',))
    selected, order = {}, []
    active = set()

    def visit(name):
        if name in standard or name == 'R' or name in selected:
            return
        if name in active:
            raise ValueError('Cyclic R dependency: ' + name)
        if name not in available:
            raise ValueError('Missing pinned R binary dependency: ' + name)
        path, info = available[name]
        if not info['Built'].startswith('R ' + r_version + ';'):
            raise ValueError('R binary version differs: ' + str(path))
        active.add(name)
        for dependency in sorted(names(info)):
            visit(dependency)
        active.remove(name)
        selected[name] = {'file': path.name, 'version': info['Version'], 'license': info['License'],
                          'dependencies': sorted(names(info)),
                          'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}
        order.append(path.name)

    for name in sorted(required):
        visit(name)
    return {'r': r_version, 'packages': selected, 'install_order': order}


def write(folder, result):
    folder = Path(folder)
    (folder / 'inventory.json').write_text(json.dumps(result, indent=2) + '\n')
    (folder / 'install-order.txt').write_text('\n'.join(result['install_order']) + '\n')


def bundle(description, archives, folder, r_version, standard, suggests=False):
    archives = list(map(Path, archives))
    result = inventory(description, archives, r_version, standard, suggests)
    folder = Path(folder)
    folder.mkdir(parents=True, exist_ok=True)
    by_name = {path.name: path for path in archives}
    for name in result['install_order']:
        if by_name[name].resolve() != (folder / name).resolve():
            shutil.copy2(by_name[name], folder / name)
    write(folder, result)
    return result


def terms(path, root):
    """Keep each binary's own license terms and any reviewed vendored terms."""
    files = contents(path)
    info = metadata(path)
    name = info['Package']
    result = {'DESCRIPTION': files[name + '/DESCRIPTION']}
    for relative, data in files.items():
        leaf = Path(relative).name.upper()
        if 'THIRD-PARTY-NOTICES' not in relative.split('/') and relative.startswith(name + '/') and leaf.startswith(('LICENSE', 'LICENCE', 'COPYING', 'COPYRIGHT')):
            result[relative[len(name) + 1:]] = data
    reviewed = Path(root) / 'licenses/runtime' / (name + '-' + info['Version'])
    if reviewed.is_dir():
        result.update({p.relative_to(reviewed).as_posix(): p.read_bytes() for p in reviewed.rglob('*') if p.is_file()})
    # The R distribution supplies standard license texts used by its packages.
    license_dir = Path(path).parent / 'licenses'
    if license_dir.is_dir():
        result.update({(Path('R') / p.name).as_posix(): p.read_bytes() for p in license_dir.iterdir() if p.is_file()})
    if len(result) == 1:
        raise ValueError('Missing R dependency license terms: ' + name)
    from release_notices import NOTICE
    index = {'components': {name: {'files': {key: hashlib.sha256(data).hexdigest()
                                          for key, data in result.items()}}}}
    return {NOTICE + '/index.json': (json.dumps(index, indent=2) + '\n').encode(),
            **{NOTICE + '/' + name + '/' + key: data for key, data in result.items()}}


def directories(out):
    return [path for path in (Path(out) / 'prerequisites/r', Path(out) / 'prerequisites/r-frames')
            if (path / 'inventory.json').is_file()]


def refresh(folder):
    result = json.loads((folder / 'inventory.json').read_text())
    for package in result['packages'].values():
        package['sha256'] = hashlib.sha256((folder / package['file']).read_bytes()).hexdigest()
    write(folder, result)
