"""Checks over this binding's own files, run by check.sh.

- R5-21: the SIGINT handler and its bridge write lock nothing, allocate
  nothing, and call nothing in `thinkthen`.
- ADR 0081: no raw C API entry, dependency, or shipped path remains.
- R2-31: the C++ bridge catches panics only through thinkthen::contained.
- R4-19, R5-34: every `cargo` call in check.sh passes `--locked` and
  `--offline`.
- R1-33: every vendored script is named by check.sh or a tool.
- R5-25: `deny.toml` is the root copy after the raw C binding retired.
- R5-23: the README pins the `con.interrupt()` limit.
- R2-29: ADR 0038's DuckDB amendment keeps one pinned sentence per ruling.
- R3-29: `requirements.txt` pins each line with `==` and names no URL.
- Decision 14: the shipped extension carries no test hook.

Run with a file path as `--requirements PATH` to check that file alone.
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = ROOT.parent.parent
sys.path.insert(0, str(REPO / "conformance" / "children"))
from children import CARGO, child_env  # noqa: E402  the shared helper, ticket 0127
FAILED: list[str] = []


def fail(message: str) -> None:
    FAILED.append(message)


def body(text: str, name: str) -> str:
    """The body of the first `fn name`, by brace count."""
    start = text.index(f"fn {name}(")
    depth, at = 0, text.index("{", start)
    for end in range(at, len(text)):
        depth += {"{": 1, "}": -1}.get(text[end], 0)
        if depth == 0:
            return text[at : end + 1]
    raise ValueError(f"fn {name} has no closing brace")


def handler() -> None:
    signal_ffi = (ROOT / "src" / "signal" / "ffi.rs").read_text()
    signal_rs = (ROOT / "src" / "signal.rs").read_text()
    code = body(signal_ffi, "on_interrupt") + body(signal_rs, "on_signal") + body(signal_ffi, "wake_bridge")
    for banned in ("lock", "Box", "Vec", "String", "format!", "to_owned", "thinkthen::", "Mutex", "alloc", "println", "eprintln"):
        if banned in code:
            fail(f"R5-21: the SIGINT handler holds `{banned}`")


def retired_entry() -> None:
    manifest = (ROOT / "Cargo.toml").read_text()
    lock = (ROOT / "Cargo.lock").read_text()
    entry = (ROOT / "src" / "lib.rs").read_text()
    if ('libduckdb-sys' in manifest or 'name = "libduckdb-sys"' in lock or
            'crate-type = ["cdylib"]' in manifest or
            'mod ffi;' in entry or (ROOT / "src" / "ffi.rs").exists()):
        fail("ADR 0081: the retired raw C API still has a dependency or entry point")
    release = (REPO / "sdlc" / "scripts" / "release-pack").read_text()
    check = (ROOT / "check.sh").read_text()
    if "sh databases/duckdb/cpp/build.sh" not in release or "sh cpp/build.sh" not in check:
        fail("ADR 0081: release and check must build the C++ extension")


def guards() -> None:
    sites = sum(path.read_text().count("catch_unwind(")
                for path in (ROOT / "bridge" / "src").rglob("*.rs"))
    if sites != 0:
        fail("R2-31: the bridge catches panics only through thinkthen::contained")


def cargo_flags() -> None:
    for number, line in enumerate((ROOT / "check.sh").read_text().splitlines(), 1):
        words = line.split("#")[0].split()
        if words[:1] == ["echo"]:
            continue
        runs = [words[at + 1] for at, word in enumerate(words[:-1]) if word == "cargo"]
        if set(runs) & {"build", "test", "clippy", "deny", "tree", "metadata", "check"} and not {"--locked", "--offline"} <= set(words):
            fail(f"R4-19: check.sh:{number} runs cargo without --locked and --offline")


def vendored() -> None:
    named = (ROOT / "check.sh").read_text() + (REPO / "sdlc" / "scripts" / "release-pack").read_text()
    named += "".join(path.read_text() for path in (ROOT / "tools").iterdir() if path.is_file())
    for path in sorted((ROOT / "vendor").iterdir()):
        if path.name != "LICENSE" and path.name not in named:
            fail(f"R1-33: vendor/{path.name} is named by no script")


def deny() -> None:
    root = (REPO / "deny.toml").read_text()
    ours = (ROOT / "deny.toml").read_text()
    if ours != root:
        fail("R5-25: deny.toml differs from the root copy")


def readme() -> None:
    limit = "DuckDB's own `con.interrupt()` does not stop a held batch before its replies arrive; a SIGINT does, within 100 ms."
    if limit not in (ROOT / "README.md").read_text():
        fail("R5-23: the README lost the pinned interrupt limit")


ADR = REPO / "sdlc" / "planning" / "adr" / "0038-duckdb-relate-runs-on-the-callers-database.md"
RULINGS = {
    "SIGINT at LOAD": "LOAD takes SIGINT through `sigaction` and chains to the host's own action with the signature its flags name.",
    "the worker": "Every engine call runs on a detachable worker with its own cancel token.",
    "the con.interrupt() limit": "DuckDB gives a scalar no view of its own interrupt, so `con.interrupt()` does not stop a held batch before its replies arrive.",
    "file access": "`@file` opens through that file system, so the caller's own settings decide each read.",
    "the engine settings": "The first throttle wins for the process, as main's `build` rules.",
    "volatile scalars": "Every scalar registers as volatile, so the planner never folds a constant call into a send.",
    "warm": "**The aggregate reads `@file` through the kept connection of the database that registered it, under that database's gate, and runs on the engine the environment describes.**",
    "licenses": "One exception remains: `zlib-rs` (Zlib), a build-time dependency of `libduckdb-sys`.",
    "relate from rows": "**The relate query returns `id, name, kind` or `id, name`, rows with the same name and kind become one entity, and each edge returns one row per pair of their ids.**",
    "the row cap": "**Relate reads at most 255 rows, under `LIMIT 256`, and more is `usage`.**",
    "the time limit": "**`SET thinkthen_relate_seconds` bounds the query and the engine call, and `memory_limit` stays the hard bound for a step that holds its input.**",
    "the identity source": "**Each kept connection attaches an in-memory probe database named from 128 bits of `/dev/urandom`, and a caller matches only the probe its own context resolves.**",
    "the bridge's pipe": "**The SIGINT handler writes one byte to a pipe only when the pipe's recorded process id equals `getpid()`, and a bridge thread interrupts each busy kept connection.**",
}


def rulings() -> None:
    text = ADR.read_text()
    for ruling, sentence in RULINGS.items():
        if sentence not in text:
            fail(f"R2-29: ADR 0038's DuckDB amendment lost its sentence on {ruling}")


def requirements(path: Path) -> list[str]:
    wrong = []
    for line in path.read_text().splitlines():
        line = line.strip()
        if line and not line.startswith("#") and ("==" not in line or "://" in line or line.startswith("git+")):
            wrong.append(f"R3-29: {path.name} holds the unpinned or remote line {line!r}")
    return wrong


def shipped() -> None:
    built = ROOT / "build" / "thinkthen.duckdb_extension"
    data = built.read_bytes()
    for marker in (b"thinkthen_test_hook", b"ENGINE_TEST_PANIC"):
        if marker in data:
            fail(f"decision 14: the shipped extension holds {marker.decode()}")
    tree = subprocess.run(
        ["cargo", "tree", "--offline", "--locked", "-e", "features", "-i", "thinkthen-duckdb-bridge"],
        cwd=ROOT / "bridge", check=True, capture_output=True, text=True, stdin=subprocess.DEVNULL, env=child_env(CARGO),
    ).stdout
    if "test-hooks" in tree:
        fail("decision 14: the shipped build enables test-hooks")


def main() -> int:
    if sys.argv[1:2] == ["--requirements"]:
        wrong = requirements(Path(sys.argv[2]))
        print("\n".join(wrong) or "ok")
        return 1 if wrong else 0
    handler()
    retired_entry()
    guards()
    cargo_flags()
    vendored()
    deny()
    readme()
    rulings()
    FAILED.extend(requirements(ROOT / "tools" / "requirements.txt"))
    shipped()
    for message in FAILED:
        print(f"FAIL {message}")
    if not FAILED:
        print("ok   source checks")
    return 1 if FAILED else 0


if __name__ == "__main__":
    sys.exit(main())
