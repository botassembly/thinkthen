"""Per-pin ABI preflight: compare native exports with declarations in sealed header."""
import pathlib
import re
import subprocess
import sys
library, header, output = sys.argv[1:]
text = pathlib.Path(header).read_text()
# Function declarations end at ';'; strip comments so example calls cannot match.
text = re.sub(r'/\*.*?\*/', '', text, flags=re.S)
text = re.sub(r'//[^\n]*', '', text)
declared = set(re.findall(r'\b(thinkthen_\w+)\s*\([^;{}]*\)\s*;', text, flags=re.S))
listing = subprocess.check_output(['nm', '-D', '--defined-only', library], text=True)
actual = {row.split()[-1] for row in listing.splitlines() if row.split() and row.split()[-1].startswith('thinkthen_')}
if not declared or declared != actual:
    raise SystemExit(f'ABI_HEADER_EXPORT_MISMATCH missing={sorted(declared-actual)} extra={sorted(actual-declared)}')
pathlib.Path(output).write_text('\n'.join(sorted(actual))+'\n')
print(f'ABI_EXPORTS_MATCH_HEADER {len(actual)} symbols {output}')
