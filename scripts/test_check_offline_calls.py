#!/usr/bin/env python3
"""The offline checker's own test (surfaces-review-5).

Plants one gate-script call per rule in a scratch tree and pins each
verdict, so a rule that stops matching fails here.

Usage: python3 scripts/test_check_offline_calls.py
"""
import pathlib
import subprocess
import sys
import tempfile

CHECKER = pathlib.Path(__file__).resolve().parent / "check_offline_calls.py"

CASES = [
    ("uv venv .venv\n", "FAIL libraries/x/check.sh:1: uv venv without --offline\n"),
    ("uv pip install --python p \\\n  -r r.txt\n", "FAIL libraries/x/check.sh:1: uv pip without --offline\n"),
    ("pip install -r r.txt\n", "FAIL libraries/x/check.sh:1: pip install without --no-index\n"),
    ("npm ci --silent\n", "FAIL libraries/x/check.sh:1: npm ci without --offline\n"),
    ("exec npx napi build\n", "FAIL libraries/x/check.sh:1: npx may fetch a missing tool; call node_modules/.bin\n"),
    ("docker run --rm img true\n", "FAIL libraries/x/check.sh:1: docker run without --pull never\n"),
    ("docker pull img\n", "FAIL libraries/x/check.sh:1: docker pull pulls an image\n"),
    ("curl -sf https://example.org/x\n", "FAIL libraries/x/check.sh:1: curl reaches past the loopback\n"),
    ("uv venv --offline .venv\nuv pip install --offline -r r.txt\npip install --no-index x\n"
     "npm ci --offline\ndocker run --rm --pull never img true\ncurl -sf http://127.0.0.1:8211/v1/stats\n"
     "echo \"run npm ci once\"\n",
     "ok:      every package-manager call in the gate's scripts stays offline\n"),
]


def main():
    bad = 0
    for text, want in CASES:
        with tempfile.TemporaryDirectory() as scratch:
            root = pathlib.Path(scratch)
            (root / "libraries/x").mkdir(parents=True)
            (root / "databases").mkdir()
            (root / "libraries/x/check.sh").write_text(text)
            done = subprocess.run([sys.executable, str(CHECKER), "--root", str(root)],
                                  capture_output=True, text=True, check=False)
        if done.stdout != want or done.returncode != (0 if want.startswith("ok") else 1):
            print(f"FAIL {text.splitlines()[0]!r}: expected {want!r}, got {done.returncode} {done.stdout!r}")
            bad = 1
    if not bad:
        print("ok:      the offline checker refuses each fetching spelling and passes the offline ones")
    return bad


if __name__ == "__main__":
    sys.exit(main())
