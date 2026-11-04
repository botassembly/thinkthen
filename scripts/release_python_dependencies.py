"""Share the pinned Python runtime wheels with offline package consumers."""
import hashlib
import json
from pathlib import Path
import shutil
from urllib.parse import urlparse

from package_inventory import python_wheelhouse as wheelhouse

ROOT = Path(__file__).resolve().parent.parent


def inputs():
    pins = json.loads((ROOT / 'scripts/release-tools.json').read_text())['python_runtime']
    hashes = json.loads((ROOT / 'scripts/release-python-hashes.json').read_text())
    return {name: {'url': url, 'file': urlparse(url).path.rsplit('/', 1)[-1],
                   'sha256': hashes[name][urlparse(url).path.rsplit('/', 1)[-1]]}
            for name, url in pins.items()}


def versions():
    return {name: entry['file'].split('-')[1] for name, entry in inputs().items()}


def stage_runtime(package, destination):
    """Copy the portable runtime closure outside publishable registry artifacts."""
    destination.mkdir(parents=True, exist_ok=True)
    for entry in inputs().values():
        source = wheelhouse(package) / entry['file']
        if hashlib.sha256(source.read_bytes()).hexdigest() != entry['sha256']:
            raise ValueError('Python runtime wheel checksum differs: ' + source.name)
        shutil.copy2(source, destination / source.name)
