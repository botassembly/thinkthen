"""Compare current C declarations with the built native library's exact export set."""
import re
import subprocess
import sys
from pathlib import Path

header = Path(sys.argv[1]).read_text()
header = re.sub(r"/\*.*?\*/|//[^\n]*", "", header, flags=re.S)
declared = set(re.findall(r"\b(thinkthen_\w+)\s*\([^;{}]*\)\s*;", header, flags=re.S))
listing = subprocess.check_output(["nm", "-D", "--defined-only", sys.argv[2]], text=True)
actual = {line.split()[-1] for line in listing.splitlines() if line.split() and line.split()[-1].startswith("thinkthen_")}
assert declared and declared == actual, ("ABI export mismatch", sorted(declared - actual), sorted(actual - declared))
print(f"native C ABI: {len(actual)} header-derived exports match")
