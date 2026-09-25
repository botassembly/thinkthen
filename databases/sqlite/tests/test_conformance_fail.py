#!/usr/bin/env python3
"""The conformance runner fails a mismatch and an unknown form by name (R3-30, R5-32).

A planted copy of the cases file holds two cases: case 02 with its expected
answer flipped, and the same case under a verb with no SQL form written. A
runner that reported either as "not run", or skipped one, would pass the
planted copy. This test needs both named as FAIL and no traceback.
"""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys
import tempfile

from helper import HERE, ROOT, environment, expect, main


def test_a_mismatch_and_an_unknown_form_each_fail_by_name() -> None:
    document = json.loads((ROOT.parents[1] / "conformance" / "cases.json").read_text())
    flipped = next(case for case in document["cases"] if case["id"] == "02-decide-no")
    flipped["expect"]["success"]["answers"][0]["bare"] = True
    unknown = json.loads(json.dumps(flipped)) | {"id": "02-summarize", "verb": "summarize"}
    unknown["expect"]["success"]["kind"] = "summary"
    document.update(cases=[flipped, unknown], case_count=2)
    planted = pathlib.Path(tempfile.mkdtemp(prefix="thinkthen-plant-")) / "cases.json"
    planted.write_text(json.dumps(document))
    done = subprocess.run(
        [sys.executable, str(HERE / "conformance.py"), str(planted)],
        capture_output=True, text=True, check=False, env=environment(None), timeout=120,
    )
    said = done.stdout + done.stderr
    expect(done.returncode, 1, f"the exit code: {said}")
    expect("FAIL     02-decide-no: value: got false, expected true" in said, True, said)
    expect("FAIL     02-summarize: no SQL form is written for the summary kind" in said, True, said)
    expect("0 pass, 2 FAIL, 0 not run, 2 of 2" in said and "Traceback" not in said, True, said)


if __name__ == "__main__":
    sys.exit(main(globals()))
