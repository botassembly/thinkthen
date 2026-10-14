#!/usr/bin/env python3
"""Generate result declarations from the existing Rust-derived result schema."""
import argparse
import os
import importlib.util
import json
from pathlib import Path
import sys
import subprocess
import shutil
import tempfile
sys.path.insert(0, str(Path(__file__).resolve().parent))
sys.path.insert(1, str(Path(__file__).resolve().parent / 'templates'))
from registry import OUTPUTS, attributes, invocations, output_path
from native import selected_functions, immediate_schema
from formatting import format_source

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import CARGO, child_env
SCHEMA = ROOT / 'crates/thinkthen/src/public/results/complete.schema.json'
OUTPUT = output_path(ROOT, 'csharp', 'CompleteFacts.g.cs')


def references(value):
    if isinstance(value, dict):
        if '$ref' in value:
            prefix = '#/$defs/'
            reference = value['$ref']
            if not reference.startswith(prefix):
                raise ValueError(f'unsupported result reference: {reference}')
            yield reference[len(prefix):]
        for child in value.values():
            yield from references(child)
    elif isinstance(value, list):
        for child in value:
            yield from references(child)


def graph(schema, roots):
    """Keep the transitive source graph; targets never keep member inventories."""
    definitions = schema['$defs']
    pending = list(roots)
    selected = {}
    while pending:
        name = pending.pop()
        if name in selected:
            continue
        selected[name] = definitions[name]
        pending.extend(references(definitions[name]))
    return dict(sorted(selected.items()))


def object_variants(schema, definitions):
    """Separate object alternatives from assertions on an existing object."""
    if 'oneOf' in schema and 'anyOf' in schema:
        raise ValueError('combined object unions need a typed alternative')
    alternatives = schema.get('oneOf', schema.get('anyOf', []))
    alternatives = [definitions[item['$ref'].removeprefix('#/$defs/')]
                    if '$ref' in item else item for item in alternatives]
    if len(alternatives) < 2:
        return None
    if 'properties' in schema and all(assertion(item) for item in alternatives):
        return None
    if not all(item.get('type') == 'object' and 'properties' in item
               for item in alternatives):
        return None
    common = schema.get('properties', {})
    required = schema.get('required', [])
    result = []
    for item in alternatives:
        overlap = common.keys() & item['properties'].keys()
        if any(common[key] != item['properties'][key] for key in overlap):
            raise ValueError('conflicting object variant properties')
        result.append({**item, 'properties': {**common, **item['properties']},
                       'required': list(dict.fromkeys(required + item.get('required', [])))})
    return result


def assertion(schema):
    """Required-member assertions add no typed fields or object alternatives."""
    return (isinstance(schema, dict) and bool(schema) and
            set(schema) <= {'required', 'not'} and
            ('not' not in schema or assertion(schema['not'])))


def discriminator(variants, definitions):
    """Use unique required literals, otherwise exclusive patterned string identities."""
    common = set.intersection(*(set(item.get('required', [])) for item in variants))
    for member in sorted(common):
        fields = [item['properties'][member] for item in variants]
        fields = [definitions[field['$ref'].removeprefix('#/$defs/')]
                  if '$ref' in field else field for field in fields]
        fields = [{'const': field['enum'][0]} if len(field.get('enum', [])) == 1
                  else field for field in fields]
        if all(isinstance(field.get('const'), str) for field in fields):
            values = [field['const'] for field in fields]
            if len(set(values)) == len(values):
                return [('literal', member, value) for value in values]
    # Flattened Rust enum envelopes can require more than one closed tag.
    literals = []
    for item in variants:
        fields = {member: item['properties'][member].get('const')
                  for member in item.get('required', [])}
        literals.append({member: value for member, value in fields.items()
                         if isinstance(value, str)})
    if all(literals) and all(
            any(left.get(member) != right[member] for member in left.keys() & right.keys())
            for index, left in enumerate(literals) for right in literals[index + 1:]):
        return [('literals', '', fields) for fields in literals]
    tags = []
    for index, item in enumerate(variants):
        others = set.union(*(set(other.get('required', []))
                             for j, other in enumerate(variants) if j != index))
        unique = set(item.get('required', [])) - others
        unique = {member for member in unique if
                  identity_field(item['properties'][member], definitions)}
        if len(unique) != 1:
            break
        tags.append(('member', unique.pop(), None))
    if len(tags) == len(variants):
        return tags
    # Native untagged alternatives can use required-key shapes. Excluding known
    # sibling keys makes partial alternatives refuse instead of becoming extensions.
    required = [set(item.get('required', [])) for item in variants]
    if not all(required) or len({frozenset(fields) for fields in required}) != len(variants):
        raise ValueError('ambiguous object union discriminator')
    return [('structure', '', {
        'required': sorted(fields),
        'excluded': sorted(set.union(*(other for j, other in enumerate(required)
                                      if j != index)) - fields),
    }) for index, fields in enumerate(required)]


def identity_field(field, definitions):
    if '$ref' in field:
        field = definitions[field['$ref'].removeprefix('#/$defs/')]
    return field.get('type') == 'string' and 'pattern' in field


