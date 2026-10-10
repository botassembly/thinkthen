"""Started failure facts and owner reuse through the public language facade."""
import sys
import pathlib
import collections
import os
from pathlib import Path
import subprocess
import tempfile
from backend import Backend
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env

HERE = Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(prefix="thinkthen-failure-") as name:
    cache = Path(name)
    server = Backend(cache)
    try:
        env = child_env(home=name,
                        PATH=os.environ.get("PATH", "/usr/bin:/bin"),
                        XDG_CONFIG_HOME=name,
                        XDG_CACHE_HOME=name,
                        XDG_STATE_HOME=name)
        env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{server.server_port}/generic/v1",
                   THINKTHEN_API_KEY="tt-canary-295", THINKTHEN_CACHE=str(cache / "cache"),
                   LD_LIBRARY_PATH=str(HERE / "target"))
        result = subprocess.run([str(HERE / "target/failure")], env=env,
                                capture_output=True, text=True, timeout=40)
        assert result.returncode == 0, (result.returncode, result.stdout, result.stderr)
        assert collections.Counter(server.arrivals) == collections.Counter({"failure-two": 2, "first": 1}), server.arrivals
        print(result.stdout.strip(), "exact arrivals", len(server.arrivals))
    finally:
        server.close()
