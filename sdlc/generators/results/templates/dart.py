"""Dart host types from the shared prepared Rust graphs; no admission rules."""
import json
import re

ROOTS = ('completesessionPacket',)
PREFIX = ''
INPUT_ROOTS = ('RequestQuestion', 'RequestInput', 'RequestOptions', 'RequestSessionDescriptor', 'EngineSettings')


def name(key):
    key = key.removeprefix('complete')
    return PREFIX + ''.join(part[:1].upper() + part[1:] for part in key.split('_'))


def member(key):
    value = re.sub(r'_([a-z])', lambda m: m[1].upper(), key)
    return value + 'Value' if value in ('null', 'true', 'false') else value


def resolved(source, definitions):
    if '$ref' in source:
        return definitions[source['$ref'].removeprefix('#/$defs/')]
    return source


def scalar(source, definitions):
    source = resolved(source, definitions)
    if 'properties' in source or 'variants' in source or 'primitive_variants' in source:
        return None
    alternatives = source.get('anyOf', source.get('oneOf', [source]))
    kinds = set()
    for item in alternatives:
        item = resolved(item, definitions)
        kind = item.get('type')
        if kind is None and 'const' in item:
            kind = 'string' if isinstance(item['const'], str) else 'boolean' if isinstance(item['const'], bool) else 'integer' if isinstance(item['const'], int) else 'number'
        if kind is None:
            return None
        kinds.update(kind if isinstance(kind, list) else [kind])
    nullable = 'null' in kinds
    kinds.discard('null')
    if len(kinds) != 1:
        return None
    return next(iter(kinds)), nullable


def union(source, definitions):
    return isinstance(source, dict) and not any(k in source for k in ('properties', 'variants', 'primitive_variants')) and len(source.get('anyOf', source.get('oneOf', []))) > 1 and scalar(source, definitions) is None


def hint(source, definitions):
    if not isinstance(source, dict):
        return 'Object?'
    alternatives = source.get('anyOf', source.get('oneOf', []))
    if len(alternatives) == 1:
        return hint(alternatives[0], definitions)
    if len(alternatives) == 2 and any(item.get('type') == 'null' for item in alternatives):
        inner = next(item for item in alternatives if item.get('type') != 'null')
        return hint(inner, definitions).removesuffix('?') + '?'
    if '$ref' in source:
        key = source['$ref'].removeprefix('#/$defs/')
        target = definitions[key]
        if 'properties' in target or 'variants' in target or 'primitive_variants' in target or union(target, definitions):
            return name(key)
        return hint(target, definitions)
    simple = scalar(source, definitions)
    if simple:
        kind, nullable = simple
        if kind == 'array':
            item = source.get('items', {})
            value = f'List<{hint(item, definitions)}>'
        elif kind == 'object':
            value = f'Map<String, {hint(source.get("additionalProperties", {}), definitions)}>'
        else:
            value = {'string': 'String', 'boolean': 'bool', 'number': 'num', 'integer': 'BigInt'}.get(kind, 'Object')
        return value + ('?' if nullable else '')
    return 'Object?'


def read(source, value, definitions):
    if not isinstance(source, dict):
        return value
    alternatives = source.get('anyOf', source.get('oneOf', []))
    if len(alternatives) == 1:
        return read(alternatives[0], value, definitions)
    if len(alternatives) == 2 and any(item.get('type') == 'null' for item in alternatives):
        inner = next(item for item in alternatives if item.get('type') != 'null')
        return f'{value} == null ? null : {read(inner, value, definitions)}'
    if '$ref' in source:
        key = source['$ref'].removeprefix('#/$defs/')
        if any(item in definitions[key] for item in ('properties', 'variants', 'primitive_variants')) or union(definitions[key], definitions):
            return f'{name(key)}.read({value})'
        return read(definitions[key], value, definitions)
    simple = scalar(source, definitions)
    if not simple:
        return value
    kind, nullable = simple
    if kind == 'array':
        expr = f'({value} as List).map((v) => {read(source.get("items", {}), "v", definitions)}).toList(growable: false)'
    elif kind == 'object':
        expr = f'({value} as Map<String, Object?>).map((k, v) => MapEntry(k, {read(source.get("additionalProperties", {}), "v", definitions)}))'
    elif kind == 'integer':
        expr = f'readInteger({value})'
    else:
        expr = f'{value} as {hint(source, definitions).removesuffix("?")}'
    return f'{value} == null ? null : {expr}' if nullable else expr


