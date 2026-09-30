"""Build and run two independent installed GNU Objective-C source consumers."""
import collections
import os
from pathlib import Path
import shutil
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
import tempfile
from backend import Backend

ROOT = Path(__file__).resolve().parents[1]
NATIVE = ROOT.parents[1] / "libraries/c/target/debug/libthinkthen_c.so"
for name in ("alpha", "bravo"):
    with tempfile.TemporaryDirectory(prefix=f"thinkthen-objc-{name}-") as temp:
        work = Path(temp)
        package = work / "installed package"
        package.mkdir()
        for item in ("Sources", "Examples"):
            shutil.copytree(ROOT / item, package / item)
        native = work / "native"
        native.mkdir()
        shutil.copy2(NATIVE, native / "libthinkthen.so.0")
        (native / "libthinkthen.so").symlink_to("libthinkthen.so.0")
        shutil.copy2(ROOT.parents[1] / "libraries/c/include/thinkthen.h", native)
        subprocess.run(["gcc", "-std=gnu11", "-x", "objective-c", f"-I{native}", f"-I{package / 'Sources'}",
                        str(package / "Sources/ThinkThen.m"), str(package / "Sources/TTJSON.c"),
                        str(package / "Examples/consumer.m"), f"-L{native}", "-lthinkthen",
                        "-lobjc", "-pthread", "-lm", "-o", str(work / "consumer")],
                       cwd=work, env=child_env(), check=True, capture_output=True, timeout=60)
        barrier = work / "barrier"
        barrier.mkdir()
        server = Backend(barrier)
        try:
            env = child_env(HOME=str(work), XDG_CACHE_HOME=str(work / "xdg-cache"))
            env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{server.server_port}/generic/v1",
                       THINKTHEN_API_KEY="tt-canary-295", THINKTHEN_CACHE=str(work / "cache"),
                       LD_LIBRARY_PATH=str(native))
            result = subprocess.run([str(work / "consumer")], cwd=work, env=env,
                                    capture_output=True, text=True, timeout=40)
            assert result.returncode == 0 and "INSTALLED_OBJC_CONSUMER_PASS" in result.stdout, (result.stdout, result.stderr)
            assert collections.Counter(server.arrivals) == collections.Counter(["consumer-objc", "consumer-json"]), server.arrivals
            print(f"GNU Objective-C installed {name}: 2 exact arrivals")
        finally:
            server.close()
