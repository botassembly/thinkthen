"""Compare actual Dart FFI setters and native types with target C declarations."""
import importlib.util
import json
import os
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
    text = '\n'.join(p.read_text() for p in sorted((package / 'lib/src/native').glob('abi*.dart'))) + (package / 'lib/src/door.dart').read_text()
    return {name: (kind, [(field, annotation or type_) for annotation, type_, field in
                         re.findall(r'(?:@(\w+)\(\)\s*)?external ([\w<>]+) (\w+);', body)])
            for name, kind, body in re.findall(r'final class (\w+) extends (Struct|Union)\s*\{(.*?)\}', text, re.S)}


def cname(name, native):
    if name in ('CLegacyAnswerView', 'Answer'):
        return 'thinkthen_answer'
    stem = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name[1:].removesuffix('View')).lower()
    short = 'thinkthen_' + stem + '_v1'
    return short if short in native['records'] else 'thinkthen_' + stem + '_view_v1'


def split_types(text):
    out, start, depth = [], 0, 0
    for i, c in enumerate(text):
        depth += (c in '<(') - (c in '>)')
        if c == ',' and depth == 0:
            out.append(text[start:i].strip())
            start = i + 1
    return out + ([text[start:].strip()] if text[start:].strip() else [])


def native_types(package):
    signatures = {}
    for owner, relative in [('api', 'lib/src/native/functions.dart'), ('door', 'lib/src/door.dart')]:
        text = (package / relative).read_text()
        for found in re.finditer(r'lookupFunction\s*<', text):
            start, depth, end = found.end(), 1, found.end()
            while depth:
                depth += (text[end] == '<') - (text[end] == '>')
                end += 1
            native = split_types(text[start:end - 1])[0]
            returned, arguments = re.fullmatch(r'(.*?)\s+Function\((.*)\)', native, re.S).groups()
            symbol = re.match(r"\s*\(\s*['\"](thinkthen_\w+)['\"]", text[end:])[1]
            prefix = text[text.rfind('late final', 0, found.start()):found.start()]
            getter = re.search(r'(\w+)\s*=\s*lib[\s.]*$', prefix)[1]
            if (owner, symbol) in signatures:
                raise ValueError('duplicate Dart native declaration: ' + symbol)
            signatures[(owner, symbol)] = (getter, returned.strip(), split_types(arguments))
    text = (package / 'lib/src/session/abi_generated.dart').read_text()
    for signature, symbol in re.findall(r"@Native<(.*?)>\(\s*symbol: ['\"](thinkthen_\w+)['\"]", text, re.S):
        returned, arguments = re.fullmatch(r'(.*?)\s+Function\((.*)\)', signature, re.S).groups()
        signatures[('session', symbol)] = (symbol, returned.strip(), split_types(arguments))
    consumers = '\n'.join(p.read_text() for p in (package / 'lib/src/session').glob('*.dart') if p.name != 'abi_generated.dart')
    required = {n.removesuffix('Pointer') for n in re.findall(r'\b_?abi\.(thinkthen_\w+)', consumers)}
    if required - {n for owner, n in signatures if owner == 'session'}:
        raise ValueError('C ABI mismatch: Dart session missing import')
    return signatures


def type_kind(type_, width, native):
    if type_.startswith('Pointer<'):
        return 'pointer'
    if type_ == 'Void':
        return 'void'
    if type_ in ('Double', 'Float'):
        return 'float' + str(width * 8)
    if type_.startswith('C') or type_ == 'Answer':
        return cname(type_, native)
    return ('unsigned' if type_.startswith('Uint') or type_ == 'Size' else 'signed') + str(width * 8)


