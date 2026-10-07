"""Compare Ada compiler carrier/import facts with the actual target C header."""
import importlib.util
import json
import pathlib
import re
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
spec = importlib.util.spec_from_file_location('c_abi', ROOT / 'sdlc/scripts/check-c-exports.py')
abi = importlib.util.module_from_spec(spec)
spec.loader.exec_module(abi)


def cname(name):
    name = name.lower().split('.')[-1]
    return 'thinkthen_' + {'byte_string_v1': 'string_v1', 'record_data_v1': 'record_v1'}.get(name, name)


def source_records(source):
    result = {}
    for unit in source.glob('thinkthen_c*.ads'):
        text = re.sub(r'--[^\n]*', '', unit.read_text())
        for name, body in re.findall(r'\btype (\w+)(?:\s*\([^)]*\))? is record(.*?)end record', text, re.S | re.I):
            body = re.sub(r'when \w+ => ', '', body)
            fields = {n.lower(): t.strip() for n, t in re.findall(r'^\s*(\w+)\s*:\s*([\w.]+)(?:\s*:=.*?)?;', body, re.M)}
            result[name.lower()] = fields
    return result


def ada_kind(type_, width):
    type_ = type_.lower().strip()
    if type_.startswith('access ') or type_ in ('system.address', 'handle', 'chars_ptr'):
        return 'pointer'
    if type_.endswith('_data'):
        return 'union'
    if type_ in ('double', 'float', 'interfaces.c.double', 'interfaces.c.c_float'):
        return 'float' + str(width * 8)
    if type_ in ('size_t', 'interfaces.c.size_t') or type_.startswith('interfaces.unsigned_'):
        return 'unsigned' + str(width * 8)
    if type_ in ('int', 'interfaces.c.int') or type_.startswith('interfaces.integer_'):
        return 'signed' + str(width * 8)
    return cname(type_)


def components(record):
    yield from record['record']
    for variant in record.get('variant', []):
        yield from components(variant)


def compiled_units(source, scratch):
    facts = []
    units = [*source.glob('thinkthen_c*.ads'), *source.glob('thinkthen-native*.ads'), *source.glob('thinkthen-native*.adb')]
    for unit in units:
        command = ['gcc', '-c', '-gnatc', '-gnat2022', '-gnatR3j', '-gnatRm', '-I' + str(source), str(unit), '-o', str(scratch / (unit.stem + '.o'))]
        facts.extend(json.loads(subprocess.check_output(command, text=True, cwd=scratch, env=child_env())))
    return facts


