#!/usr/bin/env python3
"""The shared ctypes harness for the fourth review's DuckDB probes.

The venv's bundled engine exports the full stable C API, so the probes
drive databases and connections exactly the way the reviewer's churn.c
did: a host holds a `duckdb_database`, opens and closes its connections,
and watches which database's tables a relate reads. Nothing here needs
a key; the stand-in's null backend answers the asks.

Each probe prints PASS, FAIL, or SKIP with the observed evidence, and
exits nonzero on FAIL, so a suite can run fail-then-pass honestly.
"""

from __future__ import annotations

import ctypes
import json
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EXTENSION = ROOT / "build" / "release" / "thinkthen.duckdb_extension"
CASES = ROOT.parent.parent / "conformance" / "conformance.json"

DUCKDB_SUCCESS = 0


class Harness:
    """The C door: open, connect, run, and hold databases by handle."""

    def __init__(self) -> None:
        os.environ.setdefault("ENGINE_NULL", "1")
        import sysconfig

        candidates = sorted(
            (ROOT / "configure" / "venv").glob(
                "lib/python*/site-packages/_duckdb*.so"
            )
        )
        if not candidates:
            raise SystemExit("no bundled duckdb library under configure/venv")
        self.lib = ctypes.CDLL(str(candidates[-1]))
        for name, args in [
            ("duckdb_open_ext", [ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p]),
            ("duckdb_connect", [ctypes.c_void_p, ctypes.POINTER(ctypes.c_void_p)]),
            ("duckdb_disconnect", [ctypes.POINTER(ctypes.c_void_p)]),
            ("duckdb_close", [ctypes.POINTER(ctypes.c_void_p)]),
            ("duckdb_query", [ctypes.c_void_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]),
            ("duckdb_destroy_result", [ctypes.POINTER(ctypes.c_void_p)]),
            ("duckdb_result_error", [ctypes.POINTER(ctypes.c_void_p)]),
            ("duckdb_value_is_null", [ctypes.POINTER(ctypes.c_void_p), ctypes.c_ulonglong, ctypes.c_ulonglong]),
            ("duckdb_value_varchar", [ctypes.POINTER(ctypes.c_void_p), ctypes.c_ulonglong, ctypes.c_ulonglong]),
            ("duckdb_row_count", [ctypes.POINTER(ctypes.c_void_p)]),
            ("duckdb_column_count", [ctypes.POINTER(ctypes.c_void_p)]),
        ]:
            fn = getattr(self.lib, name)
            fn.argtypes = args
            fn.restype = ctypes.c_void_p if name not in {
                "duckdb_query", "duckdb_connect", "duckdb_open_ext",
                "duckdb_value_is_null",
            } else (
                ctypes.c_int if name in {"duckdb_query", "duckdb_connect", "duckdb_open_ext"}
                else ctypes.c_bool
            )
        self.lib.duckdb_value_varchar.restype = ctypes.c_char_p
        self.lib.duckdb_result_error.restype = ctypes.c_char_p
        self.lib.duckdb_row_count.restype = ctypes.c_ulonglong
        self.lib.duckdb_column_count.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
        self.lib.duckdb_column_count.restype = ctypes.c_ulonglong

    def config(self, **settings: str) -> ctypes.c_void_p:
        """A config with allow_unsigned_extensions and the given settings."""
        self.lib.duckdb_create_config.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
        self.lib.duckdb_create_config.restype = ctypes.c_int
        config = ctypes.c_void_p()
        if self.lib.duckdb_create_config(ctypes.byref(config)) != DUCKDB_SUCCESS:
            raise SystemExit("no config")
        self.lib.duckdb_set_config.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p,
        ]
        self.lib.duckdb_set_config.restype = ctypes.c_int
        for key, value in {"allow_unsigned_extensions": "true", **settings}.items():
            if self.lib.duckdb_set_config(config, key.encode(), value.encode()) != DUCKDB_SUCCESS:
                raise SystemExit(f"config refused {key}={value}")
        return config

    def open(self, path: str | None = None, **settings: str) -> ctypes.c_void_p:
        """A held database handle; the caller closes it with close()."""
        config = self.config(**settings)
        self.lib.duckdb_open_ext.argtypes = [
            ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p,
        ]
        self.lib.duckdb_open_ext.restype = ctypes.c_int
        database = ctypes.c_void_p()
        name = path.encode() if path is not None else None
        state = self.lib.duckdb_open_ext(name, ctypes.byref(database), config)
        if state != DUCKDB_SUCCESS or not database.value:
            raise SystemExit(f"open refused {path or ':memory:'} ({state})")
        return database

    def close(self, database: ctypes.c_void_p) -> None:
        self.lib.duckdb_close(ctypes.byref(database))

    def connect(self, database: ctypes.c_void_p) -> ctypes.c_void_p:
        connection = ctypes.c_void_p()
        if self.lib.duckdb_connect(database, ctypes.byref(connection)) != DUCKDB_SUCCESS:
            raise SystemExit("connect refused")
        return connection

    def disconnect(self, connection: ctypes.c_void_p) -> None:
        self.lib.duckdb_disconnect(ctypes.byref(connection))

    def run(self, connection: ctypes.c_void_p, sql: str) -> tuple[bool, str, list[list[str | None]]]:
        """One statement: (ok, error, rows-as-strings)."""
        result = ctypes.c_void_p()
        state = self.lib.duckdb_query(connection, sql.encode(), ctypes.byref(result))
        if state != DUCKDB_SUCCESS:
            # A failed query's flat-API result is not destroyed: the
            # bundled engine's destroy-after-error corrupts the caller's
            # heap in this ctypes harness (observed as free() of a Python
            # type object), so the error result leaks instead — bounded
            # by the probe's own trial count.
            text = self.lib.duckdb_result_error(ctypes.byref(result)) or b"the query failed"
            return False, text.decode(errors="replace"), []
        rows = int(self.lib.duckdb_row_count(ctypes.byref(result)))
        columns = int(self.lib.duckdb_column_count(ctypes.byref(result)))
        out: list[list[str | None]] = []
        for row in range(rows):
            cells = []
            for column in range(columns):
                if self.lib.duckdb_value_is_null(ctypes.byref(result), column, row):
                    cells.append(None)
                    continue
                raw = self.lib.duckdb_value_varchar(ctypes.byref(result), column, row)
                cells.append(raw.decode(errors="replace") if raw else "")
            out.append(cells)
        self.lib.duckdb_destroy_result(ctypes.byref(result))
        return True, "", out

    def load(self, connection: ctypes.c_void_p) -> None:
        ok, error, _ = self.run(connection, f"LOAD '{EXTENSION}'")
        if not ok:
            raise SystemExit(f"LOAD refused: {error}")


def relate_rules(case: dict) -> str:
    names = ",".join(f"'{rule['name']}'" for rule in case["question"]["relations"])
    return f"[{names}]"


def case_by_id(name: str) -> dict:
    cases = json.loads(CASES.read_text())["cases"]
    for case in cases:
        if case["id"] == name:
            return case
    raise SystemExit(f"no conformance case {name}")


def verdict(label: str, ok: bool, detail: str) -> int:
    print(("PASS   " if ok else "FAIL   ") + label)
    print("       " + detail.replace("\n", "\n       "))
    return 0 if ok else 1


def finish(status: int) -> None:
    """Exit without CPython finalization: the interpreter's teardown GC
    crashes in this harness (a weakref trips over the loaded engine's
    teardown order), and the probes' verdicts are already printed. The
    reviewer's C harness exits through the C runtime the same way.
    """
    sys.stdout.flush()
    sys.stderr.flush()
    os._exit(status)
