#!/usr/bin/env python3
"""Tests for the public-name check's SQL scan.

A name a test spells is not a public name: the SQLite surface's panic test
names `thinkthen_probe` in a string, and the scan must not read it as a
registration. Each test would have failed before the fix, which scanned
`#[cfg(test)]` code with the rest of the file.

Run: python3 scripts/test_check_public_names.py   (from anywhere)
"""

import contextlib
import contextlib
import importlib.util
import io
import io
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
    # The check's own per-surface lines are captured, not printed: the
    # gate runs the real check as its own step, and a second copy of its
    # `ok` lines inflated the green count (surfaces-review-5).
    said = io.StringIO()
    with contextlib.redirect_stdout(said):
        status = MODULE.main([])
    report(
        "the repository's own names pass the check",
        status == 0,
        f"the check failed on the repository:\n{said.getvalue()}",
    )


def test_ruby_privacy_markers_are_read():
    text = "  private_constant :Row, :Native\n  private_class_method :_parse_set,\n    :text_of\n"
    constants = MODULE._marked(text, "private_constant", r"[A-Z]\w*")
    methods = MODULE._marked(text, "private_class_method", r"[a-z_]\w*")
    report(
        "Ruby privacy markers name what they hide",
        constants == {"Row", "Native"} and methods == {"_parse_set", "text_of"},
        f"read {sorted(constants)} and {sorted(methods)}",
    )


def test_a_leaked_runtime_name_fails():
    # surfaces-review-5: the loaded module listed Native and _parse_question.
    with tempfile.TemporaryDirectory() as scratch:
        names = pathlib.Path(scratch) / "names.json"
        loaded = sorted(MODULE.ruby_names() | {"Native"})
        names.write_text(MODULE.json.dumps(loaded))
        # The check's own FAIL lines are the expected outcome here; the
        # gate counts FAIL lines, so they stay out of this suite's output.
        with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
            status = MODULE.main(["--ruby-runtime", str(names)])
        report(
            "a runtime name the source hides fails the check",
            status == 1,
            "a loaded Native passed",
        )


def main():
    test_ruby_privacy_markers_are_read()
    test_a_leaked_runtime_name_fails()
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
