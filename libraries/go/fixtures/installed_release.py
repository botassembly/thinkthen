"""One external Go module over unpacked source and the paired C archive."""
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

from backend import Backend

wrapper, native = map(Path, sys.argv[1:])
home = wrapper / "installed-consumer"
home.mkdir()
prefix = home / "native-install"
(prefix / "include").mkdir(parents=True)
(prefix / "lib").mkdir()
shutil.copy2(native / "include/thinkthen.h", prefix / "include/thinkthen.h")
for name in ("libthinkthen.so", "libthinkthen.a"):
    shutil.copy2(native / "lib" / name, prefix / "lib" / name)
(prefix / "lib/libthinkthen.so.0").symlink_to("libthinkthen.so")
shutil.copytree(native / "lib/pkgconfig", prefix / "lib/pkgconfig")
module = home / "module"
module.mkdir()
for name in ("LICENSE", "README.md", "go.mod", "thinkthen.go", "thinkthen_test.go", "recovery_test.go"):
    shutil.copy2(wrapper / name, module / name)
shutil.copytree(wrapper / "examples", module / "examples")
consumer = home / "external"
consumer.mkdir()
(consumer / "go.mod").write_text(
    "module example.org/installed-consumer\n\ngo 1.22\n\n"
    "require github.com/botassembly/thinkthen/libraries/go v0.0.1\n"
    f"replace github.com/botassembly/thinkthen/libraries/go => {module}\n"
)
shutil.copy2(module / "examples/decide/main.go", consumer / "main.go")
abi_env = os.environ | {
    "THINKTHEN_NATIVE_PREFIX": str(prefix),
    "THINKTHEN_NATIVE_HEADER": str(native / "include/thinkthen.h"),
    "THINKTHEN_NATIVE_SHARED": str(native / "lib/libthinkthen.so"),
    "THINKTHEN_NATIVE_STATIC": str(native / "lib/libthinkthen.a"),
}
subprocess.run([sys.executable, str(Path(__file__).with_name("abi.py"))], env=abi_env, check=True)
barrier = home / "barrier"
barrier.mkdir()
server = Backend(barrier)
try:
    env = {
        "PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": str(home),
        "GOCACHE": str(home / "cache"), "GOMODCACHE": str(home / "modcache"),
        "GOPROXY": "off", "GOSUMDB": "off", "GOTOOLCHAIN": "local", "CGO_ENABLED": "1",
        "PKG_CONFIG_PATH": str(prefix / "lib/pkgconfig"), "LD_LIBRARY_PATH": str(prefix / "lib"),
        "THINKTHEN_API_KEY": "tt-canary-274",
        "THINKTHEN_BASE_URL": f"http://127.0.0.1:{server.server_port}/generic/v1",
        "THINKTHEN_CACHE": str(home / "thinkthen-cache"), "TT_BARRIER_DIR": str(barrier),
    }
    binary = home / "consumer-shared"
    subprocess.run([os.environ.get("THINKTHEN_GO_BIN", "go"), "build", "-buildvcs=false", "-p", "2", "-o", str(binary), "."],
                   cwd=consumer, env=env, check=True)
    linked = subprocess.check_output(["ldd", str(binary)], text=True, env=env)
    assert str(prefix / "lib") in linked and "libthinkthen.so.0" in linked, linked
    print("GO_SHARED_LINK " + next(line.strip() for line in linked.splitlines() if "libthinkthen.so.0" in line))
    result = subprocess.run([str(binary)], cwd=consumer, env=env, text=True, capture_output=True, timeout=30)
    assert (result.returncode, result.stdout.strip(), result.stderr) == (0, "outcome=1 probability=0.9", ""), result
    expected = json.loads((Path(__file__).with_name("accepted_requests.jsonl")).read_text().splitlines()[0])
    assert server.requests == [expected] and server.arrivals == ["café"] and server.attempts == 1, server.requests
    print(f"GO_INSTALLED_PASS executable={binary} requests=1 body={json.dumps(expected, ensure_ascii=False, sort_keys=True)}")
    static_pc = home / "static-pc"
    static_pc.mkdir()
    archive = prefix / "lib/libthinkthen.a"
    (static_pc / "thinkthen.pc").write_text(
        "Name: thinkthen\nDescription: installed static C\nVersion: 0.0.1\n"
        f"Cflags: -I{prefix / 'include'}\n"
        f"Libs: {archive} -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc\n"
    )
    static_binary = home / "consumer-static-c"
    static_env = env | {"PKG_CONFIG_PATH": str(static_pc), "GOCACHE": str(home / "static-cache"),
                        "CGO_LDFLAGS_ALLOW": r"^" + re.escape(str(archive)) + r"$"}
    subprocess.run([os.environ.get("THINKTHEN_GO_BIN", "go"), "build", "-buildvcs=false", "-p", "2", "-o", str(static_binary), "."],
                   cwd=consumer, env=static_env, check=True)
    static_linked = subprocess.check_output(["ldd", str(static_binary)], text=True, env=static_env)
    assert "libthinkthen.so" not in static_linked, static_linked
    print(f"GO_STATIC_C_LINK_PASS executable={static_binary} ldd_libthinkthen=absent")
finally:
    server.close()
