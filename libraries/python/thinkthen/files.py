"""Explicit selections and immutable source records; the native reader owns parsing."""

import json
import os
from dataclasses import dataclass
from . import _thinkthen


@dataclass(frozen=True, repr=False)
class SourceRecord:
    record: str
    file: str
    first_line: int
    last_line: int

    def __repr__(self):
        return f"SourceRecord(first_line={self.first_line}, last_line={self.last_line}, content=<withheld>)"


@dataclass(frozen=True, repr=False)
class FileSelection:
    paths: tuple[str, ...]
    unit: str = "line"
    window: int | None = None

    def __iter__(self):
        return (SourceRecord(**json.loads(row)) for row in _thinkthen._read_files(self.paths, self.unit, self.window))

    def __repr__(self):
        return f"FileSelection(unit={self.unit!r}, window={self.window!r}, paths=<withheld>)"


def read_files(path_or_paths, *, unit="line", window=None):
    """Select files/folders explicitly; iterate or pass to any judging function.

    Every consumption uses the native reader. A string passed directly to a
    judging function remains text. Results keep provenance beside the value.
    """
    if isinstance(path_or_paths, (str, os.PathLike)):
        paths = (os.fspath(path_or_paths),)
    else:
        if isinstance(path_or_paths, (set, frozenset)):
            raise _thinkthen.UsageError("source paths require an ordered collection")
        paths = tuple(os.fspath(path) for path in path_or_paths)
    if any(not isinstance(path, str) for path in paths):
        raise _thinkthen.UsageError("source paths are text")
    return FileSelection(paths, unit, window)
