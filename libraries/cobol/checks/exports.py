"""Measure compiled COBOL storage and C bridge declarations against the header."""
import importlib.util
import json
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
spec = importlib.util.spec_from_file_location('c_abi', ROOT / 'sdlc/scripts/check-c-exports.py')
abi = importlib.util.module_from_spec(spec)
spec.loader.exec_module(abi)


def declarations(package):
    text = '\n'.join(p.read_text() for p in sorted((package / 'copybooks').glob('tt-*.cpy')))
    records = {}
    for name, body in re.findall(r'01 (tt-[\w-]+) based\.(.*?)(?=\s*01 |\Z)', text, re.S | re.I):
        fields = [(n, redef, usage) for n, redef, usage in re.findall(r'02 ([\w-]+)(?: redefines ([\w-]+))?\s+(usage [\w-]+(?: unsigned| signed)?|pic x\(\d+\))\.', body, re.I) if not n.startswith('v-padding-')]
        records[name] = (True, body, fields)
    legacy = (package / 'copybooks/thinkthen.cpy').read_text()
    body = re.search(r'01 tt-answer\.(.*?)(?=\s*01 )', legacy, re.S)[1]
    records['tt-answer'] = (False, body, [('tt-outcome', '', 'usage binary-long signed'), ('tt-probability', '', 'usage float-long')])
    # These actual inline legacy carriers also cross the C call boundary.
    text = (package / 'src/tt_decide.cob').read_text()
    for name in ('native-answer',):
        body = re.search(r'01 ' + name + r'\.(.*?)(?=\s*01 )', text, re.S)[1]
        fields = [(n, '', usage) for n, usage in re.findall(r'02 ([\w-]+) (usage [\w-]+(?: unsigned| signed)?)\.', body) if not n.endswith(('pad', 'alignment-pad'))]
        records[name] = (False, body, fields)
    return records


def cname(name):
    return 'thinkthen_answer' if name in ('tt-answer', 'native-answer', 'answer-row') else 'thinkthen_' + name[3:].replace('-', '_')


def fieldname(name, redefines):
    name = re.sub(r'^(v-|tt-|native-)', '', name).replace('-', '_')
    return 'data.' + name if redefines else name


def kind(usage, width):
    if usage.startswith('pic '):
        return 'storage'
    if usage == 'usage pointer':
        return 'pointer'
    if 'float' in usage:
        return 'float' + str(width * 8)
    return ('unsigned' if usage.endswith(' unsigned') else 'signed') + str(width * 8)


def compiled_storage(header, package, records, scratch):
    lines = ['identification division.', 'program-id. AbiStorage.', 'data division.', 'working-storage section.',
             'copy "thinkthen-typed.cpy".', 'copy "thinkthen.cpy".', '01 measured usage binary-long signed.']
    for name in ('native-answer',):
        lines += ['01 ' + name + '.', records[name][1]]
    lines += ['procedure division.']
    for name, (based, _, fields) in records.items():
        if based:
            lines += ['allocate ' + name]
        lines += ['call static "tt_carrier_alignment" using by reference ' + name + ' returning measured',
                  'display "record" x"09" "' + name + '" x"09" length of ' + name + ' x"09" measured']
        for field, _, _ in fields:
            lines += ['call static "tt_carrier_offset" using by reference ' + name + ' ' + field + ' of ' + name + ' returning measured',
                      'display "field" x"09" "' + name + '" x"09" "' + field + '" x"09" measured x"09" length of ' + field + ' of ' + name]
        if based:
            lines += ['free ' + name]
    for name in re.findall(r'78 (tt-c-[\w-]+) value ', (package / 'copybooks/tt-constants.cpy').read_text()):
        lines += ['display "constant" x"09" "THINKTHEN_' + name[5:].upper().replace('-', '_') + '" x"09" ' + name]
    lines += ['display "constant" x"09" "THINKTHEN_NO_DEADLINE" x"09" tt-deadline-ms']
    for tag, names in [('tt-outcome', [('outcome-no', 'THINKTHEN_NO'), ('outcome-yes', 'THINKTHEN_YES'), ('outcome-not-sure', 'THINKTHEN_UNSURE')]),
                       ('tt-failure-code', [('failure-' + n, 'THINKTHEN_E' + n.upper()) for n in ('usage', 'backend', 'deadline', 'local', 'cancelled', 'defect')])]:
        for name, external in names:
            lines += ['set ' + name + ' to true', 'display "constant" x"09" "' + external + '" x"09" ' + tag]
    lines += ['move 0 to return-code', 'goback.']
    unit, fixture = scratch / 'probe.cob', scratch / 'boundary.o'
    unit.write_text('\n'.join(lines) + '\n')
    subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-pedantic', '-I', str(header.parent), '-c', str(ROOT / 'libraries/ada/checks/typed_boundary.c'), '-o', str(fixture)], env=child_env(), check=True)
    program = scratch / 'probe'
    subprocess.run(['cobc', '-x', '-free', '-I', str(package / 'copybooks'), '-o', str(program), str(unit), str(fixture)], env=child_env(), check=True)
    return subprocess.check_output([str(program)], text=True, env=child_env())


