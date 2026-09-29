"""Compare current C header, built library, and installed Go native prefix."""
import hashlib
import os
from pathlib import Path
import re
import subprocess

root = Path(__file__).resolve().parents[3]
prefix = Path(os.environ.get("THINKTHEN_NATIVE_PREFIX", root / "target/go/native"))
header = Path(os.environ.get("THINKTHEN_NATIVE_HEADER", root / "libraries/c/include/thinkthen.h"))
built = Path(os.environ.get("THINKTHEN_NATIVE_SHARED", root / "libraries/c/target/debug/libthinkthen_c.so"))
static = Path(os.environ.get("THINKTHEN_NATIVE_STATIC", root / "libraries/c/target/debug/libthinkthen_c.a"))
text = re.sub(r"/\*.*?\*/", "", header.read_text(), flags=re.S)
declared = set(re.findall(r"\b(thinkthen_[a-z_]+)\s*\(", text))
raw = subprocess.check_output(["nm", "-D", "--defined-only", str(built)], text=True)
exported = {row.split()[-1] for row in raw.splitlines() if row.split()[-1].startswith("thinkthen_")}
assert declared and declared == exported, (declared - exported, exported - declared)
assert header.read_bytes() == (prefix / "include/thinkthen.h").read_bytes()
assert built.read_bytes() == (prefix / "lib/libthinkthen.so").read_bytes()
assert static.read_bytes() == (prefix / "lib/libthinkthen.a").read_bytes()
dynamic = subprocess.check_output(["readelf", "-d", str(built)], text=True)
assert "Library soname: [libthinkthen.so.0]" in dynamic
print("GO_ABI_PASS exports=" + str(len(declared)) + " header_sha256=" + hashlib.sha256(header.read_bytes()).hexdigest()
      + " library_sha256=" + hashlib.sha256(built.read_bytes()).hexdigest())
