"""Shared five-text Max case through the public GNU Objective-C bulk method."""
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
HERE = Path(__file__).resolve().parent
PACKAGE = Path(os.environ.get("THINKTHEN_PORTABLE_PACKAGE", ROOT / "libraries/objective-c")).resolve()
INSTALLED = "THINKTHEN_PORTABLE_PACKAGE" in os.environ
SOURCE = PACKAGE / "Sources"
assert (SOURCE / "ThinkThen.m").is_file(), "Objective-C package source missing"
NATIVE = Path(os.environ["THINKTHEN_PORTABLE_NATIVE"]).resolve()
LIB_DIR = NATIVE / "lib" if (NATIVE / "lib").is_dir() else NATIVE
LIB_NAME = "thinkthen" if LIB_DIR != NATIVE else "thinkthen_c"
INCLUDE = NATIVE / "include" if (NATIVE / "include").is_dir() else ROOT / "libraries/c/include"
FIXTURE = ROOT / "specification/fixtures/batching"
corpus = json.loads((FIXTURE / "portable-records.json").read_text())
backend = Path(os.environ.get("THINKTHEN_BACKEND_BIN", ROOT / "target/debug/conformance-backend"))
assert corpus["schema"] == "thinkthen.portable-batch-records/1" and len(corpus["texts"]) == 5

with tempfile.TemporaryDirectory(prefix="thinkthen-objc-portable-") as scratch:
    target = Path(scratch)
    shutil.copytree(SOURCE, target / "Sources")
    source = target / "Sources"
    program = target / "portable_batch"
    subprocess.run(["gcc", "-std=gnu11", "-x", "objective-c", "-I" + str(INCLUDE), "-I" + str(source),
                    str(source / "ThinkThen.m"), str(source / "TTJSON.c"),
                    str(HERE / "portable_batch.m"), "-L" + str(LIB_DIR),
                    "-l" + LIB_NAME, "-lobjc", "-pthread", "-lm", "-o", str(program)],
                   env=child_env(), check=True, capture_output=True, text=True, timeout=60)
    if INSTALLED:
        linked = subprocess.check_output(["ldd", program], env=child_env(LD_LIBRARY_PATH=str(LIB_DIR)), text=True)
        assert str(LIB_DIR / "libthinkthen.so.0") in linked, linked
        print("objc installed loader:", next(line.strip() for line in linked.splitlines() if "libthinkthen.so.0" in line))
    for named in (False, True):
        server = subprocess.Popen([backend], env=child_env(THINKTHEN_TEST_MARKERS=json.dumps({"local":"tt-named-loopback"})), stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
        try:
            port = int(server.stdout.readline())
            base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
            env = child_env(home=scratch, XDG_CONFIG_HOME=scratch if named else str(Path(scratch) / "config"))
            env.update(THINKTHEN_API_KEY="sk-loopback-objc-portable", THINKTHEN_BASE_URL=base,
                       TT_PORTABLE_SETTINGS=json.dumps({"base_url": base, "model": corpus["model"],
                                                        "batch": "max", "cache": False,
                                                        "max_retries": 0, "throttle": 1}),
                       THINKTHEN_CACHE=str(target / "cache"),
                       LD_LIBRARY_PATH=str(LIB_DIR))
            if named:
                row = next(row for row in ROWS if row["name"] == "typesafe")
                base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
                configuration(env, {"local": alias(row, base)})
                env[row["key"]] = "tt-named-loopback"
                env["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{port}/generic/v1"
                env["TT_PORTABLE_SETTINGS"] = json.dumps({"backend":"local", "batch":"max", "cache":False, "max_retries":0, "throttle":1})
            run = subprocess.run([program, *corpus["texts"]], env=env, capture_output=True,
                                 text=True, timeout=60)
            assert run.returncode == 0 and "OBJC_PORTABLE_BATCH_PASS" in run.stdout, (run.stdout, run.stderr)
            server.stdin.write("count\n"); server.stdin.flush()
            count = int(server.stdout.readline())
            server.stdin.write("capture\n"); server.stdin.flush()
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
                print("objective-c named backend: selected path, bearer, result and secrecy PASS")
            print("objc portable: five typed rows, one request with the fixture questions")
        finally:
            server.stdin.close()
            server.wait(timeout=10)
