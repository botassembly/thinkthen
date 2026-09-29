"""Reject private workspace paths and planted key markers in shipped sources."""
from pathlib import Path
import re
import tempfile

root = Path(__file__).resolve().parents[1]
blocked = re.compile(r'/home/ian/workspace/|PRIVATE_PATH_MARKER|PRIVATE_KEY_MARKER|sk-private-[A-Za-z0-9]+')


def check(paths):
    return [str(path) for path in paths if blocked.search(path.read_text(errors='replace'))]


sources = [path for path in root.rglob('*') if path.is_file()
           and not any(part in {'.dart_tool', 'scratch', 'logs', 'build'} for part in path.parts)
           and path != Path(__file__) and path.suffix in {'.dart', '.py', '.sh', '.md', '.yaml', '.json'}]
found = check(sources)
if found:
    raise SystemExit(f'PRIVATE_SOURCE_REFUSED {found}')
with tempfile.TemporaryDirectory() as directory:
    planted = Path(directory) / 'planted.txt'
    for marker in ('/home/ian/workspace/private/file', 'PRIVATE_KEY_MARKER', 'sk-private-SECRET123'):
        planted.write_text(marker)
        if not check([planted]):
            raise SystemExit(f'PRIVATE_PLANT_NOT_REFUSED {marker}')
print(f'PRIVATE_SOURCE_PASS {len(sources)} files, 3 planted negatives')