def typed_pointer(type_, expected, native, widths):
    """Void pointers erase pointees; represented additional layers stay strict."""
    if not type_.startswith('Pointer<'):
        return
    if not expected.endswith('*'):
        raise ValueError('C ABI mismatch: Dart pointer passed as value')
    inner = type_[8:-1]
    pointed = re.sub(r'\bconst\b', '', expected[:-1]).strip()
    if inner == 'Void':
        return
    if inner.startswith('Pointer<'):
        typed_pointer(inner, pointed, native, widths)
        return
    actual, wanted = type_kind(inner, widths[inner], native), abi.pointee_type(expected, native)
    # The binding reads counted C strings as UTF-8 bytes.
    if pointed == 'char' and inner == 'Uint8':
        wanted = 'unsigned8'
    if actual != wanted:
        raise ValueError('C ABI mismatch: Dart typed pointer ' + type_ + ' / ' + expected)


MEASUREMENT = r'''
final calloc = DynamicLibrary.process().lookupFunction<Pointer<Void> Function(Size, Size), Pointer<Void> Function(int, int)>('calloc');
final free = DynamicLibrary.process().lookupFunction<Void Function(Pointer<Void>), void Function(Pointer<Void>)>('free');
List<int> writeField(Pointer<Uint8> p, int size, void Function() write) {
  final bytes = p.asTypedList(size);
  bytes.fillRange(0, size, 0xa5);
  write();
  final changed = [for (var i = 0; i < size; i++) if (bytes[i] != 0xa5) i];
  if (changed.isEmpty || changed.last - changed.first + 1 != changed.length || changed.any((i) => bytes[i] != 0)) {
    throw StateError('FFI setter did not write one complete zeroed field');
  }
  return [changed.first, changed.length];
}
'''


