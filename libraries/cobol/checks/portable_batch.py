"""Shared five-text Max case through the public bounded COBOL TT-CALL door."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
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
    if INSTALLED:
        linked = subprocess.check_output(["ldd", program], env=child_env(LD_LIBRARY_PATH=str(LIB_DIR)), text=True)
        assert str(LIB_DIR / "libthinkthen.so.0") in linked, linked
        print("cobol installed loader:", next(line.strip() for line in linked.splitlines() if "libthinkthen.so.0" in line))
    server = subprocess.Popen([backend], env=child_env(), stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
    try:
        port = int(server.stdout.readline())
        base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
        settings = json.dumps({"base_url": base, "model": corpus["model"], "batch": "max",
                               "cache": False, "max_retries": 0, "throttle": 1})
        env = child_env()
        env.update(THINKTHEN_API_KEY="sk-loopback-cobol-portable", THINKTHEN_BASE_URL=base,
                   HOME=scratch, THINKTHEN_CACHE=str(target / "cache"),
                   LD_LIBRARY_PATH=str(LIB_DIR))
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
        # Each row names its own question's key (ADR 0111).
        keys = question_keys(base + "/systemone", captured["bodies"][0])
        assert [row["meta"]["requests"] for row in rows] == [[key] for key in keys], rows
        print("cobol portable: five JSON rows and keys, one request with the fixture questions")
    finally:
        server.stdin.close()
        server.wait(timeout=10)
