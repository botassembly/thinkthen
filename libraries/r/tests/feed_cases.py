"""Run existing saved cases through the installed R descriptor producer."""
from pathlib import Path
import os
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "libraries/python/tests"))
import native_fixture

ordinary_request = native_fixture.native_request

def producer_request(document):
    request = ordinary_request(document)
    if request["input"]["kind"] == "records":
        request["producer_items"] = request["input"]["items"]
        request["input"] = {"kind": "feed", "name": "records"}
    return request

native_fixture.native_request = producer_request
os.environ["THINKTHEN_CONFORMANCE_IDS"] = ",".join((
    "01-decide-yes-captured", "06-choose-billing", "09-tag-two", "12-score-upper",
    "13-filter-records", "15-rank-records", "18-find-second", "17-annotate-mixed",
    "41-offsets-past-an-accent-and-an-emoji", "51-same-kind-alerts"))
sys.exit(bool(native_fixture.run("r", ["Rscript", "--vanilla", str(ROOT / "libraries/r/tests/native_case.R")],
                                ROOT, {"R_LIBS": os.environ["R_LIBS"]})))
