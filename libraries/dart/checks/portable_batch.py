"""One public Dart bulk call against the shared literal Max corpus."""
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
from urllib.parse import unquote, urlparse

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
from backend_cases import ROWS, alias, configuration, paths
from native_assets import configure, package_members
from portable import one_portable_request  # noqa: E402

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
CORPUS = ROOT / "specification/fixtures/batching/portable-records.json"
RELEASE_PACKAGE = os.environ.get("THINKTHEN_RELEASE_DART_DIR")
RELEASE_NATIVE = os.environ.get("THINKTHEN_RELEASE_C_DIR")
FFI_SHA256 = "6d7fd89431262d8f3125e81b50d3847a091d846eafcd4fdb88dd06f36d705a45"


def resolved_root(entry, config):
    uri = urlparse(entry["rootUri"])
    assert uri.scheme in ("", "file"), entry
    root = Path(unquote(uri.path)) if uri.scheme == "file" else config.parent / unquote(uri.path)
    return root.resolve()


def check_ffi_lock(lock):
    text = lock.read_text()
    blocks = re.findall(r"(?ms)^  ffi:\n(.*?)(?=^  [^ ]|^sdks:|\Z)", text)
    assert len(blocks) == 1, "DART_FFI_LOCK"
    block = blocks[0]
    for line in ('    source: hosted\n', '      name: ffi\n',
                 '      url: "https://pub.dev"\n', f'      sha256: "{FFI_SHA256}"\n',
                 '    version: "2.2.0"\n'):
        assert block.count(line) == 1, "DART_FFI_LOCK"


assert bool(RELEASE_PACKAGE) == bool(RELEASE_NATIVE), "both release inputs are required"
if RELEASE_PACKAGE:
    package = Path(RELEASE_PACKAGE).resolve()
    native = Path(RELEASE_NATIVE).resolve() / "lib/libthinkthen.so"
    expected = package_members(ROOT / 'libraries/dart') | {'pubspec.lock', 'THINKTHEN-PACKAGE-INPUTS'}
    members = {str(path.relative_to(package)) for path in package.rglob("*") if path.is_file()}
    assert members == expected and not any(path.is_symlink() for path in package.rglob("*")), ("DART_ARCHIVE_MEMBERS", members)
    manifest = (package / "pubspec.yaml").read_text()
    assert "name: thinkthen_dart\n" in manifest and "ffi: ^2.1.4\n" in manifest, "DART_PUBSPEC_IDENTITY"
    check_ffi_lock(package / "pubspec.lock")
else:
    native = Path(os.environ["TT_NATIVE_LIBRARY"])
DART = os.environ["TT_DART"]
BACKEND = Path(os.environ.get("THINKTHEN_BACKEND_BIN", ROOT / "target/debug/conformance-backend"))
assert native.is_file() and BACKEND.is_file(), "select a built native library and backend"
(HERE / "scratch").mkdir(parents=True, exist_ok=True)

for named in (False, True):
    backend = subprocess.Popen([str(BACKEND)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                               text=True, env=child_env(THINKTHEN_TEST_MARKERS=json.dumps({"local":"tt-named-loopback"})))
    try:
        port = int(backend.stdout.readline())
        with tempfile.TemporaryDirectory(prefix="dart-portable-", dir=HERE / "scratch") as scratch:
            env = child_env(keep=("PUB_CACHE",))
            env.pop("THINKTHEN_API_KEY", None)
            env.update(HOME=scratch, XDG_CACHE_HOME=scratch, XDG_CONFIG_HOME=scratch,
                       THINKTHEN_CACHE=str(Path(scratch) / "cache"),
                       THINKTHEN_API_KEY="tt-portable-loopback",
                       THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/arm/full/capture/v1")
            if RELEASE_PACKAGE:
                consumer = Path(scratch) / "independent-consumer"
                (consumer / "bin").mkdir(parents=True)
                shutil.copyfile(HERE / "consumers/alpha/bin/portable_batch.dart", consumer / "bin/portable_batch.dart")
                (consumer / "pubspec.yaml").write_text(
                    "name: thinkthen_release_consumer\nversion: 0.0.1\npublish_to: none\n"
                    "environment:\n  sdk: '>=3.3.0 <4.0.0'\ndependencies:\n"
                    f"  thinkthen_dart:\n    path: {json.dumps(str(package))}\n")
                configure(package, native, [consumer])
                resolved = subprocess.run([DART, "pub", "get", "--offline", "--directory", str(consumer)],
                                          env=env, capture_output=True, text=True, timeout=60)
                assert resolved.returncode == 0, (resolved.stdout, resolved.stderr)
                config = consumer / ".dart_tool/package_config.json"
                entries = json.loads(config.read_text())["packages"]
                assert sum(item["name"] == "thinkthen_dart" for item in entries) == 1, "DART_PACKAGE_CONFIG"
                entry = next(item for item in entries if item["name"] == "thinkthen_dart")
                assert resolved_root(entry, config) == package, (entry, package)
                assert sum(item["name"] == "ffi" for item in entries) == 1, "DART_FFI_CONFIG"
                ffi = next(item for item in entries if item["name"] == "ffi")
                expected_ffi = Path(env["PUB_CACHE"]) / "hosted/pub.dev/ffi-2.2.0"
                assert resolved_root(ffi, config) == expected_ffi.resolve(), "DART_FFI_CONFIG"
                check_ffi_lock(consumer / "pubspec.lock")
            else:
                consumer = HERE / "consumers/alpha"
            if named:
                row = next(row for row in ROWS if row["name"] == "typesafe")
                env["XDG_CONFIG_HOME"] = env["HOME"]
                base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
                configuration(env, {"local": alias(row, base)})
                env[row["key"]] = "tt-named-loopback"
                env["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{port}/generic/v1"
                env["TT_PORTABLE_SETTINGS"] = json.dumps({"backend":"local", "batch":"max", "cache":False, "max_retries":0, "throttle":1})
            done = subprocess.run([DART, "run", "bin/portable_batch.dart", str(native), str(CORPUS)],
                                  cwd=consumer, env=env, capture_output=True,
                                  text=True, timeout=60)
            assert done.returncode == 0 and "DART_PORTABLE_BATCH_PASS" in done.stdout, done.stderr
        backend.stdin.write("count\n")
        backend.stdin.flush()
        assert int(backend.stdout.readline()) == 2
        backend.stdin.write("capture\n")
        backend.stdin.flush()
        actual = [body.encode() for body in json.loads(backend.stdout.readline())["bodies"]]
        for body in actual:one_portable_request([body])
        assert "tt-portable-loopback" not in done.stdout + done.stderr
        if named:
            for command, expected in (("paths", paths(row["path"], 2)),
                                      ("bearers", {"markers":{"local":2},"absent":0,"unknown":0,"overflow":False})):
                backend.stdin.write(command + "\n"); backend.stdin.flush()
                assert json.loads(backend.stdout.readline()) == expected, command
            assert "tt-named-loopback" not in done.stdout + done.stderr
            assert b"tt-named-loopback" not in b"".join(actual)
            print("dart named backend: selected path, bearer, result and secrecy PASS")
        print("Dart public bulk: legacy and complete typed values, two requests with the fixture questions")
    finally:
        backend.stdin.close()
        assert backend.wait(timeout=10) == 0