def host_layouts(source, compiled):
    declared = source_records(source)
    records = {r['name'].split('.')[-1].lower(): r for r in compiled if 'record' in r}
    def fields(name, prefix='', offset=0):
        result = {}
        for f in components(records[name]):
            member = f['name'].lower()
            type_ = declared[name][member]
            if f['First_Bit'] != 0 or f['Size'] % 8:
                raise ValueError('Ada C carrier field is not byte-aligned: ' + name + '.' + member)
            width = f['Size'] // 8
            mapped = {'end_index': 'end', 'at_index': 'at', 'function_code': 'function', 'original_record': 'record'}.get(member, member)
            result[prefix + mapped] = {'type': ada_kind(type_, width), 'offset': offset + f['Position'], 'width': width}
            if type_.lower().endswith('_data'):
                result.update(fields(type_.lower(), prefix + member + '.', offset + f['Position']))
        return result
    return {cname(name): {'size': r['Size'] // 8, 'alignment': r['Alignment'], 'fields': fields(name)}
            for name, r in records.items() if name in declared and not name.endswith('_data')}


def imported_functions(source):
    result = {}
    pattern = r'\b(function|procedure) (\w+)(?:\s*\(([^()]*)\))?(?: return ([\w.]+))?\s+with Import(?:\s*=>\s*True)?, Convention => (\w+), External_Name => "(thinkthen_\w+)";'
    for unit in [*source.glob('thinkthen_c.ads'), *source.glob('thinkthen-native*.ads'), *source.glob('thinkthen-native*.adb')]:
        for _, name, params, returned, convention, external in re.findall(pattern, unit.read_text(), re.S | re.I):
            arguments = []
            for group in params.split(';') if params else []:
                names, type_ = group.split(':', 1)
                arguments.extend([type_.strip()] * len(names.split(',')))
            package = re.search(r'package(?: body)? ([\w.]+) is', unit.read_text(), re.I)[1]
            result[external] = {'name': package + '.' + name, 'arguments': arguments, 'return': returned or 'void', 'convention': convention}
    return result


def evaluated_declarations(source, scratch, imports):
    packages = [p.stem for p in source.glob('thinkthen_c*.ads')]
    lines = ['with Ada.Text_IO; use Ada.Text_IO;', 'with Interfaces.C; use Interfaces.C;',
             'with Interfaces.C.Strings; use Interfaces.C.Strings;', 'with System;']
    lines += ['with ' + p + '; use ' + p + ';' for p in packages]
    lines += ['procedure Abi_Values is']
    values = []
    for name, signature in imports.items():
        for i, type_ in enumerate(signature['arguments']):
            var = 'V_' + str(len(values))
            initial = 'null' if type_.startswith('access ') else 'System.Null_Address' if type_ in ('Handle', 'System.Address') else 'Null_Ptr' if type_ == 'chars_ptr' else '0' if ada_kind(type_, 1).startswith(('signed', 'unsigned', 'float')) else '(others => <>)'
            lines.append('   ' + var + ' : ' + type_ + ' := ' + initial + ';')
            values.append(('argument\t' + name + '\t' + str(i), var + "'Size / 8"))
            if type_.startswith('access '):
                pointed = re.sub(r'^access(?: constant)? ', '', type_)
                values.append(('pointee\t' + name + '\t' + str(i), pointed + "'Size / 8"))
        if signature['return'] != 'void':
            var = 'V_' + str(len(values))
            initial = 'System.Null_Address' if signature['return'] == 'Handle' else 'Null_Ptr' if signature['return'] == 'chars_ptr' else '0'
            lines.append('   ' + var + ' : ' + signature['return'] + ' := ' + initial + ';')
            values.append(('return\t' + name, var + "'Size / 8"))
    constants = []
    for unit in source.glob('thinkthen_c*.ads'):
        for name in re.findall(r'^\s*(C_\w+)\s*:\s*constant', unit.read_text(), re.M):
            constants.append(('THINKTHEN_' + name[2:].upper(), name))
    constants.append(('THINKTHEN_NO_DEADLINE', 'No_Deadline'))
    values.append(('platform', "System.Address'Size / 8"))
    lines += ['begin']
    for label, expression in values:
        lines.append('   Put_Line ("' + label.replace('\t', '" & ASCII.HT & "') + '" & ASCII.HT & Integer\'Image (' + expression + '));')
    for name, expression in constants:
        lines.append('   Put_Line ("constant' + '" & ASCII.HT & "' + name + '" & ASCII.HT & ' + expression + "'Image);")
    lines += ['end Abi_Values;']
    unit = scratch / 'abi_values.adb'
    unit.write_text('\n'.join(lines) + '\n')
    subprocess.run(['gnatmake', '-q', '-gnat2022', '-I' + str(source), str(unit), '-D', str(scratch), '-o', str(scratch / 'values')], cwd=scratch, env=child_env(), check=True)
    return subprocess.check_output([str(scratch / 'values')], text=True, env=child_env())


def check(header, source):
    native = abi.header_abi(header)
    with tempfile.TemporaryDirectory(prefix='thinkthen-ada-abi-') as folder:
        scratch = pathlib.Path(folder)
        compiled = compiled_units(source.resolve(), scratch)
        actual = {'records': host_layouts(source, compiled), 'functions': {}, 'constants': {}}
        imports = imported_functions(source)
        evaluated = evaluated_declarations(source.resolve(), scratch, imports)
        widths = {}
        returns = {}
        pointees = {}
        pointer_width = 0
        for line in evaluated.splitlines():
            kind, *v = line.split('\t')
            if kind == 'argument':
                widths.setdefault(v[0], []).append(int(v[2]))
            elif kind == 'pointee':
                pointees[(v[0], int(v[1]))] = int(v[2])
            elif kind == 'platform':
                pointer_width = int(v[0])
            elif kind == 'return':
                returns[v[0]] = int(v[1])
            elif kind == 'constant':
                actual['constants'][v[0]] = int(v[1])
        mechanisms = {f['name'].lower(): f for f in compiled if 'formal' in f or 'Convention' in f}
        for name, signature in imports.items():
            compiler = mechanisms[signature['name'].lower()]
            arguments = []
            argument_widths = []
            for i, (type_, width, formal) in enumerate(zip(signature['arguments'], widths.get(name, []), compiler.get('formal', []))):
                if formal['mechanism'] == 'reference':
                    arguments.append('pointer')
                    argument_widths.append(pointer_width)
                else:
                    arguments.append(ada_kind(type_, width))
                    argument_widths.append(width)
                if type_.startswith('access '):
                    pointed = re.sub(r'^access(?: constant)? ', '', type_)
                    represented = ada_kind(pointed, pointees[(name, i)])
                    if abi.pointee_type(native['functions'][name]['arguments'][i], native) != represented:
                        raise ValueError('C ABI mismatch: Ada typed access target ' + name)
            actual['functions'][name] = {'return': 'void' if signature['return'] == 'void' else ada_kind(signature['return'], returns[name]),
                'return_width': returns.get(name, 0), 'arguments': arguments, 'argument_widths': argument_widths,
                'calling_convention': compiler['Convention']}
        omitted = {'thinkthen_decide_with_facts', 'thinkthen_decide_many_with_facts', 'thinkthen_recognize_with_facts', 'thinkthen_relate_with_facts', 'thinkthen_question_file'}
        constants = {n for n in native['constants'] if n.endswith('_V1')} | {'THINKTHEN_NO_DEADLINE'}
        expected = abi.represented_abi(native, native['records'], set(native['functions']) - omitted, constants)
        abi.compare_abi(expected, actual)
    print(f'Ada C ABI: {len(actual["records"])} compiler layouts, {len(actual["constants"])} evaluated constants, {len(actual["functions"])} compiled imports match')


def plants(header, source):
    with tempfile.TemporaryDirectory(prefix='thinkthen-ada-abi-plants-') as folder:
        copied = pathlib.Path(folder)
        for original in source.glob('*'):
            if original.suffix in ('.ads', '.adb'):
                (copied / original.name).write_bytes(original.read_bytes())
        cases = [('field offset', 'thinkthen_c_inputs.ads', 'Data at 0 range 0 .. 63;\n      Len at 8 range 0 .. 63;', 'Data at 8 range 0 .. 63;\n      Len at 0 range 0 .. 63;'),
                 ('constant', 'thinkthen_c_inputs.ads', 'C_CONTENT_TEXT_V1 : constant Interfaces.Unsigned_32 := 1;', 'C_CONTENT_TEXT_V1 : constant Interfaces.Unsigned_32 := 99;'),
                 ('return', 'thinkthen-native-inputs.ads', 'function Question_Author (P1 : System.Address; P2 : access Question_Author_V1) return Interfaces.C.int', 'function Question_Author (P1 : System.Address; P2 : access Question_Author_V1) return Interfaces.Unsigned_64'),
                 ('typed pointer', 'thinkthen-native-inputs.ads', 'P2 : access Question_Author_V1', 'P2 : access Question_Spec_V1'),
                 ('by value', 'thinkthen-native-inputs.ads', 'P2 : Byte_String_V1; P3 : access System.Address', 'P2 : access constant Byte_String_V1; P3 : access System.Address')]
        for name, file, before, after in cases:
            unit = copied / file
            text = unit.read_text()
            if before not in text:
                raise ValueError('Ada ABI plant has no declaration: ' + name)
            unit.write_text(text.replace(before, after, 1))
            try:
                check(header, copied)
            except ValueError as error:
                if not str(error).startswith('C ABI mismatch:'):
                    raise
            else:
                raise ValueError('Ada ABI drift accepted: ' + name)
            unit.write_text(text)
            print('Ada ABI ' + name + ' drift refused')


if __name__ == '__main__':
    library, header, output, *package = sys.argv[1:]
    abi.check_exports(header, library)
    source = pathlib.Path(package[0]) / 'src' if package else ROOT / 'libraries/ada/src'
    check(pathlib.Path(header), source)
    if not package:
        plants(pathlib.Path(header), source)
    pathlib.Path(output).write_text('\n'.join(sorted(abi.declarations(header)['functions'])) + '\n')
