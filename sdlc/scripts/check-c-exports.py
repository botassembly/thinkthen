"""Check native exports and describe the public ABI with the target C compiler."""
import argparse
import copy
import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "conformance/children"))
from children import child_env


def run(command):
    return subprocess.check_output(command, text=True, env=child_env(keep=('TMPDIR', 'LANG', 'LC_ALL')))


def walk(node):
    yield node
    for child in node.get('inner', []):
        yield from walk(child)


def declarations(header):
    """Clang discovers declarations; it supplies no target layout measurements."""
    if not shutil.which('clang'):
        raise RuntimeError('ABI declaration discovery requires Clang; Windows layout qualification must run the emitted probe with its own C compiler')
    ast = json.loads(run(['clang', '-std=c11', '-x', 'c', '-fsyntax-only', '-Xclang', '-ast-dump=json', str(header)]))
    records = {n['id']: n for n in walk(ast) if n['kind'] == 'RecordDecl' and n.get('completeDefinition')}
    layouts, functions = {}, {}
    for node in ast.get('inner', []):
        name = node.get('name', '')
        if not name.startswith('thinkthen_'):
            continue
        if node['kind'] == 'TypedefDecl':
            refs = [n['decl']['id'] for n in walk(node) if n['kind'] == 'RecordType']
            if refs and refs[0] in records:
                layouts[name] = fields(records[refs[0]])
        elif node['kind'] == 'FunctionDecl':
            signature = node['type']['qualType']
            if node.get('variadic') or '__attribute__' in signature:
                raise ValueError(f'unsupported C calling convention or variadic declaration: {name}')
            functions[name] = {'return': signature.split('(', 1)[0].strip(),
                               'arguments': [n['type']['qualType'] for n in node.get('inner', []) if n['kind'] == 'ParmVarDecl']}
    macros = run(['clang', '-std=c11', '-x', 'c', '-dM', '-E', str(header)])
    constants = sorted(set(re.findall(r'^#define (THINKTHEN_\w+)[ \t]+\S', macros, re.M)) |
                       {n['name'] for n in walk(ast) if n['kind'] == 'EnumConstantDecl' and n.get('name', '').startswith('THINKTHEN_')})
    if not layouts or not functions or not constants:
        raise ValueError('the public header must declare carriers, functions and constants')
    return {'records': layouts, 'functions': functions, 'constants': constants}


def fields(record, prefix=''):
    """Flatten named fields and anonymous union arms from compiler AST nodes."""
    out, anonymous = {}, None
    for node in record.get('inner', []):
        if node['kind'] == 'RecordDecl' and node.get('completeDefinition') and not node.get('name'):
            anonymous = node
        elif node['kind'] == 'FieldDecl':
            name = prefix + node['name']
            kind = node['type']['qualType']
            if anonymous is not None:
                if '(unnamed' not in kind and '(anonymous' not in kind:
                    raise ValueError(f'unassociated anonymous C record: {name}')
                out[name] = anonymous['tagUsed']
                out.update(fields(anonymous, name + '.'))
                anonymous = None
            else:
                out[name] = kind
    return out


def probe_source(header, declared):
    """Portable C11 source; compile and execute it on the platform being checked."""
    lines = ['#include <stddef.h>', '#include <stdint.h>', '#include <stdio.h>',
             '#include ' + json.dumps(str(Path(header).resolve())),
             '#if defined(_MSC_VER)', '#define ABI_CALL __cdecl',
             '#elif defined(_WIN32)', '#define ABI_CALL __attribute__((cdecl))',
             '#else', '#define ABI_CALL', '#endif']
    for name, prototype in declared['functions'].items():
        arguments = ', '.join(prototype['arguments']) or 'void'
        lines += [f'typedef {prototype["return"]} (ABI_CALL *abi_{name})({arguments});',
                  f'_Static_assert(_Generic(&{name}, abi_{name}: 1, default: 0), "{name} prototype");']
    lines += ['int main(void) {',
              'printf("platform\\t%zu\\t%zu\\t%zu\\t%zu\\n", sizeof(void *), sizeof(size_t), sizeof(long), sizeof(int));',
              '#ifdef _WIN32', 'puts("calling_convention\\tcdecl");', '#else', 'puts("calling_convention\\tC");', '#endif']
    for name, members in declared['records'].items():
        lines.append(f'printf("record\\t{name}\\t%zu\\t%zu\\n", sizeof({name}), _Alignof({name}));')
        for member in members:
            lines.append(f'printf("field\\t{name}\\t{member}\\t%zu\\t%zu\\n", offsetof({name}, {member}), sizeof((({name} *)0)->{member}));')
    for name, prototype in declared['functions'].items():
        width = '0' if prototype['return'] == 'void' else 'sizeof(' + prototype['return'] + ')'
        widths = [width, *('sizeof(' + argument + ')' for argument in prototype['arguments'])]
        format_ = 'function\\t' + name + '\\t' + '\\t'.join(['%zu'] * len(widths)) + '\\n'
        values = ', '.join('(size_t)(' + width + ')' for width in widths)
        lines.append(f'printf("{format_}", {values});')
    for name in declared['constants']:
        lines.append(f'printf("constant\\t{name}\\t%lld\\n", (long long)({name}));')
    lines += ['return 0;', '}']
    return '\n'.join(lines) + '\n'


