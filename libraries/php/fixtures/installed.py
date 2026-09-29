"""Run two copied PHP installations with matching native header and library."""

import collections
import hashlib
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

from backend import Backend

ROOT = Path(__file__).resolve().parents[3]
PHP = ROOT / "libraries/php"
NATIVE = ROOT / "libraries/c/target/debug/libthinkthen_c.so"
HEADER = ROOT / "libraries/c/include/thinkthen.h"
PACKAGE_FILES = ["LICENSE", "README.md", "autoload.php", "composer.json", "examples/direct.php", "src/ThinkThen.php"]
PRIVATE_PATTERNS = (b"tt-canary-291", b"/home/ian", b"auth.json", b"-----BEGIN PRIVATE KEY-----")


def identical(source, copied):
    return hashlib.sha256(source.read_bytes()).digest() == hashlib.sha256(copied.read_bytes()).digest()


def verify_install(package, native):
    assert sorted(str(p.relative_to(package)) for p in package.rglob("*") if p.is_file()) == PACKAGE_FILES, "package members"
    for name in PACKAGE_FILES:
        assert not any(pattern in (package / name).read_bytes() for pattern in PRIVATE_PATTERNS), f"private pattern in {name}"
        assert identical(PHP / name, package / name), f"source member changed: {name}"
    assert identical(HEADER, native / "include/thinkthen.h"), "installed header changed"
    assert identical(NATIVE, native / "lib/libthinkthen.so"), "installed library changed"
    assert (native / "lib/libthinkthen.so.0").readlink().as_posix() == "libthinkthen.so", "installed SONAME link"


def main():
    plant = sys.argv[1] if len(sys.argv) > 1 else ""
    assert plant in ("", "source", "header", "native", "canary", "private-key", "wrong-value")
    for mode in ("alpha", "beta"):
        with tempfile.TemporaryDirectory(prefix="thinkthen-php-installed-") as folder:
            work = Path(folder)
            install = work / "install with spaces"
            package = install / "package"
            native = install / "native"
            shutil.copytree(PHP, package, ignore=shutil.ignore_patterns("fixtures", "check.sh", "ratchet*.json", "__pycache__"))
            (native / "lib").mkdir(parents=True)
            (native / "include").mkdir()
            shutil.copy2(NATIVE, native / "lib/libthinkthen.so")
            (native / "lib/libthinkthen.so.0").symlink_to("libthinkthen.so")
            shutil.copy2(HEADER, native / "include/thinkthen.h")
            shutil.copy2(PHP / "fixtures/installed_consumer.php", work / "consumer.php")
            if plant == "source":
                (package / "autoload.php").write_bytes((package / "autoload.php").read_bytes() + b"\n// stale source\n")
            if plant == "header":
                (native / "include/thinkthen.h").write_bytes((native / "include/thinkthen.h").read_bytes() + b"\n/* stale */\n")
            if plant == "native":
                with (native / "lib/libthinkthen.so").open("ab") as stream: stream.write(b"stale")
            if plant == "canary":
                (package / "README.md").write_bytes((package / "README.md").read_bytes() + b"\ntt-canary-291\n")
            if plant == "private-key":
                (package / "README.md").write_bytes((package / "README.md").read_bytes() + b"\n-----BEGIN PRIVATE KEY-----\n")
            if plant in ("source", "header", "native", "canary", "private-key"):
                try: verify_install(package, native)
                except AssertionError as error:
                    print(f"PHP_INSTALLED_PLANT_REJECTED {plant}: {error}")
                    return
                raise AssertionError(f"plant passed: {plant}")
            verify_install(package, native)
            barrier = work / "barrier"
            barrier.mkdir()
            (work / "home").mkdir()
            server = Backend(barrier)
            try:
                base = f"http://127.0.0.1:{server.server_port}/generic/v1"
                env = {"PATH": "/usr/bin:/bin", "HOME": "/work/home", "THINKTHEN_CACHE": "/work/home/cache",
                       "THINKTHEN_API_KEY": "tt-canary-291", "THINKTHEN_BASE_URL": base if mode == "alpha" else "http://127.0.0.1:1/generic/v1",
                       "TT_USE_SETTINGS": "1" if mode == "beta" else "0", "TT_SETTINGS_BASE_URL": base}
                if plant == "wrong-value": env["TT_PLANT_WRONG_VALUE"] = "1"
                php_bin = os.environ.get("THINKTHEN_PHP_BIN", "/usr/bin/php8.3")
                command = [os.environ.get("THINKTHEN_BWRAP_BIN", "/usr/bin/bwrap"), "--unshare-all", "--share-net", "--die-with-parent",
                           "--ro-bind", "/usr", "/usr", "--ro-bind", "/lib", "/lib", "--ro-bind", "/lib64", "/lib64",
                           "--proc", "/proc", "--dev", "/dev", "--tmpfs", "/tmp", "--bind", str(work), "/work"]
                if php_bin != "/usr/bin/php8.3":
                    command += ["--ro-bind", php_bin, "/usr/bin/php8.3"]
                command += ["--chdir", "/work", "--", "/usr/bin/php8.3", "-n", "-d", "extension=ffi", "-d", "ffi.enable=1", "/work/consumer.php"]
                result = subprocess.run(command, env=env, capture_output=True, text=True, timeout=60)
                if result.returncode != 0 and result.stderr.startswith("bwrap:") and not server.arrivals:
                    raise SystemExit(77)
                if plant == "wrong-value":
                    assert result.returncode != 0 and "value/facts mismatch" in result.stdout, (result.returncode, result.stdout, result.stderr)
                    assert collections.Counter(server.arrivals) == {"consumer-php": 1, "consumer-json": 1}
                    print("PHP_INSTALLED_PLANT_REJECTED wrong-value: public envelope assertion")
                    return
                assert result.returncode == 0 and "INSTALLED_PHP_CONSUMER_PASS" in result.stdout and not result.stderr, (mode, result.returncode, result.stdout, result.stderr)
                assert collections.Counter(server.arrivals) == {"consumer-php": 1, "consumer-json": 1}, (mode, server.arrivals)
                assert server.attempts == 2, (mode, server.attempts)
                print(f"PHP_INSTALLED_PASS {mode} arrivals=2")
            finally:
                server.close()


if __name__ == "__main__":
    main()