def probe(package, records, functions, scratch, library, dart):
    # @Native requires the installed package's configured native asset, not a VM fallback.
    spec = importlib.util.spec_from_file_location('native_assets', ROOT / 'libraries/dart/checks/native_assets.py')
    assets = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(assets)
    staged = assets.development(ROOT / 'libraries/dart', library, scratch / 'development')
    shutil.rmtree(staged / 'lib')
    shutil.copytree(package / 'lib', staged / 'lib')
    package = staged
    (scratch / 'pubspec.yaml').write_text('name: abi_probe\npublish_to: none\nenvironment:\n  sdk: ">=3.3.0 <4.0.0"\ndependencies:\n  thinkthen_dart:\n    path: ' + str(staged) + '\n')
    assets.configure(staged, library, [scratch])
    folder = package / 'lib/src/native'
    lines = ["import 'dart:ffi';", "import 'dart:convert';"]
    lines += ["import '" + (folder / f).resolve().as_uri() + "';" for f in ('abi.dart', 'functions.dart', 'question.dart', 'input.dart')]
    lines += ["import '" + (package / 'lib/src/session/abi_generated.dart').resolve().as_uri() + "' as session_abi;"]
    lines += ["import '" + (package / 'lib/src/door.dart').resolve().as_uri() + "';", "import '" + (package / 'lib/src/typed.dart').resolve().as_uri() + "';", MEASUREMENT]
    lines += [f'final class Alignment{name} extends Struct {{ @Uint8() external int prefix; external {name} value; }}' for name in records]
    lines += ['void main(List<String> args) {', 'final api = NativeApi(args.single);', 'final door = Door(args.single);', 'final session = session_abi.NativeAbi();', 'final result = <String, dynamic>{};', 'final layouts = <String, dynamic>{};']
    for name, (_, members) in records.items():
        lines += ['{', f'final p = calloc(1, sizeOf<{name}>()).cast<{name}>();', f'final zero = calloc(1, sizeOf<{name}>()).cast<{name}>();',
                  f'final holder = calloc(1, sizeOf<Alignment{name}>()).cast<Alignment{name}>();', 'try {',
                  f'final align = writeField(holder.cast(), sizeOf<Alignment{name}>(), () {{ holder.ref.value = zero.ref; }}).first;', 'final fields = <String, dynamic>{};']
        for field, type_ in members:
            if type_.startswith('C') or type_ == 'Answer':
                lines += [f'{{ final z = calloc(1, sizeOf<{type_}>()).cast<{type_}>();', 'try {',
                          f'fields["{field}"] = writeField(p.cast(), sizeOf<{name}>(), () {{ p.ref.{field} = z.ref; }});', '} finally { free(z.cast()); } }']
            else:
                value = 'Pointer.fromAddress(0)' if type_.startswith('Pointer<') else '0.0' if type_ in ('Double', 'Float') else '0'
                lines += [f'fields["{field}"] = writeField(p.cast(), sizeOf<{name}>(), () {{ p.ref.{field} = {value}; }});']
        lines += [f'layouts["{name}"] = {{"size":sizeOf<{name}>(), "alignment":align, "fields":fields}};',
                  '} finally { free(p.cast()); free(zero.cast()); free(holder.cast()); }', '}']
    types = {t for _, members in records.values() for _, t in members}
    for _, returned, arguments in functions.values():
        types.update([returned, *arguments])
    for type_ in list(types):
        while type_.startswith('Pointer<'):
            type_ = type_[8:-1]
            types.add(type_)
    session_constants = re.findall(r'const (THINKTHEN_\w+) = ', (package / 'lib/src/session/abi_generated.dart').read_text())
    lines += ['result["session_constants"] = <String, int>{']
    lines += [json.dumps(name) + ': session_abi.' + name + ',' for name in session_constants]
    lines += ['};', 'result["session_errors"] = <String, int>{for (final e in session_abi.NativeErrorKind.values) "THINKTHEN_E${e.name.toUpperCase()}": e.code};']
    lines += ['result["records"] = layouts;', 'result["widths"] = <String, int>{']
    lines += [json.dumps(t) + ': ' + ('0' if t == 'Void' else f'sizeOf<{t}>()') + ',' for t in sorted(types)]
    lines += ['};']
    for (owner, name), (getter, _, _) in functions.items():
        lines += [f'if ({owner}.{getter} is! Function) throw StateError("{name} did not link");']
    lines += ['result["constants"] = <String, int>{']
    for group, values in [('FUNCTION', 'FunctionKind'), ('PROPERTY', 'PropertyKind'), ('LOAD', 'LoaderRole')]:
        lines += [f'for (final v in {values}.values) "THINKTHEN_{group}_${{v.name.replaceAllMapped(RegExp(r"[A-Z]"), (m) => "_${{m[0]}}").toUpperCase()}}_V1": v.index + 1,']
    for name, expr in {'CONTENT_TEXT': 'Content.text("").kind', 'CONTENT_JSON': 'Content.json(null).kind',
                       'RULE_DEFAULT': 'Rule.missing().kind', 'RULE_NULL': 'Rule.none().kind', 'RULE_CUT': 'Rule.cut(0).kind', 'RULE_BAND': 'Rule.band(0, 0).kind',
                       'DECLARATION_ABSENT': 'Declaration.absent().kind', 'DECLARATION_STRING': 'Declaration.string().kind', 'DECLARATION_OBJECT': 'Declaration.object([]).kind'}.items():
        lines += [json.dumps('THINKTHEN_' + name + '_V1') + ': ' + expr + ',']
    for at, name in enumerate(('LINE', 'WINDOW', 'FILE', 'IMAGE_FILE', 'JSONL')):
        lines += [json.dumps('THINKTHEN_SOURCE_' + name + '_V1') + f': FileUnit.values[{at}].index + 1,']
    lines += ['for (final e in ErrorKind.values) "THINKTHEN_E${e.name.toUpperCase()}": e.code,', '"THINKTHEN_NO": Outcome.no.index,', '"THINKTHEN_YES": Outcome.yes.index,', '"THINKTHEN_UNSURE": Outcome.notSure.index,', '};', 'print(jsonEncode(result));', '}']
    unit = scratch / 'probe.dart'
    unit.write_text('\n'.join(lines) + '\n')
    env = child_env(keep=('HOME', 'PUB_CACHE', 'LANG', 'XDG_CONFIG_HOME', 'XDG_CACHE_HOME', 'XDG_DATA_HOME', 'XDG_STATE_HOME'))
    subprocess.run([dart, 'pub', 'get', '--offline', '--directory', str(scratch)], env=env, check=True, stdout=sys.stderr)
    command = [dart, 'run', '--verbosity=error', str(unit), str(library)]
    return json.loads(subprocess.check_output(command, text=True, cwd=scratch, env=env))