def bridge(header, package, native, scratch):
    # Actual bridge prototypes intentionally pass these counted values by reference.
    expected = {('TT_' + n[len('thinkthen_'):-len('_complete')].upper() if n.endswith('_complete') else 'TT_' + n[len('thinkthen_'):].upper()): p
                for n, p in native['functions'].items() if n in {'thinkthen_' + function + '_complete' for function in ('decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate')} or n.endswith('_batch_start') or n in {
                    'thinkthen_question_new', 'thinkthen_question_new_authored', 'thinkthen_question_new_recognition_v1',
                    'thinkthen_result_recognition_task_v1', 'thinkthen_question_load', 'thinkthen_question_parse',
                    'thinkthen_question_load_named', 'thinkthen_question_load_reference', 'thinkthen_image_clone',
                    'thinkthen_source_records', 'thinkthen_source_files', 'thinkthen_source_image_files'}}
    ast = json.loads(abi.run(['clang', '-std=c11', '-I', str(header.parent), '-x', 'c', '-fsyntax-only', '-Xclang', '-ast-dump=json', str(package / 'src/tt_native.h')]))
    actual = {n['name'] for n in abi.walk(ast) if n['kind'] == 'FunctionDecl' and n.get('name', '').startswith('TT_')}
    if actual != set(expected):
        raise ValueError('C ABI mismatch: COBOL bridge imports')
    lines = ['#include "tt_native.h"']
    for name, prototype in expected.items():
        arguments = prototype['arguments'].copy()
        if name in ('TT_QUESTION_LOAD', 'TT_QUESTION_PARSE', 'TT_QUESTION_LOAD_NAMED', 'TT_QUESTION_LOAD_REFERENCE'):
            at = 1 if name == 'TT_QUESTION_LOAD' else 2
            arguments[at] = 'const thinkthen_string_v1 *'
        elif name == 'TT_IMAGE_CLONE':
            arguments[4] = 'const thinkthen_optional_string_v1 *'
        lines += [f'typedef {prototype["return"]} (*abi_{name})({", ".join(arguments)});',
                  f'_Static_assert(_Generic(&{name}, abi_{name}:1, default:0), "{name} declaration");']
    lines += ['int main(void) { return 0; }']
    unit = scratch / 'bridge.c'
    unit.write_text('\n'.join(lines) + '\n')
    try:
        for original in (unit, package / 'src/tt_inputs.c', package / 'src/tt_complete.c'):
            subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-pedantic', '-I', str(header.parent), '-I', str(package / 'src'), '-c', str(original), '-o', str(scratch / (original.stem + '.o'))], env=child_env(), check=True)
    except subprocess.CalledProcessError as error:
        raise ValueError('C ABI mismatch: COBOL actual bridge declaration') from error
    return len(expected)


