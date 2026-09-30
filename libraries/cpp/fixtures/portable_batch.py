"""Count exact shared Max bodies from the public C++ bulk call."""

import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
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
    server = subprocess.Popen([BACKEND], env=child_env(), stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
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
        print("cpp portable: five typed rows, one request with the fixture questions")
    finally:
        server.stdin.close()
        server.wait(timeout=10)
