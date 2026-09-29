"""Compare this sealed pin's public C declarations against its built shared archive."""
import pathlib
import os
import re
import subprocess
import sys
root = pathlib.Path(__file__).resolve().parent
repo = root.parents[2]
header = pathlib.Path(os.environ.get('THINKTHEN_NATIVE_HEADER', repo/'libraries/c/include/thinkthen.h')).read_text()
pattern = r'^\s*(?:(?:thinkthen_engine|thinkthen_cancel_token|const char|char)\s*\*\s*|(?:int|void)\s+)(thinkthen_\w+)\s*\('
declared = set(re.findall(pattern, header, re.M))
raw = subprocess.check_output(['nm', '-D', '--defined-only', str(os.environ.get('THINKTHEN_NATIVE_SHARED', repo/'libraries/c/target/debug/libthinkthen_c.so')))], text=True)
exported = {row.split()[-1] for row in raw.splitlines() if row.split()[-1].startswith('thinkthen_')}
if not declared or declared != exported:
    sys.exit(f'ABI_EXPORT_MISMATCH declared={sorted(declared)} exported={sorted(exported)}')
print(f'ABI_EXPORT_MATCH {len(declared)} {" ".join(sorted(declared))}')
