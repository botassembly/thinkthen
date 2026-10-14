"""Counted request carriers and Rust conversion from the prepared request graph."""
import json
import re
from native import JSON_KINDS


def name(key):
    key = key.replace('RequestOptions', 'CallOptions', 1) if key.startswith('RequestOptions') else key
    key = key.replace('RequestThreshold', 'CallThreshold', 1) if key.startswith('RequestThreshold') else key
    key = key.replace('RequestQuestion', 'QuestionSelector', 1) if key.startswith('RequestQuestion') else key.removeprefix('Request')
    key = key.removeprefix('Authored_')
    words = re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+', key)
    return 'thinkthen_request_' + '_'.join(word.lower() for word in words) + '_v1'


def union_member(parent, target, siblings=()):
    """Use the canonical alternative title for a public C payload field."""
    member = name(target).removeprefix("thinkthen_request_").removesuffix("_v1")
    prefix = name(parent).removeprefix("thinkthen_request_").removesuffix("_v1")
    short = member.removeprefix(prefix + "_")
    if sum(name(item).removeprefix("thinkthen_request_").removesuffix("_v1").removeprefix(prefix + "_") == short for item in siblings) > 1:
        short = member
    if short in {"unsigned", "signed", "char", "int", "float", "double", "long", "short", "void", "bool", "class", "struct", "union", "enum"}:
        short += "_value"
    return short


def quoted(value):
    return json.dumps(value, ensure_ascii=False)


def carrier_graph(definitions, roots=('RequestArguments', 'RequestSessionDescriptor', 'RequestReaderFailure')):
    """Return canonical carrier nodes and the shared inline type selector.

    The selector adds inline nodes to the returned mapping as callers traverse
    it. Adapters reuse the exact type identities that the Rust generator owns.
    """
    nodes = dict(definitions)
    roots = (*roots, *(('EngineSettings',) if 'EngineSettings' in nodes else ()))
    call = nodes['RequestCall']
    nodes['RequestArguments'] = {**call, 'properties': {key: value for key, value in call['properties'].items() if key != 'function'}, 'required': [key for key in call['required'] if key != 'function']}
    def references(value):
        if isinstance(value, dict):
            if '$ref' in value:
                yield value['$ref'].removeprefix('#/$defs/')
            for child, _ in value.get('variants', []):
                yield child
            for child in value.values():
                yield from references(child)
        elif isinstance(value, (list, tuple)):
            for child in value:
                yield from references(child)
    selected, pending = {}, list(roots)
    while pending:
        key = pending.pop()
        if key not in selected:
            selected[key] = nodes[key]
            pending.extend(references(nodes[key]))
    nodes = selected
    inline = {}
    def select(source, path):
        if isinstance(source, dict) and '$ref' in source:
            return source['$ref'].removeprefix('#/$defs/')
        source = source if isinstance(source, dict) else {}
        alternatives = source.get('anyOf', source.get('oneOf', []))
        if len(alternatives) == 1:
            return select(alternatives[0], path)
        identity = json.dumps(source, sort_keys=True)
        if identity in inline:
            return inline[identity]
        kind = source.get('type')
        key = source.get('title', path)
        if 'const' in source:
            key = 'Literal_' + re.sub(r'[^A-Za-z0-9]+', '_', str(source['const']))
        elif not source:
            key = 'JsonValue'
        elif kind in ('string', 'boolean', 'number', 'null') and not source.get('enum'):
            key = {'string':'Text', 'boolean':'Boolean', 'number':'Number', 'null':'Null'}[kind]
        elif kind == 'integer':
            key = 'Unsigned' if source.get('format', '').startswith('u') else 'Integer'
        elif kind == 'array':
            key = select(source.get('items', {}), path + '_item') + '_list'
        elif kind == 'object' and 'properties' not in source:
            key = select(source.get('additionalProperties', {}), path + '_value') + '_map'
        if key in nodes and nodes[key] != source:
            # Scalar transport constraints are admitted by Rust, not repeated here.
            if kind not in ('string', 'boolean', 'number', 'integer', 'null', 'array') and not (kind == 'object' and 'properties' not in source):
                raise ValueError('conflicting native request value type: ' + key)
        else:
            nodes[key] = source
        inline[identity] = key
        return key
    return nodes, select