def check(header, library, package, dart):
    complete = abi.header_abi(header)
    native = abi.retained_abi(complete)
    records, functions = declarations(package), native_types(package)
    with tempfile.TemporaryDirectory(prefix='thinkthen-dart-abi-') as folder:
        measured = probe(package.resolve(), records, functions, pathlib.Path(folder), library, dart)
    actual = {'records': {}, 'functions': {}, 'constants': measured['constants']}
    widths = measured['widths']
    def fields(name, prefix='', offset=0):
        result = {}
        for field, type_ in records[name][1]:
            position, width = measured['records'][name]['fields'][field]
            union = type_ in records and records[type_][0] == 'Union'
            if type_.startswith('Pointer<'):
                typed_pointer(type_, native['records'][cname(name, native)]['fields'][field]['type'], native, widths)
            result[prefix + field] = {'offset': offset + position, 'width': width, 'type': 'union' if union else type_kind(type_, width, native)}
            if union:
                result.update(fields(type_, prefix + field + '.', offset + position))
        return result
    expected = abi.represented_abi(native, native['records'], [], [])
    for name, (kind, _) in records.items():
        if kind == 'Union':
            continue
        measured_record = measured['records'][name]
        record = {'size': measured_record['size'], 'alignment': measured_record['alignment'], 'fields': fields(name)}
        abi.compare_abi({'records': {cname(name, native): expected['records'][cname(name, native)]}}, {'records': {cname(name, native): record}})
        actual['records'][cname(name, native)] = record
    expected_functions = abi.represented_abi(complete, [], complete['functions'], [])['functions']
    for (owner, name), (_, returned, arguments) in functions.items():
        for i, type_ in enumerate(arguments):
            typed_pointer(type_, complete['functions'][name]['arguments'][i], native, widths)
        typed_pointer(returned, complete['functions'][name]['return'], native, widths)
        actual['functions'][name] = {'return': type_kind(returned, widths[returned], native), 'return_width': widths[returned],
            'arguments': [type_kind(t, widths[t], native) for t in arguments], 'argument_widths': [widths[t] for t in arguments], 'calling_convention': 'C'}
        abi.compare_abi({'functions': {name: expected_functions[name]}}, {'functions': {name: actual['functions'][name]}})
    prefixes = ('THINKTHEN_FUNCTION_', 'THINKTHEN_PROPERTY_', 'THINKTHEN_LOAD_', 'THINKTHEN_SOURCE_', 'THINKTHEN_CONTENT_', 'THINKTHEN_RULE_', 'THINKTHEN_DECLARATION_')
    constants = {n for n in native['constants'] if n.endswith('_V1') and n.startswith(prefixes)}
    # These legacy entry points are not imported by either public door.
    omitted = {'thinkthen_decide_opts', 'thinkthen_decide_with_facts', 'thinkthen_decide_many_opts',
        'thinkthen_decide_many_with_facts', 'thinkthen_recognize_opts', 'thinkthen_recognize_with_facts',
        'thinkthen_relate_opts', 'thinkthen_relate_with_facts', 'thinkthen_question_file', 'thinkthen_result_row'}
    constants |= {'THINKTHEN_NO', 'THINKTHEN_YES', 'THINKTHEN_UNSURE', 'THINKTHEN_EUSAGE', 'THINKTHEN_EBACKEND',
                  'THINKTHEN_EDEADLINE', 'THINKTHEN_ELOCAL', 'THINKTHEN_ECANCELLED', 'THINKTHEN_EDEFECT'}
    expected = abi.represented_abi(native, native['records'], set(native['functions']) - omitted, constants)
    for owner, name in functions:
        if owner == 'session':
            expected['functions'][name] = expected_functions[name]
    required_constants = {n: v for n, v in complete['constants'].items() if n.startswith('THINKTHEN_SESSION_') or n.startswith('THINKTHEN_E') and not n.endswith('_V1')}
    abi.compare_abi({'constants': required_constants}, {'constants': measured['session_constants']})
    abi.compare_abi({'constants': {n: v for n, v in required_constants.items() if not n.startswith('THINKTHEN_SESSION_')}}, {'constants': measured['session_errors']})
    abi.compare_abi(expected, actual)
    print(f'Dart C ABI: {len(actual["records"])} setter-measured layouts, {len(actual["constants"])} evaluated constants, {len(actual["functions"])} compiled native imports match')


