"""Count exact shared Max bodies from the public PHP bulk call."""

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
NATIVE = Path(os.environ.get("THINKTHEN_PORTABLE_NATIVE", ROOT / "libraries/c/target/debug"))
RELEASE_PACKAGE = os.environ.get("THINKTHEN_RELEASE_PHP_DIR")
RELEASE_NATIVE = os.environ.get("THINKTHEN_RELEASE_C_DIR")
assert bool(RELEASE_PACKAGE) == bool(RELEASE_NATIVE), "both release inputs are required"
if RELEASE_PACKAGE:
    package = Path(RELEASE_PACKAGE).resolve()
    native = Path(RELEASE_NATIVE).resolve()
    expected = {"LICENSE", "README.md", "composer.json", "autoload.php",
                "src/ThinkThen.php", "examples/direct.php", "THINKTHEN-PACKAGE-INPUTS",
                'src/complete/models.php',
                'src/native/abi.h',
                'src/native/views.php',
                'src/native/views_0.php',
                'src/native/views_1.php',
                'src/native/input.php',
                'src/native/question.php',
                'src/native/question_adapter.php',
                'src/native/result.php',
                'src/native/batch.php',
                'src/native/engine.php'}
    members = {str(path.relative_to(package)) for path in package.rglob("*") if path.is_file()}
    assert members == expected and not any(path.is_symlink() for path in package.rglob("*")), ("PHP_ARCHIVE_MEMBERS", members)
    metadata = json.loads((package / "composer.json").read_text())
    assert metadata["name"] == "botassembly/thinkthen", "PHP_COMPOSER_IDENTITY"
    assert metadata["require"] == {"php": ">=8.3", "ext-ffi": "*"}, "PHP_COMPOSER_REQUIREMENTS"
    assert metadata["autoload"] == {"files": ["autoload.php"]}, "PHP_COMPOSER_AUTOLOAD"
    autoload = package / "autoload.php"
    library = native / "lib/libthinkthen.so"
    assert library.is_file() and (native / "include/thinkthen.h").is_file()
else:
    autoload = ROOT / "libraries/php/autoload.php"
    library = Path(os.environ.get("THINKTHEN_PORTABLE_LIBRARY", str(NATIVE / "lib/libthinkthen.so")))
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
        with tempfile.TemporaryDirectory(prefix="thinkthen-php-portable-") as scratch:
            base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
            env = child_env()
            env.update(THINKTHEN_API_KEY="sk-loopback-php-portable", THINKTHEN_BASE_URL=base,
                       TT_PORTABLE_SETTINGS=json.dumps({"base_url": base, "model": corpus["model"],
                                                        "batch": "max", "cache": False, "max_retries": 0,
                                                        "throttle": 1}),
                       TT_PORTABLE_CORPUS=str(CORPUS), TT_AUTOLOAD=str(autoload),
                       TT_LIBRARY=str(library),
                       HOME=scratch, THINKTHEN_CACHE=scratch,
                       LD_LIBRARY_PATH=str(library.parent) if RELEASE_PACKAGE else str(NATIVE / "lib"))
            if named:
                row = next(row for row in ROWS if row["name"] == "typesafe")
                env["XDG_CONFIG_HOME"] = env["HOME"]
                base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
                configuration(env, {"local": alias(row, base)})
                env[row["key"]] = "tt-named-loopback"
                env["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{port}/generic/v1"
                env["TT_PORTABLE_SETTINGS"] = json.dumps({"backend":"local", "batch":"max", "cache":False, "max_retries":0, "throttle":1})
            run = subprocess.run([os.environ.get("THINKTHEN_PHP_BIN", "/usr/bin/php8.3"),
                                  "-d", "ffi.enable=1", "fixtures/portable_consumer.php"],
                                 cwd=ROOT / "libraries/php", env=env, capture_output=True, text=True, timeout=60)
            assert run.returncode == 0 and "PHP_PORTABLE_BATCH_PASS" in run.stdout, (run.stdout, run.stderr)
        server.stdin.write("count\n")
        server.stdin.flush()
        count = int(server.stdout.readline())
        server.stdin.write("capture\n")
        server.stdin.flush()
        captured = json.loads(server.stdout.readline())
        assert count == 2, (count, captured)
        for body in captured["bodies"]:one_portable_request([body])
        assert "sk-loopback-php-portable" not in run.stdout + run.stderr
        if named:
            for command, expected in (("paths", paths(row["path"], 2)),
                                      ("bearers", {"markers":{"local":2},"absent":0,"unknown":0,"overflow":False})):
                server.stdin.write(command + "\n"); server.stdin.flush()
                assert json.loads(server.stdout.readline()) == expected, command
            assert "tt-named-loopback" not in run.stdout + run.stderr
            assert "tt-named-loopback" not in json.dumps(captured["bodies"])
            print("php named backend: selected path, bearer, result and secrecy PASS")
        print("php portable: legacy and complete typed rows, two requests with the fixture questions")
    finally:
        server.stdin.close()
        server.wait(timeout=10)
