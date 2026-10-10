"""Focused public Zig bulk proof against the accepted literal Max corpus."""
import json
import os
from pathlib import Path
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
from backend_cases import ROWS, alias, configuration, paths
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
for named in (False, True):
    backend = subprocess.Popen([str(BACKEND)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                               text=True, env=child_env(THINKTHEN_TEST_MARKERS=json.dumps({"local":"tt-named-loopback"})))
    try:
        port = int(backend.stdout.readline())
        corpus = json.loads(CORPUS.read_text())
        with tempfile.TemporaryDirectory(prefix="zig-portable-", dir=PACKAGE / "target/scratch") as scratch:
            env = child_env(home=scratch, XDG_CACHE_HOME=scratch, XDG_CONFIG_HOME=scratch,
                       LD_LIBRARY_PATH=str(NATIVE / "lib"), THINKTHEN_CACHE=str(Path(scratch) / "cache"),
                       THINKTHEN_API_KEY="tt-portable-loopback",
                       THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/arm/full/capture/v1")
            if named:
                row = next(row for row in ROWS if row["name"] == "typesafe")
                base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
                configuration(env, {"local": alias(row, base)})
                env[row["key"]] = "tt-named-loopback"
                env["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{port}/generic/v1"
                env["TT_PORTABLE_SETTINGS"] = json.dumps({"backend":"local", "batch":"max", "cache":False, "max_retries":0, "throttle":1})
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
        if named:
            for command, expected in (("paths", paths(row["path"])),
                                      ("bearers", {"markers":{"local":1},"absent":0,"unknown":0,"overflow":False})):
                backend.stdin.write(command + "\n"); backend.stdin.flush()
                assert json.loads(backend.stdout.readline()) == expected, command
            assert "tt-named-loopback" not in done.stdout + done.stderr
            assert b"tt-named-loopback" not in b"".join(actual)
            print("zig named backend: selected path, bearer, result and secrecy PASS")
        print("Zig public bulk: five values, one request with the fixture questions")
    finally:
        backend.stdin.close()
        assert backend.wait(timeout=10) == 0
