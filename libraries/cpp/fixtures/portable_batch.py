"""Count exact shared Max bodies from the public C++ bulk call."""

import json
import os
from pathlib import Path
import shutil
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
CPP_INCLUDE = Path(os.environ.get("THINKTHEN_PORTABLE_CPP_INCLUDE", ROOT / "libraries/cpp/include"))
target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target"))
if not target.is_absolute():
    target = ROOT / target
BACKEND = Path(os.environ.get("THINKTHEN_BACKEND_BIN", target / "debug/conformance-backend"))
CORPUS = FIXTURES / "portable-records.json"
corpus = json.loads(CORPUS.read_text())
assert corpus["schema"] == "thinkthen.portable-batch-records/1" and len(corpus["texts"]) == 5

with tempfile.TemporaryDirectory(prefix="thinkthen-cpp-portable-") as scratch:
    folder = Path(scratch)
    include = folder / "include/thinkthen"
    include.mkdir(parents=True)
    header = NATIVE / "include/thinkthen.h"
    if not header.exists():
        header = NATIVE / "include/thinkthen/thinkthen.h"
    shutil.copyfile(header, include / "thinkthen.h")
    consumer = folder / "portable-consumer"
    library = NATIVE / "lib/libthinkthen.so"
    if not library.exists():
        library = NATIVE / "lib/libthinkthen.so.0"
    subprocess.run([os.environ.get("CXX", "c++"), "-std=c++17", "-I", str(CPP_INCLUDE),
                    "-I", str(folder / "include"), str(ROOT / "libraries/cpp/fixtures/portable_consumer.cpp"),
                    "-L", str(NATIVE / "lib"), "-Wl,-rpath," + str(NATIVE / "lib"), "-l:" + library.name,
                    "-o", str(consumer)], env=child_env(), check=True, timeout=60)
    for named in (False, True):
        server = subprocess.Popen([BACKEND], env=child_env(THINKTHEN_TEST_MARKERS=json.dumps({"local":"tt-named-loopback"})), stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
        try:
            port = int(server.stdout.readline())
            base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
            env = child_env()
            env.update(THINKTHEN_API_KEY="sk-loopback-cpp-portable", THINKTHEN_BASE_URL=base,
                       TT_PORTABLE_SETTINGS=json.dumps({"base_url": base, "model": corpus["model"],
                                                        "batch": "max", "cache": False, "max_retries": 0,
                                                        "throttle": 1}),
                       TT_PORTABLE_CORPUS=str(CORPUS), HOME=scratch, THINKTHEN_CACHE=str(folder / "cache"),
                       LD_LIBRARY_PATH=str(NATIVE / "lib"))
            if named:
                row = next(row for row in ROWS if row["name"] == "typesafe")
                env["XDG_CONFIG_HOME"] = env["HOME"]
                base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
                configuration(env, {"local": alias(row, base)})
                env[row["key"]] = "tt-named-loopback"
                env["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{port}/generic/v1"
                env["TT_PORTABLE_SETTINGS"] = json.dumps({"backend":"local", "batch":"max", "cache":False, "max_retries":0, "throttle":1})
            run = subprocess.run([consumer], env=env, capture_output=True, text=True, timeout=60)
            assert run.returncode == 0 and "CPP_PORTABLE_BATCH_PASS" in run.stdout, (run.stdout, run.stderr)
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
                print("cpp named backend: selected path, bearer, result and secrecy PASS")
            print("cpp portable: five typed rows, one request with the fixture questions")
        finally:
            server.stdin.close()
            server.wait(timeout=10)
