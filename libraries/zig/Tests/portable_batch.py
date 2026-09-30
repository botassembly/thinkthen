"""Focused public Zig bulk proof against the accepted literal Max corpus."""
import json
import os
from pathlib import Path
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
from portable import one_portable_request
import tempfile

PACKAGE = Path(__file__).resolve().parent.parent
ROOT = PACKAGE.parents[1]
PROJECT = Path(os.environ.get("THINKTHEN_PORTABLE_ZIG_PROJECT", PACKAGE / "Tests")).resolve()
CORPUS = ROOT / "specification/fixtures/batching/portable-records.json"
NATIVE = Path(os.environ.get("THINKTHEN_NATIVE_ROOT", PACKAGE / "target/native"))
BACKEND = ROOT / "target/debug/conformance-backend"
assert (NATIVE / "lib/libthinkthen.so").is_file() and BACKEND.is_file()
(PACKAGE / "target/scratch").mkdir(parents=True, exist_ok=True)
(PACKAGE / "target/cache").mkdir(parents=True, exist_ok=True)

zig = os.environ.get("THINKTHEN_ZIG", "zig")
subprocess.run([zig, "build", "-j2", f"-Dnative={NATIVE}", "-Dlink-mode=shared",
                "--build-file", str(PROJECT / "build.zig"),
                "--cache-dir", str(PACKAGE / "target/scratch/tests-cache"),
                "--global-cache-dir", str(PACKAGE / "target/cache"), "portable-batch"],
               cwd=PROJECT, env=child_env(HOME=str(PACKAGE / "target/home")), check=True, timeout=120)
exe = PROJECT / "zig-out/bin/portable-batch"
assert exe.is_file(), "focused Zig step did not install its consumer"
backend = subprocess.Popen([str(BACKEND)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           text=True, env={"PATH": os.environ.get("PATH", "/usr/bin:/bin")})
try:
    port = int(backend.stdout.readline())
    corpus = json.loads(CORPUS.read_text())
    with tempfile.TemporaryDirectory(prefix="zig-portable-", dir=PACKAGE / "target/scratch") as scratch:
        env = child_env()
        env.update(HOME=scratch, XDG_CACHE_HOME=scratch, XDG_CONFIG_HOME=scratch,
                   LD_LIBRARY_PATH=str(NATIVE / "lib"), THINKTHEN_CACHE=str(Path(scratch) / "cache"),
                   THINKTHEN_API_KEY="tt-portable-loopback",
                   THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/arm/full/capture/v1")
        done = subprocess.run([str(exe), corpus["question"], *corpus["texts"]],
                              cwd=PACKAGE, env=env, capture_output=True, text=True, timeout=60)
        assert done.returncode == 0 and "ZIG_PORTABLE_BATCH_PASS" in done.stderr, done.stderr
    backend.stdin.write("count\n")
    backend.stdin.flush()
    assert int(backend.stdout.readline()) == 1
    backend.stdin.write("capture\n")
    backend.stdin.flush()
    actual = [body.encode() for body in json.loads(backend.stdout.readline())["bodies"]]
    one_portable_request(actual)
    print("Zig public bulk: five values, one request with the fixture questions")
finally:
    backend.stdin.close()
    assert backend.wait(timeout=10) == 0
