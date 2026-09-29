"""Public Ada facade task cancellation, bulk order, deadline and recovery."""
import collections
import json
import os
from pathlib import Path
import subprocess
import tempfile
from backend import Backend

HERE = Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(prefix="thinkthen-ada-typed-") as name:
    work = Path(name)
    backend = Backend(work)
    try:
        env = os.environ.copy()
        env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{backend.server_port}/generic/v1",
                   THINKTHEN_API_KEY="tt-canary-293", THINKTHEN_CACHE=str(work / "cache"),
                   TT_BARRIER_DIR=str(work), LD_LIBRARY_PATH=str(HERE / "target"))
        result = subprocess.run([str(HERE / "target/package_bulk")], env=env,
                                capture_output=True, text=True, timeout=90)
        assert result.returncode == 0 and "TYPED_ADA_MATRIX_PASS" in result.stdout, (result.stdout, result.stderr)
        shared = "Each question quotes the text it asks about."
        relation = {"entities": [{"id": "i1", "name": "Third", "kind": "alert"},
                                 {"id": "i2", "name": "Fourth", "kind": "alert"}]}
        key = lambda value: json.dumps(value, sort_keys=True)
        expected = [shared, shared, "hold-scalar", "hold-deadline", "recovery-package",
                    "John Smith", "John Smith", relation, "status-401", "after-error"]
        assert collections.Counter(map(key, backend.arrivals)) == collections.Counter(map(key, expected)), backend.arrivals
        bodies = [json.loads(line) for line in (work / "request-bodies.jsonl").read_text().splitlines()]
        golden = json.loads((HERE / "expected-requests.json").read_text())
        assert collections.Counter(key(b) for b in bodies if b["state"] in ("John Smith", relation)) == collections.Counter(
            key(b) for b in golden if b["state"] in ("John Smith", relation)), "typed structured request bytes"
        packed = [b["questions"] for b in bodies if b["state"] == shared]
        assert packed == [
            {f"q{i}": {"instructions": f'The text is "{row}". Is it?', "type": "noul"} for i,row in enumerate(("first","second","third"),1)},
            {f"q{i}": {"instructions": f'The text is "{row}". Is it?', "type": "noul"} for i,row in enumerate(("hold-bulk-1","hold-bulk-2"),1)}
        ], packed
        boundary = subprocess.run([str(HERE / "target/facts_boundary")], env=env,
                                  capture_output=True, text=True, timeout=20)
        assert boundary.returncode == 0 and "ADA_FACTS_BOUNDARY_PASS" in boundary.stdout, (
            boundary.stdout, boundary.stderr)
        assert len(backend.arrivals) == 10, "controlled facts conversion made no provider request"
        print("Ada public typed matrix: 10 exact arrivals, four typed routes, held cancellation and owned failure")
    finally:
        backend.close()
