#!/usr/bin/env python3
"""The private-reference checker's own test.

Plants violations in a scratch repository and proves the checker refuses
them: a tracked file naming the private repository fails, a tracked file
carrying a home path fails, the same strings under sdlc/ pass, and the
clean tree passes again once the planted files are gone. The planted
name is a word of this test's own, passed to the checker by its hash, so
neither file spells the real one. A tree without git is walked and still
refuses the planted name (surfaces-review-5: it used to exit 2).
"""

import hashlib
import os
import subprocess
import sys
import tempfile

CHECKER = os.path.join(os.path.dirname(os.path.abspath(__file__)), "check_no_private_refs.py")
PLANTED = "qzvx"
PLANTED_SHA256 = hashlib.sha256(PLANTED.encode()).hexdigest()


def run_checker(root):
    return subprocess.run(
        [sys.executable, CHECKER, "--root", root, "--name-sha256", PLANTED_SHA256],
        capture_output=True,
        text=True,
    )


def git(root, *args):
    subprocess.run(["git", "-C", root, *args], check=True, capture_output=True)


def main():
    failures = []
    with tempfile.TemporaryDirectory() as root:
        git(root, "init", "-q")

        # A clean tree passes.
        result = run_checker(root)
        if result.returncode != 0:
            failures.append(f"empty tree refused: {result.stdout}{result.stderr}")

        # A tracked file naming the private repository fails.
        with open(f"{root}/leak.md", "w") as handle:
            handle.write(f"the deck lives in repos/{PLANTED}/decks\n")
        git(root, "add", "leak.md")
        result = run_checker(root)
        if result.returncode != 1 or "leak.md:1" not in result.stdout:
            failures.append(f"private name not refused: {result.stdout}{result.stderr}")
        git(root, "rm", "-q", "--cached", "leak.md")
        os.remove(f"{root}/leak.md")

        # A tracked file carrying a home path fails.
        with open(f"{root}/path.md", "w") as handle:
            handle.write("built in /" + "home/example/workspace\n")
        git(root, "add", "path.md")
        result = run_checker(root)
        if result.returncode != 1 or "path.md:1" not in result.stdout:
            failures.append(f"home path not refused: {result.stdout}{result.stderr}")
        git(root, "rm", "-q", "--cached", "path.md")
        os.remove(f"{root}/path.md")

        # The same strings under sdlc/ are exempt.
        os.makedirs(f"{root}/sdlc", exist_ok=True)
        with open(f"{root}/sdlc/record.md", "w") as handle:
            handle.write(f"repos/{PLANTED}/decks and /" + "home/example/workspace\n")
        git(root, "add", "sdlc/record.md")
        result = run_checker(root)
        if result.returncode != 0:
            failures.append(f"sdlc/ not exempt: {result.stdout}{result.stderr}")
        git(root, "rm", "-q", "--cached", "sdlc/record.md")

    # A copy of the checker in a tree without git, run with no --root from
    # elsewhere, walks its own tree, and the planted name still fails.
    with tempfile.TemporaryDirectory() as root:
        os.makedirs(f"{root}/scripts")
        with open(CHECKER) as source, open(f"{root}/scripts/check_no_private_refs.py", "w") as copy:
            copy.write(source.read())
        with open(f"{root}/leak.md", "w") as handle:
            handle.write(f"the deck lives in repos/{PLANTED}/decks\n")
        result = subprocess.run(
            [sys.executable, f"{root}/scripts/check_no_private_refs.py", "--name-sha256", PLANTED_SHA256],
            capture_output=True, text=True, cwd="/",
        )
        if result.returncode != 1 or result.stdout != "leak.md:1: names the private repository\n":
            failures.append(f"a tree without git not walked: {result.returncode} {result.stdout}{result.stderr}")

    if failures:
        for failure in failures:
            print(f"FAIL: {failure}")
        return 1
    print("ok: the checker refuses a planted private name and home path, exempts sdlc/, walks a tree without git, and passes a clean tree")
    return 0


if __name__ == "__main__":
    sys.exit(main())
