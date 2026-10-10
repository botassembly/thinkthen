"""Compare generated COBOL packet/request storage with the installed C compiler ABI."""
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
from session_records import module, ROOT, template
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
abi = module('cobol_abi_check', ROOT / 'sdlc/scripts/check-c-exports.py')

library, header, output, *packages = sys.argv[1:]
package = Path(packages[0]) if packages else ROOT / 'libraries/cobol'
abi.check_exports(header, library)
copybook = (package / 'copybooks/tt-native-generated.cpy').read_text()
with tempfile.TemporaryDirectory(prefix='thinkthen-cobol-generated-abi-') as temporary:
    work = Path(temporary)
    shutil.copy(header, work / 'thinkthen.h')
    for name in ('tt_session.h','tt_requests_generated.h'): shutil.copy(package / 'src' / name, work / name)
    combined = work / 'bridge.h'
    combined.write_text('#include \"tt_session.h\"\n')
    facts = abi.header_abi(combined)
    c = ['#include "bridge.h"','#include <stddef.h>','#include <stdio.h>','int main(void) {']
    cobol = ['identification division.','program-id. GeneratedAbi.','data division.','working-storage section.','copy "tt-native-generated.cpy".','01 measured usage binary-double signed.','procedure division.']
    counted = 0
    for name, record in facts['records'].items():
        if not name.startswith(('thinkthen_complete_', 'thinkthen_cobol_')) or name.endswith('_data_v1'): continue
        group = template.identifier(name)
        assert '01 ' + group + ' based.' in copybook, name
        body = re.search(r'01 ' + re.escape(group) + r' based\.(.*?)(?=\n01 |\n78 |\Z)',copybook,re.S)[1]
        cobol.append('allocate ' + group)
        c.append(f'printf("{name} size %zu\\n",sizeof({name}));')
        cobol.append(f'display "{name} size " length of {group}')
        for member, value in record['fields'].items():
            if '.' in member: continue
            field = 'v-' + member.replace('_','-').rstrip('-')
            assert re.search(r'\b02 ' + re.escape(field) + r'(?:\.|\s)',body), (name,member)
            c.extend([f'printf("{name} {member} offset %zu\\n",offsetof({name},{member}));',f'printf("{name} {member} width %zu\\n",sizeof((({name}*)0)->{member}));'])
            cobol.extend([f'call static "field_offset" using by reference {group} {field} of {group} returning measured',f'display "{name} {member} offset " measured',f'display "{name} {member} width " length of {field} of {group}'])
        cobol.append('free ' + group); counted += 1
    for name,value in facts['constants'].items():
        label = template.identifier(name.lower() + '_constant')
        if re.search(r'78 ' + re.escape(label) + ' value ',copybook):
            c.append(f'printf("{name} value %lld\\n",(long long){name});')
            cobol.append(f'display "{name} value " {label}')
    c += ['return 0;}']
    cobol += ['move 0 to return-code goback.']
    (work / 'layout.c').write_text('\n'.join(c))
    (work / 'layout.cob').write_text('\n'.join(cobol)+'\n')
    (work / 'offset.c').write_text('#include <stdint.h>\nint64_t field_offset(const char *owner,const char *field) {return field-owner;}\n')
    env = child_env(LC_ALL='C.UTF-8')
    subprocess.run(['cc','-std=c11','-I'+str(Path(header).parent),str(work / 'layout.c'),'-o',str(work / 'c-layout')],env=env,check=True)
    subprocess.run(['cobc','-x','-free','-I'+str(package / 'copybooks'),str(work / 'layout.cob'),str(work / 'offset.c'),'-o',str(work / 'cobol-layout')],env=env,check=True)
    def measured(binary):
        rows = subprocess.check_output([str(binary)],env=env,text=True).splitlines()
        return {' '.join(row.split()[:-1]):int(row.split()[-1]) for row in rows}
    wanted,actual = measured(work / 'c-layout'), measured(work / 'cobol-layout')
    assert wanted == actual, [(key,wanted[key],actual.get(key)) for key in wanted if wanted[key] != actual.get(key)]
print(f'COBOL generated ABI: {counted} packet/request layouts and {len(actual)} compiler measurements match')
Path(output).write_text('\n'.join(sorted(abi.declarations(header)['functions']))+'\n')
