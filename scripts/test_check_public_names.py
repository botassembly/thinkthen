#!/usr/bin/env python3
"""Tests for the public-name check's SQL scan.

A name a test spells is not a public name: the SQLite surface's panic test
names `thinkthen_probe` in a string, and the scan must not read it as a
registration. Each test would have failed before the fix, which scanned
`#[cfg(test)]` code with the rest of the file.

Run: python3 scripts/test_check_public_names.py   (from anywhere)
"""

import importlib.util
import pathlib
import sys
import tempfile

HERE = pathlib.Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("check_public_names", HERE / "check_public_names.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

FAILURES = 0


def report(name, ok, why=""):
    global FAILURES
    if ok:
        print(f"ok       {name}")
    else:
        FAILURES += 1
        print(f"FAIL     {name}: {why}")


def test_test_code_is_ignored():
    with tempfile.TemporaryDirectory() as scratch:
        root = pathlib.Path(scratch)
        (root / "lib.rs").write_text(
            'register("thinkthen_decide");\n'
            "\n"
            "#[cfg(test)]\n"
            "mod tests {\n"
            '    #[test]\n'
            "    fn a_panic_becomes_a_defect() {\n"
            '        let held = guarded("thinkthen_probe", || panic!("the probe blew up"));\n'
            "    }\n"
            "}\n"
        )
        found = MODULE.sql_names(root)
    report(
        "a test-only name is not a public name",
        found == {"thinkthen_decide"},
        f"found {sorted(found)}",
    )


def test_real_registrations_still_found():
    with tempfile.TemporaryDirectory() as scratch:
        root = pathlib.Path(scratch)
        (root / "lib.rs").write_text(
            'register("thinkthen_warm");\n'
            "fn thinkthen_recognize() {}\n"
            "#[cfg(test)]\n"
            "fn helper() {}\n"
        )
        found = MODULE.sql_names(root)
    report(
        "registrations and declared functions are still found",
        found == {"thinkthen_warm", "thinkthen_recognize"},
        f"found {sorted(found)}",
    )


def test_the_repository_passes():
    report(
        "the repository's own names pass the check",
        MODULE.main() == 0,
        "the check failed on the repository",
    )


def main():
    test_test_code_is_ignored()
    test_real_registrations_still_found()
    test_the_repository_passes()
    if FAILURES:
        print(f"{FAILURES} name-check test(s) failed")
        return 1
    print("name-check tests green")
    return 0


if __name__ == "__main__":
    sys.exit(main())