def refinement(schema, source, definitions):
    """Ignore value assertions only when they add no conversion members."""
    if '$ref' in source:
        source = definitions[source['$ref'].removeprefix('#/$defs/')]
    if set(schema) <= {'const', 'type', 'maxItems', 'minItems', 'required', 'not'}:
        return 'not' not in schema or refinement(schema['not'], source, definitions)
    if set(schema) <= {'if', 'then', 'else'}:
        return all(refinement(value, source, definitions) for value in schema.values())
    if set(schema) == {'properties'}:
        alternatives = [source] + source.get('oneOf', source.get('anyOf', []))
        return all(any(member in alternative.get('properties', {}) and
                       refinement(value, alternative['properties'][member], definitions)
                       for alternative in alternatives)
                   for member, value in schema['properties'].items())
    if set(schema) <= {'properties', 'not', 'required'}:
        return all(refinement({key: value}, source, definitions)
                   for key, value in schema.items())
    return False


def json_alternatives(schema, definitions):
    """Resolve native bare-value alternatives by disjoint JSON kinds."""
    if '$ref' in schema:
        source = definitions[schema['$ref'].removeprefix('#/$defs/')]
        alternatives = json_alternatives(source, definitions)
        return alternatives if alternatives and len(alternatives) > 1 else [schema]
    if 'anyOf' in schema:
        alternatives = [json_alternatives(item, definitions) for item in schema['anyOf']]
        if all(alternatives):
            return [item for group in alternatives for item in group]
        return None
    kinds = schema.get('type')
    if isinstance(kinds, list):
        return [{**schema, 'type': kind} for kind in kinds]
    return [schema] if isinstance(kinds, str) else None


def native_value_union(schema, definitions):
    alternatives = json_alternatives(schema, definitions)
    if not alternatives:
        return None
    selected = {}
    for item in alternatives:
        source = definitions[item['$ref'].removeprefix('#/$defs/')] if '$ref' in item else item
        kind = source.get('type')
        if kind in selected and kind != 'null':
            return None
        selected[kind] = item
    if not {'boolean', 'string', 'number', 'array'} <= selected.keys():
        return None
    if selected.keys() - {'boolean', 'string', 'number', 'array', 'object', 'null'}:
        return None
    metadata = {key: schema[key] for key in ('title', 'description', 'x-preserve-order') if key in schema}
    return {**metadata, 'primitive_variants': list(selected), 'primitive_schemas': selected,
            'nullable_json': 'null' in selected}


def normalize(value):
    """Remove only redundant singleton type syntax throughout the source graph."""
    if isinstance(value, list):
        return [normalize(child) for child in value]
    if not isinstance(value, dict):
        return value
    result = {key: normalize(child) for key, child in value.items()}
    kinds = result.get('type')
    if isinstance(kinds, list) and len(kinds) == 1:
        result['type'] = kinds[0]
    return result


def lift_inline(definitions):
    """Name titled objects and nested alternatives from their schema source."""
    definitions = normalize(definitions)
    pending = list(definitions)
    seen = {}
    def visit(value, path):
        if isinstance(value, list):
            return [visit(child, path + '_' + str(index)) for index, child in enumerate(value)]
        if not isinstance(value, dict):
            return value
        if object_variants(value, definitions) or (value.get("title") and "properties" in value):
            identity = json.dumps(value, sort_keys=True)
            if identity not in seen:
                key = value.get('title', path)
                if key in definitions and definitions[key] != value:
                    raise ValueError('colliding nested result alternative name')
                seen[identity] = key
                if key not in definitions:
                    definitions[key] = value
                    pending.append(key)
            return {'$ref': '#/$defs/' + seen[identity]}
        return {key: visit(child, path + '_' + key) for key, child in value.items()}
    while pending:
        key = pending.pop()
        source = definitions[key]
        definitions[key] = {member: visit(value, key + '_' + member)
                            for member, value in source.items()}
    return definitions


def prepare(definitions):
    """Derive reusable tagged object definitions without a target member inventory."""
    definitions = lift_inline(definitions)
    result = {}
    for key, source in definitions.items():
        if 'allOf' in source:
            if not all(refinement(item, source, definitions) for item in source['allOf']):
                raise ValueError('object intersections need an explicit typed representation')
            source = {key: value for key, value in source.items() if key != 'allOf'}
        native = native_value_union(source, definitions)
        if native:
            result[key] = native
            continue
        variants = object_variants(source, definitions)
        if variants is None:
            source = dict(source)
            if 'properties' in source:
                for keyword in ('oneOf', 'anyOf'):
                    alternatives = source.get(keyword, [])
                    if alternatives and all(assertion(item) for item in alternatives):
                        del source[keyword]
            result[key] = source
            continue
        tags = discriminator(variants, definitions)
        alternatives = source.get('oneOf', source.get('anyOf', []))
        names = []
        for index, (mode, member, value) in enumerate(tags):
            original = alternatives[index]
            reference = original.get('$ref', '').removeprefix('#/$defs/')
            if reference and not source.get('properties'):
                suffix = reference.removeprefix('complete')
                parent = key.removeprefix('complete')
                if suffix.lower().endswith(parent.lower()) and len(suffix) > len(parent):
                    suffix = suffix[:-len(parent)]
                name = key + '_' + suffix
            elif 'title' in original:
                name = original['title']
            else:
                suffix = (value if mode == 'literal' else
                          '_'.join(value.values()) if mode == 'literals' else
                          '_'.join(value['required']) if mode == 'structure' else member)
                name = key + '_' + suffix
            names.append(name)
        if len(set(names)) != len(names):
            raise ValueError('colliding generated object variant names')
        result[key] = {'variants': list(zip(names, tags))}
        for name, variant in zip(names, variants):
            if name in definitions and definitions[name] != variant:
                raise ValueError('colliding generated object variant definition')
            result[name] = variant
    return dict(sorted(result.items()))


