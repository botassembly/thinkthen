"""Extend shared forwarding fixtures with Ruby decoder compatibility cases."""
import argparse
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys


def prepare(root, engine, fixtures, output):
    fixture_source = runpy.run_path(str(root / 'scripts/native-fixtures'))['source']
    registry = json.loads((root / 'schemas/abi.json').read_text())
    manifest = json.loads(fixtures.read_text())
    output.mkdir(parents=True, exist_ok=True)
    suffix = engine.suffix
    version = next(case['version'] for case in manifest['cases'] if case['name'] == 'current')
    for name, field in (('unknown-function', 'function'), ('unknown-status', 'status'), ('unknown-error', 'error_code')):
        case = {'name': name, 'version': version, 'forward': True, 'accept': False}
        source = fixture_source(case, registry, engine)
        original = 'return fixture_function(call, out, error);'
        replacement = ('int32_t result = fixture_function(call, out, error); '
                       f'if (result == THINKTHEN_OK) out->{field} = 2147483647; return result;')
        if source.count(original) != 1:
            raise ValueError('shared call-next forwarding declaration changed')
        code = output / (name + '.c')
        library = output / (name + suffix)
        code.write_text(source.replace(original, replacement))
        command = [os.environ.get('CC', 'cc'), '-std=c11', '-Wall', '-Wextra', '-Werror',
                   '-I', str(root / 'crates/thinkthen-ffi/include'), str(code), '-o', str(library)]
        command += ['-dynamiclib', '-pthread'] if sys.platform == 'darwin' else ['-shared', '-fPIC', '-pthread', '-ldl']
        subprocess.run(command, check=True)
        code.unlink()
        manifest['cases'].append({**case, 'library': str(library)})
    (output / 'manifest.json').write_text(json.dumps(manifest) + '\n')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--engine', required=True, type=Path)
    parser.add_argument('--fixtures', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    args = parser.parse_args()
    prepare(Path(__file__).resolve().parents[3], args.engine.resolve(strict=True),
            args.fixtures.resolve(strict=True), args.output.resolve())
