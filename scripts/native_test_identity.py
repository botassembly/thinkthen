"""Derive SQL fixture identity from source metadata, without producing native code."""
import hashlib
import json
import subprocess


def identity(root, cache, producer):
    def output(*args):
        return subprocess.check_output(args, cwd=root, text=True).strip()

    def digest(path):
        return hashlib.sha256(path.read_bytes()).hexdigest()

    host = output('rustc', '-vV').split('host: ')[1].splitlines()[0]
    producer = digest(producer)
    metadata = root / 'scripts/native_test_identity.py'
    if metadata.is_file():
        producer = hashlib.sha256(json.dumps({'producer': producer, 'metadata': digest(metadata)},
                                            sort_keys=True).encode()).hexdigest()
    loopback = {name: digest(root / name) for name in output('git', 'ls-files', '--',
                'crates/loopback', 'crates/thinkthen/test-support').splitlines()}
    configuration = hashlib.sha256(json.dumps({'producer': producer, 'loopback': loopback}, sort_keys=True).encode()).hexdigest()
    return cache / 'test-fixtures' / host / configuration, {
        'native_key': cache.name, 'producer': producer, 'target': host,
        'profile': 'debug', 'features': ['fixture-routing'], 'loopback': loopback,
    }