def generated(schema, language="csharp"):
    if language == "zig":
        return zig_generated(prepare(graph(schema, ("completeplan",))))
    if language in ("r", "python"):
        path = Path(__file__).parent / f"templates/{language}.py"
        spec = importlib.util.spec_from_file_location(f"result_{language}", path)
        target = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(target)
        return subprocess.run(["rustfmt", "--edition", "2024", "--emit", "stdout"],
                              input=target.render(prepare(graph(schema, target.ROOTS))),
                              text=True, capture_output=True, check=True,
                              env=child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'),
                                            CARGO_NET_OFFLINE='true')).stdout
    if language == "c":
        path = Path(__file__).parent / "templates/c.py"
        spec = importlib.util.spec_from_file_location("result_c", path)
        target = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(target)
        definitions = prepare(graph(immediate_schema(schema), ("completesessionPacket", "completeImmediateFacts")))
        selected, pending = {}, ["completesessionPacket", "completeImmediateFacts"]
        while pending:
            name = pending.pop()
            if name in selected:
                continue
            source = selected[name] = definitions[name]
            pending.extend(references(source))
            pending.extend(child for child, _ in source.get("variants", []))
        return subprocess.run(["rustfmt", "--edition", "2024", "--emit", "stdout"],
                              input=target.render(dict(sorted(selected.items()))),
                              text=True, capture_output=True, check=True,
                              env=child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'),
                                            CARGO_NET_OFFLINE='true')).stdout
    sys.path.insert(0, str(Path(__file__).parent / 'templates'))
    path = Path(__file__).parent / 'templates/csharp.py'
    spec = importlib.util.spec_from_file_location('result_csharp', path)
    target = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(target)
    roots = ('completeFacts', 'completeanswer', 'completefindAnswer',
             'completeCallError', 'completeRelationMember', 'completeObservation',
             'completeUsage', 'completequestionDocument', 'completesourceRelationEndpoint',
             'completesessionPacket', 'completeplan')
    return target.render(prepare(graph(schema, roots)))