def plants(header, library, package, dart):
    with tempfile.TemporaryDirectory(prefix='thinkthen-dart-abi-plants-') as folder:
        copied = pathlib.Path(folder)
        shutil.copytree(package / 'lib', copied / 'lib')
        (copied / '.dart_tool').mkdir()
        shutil.copyfile(package / '.dart_tool/package_config.json', copied / '.dart_tool/package_config.json')
        cases = [('signed counts', 'lib/src/door.dart', 'Size', 'IntPtr'),
                 ('field offset', 'lib/src/native/abi.dart', 'external Pointer<Uint8> data;\n  @Size()\n  external int len;', '@Size()\n  external int len;\n  external Pointer<Uint8> data;'),
                 ('field width', 'lib/src/native/abi.dart', 'external Pointer<Uint8> data;\n  @Size()', 'external Pointer<Uint8> data;\n  @Uint32()'),
                 ('alignment', 'lib/src/native/abi.dart', 'final class CStringsView extends Struct', '@Packed(4)\nfinal class CStringsView extends Struct'),
                 ('constant', 'lib/src/native/question.dart', 'const Rule.band(this.low, this.high) : kind = 3;', 'const Rule.band(this.low, this.high) : kind = 99;'),
                 ('return', 'lib/src/native/functions.dart', 'Int32 Function(Pointer<Void>, Size, Pointer<CSourceRelationsView>)', 'Int64 Function(Pointer<Void>, Size, Pointer<CSourceRelationsView>)')]
        for name, file, before, after in cases:
            unit = copied / file
            text = unit.read_text()
            if before not in text:
                raise ValueError('Dart ABI plant has no declaration: ' + name)
            unit.write_text(text.replace(before, after) if name == 'signed counts' else text.replace(before, after, 1))
            try:
                check(header, library, copied, dart)
            except ValueError as error:
                if not str(error).startswith('C ABI mismatch:'):
                    raise
            else:
                raise ValueError('Dart ABI drift accepted: ' + name)
            unit.write_text(text)
            print('Dart ABI ' + name + ' drift refused')


if __name__ == '__main__':
    library, header, output, *package = sys.argv[1:]
    abi.check_exports(header, library)
    package = pathlib.Path(package[0]) if package else ROOT / 'libraries/dart'
    dart = os.environ.get('TT_DART') or shutil.which('dart')
    if not dart:
        raise RuntimeError('Dart ABI comparison requires the target Dart compiler')
    check(pathlib.Path(header), pathlib.Path(library), package, dart)
    if len(sys.argv) == 4:
        plants(pathlib.Path(header), pathlib.Path(library), package, dart)
    pathlib.Path(output).write_text('\n'.join(sorted(abi.declarations(header)['functions'])) + '\n')
