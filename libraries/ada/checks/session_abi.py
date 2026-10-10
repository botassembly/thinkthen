"""Compare generated full-header Ada declarations with actual C compiler layouts."""
from pathlib import Path
import re
import subprocess
import sys
import tempfile
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'conformance/children'))
from children import child_env

ROOT = Path(__file__).resolve().parents[3]
HEADER = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / 'libraries/c/include/thinkthen.h'
ADA = Path(sys.argv[2]) if len(sys.argv) > 2 else ROOT / 'libraries/ada/src/thinkthen_session_c.ads'
source = re.sub(r'/\*.*?\*/', '', HEADER.read_text(), flags=re.S)
imports = set(re.findall(r'External_Name => "(thinkthen_\w+)"', ADA.read_text()))
functions = set(re.findall(r'\b(thinkthen_\w+)\s*\([^;{}]*\)\s*;', source))
assert functions == imports, (functions - imports, imports - functions)
RESERVED = set('abort abs abstract accept access aliased all and array at begin body case constant declare delay delta digits do else elsif end entry exception exit for function generic goto if in interface is limited loop mod new not null of or others out overriding package pragma private procedure protected raise range record rem renames requeue return reverse select separate some subtype synchronized tagged task terminate then type until use when while with xor'.split())
layouts = re.findall(r'typedef\s+(struct|union)\s+(thinkthen_\w+)\s*\{([^}]+)\}\s*\w+\s*;', source)
constants = re.findall(r'#define\s+(THINKTHEN_\w+)\s+([0-9]+)\b', source)
with tempfile.TemporaryDirectory(prefix='thinkthen-ada-layout-') as temporary:
    work = Path(temporary)
    c = ['#include "thinkthen.h"', '#include <stdio.h>', '#include <stddef.h>', 'int main(void) {']
    ada = ['with Ada.Text_IO; use Ada.Text_IO;', 'with Thinkthen_Session_C; use Thinkthen_Session_C;', 'procedure Layout is']
    reads = []
    for kind, typ, body in layouts:
        ada.append(f'V_{typ} : {typ};')
        c.append(f'printf("{typ} size %zu\\n{typ} alignment %zu\\n", sizeof({kind} {typ}), _Alignof({kind} {typ}));')
        reads += [(f'{typ} size', f"{typ}'Size / 8"), (f'{typ} alignment', f"{typ}'Alignment")]
        for field in re.findall(r'\b(\w+)\s*(?:\[[^]]+\])?\s*;', body):
            # GNAT prefixes Ada reserved words, preserving all ordinary C names.
            component = 'c_' + field if field.lower() in RESERVED else field
            if component.endswith('_'):
                component = component + 'u'
            c.append(f'printf("{typ} {field} %zu\\n", offsetof({kind} {typ}, {field}));')
            reads.append((f'{typ} {field}', f"V_{typ}.{component}'Position"))
            c.append(f'printf("{typ} {field} size %zu\\n", sizeof((( {kind} {typ} *) 0)->{field}));')
            reads.append((f'{typ} {field} size', f"V_{typ}.{component}'Size / 8"))
    for constant, _ in constants:
        c.append(f'printf("{constant} %llu\\n", (unsigned long long) {constant});')
        reads.append((constant, 'K_' + constant))
    c.append('return 0; }')
    (work / 'layout.c').write_text('\n'.join(c))
    subprocess.run(['gcc', '-std=c11', '-I' + str(HEADER.parent), str(work / 'layout.c'), '-o', str(work / 'c-layout')], env=child_env(), check=True, capture_output=True)
    output = subprocess.run([str(work / 'c-layout')], env=child_env(), check=True, text=True, capture_output=True).stdout
    measured = {' '.join(row.split()[:-1]): int(row.split()[-1]) for row in output.splitlines()}
    ada += ['begin', *(f'pragma Compile_Time_Error ({expression} /= {measured[key]}, "C/Ada layout differs: {key}");' for key, expression in reads), 'end Layout;']
    (work / 'layout.adb').write_text('\n'.join(ada))
    build = subprocess.run(['gnatmake', '-q', '-gnat2022', '-gnatc', '-I' + str(ADA.parent), 'layout.adb'], cwd=work, env=child_env(), text=True, capture_output=True)
    assert build.returncode == 0, build.stdout + build.stderr
    print(f'Ada full-header ABI: {len(imports)} imports, {len(layouts)} layouts, {len(constants)} constants, {len(measured)} compiler measurements')
