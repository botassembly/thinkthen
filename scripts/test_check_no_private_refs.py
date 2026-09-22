#!/usr/bin/env python3
"""The private-reference checker's own test.

Plants violations in a scratch repository and proves the checker refuses
them: a tracked file naming the private repository fails, a tracked file
carrying a home path fails, the same strings under sdlc/ pass, and the
clean tree passes again once the planted files are gone.
"""

import os
import subprocess
import sys
import tempfile

CHECKER = os.path.join(os.path.dirname(os.path.abspath(__file__)), "check_no_private_refs.py")


def run_checker(root):
    return subprocess.run(
        [sys.executable, CHECKER, "--root", root],
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
            handle.write("the deck lives in repos/" + "mktg" + "/decks\n")
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
            handle.write("repos/" + "mktg" + "/decks and /" + "home/example/workspace\n")
        git(root, "add", "sdlc/record.md")
        result = run_checker(root)
        if result.returncode != 0:
            failures.append(f"sdlc/ not exempt: {result.stdout}{result.stderr}")
        git(root, "rm", "-q", "--cached", "sdlc/record.md")

    if failures:
        for failure in failures:
            print(f"FAIL: {failure}")
        return 1
    print("ok: the checker refuses a planted private name and home path, exempts sdlc/, and passes a clean tree")
    return 0


if __name__ == "__main__":
    sys.exit(main())
