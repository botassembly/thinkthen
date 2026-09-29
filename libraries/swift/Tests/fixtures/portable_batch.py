"""Focused public Swift bulk proof against the accepted literal Max corpus."""
import json
import os
from pathlib import Path
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[4] / "conformance/children"))
from children import child_env
import tempfile

PACKAGE = Path(__file__).resolve().parents[2]
ROOT = PACKAGE.parents[1]
SOURCE = Path(os.environ.get("THINKTHEN_PORTABLE_SWIFT_SOURCE", PACKAGE)).resolve()
CORPUS = ROOT / "specification/fixtures/batching/portable-records.json"
EXPECTED = [(ROOT / f"specification/fixtures/batching/portable-{n}.request.json")
            .read_bytes().removesuffix(b"\n") for n in (1, 2, 3)]
NATIVE = Path(os.environ.get("THINKTHEN_NATIVE_ROOT", PACKAGE / "target/native"))
BACKEND = ROOT / "target/debug/conformance-backend"
assert (NATIVE / "lib/libthinkthen.so").is_file() and BACKEND.is_file()
(PACKAGE / "target/scratch").mkdir(parents=True, exist_ok=True)

with tempfile.TemporaryDirectory(prefix="swift-portable-", dir=PACKAGE / "target/scratch") as scratch:
    scratch = Path(scratch)
    exe = scratch / "portable-batch"
    swiftc = os.environ.get("THINKTHEN_SWIFTC", "swiftc")
    subprocess.run([swiftc, "-j", "2", "-I", str(SOURCE / "Sources/CThinkThen"),
                    str(SOURCE / "Sources/ThinkThen/ThinkThen.swift"),
                    str(PACKAGE / "Tests/fixtures/portable_batch.swift"),
                    "-L", str(NATIVE / "lib"), "-lthinkthen", "-Xlinker", "-rpath",
                    "-Xlinker", str(NATIVE / "lib"), "-o", str(exe)],
                   env=child_env(keep=("SWIFTPM_MODULECACHE_OVERRIDE",), HOME=str(PACKAGE / "target/home")), check=True, timeout=120)
    backend = subprocess.Popen([str(BACKEND)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                               text=True, env={"PATH": os.environ.get("PATH", "/usr/bin:/bin")})
    try:
        port = int(backend.stdout.readline())
        env = child_env()
        env.update(HOME=str(scratch), XDG_CACHE_HOME=str(scratch), XDG_CONFIG_HOME=str(scratch),
                   LD_LIBRARY_PATH=str(NATIVE / "lib"), THINKTHEN_CACHE=str(scratch / "cache"),
                   THINKTHEN_API_KEY="tt-portable-loopback",
                   THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/arm/full/capture/v1")
        done = subprocess.run([str(exe), str(CORPUS)], cwd=PACKAGE, env=env,
                              capture_output=True, text=True, timeout=60)
        assert done.returncode == 0 and "SWIFT_PORTABLE_BATCH_PASS" in done.stdout, done.stderr
        backend.stdin.write("count\n")
        backend.stdin.flush()
        assert int(backend.stdout.readline()) == 3
        backend.stdin.write("capture\n")
        backend.stdin.flush()
        actual = [body.encode() for body in json.loads(backend.stdout.readline())["bodies"]]
        assert sorted(actual) == sorted(EXPECTED), "literal body membership changed"
        print("Swift public bulk: five values, three literal bodies and sends")
    finally:
        backend.stdin.close()
        assert backend.wait(timeout=10) == 0