def condition(tag, variable='map'):
    mode, field, value = tag
    if mode == 'literal':
        return f'{variable}[{json.dumps(field)}] == {json.dumps(value)}'
    if mode == 'literals':
        return ' && '.join(f'{variable}[{json.dumps(k)}] == {json.dumps(v)}' for k, v in value.items())
    if mode == 'member':
        return f'{variable}.containsKey({json.dumps(field)})'
    return ' && '.join([f'{variable}.containsKey({json.dumps(k)})' for k in value['required']] + [f'!{variable}.containsKey({json.dumps(k)})' for k in value['excluded']])


def render(definitions, inputs=False):
    global PREFIX
    PREFIX = 'Input' if inputs else ''
    lines = ['// Generated from the shared Rust graph. Do not edit.', "import 'values.dart';"]
    parents = {child: name(key) for key, source in definitions.items() for child, _ in source.get('variants', [])}
    for key, source in definitions.items():
        cls = name(key)
        if 'variants' in source:
            lines += [f'sealed class {cls} extends NativeObject {{', f'  {cls}(super.json);', f'  factory {cls}.read(Object? value) {{', '    final map = readObject(value);']
            for child, tag in source['variants']:
                lines += [f'    if ({condition(tag)}) return {name(child)}.read(map);']
            lines += [f'    throw FormatException("Unknown {cls} alternative");', '  }', '}']
        elif 'primitive_variants' in source:
            lines += [f'final class {cls} {{', '  final Object? value;', f'  const {cls}(this.value);', f'  factory {cls}.read(Object? value) => value is {cls} ? value : {cls}(value);', '  Object? toJson() => value;']
            for kind, field in source['primitive_schemas'].items():
                if kind != 'null':
                    lines += [f'  {hint(field, definitions)} get as{kind.title()} => {read(field, "value", definitions)};']
            lines += ['}']
        elif union(source, definitions):
            lines += [f'final class {cls} {{', '  final Object? value;', f'  const {cls}._(this.value);', f'  factory {cls}.read(Object? value) => value is {cls} ? value : {cls}._(value);', '  Object? toJson() => value;']
            for index, field in enumerate(source.get('anyOf', source.get('oneOf', []))):
                if field.get('type') == 'null':
                    lines += [f'  const {cls}.nullValue() : value = null;']
                else:
                    lines += [f'  const {cls}.alternative{index}({hint(field, definitions)} this.value);', f'  {hint(field, definitions)} get asAlternative{index} => {read(field, "value", definitions)};']
            lines += ['}']
        elif 'properties' in source:
            parent = parents.get(key, 'NativeObject')
            lines += [f'final class {cls} extends {parent} {{', f'  {cls}._(super.json);', f'  factory {cls}.read(Object? value) => {cls}._(readObject(value));']
            fields = source['properties']
            required = set(source.get('required', []))
            if inputs:
                args, values = [], []
                for prop, field in fields.items():
                    var, typ = member(prop), hint(field, definitions)
                    if isinstance(field, dict) and 'const' in field:
                        values += [f'{json.dumps(prop)}: {json.dumps(field["const"])}']
                    elif prop in required:
                        args += [f'required {typ} {var}']
                        values += [f'{json.dumps(prop)}: {var}']
                    else:
                        args += [f'Presence<{typ}> {var} = const Presence.absent()']
                        values += [f'if ({var}.isPresent) {json.dumps(prop)}: {var}.value']
                signature = '{' + ', '.join(args) + '}' if args else ''
                lines += [f'  {cls}({signature}) : super({{{", ".join(values)}}});']
            for prop, field in fields.items():
                typ, var = hint(field, definitions), member(prop)
                expr = read(field, f'json[{json.dumps(prop)}]', definitions)
                if prop in required:
                    lines += [f'  {typ} get {var} => {expr};']
                else:
                    lines += [f'  Presence<{typ}> get {var} => json.containsKey({json.dumps(prop)}) ? Presence.present({expr}) : const Presence.absent();']
            lines += ['}']
    return '\n'.join(lines) + '\n'


