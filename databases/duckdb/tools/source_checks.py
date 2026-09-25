"""Checks over this binding's own files, run by check.sh.

- R5-21: the SIGINT handler and its bridge write lock nothing, allocate
  nothing, and call nothing in `thinkthen`.
- R4-16: no call to a C API function the bindings mark deprecated, and
  every logical type is made and destroyed by the one wrapper.
- R2-31: one `catch_unwind` site in `src`.
- R4-19, R5-34: every `cargo` call in check.sh passes `--locked` and
  `--offline`.
- R1-33: every vendored script is named by check.sh or a tool.
- R5-25: `deny.toml` is the root copy plus the one exception.
- R5-23: the README pins the `con.interrupt()` limit.
- R3-29: `requirements.txt` pins each line with `==` and names no URL.
- Decision 14: the shipped extension carries no test hook.

Run with a file path as `--requirements PATH` to check that file alone.
"""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = ROOT.parent.parent
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


def deprecated() -> None:
    meta = json.loads(
        subprocess.run(
            ["cargo", "metadata", "--offline", "--locked", "--format-version", "1"],
            cwd=ROOT, check=True, capture_output=True, text=True, stdin=subprocess.DEVNULL,
        ).stdout
    )
    crate = next(package for package in meta["packages"] if package["name"] == "libduckdb-sys")
    text = (Path(crate["manifest_path"]).parent / "src" / "bindgen_bundled_version.rs").read_text()
    notices = {match.group(2) for match in re.finditer(r'#\[doc = "(.*?)"\]\s*pub fn (\w+)', text, re.S) if "DEPRECATION NOTICE" in match.group(1)}
    if len(notices) < 10:
        fail(f"R4-16: read only {len(notices)} deprecation notices, so the check cannot see them")
    pattern = re.compile(r"\b(" + "|".join(sorted(notices)) + r")\b")
    for path in sorted((ROOT / "src").rglob("*.rs")):
        for number, line in enumerate(path.read_text().splitlines(), 1):
            for match in pattern.finditer(line):
                fail(f"R4-16: {path.relative_to(ROOT)}:{number} calls the deprecated {match.group(1)}")


def logical_types() -> None:
    makers = re.compile(r"duckdb_(create_logical_type|create_list_type|create_struct_type|destroy_logical_type)\b")
    for path in sorted((ROOT / "src").rglob("*.rs")):
        text = path.read_text()
        found = makers.findall(text)
        if not found:
            continue
        wrapper = text[text.index("impl Logical") :] if "impl Logical" in text else ""
        inside = makers.findall(body(wrapper, "new") + body(wrapper, "drop")) if wrapper else []
        if path != ROOT / "src" / "ffi.rs" or len(found) != len(inside):
            fail(f"R4-16: {path.relative_to(ROOT)} makes or destroys a logical type outside the Logical wrapper")


def guards() -> None:
    sites = sum(path.read_text().count("catch_unwind(") for path in (ROOT / "src").rglob("*.rs"))
    if sites != 1:
        fail(f"R2-31: src holds {sites} catch_unwind sites, and the one guard is the only one")


def cargo_flags() -> None:
    for number, line in enumerate((ROOT / "check.sh").read_text().splitlines(), 1):
        words = line.split("#")[0].split()
        if words[:1] == ["echo"]:
            continue
        runs = [words[at + 1] for at, word in enumerate(words[:-1]) if word == "cargo"]
        if set(runs) & {"build", "test", "clippy", "deny", "tree", "metadata", "check"} and not {"--locked", "--offline"} <= set(words):
            fail(f"R4-19: check.sh:{number} runs cargo without --locked and --offline")


def vendored() -> None:
    named = (ROOT / "check.sh").read_text() + "".join(path.read_text() for path in (ROOT / "tools").iterdir() if path.is_file())
    for path in sorted((ROOT / "vendor").iterdir()):
        if path.name != "LICENSE" and path.name not in named:
            fail(f"R1-33: vendor/{path.name} is named by no script")


def deny() -> None:
    root = (REPO / "deny.toml").read_text()
    ours = (ROOT / "deny.toml").read_text()
    start = ours.find("# ADR 0047: the DuckDB binding")
    end = ours.find("\n", ours.find("exceptions = [{", start)) if start >= 0 else -1
    exception = ours[start:end] if end > start else ""
    if not exception or ours.replace(exception, "exceptions = []", 1) != root:
        fail("R5-25: deny.toml is not the root copy with the one DuckDB exception")


def readme() -> None:
    limit = "DuckDB's own `con.interrupt()` does not stop a held batch before its replies arrive; a SIGINT does, within 100 ms."
    if limit not in (ROOT / "README.md").read_text():
        fail("R5-23: the README lost the pinned interrupt limit")


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
        ["cargo", "tree", "--offline", "--locked", "-e", "features", "-i", "thinkthen-duckdb"],
        cwd=ROOT, check=True, capture_output=True, text=True, stdin=subprocess.DEVNULL,
    ).stdout
    if "test-hooks" in tree:
        fail("decision 14: the shipped build enables test-hooks")


def main() -> int:
    if sys.argv[1:2] == ["--requirements"]:
        wrong = requirements(Path(sys.argv[2]))
        print("\n".join(wrong) or "ok")
        return 1 if wrong else 0
    handler()
    deprecated()
    logical_types()
    guards()
    cargo_flags()
    vendored()
    deny()
    readme()
    FAILED.extend(requirements(ROOT / "tools" / "requirements.txt"))
    shipped()
    for message in FAILED:
        print(f"FAIL {message}")
    if not FAILED:
        print("ok   source checks")
    return 1 if FAILED else 0


if __name__ == "__main__":
    sys.exit(main())
