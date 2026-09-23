#!/usr/bin/env python3
"""Every build-tool call site in the scripts carries the lock.

A cargo, maturin, or napi call that can resolve dependencies must pass
`--locked` on the same logical line (napi: `--cargo-flags=--locked`). The
offline switch is not the lock: CARGO_NET_OFFLINE=true still rewrites a
drifted Cargo.lock from the local cache (surfaces-review-5). cargo-pgrx
0.17 has no lock flag, so its one call lives in
scripts/pgrx-package-locked.sh, which proves the lock before and after;
a bare `cargo pgrx package` anywhere else fails this check.

Usage: python3 scripts/check_locked_calls.py [--root DIR]
Exit 0 when clean, 1 with one `path:line: why` per call site.
"""
import pathlib
import re
import sys

SCANNED_DIRS = ("libraries", "databases", "tools", "scripts", "sdlc/scripts", "contract", "standin")
SKIPPED_PARTS = {"target", "node_modules", ".venv", ".runtimes", "vendor", "dist", "build"}
SCANNED_NAMES = re.compile(r"(\.sh|Makefile|Makevars\.in|\.mk|package\.json|Dockerfile)$|^(lint|test|spec|install|surfaces|lint-workspaces|demos|live)$")

RESOLVING = re.compile(
    # Flags may sit before the subcommand (`cargo +1.95 -q build`,
    # `cargo --config x build`); each one, with an optional value, is
    # skipped (surfaces-review-5: only a toolchain was skipped before).
    r"\bcargo(?:\s+[-+]\S*(?:\s+[^-\s]\S*)??)*\s+"
    r"(build|test|run|check|clippy|zigbuild|fetch|vendor|doc|metadata|install)\b"
    r"|\bmaturin\s+(build|develop)\b"
    r"|\bnapi\s+build\b"
)
PGRX = re.compile(r"\bcargo\s+pgrx\s+package\b")
PGRX_HOME = "scripts/pgrx-package-locked.sh"


def logical_lines(text):
    """Join backslash continuations so a flag on the next line counts."""
    held, start = [], None
    for number, line in enumerate(text.splitlines(), start=1):
        if start is None:
            start = number
        if line.rstrip().endswith("\\"):
            held.append(line.rstrip()[:-1])
            continue
        held.append(line)
        yield start, " ".join(held)
        held, start = [], None
    if held:
        yield start, " ".join(held)


def problems(root):
    found = []
    for top in SCANNED_DIRS:
        base = root / top
        if not base.exists():
            continue
        for path in sorted(base.rglob("*")):
            if not path.is_file() or SKIPPED_PARTS & set(path.relative_to(root).parts):
                continue
            if not SCANNED_NAMES.search(path.name):
                continue
            name = str(path.relative_to(root))
            if name == "scripts/check_locked_calls.py":
                continue
            for number, line in logical_lines(path.read_text(errors="replace")):
                code = line.split("#", 1)[0] if not line.lstrip().startswith("#") else ""
                if not code.strip():
                    continue
                if PGRX.search(code) and name != PGRX_HOME:
                    found.append(f"{name}:{number}: cargo pgrx package outside {PGRX_HOME}")
                    continue
                hit = RESOLVING.search(code)
                if not hit or re.search(r"\b(echo|printf)\b", code[: hit.start()]):
                    continue  # no call, or the tool's name inside a message
                # R's Makevars takes its flags from tools/config.R, whose
                # every .cran_flags carries --locked (checked below). The
                # DuckDB extension's vendored makefile takes its flags from
                # CARGO_OVERRIDE_DUCKDB_RS_FLAG, which the DuckDB Makefile
                # appends --locked to (checked below).
                carried = ("@CRAN_FLAGS@" in code
                           or "$(CARGO_OVERRIDE_DUCKDB_RS_FLAG)" in code)
                if "--locked" not in code and not carried:
                    found.append(f"{name}:{number}: {hit.group(0)} without --locked")
    config = root / "libraries/r/thinkthen/tools/config.R"
    if config.exists():
        for number, line in enumerate(config.read_text().splitlines(), start=1):
            if re.match(r"\s*\.cran_flags <-", line) and "--locked" not in line:
                found.append(f"{config.relative_to(root)}:{number}: .cran_flags without --locked")
    duckdb = root / "databases/duckdb/Makefile"
    if duckdb.exists() and not re.search(
            r"^CARGO_OVERRIDE_DUCKDB_RS_FLAG \+= --locked$", duckdb.read_text(), re.M):
        found.append(f"{duckdb.relative_to(root)}: CARGO_OVERRIDE_DUCKDB_RS_FLAG does not append --locked")
    return found


def main():
    root = pathlib.Path(__file__).resolve().parents[1]
    if len(sys.argv) == 3 and sys.argv[1] == "--root":
        root = pathlib.Path(sys.argv[2]).resolve()
    found = problems(root)
    for line in found:
        print(f"FAIL {line}")
    if found:
        return 1
    print("ok:      every build-tool call site carries the lock")
    return 0


if __name__ == "__main__":
    sys.exit(main())