def check(header, package):
    native, records = abi.header_abi(header), declarations(package)
    with tempfile.TemporaryDirectory(prefix='thinkthen-cobol-abi-') as folder:
        scratch = pathlib.Path(folder)
        output = compiled_storage(header, package.resolve(), records, scratch)
        measured, constants = {}, {}
        for line in output.splitlines():
            tag, *v = line.split('\t')
            if tag == 'record':
                measured[v[0]] = {'size': int(v[1]), 'allocated_alignment': int(v[2]), 'fields': {}}
            elif tag == 'field':
                measured[v[0]]['fields'][v[1]] = (int(v[2]), int(v[3]))
            else:
                constants[v[0]] = int(v[1])
        actual = {'records': {}, 'constants': constants, 'functions': {}}
        expected = abi.represented_abi(native, native['records'], [], [])
        for name, (_, _, fields) in records.items():
            external = cname(name)
            wanted = abi.represented_abi(native, [external], [], [])['records'][external]
            required_alignment = wanted.pop('alignment')
            if not measured[name]['allocated_alignment'] or measured[name]['allocated_alignment'] % required_alignment:
                raise ValueError('C ABI mismatch: COBOL allocated storage alignment ' + name)
            wanted['fields'] = {n: {**f, 'type': 'storage' if f['type'] == 'union' or f['type'].startswith('thinkthen_') else f['type']} for n, f in wanted['fields'].items()}
            actual_fields = {}
            for field, redef, usage in fields:
                offset, width = measured[name]['fields'][field]
                actual_fields[fieldname(field, redef)] = {'offset': offset, 'width': width, 'type': kind(usage, width)}
            record = {'size': measured[name]['size'], 'fields': actual_fields}
            abi.compare_abi({'records': {external: wanted}}, {'records': {external: record}})
            actual['records'][external] = record
            expected['records'][external] = wanted
        required_constants = {n for n in native['constants'] if n.endswith('_V1')} | {'THINKTHEN_NO_DEADLINE', 'THINKTHEN_NO', 'THINKTHEN_YES', 'THINKTHEN_UNSURE',
            'THINKTHEN_EUSAGE', 'THINKTHEN_EBACKEND', 'THINKTHEN_EDEADLINE', 'THINKTHEN_ELOCAL', 'THINKTHEN_ECANCELLED', 'THINKTHEN_EDEFECT'}
        expected['constants'] = {n: native['constants'][n] for n in required_constants}
        abi.compare_abi(expected, actual)
        imports = bridge(header, package.resolve(), native, scratch)
    print(f'COBOL C ABI: {len(actual["records"])} compiled storage layouts, {len(constants)} evaluated constants, {imports} header-compiled bridge declarations match')


def plants(header, package):
    with tempfile.TemporaryDirectory(prefix='thinkthen-cobol-abi-plants-') as folder:
        copied = pathlib.Path(folder)
        shutil.copytree(package / 'copybooks', copied / 'copybooks')
        shutil.copytree(package / 'src', copied / 'src')
        cases = [('field offset', 'copybooks/tt-inputs.cpy', '02 v-data usage pointer.\n          02 v-len usage binary-double unsigned.', '02 v-len usage binary-double unsigned.\n          02 v-data usage pointer.'),
                 ('field width', 'copybooks/tt-inputs.cpy', '02 v-len usage binary-double unsigned.', '02 v-len usage binary-long unsigned.'),
                 ('constant', 'copybooks/tt-constants.cpy', 'tt-c-content-text-v1 value 1.', 'tt-c-content-text-v1 value 99.'),
                 ('prototype', 'src/tt_native.h', 'int TT_QUESTION_NEW(', 'long TT_QUESTION_NEW(')]
        for name, file, before, after in cases:
            unit = copied / file
            text = unit.read_text()
            if before not in text:
                raise ValueError('COBOL ABI plant has no declaration: ' + name)
            unit.write_text(text.replace(before, after, 1))
            try:
                check(header, copied)
            except ValueError as error:
                if not str(error).startswith('C ABI mismatch:'):
                    raise
            else:
                raise ValueError('COBOL ABI drift accepted: ' + name)
            unit.write_text(text)
            print('COBOL ABI ' + name + ' drift refused')


if __name__ == '__main__':
    library, header, output, *package = sys.argv[1:]
    abi.check_exports(header, library)
    package = pathlib.Path(package[0]) if package else ROOT / 'libraries/cobol'
    check(pathlib.Path(header), package)
    if len(sys.argv) == 4:
        plants(pathlib.Path(header), package)
    pathlib.Path(output).write_text('\n'.join(sorted(abi.declarations(header)['functions'])) + '\n')
