"""Count exact shared Max bodies from the public Go bulk call."""

import collections
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / "specification/fixtures/batching"
NATIVE = Path(os.environ["THINKTHEN_PORTABLE_NATIVE"])
MODULE = Path(os.environ.get("THINKTHEN_PORTABLE_MODULE", ROOT / "libraries/go"))
target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target"))
if not target.is_absolute():
    target = ROOT / target
BACKEND = Path(os.environ.get("THINKTHEN_BACKEND_BIN", target / "debug/conformance-backend"))
CORPUS = FIXTURES / "portable-records.json"
corpus = json.loads(CORPUS.read_text())
bodies = [(FIXTURES / f"portable-{at}.request.json").read_text().removesuffix("\n") for at in range(1, 4)]
assert corpus["schema"] == "thinkthen.portable-batch-records/1" and len(corpus["texts"]) == 5

server = subprocess.Popen([BACKEND], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
try:
    port = int(server.stdout.readline())
    with tempfile.TemporaryDirectory(prefix="thinkthen-go-portable-") as scratch:
        env = os.environ.copy()
        for name in ("THINKTHEN_API_KEY", "THINKTHEN_BASE_URL", "THINKTHEN_CACHE"):
            env.pop(name, None)
        env.update(THINKTHEN_API_KEY="sk-loopback-go-portable",
                   THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/arm/full/capture/v1",
                   TT_PORTABLE_SETTINGS=json.dumps({"base_url": f"http://127.0.0.1:{port}/arm/full/capture/v1",
                                                    "model": corpus["model"], "batch": "max", "cache": False,
                                                    "max_retries": 0, "throttle": 1}),
                   TT_PORTABLE_CORPUS=str(CORPUS), HOME=scratch, THINKTHEN_CACHE=scratch,
                   PKG_CONFIG_PATH=str(NATIVE / "lib/pkgconfig"), LD_LIBRARY_PATH=str(NATIVE / "lib"),
                   GOPROXY="off", GOSUMDB="off", GOTOOLCHAIN="local", CGO_ENABLED="1",
                   GOCACHE=str(ROOT / "target/0260-wrapper/go-cache"))
        Path(env["GOCACHE"]).mkdir(parents=True, exist_ok=True)
        run = subprocess.run([os.environ.get("THINKTHEN_GO_BIN", "go"), "test", "-count=1", "-run",
                              "^TestPortableBatch$", "-v", "."], cwd=MODULE, env=env,
                             capture_output=True, text=True, timeout=120)
        assert run.returncode == 0 and "PASS: TestPortableBatch" in run.stdout, (run.stdout, run.stderr)
    server.stdin.write("count\n")
    server.stdin.flush()
    count = int(server.stdout.readline())
    server.stdin.write("capture\n")
    server.stdin.flush()
    captured = json.loads(server.stdout.readline())
    assert count == 3 and collections.Counter(captured["bodies"]) == collections.Counter(bodies), (count, captured)
    print("go portable: five typed rows, three exact requests")
finally:
    server.stdin.close()
    server.wait(timeout=10)
