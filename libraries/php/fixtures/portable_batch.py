"""Count exact shared Max bodies from the public PHP bulk call."""

import collections
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / "specification/fixtures/batching"
NATIVE = Path(os.environ.get("THINKTHEN_PORTABLE_NATIVE", ROOT / "libraries/c/target/debug"))
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
    with tempfile.TemporaryDirectory(prefix="thinkthen-php-portable-") as scratch:
        base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
        env = os.environ.copy()
        for name in ("THINKTHEN_API_KEY", "THINKTHEN_BASE_URL", "THINKTHEN_CACHE"):
            env.pop(name, None)
        env.update(THINKTHEN_API_KEY="sk-loopback-php-portable", THINKTHEN_BASE_URL=base,
                   TT_PORTABLE_SETTINGS=json.dumps({"base_url": base, "model": corpus["model"],
                                                    "batch": "max", "cache": False, "max_retries": 0,
                                                    "throttle": 1}),
                   TT_PORTABLE_CORPUS=str(CORPUS), TT_AUTOLOAD=str(ROOT / "libraries/php/autoload.php"),
                   TT_LIBRARY=os.environ.get("THINKTHEN_PORTABLE_LIBRARY", str(NATIVE / "lib/libthinkthen.so")),
                   HOME=scratch, THINKTHEN_CACHE=scratch,
                   LD_LIBRARY_PATH=str(NATIVE / "lib"))
        run = subprocess.run([os.environ.get("THINKTHEN_PHP_BIN", "/usr/bin/php8.3"),
                              "-d", "ffi.enable=1", "fixtures/portable_consumer.php"],
                             cwd=ROOT / "libraries/php", env=env, capture_output=True, text=True, timeout=60)
        assert run.returncode == 0 and "PHP_PORTABLE_BATCH_PASS" in run.stdout, (run.stdout, run.stderr)
    server.stdin.write("count\n")
    server.stdin.flush()
    count = int(server.stdout.readline())
    server.stdin.write("capture\n")
    server.stdin.flush()
    captured = json.loads(server.stdout.readline())
    assert count == 3 and collections.Counter(captured["bodies"]) == collections.Counter(bodies), (count, captured)
    print("php portable: five typed rows, three exact requests")
finally:
    server.stdin.close()
    server.wait(timeout=10)