def compile_probe(source, compiler=None):
    """MSVC and Unix compilers measure their own ABI; no Linux offsets are reused."""
    compiler = compiler or ('cl' if sys.platform == 'win32' else 'cc')
    if not shutil.which(compiler):
        raise RuntimeError(f'ABI layout qualification requires the target C compiler: {compiler}')
    with tempfile.TemporaryDirectory(prefix='thinkthen-c-abi-') as scratch:
        root = Path(scratch)
        unit, program = root / 'probe.c', root / ('probe.exe' if sys.platform == 'win32' else 'probe')
        unit.write_text(source)
        if Path(compiler).name.lower() in ('cl', 'cl.exe'):
            command = [compiler, '/nologo', '/std:c11', '/W4', '/WX', str(unit), '/Fe' + str(program), '/Fo' + str(root / 'probe.obj')]
        else:
            command = [compiler, '-std=c11', '-Wall', '-Wextra', '-Werror', '-pedantic', str(unit), '-o', str(program)]
        run(command)
        return run([str(program)])


def header_abi(header, compiler=None):
    declared = declarations(header)
    output = compile_probe(probe_source(header, declared), compiler)
    facts = {'records': {}, 'constants': {}, 'functions': copy.deepcopy(declared['functions'])}
    for line in output.splitlines():
        kind, *values = line.split('\t')
        if kind == 'platform':
            facts['platform'] = dict(zip(('pointer', 'size_t', 'long', 'int'), map(int, values)))
        elif kind == 'calling_convention':
            for prototype in facts['functions'].values():
                prototype['calling_convention'] = values[0]
        elif kind == 'record':
            name, size, alignment = values
            facts['records'][name] = {'size': int(size), 'alignment': int(alignment), 'fields': {}}
        elif kind == 'field':
            name, field, offset, width = values
            facts['records'][name]['fields'][field] = {'type': declared['records'][name][field], 'offset': int(offset), 'width': int(width)}
        elif kind == 'function':
            name, returned, *arguments = values
            facts['functions'][name]['return_width'] = int(returned)
            facts['functions'][name]['argument_widths'] = list(map(int, arguments))
        elif kind == 'constant':
            name, value = values
            facts['constants'][name] = int(value)
        else:
            raise ValueError(f'unexpected C ABI probe output: {kind}')
    return facts


def compare_abi(expected, actual):
    """Compare independently selected represented declarations, including omissions."""
    if expected == actual:
        return
    changed = [f'{section}.{name}' for section in ('records', 'constants', 'functions')
               for name in sorted(set(expected.get(section, {})) | set(actual.get(section, {})))
               if expected.get(section, {}).get(name) != actual.get(section, {}).get(name)]
    if expected.get('platform') != actual.get('platform'):
        changed.append('platform')
    raise ValueError('C ABI mismatch: ' + ', '.join(changed))


def check_exports(header, library):
    text = re.sub(r'/\*.*?\*/|//[^\n]*', '', Path(header).read_text(), flags=re.S)
    declared = set(re.findall(r'\b(thinkthen_\w+)\s*\([^;{}]*\)\s*;', text, flags=re.S))
    listing = run(['nm', '-D', '--defined-only', str(library)])
    actual = {line.split()[-1] for line in listing.splitlines() if line.split() and line.split()[-1].startswith('thinkthen_')}
    if not declared or declared != actual:
        raise ValueError(f'ABI export mismatch: missing={sorted(declared-actual)}, extra={sorted(actual-declared)}')
    print(f'native C ABI: {len(actual)} header-derived exports match')


def self_test(header):
    native = header_abi(header)
    compare_abi(native, header_abi(header, 'clang'))
    text = Path(header).read_text()
    plants = [
        ('field', 'const char *data; size_t len;', 'size_t len; const char *data;'),
        ('enum', '#define THINKTHEN_FUNCTION_DECIDE_V1 UINT32_C(1)', '#define THINKTHEN_FUNCTION_DECIDE_V1 UINT32_C(99)'),
        ('argument', 'size_t row, size_t member, thinkthen_details_v1 *', 'size_t row, uint16_t member, thinkthen_details_v1 *'),
        ('return', 'int thinkthen_result_row(', 'uint64_t thinkthen_result_row('),
        ('pointer-depth', 'thinkthen_result **);', 'thinkthen_result *);'),
        ('by-value', 'uint32_t media, thinkthen_optional_string_v1 filename', 'uint32_t media, const thinkthen_optional_string_v1 *filename'),
    ]
    with tempfile.TemporaryDirectory(prefix='thinkthen-c-abi-plants-') as scratch:
        for name, before, after in plants:
            if before not in text:
                raise ValueError(f'ABI drift plant has no declaration: {name}')
            copied = Path(scratch) / 'thinkthen.h'
            copied.write_text(text.replace(before, after, 1))
            try:
                compare_abi(native, header_abi(copied))
            except ValueError as error:
                if not str(error).startswith('C ABI mismatch:'):
                    raise
            else:
                raise ValueError(f'C ABI drift was accepted: {name}')
    missing = copy.deepcopy(native)
    del missing['functions']['thinkthen_result_row']
    try:
        compare_abi(native, missing)
    except ValueError:
        pass
    else:
        raise ValueError('C ABI missing prototype was accepted')
    print(f'C ABI: {len(native["records"])} carrier layouts/fields, {len(native["constants"])} constants, {len(native["functions"])} prototypes; field/enum/return/argument/pointer/by-value/omission drift refused')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--describe', action='store_true')
    parser.add_argument('--self-test', action='store_true')
    parser.add_argument('header', type=Path)
    parser.add_argument('library', nargs='?', type=Path)
    args = parser.parse_args()
    if args.self_test:
        self_test(args.header)
    elif args.describe:
        print(json.dumps(header_abi(args.header), sort_keys=True))
    elif args.library:
        check_exports(args.header, args.library)
    else:
        parser.error('provide a library, --describe or --self-test')


if __name__ == '__main__':
    main()
