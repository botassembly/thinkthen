"""Shared five-text Max case through the public Ada bulk procedure."""
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
NATIVE = Path(os.environ["THINKTHEN_PORTABLE_NATIVE"]).resolve()
PACKAGE = Path(os.environ.get("THINKTHEN_PORTABLE_PACKAGE", ROOT / "libraries/ada")).resolve()
INSTALLED = "THINKTHEN_PORTABLE_PACKAGE" in os.environ
assert (PACKAGE / "src/thinkthen.ads").is_file(), "Ada package source missing"
LIB_DIR = NATIVE / "lib" if (NATIVE / "lib").is_dir() else NATIVE
LIB_NAME = "thinkthen" if LIB_DIR != NATIVE else "thinkthen_c"
FIXTURE = ROOT / "specification/fixtures/batching"
corpus = json.loads((FIXTURE / "portable-records.json").read_text())
backend = Path(os.environ.get("THINKTHEN_BACKEND_BIN", ROOT / "target/debug/conformance-backend"))
assert corpus["schema"] == "thinkthen.portable-batch-records/1" and len(corpus["texts"]) == 5

with tempfile.TemporaryDirectory(prefix="thinkthen-ada-portable-") as scratch:
    target = Path(scratch)
    shutil.copytree(PACKAGE / "src", target / "src")
    if INSTALLED:
        project = subprocess.run(["gprbuild", "-P", str(PACKAGE / "thinkthen.gpr"),
                                  "-j2"], cwd=PACKAGE,
                                 env=child_env(), capture_output=True, text=True, timeout=90)
        assert project.returncode == 0, (project.stdout, project.stderr)
    program = target / "portable_batch"
    build = subprocess.run(["gnatmake", "-gnat2022", "-I" + str(target / "src"),
                    str(HERE / "portable_batch.adb"), "-D", scratch, "-o", str(program),
                    "-largs", "-L" + str(LIB_DIR), "-l" + LIB_NAME],
                   env=child_env(), capture_output=True, text=True, timeout=90)
    assert build.returncode == 0, (build.stdout, build.stderr)
    native_program = target / "native_consumer"
    subprocess.run(["gnatmake", "-gnat2022", "-I" + str(target / "src"),
                    str(PACKAGE / "examples/native.adb"), "-D", scratch,
                    "-o", str(native_program), "-largs", "-L" + str(LIB_DIR),
                    "-l" + LIB_NAME], env=child_env(), check=True,
                   capture_output=True, text=True, timeout=90)
    if INSTALLED:
        linked = subprocess.check_output(["ldd", program], env=child_env(LD_LIBRARY_PATH=str(LIB_DIR)), text=True)
        assert str(LIB_DIR / "libthinkthen.so.0") in linked, linked
        print("ada installed loader:", next(line.strip() for line in linked.splitlines() if "libthinkthen.so.0" in line))
    for named in (False, True):
        server = subprocess.Popen([backend], env=child_env(THINKTHEN_TEST_MARKERS=json.dumps({"local":"tt-named-loopback"})), stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
        try:
            port = int(server.stdout.readline())
            base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
            env = child_env(home=scratch, XDG_CONFIG_HOME=scratch if named else str(Path(scratch) / "config"))
            env.update(THINKTHEN_API_KEY="sk-loopback-ada-portable", THINKTHEN_BASE_URL=base,
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
            assert run.returncode == 0 and "ADA_PORTABLE_BATCH_PASS" in run.stdout, (run.stdout, run.stderr)
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
                print("ada named backend: selected path, bearer, result and secrecy PASS")
            print("ada portable: five typed rows, one request with the fixture questions")
            env["TT_NATIVE_SETTINGS"] = json.dumps({"base_url": base, "model": "literal", "cache": False, "max_retries": 0})
            owned = subprocess.run([native_program], env=env, capture_output=True,
                                   text=True, timeout=60)
            assert owned.returncode == 0 and owned.stdout.strip() == "ADA_NATIVE_PASS", (owned.returncode, owned.stdout, owned.stderr)
            server.stdin.write("count\n"); server.stdin.flush()
            assert int(server.stdout.readline()) == 2
            server.stdin.write("capture\n"); server.stdin.flush()
            snapshot = json.loads(server.stdout.readline())
            body = json.loads(snapshot["bodies"][-1])
            assert "x" * 9000 in json.dumps(body) and "z" * 9000 not in json.dumps(body), body
            print("ada native: 9,000 counted bytes cloned before mutation, typed result after owners freed")
        finally:
            server.stdin.close()
            server.wait(timeout=10)
