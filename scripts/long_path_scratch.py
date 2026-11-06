"""Scratch folders whose cleanup reaches every path a native tool wrote on Windows."""
from contextlib import contextmanager
import ntpath
import os
from pathlib import Path
import tempfile

EXTENDED = '\\\\?\\'


def extended(folder):
    """Give a drive-letter folder the extended-length prefix and leave others unchanged."""
    folder = str(folder)
    return EXTENDED + folder if len(ntpath.splitdrive(folder)[0]) == 2 else folder


@contextmanager
def long_path_scratch(prefix, label):
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
            raise RuntimeError(label + ' scratch cleanup failed; retained '
                               + str(scratch) + ': ' + str(error)) from error
