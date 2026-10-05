"""Package both pinned DuckDB legal inventories and the common Rust notices."""
from __future__ import annotations

import json
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parents[2] / 'conformance/children'))
sys.path.insert(0, str(HERE.parent / 'tools'))
from children import CARGO, child_env  # noqa: E402
from inputs import versions  # noqa: E402
from validate_inputs import inventory as archive_inventory  # noqa: E402

BRIDGE = HERE.parent / 'bridge/Cargo.toml'
LEGAL = ('license', 'notice', 'copying', 'authors', 'copyright', 'patents')


def main() -> None:
    if len(sys.argv) < 6 or (len(sys.argv) - 3) % 3:
        raise SystemExit('usage: package_notices.py PACKAGE VERSION [DUCKDB_VERSION SOURCE MANIFEST]...')
    destination = Path(sys.argv[1]).resolve(strict=True)
    rows = list(zip(sys.argv[3::3], sys.argv[4::3], sys.argv[5::3]))
    if [row[0] for row in rows] != versions():
        raise SystemExit('notices require every supported DuckDB version in order')
    notice = [f'ThinkThen {sys.argv[2]}: MIT (LICENSE.thinkthen)']
    inventory = []
    identical: dict[tuple[str, bytes], Path] = {}
    for version, source_name, manifest in rows:
        source = Path(source_name).resolve(strict=True)
        files = sorted(p for p in source.rglob('*') if p.is_file() and p.name.lower().startswith(LEGAL))
        if source / 'LICENSE' not in files or len(files) < 2:
            raise SystemExit('the pinned DuckDB source lacks license or third-party notices')
        inventory.extend((f'Pinned DuckDB {version} static archives:', *archive_inventory(Path(manifest)),
                          '', f'Pinned DuckDB {version} source license and notice files:'))
        for path in files:
            rel = path.relative_to(source)
            key = (str(rel), path.read_bytes())
            target = identical.get(key)
            if target is None:
                target = (destination / 'LICENSE.duckdb' if rel == Path('LICENSE') and version == rows[0][0]
                          else destination / 'LICENSES/duckdb' / version / rel)
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(path, target)
                identical[key] = target
            inventory.append(f'{rel} -> {target.relative_to(destination)}')
            if rel == Path('LICENSE'):
                notice.append(f'DuckDB {version} source and static archives: MIT ({target.relative_to(destination)})')
        inventory.append('')
    metadata = json.loads(subprocess.check_output([
        'cargo', 'metadata', '--format-version', '1', '--locked', '--offline', '--manifest-path', str(BRIDGE),
    ], env=child_env(CARGO)))
    inventory.append('Bundled Rust crates (name version | declared license):')
    for package in sorted(metadata['packages'], key=lambda item: (item['name'], item['version'])):
        folder = Path(package['manifest_path']).parent
        if '/registry/' not in str(folder):
            continue
        files = sorted(p for p in folder.iterdir() if p.is_file() and p.name.lower().startswith(LEGAL))
        if not files:
            raise SystemExit(f"missing legal file for {package['name']} {package['version']}")
        name = f"{package['name']}-{package['version']}"
        inventory.append(f"{name} | {package['license'] or 'see copied legal files'}")
        target = destination / 'LICENSES/rust' / name
        target.mkdir(parents=True, exist_ok=True)
        for path in files:
            shutil.copyfile(path, target / path.name)
    notice.append('Third-party license texts and Rust dependency inventory: LICENSES/ and DEPENDENCIES.txt')
    (destination / 'NOTICE').write_text('\n'.join(notice) + '\n')
    (destination / 'DEPENDENCIES.txt').write_text('\n'.join(inventory) + '\n')


if __name__ == '__main__':
    main()
