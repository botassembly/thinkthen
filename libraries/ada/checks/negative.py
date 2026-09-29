"""A wrong first packed answer must fail the public Ada matrix assertion."""
import os
from pathlib import Path
import subprocess
import tempfile
from backend import Backend

HERE = Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(prefix="thinkthen-ada-negative-") as name:
    work = Path(name)
    os.environ["TT_ADA_PLANT"] = "wrong-packed-answer"
    backend = Backend(work)
    try:
        env = os.environ.copy()
        env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{backend.server_port}/generic/v1",
                   THINKTHEN_API_KEY="tt-canary-293", THINKTHEN_CACHE=str(work / "cache"),
                   TT_BARRIER_DIR=str(work), TT_ADA_PLANT="wrong-packed-answer",
                   LD_LIBRARY_PATH=str(HERE / "target"))
        result = subprocess.run([str(HERE / "target/legacy/main"), "bulk"], env=env,
                                capture_output=True, text=True, timeout=40)
        assert result.returncode != 0 and "bulk input order/ABI" in result.stderr, (result.returncode, result.stderr)
        assert len(backend.arrivals) == 1 and backend.arrivals[0] == "Each question quotes the text it asks about."
        print("Ada wrong packed answer rejected at bulk assertion")
    finally:
        backend.close()
        os.environ.pop("TT_ADA_PLANT", None)
