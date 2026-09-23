#!/usr/bin/env python3
"""The lock checker's own test (surfaces-review-5).

Plants call sites in scratch trees and pins each verdict: a flag before
the subcommand still needs the lock, the DuckDB extension's vendored
makefile is scanned, and its build passes only while the DuckDB Makefile
appends --locked to the flag variable both build lines read. The pgrx
wrapper restores a lock the package step rewrote, and fails.

Usage: python3 scripts/test_check_locked_calls.py
"""
import os
import pathlib
import subprocess
import sys
import tempfile

CHECKER = pathlib.Path(__file__).resolve().parent / "check_locked_calls.py"
WRAPPER = pathlib.Path(__file__).resolve().parent / "pgrx-package-locked.sh"
# A stand-in cargo: metadata passes, and `pgrx package` rewrites the lock.
FAKE_CARGO = """#!/bin/sh
if [ "$1" = pgrx ]; then echo drifted >> Cargo.lock; fi
exit 0
"""


def wrapper_restores_the_lock():
    with tempfile.TemporaryDirectory() as scratch:
        root = pathlib.Path(scratch)
        (root / "bin").mkdir()
        (root / "bin" / "cargo").write_text(FAKE_CARGO)
        (root / "bin" / "cargo").chmod(0o755)
        (root / "Cargo.lock").write_text("proven\n")
        env = dict(os.environ, PATH=f"{root / 'bin'}:{os.environ['PATH']}")
        done = subprocess.run(["bash", str(WRAPPER), "--pg-config", "x"], cwd=root, env=env,
                              capture_output=True, text=True, check=False)
        lock = (root / "Cargo.lock").read_text()
    return (done.returncode, done.stderr, lock)
VENDORED = "databases/duckdb/extension-ci-tools/makefiles/c_api_extensions/rust.Makefile"
BUILD = "\tcargo build $(CARGO_OVERRIDE_DUCKDB_RS_FLAG) --release $(TARGET_INFO)\n"


def verdict(files):
    with tempfile.TemporaryDirectory() as scratch:
        root = pathlib.Path(scratch)
        for name, text in files.items():
            (root / name).parent.mkdir(parents=True, exist_ok=True)
            (root / name).write_text(text)
        done = subprocess.run([sys.executable, str(CHECKER), "--root", str(root)],
                              capture_output=True, text=True, check=False)
    return done.returncode, done.stdout


CASES = [
    ("a flag before the subcommand needs the lock",
     {"scripts/a.sh": "cargo -q build --release\n"},
     (1, "FAIL scripts/a.sh:1: cargo -q build without --locked\n")),
    ("a toolchain and a config before the subcommand need the lock",
     {"scripts/a.sh": "cargo +1.95 --config 'a=b' test\n"},
     (1, "FAIL scripts/a.sh:1: cargo +1.95 --config 'a=b' test without --locked\n")),
    ("a locked call with a flag first passes",
     {"scripts/a.sh": "cargo -q build --locked\n"},
     (0, "ok:      every build-tool call site carries the lock\n")),
    ("the vendored DuckDB build fails without the Makefile's lock",
     {VENDORED: BUILD, "databases/duckdb/Makefile": "include x\n"},
     (1, "FAIL databases/duckdb/Makefile: CARGO_OVERRIDE_DUCKDB_RS_FLAG does not append --locked\n")),
    ("the vendored DuckDB build passes with the Makefile's lock",
     {VENDORED: BUILD, "databases/duckdb/Makefile": "CARGO_OVERRIDE_DUCKDB_RS_FLAG += --locked\n"},
     (0, "ok:      every build-tool call site carries the lock\n")),
    ("a vendored cargo build without the flag variable fails",
     {VENDORED: "\tcargo build --release\n",
      "databases/duckdb/Makefile": "CARGO_OVERRIDE_DUCKDB_RS_FLAG += --locked\n"},
     (1, f"FAIL {VENDORED}:1: cargo build without --locked\n")),
]


def main():
    bad = 0
    for name, files, want in CASES:
        got = verdict(files)
        if got != want:
            print(f"FAIL {name}: expected {want!r}, got {got!r}")
            bad = 1
    got = wrapper_restores_the_lock()
    want = (1, "pgrx-package-locked: the package step rewrote Cargo.lock; the proven lock is restored\n", "proven\n")
    if got != want:
        print(f"FAIL the pgrx wrapper: expected {want!r}, got {got!r}")
        bad = 1
    if not bad:
        print("ok:      the lock checker reads flags before the subcommand and the vendored DuckDB build, and the pgrx wrapper restores the lock")
    return bad


if __name__ == "__main__":
    sys.exit(main())