def value_tree_reader():
    """Lower the one canonical JSON layout grammar into counted recursive admission."""
    lines = ['unsafe fn read_json_value(pointer: *const thinkthen_complete_json_v1, context: &mut Context) -> Result<Value, CarrierError> {',
        '// SAFETY: the caller retains its aligned node and all counted children.',
        'let input = unsafe { borrow(pointer)? };',
        'context.enter(|context| {',
        '// SAFETY: only the validated tag selects a payload; each extent is checked.',
        'unsafe { match input.kind {']
    for kind in JSON_KINDS:
        symbol = 'THINKTHEN_COMPLETE_JSON_' + kind.upper() + '_V1'
        payload = 'input.data.' + kind
        if kind == 'null':
            body = ['Value::literal("null")']
        elif kind == 'boolean':
            body = ['Ok(Value::Boolean(match ' + payload + ' { 0 => false, 1 => true, _ => return Err(CarrierError::Invalid) }))']
        elif kind == 'string':
            body = ['Ok(Value::Text(context.text(' + payload + ')?))']
        elif kind == 'number':
            body = ['let number = context.text(' + payload + ')?;',
                "if !matches!(number.as_bytes().first(), Some(b'-' | b'0'..=b'9')) { return Err(CarrierError::Invalid); }",
                'Value::literal(&number)']
        else:
            body = ['let input = ' + payload + ';', 'let mut values = Vec::new();',
                'for item in extent(input.data, input.len)? {']
            if kind == 'array':
                body.append('values.push(read_json_value(*item, context)?);')
            elif kind == 'object':
                body.append('values.push((context.text(item.name)?, read_json_value(item.value, context)?));')
            else:
                raise ValueError('unsupported canonical JSON layout kind: ' + kind)
            body += ['}', 'Ok(Value::' + ('Array' if kind == 'array' else 'Object') + '(values))']
        lines += [symbol + ' => {', *body, '},']
    return '\n'.join([*lines, '_ => Err(CarrierError::Invalid),', '} }', '})', '}'])


