#!/usr/bin/env python3
"""Every package-manager call the gate's scripts make stays offline.

The gate never fetches (surfaces-review-5: the Python check downloaded
numpy, pyarrow, polars, and pandas through uv). Each rule below names the
spelling that keeps a call offline. A one-time provisioning step belongs
on a networked machine and in scripts/gate-hermeticity.md, never in a
gate script.

Scanned: every check.sh and the build helpers they call (build*.sh,
make-tarball.sh, tests/*.sh) under libraries/ and databases/.

- uv: `--offline` (or UV_OFFLINE=1 on the line).
- pip install: `--no-index`.
- npm ci or install: `--offline`. npx may fetch a missing tool, so a gate
  script calls node_modules/.bin instead.
- docker run or create: `--pull never`. docker pull is refused, and
  docker build is refused outside the one guarded builder (ALLOWED).
- curl or wget: the loopback only.
- cargo inside a container: `--offline`. A container inherits none of the
  gate's environment, so CARGO_NET_OFFLINE does not reach it. A container
  line is a `docker run` line, the lines of a quoted command it opens, or
  a call of a shell function whose body runs `docker run`.

Usage: python3 scripts/check_offline_calls.py [--root DIR]
Exit 0 when clean, 1 with one `FAIL path:line: why` per call site.
"""
import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from check_locked_calls import logical_lines  # noqa: E402

SKIPPED_PARTS = {"target", "node_modules", ".venv", ".runtimes", "extension-ci-tools", "vendor"}
SCANNED = re.compile(r"^(check\.sh|build[\w-]*\.sh|make-tarball\.sh)$")
# The Ruby builder image: build.sh rebuilds it, and check.sh refuses a
# missing or stale image before build.sh runs, so the gate never reaches
# this build.
ALLOWED = {("libraries/ruby/build.sh", "docker build")}

RULES = [
    (re.compile(r"\buv\s+(venv|pip|sync|run|tool|lock|add)\b"),
     lambda code: "--offline" in code or "UV_OFFLINE=1" in code, "without --offline"),
    (re.compile(r"(?<!uv )\bpip3?\s+install\b"), lambda code: "--no-index" in code, "without --no-index"),
    (re.compile(r"\bnpm\s+(ci|install|i)\b"), lambda code: "--offline" in code, "without --offline"),
    (re.compile(r"(^|[\s;&|(])npx\s"), lambda code: False, "may fetch a missing tool; call node_modules/.bin"),
    (re.compile(r"\bdocker\s+(run|create)\b"),
     lambda code: re.search(r"--pull[= ]never", code) is not None, "without --pull never"),
    (re.compile(r"\bdocker\s+pull\b"), lambda code: False, "pulls an image"),
    (re.compile(r"\bdocker\s+build\b"), lambda code: False, "builds an image, which pulls its base"),
    (re.compile(r"\b(curl|wget)\s"),
     lambda code: not re.search(r"https?://(?!127\.0\.0\.1|localhost)", code), "reaches past the loopback"),
]


CARGO = re.compile(r"(^|[\s;&|('\"])cargo\s+[a-z]")


def container_functions(lines):
    """The names of the shell functions whose body runs `docker run`."""
    names, current, body = set(), None, []
    for line in lines:
        start = re.match(r"^\s*(\w+)\s*\(\)\s*\{", line)
        if current is None and start:
            current, body = start.group(1), []
        elif current is not None and line.strip() == "}":
            if any(re.search(r"\bdocker\s+run\b", held) for held in body):
                names.add(current)
            current = None
        elif current is not None:
            body.append(line)
    return names


def container_cargo(text):
    """Line numbers of cargo calls a container runs without --offline."""
    lines = text.splitlines()
    wrappers = container_functions(lines)
    calls = [re.compile(rf"(^|[\s;&|(]){re.escape(name)}\s") for name in wrappers]
    found, inside, quote = [], False, None
    for number, line in logical_lines(text):
        code = "" if line.lstrip().startswith("#") else line
        if quote is not None:
            inside = True
        else:
            inside = bool(re.search(r"\bdocker\s+run\b", code)) or any(
                call.search(code) and not re.match(r"^\s*\w+\s*\(\)", code) for call in calls)
        if inside and CARGO.search(code) and "--offline" not in code:
            found.append(number)
        # A quoted command a container line opens runs in that container
        # until its quote closes.
        for mark in ('"', "'"):
            if code.count(mark) % 2 == 1:
                if quote is None and inside:
                    quote = mark
                elif quote == mark:
                    quote = None
    return found


def problems(root):
    found = []
    for top in ("libraries", "databases"):
        for path in sorted((root / top).rglob("*.sh")):
            rel = path.relative_to(root)
            if SKIPPED_PARTS & set(rel.parts):
                continue
            if not (SCANNED.match(path.name) or "tests" in rel.parts):
                continue
            name = str(rel)
            text = path.read_text(errors="replace")
            for number in container_cargo(text):
                found.append(f"{name}:{number}: cargo inside a container without --offline")
            for number, line in logical_lines(text):
                code = "" if line.lstrip().startswith("#") else line.split(" #", 1)[0]
                if not code.strip():
                    continue
                for pattern, offline, why in RULES:
                    hit = pattern.search(code)
                    if not hit or re.search(r"\b(echo|printf)\b", code[: hit.start()]):
                        continue
                    call = hit.group(0).strip()
                    if (name, call) in ALLOWED or offline(code):
                        continue
                    found.append(f"{name}:{number}: {call} {why}")
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
    print("ok:      every package-manager call in the gate's scripts stays offline")
    return 0


if __name__ == "__main__":
    sys.exit(main())
