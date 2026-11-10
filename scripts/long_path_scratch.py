"""Scratch folders whose cleanup reaches every path a native tool wrote on Windows."""
from contextlib import contextmanager
import json
import ntpath
import os
from pathlib import Path
import tempfile
import stat
import sys

EXTENDED = '\\\\?\\'


def extended(folder):
    """Give a drive-letter folder the extended-length prefix and leave others unchanged."""
    folder = str(folder)
    return EXTENDED + folder if len(ntpath.splitdrive(folder)[0]) == 2 else folder


def residuals(root, limit=64, path_limit=512, byte_limit=32768):
    """Bounded relative metadata only; never traverse links or reparse points."""
    pending, entries = [('', os.fspath(root))], []
    truncated = False
    def report():
        result = dict(entries=entries, truncated=truncated or bool(pending),
                      limits=dict(entries=limit, path_characters=path_limit, bytes=byte_limit))
        while len(json.dumps(result).encode()) > byte_limit and entries:
            entries.pop()
            result['truncated'] = True
        return result
    while pending and len(entries) < limit:
        prefix, folder = pending.pop()
        try:
            metadata = os.stat(folder, follow_symlinks=False)
            if stat.S_ISLNK(metadata.st_mode) or getattr(metadata, 'st_file_attributes', 0) & 0x400:
                entries.append(dict(path=prefix[:path_limit], path_truncated=len(prefix) > path_limit, type='reparse'))
                continue
            with os.scandir(folder) as children:
                for child in children:
                    if len(entries) >= limit:
                        truncated = True
                        return report()
                    relative = prefix + child.name
                    try:
                        info = child.stat(follow_symlinks=False)
                        reparse = bool(getattr(info, 'st_file_attributes', 0) & 0x400)
                        kind = ('reparse' if reparse else 'symlink' if stat.S_ISLNK(info.st_mode)
                                else 'directory' if stat.S_ISDIR(info.st_mode) else 'file')
                        entries.append(dict(path=relative[:path_limit], path_truncated=len(relative) > path_limit, type=kind))
                        if kind == 'directory':
                            pending.append((relative + '/', child.path))
                    except OSError as error:
                        entries.append(dict(path=relative[:path_limit], path_truncated=len(relative) > path_limit, error=error.errno, winerror=getattr(error, 'winerror', None)))
        except OSError as error:
            entries.append(dict(path=prefix[:path_limit], path_truncated=len(prefix) > path_limit, error=error.errno, winerror=getattr(error, 'winerror', None)))
    return report()


@contextmanager
def long_path_scratch(prefix, label, *, process_witness=None):
    # npm, Go, Swift and Zig caches can pass 260 characters. Without the
    # extended-length prefix, Windows reports them missing, tempfile skips them,
    # and their folder stays full. Tools still receive the ordinary path.
    root = extended(Path(tempfile.gettempdir()).resolve()) if os.name == 'nt' else None
    temporary = tempfile.TemporaryDirectory(prefix=prefix, dir=root)
    scratch = Path(temporary.name.removeprefix(EXTENDED))
    try:
        yield scratch
    finally:
        try:
            temporary.cleanup()
        except OSError as error:
            # Diagnostics must not replace the original failure or trigger cleanup again.
            try:
                witness = dict(kind='scratch-cleanup-failure', residuals=residuals(temporary.name),
                               errno=error.errno, winerror=getattr(error, 'winerror', None))
                if process_witness:
                    try:
                        witness['owned_processes'] = process_witness()
                    except Exception as diagnostic_error:  # noqa: BLE001 - preserve the primary cleanup failure
                        witness['owned_processes_error'] = type(diagnostic_error).__name__
                print(json.dumps(witness, sort_keys=True), file=sys.stderr, flush=True)
            except Exception as diagnostic_error:  # noqa: BLE001 - preserve the primary cleanup failure
                try:
                    print('Scratch witness unavailable: ' + type(diagnostic_error).__name__, file=sys.stderr, flush=True)
                except Exception:  # noqa: BLE001,S110 - even unavailable stderr cannot replace cleanup failure
                    pass  # The original cleanup failure must still propagate.
            raise RuntimeError(label + ' scratch cleanup failed; retained '
                               + str(scratch) + ': ' + str(error)) from error
