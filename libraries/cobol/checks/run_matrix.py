"""Owned-process counted runner. No inherited credentials or non-loopback backend."""
import datetime
import json
import os
import pathlib
import signal
import subprocess
import sys
import time
import threading
from collections import Counter
from backend import Backend, one_record

here = pathlib.Path(__file__).resolve().parent
repo = here.parents[2]
logs = here / "logs" / ("run-" + datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ"))
logs.mkdir(parents=True)
barrier = logs / "barrier"
barrier.mkdir()
home = logs / "home"
(home / "cache").mkdir(parents=True)
backend = Backend(barrier)
env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": str(home), "XDG_CONFIG_HOME": str(home),
       "XDG_CACHE_HOME": str(home), "LD_LIBRARY_PATH": str(here / "target"),
       "THINKTHEN_BASE_URL": f"http://127.0.0.1:{backend.server_port}/generic/v1",
       "THINKTHEN_API_KEY": "tt-fixture-key", "THINKTHEN_CACHE": str(home / "cache"),
       "TT_BARRIER_DIR": str(barrier)}
receipts = []
active = None

# The deadline ends the held call while its reply is held. The matrix sends
# its next request only after that call returns, so the release waits for
# that next arrival, up to 30 s (ticket 0356).
def release_deadline():
    marker = barrier / "arrived-hold-deadline"
    end = time.monotonic() + 60
    while not marker.exists() and time.monotonic() < end:
        time.sleep(.005)
    if marker.exists():
        seen = len(backend.arrivals)
        end = time.monotonic() + 30
        while len(backend.arrivals) <= seen and time.monotonic() < end:
            time.sleep(.005)
        (barrier / "release-hold-deadline").touch()

release_worker = threading.Thread(target=release_deadline, daemon=True)
release_worker.start()

def verify_json_shapes():
    lines = (logs / "matrix.log").read_text().splitlines()
    if lines.count("all-cobol-cases-pass") != 1:
        return False
    outputs = {}
    for line in lines:
        if line.startswith("JSON "):
            _, name, payload = line.split(" ", 2)
            if name in outputs:
                return False
            outputs[name] = json.loads(payload)
    required = {"json-decide", "choose", "choose-map", "tag", "score", "filter", "rank", "find", "annotate",
                "recognize", "relate", "usage", "typed-recognize", "typed-relate", "json-call-opts"}
    if set(outputs) != required:
        return False
    from jsonschema import Draft202012Validator
    schema = json.loads((repo / "specification/result.schema.json").read_text())
    validator = Draft202012Validator({"$schema": schema["$schema"], "$defs": schema["$defs"], "$ref": "#/$defs/callSuccess"})
    asking = required - {"usage", "typed-recognize", "typed-relate", "json-call-opts"}
    for name in asking:
        record = outputs[name]
        validator.validate(record)  # exact top-level {value,facts}; facts schema typed.
        if set(record) != {"value", "facts"} or record["facts"]["requests_sent"] < 1:
            return False
    values = {name: outputs[name]["value"] for name in asking}
    detail = values["json-decide"]
    return (set(detail) == {"schema", "value", "question", "answer", "threshold", "meta"}
        and detail["schema"] == "thinkthen.result/1"
        and detail["value"] is True
        and detail["question"] == {"verb": "decide", "text": "Is it?"}
        and detail["answer"] == {"kind": "yes_no", "probability": .9}
        and detail["threshold"] == .5
        and values["choose"] == "first"
        and values["choose-map"] == "first"
        and values["tag"] == ["first", "second"]
        and values["score"] == .1
        and values["filter"] == ["filter-one", "filter-two"]
        and values["rank"] == ["rank-one", "rank-two"]
        and values["find"] == {"index": 0, "unit": "find-one", "probability": .9}
        and values["annotate"] == [{"check": True}]
        and values["recognize"]["entities"][0]["text"] == "Maria Chen"
        and values["recognize"]["entities"][0]["length"] == 10
        and values["recognize"]["entities"][0]["strength"] == .81
        and len(values["relate"]["edges"]) == 2
        and outputs["typed-recognize"]["entities"][0]["text"] == "John Smith"
        and len(outputs["typed-relate"]["edges"]) == 2
        and isinstance(outputs["usage"]["requests_sent"], int)
        and isinstance(outputs["json-call-opts"]["requests_sent"], int))

def stop_group(p):
    signals = []
    for sig in (signal.SIGTERM, signal.SIGKILL):
        try:
            os.killpg(p.pid, sig)
            signals.append(sig.name)
        except ProcessLookupError:
            pass
        if sig == signal.SIGTERM:
            time.sleep(.2)
    p.wait(timeout=5)
    return signals

