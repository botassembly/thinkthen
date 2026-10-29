"""Exercise the Kotlin public constructor against the shared packaged ABI fixtures."""
import json
import os
import runpy
from pathlib import Path
import subprocess
import sys
import zipfile

manifest, classpath, java, platform, library, scratch = sys.argv[1:]
entries = classpath.split(':')
jar, = [Path(item) for item in entries if Path(item).name.startswith('thinkthen-java-')]
for case in json.loads(Path(manifest).read_text())['cases']:
    replacement = Path(scratch) / ('kotlin-abi-' + case['name'] + '.jar')
    with zipfile.ZipFile(jar) as source, zipfile.ZipFile(replacement, 'w') as target:
        for entry in source.infolist():
            data = Path(case['library']).read_bytes() if entry.filename == f'io/thinkthen/native/{platform}/{library}' else source.read(entry)
            target.writestr(entry, data)
    replaced = ':'.join(str(replacement) if item == str(jar) else item for item in entries)
    subprocess.run([java, '--enable-native-access=ALL-UNNAMED', f'-Duser.home={scratch}/home', '-cp', replaced,
                    'consumer.AbiAndCodesKt', 'abi', str(case['accept']).lower()], check=True, timeout=60)
    if not case['accept']:
        subprocess.run([java, '--enable-native-access=ALL-UNNAMED', f'-Duser.home={scratch}/home', '-cp', replaced,
                        'JavaAbiRefusalCases'], check=True, timeout=60)
print('Kotlin packaged ABI cases passed')

# Feature: current-ABI engines returning malformed rows are refused by both APIs.
failures = []
root = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(root / 'scripts'))
producer = runpy.run_path(str(root / 'scripts/abi-minor-check'))
original = Path(scratch) / 'row-original' / library
original.parent.mkdir(parents=True)
with zipfile.ZipFile(jar) as source:
    original.write_bytes(source.read(f'io/thinkthen/native/{platform}/{library}'))
selected = os.environ.get('THINKTHEN_DECODER_CASE')
if selected == 'unknown-family':
    subprocess.run([java, '--enable-native-access=ALL-UNNAMED', f'-Duser.home={scratch}/home', '-cp', classpath,
                    'consumer.AbiAndCodesKt', 'registry', str(root / 'schemas/contract.json')], check=True, timeout=60)
companions = {} if selected else {kind: Path(os.environ['THINKTHEN_MALFORMED_ROW_FIXTURES']) / kind / library
              for kind in ('invalid-json', 'unanswered-answer', 'unknown-function')}
companions.update(producer['decoder_fixtures'](original, Path(scratch) / 'decoder-fixtures', selected))
for kind, malformed in companions.items():
    assert malformed.is_file(), 'Missing required current-ABI malformed row fixture: ' + kind
    replacement = Path(scratch) / ('jvm-malformed-' + kind + '.jar')
    with zipfile.ZipFile(jar) as source, zipfile.ZipFile(replacement, 'w') as target:
        for entry in source.infolist():
            target.writestr(entry, malformed.read_bytes() if entry.filename == f'io/thinkthen/native/{platform}/{library}' else source.read(entry))
    replaced = ':'.join(str(replacement) if item == str(jar) else item for item in entries)
    commands = (['consumer.AbiAndCodesKt', 'family'],) if kind == 'unknown-family' else (['JavaAbiRefusalCases', 'row', kind], ['consumer.AbiAndCodesKt', 'row', kind])
    for command in commands:
        result = subprocess.run([java, '--enable-native-access=ALL-UNNAMED', f'-Duser.home={scratch}/home', '-cp', replaced, *command], check=False, timeout=60)
        if result.returncode: failures.append(kind + '/' + command[0])
assert not failures, 'Malformed row refusal failed: ' + ', '.join(failures)
print('Java and Kotlin reject every current-ABI malformed row mode')


# Feature: Java and Kotlin report load failures through each public constructor.
import hashlib
import platform as host
resource = f'io/thinkthen/native/{platform}/{library}'
for failure in ('missing', 'broken'):
    replacement = Path(scratch) / ('jvm-load-' + failure + '.jar')
    broken = b'not a shared library\n'
    with zipfile.ZipFile(jar) as source, zipfile.ZipFile(replacement, 'w') as target:
        for entry in source.infolist():
            if entry.filename == resource:
                if failure == 'missing': continue
                data = broken
            else: data = source.read(entry)
            target.writestr(entry, data)
    replaced = ':'.join(str(replacement) if item == str(jar) else item for item in entries)
    cache = (Path(scratch) / 'home/Library/Caches' if host.system() == 'Darwin'
             else Path(os.environ.get('XDG_CACHE_HOME', str(Path(scratch) / 'home/.cache'))))
    attempted = '/' + resource if failure == 'missing' else str(cache / 'thinkthen/native' / hashlib.sha256(broken).hexdigest() / library)
    for surface in (['JavaLoaderFailureCases', 'load'], ['consumer.AbiAndCodesKt', 'abi', 'true']):
        result = subprocess.run([java, '--enable-native-access=ALL-UNNAMED', f'-Duser.home={scratch}/home', '-cp', replaced, *surface], text=True, capture_output=True)
        output = result.stdout + result.stderr
        assert result.returncode != 0 and attempted in output, (failure, surface, output)
        assert 'Cannot load' in output and 'usage.abi' not in output and 'incompatible' not in output, output
        print(f'{surface[0]}: {failure} reports load failure and attempted path')
