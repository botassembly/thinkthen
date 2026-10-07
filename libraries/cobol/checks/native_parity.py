"""Execute shared cases through compiled COBOL named calls and typed views."""
import inspect
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance'))
import c_parity as shared

run = subprocess.run

def compile_consumer(command, *args, **kwargs):
    if command[0] != 'cc' or '-fsanitize=address' not in command:
        return run(command, *args, **kwargs)
    source = Path(next(part for part in command if part.endswith('/driver.c')))
    scratch = source.parent
    header = (ROOT / 'libraries/c/include/thinkthen.h').read_text()
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
    obj = scratch / 'consumer.o'
    run(['cobc', '-c', '-free', '-fstatic-call', '-fno-gen-c-decl-static-call', '-o', str(obj), '-A', '-include '+str(ROOT / 'libraries/cobol/src/tt_native.h')+' -Wno-incompatible-pointer-types', '-I', str(ROOT / 'libraries/c/include'), str(cobol)], check=True, env=kwargs.get('env'))
    prefix = '#include "thinkthen.h"\n#include <libcob.h>\n' + '\n'.join(declarations + definitions) + '\n'
    prefix += '\n'.join('#define '+name+' cobol_'+name for _,name,_ in signatures) + '\n'
    text = source.read_text().replace('int main(int argc,char **argv) {', 'int main(int argc,char **argv) { cob_init(0,NULL);')
    source.write_text(prefix + text)
    bridge = [str(ROOT / ('libraries/cobol/src/' + name)) for name in ('tt_complete.c','tt_inputs.c')]
    index = command.index('-o')
    return run(command[:index] + [str(obj), *bridge, '-lcob', '-rdynamic'] + command[index:], *args, **kwargs)

subprocess.run = compile_consumer
source = inspect.getsource(shared.main).replace("required_cases(inventory, 'c')", "required_cases(inventory, 'cobol')").replace("'consumer': 'c'", "'consumer': 'cobol'").replace('C fixture ', 'COBOL fixture ').replace('C shared fixture results:', 'COBOL shared fixture results:')
exec(compile(source, __file__, 'exec'), shared.__dict__)
raise SystemExit(shared.main())
