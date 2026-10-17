"""Java owned views over the shared prepared Rust result graph."""
import json
import re
from csharp import shape, nullable, enum_values
from host_answers import named_object_fields
from jvm_host import answers

ROOTS = ('completesessionPacket', 'completeplan')


def name(key):
    key = key.removeprefix('complete')
    return ''.join(part[0].upper() + part[1:] for part in re.findall(
        r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+', key))


def member(key):
    value = name(key)
    value = value[0].lower() + value[1:]
    return value + 'Value' if value in ('default', 'class', 'interface', 'package', 'private', 'public', 'null', 'true', 'false') else value


def conversion(source, definitions, expression='value', depth=0):
    source = shape(source)
    if '$ref' in source:
        key = source['$ref'].removeprefix('#/$defs/')
        return name(key), f'{name(key)}.read({expression})'
    if source == {} or source is True or source.get('type') == 'null':
        return 'Object', f'Values.freeze({expression})'
    if enum_values(source) or isinstance(source.get('const'), str):
        return 'String', f'(String){expression}'
    kind = source.get('type')
    if kind == 'array':
        var = f'item{depth}'
        typ, decode = conversion(source['items'], definitions, var, depth + 1)
        return f'List<{typ}>', f'Values.list({expression}, {var} -> {decode})'
    if kind == 'object':
        if isinstance(source.get('additionalProperties'), dict):
            var = f'item{depth}'
            typ, decode = conversion(source['additionalProperties'], definitions, var, depth + 1)
            return f'Map<String,{typ}>', f'Values.map({expression}, {var} -> {decode})'
        return 'Map<String,Object>', f'Values.object({expression})'
    if kind in ('integer', 'number', 'boolean', 'string'):
        typ = {'integer': 'BigInteger', 'number': 'BigDecimal', 'boolean': 'Boolean', 'string': 'String'}[kind]
        return typ, (f'Values.integer({expression})' if kind == 'integer' else f'({typ}){expression}')
    if 'primitive_variants' in source:
        return 'Object', f'Values.freeze({expression})'
    raise ValueError(f'Java conversion needs a typed shape: {source}')


def condition(tag):
    mode, field, value = tag
    if mode == 'literal':
        return f'{json.dumps(value)}.equals(object.get({json.dumps(field)}))'
    if mode == 'literals':
        return ' && '.join(f'{json.dumps(v)}.equals(object.get({json.dumps(k)}))' for k, v in value.items())
    if mode == 'member':
        return f'object.containsKey({json.dumps(field)})'
    return ' && '.join([f'object.containsKey({json.dumps(k)})' for k in value['required']] +
                       [f'!object.containsKey({json.dumps(k)})' for k in value['excluded']])


def render(definitions):
    definitions = dict(definitions)
    facts = dict(definitions['completeFacts'])
    facts['required'] = [field for field in facts['required'] if field != 'call_id']
    definitions['completeImmediateFacts'] = facts
    lines = ['// Generated from the Rust result graph. Do not edit.',
             'package thinkthen;', 'import java.util.*;', 'import java.math.*;',
             'public final class Results {', 'private Results() {}', 'public static class View extends Values.View { protected View(Object value) { super(value); } }']
    trees = []
    parents = {child: name(key) for key, source in definitions.items() for child, _ in source.get('variants', [])}
    for key, raw in definitions.items():
        typ = name(key)
        source = shape(raw)
        if 'variants' in source:
            lines += [f'public sealed interface {typ} permits ' + ','.join(name(child) for child, _ in source['variants']) + ' {',
                      f'static {typ} read(Object value) {{ Map<String,Object> object = Values.object(value);']
            for child, tag in source['variants']:
                lines.append(f'if ({condition(tag)}) return {name(child)}.read(value);')
            lines += ['throw new IllegalStateException("Unknown native result alternative");', '}', '}']
            continue
        if 'primitive_variants' in source:
            alternatives = source['primitive_variants']
            lines += [f'public sealed interface {typ} permits ' + ','.join(typ + name(kind) for kind in alternatives) + ' {', f'static {typ} read(Object value) {{']
            tests = {'boolean': 'Boolean', 'string': 'String', 'number': 'BigDecimal', 'integer': 'BigDecimal', 'array': 'List<?>', 'object': 'Map<?,?>'}
            for kind in alternatives:
                if kind == 'null':
                    lines.append(f'if (value == null) return new {typ}Null(null);')
                else:
                    lines.append(f'if (value instanceof {tests[kind]}) return {typ}{name(kind)}.read(value);')
            lines += ['throw new IllegalStateException("Unknown native value alternative");', '}', '}']
            for kind in alternatives:
                trees.append(f'if (value instanceof {typ}{name(kind)} item) return Values.json(item.value());')
                target, decode = conversion(source.get('primitive_schemas', {}).get(kind, {'type': kind}), definitions)
                lines += [f'public record {typ}{name(kind)}({target} value) implements {typ} {{', f'static {typ}{name(kind)} read(Object value) {{ return new {typ}{name(kind)}({decode}); }}', 'Object json() { return Values.json(value); }', '}']
            continue
        parent = f' implements {parents[key]}' if key in parents else ''
        lines += [f'public static final class {typ} extends View{parent} {{', f'public {typ}(Object value) {{ super(value); }}', f'static {typ} read(Object value) {{ return new {typ}(value); }}']
        if 'properties' in source:
            for field, spec in source['properties'].items():
                target, decode = conversion(spec, definitions, 'value')
                quoted = json.dumps(field)
                if field not in source.get('required', []) or nullable(spec, definitions):
                    required = f'required({quoted}); ' if field in source.get('required', []) else ''
                    lines.append(f'public Presence<{target}> {member(field)}() {{ {required}return presence({quoted}, value -> {decode}); }}')
                else:
                    lines.append(f'public {target} {member(field)}() {{ Object value = required({quoted}); return {decode}; }}')
        else:
            target, decode = conversion(source, definitions, 'json()')
            lines.append(f'public {target} value() {{ return {decode}; }}')
        for field in named_object_fields(source, definitions):
            lines.append(f'public String {member(field)}Name() {{ return (String)Values.object(required({json.dumps(field)})).get("name"); }}')
        lines.append('}')
    lines += ['static Object tree(Object value) {', *trees, 'return value;', '}']
    lines.append(answers(definitions))
    return '\n'.join(lines + ['}']) + '\n'

INPUT_ROOTS = ('Request', 'EngineSettings', 'RequestSessionDescriptor', 'RequestReaderFailure')


def inputs(definitions):
    """Mechanical schema builders; admission, defaults and bounds remain native."""
    definitions = dict(definitions)
    # Ordinary inline objects also need named host types, including recognition.
    def lift(value, path):
        if isinstance(value, list):
            return [lift(child, path + '_' + str(index)) for index, child in enumerate(value)]
        if not isinstance(value, dict):
            return value
        if (value.get('type') == 'object' and 'properties' in value) or 'anyOf' in value or 'oneOf' in value:
            key = path
            definitions[key] = {key: lift(child, path + '_' + key) for key, child in value.items()}
            return {'$ref': '#/$defs/' + key}
        return {key: lift(child, path + '_' + key) for key, child in value.items()}
    for key, source in list(definitions.items()):
        definitions[key] = {field: lift(value, key + '_' + field) for field, value in source.items()}
    parents = {}
    for key, source in definitions.items():
        for child, _ in source.get('variants', []):
            parents.setdefault(child, []).append(name(key))
    def target(source):
        if source is True or source == {}:
            return 'Object'
        if 'const' in source:
            value = source['const']
            return 'String' if isinstance(value, str) else 'Boolean' if isinstance(value, bool) else 'Number' if isinstance(value, (int, float)) else 'Object'
        if '$ref' in source:
            key = source['$ref'].removeprefix('#/$defs/')
            source = definitions[key]
            if 'properties' in source or 'variants' in source or enum_values(source) or 'anyOf' in source or 'oneOf' in source or 'primitive_variants' in source or isinstance(source.get('type'), list):
                return name(key)
            return target(source)
        if source.get('type') == 'array':
            return 'List<? extends ' + target(source.get('items', {})) + '>'
        if source.get('type') == 'object':
            value_type = target(source.get('additionalProperties', {}))
            return ('Map<String,? extends ' if value_type == 'Object' else 'Ordered<? extends ') + value_type + '>'
        if isinstance(source.get('type'), list):
            kinds = [kind for kind in source['type'] if kind != 'null']
            if len(kinds) == 1:
                return target({**source, 'type': kinds[0]})
        if 'anyOf' in source or 'oneOf' in source or isinstance(source.get('type'), list) or 'primitive_variants' in source:
            raise ValueError(f'JVM input union needs a named schema shape: {source}')
        return {'string':'String', 'integer':'Number', 'number':'Number', 'boolean':'Boolean', 'null':'Object'}.get(source.get('type'), 'Object')
    lines = ['// Generated from the shared Rust Request schema. Do not edit.',
             'package thinkthen;', 'import java.util.*;',
             '/** Typed wire builders. Native code owns grammar and admission errors. */',
             'public final class Inputs {', 'private Inputs() {}',
             '/** Authored entries retain the order of calls; duplicate names are rejected. */',
             'public static final class Ordered<T> {',
             'private final Map<String,T> values = new LinkedHashMap<>();',
             'public Ordered<T> entry(String name, T value) { if (values.containsKey(name)) throw new IllegalArgumentException("Duplicate authored name: " + name); values.put(name, value); return this; }',
             'Object json() { return values; }', '}', 'public abstract static class Builder<T extends Builder<T>> extends Values.Builder<T> {}']
    trees = []
    readers = []
    for key, source in sorted(definitions.items()):
        typ = name(key)
        if 'variants' in source:
            lines += [f'public sealed interface {typ} permits ' + ','.join(name(child) for child, _ in source['variants']) + ' {']
            alternatives = [definitions[child].get('properties', {}) for child, _ in source['variants']]
            for field, spec in alternatives[0].items():
                if isinstance(spec, dict) and 'const' not in spec and all(fields.get(field) == spec for fields in alternatives):
                    lines.append(f'{typ} {member(field)}({target(spec)} value);')
            lines.append('}')
            selected_reader = [f'static {typ} read{typ}(Object value) {{ Map<String,Object> object = Values.object(value);']
            for child, tag in source['variants']:
                selected_reader.append(f'if ({condition(tag)}) return {name(child)}.fromJson(value);')
            # Selection only chooses the host carrier; native admission owns invalid shapes.
            readers += selected_reader + [f'return {name(source["variants"][0][0])}.fromJson(value);', '}']
            continue
        values = enum_values(source)
        if values:
            constants = ','.join(member(value).upper() + '(' + json.dumps(value) + ')' for value in values)
            trees.append(f'if (value instanceof {typ} item) return item.json();')
            lines += [f'public enum {typ} {{ {constants};',
                      f'private final String value; {typ}(String value) {{ this.value = value; }}',
                      'Object json() { return value; }', '}']
            continue
        if 'properties' not in source:
            alternatives = source.get('anyOf', source.get('oneOf', []))
            if 'primitive_variants' in source:
                alternatives = list(source.get('primitive_schemas', {}).values()) or [{'type': kind} for kind in source['primitive_variants']]
            if isinstance(source.get('type'), list):
                alternatives = [{**source, 'type': kind} for kind in source['type']]
            if alternatives:
                types = dict.fromkeys(target(item) for item in alternatives if item.get('type') != 'null')
                trees.append(f'if (value instanceof {typ} item) return item.json();')
                lines += [f'public static final class {typ} {{', 'private final Object value;']
                for value_type in types:
                    lines.append(f'public {typ}({value_type} value) {{ this.value = Values.freeze(value); }}')
                lines += ['Object json() { return value; }', '}']
            continue
        interfaces = ' implements ' + ','.join(parents[key]) if key in parents else ''
        lines += [f'public static final class {typ} extends Builder<{typ}>{interfaces} {{', f'public {typ}() {{']
        for field, spec in source['properties'].items():
            if field in source.get('required', []) and isinstance(spec, dict):
                resolved = definitions[spec['$ref'].removeprefix('#/$defs/')] if '$ref' in spec else spec
                values = enum_values(resolved)
                constant = resolved.get('const', values[0] if values and len(values) == 1 else None)
                if constant is not None:
                    lines.append(f'put({json.dumps(field)}, {json.dumps(constant)});')
        clear = ' '.join(f'result.omit({json.dumps(field)});' for field in source['properties'])
        lines += ['}', f'static {typ} fromJson(Object value) {{ {typ} result = new {typ}(); {clear} Values.object(value).forEach(result::extension); return result; }}', f'protected {typ} self() {{ return this; }}']
        for field, spec in source['properties'].items():
            if isinstance(spec, dict) and 'const' in spec:
                continue
            quoted, method = json.dumps(field), member(field)
            lines += [f'public {typ} {method}({target(spec)} value) {{ return put({quoted}, value); }}',
                      f'public {typ} {method}Null() {{ return put({quoted}, null); }}',
                      f'public {typ} omit{method[0].upper() + method[1:]}() {{ return omit({quoted}); }}']
        lines.append('}')
    lines += readers + ['static Object tree(Object value) {', 'if (value instanceof Ordered<?> item) return item.json();', *trees, 'return value;', '}']
    return '\n'.join(lines + ['}']) + '\n'
