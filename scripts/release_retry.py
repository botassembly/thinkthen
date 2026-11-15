"""Source inventory shared by ordinary release preparation and publication."""
from pathlib import Path
import subprocess

REPOSITORY = 'botassembly/thinkthen'
ROOT = Path(__file__).resolve().parent.parent


def git(*args, root=ROOT):
    return subprocess.check_output(['git', *args], cwd=root)


def manifest(revision, root=ROOT):
    """Hash every tracked byte and mode; preserve uninitialized gitlinks."""
    entries = {}
    for record in git('ls-tree', '-rz', '--full-tree', revision, root=root).split(b'\0'):
        if not record:
            continue
        metadata, name = record.split(b'\t', 1)
        mode, kind, oid = metadata.decode().split()
        path = name.decode('utf-8')
        entries[path] = {'mode': mode, 'kind': kind, 'object': oid}
    return {'revision': revision, 'tree': git('rev-parse', revision + '^{tree}', root=root).decode().strip(),
            'files': entries}
