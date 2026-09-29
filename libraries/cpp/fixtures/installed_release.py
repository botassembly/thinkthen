"""Install unpacked CMake source and run focused find_package consumers."""
import json
import os
from pathlib import Path
import subprocess
import sys

from fixture import Backend

wrapper, native = map(Path, sys.argv[1:])
home = wrapper / "installed-consumer"
home.mkdir()
prefix = home / "prefix"
build = home / "package-build"
cmake = os.environ.get("THINKTHEN_CMAKE_BIN", "cmake")
header = native / "include/thinkthen.h"
shared = native / "lib/libthinkthen.so"
static = native / "lib/libthinkthen.a"
abi_env = os.environ | {"THINKTHEN_NATIVE_HEADER": str(header), "THINKTHEN_NATIVE_SHARED": str(shared)}
subprocess.run([sys.executable, str(Path(__file__).with_name("exports.py"))], env=abi_env, check=True)
subprocess.run([cmake, "-S", str(wrapper), "-B", str(build), "-DCMAKE_BUILD_TYPE=Release",
                f"-DCMAKE_INSTALL_PREFIX={prefix}", f"-DTHINKTHEN_C_HEADER={header}",
                f"-DTHINKTHEN_NATIVE_SHARED={shared}", f"-DTHINKTHEN_NATIVE_STATIC={static}"], check=True)
subprocess.run([cmake, "--build", str(build), "--parallel", "2"], check=True)
subprocess.run([cmake, "--install", str(build)], check=True)
for original, installed in ((header, prefix / "include/thinkthen/thinkthen.h"),
                            (shared, prefix / "lib/libthinkthen.so.0"),
                            (static, prefix / "lib/libthinkthen.a")):
    assert original.read_bytes() == installed.read_bytes(), installed
barrier = home / "barrier"
barrier.mkdir()
server = Backend(barrier)
try:
    env = {
        "PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": str(home),
        "XDG_CONFIG_HOME": str(home), "XDG_CACHE_HOME": str(home / "cache"),
        "THINKTHEN_BASE_URL": f"http://127.0.0.1:{server.server_port}/generic/v1",
        "THINKTHEN_API_KEY": "tt-canary-301", "THINKTHEN_CACHE": str(home / "cache"),
        "TT_BARRIER_DIR": str(barrier),
    }
    fixture = Path(__file__).with_name("installed_release")
    for mode in ("shared", "static"):
        consumer_build = home / (mode + "-build")
        subprocess.run([cmake, "-S", str(fixture), "-B", str(consumer_build),
                        f"-DCMAKE_PREFIX_PATH={prefix}", "-DCMAKE_BUILD_TYPE=Release",
                        f"-DTHINKTHEN_LINK_STATIC_C={'ON' if mode == 'static' else 'OFF'}"], check=True)
        subprocess.run([cmake, "--build", str(consumer_build), "--parallel", "2"], check=True)
        binary = consumer_build / "consumer"
        linked = subprocess.check_output(["ldd", str(binary)], text=True)
        if mode == "shared":
            assert "libthinkthen.so.0" in linked and str(prefix / "lib") in linked, linked
        else:
            assert "libthinkthen.so.0" not in linked, linked
        result = subprocess.run([str(binary)], cwd=home, env=env, text=True, capture_output=True, timeout=30)
        assert (result.returncode, result.stdout.strip(), result.stderr) == (0, "outcome=1 probability=0.9", ""), result
        expected = json.loads((Path(__file__).with_name("accepted_requests.jsonl")).read_text().splitlines()[0])
        assert server.requests == [expected] * (1 if mode == "shared" else 2), server.requests
        print(f"CPP_INSTALLED_{mode.upper()}_PASS executable={binary} requests={len(server.requests)} body={json.dumps(expected, sort_keys=True)}")
finally:
    server.close()
