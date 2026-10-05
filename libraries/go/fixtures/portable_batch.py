"""Count exact shared Max bodies from the public Go bulk call."""

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
assert corpus["schema"] == "thinkthen.portable-batch-records/1" and len(corpus["texts"]) == 5

for named in (False, True):
    server = subprocess.Popen([BACKEND], env=child_env(THINKTHEN_TEST_MARKERS=json.dumps({"local":"tt-named-loopback"})), stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
    try:
        port = int(server.stdout.readline())
        with tempfile.TemporaryDirectory(prefix="thinkthen-go-portable-") as scratch:
            env = child_env(HOME=scratch, GOMODCACHE=str(Path(scratch) / "modcache"))
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
            if named:
                row = next(row for row in ROWS if row["name"] == "typesafe")
                env["XDG_CONFIG_HOME"] = env["HOME"]
                base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
                configuration(env, {"local": alias(row, base)})
                env[row["key"]] = "tt-named-loopback"
                env["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{port}/generic/v1"
                env["TT_PORTABLE_SETTINGS"] = json.dumps({"backend":"local", "batch":"max", "cache":False, "max_retries":0, "throttle":1})
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
        assert count == 1, (count, captured)
        one_portable_request(captured["bodies"])
        if named:
            for command, expected in (("paths", paths(row["path"])),
                                      ("bearers", {"markers":{"local":1},"absent":0,"unknown":0,"overflow":False})):
                server.stdin.write(command + "\n"); server.stdin.flush()
                assert json.loads(server.stdout.readline()) == expected, command
            assert "tt-named-loopback" not in run.stdout + run.stderr
            assert "tt-named-loopback" not in json.dumps(captured["bodies"])
            print("go named backend: selected path, bearer, result and secrecy PASS")
        print("go portable: five typed rows, one request with the fixture questions")
    finally:
        server.stdin.close()
        server.wait(timeout=10)
