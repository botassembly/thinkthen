"""Freeze the Windows CPython frame dependencies before offline packaging."""
import email
import hashlib
import json
from pathlib import Path
import shutil
from urllib.parse import urlparse
import zipfile

from release_prepare_common import TOOLS, PINS, fetch
HASHES = Path(__file__).with_name('release-python-hashes.json')


def inputs():
    catalog = json.loads(HASHES.read_text(encoding='utf-8'))
    return {name: {'url': url, 'sha256': catalog[name][urlparse(url).path.rsplit('/', 1)[-1]]}
            for name, url in PINS['windows_python_frames'].items()}


def location():
    key = hashlib.sha256(json.dumps(inputs(), sort_keys=True).encode()).hexdigest()
    return TOOLS / 'windows-python-frames' / key


def prepare():
    destination = location()
    wheels = destination / 'wheels'
    wheels.mkdir(parents=True, exist_ok=True)
    rows = []
    requirements = []
    for name, entry in inputs().items():
        filename = urlparse(entry['url']).path.rsplit('/', 1)[-1]
        wheel = fetch(entry['url'], wheels / filename, entry['sha256'])
        with zipfile.ZipFile(wheel) as archive:
            metadata = email.message_from_bytes(archive.read(next(
                item for item in archive.namelist() if item.endswith('.dist-info/METADATA'))))
            if metadata['Name'].lower().replace('_', '-') != name:
                raise ValueError('Windows wheel package differs: ' + filename)
            licenses = [item for item in archive.namelist() if 'license' in item.lower() or 'copying' in item.lower()]
        requirements.append(f'{name}=={metadata["Version"]} --hash=sha256:{entry["sha256"]}')
        rows.append(dict(name=name, version=metadata['Version'], file=filename,
                         sha256=entry['sha256'], requires=metadata.get_all('Requires-Dist', []), licenses=licenses))
    (destination / 'requirements.txt').write_text('\n'.join(requirements) + '\n', encoding='utf-8')
    (destination / 'inventory.json').write_text(json.dumps({
        'scope': 'Windows x86-64 CPython 3.13; optional pandas and Polars dependencies',
        'wheels': rows}, indent=2) + '\n', encoding='utf-8')
    return destination


def stage(out):
    source = location()
    inventory = json.loads((source / 'inventory.json').read_text(encoding='utf-8'))
    expected = inputs()
    if (len(inventory['wheels']) != len(expected) or
            {row['name'] for row in inventory['wheels']} != set(expected)):
        raise ValueError('Windows frame dependency inventory differs')
    actual = set()
    requirements = []
    for row in inventory['wheels']:
        entry = expected[row['name']]
        filename = urlparse(entry['url']).path.rsplit('/', 1)[-1]
        if row['file'] != filename or row['version'] != filename.split('-')[1]:
            raise ValueError('Windows frame dependency identity differs')
        requirements.append(f'{row["name"]}=={row["version"]} --hash=sha256:{entry["sha256"]}')
        wheel = source / 'wheels' / row['file']
        digest = hashlib.sha256(wheel.read_bytes()).hexdigest()
        if digest != row['sha256'] or digest != expected[row['name']]['sha256']:
            raise ValueError('Windows frame dependency checksum differs: ' + row['file'])
        actual.add(row['file'])
    if actual != {path.name for path in (source / 'wheels').iterdir()}:
        raise ValueError('Windows frame dependency wheel inventory differs')
    if (source / 'requirements.txt').read_text(encoding='utf-8') != '\n'.join(requirements) + '\n':
        raise ValueError('Windows frame dependency requirements differ')
    shutil.copytree(source, out / 'prerequisites/python-frames-windows')
