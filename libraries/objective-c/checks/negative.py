"""A wrong detailed probability must fail the public Objective-C assertion."""
import os
from pathlib import Path
import subprocess
import tempfile
from backend import Backend, one_record

HERE = Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(prefix="thinkthen-objc-negative-") as name:
    work = Path(name)
    os.environ["TT_PLANT_JSON_WRONG"] = "1"
    backend = Backend(work)
    try:
        env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": name, "XDG_CONFIG_HOME": name, "XDG_CACHE_HOME": name, "XDG_STATE_HOME": name}
        env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{backend.server_port}/generic/v1",
                   THINKTHEN_API_KEY="tt-canary-295", THINKTHEN_CACHE=str(work / "cache"),
                   TT_BARRIER_DIR=str(work), TT_PLANT_JSON_WRONG="1",
                   LD_LIBRARY_PATH=str(HERE / "target"))
        result = subprocess.run([str(HERE / "target/main"), "basic"], env=env,
                                capture_output=True, text=True, timeout=40)
        assert result.returncode != 0 and "FAIL: detailed decision true at 0.9" in result.stderr, (result.returncode, result.stderr)
        assert any(one_record(request)["state"] == "json-decide" for request in backend.requests)
        print("GNU Objective-C wrong detailed answer rejected at probability assertion")
    finally:
        backend.close()
        os.environ.pop("TT_PLANT_JSON_WRONG", None)