def render(definitions, metadata=None):
    nodes, select = carrier_graph(definitions)
    declarations, readers, done = [], [], set()
    def structure(typ, fields):
        declarations.append('#[repr(C)]\n#[derive(Clone, Copy, Debug)]\npub struct ' + typ + ' {\n' + '\n'.join('    pub ' + field + ': ' + value + ',' for field, value in fields) + '\n}')
    pending = list(nodes)
    while pending:
        key = pending.pop(0)
        if key in done:
            continue
        done.add(key)
        source = nodes[key]
        typ = name(key)
        fields, body = [], []
        def child(value, path):
            before = set(nodes)
            selected = select(value, path)
            if metadata is not None:
                metadata.setdefault("children", {})[(key, path)] = selected
            pending.extend(sorted(set(nodes) - before))
            return selected
        def read(target, pointer):
            return 'read_' + name(target) + '(' + pointer + ', context)?'
        if '$ref' in source:
            target = source['$ref'].removeprefix('#/$defs/')
            fields = [('value', '*const ' + name(target))]
            body = [read(target, 'input.value')]
        elif 'properties' in source:
            initial, optional = [], []
            body = []
            for member, value in source['properties'].items():
                resolved = nodes[value['$ref'].removeprefix('#/$defs/')] if isinstance(value, dict) and '$ref' in value else value
                if isinstance(resolved, dict) and 'const' in resolved:
                    initial.append('(' + quoted(member) + '.into(), Value::literal(' + quoted(json.dumps(resolved['const'])) + ')?)')
                    continue
                target = child(value, key + '_' + member)
                field = 'm_' + member
                fields.append((field, '*const ' + name(target)))
                if member in source.get('required', []):
                    initial.append('(' + quoted(member) + '.into(), ' + read(target, 'input.' + field) + ')')
                else:
                    optional.append('optional(' + quoted(member) + ', input.' + field + ', context, &mut fields, read_' + name(target) + ')?;')
            if not fields:
                fields = [('reserved', 'u32')]
            body = ['let ' + ('mut ' if optional else '') + 'fields = vec![' + ', '.join(initial) + '];', *optional, 'Value::Object(fields)']
        elif source.get('variants') or source.get('primitive_variants') or source.get('anyOf') or source.get('oneOf') or isinstance(source.get('type'), list):
            alternatives = source.get('oneOf', source.get('anyOf', []))
            if source.get('variants'):
                alternatives = [{'$ref':'#/$defs/' + target} for target, _ in source['variants']]
            elif source.get('primitive_variants'):
                alternatives = [source['primitive_schemas'][kind] for kind in source['primitive_variants']]
            elif isinstance(source.get('type'), list):
                alternatives = [{**source, 'type':kind} for kind in source['type']]
            union = typ.removesuffix('_v1') + '_data_v1'
            arms = []
            body = ['match input.kind {']
            targets = [child(alternative, key + "_arm_" + str(index)) for index, alternative in enumerate(alternatives, 1)]
            for index, target in enumerate(targets, 1):
                arm = union_member(key, target, targets)
                if any(line.startswith('    pub ' + arm + ':') for line in arms):
                    raise ValueError('duplicate canonical request union field: ' + key + '.' + arm)
                if metadata is not None:
                    metadata.setdefault('arms', {})[(key, index)] = arm
                symbol = typ.removesuffix('_v1').upper() + '_ARM_' + str(index) + '_V1'
                declarations.append('pub const ' + symbol + ': u32 = ' + str(index) + ';')
                arms.append('    pub ' + arm + ': *const ' + name(target) + ',')
                body.append(symbol + ' => ' + read(target, 'input.data.' + arm) + ',')
            body += ['_ => return Err(CarrierError::Invalid),', '}']
            declarations.append('#[repr(C)]\n#[derive(Clone, Copy)]\npub union ' + union + ' {\n' + '\n'.join(arms) + '\n}\nimpl std::fmt::Debug for ' + union + ' {\nfn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { f.write_str("request union payload") }\n}')
            fields = [('kind', 'u32'), ('data', union)]
        elif source.get('type') == 'array':
            target = child(source.get('items', {}), key + '_item')
            fields = [('data', '*const *const ' + name(target)), ('len', 'usize')]
            body = ['let mut values = Vec::new();', 'for item in extent(input.data, input.len)? { values.push(' + read(target, '*item') + '); }', 'Value::Array(values)']
        elif source.get('type') == 'object':
            target = child(source.get('additionalProperties', {}), key + '_value')
            entry = typ.removesuffix('_v1') + '_entry_v1'
            structure(entry, [('key', 'thinkthen_complete_utf8_v1'), ('value', '*const ' + name(target))])
            fields = [('data', '*const ' + entry), ('len', 'usize')]
            body = ['let mut values = Vec::new();', 'for item in extent(input.data, input.len)? { values.push((context.text(item.key)?, ' + read(target, 'item.value') + ')); }', 'Value::Object(values)']
        elif source.get('enum'):
            fields = [('kind', 'u32')]
            body = ['match input.kind {']
            for index, value in enumerate(source['enum'], 1):
                symbol = typ.removesuffix('_v1').upper() + '_' + re.sub(r'[^A-Za-z0-9]+', '_', str(value)).upper() + '_V1'
                declarations.append('pub const ' + symbol + ': u32 = ' + str(index) + ';')
                body.append(symbol + ' => Value::literal(' + quoted(json.dumps(value)) + ')?,')
            body += ['_ => return Err(CarrierError::Invalid),', '}']
        elif 'const' in source or source.get('type') == 'null':
            fields = [('reserved', 'u32')]
            body = ['Value::literal(' + quoted(json.dumps(source.get('const'))) + ')?']
        elif source.get('type') in ('integer', 'number', 'boolean'):
            kind = source['type']
            scalar = 'f64' if kind == 'number' else 'u32' if kind == 'boolean' else 'u64' if source.get('format', '').startswith('u') else 'i64'
            fields = [('value', scalar)]
            body = [({'boolean':'Value::Boolean(match input.value { 0 => false, 1 => true, _ => return Err(CarrierError::Invalid) })', 'number':'Value::Number(serde_json::Number::from_f64(input.value).ok_or(CarrierError::Invalid)?)', 'integer':'Value::Number(input.value.into())'})[kind]]
        elif source.get('type') == 'string':
            fields = [('text', 'thinkthen_complete_utf8_v1')]
            body = ['Value::Text(context.text(input.text)?)']
        else:
            fields = [('value', '*const thinkthen_complete_json_v1')]
            body = ['let value = read_json_value(input.value, context)?;',
                'if value.json().map_err(|_| CarrierError::Invalid)?.len() > context.scalar_capacity { return Err(CarrierError::Overflow); }',
                'value']
        if metadata is not None:
            metadata.setdefault('fields', {})[key] = tuple(fields)
        structure(typ, fields)
        if 'const' in source or source.get('type') == 'null':
            read_body = 'let _ = unsafe { borrow(pointer)? };\nlet _ = context;\n' + body[0].removesuffix('?')
        else:
            expression = '\n'.join(body)
            conversion = (expression.removesuffix('?') if len(body) == 1 and expression.endswith('?') else
                          'Ok(' + expression + ')' if len(body) == 1 or body[0].startswith('match ') else
                          'Ok({\n' + expression + '\n})')
            if any(token in conversion for token in ('read_', 'context.text(', 'extent(', 'optional(')):
                conversion = '// SAFETY: nested pointers retain the caller\'s live counted carrier graph.\nunsafe { ' + conversion + ' }'
            variable = 'input' if 'input.' in conversion else '_input'
            parameter = 'context' if 'context' in conversion else '_context'
            read_body = 'let ' + variable + ' = unsafe { borrow(pointer)? };\ncontext.enter(|' + parameter + '| {\n' + conversion + '\n})'
        readers.append('pub(super) unsafe fn read_' + typ + '(pointer: *const ' + typ + ', context: &mut Context) -> Result<Value, CarrierError> {\n// SAFETY: the caller retains every referenced aligned carrier and counted extent.\n' + read_body + '\n}')
    if metadata is not None:
        metadata['nodes'] = nodes
    return '''// Generated counted carriers from the canonical request graph; do not edit.
#![allow(non_camel_case_types, missing_docs, clippy::too_many_lines, reason = "generated versioned request carriers and conversion")]
use super::{CarrierError, Context, Value, borrow, extent, optional};
use crate::session::views::{JSON_IMPORTS};
'''.replace('JSON_IMPORTS', ', '.join(['thinkthen_complete_utf8_v1', 'thinkthen_complete_json_v1', *('THINKTHEN_COMPLETE_JSON_' + kind.upper() + '_V1' for kind in JSON_KINDS)])) + '\n'.join(declarations + [value_tree_reader()] + readers) + '\n'
