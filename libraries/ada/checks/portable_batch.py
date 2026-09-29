"""Shared five-text Max case through the public Ada bulk procedure."""
import collections
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
NATIVE = Path(os.environ["THINKTHEN_PORTABLE_NATIVE"]).resolve()
LIB_DIR = NATIVE / "lib" if (NATIVE / "lib").is_dir() else NATIVE
LIB_NAME = "thinkthen" if LIB_DIR != NATIVE else "thinkthen_c"
FIXTURE = ROOT / "specification/fixtures/batching"
corpus = json.loads((FIXTURE / "portable-records.json").read_text())
bodies = [(FIXTURE / f"portable-{n}.request.json").read_text().removesuffix("\n") for n in range(1, 4)]
backend = Path(os.environ.get("THINKTHEN_BACKEND_BIN", ROOT / "target/debug/conformance-backend"))
assert corpus["schema"] == "thinkthen.portable-batch-records/1" and len(corpus["texts"]) == 5

with tempfile.TemporaryDirectory(prefix="thinkthen-ada-portable-") as scratch:
    target = Path(scratch)
    shutil.copytree(ROOT / "libraries/ada/src", target / "src")
    program = target / "portable_batch"
    build = subprocess.run(["gnatmake", "-gnat2022", "-I" + str(target / "src"),
                    str(HERE / "portable_batch.adb"), "-D", scratch, "-o", str(program),
                    "-largs", "-L" + str(LIB_DIR), "-l" + LIB_NAME],
                   capture_output=True, text=True, timeout=90)
    assert build.returncode == 0, (build.stdout, build.stderr)
    server = subprocess.Popen([backend], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
    try:
        port = int(server.stdout.readline())
        base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
        env = os.environ.copy()
        for name in ("THINKTHEN_API_KEY", "THINKTHEN_BASE_URL", "THINKTHEN_CACHE"):
            env.pop(name, None)
        env.update(THINKTHEN_API_KEY="sk-loopback-ada-portable", THINKTHEN_BASE_URL=base,
                   TT_PORTABLE_SETTINGS=json.dumps({"base_url": base, "model": corpus["model"],
                                                    "batch": "max", "cache": False,
                                                    "max_retries": 0, "throttle": 1}),
                   HOME=scratch, THINKTHEN_CACHE=str(target / "cache"),
                   LD_LIBRARY_PATH=str(LIB_DIR))
        run = subprocess.run([program, *corpus["texts"]], env=env, capture_output=True,
                             text=True, timeout=60)
        assert run.returncode == 0 and "ADA_PORTABLE_BATCH_PASS" in run.stdout, (run.stdout, run.stderr)
        server.stdin.write("count\n"); server.stdin.flush()
        count = int(server.stdout.readline())
        server.stdin.write("capture\n"); server.stdin.flush()
        captured = json.loads(server.stdout.readline())
        assert count == 3 and collections.Counter(captured["bodies"]) == collections.Counter(bodies), (count, captured)
        print("ada portable: five typed rows, three exact requests")
    finally:
        server.stdin.close()
        server.wait(timeout=10)
