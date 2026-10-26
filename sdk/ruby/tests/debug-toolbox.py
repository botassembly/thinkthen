"""Check toolbox translation against a debug engine without release assets."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

root = Path(__file__).resolve().parents[3]
environment = dict(os.environ)
target = Path(environment.get("CARGO_TARGET_DIR", root / "target")).resolve()
environment["CARGO_TARGET_DIR"] = str(target)
subprocess.run(["cargo", "build", "--offline", "--locked", "-p", "thinkthen-ffi"],
               cwd=root, env=environment, check=True)
suffix = "dylib" if os.uname().sysname == "Darwin" else "so"
with tempfile.TemporaryDirectory(prefix="ruby-toolbox-") as temporary:
    stage = Path(temporary)
    shutil.copytree(root / "sdk/ruby/lib", stage / "lib")
    shutil.copy2(target / f"debug/libthinkthen_ffi.{suffix}", stage / f"lib/thinkthen/libthinkthen_ffi.{suffix}")
    environment["THINKTHEN_CASES"] = str(root / "conformance")
    environment["GEM_HOME"] = environment["GEM_PATH"] = str(Path.home() / ".local/share/thinkthen-build-tools/ruby/dev")
    subprocess.run(["ruby", "-w", "-I", str(stage / "lib"), "-r", str(root / "sdk/ruby/tests/cases"),
                    "-r", str(root / "sdk/ruby/tests/toolbox_json"), "-e",
                    'isolated_ruby_environment { require "thinkthen"; ruby_toolbox_json }'],
                   cwd=stage, env=environment, check=True)
print("Ruby toolbox translation passed against the debug engine")
