"""Check copied started-failure facts after repeated public COBOL calls, and that reused caller buffers stay whole."""
import collections
import json
import os
from pathlib import Path
import subprocess
import tempfile
from backend import Backend

HERE = Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(prefix="thinkthen-cobol-failure-") as name:
    work = Path(name)
    server = Backend(work)
    try:
        env = os.environ.copy()
        env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{server.server_port}/generic/v1",
                   THINKTHEN_API_KEY="tt-fixture-key", THINKTHEN_CACHE=str(work / "cache"),
                   LD_LIBRARY_PATH=str(HERE / "target"))
        result = subprocess.run([str(HERE / "target/failure")], cwd=work, env=env,
                                capture_output=True, text=True, timeout=40)
        assert result.returncode == 0, (result.returncode, result.stdout, result.stderr)
        lines = result.stdout.splitlines()
        assert lines[-1] == "COBOL_FAILURE_OWNERSHIP_PASS", lines
        facts = json.loads(next(line[6:] for line in lines if line.startswith("FACTS ")))
        assert isinstance(facts, dict) and facts["requests_sent"] == 1, facts
        assert collections.Counter(server.arrivals) == collections.Counter({"failure-two": 2, "first": 1, "decide-again": 2, "call-again": 2, "decide-full": 1}), server.arrivals
        print("COBOL copied failure facts and reused caller buffers: 8 exact arrivals")
    finally:
        server.close()
