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

The private name is held as the SHA-256 of its one word, so this public
file never spells it (surfaces-review-5). Every word of a scanned line is
hashed and compared. The test plants a word of its own and passes that
word's hash with --name-sha256.

The root defaults to the repository that holds this script, so a tree
without git walks that bounded tree instead of refusing.

Usage: check_no_private_refs.py [--root DIR] [--name-sha256 HEX]
Exit 0 when clean, 1 with one `path:line: what` line per violation.
"""

import hashlib
import os
import re
import subprocess
import sys

# What the plain-tree fallback skips: git-ignored build output by name, and
# the packaged trees that are never source.
IGNORED_DIRS = {"target", "build", "dist", "node_modules", ".venv", ".cargo", ".runtimes"}
PACKAGE_SUFFIXES = (".node", ".so", ".dylib", ".dll", ".a", ".gem", ".tgz", ".gz")

# The SHA-256 of the private repository's directory name, and the
# home-directory roots to refuse.
PRIVATE_SHA256 = "eff9de6855d51a876662055ed6a6a8702179a1b5c335f3b1f66acaaa8a8e22ee"
HOME_ROOTS = ("home", "Users")

SELF = {"scripts/check_no_private_refs.py", "scripts/test_check_no_private_refs.py"}

WORD_RE = re.compile(r"\w+")
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


def names_private(line, digest, seen):
    """True when any word of the line hashes to the private name's digest.
    `seen` caches each distinct word's verdict across the whole scan."""
    for word in WORD_RE.findall(line):
        verdict = seen.get(word)
        if verdict is None:
            verdict = hashlib.sha256(word.encode()).hexdigest() == digest
            seen[word] = verdict
        if verdict:
            return True
    return False


def violations(root, digest=PRIVATE_SHA256):
    found = []
    seen = {}
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
            if names_private(line, digest, seen):
                found.append(f"{path}:{number}: names the private repository")
            if HOME_RE.search(line):
                found.append(f"{path}:{number}: carries a home-directory path")
    return found


def main():
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    digest = PRIVATE_SHA256
    args = sys.argv[1:]
    while args:
        if len(args) >= 2 and args[0] == "--root":
            root = args[1]
        elif len(args) >= 2 and args[0] == "--name-sha256":
            digest = args[1]
        else:
            print("usage: check_no_private_refs.py [--root DIR] [--name-sha256 HEX]", file=sys.stderr)
            return 2
        args = args[2:]
    found = violations(root, digest)
    if found:
        for line in found:
            print(line)
        print(f"FAIL: {len(found)} private reference(s) outside sdlc/", file=sys.stderr)
        return 1
    print("ok: no private repository names or home paths outside sdlc/")
    return 0


if __name__ == "__main__":
    sys.exit(main())
