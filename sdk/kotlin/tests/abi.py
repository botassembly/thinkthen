"""Exercise the Kotlin public constructor against the shared packaged ABI fixtures."""
import json
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
                    'consumer.AbiAndCodesKt', 'abi', str(case['accept']).lower()], check=True)
print('Kotlin packaged ABI cases passed')

# Feature: Java and Kotlin report load failures through each public constructor.
import hashlib
import os
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
