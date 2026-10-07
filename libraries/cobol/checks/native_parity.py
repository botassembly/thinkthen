"""Execute shared cases through compiled COBOL named calls and typed views."""
from pathlib import Path
import re
import subprocess
import tempfile
import os
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance'))
import c_parity as shared

run = subprocess.run
PACKAGE = Path(os.environ.get("THINKTHEN_PARITY_PACKAGE", ROOT / "libraries/cobol" )).resolve(strict=True)
HEADER = Path(os.environ.get("THINKTHEN_C_HEADER", ROOT / "libraries/c/include/thinkthen.h")).resolve(strict=True)
if os.environ.get("THINKTHEN_ARTIFACT") and not os.environ.get("THINKTHEN_PARITY_PACKAGE"):
    raise ValueError("installed cobol parity requires its extracted package")

def adapter(scratch):
    header = HEADER.read_text()
    header = re.sub(r'/\*.*?\*/', '', header, flags=re.S)
    signatures = []
    for match in re.finditer(r'\b(int|void)\s+(thinkthen_\w+)\s*\(([^;{}]+)\)\s*;', header):
        ret, name, arguments = match.groups()
        if name == 'thinkthen_question_file':
            continue
        if name.endswith('_complete') or name.endswith('_batch_start') or name.startswith(('thinkthen_result_', 'thinkthen_batch_', 'thinkthen_question_', 'thinkthen_source_', 'thinkthen_image_')):
            signatures.append((ret, name, arguments))
    declarations, definitions, programs = [], [], []
    for ret, name, arguments in signatures:
        # COBOL calls the C by-value descriptor bridge, keeping native counts.
        parameters = []
        for at, argument in enumerate(arguments.split(',')):
            argument = argument.strip()
            argument = re.sub(r'\b(?:engine|role|path|name|reference|json|row|member|observation|media|filename|out)\b$', '', argument).strip()
            parameters.append((argument, 'p' + str(at)))
        params = ','.join(typ + ' ' + name for typ, name in parameters)
        declarations.append(f'{ret} cobol_{name}({params});')
        program_name = 'P_' + name.removeprefix('thinkthen_').upper()
        # Fixed COBOL linkage fields are pointers and explicit native-width
        # scalars; by-value C structs are handled by the small C public bridge.
        if any(typ in ('thinkthen_string_v1', 'thinkthen_optional_string_v1') for typ, _ in parameters):
            bridge_params = [(typ + ' *' if typ in ('thinkthen_string_v1', 'thinkthen_optional_string_v1') else typ, p) for typ,p in parameters]
        else:
            bridge_params = parameters
        linkage, passing = [], []
        for typ, p in bridge_params:
            category = 'usage pointer' if '*' in typ else 'usage binary-long unsigned' if typ == 'uint32_t' else 'usage binary-double unsigned'
            linkage.append(f'01 {p} {category}.')
            passing.append('by value ' + ('size is 4 ' if typ=='uint32_t' else 'size is 8 ' if '*' not in typ else '') + p)
        call_args = ' '.join(passing)
        target = 'TT_' + name.removeprefix('thinkthen_').removesuffix('_complete').upper() if name.endswith('_complete') and name!='thinkthen_error_complete' or name.endswith('_batch_start') else 'TT_' + name.removeprefix('thinkthen_').upper() if name in ('thinkthen_question_new','thinkthen_question_new_authored','thinkthen_question_load','thinkthen_question_parse','thinkthen_question_load_named','thinkthen_question_load_reference','thinkthen_image_clone','thinkthen_source_records','thinkthen_source_files','thinkthen_source_image_files') else name
        programs.append(f'identification division.\nprogram-id. {program_name}.\ndata division.\nlinkage section.\n'+ '\n'.join(linkage) + '\nprocedure division using ' + ' '.join('by reference '+p for _,p in bridge_params) + ' .\n call "'+target+'" using '+call_args+' returning '+('omitted\n move 0 to return-code' if ret=='void' else 'return-code')+'\n goback.\nend program '+program_name+'.\n')
        # Use the runtime's supported C-call protocol. It supplies the parameter
        # count and calls the compiled COBOL program with referenced fields.
        aliases = ''.join(f'{typ} *bridge_{p}=&{p};' for typ,p in parameters
                          if typ in ('thinkthen_string_v1','thinkthen_optional_string_v1'))
        values = ','.join('(void *)&bridge_'+p if typ in ('thinkthen_string_v1','thinkthen_optional_string_v1') else '(void *)&'+p for typ,p in parameters)
        definitions.append(f'{ret} cobol_{name}({params}) {{ '+aliases+f'void *arguments[]={{{values}}}; '+('' if ret=='void' else 'return ')+f'cob_call("{program_name}",{len(parameters)},arguments); }}')
    cobol = scratch / 'consumer.cob'; cobol.write_text('\n'.join(programs))
    prefix = '#include "thinkthen.h"\n#include <libcob.h>\n' + '\n'.join(declarations + definitions) + '\n'
    prefix += '\n'.join('#define '+name+' cobol_'+name for _,name,_ in signatures) + '\n'
    path = scratch / 'adapter.h'; path.write_text(prefix)
    return path


def compile_consumer(scratch, env, include, library, source):
    (scratch / 'consumer.cob').write_text((ADAPTER.parent / 'consumer.cob').read_text())
    cobol = scratch / 'consumer.cob'
    obj = scratch / 'consumer.o'
    run(['cobc', '-c', '-free', '-fstatic-call', '-fno-gen-c-decl-static-call', '-o', str(obj), '-A', '-include '+str(PACKAGE / 'src/tt_native.h')+' -Wno-incompatible-pointer-types', '-I', str(include), str(cobol)], check=True, env=env)
    output = scratch / 'driver'
    bridge = [str(PACKAGE / 'src' / name) for name in ('tt_complete.c','tt_inputs.c')]
    run(['cc', '-pthread', '-std=c11', '-Wall', '-Wextra', '-Werror', '-g', '-fsanitize=address', '-fno-omit-frame-pointer', '-I', str(include), str(source), str(obj), *bridge, str(library), '-lcob', '-rdynamic', '-Wl,-rpath,' + str(scratch), '-o', str(output)], env=env, check=True)
    return output


with tempfile.TemporaryDirectory(prefix='thinkthen-cobol-adapter-') as folder:
    ADAPTER = adapter(Path(folder))
    raise SystemExit(shared.main(consumer='cobol', compile_consumer=compile_consumer,
                                adapter_header=ADAPTER, initialize='cob_init(0,NULL);'))
