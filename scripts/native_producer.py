"""Keep native production separate from receipt-only pair registration."""
import argparse
import hashlib
import json
import os
import re
from pathlib import Path
import runpy
import shutil
import subprocess

from native_platform import HOSTS


ROOT = Path(__file__).resolve().parent.parent
HOST_ASSETS = {
    'mac': {'osx-arm64/libthinkthen_ffi.dylib', 'osx-arm64/libthinkthen_ffi.a'},
    'linux': {'linux-x64/libthinkthen_ffi.so', 'linux-x64/libthinkthen_ffi.a', 'thinkthen.h'},
}


def validate_receipt(receipt, name):
    expected_host = next(row[3] for row in HOSTS.values()
                         if row[1] == ('osx-arm64' if name == 'mac' else 'linux-x64'))
    for field, width in (('source_revision', 40), ('native_key', 64), ('config_sha256', 64)):
        if not isinstance(receipt.get(field), str) or not re.fullmatch('[0-9a-f]{' + str(width) + '}', receipt[field]):
            raise ValueError('invalid producer ' + field)
    if receipt.get('host') != expected_host or receipt.get('native_production_executed') is not True:
        raise ValueError('producer host or production outcome differs: ' + name)
    if not isinstance(receipt.get('producer_job'), str) or not receipt['producer_job'].strip():
        raise ValueError('producer job is required')
    if not isinstance(receipt.get('target_directory'), str) or not Path(receipt['target_directory']).is_absolute():
        raise ValueError('producer target must be absolute')
    if not re.fullmatch('sha256:[0-9a-f]{64}', receipt.get('floor_image', '')):
        raise ValueError('producer floor image must be immutable')
    if set(receipt.get('assets', {})) != HOST_ASSETS[name] or any(
            not isinstance(value, str) or not re.fullmatch('[0-9a-f]{64}', value)
            for value in receipt['assets'].values()):
        raise ValueError('producer asset inventory differs: ' + name)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(*args):
    return subprocess.check_output(list(map(str, args)), cwd=ROOT, text=True).strip()


def register(destination, mac, linux):
    # cache_path queries canonical metadata only. main/--path also checks a
    # generated header with a compiler, which receipt assembly must never do.
    cache = runpy.run_path(str(ROOT / 'scripts/native-prepare'))['cache_path']()
    receipts = {name: json.loads((folder / 'receipt.json').read_text())
                for name, folder in (('mac', mac), ('linux', linux))}
    hashes = {}
    for name, folder in (('mac', mac), ('linux', linux)):
        receipt = receipts[name]
        validate_receipt(receipt, name)
        if (receipt['native_key'] != cache.name or receipt['native_production_executed'] is not True
                or receipt['floor_image'] != os.environ['THINKTHEN_FLOOR_IMAGE']
                or set(receipt['assets']) != HOST_ASSETS[name]
                or digest(folder / 'cargo-config.toml') != receipt['config_sha256']):
            raise ValueError('producer identity/configuration differs: ' + name)
        for path, expected in receipt['assets'].items():
            if digest(folder / 'assets' / path) != expected:
                raise ValueError('producer asset differs: ' + path)
            hashes[path] = expected
    for name, folder in (('mac', mac), ('linux', linux)):
        for path in HOST_ASSETS[name]:
            target = destination / 'assets' / path
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(folder / 'assets' / path, target)
        shutil.copytree(folder, destination / name)
    registration = dict(consumer_revision=run('git', 'rev-parse', 'HEAD'), native_key=cache.name,
        native_key_inputs_exact=True, production_receipts=receipts, five_assets=hashes)
    (destination / 'registration.json').write_text(json.dumps(registration, indent=2) + '\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('prepare', 'prepare-gate', 'inspect-source', 'build', 'register'))
    parser.add_argument('--output', type=Path)
    parser.add_argument('--mac', type=Path)
    parser.add_argument('--linux', type=Path)
    parser.add_argument('--floor-image', required=True)
    parser.add_argument('--cargo-home', required=True, type=Path)
    parser.add_argument('--target', type=Path, help='fixed absolute production target directory')
    args = parser.parse_args()
    if args.action == 'register' and (args.mac is None or args.linux is None):
        parser.error('register requires --mac and --linux producer directories')
    if not re.fullmatch('sha256:[0-9a-f]{64}', args.floor_image):
        parser.error('provide the immutable --floor-image')
    if not args.cargo_home.is_absolute():
        parser.error('provide an absolute --cargo-home')
    os.environ['THINKTHEN_FLOOR_IMAGE'] = args.floor_image
    os.environ['CARGO_HOME'] = str(args.cargo_home)
    if args.target is not None:
        if not args.target.is_absolute():
            parser.error('provide a fixed absolute --target')
        os.environ['CARGO_TARGET_DIR'] = str(args.target)
    if args.action == 'inspect-source':
        command = ['git', 'status', '--porcelain', '--untracked-files=all']
        print(json.dumps(dict(host_revision=run('git', 'rev-parse', 'HEAD'),
                              host_status=run(*command))), flush=True)
        if os.sys.platform == 'linux':
            subprocess.run(['scripts/runtime-floor-build', *command], cwd=ROOT, check=True)
        return
    if args.action == 'prepare':
        # SQLite owns its compiler-floor boundary. Host qualification tools do
        # not belong inside that floor and are not needed to produce the pair.
        subprocess.run(['scripts/qualification-gate-prepare', '--native-only'], cwd=ROOT, check=True)
        return
    if args.action == 'prepare-gate':
        command = ['scripts/qualification-gate-prepare']
        if os.sys.platform == 'linux':
            command = ['scripts/runtime-floor-build', '--prepare', *command]
        subprocess.run(command, cwd=ROOT, check=True)
        return
    if args.output is None:
        parser.error('build and register require --output')
    if args.action == 'build':
        if args.target is None:
            parser.error('build requires the fixed absolute --target')
        if run('git', 'status', '--porcelain'):
            parser.error('native production requires a clean committed checkout')
    output = args.output.resolve()
    output.mkdir()
    if args.action == 'register':
        register(output, args.mac.resolve(), args.linux.resolve())
    else:
        command = ['scripts/native-prepare', '--receipt', output / 'receipt.json']
        if os.sys.platform == 'linux':
            # Header validation and native compilation share the same compiler.
            command = ['scripts/runtime-floor-build', *command]
        subprocess.run(list(map(str, command)), cwd=ROOT, check=True)
        receipt = json.loads((output / 'receipt.json').read_text())
        cache = runpy.run_path(str(ROOT / 'scripts/native-prepare'))['cache_path']()
        if cache.name != receipt['native_key']:
            raise ValueError('current native inputs differ from the recorded producer key')
        config = Path(os.environ['CARGO_HOME']) / 'config.toml'
        if digest(config) != receipt['config_sha256']:
            raise ValueError('current Cargo configuration is not the recorded producer configuration')
        shutil.copy2(config, output / 'cargo-config.toml')
        for path, expected in receipt['assets'].items():
            source = cache / path
            if digest(source) != expected:
                raise ValueError('native asset changed before collection: ' + path)
            target = output / 'assets' / path
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, target)
    print(json.dumps(dict(output=str(output), action=args.action)))


if __name__ == '__main__':
    main()
