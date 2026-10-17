"""Copy the compiler's native result graph before releasing its packet owner."""
import json
from c import ident, normalized, nullable, container_key, variant_member


def render(definitions):
    from generate import references
    selected, pending = {}, ['completesessionPacket']
    while pending:
        key = pending.pop()
        if key in selected:
            continue
        source = selected[key] = definitions[key]
        pending.extend(references(source))
        pending.extend(child for child, _ in source.get('variants', []))
    definitions = selected
    lines = ['import CThinkThen', '\nfunc nativeElements<T>(_ pointer: UnsafePointer<T>?, _ count: Int) throws -> [T] {\n    if count == 0 { return [] }\n    guard let pointer else { throw ValueConversionError("Missing native collection") }\n    return Array(UnsafeBufferPointer(start: pointer, count: count))\n}\nfunc copyNativeLeaf(_ pointer: UnsafePointer<thinkthen_complete_json_v1>?) throws -> DataValue {\n    guard let pointer else { throw ValueConversionError("Missing native leaf") }\n    return try DataValue.copyNative(pointer)\n}\nfunc copyNativeExtensions(_ value: thinkthen_complete_extensions_v1) throws -> [ValueMember] {\n    try nativeElements(value.data, value.len).map {\n        ValueMember(nativeString($0.name), try copyNativeLeaf($0.value))\n    }\n}\n']
    lines += ['func nativeValue(_ owner: OpaquePointer) throws -> DataValue {\n    var view: UnsafePointer<thinkthen_complete_json_v1>?\n    try sessionCheck(thinkthen_value_view(owner, &view))\n    guard let view else { throw ValueConversionError("Missing native value view") }\n    return try DataValue.copyNative(view)\n}\n']
    emitted = set()

    def shape(source, path, value):
        source = normalized(source)
        if '$ref' in source:
            return f'try copyNativeGraph({value})'
        if source == {} or source.get('type') == 'null':
            return f'try copyNativeLeaf({value})'
        if any(key in source for key in ('enum', 'properties', 'variants', 'primitive_variants')):
            define(path, source)
            return f'try copyNativeGraph({value})'
        kind = source.get('type')
        if kind == 'string' or isinstance(source.get('const'), str):
            return f'.string(nativeString({value}))'
        if kind == 'boolean':
            return f'.bool({value} != 0)'
        if kind in ('integer', 'number'):
            return f'.number(NumberLiteral(unchecked: String({value})))'
        container = container_key(source, path)
        if kind == 'array':
            child = shape(source['items'], container + '_item', 'item')
            return f'.array(try nativeElements({value}.data, {value}.len).map {{ item in {child} }})'
        if kind == 'object':
            child = shape(source['additionalProperties'], container + '_entry_value', 'item.value')
            return f'.orderedObject(try nativeElements({value}.data, {value}.len).map {{ item in ValueMember(nativeString(item.name), {child}) }})'
        raise ValueError(source)

    def define(key, source):
        cname = ident(key)
        if cname in emitted:
            return
        emitted.add(cname)
        source = normalized(source)
        body = []
        if 'variants' in source:
            body += ['    switch value.kind {']
            for index, (child, _) in enumerate(source['variants'], 1):
                arm = variant_member(key, child)
                body += [f'    case {index}: return try copyNativeGraph(value.data.`{arm}`)']
            body += ['    default: throw ValueConversionError("Unknown native packet alternative")', '    }']
        elif 'primitive_variants' in source:
            body += ['    switch value.kind {']
            for index, kind in enumerate(source['primitive_variants'], 1):
                expression = '.null' if kind == 'null' else shape(source.get('primitive_schemas', {}).get(kind, {'type': kind}), key + '_' + kind, f'value.data.`{kind}`')
                body += [f'    case {index}: return {expression}']
            body += ['    default: throw ValueConversionError("Unknown native value alternative")', '    }']
        elif 'enum' in source:
            body += ['    switch value.kind {']
            body += [f'    case {index}: return .string({json.dumps(item)})' for index, item in enumerate(source['enum'], 1)]
            body += ['    default: throw ValueConversionError("Unknown native enum value")', '    }']
        elif 'properties' in source:
            body += ['    var members = try copyNativeExtensions(value.extensions)']
            for field, schema in source['properties'].items():
                access = f'value.`{field + chr(95) if field in ("true", "false") else field}`'
                presence = field not in source.get('required', []) or nullable(schema, definitions)
                expression = shape(schema, key + '_field_' + field, access + ('.value' if presence else ''))
                if presence:
                    body += [f'    switch {access}.presence {{', '    case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break', f'    case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember({json.dumps(field)}, .null))', f'    case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1): members.append(ValueMember({json.dumps(field)}, {expression}))', '    default: throw ValueConversionError("Unknown native presence")', '    }']
                else:
                    body += [f'    members.append(ValueMember({json.dumps(field)}, {expression}))']
            body += ['    return .orderedObject(members)']
        else:
            body += ['    return ' + shape(source, key + '_value', 'value.value')]
        lines.extend([f'private func copyNativeGraph(_ pointer: UnsafePointer<{cname}>?) throws -> DataValue {{', '    guard let pointer else { throw ValueConversionError("Missing native graph member") }', '    let value = pointer.pointee', *body, '}'])

    for key, source in definitions.items():
        define(key, source)
    # Only this entry is visible to the handwritten session owner.
    lines += ['func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_packet_v1>) throws -> DataValue {', '    try copyNativeGraph(Optional(pointer))', '}']
    return '\n'.join(lines)
