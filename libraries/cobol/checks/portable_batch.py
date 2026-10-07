"""Shared five-text Max case through the public bounded COBOL TT-CALL door."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
from backend_cases import ROWS, alias, configuration, paths
from portable import one_portable_request, question_keys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
PACKAGE = Path(os.environ.get("THINKTHEN_PORTABLE_PACKAGE", ROOT / "libraries/cobol")).resolve()
INSTALLED = "THINKTHEN_PORTABLE_PACKAGE" in os.environ
assert (PACKAGE / "src/tt_call.cob").is_file(), "COBOL package source missing"
NATIVE = Path(os.environ["THINKTHEN_PORTABLE_NATIVE"]).resolve()
LIB_DIR = NATIVE / "lib" if (NATIVE / "lib").is_dir() else NATIVE
LIB_NAME = "thinkthen" if LIB_DIR != NATIVE else "thinkthen_c"
FIXTURE = ROOT / "specification/fixtures/batching"
corpus = json.loads((FIXTURE / "portable-records.json").read_text())
backend = Path(os.environ.get("THINKTHEN_BACKEND_BIN", ROOT / "target/debug/conformance-backend"))
assert corpus["schema"] == "thinkthen.portable-batch-records/1" and len(corpus["texts"]) == 5
request = json.dumps({"decide": corpus["question"], "records": corpus["texts"],
                      "details": True, "call": {"batch": "max"}}, ensure_ascii=False, separators=(",", ":"))
assert len(request.encode()) < 8192

with tempfile.TemporaryDirectory(prefix="thinkthen-cobol-portable-") as scratch:
    target = Path(scratch)
    shutil.copytree(PACKAGE / "src", target / "src")
    shutil.copytree(PACKAGE / "copybooks", target / "copybooks")
    program = target / "portable_door"
    header = NATIVE / "include/thinkthen.h" if LIB_DIR != NATIVE else ROOT / "libraries/c/include/thinkthen.h"
    if INSTALLED:
        assert header == NATIVE / "include/thinkthen.h" and header.is_file(), "installed C header missing"
    subprocess.run(["cobc", "-x", "-free", "-fstatic-call", "-fno-gen-c-decl-static-call",
                    "-I", str(target / "copybooks"),
                    "-A", f"-include {header} -Wno-incompatible-pointer-types -Wno-implicit-function-declaration",
                    "-o", str(program), str(HERE / "door.cob"),
                    str(target / "src/tt_engine.cob"), str(target / "src/tt_call.cob"),
                    str(target / "src/tt_error.cob"), "-L", str(LIB_DIR),
                    "-l" + LIB_NAME], env=child_env(), check=True, capture_output=True, text=True, timeout=90)
    native_program = target / "native_consumer"
    subprocess.run(["cobc", "-x", "-free", "-fstatic-call", "-fno-gen-c-decl-static-call",
                    "-I", str(target / "copybooks"), "-I", str(header.parent),
                    "-A", '-include "' + str(target / "src/tt_native.h") + '" -Wno-incompatible-pointer-types',
                    "-o", str(native_program), str(PACKAGE / "examples/native.cob"),
                    str(target / "src/tt_inputs.c"), str(target / "src/tt_complete.c"),
                    "-L", str(LIB_DIR), "-l" + LIB_NAME], env=child_env(), check=True,
                   capture_output=True, text=True, timeout=90)
    if INSTALLED:
        linked = subprocess.check_output(["ldd", program], env=child_env(LD_LIBRARY_PATH=str(LIB_DIR)), text=True)
        assert str(LIB_DIR / "libthinkthen.so.0") in linked, linked
        print("cobol installed loader:", next(line.strip() for line in linked.splitlines() if "libthinkthen.so.0" in line))
    for named in (False, True):
        server = subprocess.Popen([backend], env=child_env(THINKTHEN_TEST_MARKERS=json.dumps({"local":"tt-named-loopback"})), stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
        try:
            port = int(server.stdout.readline())
            base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
            settings = json.dumps({"base_url": base, "model": corpus["model"], "batch": "max",
                                   "cache": False, "max_retries": 0, "throttle": 1})
            env = child_env()
            env.update(THINKTHEN_API_KEY="sk-loopback-cobol-portable", THINKTHEN_BASE_URL=base,
                       HOME=scratch, THINKTHEN_CACHE=str(target / "cache"),
                       LD_LIBRARY_PATH=str(LIB_DIR))
            if named:
                backend_row = next(row for row in ROWS if row["name"] == "typesafe")
                env["XDG_CONFIG_HOME"] = env["HOME"]
                base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
                configuration(env, {"local": alias(backend_row, base)})
                env[backend_row["key"]] = "tt-named-loopback"
                env["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{port}/generic/v1"
                settings = json.dumps({"backend":"local", "batch":"max", "cache":False, "max_retries":0, "throttle":1})
            run = subprocess.run([program, request, settings], env=env, capture_output=True,
                                 text=True, timeout=60)
            assert run.returncode == 0, (run.stdout, run.stderr)
            result = json.loads(run.stdout)
            assert set(result) == {"value", "facts"} and result["facts"]["requests_sent"] == 1, result
            rows = result["value"]
            assert len(rows) == 5
            for at, row in enumerate(rows):
                assert (row["input"] == corpus["texts"][at] and row["value"] is True
                        and row["answer"]["probability"] == 0.9), row
                assert "batch" not in row["meta"], row
            server.stdin.write("count\n"); server.stdin.flush()
            count = int(server.stdout.readline())
            server.stdin.write("capture\n"); server.stdin.flush()
            captured = json.loads(server.stdout.readline())
            assert count == 1, (count, captured)
            one_portable_request(captured["bodies"])
            if named:
                for command, expected in (("paths", paths(backend_row["path"])),
                                          ("bearers", {"markers":{"local":1},"absent":0,"unknown":0,"overflow":False})):
                    server.stdin.write(command + "\n"); server.stdin.flush()
                    assert json.loads(server.stdout.readline()) == expected, command
                assert "tt-named-loopback" not in run.stdout + run.stderr
                assert "tt-named-loopback" not in json.dumps(captured["bodies"])
                print("cobol named backend: selected path, bearer, result and secrecy PASS")
            # Each row names its own question's key (ADR 0111).
            keys = question_keys(base + "/systemone", captured["bodies"][0])
            assert [row["meta"]["requests"] for row in rows] == [[key] for key in keys], rows
            print("cobol portable: five JSON rows and keys, one request with the fixture questions")
            env["TT_NATIVE_SETTINGS"] = json.dumps({"base_url": base, "model": "literal", "cache": False, "max_retries": 0})
            owned = subprocess.run([native_program], env=env, capture_output=True,
                                   text=True, timeout=60)
            assert owned.returncode == 0 and owned.stdout.strip() == "COBOL_NATIVE_PASS", (owned.returncode, owned.stdout, owned.stderr)
            server.stdin.write("count\n"); server.stdin.flush()
            assert int(server.stdout.readline()) == 2
            server.stdin.write("capture\n"); server.stdin.flush()
            snapshot = json.loads(server.stdout.readline())
            body = json.loads(snapshot["bodies"][-1])
            assert "x" * 9000 in json.dumps(body) and "z" * 9000 not in json.dumps(body), body
            print("cobol native: 9,000 counted bytes cloned before mutation, typed result after owners freed")
        finally:
            server.stdin.close()
            server.wait(timeout=10)
