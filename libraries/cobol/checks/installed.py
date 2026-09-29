"""Compile two copied COBOL packages without source-tree paths at runtime."""
import collections
import ctypes
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
import tempfile
from backend import Backend

PACKAGE = Path(__file__).resolve().parents[1]
COBC = os.environ["TT_COBC"]
CC = os.environ["TT_CC"]
NATIVE = Path(os.environ["TT_NATIVE"])
HEADER = Path(os.environ["TT_HEADER"])

for name in ("alpha", "bravo"):
    with tempfile.TemporaryDirectory(prefix=f"thinkthen-cobol-{name}-") as temp:
        work = Path(temp)
        installed = work / "installed COBOL package with spaces"
        installed.mkdir()
        for folder in ("copybooks", "src", "examples"):
            shutil.copytree(PACKAGE / folder, installed / folder)
        native = work / "native"
        native.mkdir()
        shutil.copy2(NATIVE, native / "libthinkthen.so.0")
        (native / "libthinkthen.so").symlink_to("libthinkthen.so.0")
        shutil.copy2(HEADER, native / "thinkthen.h")
        flags = f'-include "{native / "thinkthen.h"}" -Wno-incompatible-pointer-types -Wno-implicit-function-declaration'
        common = [COBC, "-x", "-free", "-fstatic-call", "-fno-gen-c-decl-static-call", "-A", flags,
                  "-I", str(installed / "copybooks")]
        def compile(name, source, extra=()):
            subprocess.run([*common, "-o", str(work / name), str(source), *map(str, extra),
                            "-L", str(native), "-lthinkthen"], cwd=work, env=child_env(), check=True,
                           capture_output=True, timeout=60)
        subprocess.run([CC, "-std=c11", "-D_GNU_SOURCE", "-Wno-misleading-indentation",
                        "-c", str(installed / "src/TTJSON.c"), "-o", str(work / "ttjson.o")],
                       cwd=work, env=child_env(), check=True, capture_output=True, timeout=60)
        subprocess.run([CC, "-std=c11", "-D_GNU_SOURCE", "-c", str(installed / "src/tt_shape.c"),
                        "-o", str(work / "ttshape.o")], cwd=work, env=child_env(), check=True,
                       capture_output=True, timeout=60)
        if name == "alpha":
            decoder = work / "facts.so"
            subprocess.run([CC, "-std=c11", "-D_GNU_SOURCE", "-shared", "-fPIC",
                            str(installed / "src/TTJSON.c"), str(installed / "src/tt_shape.c"),
                            "-lm", "-o", str(decoder)], cwd=work, env=child_env(), check=True,
                           capture_output=True, timeout=60)
            check = ctypes.CDLL(str(decoder)).tt_cobol_facts
            check.argtypes = [ctypes.c_char_p, ctypes.c_size_t]
            check.restype = ctypes.c_int
            valid = b'{"records":1,"requests_sent":1,"cache_answers":0,"seconds":0.125,"model":"jev-1.13.0"}'
            invalid = [
                b'{"requests_sent":1,"cache_answers":0,"seconds":0.125}',
                b'{"records":1,"requests_sent":1,"cache_answers":0,"seconds":0.125,"input_tokens":null}',
                b'{"records":18446744073709551616,"requests_sent":1,"cache_answers":0,"seconds":0.125}',
                b'{"records":1.5,"requests_sent":1,"cache_answers":0,"seconds":0.125}',
                b'{"records":1,"requests_sent":1,"cache_answers":0,"seconds":-0.125}',
                b'{"records":1,"requests_sent":1,"cache_answers":0,"seconds":0.125,"model":null}',
            ]
            assert check(valid, len(valid)) == 0
            assert all(check(row, len(row)) != 0 for row in invalid), "strict copied facts decoder"
        compile("direct", installed / "examples/direct.cob")
        compile("settings", installed / "examples/settings.cob",
                [installed / "src/tt_engine.cob", installed / "src/tt_error.cob"])
        compile("types", installed / "examples/types.cob",
                [installed / "src/tt_validate.cob", work / "ttjson.o", work / "ttshape.o", "-lm"])
        compile("typed", PACKAGE / "checks/failure.cob",
                [installed / "src/tt_decide.cob", installed / "src/tt_error.cob",
                 work / "ttjson.o", work / "ttshape.o", "-lm"])
        barrier = work / "barrier"
        barrier.mkdir()
        server = Backend(barrier)
        try:
            env = child_env(HOME=str(work), XDG_CACHE_HOME=str(work / "xdg-cache"))
            env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{server.server_port}/generic/v1",
                       THINKTHEN_API_KEY="tt-fixture-key", THINKTHEN_CACHE=str(work / "cache"),
                       LD_LIBRARY_PATH=str(native))
            for entry, marker in (("direct", "outcome="), ("settings", "COBOL_SETTINGS_PASS"),
                                  ("types", "COBOL_TYPES_PASS"), ("typed", "COBOL_FAILURE_OWNERSHIP_PASS")):
                result = subprocess.run([str(work / entry)], cwd=work, env=env,
                                        capture_output=True, text=True, timeout=40)
                assert result.returncode == 0 and marker in result.stdout, (entry, result.stdout, result.stderr)
                if entry == "typed":
                    row = next(line[14:] for line in result.stdout.splitlines() if line.startswith("SUCCESS_FACTS "))
                    facts = json.loads(row)
                    assert (facts["records"], facts["requests_sent"], facts["cache_answers"]) == (1, 1, 0), facts
            assert collections.Counter(server.arrivals) == collections.Counter(
                ["café", "settings", "failure-two", "failure-two", "first"]), server.arrivals
            print(f"COBOL installed {name}: 5 exact arrivals, including copied typed facade")
        finally:
            server.close()
