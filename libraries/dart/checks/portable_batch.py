"""One public Dart bulk call against the shared literal Max corpus."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
from urllib.parse import unquote, urlparse

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
CORPUS = ROOT / "specification/fixtures/batching/portable-records.json"
EXPECTED = [(ROOT / f"specification/fixtures/batching/portable-{n}.request.json")
            .read_bytes().removesuffix(b"\n") for n in (1, 2, 3)]
RELEASE_PACKAGE = os.environ.get("THINKTHEN_RELEASE_DART_DIR")
RELEASE_NATIVE = os.environ.get("THINKTHEN_RELEASE_C_DIR")
assert bool(RELEASE_PACKAGE) == bool(RELEASE_NATIVE), "both release inputs are required"
if RELEASE_PACKAGE:
    package = Path(RELEASE_PACKAGE).resolve()
    native = Path(RELEASE_NATIVE).resolve() / "lib/libthinkthen.so"
    expected = {"CHANGELOG.md", "LICENSE", "README.md", "pubspec.yaml", "pubspec.lock",
                "lib/thinkthen_dart.dart", "lib/src/allocator.dart", "lib/src/door.dart",
                "lib/src/typed.dart", "THINKTHEN-PACKAGE-INPUTS"}
    members = {str(path.relative_to(package)) for path in package.rglob("*") if path.is_file()}
    assert members == expected and not any(path.is_symlink() for path in package.rglob("*")), ("DART_ARCHIVE_MEMBERS", members)
    manifest = (package / "pubspec.yaml").read_text()
    assert "name: thinkthen_dart\n" in manifest and "ffi: ^2.1.4\n" in manifest, "DART_PUBSPEC_IDENTITY"
else:
    native = Path(os.environ["TT_NATIVE_LIBRARY"])
DART = os.environ["TT_DART"]
BACKEND = Path(os.environ.get("THINKTHEN_BACKEND_BIN", ROOT / "target/debug/conformance-backend"))
assert native.is_file() and BACKEND.is_file(), "select a built native library and backend"
(HERE / "scratch").mkdir(parents=True, exist_ok=True)

backend = subprocess.Popen([str(BACKEND)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           text=True, env={"PATH": os.environ.get("PATH", "/usr/bin:/bin")})
try:
    port = int(backend.stdout.readline())
    with tempfile.TemporaryDirectory(prefix="dart-portable-", dir=HERE / "scratch") as scratch:
        env = os.environ.copy()
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
            resolved = subprocess.run([DART, "pub", "get", "--offline", "--directory", str(consumer)],
                                      env=env, capture_output=True, text=True, timeout=60)
            assert resolved.returncode == 0, (resolved.stdout, resolved.stderr)
            config = consumer / ".dart_tool/package_config.json"
            entries = json.loads(config.read_text())["packages"]
            entry = next(item for item in entries if item["name"] == "thinkthen_dart")
            uri = urlparse(entry["rootUri"])
            assert uri.scheme in ("", "file"), entry
            root = Path(unquote(uri.path)) if uri.scheme == "file" else config.parent / unquote(uri.path)
            assert root.resolve() == package, (root, package)
        else:
            consumer = HERE / "consumers/alpha"
        done = subprocess.run([DART, "run", "bin/portable_batch.dart", str(native), str(CORPUS)],
                              cwd=consumer, env=env, capture_output=True,
                              text=True, timeout=60)
        assert done.returncode == 0 and "DART_PORTABLE_BATCH_PASS" in done.stdout, done.stderr
    backend.stdin.write("count\n")
    backend.stdin.flush()
    assert int(backend.stdout.readline()) == 3
    backend.stdin.write("capture\n")
    backend.stdin.flush()
    actual = [body.encode() for body in json.loads(backend.stdout.readline())["bodies"]]
    assert sorted(actual) == sorted(EXPECTED), "literal body membership changed"
    print("Dart public bulk: five values, three literal bodies and sends")
finally:
    backend.stdin.close()
    assert backend.wait(timeout=10) == 0
