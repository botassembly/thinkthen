"""Feature: an installed C# consumer refuses a malformed current-ABI row."""
from pathlib import Path
import platform
import os
import runpy
import tempfile
import shutil
import subprocess
import sys

folder, fixtures = map(Path, sys.argv[1:])
name = 'libthinkthen_ffi.dylib' if platform.system() == 'Darwin' else 'libthinkthen_ffi.so'
engine, = folder.rglob(name)
saved = engine.with_suffix(engine.suffix + '.original')
root = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(root / 'scripts'))
producer = runpy.run_path(str(root / 'scripts/abi-minor-check'))
temporary = tempfile.TemporaryDirectory(prefix='csharp-row-fixtures-')
selected = os.environ.get('THINKTHEN_DECODER_CASE')
companions = {} if selected else {kind: fixtures / kind / name for kind in ('invalid-json', 'unanswered-answer', 'unknown-function')}
companions.update(producer['decoder_fixtures'](engine, Path(temporary.name), selected))
engine.rename(saved)
failures = []
try:
    for kind, replacement in companions.items():
        assert replacement.is_file(), 'Missing required current-ABI malformed row fixture: ' + kind
        # Replace the inode so macOS never reuses a signed image that we overwrote.
        engine.unlink(missing_ok=True)
        shutil.copy2(replacement, engine)
        result = subprocess.run(['dotnet', str(folder / 'Consumer.dll'), '--malformed-row', kind], check=False, timeout=60)
        print(f'C# malformed {kind}: exit {result.returncode}', flush=True)
        if result.returncode: failures.append(kind)
finally:
    engine.unlink(missing_ok=True)
    saved.rename(engine)
    temporary.cleanup()
assert not failures, 'C# malformed row refusal failed: ' + ', '.join(failures)
