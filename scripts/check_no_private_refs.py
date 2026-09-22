#!/usr/bin/env python3
"""Refuse the private repository's name and home-directory paths.

The repository goes public, so no tracked file outside sdlc/ may name the
private repository that carries the product deck, and none may carry an
absolute home-directory path (the builder's home leaks through examples
and logs). sdlc/ is the internal record and is exempt. This checker and its
test are exempt because the refused strings are their own pattern data.

Only tracked files are scanned: untracked build output (target/, dist/) is
not shipped and never was the finding.

Usage: check_no_private_refs.py [--root DIR]
Exit 0 when clean, 1 with one `path:line: what` line per violation.
"""

import re
import subprocess
import sys

# The private repository's directory name, and the home-directory roots to
# refuse. Written as patterns here and nowhere else in the tree.
PRIVATE_NAME = "mktg"
HOME_ROOTS = ("home", "Users")

SELF = {"scripts/check_no_private_refs.py", "scripts/test_check_no_private_refs.py"}

PRIVATE_RE = re.compile(r"\b" + PRIVATE_NAME + r"\b")
HOME_RE = re.compile(r"/(?:" + "|".join(HOME_ROOTS) + r")/[A-Za-z0-9._-]+")


def tracked_files(root):
    out = subprocess.run(
        ["git", "-C", root, "ls-files", "-z"],
        check=True,
        capture_output=True,
    ).stdout
    return [name.decode() for name in out.split(b"\0") if name]


def violations(root):
    found = []
    for path in tracked_files(root):
        if path in SELF or path == "sdlc" or path.startswith("sdlc/"):
            continue
        try:
            with open(f"{root}/{path}", "rb") as handle:
                data = handle.read()
        except OSError:
            continue
        text = data.decode("utf-8", errors="replace")
        for number, line in enumerate(text.splitlines(), start=1):
            if PRIVATE_RE.search(line):
                found.append(f"{path}:{number}: names the private repository")
            if HOME_RE.search(line):
                found.append(f"{path}:{number}: carries a home-directory path")
    return found


def main():
    root = "."
    if len(sys.argv) == 3 and sys.argv[1] == "--root":
        root = sys.argv[2]
    elif len(sys.argv) != 1:
        print("usage: check_no_private_refs.py [--root DIR]", file=sys.stderr)
        return 2
    found = violations(root)
    if found:
        for line in found:
            print(line)
        print(f"FAIL: {len(found)} private reference(s) outside sdlc/", file=sys.stderr)
        return 1
    print(f"ok: no private repository names or home paths outside sdlc/")
    return 0


if __name__ == "__main__":
    sys.exit(main())