def execute(label, command, timeout=90, extra=None):
    global active
    with (logs / (label + ".log")).open("wb") as output:
        active = subprocess.Popen(command, cwd=here, env=env | (extra or {}), stdout=output,
                                  stderr=subprocess.STDOUT, start_new_session=True)
        r = {"case": label, "pid": active.pid, "pgid": active.pid,
             "command": command, "timeout_seconds": timeout, "signals": []}
        receipts.append(r)
        (logs / "receipts.json").write_text(json.dumps(receipts, indent=2) + "\n")
        try:
            r["exit"] = active.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            r["signals"] = stop_group(active)
            r["exit"] = "timeout"
        except BaseException:
            r["signals"] = stop_group(active)
            r["exit"] = "interrupted"
            raise
        finally:
            active = None
            (logs / "receipts.json").write_text(json.dumps(receipts, indent=2) + "\n")
    return r["exit"]

status = "interrupted"
try:
    status = execute("direct", [str(here / "target/direct")])
    if status == 0:
        status = execute("matrix", [str(here / "target/matrix")], 120)
    if status == 0:
        status = "SHAPE_MISMATCH" if not verify_json_shapes() else 0
    if status == 0:
        status = execute("settings", [str(here / "target/settings")])
        if status == 0 and "COBOL_SETTINGS_PASS" not in (logs / "settings.log").read_text(): status = "SETTINGS_MARKER_MISSING"
    if status == 0:
        status = execute("types", [str(here / "target/types")])
        if status == 0 and "COBOL_TYPES_PASS" not in (logs / "types.log").read_text(): status = "TYPES_MARKER_MISSING"
    if status == 0:
        status = execute("strict-engine", [sys.executable, str(here / "strict_engine.py")], 60)
    if status == 0 and "STRICT_ENGINE_CANCEL_PASS" not in (logs / "strict-engine.log").read_text():
        status = "STRICT_MARKER_MISSING"
    if status == 0:
        expected = json.loads((here / "expected-arrivals.json").read_text())
        actual = backend.arrivals
        golden = json.loads((here / "expected-requests.json").read_text())
        bodies = [json.loads(path.read_text()) for path in sorted(barrier.glob("request-*.json"))]
        normalize = lambda item: json.dumps(item, sort_keys=True, ensure_ascii=False, separators=(",", ":"))
        def key(state):
            return json.dumps(state, sort_keys=True, ensure_ascii=False)
        status = "PASS" if Counter(map(key, actual)) == Counter(map(key, expected)) and Counter(map(normalize, bodies)) == Counter(map(normalize, golden)) else "COUNT_MISMATCH"
        if status == "PASS":
            # ADR 0055: one packed request carries three distinct records.
            # Result order is asserted in matrix.cob; a reverse-completion
            # assertion on three independent requests no longer applies.
            packed = "Each question quotes the text it asks about."
            packed_requests = []
            for p in sorted(barrier.glob("request-*.json")):
                body = json.loads(p.read_text())
                if one_record(body)["state"] == packed:
                    packed_requests.append({k: q["instructions"] for k, q in body["questions"].items()})
            if packed_requests != [
                {"q1": 'The text is "first". Is it?', "q2": 'The text is "second". Is it?', "q3": 'The text is "third". Is it?'},
                {"q1": 'The text is "filter-one". Is it?', "q2": 'The text is "filter-two". Is it?'},
                {"q1": 'The text is "rank-one". Is it?', "q2": 'The text is "rank-two". Is it?'},
            ]:
                status = "PACKED_QUESTION_IDENTITY_MISMATCH"
            else:
                print("REPIN-ADAPTED-PACKING-AND-RESULT: three exact packed request bodies + exact 30-arrival multiset", flush=True)
finally:
    if active is not None:
        stop_group(active)
    (barrier / "release-hold-deadline").touch()
    release_worker.join(timeout=1)
    backend.close()
    (logs / "completions.json").write_text(json.dumps(backend.completions, ensure_ascii=False, indent=2) + "\n")
    (logs / "arrivals.json").write_text(json.dumps(backend.arrivals, ensure_ascii=False, indent=2) + "\n")
    (logs / "outcome.json").write_text(json.dumps({"status": status, "arrivals": len(backend.arrivals),
        "receipts": receipts}, indent=2) + "\n")
    print(json.dumps({"status": status, "logs": str(logs), "arrivals": len(backend.arrivals)}))
if status != "PASS":
    if os.environ.get("TT_PLANT_PACKED_SWAP") == "1" and status != 0 and (logs / "matrix.log").exists() and "FAIL typed-bulk " in (logs / "matrix.log").read_text():
        print("PACKED_NEGATIVE_ASSERTED: first/second probabilities swapped; literal COBOL row values refused")
    sys.exit(1)
