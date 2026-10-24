"""Verify the declared, source-controlled changes to patched upstream crates."""
import difflib
import hashlib
import json
from pathlib import Path
import tomllib


def patched(root):
    root = Path(root).resolve()
    result = {}
    declarations = tomllib.loads((root / 'Cargo.toml').read_text()).get('patch', {}).get('crates-io', {})
    for name, declaration in declarations.items():
        folder = (root / declaration['path']).resolve()
        folder.relative_to(root)
        record = json.loads((folder / 'UPSTREAM.json').read_text())
        package = tomllib.loads((folder / 'Cargo.toml').read_text())['package']
        if (package['name'], package['version']) != (name, record['version']) or record['name'] != name:
            raise ValueError('patched crate identity differs: ' + name)
        generated = []
        for relative, expected in record['upstream_files'].items():
            path = folder / relative
            path.resolve().relative_to(folder)
            actual = path.read_bytes()
            original = actual.replace((record['linkage_attribute'] + '\n').encode(), b'') if path.suffix == '.rs' else actual
            if hashlib.sha256(original).hexdigest() != expected:
                raise ValueError('patched upstream source differs: ' + relative)
            if original != actual:
                generated.extend(difflib.unified_diff(original.decode().splitlines(True), actual.decode().splitlines(True),
                                                    fromfile='a/' + relative, tofile='b/' + relative, n=0))
        patch = ''.join(generated).encode()
        if (patch != (folder / 'msvc-linkage.patch').read_bytes()
                or hashlib.sha256(patch).hexdigest() != record['patch_sha256']
                or hashlib.sha256((folder / 'LICENSE').read_bytes()).hexdigest() != record['license_sha256']):
            raise ValueError('patched crate patch or license differs: ' + name)
        expected_files = {*record['upstream_files'], 'UPSTREAM.json', 'msvc-linkage.patch', 'LICENSE'}
        if {p.relative_to(folder).as_posix() for p in folder.rglob('*') if p.is_file()} != expected_files:
            raise ValueError('patched crate has unlisted files: ' + name)
        result[name] = dict(record, folder=folder)
    return result