def bridge(abi):
    def types(value):
        carriers = {'thinkthen_complete_usage_persistence_v1 *': 'NativeUsageState', 'thinkthen_complete_utf8_v1 *': 'NativeUsageAdvice'}
        if value in carriers:
            return (f'Pointer<{carriers[value]}>',) * 2
        if value.endswith('**'):
            return ('Pointer<Pointer<Void>>',) * 2
        if value.endswith('*'):
            if value.removeprefix('const ').startswith('char'):
                return ('Pointer<Uint8>',) * 2
            if value.removeprefix('const ').startswith('uint32_t'):
                return ('Pointer<Uint32>',) * 2
            if value.removeprefix('const ').startswith('size_t'):
                return ('Pointer<Size>',) * 2
            return ('Pointer<Void>',) * 2
        return {'int': ('Int32', 'int'), 'size_t': ('Size', 'int'), 'void': ('Void', 'void')}[value]
    symbols = ('thinkthen_engine_new_with', 'thinkthen_engine_free', 'thinkthen_error_code',
               'thinkthen_error_retryable', 'thinkthen_error_message', 'thinkthen_error_facts_json',
               'thinkthen_session_new_with_surface', 'thinkthen_session_try_push',
               'thinkthen_session_try_read', 'thinkthen_session_finish', 'thinkthen_session_cancel',
               'thinkthen_session_free', 'thinkthen_session_result_free', 'thinkthen_session_result_json',
               'thinkthen_session_error_message', 'thinkthen_engine_usage_persistence_v1',
               'thinkthen_engine_finish_usage_status_v1')
    lines = ['// Generated from the canonical C header ABI. Do not edit.', "import 'dart:ffi';"]
    for carrier, host in [('thinkthen_complete_usage_persistence_v1', 'NativeUsageState'), ('thinkthen_complete_utf8_v1', 'NativeUsageAdvice')]:
        lines += [f'final class {host} extends Struct {{']
        for name, field in abi['records'][carrier]['fields'].items():
            annotation, typ = {'uint32_t': ('@Uint32()', 'int'), 'size_t': ('@Size()', 'int'), 'const char *': ('', 'Pointer<Uint8>')}[field['type']]
            lines += ([f'  {annotation}'] if annotation else []) + [f'  external {typ} {name};']
        lines += ['}']
    states = [(key, value) for key, value in abi['constants'].items() if key.startswith('THINKTHEN_COMPLETE_USAGE_PERSISTENCE_')]
    lines += ['enum UsagePersistenceState { ' + ', '.join(key.removeprefix('THINKTHEN_COMPLETE_USAGE_PERSISTENCE_').removesuffix('_V1').lower() + '(' + str(value) + ')' for key, value in states) + '; final int code; const UsagePersistenceState(this.code); }']
    members = []
    for symbol in symbols:
        fn = abi['functions'][symbol]
        ret = types(fn['return'])
        args = [types(value) for value in fn['arguments']]
        native = f'{ret[0]} Function({", ".join(arg[0] for arg in args)})'
        parameters = ', '.join(f'{arg[1]} arg{i}' for i, arg in enumerate(args))
        lines += [f"@Native<{native}>(symbol: '{symbol}', assetId: 'package:thinkthen_dart/thinkthen')",
                  f'external {ret[1]} _{symbol}({parameters});']
        members += [f'  final {symbol} = _{symbol};']
        if symbol in ('thinkthen_engine_free', 'thinkthen_session_free'):
            members += [f'  final {symbol}Pointer = Native.addressOf<NativeFunction<{native}>>(_{symbol});']
    lines += ['final class NativeAbi {', *members, '}']
    errors = [(key, value) for key, value in abi['constants'].items() if key.startswith('THINKTHEN_E') and not key.endswith('_V1')]
    lines += ['enum NativeErrorKind { ' + ', '.join(key.removeprefix('THINKTHEN_E').lower() + '(' + str(value) + ')' for key, value in errors) + '; final int code; const NativeErrorKind(this.code); }']
    for key, value in abi['constants'].items():
        if key.startswith('THINKTHEN_SESSION_') or key.startswith('THINKTHEN_E') and not key.endswith('_V1'):
            lines += [f'const {key} = {value};']
    return '\n'.join(lines) + '\n'
