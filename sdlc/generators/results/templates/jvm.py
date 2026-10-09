"""Java owned views over the shared prepared Rust result graph."""
import json
import re
from csharp import shape, nullable, enum_values

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
    lines = ['// Generated from the Rust result graph. Do not edit.',
             'package thinkthen;', 'import java.util.*;', 'import java.math.*;',
             'public final class Results {', 'private Results() {}']
    parents = {child: name(key) for key, source in definitions.items() for child, _ in source.get('variants', [])}
    for key, raw in definitions.items():
        typ = name(key)
        source = shape(raw)
        if 'variants' in source:
            lines += [f'public sealed interface {typ} extends Values.Value permits ' + ','.join(name(child) for child, _ in source['variants']) + ' {',
                      f'static {typ} read(Object value) {{ Map<String,Object> object = Values.object(value);']
            for child, tag in source['variants']:
                lines.append(f'if ({condition(tag)}) return {name(child)}.read(value);')
            lines += ['throw new IllegalStateException("Unknown native result alternative");', '}', '}']
            continue
        if 'primitive_variants' in source:
            alternatives = source['primitive_variants']
            lines += [f'public sealed interface {typ} extends Values.Value permits ' + ','.join(typ + name(kind) for kind in alternatives) + ' {', f'static {typ} read(Object value) {{']
            tests = {'boolean': 'Boolean', 'string': 'String', 'number': 'BigDecimal', 'integer': 'BigDecimal', 'array': 'List<?>', 'object': 'Map<?,?>'}
            for kind in alternatives:
                if kind == 'null':
                    lines.append(f'if (value == null) return new {typ}Null(null);')
                else:
                    lines.append(f'if (value instanceof {tests[kind]}) return {typ}{name(kind)}.read(value);')
            lines += ['throw new IllegalStateException("Unknown native value alternative");', '}', '}']
            for kind in alternatives:
                target, decode = conversion(source.get('primitive_schemas', {}).get(kind, {'type': kind}), definitions)
                lines += [f'public record {typ}{name(kind)}({target} value) implements {typ} {{', f'static {typ}{name(kind)} read(Object value) {{ return new {typ}{name(kind)}({decode}); }}', 'public Object json() { return Values.json(value); }', '}']
            continue
        parent = f' implements {parents[key]}' if key in parents else ''
        lines += [f'public static final class {typ} extends Values.View{parent} {{', f'public {typ}(Object value) {{ super(value); }}', f'static {typ} read(Object value) {{ return new {typ}(value); }}']
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
        lines.append('}')
    return '\n'.join(lines + ['}']) + '\n'
