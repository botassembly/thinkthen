"""Refuse a call to a C API function its own bindings mark deprecated.

Review 4 (R4-16) found the per-value result functions still in use after
DuckDB 1.5.5 deprecated them. The bindings carry the notice only in their
doc comments, so rustc never warns; this reads the notices from the locked
libduckdb-sys and fails on any `ffi::<name>` use of one in src/.
"""

import json
import pathlib
import re
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent.parent
meta = json.loads(
    subprocess.run(
        ["cargo", "metadata", "--offline", "--locked", "--format-version", "1"],
        cwd=HERE, check=True, capture_output=True, text=True, stdin=subprocess.DEVNULL,
    ).stdout
)
sys_crate = next(p for p in meta["packages"] if p["name"] == "libduckdb-sys")
bindings = pathlib.Path(sys_crate["manifest_path"]).parent / "src" / "bindgen_bundled_version.rs"
text = bindings.read_text()
deprecated = {
    m.group(2)
    for m in re.finditer(r'#\[doc = "(.*?)"\]\s*pub fn (\w+)', text, re.S)
    if "DEPRECATION NOTICE" in m.group(1)
}
if len(deprecated) < 10:
    print(f"FAILED   read only {len(deprecated)} deprecation notices from {bindings.name}; the check cannot see them")
    sys.exit(1)
pattern = re.compile(r"ffi::(" + "|".join(sorted(deprecated)) + r")\b")
uses = [
    f"{path.name}:{number}: {match.group(1)}"
    for path in sorted((HERE / "src").glob("*.rs"))
    for number, line in enumerate(path.read_text().splitlines(), 1)
    for match in pattern.finditer(line)
]
if uses:
    print(f"FAILED   {len(uses)} calls to deprecated DuckDB C API functions:")
    print("\n".join(uses))
    sys.exit(1)
print(f"ok       no call to the {len(deprecated)} deprecated DuckDB C API functions")
