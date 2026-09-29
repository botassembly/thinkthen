"""Started failure facts and owner reuse through the public language facade."""
import collections
import os
from pathlib import Path
import subprocess
import tempfile
from backend import Backend

HERE = Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(prefix="thinkthen-failure-") as name:
    cache = Path(name)
    server = Backend(cache)
    try:
        env = os.environ.copy()
        env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{server.server_port}/generic/v1",
                   THINKTHEN_API_KEY="tt-canary-293", THINKTHEN_CACHE=str(cache / "cache"),
                   LD_LIBRARY_PATH=str(HERE / "target"))
        result = subprocess.run([str(HERE / "target/failure")], env=env,
                                capture_output=True, text=True, timeout=40)
        assert result.returncode == 0, (result.returncode, result.stdout, result.stderr)
        assert collections.Counter(server.arrivals) == collections.Counter({"failure-two": 1, "first": 1}), server.arrivals
        print(result.stdout.strip(), "exact arrivals", len(server.arrivals))
    finally:
        server.close()
