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
class Located:
    source: SourceRecord
    value: object

    def __repr__(self):
        return "Located(source=<withheld>, value=<withheld>)"


@dataclass(frozen=True, repr=False)
class FileSelection:
    paths: tuple[str, ...]
    unit: str = "line"
    window: int | None = None

    def _json(self):
        return json.dumps(dict(paths=self.paths, unit=self.unit, window=self.window))

    def __iter__(self):
        return (SourceRecord(**json.loads(row)) for row in _thinkthen._read_files(self._json()))

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
    if unit not in ("line", "window", "file") or (unit == "window" and
            (type(window) is not int or window <= 0)) or (unit != "window" and window is not None):
        raise _thinkthen.UsageError("reader window requires unit window and a positive whole number")
    return FileSelection(paths, unit, window)


def rebuild(row):
    if row is None:
        return None
    if isinstance(row, list):
        return [rebuild(value) for value in row]
    if "record" in row and "file" in row:
        source = SourceRecord(*(row[key] for key in ("record", "file", "first_line", "last_line")))
        return Located(source, row["value"])
    if "edges" in row:
        return [dict(edge, source=rebuild(edge["source"]), target=rebuild(edge["target"])) for edge in row["edges"]]
    return row


def call(engine, verb, question, selection, batch, context, deadline, token):
    from . import _mapped
    done = engine._engine._source(verb, question, selection._json(), batch, context, deadline, token)
    return _mapped(done, lambda text: rebuild(json.loads(text)))


def spec_source(ask):
    if isinstance(ask, type):
        from . import pydantic as adapter
        ask = adapter.question_set(ask)
    return json.dumps(ask) if isinstance(ask, dict) else _thinkthen._spec_source(os.fspath(ask))