def zig_generated(definitions):
    path = Path(__file__).parent / 'templates/zig.py'
    spec = importlib.util.spec_from_file_location('result_zig', path)
    target = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(target)
    return subprocess.run(['zig', 'fmt', '--stdin'], input=target.render(definitions),
                          text=True, capture_output=True, check=True,
                          env=child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'))).stdout


def bridge():
    path = ROOT / 'sdlc/scripts/check-c-exports.py'
    spec = importlib.util.spec_from_file_location('csharp_native_abi', path)
    abi = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(abi)
    native = abi.header_abi(ROOT / 'libraries/c/include/thinkthen.h')
    types = {'const char *': 'IntPtr', 'size_t': 'nuint', 'uint32_t': 'uint', 'thinkthen_complete_utf8_v1': 'CompleteUtf8V1'}
    lines = ['// Generated from the compiler-derived C header ABI; do not edit.',
             'using System.Runtime.InteropServices;', 'namespace ThinkThen;']
    carriers = ('thinkthen_string_v1', 'thinkthen_complete_usage_persistence_v1', 'thinkthen_complete_utf8_v1', 'thinkthen_named_request_v1')
    def host(name):
        return ''.join(word.title() for word in name.removeprefix('thinkthen_').split('_'))
    for carrier in carriers:
        lines += ['[StructLayout(LayoutKind.Sequential)]', 'internal struct ' + host(carrier) + ' {']
        lines += [' public ' + types[field['type']] + ' ' + name + ';'
                  for name, field in native['records'][carrier]['fields'].items()]
        lines += ['}']
    for enum, prefix, suffix, underlying in [('FailureKind', 'THINKTHEN_E', '', 'int'), ('UsagePersistenceState', 'THINKTHEN_COMPLETE_USAGE_PERSISTENCE_', '_V1', 'uint')]:
        members = [(name.removeprefix(prefix).removesuffix(suffix), value) for name, value in native['constants'].items()
                   if name.startswith(prefix) and (name.endswith(suffix) if suffix else not name.endswith('_V1'))]
        lines += ['public enum ' + enum + ' : ' + underlying + ' { ' + ', '.join(''.join(word.title() for word in name.split('_')) + ' = ' + str(value) for name, value in members) + ' }']
    parameters = {'const thinkthen_engine *': 'EngineHandle engine',
                  **{carrier + ' *': 'out ' + host(carrier) + ' output' + str(index)
                     for index, carrier in enumerate(carriers)}}
    lines += ['internal static class NativeUsage {']
    for name in ('thinkthen_engine_usage_persistence_v1', 'thinkthen_engine_finish_usage_status_v1'):
        prototype = native['functions'][name]
        lines += [' [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern '
                  + {'int': 'int'}[prototype['return']] + ' ' + name + '('
                  + ', '.join(parameters[argument] for argument in prototype['arguments']) + ');']
    prototype = native['functions']['thinkthen_session_new_named_with_surface']
    named_parameters = {'const thinkthen_engine *': 'EngineHandle engine',
                        'const thinkthen_named_request_v1 *': 'in NamedRequestV1 request',
                        'thinkthen_complete_utf8_v1': 'CompleteUtf8V1 surface',
                        'thinkthen_session **': 'out IntPtr session'}
    lines += ['}', 'internal static class NativeNamed {',
              ' [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_new_named_with_surface('
              + ', '.join(named_parameters[argument] for argument in prototype['arguments']) + ');',
              '}', 'internal static class NativeWait {']
    for name, value in native['constants'].items():
        if name.startswith('THINKTHEN_SESSION_WAIT_'):
            lines += [' internal const uint ' + ''.join(part.title() for part in name.removeprefix('THINKTHEN_SESSION_WAIT_').removesuffix('_V1').split('_')) + ' = ' + str(value) + ';']
    for name, value in native['constants'].items():
        if name.startswith('THINKTHEN_SESSION_READY_'):
            suffix = ''.join(part.title() for part in name.removeprefix('THINKTHEN_SESSION_READY_').removesuffix('_V1').split('_'))
            lines += [' internal const uint Ready' + suffix + ' = ' + str(value) + ';']
    prototype = native['functions']['thinkthen_session_wait']
    parameters = {'const thinkthen_session *': 'SessionHandle session', 'uint32_t': 'uint', 'uint32_t *': 'out uint ready'}
    arguments = [parameters[typ] + (' interest' if at == 1 else ' timeoutMilliseconds' if at == 2 else '') for at, typ in enumerate(prototype['arguments'])]
    lines += [' [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_wait(' + ', '.join(arguments) + ');']
    prototype = native['functions']['thinkthen_session_next_packet']
    packet_parameters = {'thinkthen_session *': 'SessionHandle session', 'uint32_t': 'uint interest',
                         'uint32_t *': 'out uint status', 'thinkthen_session_result **': 'out IntPtr result'}
    lines += [' [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_next_packet('
              + ', '.join(packet_parameters[argument] for argument in prototype['arguments']) + ');']
    prototype = native['functions']['thinkthen_session_push']
    push_parameters = {'thinkthen_session *': 'SessionHandle session', 'const char *': 'byte[] descriptor',
                       'size_t': 'nuint length', 'uint32_t *': 'out uint status'}
    lines += [' [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_push('
              + ', '.join(push_parameters[argument] for argument in prototype['arguments']) + ');']
    for state in ('RESULT', 'PENDING', 'END'):
        lines += [' internal const uint Read' + state.title() + ' = ' + str(native['constants']['THINKTHEN_SESSION_' + state + '_V1']) + ';']
    lines += ['}']
    return '\n'.join(lines) + '\n'

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--all', action='store_true')
    parser.add_argument('--schema', type=Path, default=SCHEMA)
    parser.add_argument('--output', type=Path, default=OUTPUT)
    parser.add_argument('--inputs', action='store_true')
    parser.add_argument('--target', choices=('csharp', 'cobol', 'c', 'r', 'zig', 'python', 'jvm', 'ruby', 'typescript', 'go', 'php', 'cpp', 'dart', 'ada', 'objc', 'swift', 'postgresql', 'sqlite'), default='csharp')
    parser.add_argument('--bridge', action='store_true')
    parser.add_argument('--operations', action='store_true')
    args = parser.parse_args()
    if args.all:
        failures = []
        attributes_path = ROOT / '.gitattributes'
        attributes_source = attributes_path.read_text()
        generated_attributes = attributes(attributes_source)
        if args.check and attributes_source != generated_attributes:
            print('generated Git attributes differ', file=sys.stderr)
            failures.append('registry:attributes')
        elif not args.check:
            attributes_path.write_text(generated_attributes)
        for producer, target, mode in invocations():
            command = ([sys.executable, str(ROOT / 'sdlc/scripts/generate-c-header.py')]
                       if producer == 'header' else
                       [sys.executable, str(Path(__file__).resolve()), '--target', target])
            if mode in ('inputs', 'bridge', 'operations'):
                command.append('--' + mode)
            if args.check:
                command.append('--check')
            completed = subprocess.run(command, env=child_env(
                keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR', 'TT_DART'),
                CARGO_NET_OFFLINE='true'))
            if completed.returncode:
                failures.append(f'{producer}:{target}:{mode}')
        if failures:
            print('generation failed: ' + ', '.join(failures), file=sys.stderr)
        return int(bool(failures))
    if args.target == 'sqlite':
        if args.inputs or args.bridge or args.operations:
            parser.error('SQLite emits receipt projections in one producer invocation')
        command = [sys.executable, str(Path(__file__).parent / 'templates/sqlite.py')]
        if args.check:
            command.append('--check')
        return subprocess.run(command, env=child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'))).returncode
    if args.target == 'postgresql':
        if args.inputs or args.bridge or args.operations:
            parser.error('PostgreSQL emits results and functions in one producer invocation')
        command = [sys.executable, str(Path(__file__).parent / 'templates/postgresql.py')]
        if args.check:
            command.append('--check')
        with tempfile.TemporaryDirectory(prefix='thinkthen-postgresql-generator-') as home:
            return subprocess.run(command, env=child_env(
                keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'), home=home)).returncode
    if args.operations:
        if args.target != 'c' or args.inputs or args.bridge:
            parser.error('operation transport is a separate C producer')
        from operations import OPERATIONS, STATUSES, PROJECTIONS
        path = Path(__file__).parent / 'templates/operations.py'
        spec = importlib.util.spec_from_file_location('operation_target', path)
        emitter = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(emitter)
        from registry import paths
        for module, output in zip(('engine', 'session', 'typed', 'question'), (*paths('c', 'operations')[:3], paths('c', 'operations')[5])):
            result = emitter.render([operation for operation in OPERATIONS if operation.module == module], STATUSES if module == 'session' else (), PROJECTIONS if module == 'session' else ())
            output = ROOT / output
            result = format_source(output, result, child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR')))
            if args.check and (not output.exists() or output.read_text() != result):
                print('generated native operations differ', file=sys.stderr)
                return 1
            if not args.check:
                output.write_text(result)
        from layouts import PRIMITIVES, AUTHOR, JSON_KINDS
        path = Path(__file__).parent / 'templates/layouts.py'
        spec = importlib.util.spec_from_file_location('layout_target', path)
        layouts = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(layouts)
        for records, kinds, output in zip((PRIMITIVES, AUTHOR), (JSON_KINDS, ()), paths('c', 'operations')[3:]):
            output = ROOT / output
            result = format_source(output, layouts.render(records, kinds), child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR')))
            if args.check and (not output.exists() or output.read_text() != result):
                print('generated native layouts differ', file=sys.stderr)
                return 1
            if not args.check:
                output.write_text(result)
        return 0
    if args.target == 'swift':
        sys.path.insert(0, str(Path(__file__).parent / 'templates'))
        import swift
        schema = json.loads((ROOT / 'specification/request.schema.json').read_text()) if args.inputs else json.loads(args.schema.read_text())
        if args.inputs:
            schema['$defs']['Request'] = {key: value for key, value in schema.items() if key != '$defs'}
        result = swift.render(prepare(graph(schema, swift.INPUT_ROOTS if args.inputs else swift.ROOTS)), args.inputs)
        if args.inputs:
            result += 'public let ownedRequestVersion = ' + json.dumps(schema['$defs']['RequestVersion']['oneOf'][0]['const']) + '\n'
        output = output_path(ROOT, 'swift', 'InputsGenerated.swift' if args.inputs else 'ResultsGenerated.swift')
        if args.bridge:
            path = ROOT / 'sdlc/scripts/check-c-exports.py'
            spec = importlib.util.spec_from_file_location('swift_abi', path)
            abi = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(abi)
            result = swift.bridge(abi.header_abi(ROOT / 'libraries/c/include/thinkthen.h'))
            output = output_path(ROOT, 'swift', 'ABIGenerated.swift')
            loader = output_path(ROOT, 'swift', 'bridge.c')
            linux_bridge = swift.linux_bridge(selected_functions(abi.declarations(ROOT / 'libraries/c/include/thinkthen.h')['functions'], 'swift'))
            if args.check:
                if not loader.exists() or loader.read_text() != linux_bridge:
                    print('generated Swift Linux bridge differs', file=sys.stderr)
                    return 1
            else:
                loader.write_text(linux_bridge)
        result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
        if args.check:
            if not output.exists() or output.read_text() != result:
                print('generated Swift types differ', file=sys.stderr)
                return 1
        else:
            output.write_text(result)
        return 0
    if args.target == "objc":
        if args.inputs or args.bridge:
            parser.error("Objective-C generates owned Foundation results only")
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import objc
        definitions = prepare(graph(json.loads(args.schema.read_text()), objc.ROOTS))
        version = json.loads((ROOT / "specification/request.schema.json").read_text())["$defs"]["RequestVersion"]["oneOf"][0]["const"]
        request_schema = json.loads((ROOT / 'specification/request.schema.json').read_text())
        request_schema['$defs']['Request'] = {key: value for key, value in request_schema.items() if key != '$defs'}
        input_definitions = prepare(graph(request_schema, ('Request', 'RequestSessionDescriptor', 'RequestReaderFailure', 'EngineSettings')))
        abi_spec = importlib.util.spec_from_file_location('objc_native_abi', ROOT / 'sdlc/scripts/check-c-exports.py')
        abi_module = importlib.util.module_from_spec(abi_spec)
        abi_spec.loader.exec_module(abi_module)
        abi = abi_module.header_abi(ROOT / 'libraries/c/include/thinkthen.h')
        for filename, result in objc.outputs(definitions, version, input_definitions, abi).items():
            output = output_path(ROOT, 'objc', filename)
            result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
            if args.check:
                if not output.exists() or output.read_text() != result:
                    print("generated Foundation results differ", file=sys.stderr)
                    return 1
            else:
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text(result)
        return 0
    if args.target == "ada":
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import ada
        ada.generate(ROOT, graph, prepare, child_env(), args.check)
        return
    if args.target == "cobol":
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import cobol
        schema = json.loads((ROOT / "specification/request.schema.json").read_text())
        schema["$defs"]["Request"] = {key: value for key, value in schema.items() if key != "$defs"}
        definitions = prepare(graph(schema, ("Request", "RequestSessionDescriptor")))
        for filename, result in cobol.outputs(ROOT, definitions).items():
            output = output_path(ROOT, 'cobol', filename)
            result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
            if args.check:
                if not output.exists() or output.read_text() != result:
                    print(f"generated COBOL records differ: {output}", file=sys.stderr)
                    return 1
            else:
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text(result)
        return 0
    if args.target == "dart":
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import dart
        schema = json.loads((ROOT / "specification/request.schema.json").read_text()) if args.inputs else immediate_schema(json.loads(args.schema.read_text()))
        definitions = prepare(graph(schema, dart.INPUT_ROOTS if args.inputs else dart.ROOTS))
        result = dart.render(definitions, args.inputs)
        if args.bridge:
            path = ROOT / "sdlc/scripts/check-c-exports.py"
            spec = importlib.util.spec_from_file_location("dart_abi", path)
            abi = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(abi)
            result = dart.bridge(abi.header_abi(ROOT / "libraries/c/include/thinkthen.h"), prepare(graph(schema, dart.ROOTS)))
        if args.inputs:
            result += "const requestVersion = " + json.dumps(schema["$defs"]["RequestVersion"]["oneOf"][0]["const"]) + ";\n"
        filename = 'abi_generated.dart' if args.bridge else 'inputs_generated.dart' if args.inputs else 'results_generated.dart'
        outputs = {filename: result}
        if args.inputs:
            outputs['calls_generated.dart'] = dart.calls(definitions, prepare(graph(immediate_schema(json.loads(args.schema.read_text())), dart.ROOTS)))
        for filename, result in outputs.items():
            output = output_path(ROOT, 'dart', filename)
            result = subprocess.run([os.environ.get("TT_DART", shutil.which("dart") or str(Path.home() / ".local/opt/flutter/bin/dart")), "format", "--language-version=3.3", "--output=show"], input=result, text=True, capture_output=True, check=True,
                                    env=child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'))).stdout
            result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
            if args.check:
                if not output.exists() or output.read_text() != result:
                    print("generated Dart types differ: " + filename, file=sys.stderr)
                    return 1
            else:
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text(result)
        return 0
    if args.target == "cpp":
        if args.bridge:
            parser.error("C++ reads the installed native header")
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import cpp
        definitions = prepare(graph(json.loads(args.schema.read_text()), cpp.ROOTS))
        version = json.loads((ROOT / "specification/request.schema.json").read_text())["$defs"]["RequestVersion"]["oneOf"][0]["const"]
        if args.inputs:
            from csharp import enum_values
            schema = json.loads((ROOT / "specification/request.schema.json").read_text())
            schema["$defs"]["Request"] = {key: value for key, value in schema.items() if key != "$defs"}
            abi_spec = importlib.util.spec_from_file_location('cpp_native_abi', ROOT / 'sdlc/scripts/check-c-exports.py')
            abi = importlib.util.module_from_spec(abi_spec)
            abi_spec.loader.exec_module(abi)
            functions = abi.declarations(ROOT / 'libraries/c/include/thinkthen.h')['functions']
            outputs = {
                'inputs_generated.hpp': cpp.input_render(prepare(graph(schema, ("Request", "RequestSessionDescriptor", "EngineSettings")))),
                'calls_generated.hpp': cpp.named_calls(enum_values(schema['$defs']['RequestFunction']), functions),
            }
        else:
            outputs = {'results_generated.hpp': cpp.render(definitions, version) + cpp.usage((ROOT / "libraries/c/include/thinkthen.h").read_text()).replace("#pragma once\n", "")}
        for filename, result in outputs.items():
            output = output_path(ROOT, 'cpp', filename)
            result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
            if args.check:
                if not output.exists() or output.read_text() != result:
                    print(f"generated C++ declarations differ: {output}", file=sys.stderr)
                    return 1
            else:
                output.write_text(result)
        return 0
    if args.target == "php":
        if args.bridge:
            parser.error("PHP reads the generated C header directly")
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import php
        definitions = prepare(graph(immediate_schema(json.loads(args.schema.read_text())), php.ROOTS))
        outputs = php.outputs(definitions, (ROOT / "libraries/c/include/thinkthen.h").read_text())
        schema = json.loads((ROOT / "specification/request.schema.json").read_text())
        schema['$defs']['Request'] = {key: value for key, value in schema.items() if key != '$defs'}
        inputs = prepare(graph(schema, php.INPUT_ROOTS))
        declarations = {'inputs_generated.php': php.inputs(inputs)}
        outputs = declarations if args.inputs else {**outputs, **declarations}
        for filename, result in outputs.items():
            output = output_path(ROOT, 'php', filename)
            result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
            if args.check:
                if not output.exists() or output.read_text() != result:
                    print("generated PHP results differ", file=sys.stderr)
                    return 1
            else:
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text(result)
        return 0
    if args.target == "go":
        if args.inputs or args.bridge:
            parser.error("Go target generates owned results only")
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import go
        definitions = prepare(graph(json.loads(args.schema.read_text()), go.ROOTS))
        version = json.loads((ROOT / "specification/request.schema.json").read_text())["$defs"]["RequestVersion"]["oneOf"][0]["const"]
        request_schema = json.loads((ROOT / "specification/request.schema.json").read_text())
        request_schema['$defs']['Request'] = {key: value for key, value in request_schema.items() if key != '$defs'}
        input_definitions = prepare(graph(request_schema, ('Request', 'RequestSessionDescriptor', 'EngineSettings')))
        result = subprocess.run(["gofmt"], input=go.render(definitions, input_definitions) + "\nconst OwnedRequestVersion = " + json.dumps(version) + "\n" + go.usage((ROOT / "libraries/c/include/thinkthen.h").read_text()).replace("package thinkthen\n", ""), text=True, capture_output=True, check=True,
                                env=child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'))).stdout
        output = output_path(ROOT, 'go', 'owned_results_generated.go')
        result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
        if args.check:
            if not output.exists() or output.read_text() != result:
                print("generated Go results differ", file=sys.stderr)
                return 1
        else:
            output.write_text(result)
        return 0
    if args.target == "typescript":
        if args.inputs or args.bridge:
            parser.error("TypeScript target generates owned results only")
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import typescript
        definitions = prepare(graph(json.loads(args.schema.read_text()), typescript.ROOTS))
        version = json.loads((ROOT / "specification/request.schema.json").read_text())["$defs"]["RequestVersion"]["oneOf"][0]["const"]
        request_schema = json.loads((ROOT / 'specification/request.schema.json').read_text())
        input_roots = getattr(typescript, 'INPUT_ROOTS', ())
        input_definitions = prepare(graph(request_schema, input_roots))
        runtime = (typescript.runtime(definitions, version, input_definitions) if input_roots
                   else typescript.runtime(definitions, version))
        declarations = (typescript.declarations(definitions, input_definitions) if input_roots
                        else typescript.declarations(definitions))
        outputs = {output_path(ROOT, 'typescript', 'results_generated.js'): runtime,
                   output_path(ROOT, 'typescript', 'results_generated.d.ts'): declarations}
        for output, result in outputs.items():
            result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
            if args.check:
                if not output.exists() or output.read_text() != result:
                    print("generated TypeScript results differ", file=sys.stderr)
                    return 1
            else:
                output.write_text(result)
        return 0
    if args.target == "ruby":
        if args.inputs or args.bridge:
            parser.error("Ruby target generates owned results only")
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import ruby
        definitions = prepare(graph(json.loads(args.schema.read_text()), ruby.ROOTS))
        version = json.loads((ROOT / "specification/request.schema.json").read_text())["$defs"]["RequestVersion"]["oneOf"][0]["const"]
        outputs = {output_path(ROOT, 'ruby', 'results_generated.rb'): ruby.render(definitions) + "module ThinkThen\n  class Client\n    REQUEST_VERSION = " + repr(version) + ".freeze\n  end\nend\n",
                   output_path(ROOT, 'ruby', 'results_generated.rs'): ruby.rust(definitions),
                   output_path(ROOT, 'ruby', 'results_generated.rbs'): ruby.rbs(definitions)}
        for output, result in outputs.items():
            result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
            if args.check:
                if not output.exists() or output.read_text() != result:
                    print("generated Ruby results differ", file=sys.stderr)
                    return 1
            else:
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text(result)
        return 0
    if args.target == "jvm":
        if args.bridge:
            parser.error("JVM uses the shared stable native session ABI")
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import jvm
        import jvm_usage
        for filename, lines in jvm_usage.outputs(ROOT).items():
            output = output_path(ROOT, 'jvm', filename)
            source = "\n".join(lines) + "\n"
            if args.check and (not output.exists() or output.read_text() != source):
                print("generated JVM usage status differs", file=sys.stderr)
                return 1
            if not args.check:
                output.write_text(source)
        request_schema = json.loads((ROOT / "specification/request.schema.json").read_text())
        request_schema["$defs"]["Request"] = {key: value for key, value in request_schema.items() if key != "$defs"}
        input_definitions = prepare(graph(request_schema, jvm.INPUT_ROOTS))
        selected, pending = {}, list(jvm.INPUT_ROOTS)
        while pending:
            key = pending.pop()
            if key in selected:
                continue
            source = selected[key] = input_definitions[key]
            pending.extend(references(source))
            pending.extend(child for child, _ in source.get("variants", []))
        input_source = format_source("Inputs.java", jvm.inputs(selected), child_env())
        input_output = output_path(ROOT, 'jvm', 'Inputs.java')
        if args.check and (not input_output.exists() or input_output.read_text() != input_source):
            print("generated JVM inputs differ", file=sys.stderr)
            return 1
        if not args.check:
            input_output.write_text(input_source)
        definitions = prepare(graph(json.loads(args.schema.read_text()), jvm.ROOTS))
        result = format_source("Results.java", jvm.render(definitions), child_env())
        if args.output == OUTPUT:
            args.output = output_path(ROOT, 'jvm', 'Results.java')
        version = json.loads((ROOT / "specification/request.schema.json").read_text())["$defs"]["RequestVersion"]["oneOf"][0]["const"]
        version_source = ('// Generated from request.schema.json. Do not edit.\npackage thinkthen;\n'
                          'final class RequestVersion { static final String VALUE = ' + json.dumps(version) + '; private RequestVersion() {} }\n')
        version_output = output_path(ROOT, 'jvm', 'RequestVersion.java')
        if args.check and (not version_output.exists() or version_output.read_text() != version_source):
            print("generated JVM request version differs", file=sys.stderr)
            return 1
        if not args.check:
            version_output.parent.mkdir(parents=True, exist_ok=True)
            version_output.write_text(version_source)
        if args.check:
            if not args.output.exists() or args.output.read_text() != result:
                print("generated JVM results differ", file=sys.stderr)
                return 1
        else:
            args.output.parent.mkdir(parents=True, exist_ok=True)
            args.output.write_text(result)
        return 0
    if args.target == "zig":
        if args.bridge:
            parser.error("Zig reads the generated C header directly")
        sys.path.insert(0, str(Path(__file__).parent / 'templates'))
        import zig
        schema = json.loads((ROOT / 'specification/request.schema.json').read_text()) if args.inputs else json.loads(args.schema.read_text())
        if args.inputs:
            schema['$defs']['Request'] = {key: value for key, value in schema.items() if key != '$defs'}
            definitions = prepare(graph(schema, ('Request', 'RequestSessionDescriptor', 'EngineSettings')))
            outputs = {'request_generated.zig': zig.render(definitions), 'calls_generated.zig': zig.calls(definitions)}
        else:
            outputs = {'plan_generated.zig': zig.render(prepare(graph(schema, ('completeplan',)))),
                       'results_generated.zig': zig.render_results(prepare(graph(immediate_schema(schema), ('completesessionPacket', 'completeImmediateFacts'))))}
        primary = 'request_generated.zig' if args.inputs else 'plan_generated.zig'
        for filename, result in outputs.items():
            output = args.output if filename == primary and args.output != OUTPUT else output_path(ROOT, 'zig', filename)
            result = subprocess.run(['zig', 'fmt', '--stdin'], input=result,
                                    text=True, capture_output=True, check=True,
                                    env=child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'))).stdout
            if args.check:
                if not output.exists() or output.read_text() != result:
                    print('generated Zig source differs: ' + filename, file=sys.stderr)
                    return 1
            else:
                output.write_text(result)
        return 0
    if args.target in ("python", "r") and args.inputs:
        path = Path(__file__).parent / ("templates/" + args.target + ".py")
        spec = importlib.util.spec_from_file_location("host_inputs", path)
        target = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(target)
        schema = json.loads((ROOT / "specification/request.schema.json").read_text())
        filename = "_named_generated.py" if args.target == "python" else "inputs-generated.R"
        output = output_path(ROOT, args.target, filename)
        result = target.named_calls(schema) if args.target == "python" else target.inputs(schema)
        result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
        if args.check:
            if not output.exists() or output.read_text() != result:
                print("generated " + args.target + " inputs differ", file=sys.stderr)
                return 1
        else:
            output.write_text(result)
        return 0
    if args.target == "python":
        if args.inputs or args.bridge:
            parser.error("Python target generates native results only")
        if args.output == OUTPUT:
            args.output = output_path(ROOT, 'python', 'results_generated.rs')
        path = Path(__file__).parent / "templates/python.py"
        spec = importlib.util.spec_from_file_location("result_python", path)
        target = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(target)
        definitions = prepare(graph(json.loads(args.schema.read_text()), target.ROOTS))
        for extension in ("py", "pyi"):
            output = output_path(ROOT, 'python', '_native_results.' + extension)
            result = target.types(definitions, stubs=extension == "pyi")
            result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
            if args.check:
                if not output.exists() or output.read_text() != result:
                    print("generated Python types differ", file=sys.stderr)
                    return 1
            else:
                output.write_text(result)
    if args.target == "r":
        if args.inputs or args.bridge:
            parser.error("R target generates native result conversions only")
        if args.output == OUTPUT:
            args.output = output_path(ROOT, 'r', 'results_generated.rs')
    if args.target == "c" and args.inputs:
        sys.path.insert(0, str(Path(__file__).parent / 'templates'))
        import native_request
        schema = json.loads((ROOT / 'specification/request.schema.json').read_text())
        result = subprocess.run(['rustfmt', '--edition', '2024', '--emit', 'stdout'],
            input=native_request.render(prepare(graph(schema, ('RequestCall', 'RequestSessionDescriptor', 'RequestReaderFailure', 'EngineSettings')))),
            text=True, capture_output=True, check=True,
            env=child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'))).stdout
        output = output_path(ROOT, 'c', 'carriers_generated.rs')
        result = format_source(output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
        if args.check:
            if not output.exists() or output.read_text() != result:
                print('generated native request carriers differ', file=sys.stderr)
                return 1
        else:
            output.write_text(result)
        return 0
    if args.target == "c":
        if args.inputs or args.bridge:
            parser.error("C target generates result views only")
        if args.output == OUTPUT:
            args.output = output_path(ROOT, 'c', 'views_generated.rs')
    if args.bridge:
        sys.path.insert(0, str(Path(__file__).parent / 'templates'))
        import csharp
        request_schema = json.loads((ROOT / 'specification/request.schema.json').read_text())
        calls_path = output_path(ROOT, 'csharp', 'NamedCalls.g.cs')
        calls = format_source(calls_path, csharp.named_calls(csharp.enum_values(request_schema['$defs']['RequestFunction'])) + csharp.answer_helpers(prepare(graph(json.loads(args.schema.read_text()), ('completesessionPacket', 'completeplan')))), child_env())
        if args.check and (not calls_path.exists() or calls_path.read_text() != calls):
            print('generated C# named calls differ', file=sys.stderr)
            return 1
        if not args.check:
            calls_path.write_text(calls)
        result = bridge()
        args.output = output_path(ROOT, 'csharp', 'NativeBridge.g.cs')
    elif args.inputs and args.target == 'zig':
        schema = json.loads((ROOT / 'specification/request.schema.json').read_text())
        schema['$defs']['Request'] = {k: v for k, v in schema.items() if k != '$defs'}
        result = zig_generated(prepare(graph(schema, ('Request', 'RequestSessionDescriptor'))))
    elif args.inputs:
        path = Path(__file__).parent / 'templates/csharp.py'
        spec = importlib.util.spec_from_file_location('result_csharp', path)
        target = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(target)
        result = target.render_inputs(json.loads((ROOT / 'specification/request.schema.json').read_text()), prepare, graph)
        args.output = output_path(ROOT, 'csharp', 'RequestInputs.g.cs')
    else:
        result = generated(json.loads(args.schema.read_text()), args.target)
    result = format_source(args.output, result, child_env(keep=(*CARGO, "LANG", "LC_ALL", "TMPDIR")))
    if args.check:
        if not args.output.exists() or args.output.read_text() != result:
            print('generated results differ; run sdlc/generators/results/generate.py',
                  file=sys.stderr)
            return 1
    else:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(result)
    if args.check and not args.inputs and not args.bridge and args.schema == SCHEMA and args.output == OUTPUT:
        path = Path(__file__).parent / 'templates/csharp.py'
        spec = importlib.util.spec_from_file_location('request_csharp', path)
        target = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(target)
        inputs = format_source('RequestInputs.g.cs', target.render_inputs(json.loads((ROOT / 'specification/request.schema.json').read_text()), prepare, graph), child_env())
        output = output_path(ROOT, 'csharp', 'RequestInputs.g.cs')
        if not output.exists() or output.read_text() != inputs:
            print('generated C# inputs differ; run sdlc/generators/results/generate.py --inputs', file=sys.stderr)
            return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
