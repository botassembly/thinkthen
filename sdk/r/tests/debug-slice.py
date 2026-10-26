"""Build and test the isolated debug R consumer without production native assets."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
environment = dict(os.environ, CARGO_NET_OFFLINE="true")
target = Path(environment.get("CARGO_TARGET_DIR", ROOT / "target")).resolve()
environment["CARGO_TARGET_DIR"] = str(target)
environment["R_HOME"] = subprocess.check_output(["R", "RHOME"], text=True).strip()
subprocess.run(["cargo", "build", "-p", "thinkthen-r", "--locked"], cwd=ROOT, env=environment, check=True)
subprocess.run(["cargo", "build", "-p", "loopback", "--bin", "loopback", "--locked"],
               cwd=ROOT, env=environment, check=True)
with tempfile.TemporaryDirectory(prefix="thinkthen-r-debug-") as temporary:
    scratch = Path(temporary)
    stage = scratch / "thinkthen"
    stage.mkdir()
    for name in ("DESCRIPTION", "NAMESPACE", "LICENSE"):
        shutil.copy2(ROOT / "sdk/r" / name, stage / name)
    shutil.copytree(ROOT / "sdk/r/R", stage / "R")
    libraries = stage / "inst/libs"
    libraries.mkdir(parents=True)
    shutil.copy2(ROOT / "sdk/r/src/entrypoint.c", scratch / "entrypoint.c")
    subprocess.run(["R", "CMD", "SHLIB", "entrypoint.c", str(target / "debug/libthinkthen_r.a"),
                    "-o", str(libraries / "thinkthen.so")],
                   cwd=scratch, env=environment, check=True)
    library = scratch / "library"
    library.mkdir()
    (scratch / "home").mkdir()
    subprocess.run(["R", "CMD", "INSTALL", "--library=" + str(library), str(stage)],
                   cwd=scratch, env=environment, check=True)
    environment["THINKTHEN_LOOPBACK"] = str(target / "debug/loopback")
    subprocess.run(["python3", str(ROOT / "sdk/r/tests/driver.py"), "--root", str(ROOT),
                    "--library", str(library), "--scratch", str(scratch)],
                   cwd=ROOT, env=environment, check=True)
print("Isolated debug R slice passed; production native assets were not prepared.")
