#!/usr/bin/env python3
"""The conformance driver prints a counted FAIL line when the engine
refuses a case, and never a traceback.

A relate refusal reaches Python as sqlite3.IntegrityError. The driver
once caught only OperationalError, so a refused relate case crashed the
run instead of failing it by name. This test plants a copy of the
conformance file whose first relate case asks a both-ways rule one way,
which the stand-in refuses, and runs the driver on it.
Run through ./check.sh, after the extension is built.
"""
import json
import os
import pathlib
import subprocess
import sys
import tempfile

HERE = pathlib.Path(__file__).resolve().parent
CASES = HERE.parents[2] / "conformance" / "conformance.json"

data = json.loads(CASES.read_text())
relate = next(
    case for case in data["cases"]
    if case["verb"] == "relate" and any(rule.get("either") for rule in case["question"]["relations"])
)
for rule in relate["question"]["relations"]:
    rule.pop("either", None)
data["cases"] = [relate]
data["case_count"] = 1
data["skips"] = []
with tempfile.TemporaryDirectory() as scratch:
    planted = pathlib.Path(scratch) / "planted.json"
    planted.write_text(json.dumps(data))
    done = subprocess.run(
        [sys.executable, str(HERE / "conformance_driver.py")],
        capture_output=True, text=True, check=False, stdin=subprocess.DEVNULL,
        env={**os.environ, "ENGINE_NULL": "1", "THINKTHEN_CONFORMANCE_FILE": str(planted)},
    )
out = done.stdout + done.stderr
ok = (
    done.returncode == 1
    and f"FAIL     {relate['id']}: unexpected:" in out
    and "Traceback" not in out
    and "conformance slice failed" in out
)
print(("ok       " if ok else "FAIL     ") + "a refused relate case fails by name, with no traceback")
if not ok:
    print(out[-2000:])
sys.exit(0 if ok else 1)
