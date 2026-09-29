"""Keep the Dart binding, consumers, and Flutter host source visible."""
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
files = [path for path in root.rglob('*.dart')
         if not any(part in {'.dart_tool', 'build', 'scratch', 'logs'} for part in path.parts)]
actual = sum(sum(bool(line.strip()) for line in path.read_text().splitlines()) for path in files)
expected = json.loads((root / 'ratchet.json').read_text())['dart_nonblank_lines']
if not files or actual != expected:
    raise SystemExit(f'DART_RATCHET_MISMATCH actual={actual} expected={expected}')
print(f'DART_RATCHET_PASS {len(files)} files {actual} nonblank lines')
