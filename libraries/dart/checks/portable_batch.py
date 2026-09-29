"""One public Dart bulk call against the shared literal Max corpus."""
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
CORPUS = ROOT / "specification/fixtures/batching/portable-records.json"
EXPECTED = [(ROOT / f"specification/fixtures/batching/portable-{n}.request.json")
            .read_bytes().removesuffix(b"\n") for n in (1, 2, 3)]
NATIVE = Path(os.environ["TT_NATIVE_LIBRARY"])
DART = os.environ["TT_DART"]
BACKEND = ROOT / "target/debug/conformance-backend"
assert NATIVE.is_file() and BACKEND.is_file(), "select a built native library and backend"
(HERE / "scratch").mkdir(parents=True, exist_ok=True)

backend = subprocess.Popen([str(BACKEND)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           text=True, env={"PATH": os.environ.get("PATH", "/usr/bin:/bin")})
try:
    port = int(backend.stdout.readline())
    with tempfile.TemporaryDirectory(prefix="dart-portable-", dir=HERE / "scratch") as scratch:
        env = os.environ.copy()
        env.pop("THINKTHEN_API_KEY", None)
        env.update(HOME=scratch, XDG_CACHE_HOME=scratch, XDG_CONFIG_HOME=scratch,
                   THINKTHEN_CACHE=str(Path(scratch) / "cache"),
                   THINKTHEN_API_KEY="tt-portable-loopback",
                   THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/arm/full/capture/v1")
        done = subprocess.run([DART, "run", "bin/portable_batch.dart", str(NATIVE), str(CORPUS)],
                              cwd=HERE / "consumers/alpha", env=env, capture_output=True,
                              text=True, timeout=60)
        assert done.returncode == 0 and "DART_PORTABLE_BATCH_PASS" in done.stdout, done.stderr
    backend.stdin.write("count\n")
    backend.stdin.flush()
    assert int(backend.stdout.readline()) == 3
    backend.stdin.write("capture\n")
    backend.stdin.flush()
    actual = [body.encode() for body in json.loads(backend.stdout.readline())["bodies"]]
    assert sorted(actual) == sorted(EXPECTED), "literal body membership changed"
    print("Dart public bulk: five values, three literal bodies and sends")
finally:
    backend.stdin.close()
    assert backend.wait(timeout=10) == 0
