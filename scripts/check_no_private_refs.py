#!/usr/bin/env python3
"""Refuse the private repository's name and home-directory paths.

The repository goes public, so no tracked file outside sdlc/ may name the
private repository that carries the product deck, and none may carry an
absolute home-directory path (the builder's home leaks through examples
and logs). sdlc/ is the internal record and is exempt.

Scope decision (2026-09-22, recorded here because this checker is the
enforcement point): sdlc/ is part of the public checkout, so the private
name and home paths are cleaned there too even though the checker does
not scan it — historical records keep their evidence (hashes, paths
inside the deck repository) with the name genericized to "the deck
repository" and home paths made relative. New records write the generic
forms from the start. This checker and its test are exempt because the
refused strings are their own pattern data.

Only tracked files are scanned (the plain-tree fallback walks the same
set when the root is not a git checkout, so an exported tree answers
too). Untracked build output (target/, dist/) is not shipped and never
was the finding.

Usage: check_no_private_refs.py [--root DIR]
Exit 0 when clean, 1 with one `path:line: what` line per violation.
"""

import os
import re
import subprocess
import sys

# What the plain-tree fallback skips: git-ignored build output by name, and
# the packaged trees that are never source.
IGNORED_DIRS = {"target", "build", "dist", "node_modules", ".venv", ".cargo", ".runtimes"}
PACKAGE_SUFFIXES = (".node", ".so", ".dylib", ".dll", ".a", ".gem", ".tgz", ".gz")

# The private repository's directory name, and the home-directory roots to
# refuse. Written as patterns here and nowhere else in the tree.
PRIVATE_NAME = "mktg"
HOME_ROOTS = ("home", "Users")

SELF = {"scripts/check_no_private_refs.py", "scripts/test_check_no_private_refs.py"}

PRIVATE_RE = re.compile(r"\b" + PRIVATE_NAME + r"\b")
HOME_RE = re.compile(r"/(?:" + "|".join(HOME_ROOTS) + r")/[A-Za-z0-9._-]+")


def tracked_files(root):
    """The files a public checkout would carry: git-tracked when the root is
    a checkout, and the plain tree otherwise, so the check answers outside a
    clone too (an exported tree, a tarball unpacked) instead of crashing."""
    run = subprocess.run(
        ["git", "-C", root, "ls-files", "-z"],
        capture_output=True,
    )
    if run.returncode == 0:
        return [name.decode() for name in run.stdout.split(b"\0") if name]
    # Not a git checkout (or git absent): walk the tree, skipping what git
    # would ignore — build output and the checker's own pattern data.
    names = []
    for base, dirs, files in os.walk(root):
        keep = [d for d in dirs if d not in IGNORED_DIRS and not d.startswith(".")]
        dirs[:] = keep
        for file in files:
            path = os.path.relpath(os.path.join(base, file), root)
            if path.endswith(PACKAGE_SUFFIXES):
                continue
            names.append(path)
    return names


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
    explicit = False
    if len(sys.argv) == 3 and sys.argv[1] == "--root":
        root = sys.argv[2]
        explicit = True
    elif len(sys.argv) != 1:
        print("usage: check_no_private_refs.py [--root DIR]", file=sys.stderr)
        return 2
    if not explicit:
        probe = subprocess.run(
            ["git", "-C", root, "ls-files"],
            capture_output=True,
        )
        if probe.returncode != 0:
            print(
                "refusing an unbounded walk: cwd is not a git checkout; "
                "pass --root DIR to check an exported tree",
                file=sys.stderr,
            )
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
