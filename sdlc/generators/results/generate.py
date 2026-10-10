#!/usr/bin/env python3
"""Generate result declarations from the existing Rust-derived result schema."""
import argparse
import os
import importlib.util
import json
from pathlib import Path
import sys
import subprocess

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import CARGO, child_env
SCHEMA = ROOT / 'crates/thinkthen/src/public/results/complete.schema.json'
OUTPUT = Path(__file__).parent / 'csharp/CompleteFacts.g.cs'


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
    return {'primitive_variants': list(selected), 'primitive_schemas': selected,
            'nullable_json': 'null' in selected}


def lift_inline(definitions):
    """Name nested object alternatives from their native schema position."""
    definitions = dict(definitions)
    pending = list(definitions)
    seen = {}
    def visit(value, path):
        if isinstance(value, list):
            return [visit(child, path + '_' + str(index)) for index, child in enumerate(value)]
        if not isinstance(value, dict):
            return value
        if object_variants(value, definitions):
            identity = json.dumps(value, sort_keys=True)
            if identity not in seen:
                key = path
                if key in definitions:
                    raise ValueError('colliding nested result alternative name')
                seen[identity] = key
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
        names = [key + '_' + (value if mode == 'literal' else
                             '_'.join(value.values()) if mode == 'literals' else
                             'fields_' + '_'.join(value['required']) if mode == 'structure' else member)
                 for mode, member, value in tags]
        if len(set(names)) != len(names) or any(name in definitions for name in names):
            raise ValueError('colliding generated object variant names')
        result[key] = {'variants': list(zip(names, tags))}
        result.update(zip(names, variants))
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
        definitions = prepare(graph(schema, ("completesessionPacket",)))
        selected, pending = {}, ["completesessionPacket"]
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
    path = Path(__file__).parent / 'templates/csharp.py'
    spec = importlib.util.spec_from_file_location('result_csharp', path)
    target = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(target)
    roots = ('completeFacts', 'completeanswer', 'completefindAnswer',
             'completeCallError', 'completeRelationMember', 'completeObservation',
             'completeUsage', 'completeReadableQuestion', 'completesourceRelationEndpoint',
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
    fields = native['records']['thinkthen_string_v1']['fields']
    types = {'const char *': 'IntPtr', 'size_t': 'nuint'}
    lines = ['// Generated from the compiler-derived C header ABI; do not edit.',
             'using System.Runtime.InteropServices;', 'namespace ThinkThen;',
             '[StructLayout(LayoutKind.Sequential)]', 'internal struct StringV1 {']
    lines += [' public ' + types[field['type']] + ' ' + name + ';' for name, field in fields.items()]
    lines += ['}']
    for enum, prefix, suffix, underlying in [('AuthoredQuestionKind', 'THINKTHEN_LOAD_', '_V1', 'uint'), ('FailureKind', 'THINKTHEN_E', '', 'int')]:
        members = [(name.removeprefix(prefix).removesuffix(suffix), value) for name, value in native['constants'].items()
                   if name.startswith(prefix) and (name.endswith(suffix) if suffix else not name.endswith('_V1'))]
        lines += ['public enum ' + enum + ' : ' + underlying + ' { ' + ', '.join(''.join(word.title() for word in name.split('_')) + ' = ' + str(value) for name, value in members) + ' }']
    return '\n'.join(lines) + '\n'

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--schema', type=Path, default=SCHEMA)
    parser.add_argument('--output', type=Path, default=OUTPUT)
    parser.add_argument('--inputs', action='store_true')
    parser.add_argument('--target', choices=('csharp', 'c', 'r', 'zig', 'python', 'jvm', 'ruby', 'typescript', 'go', 'php', 'cpp', 'dart'), default='csharp')
    parser.add_argument('--bridge', action='store_true')
    args = parser.parse_args()
    if args.target == "dart":
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import dart
        schema = json.loads((ROOT / "specification/request.schema.json").read_text()) if args.inputs else json.loads(args.schema.read_text())
        result = dart.render(prepare(graph(schema, dart.INPUT_ROOTS if args.inputs else dart.ROOTS)), args.inputs)
        if args.bridge:
            path = ROOT / "sdlc/scripts/check-c-exports.py"
            spec = importlib.util.spec_from_file_location("dart_abi", path)
            abi = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(abi)
            result = dart.bridge(abi.header_abi(ROOT / "libraries/c/include/thinkthen.h"))
        if args.inputs:
            result += "const requestVersion = " + json.dumps(schema["$defs"]["RequestVersion"]["oneOf"][0]["const"]) + ";\n"
        output = ROOT / "libraries/dart/lib/src/session" / ("abi_generated.dart" if args.bridge else "inputs_generated.dart" if args.inputs else "results_generated.dart")
        result = subprocess.run([os.environ.get("TT_DART", "dart"), "format", "--language-version=3.3", "--output=show"], input=result, text=True, capture_output=True, check=True).stdout
        if args.check:
            if not output.exists() or output.read_text() != result:
                print("generated Dart types differ", file=sys.stderr)
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
            schema = json.loads((ROOT / "specification/request.schema.json").read_text())
            result = cpp.input_render(prepare(graph(schema, ("RequestQuestion", "RequestInput", "RequestOptions", "RequestSessionDescriptor"))))
            output = ROOT / "libraries/cpp/include/thinkthen/inputs_generated.hpp"
        else:
            result = cpp.render(definitions, version)
            output = ROOT / "libraries/cpp/include/thinkthen/results_generated.hpp"
        if args.check:
            if not output.exists() or output.read_text() != result:
                print("generated C++ results differ", file=sys.stderr)
                return 1
        else:
            output.write_text(result)
        return 0
    if args.target == "php":
        if args.inputs or args.bridge:
            parser.error("PHP target generates owned results only")
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import php
        definitions = prepare(graph(json.loads(args.schema.read_text()), php.ROOTS))
        version = json.loads((ROOT / "specification/request.schema.json").read_text())["$defs"]["RequestVersion"]["oneOf"][0]["const"]
        outputs = php.outputs(definitions, version, (ROOT / "libraries/c/include/thinkthen.h").read_text())
        for filename, result in outputs.items():
            output = ROOT / "libraries/php/src/session" / filename
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
        result = subprocess.run(["gofmt"], input=go.render(definitions) + "\nconst OwnedRequestVersion = " + json.dumps(version) + "\n", text=True, capture_output=True, check=True,
                                env=child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'))).stdout
        output = ROOT / "libraries/go/owned_results_generated.go"
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
        outputs = {ROOT / "libraries/typescript/results_generated.js": typescript.runtime(definitions, version),
                   ROOT / "libraries/typescript/results_generated.d.ts": typescript.declarations(definitions)}
        for output, result in outputs.items():
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
        outputs = {ROOT / "libraries/ruby/lib/thinkthen/results_generated.rb": ruby.render(definitions) + "module ThinkThen\n  class Client\n    REQUEST_VERSION = " + repr(version) + ".freeze\n  end\nend\n",
                   ROOT / "libraries/ruby/src/ffi/results_generated.rs": ruby.rust(definitions)}
        for output, result in outputs.items():
            if args.check:
                if not output.exists() or output.read_text() != result:
                    print("generated Ruby results differ", file=sys.stderr)
                    return 1
            else:
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text(result)
        return 0
    if args.target == "jvm":
        if args.inputs or args.bridge:
            parser.error("JVM target generates owned results only")
        sys.path.insert(0, str(Path(__file__).parent / "templates"))
        import jvm
        result = jvm.render(prepare(graph(json.loads(args.schema.read_text()), jvm.ROOTS)))
        if args.output == OUTPUT:
            args.output = ROOT / "libraries/jvm/session/thinkthen/Results.java"
        version = json.loads((ROOT / "specification/request.schema.json").read_text())["$defs"]["RequestVersion"]["oneOf"][0]["const"]
        version_source = ('// Generated from request.schema.json. Do not edit.\npackage thinkthen;\n'
                          'final class RequestVersion { static final String VALUE = ' + json.dumps(version) + '; private RequestVersion() {} }\n')
        version_output = ROOT / "libraries/jvm/session/thinkthen/RequestVersion.java"
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
        if args.output == OUTPUT:
            filename = 'request_generated.zig' if args.inputs else 'plan_generated.zig'
            args.output = ROOT / 'libraries/zig/src' / filename
    if args.target == "python":
        if args.inputs or args.bridge:
            parser.error("Python target generates native results only")
        if args.output == OUTPUT:
            args.output = ROOT / "libraries/python/src/results_generated.rs"
        path = Path(__file__).parent / "templates/python.py"
        spec = importlib.util.spec_from_file_location("result_python", path)
        target = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(target)
        definitions = prepare(graph(json.loads(args.schema.read_text()), target.ROOTS))
        for extension in ("py", "pyi"):
            output = ROOT / "libraries/python/thinkthen" / ("_native_results." + extension)
            result = target.types(definitions, stubs=extension == "pyi")
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
            args.output = ROOT / "libraries/r/thinkthen/src/rust/src/results_generated.rs"
    if args.target == "c":
        if args.inputs or args.bridge:
            parser.error("C target generates result views only")
        if args.output == OUTPUT:
            args.output = ROOT / "libraries/c/src/session/views_generated.rs"
    if args.bridge:
        result = bridge()
        args.output = ROOT / 'libraries/csharp/src/NativeBridge.g.cs'
    elif args.inputs and args.target == 'zig':
        schema = json.loads((ROOT / 'specification/request.schema.json').read_text())
        schema['$defs']['Request'] = {k: v for k, v in schema.items() if k != '$defs'}
        result = zig_generated(prepare(graph(schema, ('Request', 'RequestSessionDescriptor'))))
    elif args.inputs:
        path = Path(__file__).parent / 'templates/csharp.py'
        spec = importlib.util.spec_from_file_location('result_csharp', path)
        target = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(target)
        result = target.render_inputs(json.loads((ROOT / 'specification/request.schema.json').read_text()))
        args.output = ROOT / 'libraries/csharp/src/RequestInputs.g.cs'
    else:
        result = generated(json.loads(args.schema.read_text()), args.target)
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
        inputs = target.render_inputs(json.loads((ROOT / 'specification/request.schema.json').read_text()))
        output = ROOT / 'libraries/csharp/src/RequestInputs.g.cs'
        if not output.exists() or output.read_text() != inputs:
            print('generated C# inputs differ; run sdlc/generators/results/generate.py --inputs', file=sys.stderr)
            return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
