"""Run installed C ownership, registry and ABI refusal consumers."""
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import tempfile

root, flags, engine = Path(sys.argv[1]), shlex.split(sys.argv[2]), Path(sys.argv[3])
with tempfile.TemporaryDirectory(prefix='thinkthen-c-boundaries-') as temporary:
    scratch = Path(temporary)
    codes = json.loads((root / 'schemas/contract.json').read_text())['codes']
    (scratch / 'test_registry.h').write_text(
        'static const int32_t registry_codes[] = {' + ','.join(str(code['value']) for code in codes) + '};\n'
        'static const char *registry_names[] = {' + ','.join(json.dumps(code['name']) for code in codes) + '};\n'
        'static const char *registry_families[] = {' + ','.join(json.dumps(code['family']) for code in codes) + '};\n')
    consumer = scratch / 'typed'
    subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-UNDEBUG', '-I' + str(scratch),
        root / 'sdk/c/tests/typed_consumer.c', root / 'sdk/c/tests/typed_api_driver.c', *flags, '-o', consumer], check=True)
    subprocess.run([consumer], check=True)
    clang = shutil.which('clang')
    if clang is None:
        print('C caller sanitizers: skipped; clang is unavailable')
    else:
        sanitized = scratch / 'typed-sanitized'
        subprocess.run([clang, '-std=c11', '-Wall', '-Wextra', '-Werror', '-UNDEBUG',
            '-fsanitize=address,undefined', '-fno-omit-frame-pointer', '-I' + str(scratch),
            root / 'sdk/c/tests/typed_consumer.c', root / 'sdk/c/tests/typed_api_driver.c', *flags,
            '-o', sanitized], check=True)
        subprocess.run([sanitized], check=True, env=dict(os.environ, ASAN_OPTIONS="halt_on_error=1", UBSAN_OPTIONS="halt_on_error=1"))
        print('C caller address/undefined sanitizers pass; Rust library is uninstrumented')

    if os.environ.get('THINKTHEN_NATIVE_FIXTURES'):
        fixture = Path(os.environ['THINKTHEN_NATIVE_FIXTURES'])
        assert (fixture / 'manifest.json').is_file(), fixture
    else:
        fixture = scratch / 'fixtures'
        subprocess.run([root / 'scripts/native-fixtures', '--engine', engine, '--output', fixture], check=True)
    abi = scratch / 'abi'
    dynamic = [] if sys.platform == 'darwin' else ['-ldl']
    subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-UNDEBUG',
        root / 'crates/thinkthen-ffi/tests/abi-consumer.c', *flags, *dynamic, '-o', abi], check=True)
    exports = [record['name'] for record in json.loads((root / 'schemas/abi.json').read_text())['exports']]
    for case in json.loads((fixture / 'manifest.json').read_text())['cases']:
        result = subprocess.check_output([abi, case['library'], *exports], text=True).strip()
        assert result == ('accepted' if case['accept'] else 'usage.abi'), (case['name'], result)
    print('installed C: sized ownership, registry and ABI refusal pass')
