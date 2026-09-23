#!/usr/bin/env python3
"""Tests for the conformance checker's path handling and failure exits.

The checker must read the file it is given, default to the repository's
own file from any working directory (the repository root included), and
exit nonzero with the offending case named when a case is corrupted. Each
test would have failed before the fix: the old checker read a hardcoded
relative `conformance.json` and crashed from the repository root.

Run: python3 tools/test_validate_conformance.py   (from anywhere)
"""

import json
import pathlib
import subprocess
import sys
import tempfile

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]
CHECKER = HERE / "validate_conformance.py"
CASES = ROOT / "conformance" / "conformance.json"

FAILURES = 0


def run(args, cwd):
    return subprocess.run(
        [sys.executable, str(CHECKER), *args],
        capture_output=True, text=True, cwd=cwd, check=False,
    )


def report(name, ok, why=""):
    global FAILURES
    if ok:
        print(f"ok       {name}")
    else:
        FAILURES += 1
        print(f"FAIL     {name}: {why}")


def test_default_from_repo_root():
    done = run([], ROOT)
    report(
        "the default file is found from the repository root",
        done.returncode == 0 and done.stdout.startswith("OK:"),
        f"exit {done.returncode}: {(done.stdout + done.stderr).strip()[:200]}",
    )


def test_explicit_relative_path():
    # The gate's own invocation: from `conformance/`, naming the file
    # beside the working directory, and from the repository root naming
    # it by its path.
    done = run(["conformance.json"], ROOT / "conformance")
    report(
        "the file named on the command line is the one read",
        done.returncode == 0 and done.stdout.startswith("OK:"),
        f"exit {done.returncode}: {(done.stdout + done.stderr).strip()[:200]}",
    )
    done = run(["conformance/conformance.json"], ROOT)
    report(
        "a repository-root relative path is honored too",
        done.returncode == 0 and done.stdout.startswith("OK:"),
        f"exit {done.returncode}: {(done.stdout + done.stderr).strip()[:200]}",
    )


def test_absolute_path_from_elsewhere():
    with tempfile.TemporaryDirectory() as scratch:
        done = run([str(CASES)], scratch)
    report(
        "an absolute path works from an unrelated directory",
        done.returncode == 0 and done.stdout.startswith("OK:"),
        f"exit {done.returncode}: {(done.stdout + done.stderr).strip()[:200]}",
    )


def test_corrupted_copy_fails_and_names_the_case():
    data = json.loads(CASES.read_text())
    changed = None
    for case in data["cases"]:
        if case["verb"] == "decide" and "answer" in case.get("expect", {}):
            case["expect"]["answer"] = not case["expect"]["answer"]
            changed = case["id"]
            break
    if changed is None:
        report("a corrupted case fails the check", False, "no decide case to corrupt")
        return
    with tempfile.TemporaryDirectory() as scratch:
        copy = pathlib.Path(scratch) / "corrupted.json"
        copy.write_text(json.dumps(data))
        done = run([str(copy)], scratch)
    report(
        "a corrupted case fails the check and is named",
        done.returncode == 1 and changed in (done.stdout + done.stderr),
        f"exit {done.returncode}, wanted {changed!r} named: "
        f"{(done.stdout + done.stderr).strip()[:200]}",
    )


def test_missing_file_reports_cleanly():
    done = run(["/nonexistent/conformance.json"], "/")
    report(
        "a missing file reports and exits nonzero",
        done.returncode == 1 and "cannot read the cases file" in done.stdout,
        f"exit {done.returncode}: {(done.stdout + done.stderr).strip()[:200]}",
    )


def test_wrong_case_count_fails():
    data = json.loads(CASES.read_text())
    data["case_count"] = data["case_count"] + 1
    with tempfile.TemporaryDirectory() as scratch:
        copy = pathlib.Path(scratch) / "wrong-count.json"
        copy.write_text(json.dumps(data))
        done = run([str(copy)], scratch)
    report(
        "a wrong case_count fails the check",
        done.returncode == 1 and "case_count" in (done.stdout + done.stderr),
        f"exit {done.returncode}: {(done.stdout + done.stderr).strip()[:200]}",
    )


def test_skip_table_entries_are_checked():
    data = json.loads(CASES.read_text())
    data["skips"].append({"when": {"id": "no-such-case"}, "why": "x"})
    with tempfile.TemporaryDirectory() as scratch:
        copy = pathlib.Path(scratch) / "bad-skip.json"
        copy.write_text(json.dumps(data))
        done = run([str(copy)], scratch)
    report(
        "a skip entry naming no case fails the check",
        done.returncode == 1 and "names no case" in (done.stdout + done.stderr),
        f"exit {done.returncode}: {(done.stdout + done.stderr).strip()[:200]}",
    )
    data = json.loads(CASES.read_text())
    data["skips"].append({"when": {"verb": "cancel"}})
    with tempfile.TemporaryDirectory() as scratch:
        copy = pathlib.Path(scratch) / "no-why.json"
        copy.write_text(json.dumps(data))
        done = run([str(copy)], scratch)
    report(
        "a skip entry without a reason fails the check",
        done.returncode == 1 and "needs a written why" in (done.stdout + done.stderr),
        f"exit {done.returncode}: {(done.stdout + done.stderr).strip()[:200]}",
    )


def test_an_unrecorded_request_id_fails():
    """A case must name only requests its replay-table row recorded. A
    request id the table does not hold means the case outlived its row."""
    data = json.loads(CASES.read_text())
    changed = None
    for case in data["cases"]:
        if case["verb"] in ("relate", "recognize") and case["requests"]:
            case["requests"][0] = "0" * 64
            changed = case["id"]
            break
    with tempfile.TemporaryDirectory() as scratch:
        copy = pathlib.Path(scratch) / "unrecorded.json"
        copy.write_text(json.dumps(data))
        done = run([str(copy)], scratch)
    report(
        "a request id the replay table does not hold fails the check",
        changed is not None and done.returncode == 1
        and f"{changed} requests are recorded in the replay table" in (done.stdout + done.stderr),
        f"exit {done.returncode}, wanted {changed!r} named: "
        f"{(done.stdout + done.stderr).strip()[:200]}",
    )


def main():
    test_default_from_repo_root()
    test_explicit_relative_path()
    test_absolute_path_from_elsewhere()
    test_corrupted_copy_fails_and_names_the_case()
    test_missing_file_reports_cleanly()
    test_wrong_case_count_fails()
    test_skip_table_entries_are_checked()
    test_an_unrecorded_request_id_fails()
    if FAILURES:
        print(f"{FAILURES} checker test(s) failed")
        return 1
    print("checker tests green")
    return 0


if __name__ == "__main__":
    sys.exit(main())
