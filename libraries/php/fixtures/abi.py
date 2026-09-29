"""Compare the current installed native library with its public header."""

import hashlib
import os
from pathlib import Path
import re
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env

ROOT = Path(__file__).resolve().parents[3]
HEADER = ROOT / "libraries/c/include/thinkthen.h"
LIBRARY = ROOT / "libraries/c/target/debug/libthinkthen_c.so"


def main():
    text = re.sub(r"/\*.*?\*/", "", HEADER.read_text(), flags=re.S)
    declared = set(re.findall(r"\b(thinkthen_[a-z_]+)\s*\(", text))
    lines = subprocess.check_output(["nm", "-D", "--defined-only", str(LIBRARY)], text=True, env=child_env()).splitlines()
    exported = {line.split()[-1] for line in lines if line.split()[-1].startswith("thinkthen_")}
    assert exported == declared, {"missing": sorted(declared - exported), "extra": sorted(exported - declared)}
    dynamic = subprocess.check_output(["readelf", "-d", str(LIBRARY)], text=True, env=child_env())
    assert "Library soname: [libthinkthen.so.0]" in dynamic
    print(f"PHP_ABI_PASS source={subprocess.check_output([os.environ.get('THINKTHEN_GIT_BIN', '/usr/bin/git'), 'rev-parse', 'HEAD'], cwd=ROOT, text=True, env=child_env()).strip()} "
          f"exports={len(exported)} header_sha256={hashlib.sha256(HEADER.read_bytes()).hexdigest()} "
          f"library_sha256={hashlib.sha256(LIBRARY.read_bytes()).hexdigest()}")


if __name__ == "__main__":
    main()
