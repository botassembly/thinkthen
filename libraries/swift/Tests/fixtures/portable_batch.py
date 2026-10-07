"""Focused public Swift bulk proof against the accepted literal Max corpus."""
import json
import os
from pathlib import Path
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[4] / "conformance/children"))
from children import child_env
from backend_cases import ROWS, alias, configuration, paths
from portable import one_portable_request
import tempfile

PACKAGE = Path(__file__).resolve().parents[2]
ROOT = PACKAGE.parents[1]
SOURCE = Path(os.environ.get("THINKTHEN_PORTABLE_SWIFT_SOURCE", PACKAGE)).resolve()
CORPUS = ROOT / "specification/fixtures/batching/portable-records.json"
NATIVE = Path(os.environ.get("THINKTHEN_NATIVE_ROOT", PACKAGE / "target/native"))
BACKEND = ROOT / "target/debug/conformance-backend"
assert (NATIVE / "lib/libthinkthen.so").is_file() and BACKEND.is_file()
(PACKAGE / "target/scratch").mkdir(parents=True, exist_ok=True)

with tempfile.TemporaryDirectory(prefix="swift-portable-", dir=PACKAGE / "target/scratch") as scratch:
    scratch = Path(scratch)
    exe = scratch / "portable-batch"
    swiftc = os.environ.get("THINKTHEN_SWIFTC", "swiftc")
    subprocess.run([swiftc, "-j", "2", "-I", str(SOURCE / "Sources/CThinkThen"),
                    str(SOURCE / "Sources/ThinkThen/ThinkThen.swift"), str(SOURCE / "Sources/ThinkThen/Complete.swift"),
                    str(PACKAGE / "Tests/fixtures/portable_batch.swift"),
                    "-L", str(NATIVE / "lib"), "-lthinkthen", "-Xlinker", "-rpath",
                    "-Xlinker", str(NATIVE / "lib"), "-o", str(exe)],
                   env=child_env(keep=("SWIFTPM_MODULECACHE_OVERRIDE",), HOME=str(PACKAGE / "target/home")), check=True, timeout=120)
    for named in (False, True):
        backend = subprocess.Popen([str(BACKEND)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   text=True, env=child_env(THINKTHEN_TEST_MARKERS=json.dumps({"local":"tt-named-loopback"})))
        try:
            port = int(backend.stdout.readline())
            env = child_env()
            env.update(HOME=str(scratch), XDG_CACHE_HOME=str(scratch), XDG_CONFIG_HOME=str(scratch),
                       LD_LIBRARY_PATH=str(NATIVE / "lib"), THINKTHEN_CACHE=str(scratch / "cache"),
                       THINKTHEN_API_KEY="tt-portable-loopback",
                       THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/arm/full/capture/v1")
            if named:
                row = next(row for row in ROWS if row["name"] == "typesafe")
                env["XDG_CONFIG_HOME"] = env["HOME"]
                base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
                configuration(env, {"local": alias(row, base)})
                env[row["key"]] = "tt-named-loopback"
                env["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{port}/generic/v1"
                env["TT_PORTABLE_SETTINGS"] = json.dumps({"backend":"local", "batch":"max", "cache":False, "max_retries":0, "throttle":1})
            done = subprocess.run([str(exe), str(CORPUS)], cwd=PACKAGE, env=env,
                                  capture_output=True, text=True, timeout=60)
            assert done.returncode == 0 and "SWIFT_PORTABLE_BATCH_PASS" in done.stdout, done.stderr
            backend.stdin.write("count\n")
            backend.stdin.flush()
            assert int(backend.stdout.readline()) == 1
            backend.stdin.write("capture\n")
            backend.stdin.flush()
            actual = [body.encode() for body in json.loads(backend.stdout.readline())["bodies"]]
            one_portable_request(actual)
            if named:
                for command, expected in (("paths", paths(row["path"])),
                                          ("bearers", {"markers":{"local":1},"absent":0,"unknown":0,"overflow":False})):
                    backend.stdin.write(command + "\n"); backend.stdin.flush()
                    assert json.loads(backend.stdout.readline()) == expected, command
                assert "tt-named-loopback" not in done.stdout + done.stderr
                assert b"tt-named-loopback" not in b"".join(actual)
                print("swift named backend: selected path, bearer, result and secrecy PASS")
            print("Swift public bulk: five values, one request with the fixture questions")
        finally:
            backend.stdin.close()
            assert backend.wait(timeout=10) == 0
