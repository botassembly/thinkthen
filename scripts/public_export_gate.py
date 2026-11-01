"""Prepare the exported candidate from verified producer assets, then gate it."""
import hashlib
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import tempfile
import tomllib

from release_go import home as go_home, version as go_version
from native_producer import HOST_ASSETS, validate_receipt

ROOT = Path(__file__).resolve().parent.parent
NODE_VERSION = '22.22.3'
ASSETS = set.union(*HOST_ASSETS.values())


def run(*args, **kwargs):
    return subprocess.run(list(map(str, args)), cwd=ROOT, check=True, **kwargs)


def output(*args):
    return subprocess.check_output(list(map(str, args)), cwd=ROOT, text=True).strip()


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(evidence, registration, assets):
    node = Path(os.environ['THINKTHEN_NODE_HOME']) / 'bin/node'
    versions = dict(node=output(node, '--version'), go=output(go_home() / 'bin/go', 'env', 'GOVERSION'))
    if versions != dict(node='v' + NODE_VERSION, go='go' + go_version()):
        raise ValueError('prepare the exact candidate Node and Go runtimes')
    data = json.loads(registration.read_text())
    receipts = data['production_receipts']
    cache = Path(output('scripts/native-prepare', '--path'))
    if cache.name != data['native_key'] or data['native_key_inputs_exact'] is not True:
        raise ValueError('producer native inputs differ from this candidate')
    if set(data['five_assets']) != ASSETS:
        raise ValueError('producer registration lacks the complete native pair')
    produced = {}
    for name, receipt in receipts.items():
        validate_receipt(receipt, name)
        if receipt['native_key'] != cache.name:
            raise ValueError('producer receipt identity differs from registration')
        produced.update(receipt['assets'])
    if produced != data['five_assets']:
        raise ValueError('producer receipts do not certify the registered pair')
    config = Path(os.environ['CARGO_HOME']) / 'config.toml'
    if digest(config) != receipts['linux']['config_sha256']:
        raise ValueError('Cargo configuration differs from the genuine Linux producer')
    for name, expected in data['five_assets'].items():
        source = assets / name
        if digest(source) != expected:
            raise ValueError('producer asset hash differs: ' + name)
        target = cache / name
        if target.exists() and digest(target) != expected:
            raise ValueError('cached native asset conflicts with producer: ' + name)
        if not target.exists():
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, target)
    pair = evidence / 'consumer-pair.json'
    run('scripts/native-pair', 'export', pair)
    run('scripts/native-pair', 'import', pair, cache)
    closures = {}
    for command in ('native-test-assets', 'native-abi-assets'):
        run('scripts/' + command, 'prepare')
        environment = output('scripts/' + command, 'env')
        selected = Path(shlex.split(environment.splitlines()[0].split('=', 1)[1])[0])
        folder = selected.parent if command == 'native-test-assets' else selected
        identity = json.loads((folder / 'identity.json').read_text())
        if identity['inputs']['native_key'] != cache.name:
            raise ValueError('fixture closure uses another native key')
        for name, expected in identity['files'].items():
            if digest(folder / name) != expected:
                raise ValueError('fixture closure hash differs: ' + name)
        closures[command] = identity
    pin = tomllib.loads((ROOT / 'Cargo.toml').read_text())['workspace']['metadata']['supply-chain']['advisory-revision']
    tools = Path.home() / '.local/share/thinkthen-build-tools'
    advisory_root = tools / 'advisory-dbs' / pin
    matches = [p for p in advisory_root.glob('advisory-db-*') if (p / '.git').exists()
               and output('git', '-C', p, 'remote', 'get-url', 'origin').removesuffix('.git').lower()
               == 'https://github.com/rustsec/advisory-db']
    if len(matches) != 1 or output('git', '-C', matches[0], 'rev-parse', 'HEAD') != pin or output('git', '-C', matches[0], 'status', '--porcelain'):
        raise ValueError('prepare the exact clean advisory database before the candidate gate')
    receipt = dict(candidate_revision=output('git', 'rev-parse', 'HEAD'),
                   producer_registration=data, consumer_pair=json.loads(pair.read_text()),
                   cargo_config_sha256=digest(config), fixture_closures=closures, tools=versions)
    (evidence / 'prepared.json').write_text(json.dumps(receipt, indent=2) + '\n')


def main():
    mode, evidence, registration, assets, cargo_home, swift, postgres, patterns = sys.argv[1:]
    evidence, registration, assets, cargo_home, patterns = map(Path, (evidence, registration, assets, cargo_home, patterns))
    if mode == 'verify':
        verify(evidence, registration, assets)
        return
    if sys.platform != 'linux':
        raise ValueError('strict candidate qualification requires Linux')
    if output('git', 'status', '--porcelain'):
        raise ValueError('candidate checkout must be clean before preparation')
    evidence.mkdir(exist_ok=True)
    if (evidence / 'target').exists():
        raise ValueError('candidate gate requires a fresh target at its final absolute path')
    data = json.loads(registration.read_text())
    floor = data['production_receipts']['linux']['floor_image']
    for image in (floor, swift, postgres):
        if not image.startswith('sha256:') or output('docker', 'image', 'inspect', image, '--format', '{{.Id}}') != image:
            raise ValueError('prepare the exact immutable runtime image: ' + image)
    tools = Path.home() / '.local/share/thinkthen-build-tools'
    environment = dict(os.environ, CARGO_HOME=str(cargo_home), THINKTHEN_FLOOR_IMAGE=floor,
                       THINKTHEN_SWIFT_IMAGE=swift, THINKTHEN_POSTGRES_TEST_IMAGE=postgres,
                       THINKTHEN_DOCKER_IN_FLOOR='1', THINKTHEN_SDK_GATE_RUNNER='scripts/linux-binding-gate',
                       THINKTHEN_GATE_STRICT='1', PYTHONDONTWRITEBYTECODE='1',
                       THINKTHEN_NODE_HOME=str(tools / 'gate-node' / NODE_VERSION),
                       THINKTHEN_GO_HOME=str(go_home()),
                       CARGO_TARGET_DIR=str(evidence / 'target'),
                       PATH=str(tools / 'cargo-deny/0.20.2-glibc2.28-rust1.95.0/installed/bin') + ':' + os.environ['PATH'])
    environment.pop('THINKTHEN_FLOOR_BUILD', None)
    # Everything the floor reads lives in its already supported TMPDIR mount.
    scratch = Path(tempfile.mkdtemp(prefix='candidate-', dir='/tmp'))
    failed = True
    inside_evidence = scratch / 'evidence'
    inside_evidence.mkdir()
    try:
        shutil.copy2(registration, scratch / 'registration.json')
        shutil.copytree(assets, scratch / 'assets')
        shutil.copy2(patterns, scratch / 'patterns')
        temporary_alias = scratch / 'temporary'
        temporary_alias.symlink_to(scratch, target_is_directory=True)
        environment.update(TMPDIR=str(temporary_alias), THINKTHEN_PRIVATE_PATTERNS=str(scratch / 'patterns'))
        arguments = [str(inside_evidence), str(scratch / 'registration.json'), str(scratch / 'assets'),
                     str(cargo_home), swift, postgres, str(scratch / 'patterns')]
        run('scripts/runtime-floor-build', 'scripts/public-export-gate', '--inside', *arguments, env=environment)
        failed = False
    finally:
        for receipt in inside_evidence.iterdir():
            shutil.copy2(receipt, evidence / receipt.name)
        if failed:
            (evidence / 'failed-scratch.txt').write_text(str(scratch) + '\n')
        else:
            shutil.rmtree(scratch)


if __name__ == '__main__':
    main()
