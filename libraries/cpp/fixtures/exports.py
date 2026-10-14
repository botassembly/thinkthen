"""Compare this sealed pin's public C declarations against its built shared archive."""
import pathlib
import os
import importlib.util
import subprocess
import sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
root = pathlib.Path(__file__).resolve().parent
repo = root.parents[2]
header = pathlib.Path(os.environ.get('THINKTHEN_NATIVE_HEADER', repo/'libraries/c/include/thinkthen.h'))
spec = importlib.util.spec_from_file_location('native_exports', repo/'sdlc/scripts/check-c-exports.py')
abi = importlib.util.module_from_spec(spec)
spec.loader.exec_module(abi)
declared = set(abi.declarations(header)['functions'])
raw = subprocess.check_output(['nm', '-D', '--defined-only', str(os.environ.get('THINKTHEN_NATIVE_SHARED', repo/'libraries/c/target/debug/libthinkthen_c.so'))], text=True, env=child_env())
exported = {row.split()[-1] for row in raw.splitlines() if row.split()[-1].startswith('thinkthen_')}
if not declared or declared != exported:
    sys.exit(f'ABI_EXPORT_MISMATCH declared={sorted(declared)} exported={sorted(exported)}')
print(f'ABI_EXPORT_MATCH {len(declared)} {" ".join(sorted(declared))}')
