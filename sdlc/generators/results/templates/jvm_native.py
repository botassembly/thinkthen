"""Copy compiler-measured immutable C result trees into independent JVM values."""
import json
from c import Target, ident, normalized


def render(root, abi):
    from generate import graph, prepare, references
    from native import immediate_schema
    ROOTS = ("completesessionPacket", "completeImmediateFacts")
    all_definitions = prepare(graph(immediate_schema(json.loads((root / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())), ROOTS))
    definitions, pending = {}, list(ROOTS)
    while pending:
        key = pending.pop()
        if key in definitions: continue
        source = definitions[key] = all_definitions[key]
        pending.extend(references(source))
        pending.extend(child for child, _ in source.get('variants', []))
    class Measured(Target):
        def __init__(self):
            super().__init__(definitions)
            self.fields, self.arms = {}, {}
        def structure(self, name, members):
            self.fields[name] = members
            super().structure(name, members)
        def union(self, key, alternatives):
            self.arms[ident(key)] = [(arm, typ) for arm, _, typ, _ in alternatives]
            super().union(key, alternatives)
    target = Measured()
    for key, source in sorted(definitions.items()):
        target.definition(key, source)
    records = abi['records']
    def offset(typ, field):
        return records[typ]['fields'][field + '_' if field in ('false', 'true') else field]['offset']
    def size(typ):
        return records[typ]['size']
    def read(typ, expr):
        if typ.startswith('*const '):
            return read(typ[7:], f'pointer({expr})')
        return {'u32': f'new java.math.BigDecimal(Integer.toUnsignedLong({expr}.get(ValueLayout.JAVA_INT, 0)))',
                'u64': f'new java.math.BigDecimal(Long.toUnsignedString({expr}.get(ValueLayout.JAVA_LONG, 0)))',
                'i64': f'java.math.BigDecimal.valueOf({expr}.get(ValueLayout.JAVA_LONG, 0))',
                'f64': f'java.math.BigDecimal.valueOf({expr}.get(ValueLayout.JAVA_DOUBLE, 0))',
                'thinkthen_complete_utf8_v1': f'text({expr})',
                'thinkthen_complete_json_v1': f'json({expr})'}.get(typ, f'{typ}({expr})')
    lines = ['// Generated from the canonical result graph and compiler-measured C ABI. Do not edit.',
             'package thinkthen;', 'import java.lang.foreign.*;', 'import java.util.*;',
             'final class NativeResults {', 'private NativeResults() {}',
             '@SuppressWarnings("restricted") static MemorySegment pointer(MemorySegment p) { return p.get(ValueLayout.ADDRESS, 0).reinterpret(Long.MAX_VALUE); }',
             'static String text(MemorySegment p) { long length = p.get(NativeSession.SIZE, ' + str(offset('thinkthen_complete_utf8_v1', 'len')) + '); if (length < 0 || length > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native string length"); return new String(pointer(p).asSlice(0, length).toArray(ValueLayout.JAVA_BYTE), java.nio.charset.StandardCharsets.UTF_8); }',
             'static Object packet(MemorySegment owner) { try (Arena arena = Arena.ofConfined()) { var out = arena.allocate(ValueLayout.ADDRESS); NativeSession.check((int)NativeSession.call("thinkthen_session_result_view", ValueLayout.JAVA_INT, new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS}, owner, out)); return ' + ident('completesessionPacket') + '(pointer(out)); } }',
             'static Object json(MemorySegment p) { var data = p.asSlice(' + str(offset('thinkthen_complete_json_v1', 'data')) + '); switch (p.get(ValueLayout.JAVA_INT, 0)) {']
    kinds = abi['constants']
    for kind in ('null', 'boolean', 'number', 'string', 'array', 'object'):
        expression = {'null': 'null', 'boolean': 'data.get(ValueLayout.JAVA_INT, 0) != 0', 'number': 'new java.math.BigDecimal(text(data))', 'string': 'text(data)', 'array': 'thinkthen_complete_json_array_v1(data)', 'object': 'thinkthen_complete_json_object_v1(data)'}[kind]
        lines.append(f'case {kinds["THINKTHEN_COMPLETE_JSON_" + kind.upper() + "_V1"]}: return {expression};')
    lines += ['default: throw new IllegalStateException("Unknown native JSON kind"); } }']
    lines += ['static NativeFailure failure(MemorySegment view) {', '@SuppressWarnings("restricted") var p = view.reinterpret(' + str(size('thinkthen_error_view_v1')) + ');', 'var facts = p.get(ValueLayout.ADDRESS, ' + str(offset('thinkthen_error_view_v1', 'facts')) + ');', 'return new NativeFailure(p.get(ValueLayout.JAVA_INT, ' + str(offset('thinkthen_error_view_v1', 'code')) + '), p.get(ValueLayout.JAVA_INT, ' + str(offset('thinkthen_error_view_v1', 'retryable')) + ') != 0, text(p.asSlice(' + str(offset('thinkthen_error_view_v1', 'message')) + ')), facts.equals(MemorySegment.NULL) ? null : new Results.ImmediateFacts(thinkthen_complete_immediate_facts_v1(pointer(p.asSlice(' + str(offset('thinkthen_error_view_v1', 'facts')) + ')))));', '}']
    fields = dict(target.fields)
    fields['thinkthen_complete_json_array_v1'] = [('data', '*const *const thinkthen_complete_json_v1'), ('len', 'usize')]
    fields['thinkthen_complete_json_object_v1'] = [('data', '*const thinkthen_complete_json_entry_v1'), ('len', 'usize')]
    fields['thinkthen_complete_extensions_v1'] = fields['thinkthen_complete_json_object_v1']
    for typ, members in fields.items():
        source = target.schemas.get(typ, {})
        lines.append(f'static Object {typ}(MemorySegment p) {{')
        if typ in target.arms:
            lines.append(f'var data = p.asSlice({offset(typ, "data")}); switch (p.get(ValueLayout.JAVA_INT, {offset(typ, "kind")})) {{')
            for index, (arm, child) in enumerate(target.arms[typ], 1):
                expression = 'null' if arm == 'null' else read(child, 'data')
                if arm == 'boolean': expression = 'data.get(ValueLayout.JAVA_INT, 0) != 0'
                lines.append(f'case {index}: return {expression};')
            lines += ['default: throw new IllegalStateException("Unknown native result kind"); }']
        elif 'enum' in source:
            lines.append(f'return switch (p.get(ValueLayout.JAVA_INT, {offset(typ, "kind")})) {{')
            lines += [f'case {index}: yield {json.dumps(value)};' for index, value in enumerate(source['enum'], 1)]
            lines += ['default: throw new IllegalStateException("Unknown native enum kind"); };']
        elif members[0][0] == 'data' and members[-1][0] == 'len':
            child = members[0][1][7:]
            mapping = child.endswith('_entry_v1')
            stride = 8 if child.startswith('*const ') else size(child) if child in records else {'u32': 4, 'u64': 8, 'i64': 8, 'f64': 8}[child]
            lines += [f'long count = p.get(NativeSession.SIZE, {offset(typ, "len")}); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");', 'var data = pointer(p);', 'var copy = new LinkedHashMap<String,Object>();' if mapping else 'var copy = new ArrayList<Object>();', f'for (long i = 0; i < count; i++) {{ var item = data.asSlice(i * {stride});']
            if mapping:
                val_type = records[child]['fields']['value']['type']
                val_type = val_type.replace('const ', '').replace(' *', '')
                val_type = {'double': 'f64', 'uint32_t': 'u32', 'uint64_t': 'u64', 'int64_t': 'i64'}.get(val_type, val_type)
                is_pointer = '*' in records[child]['fields']['value']['type']
                value = read(('*const ' if is_pointer else '') + val_type, f'item.asSlice({offset(child,"value")})')
                lines.append(f'copy.put(text(item.asSlice({offset(child,"name")})), {value});')
            else: lines.append(f'copy.add({read(child, "item")});')
            lines += ['}', 'return Collections.unmodifiableMap(copy);' if mapping else 'return Collections.unmodifiableList(copy);']
        elif members[0][0] == 'presence':
            lines.append('throw new IllegalStateException("Presence must be decoded with its owning field");')
        elif 'properties' in source:
            lines.append('var copy = new LinkedHashMap<String,Object>();')
            for member, child in members:
                expr = f'p.asSlice({offset(typ, member)})'
                if member == 'extensions':
                    lines.append(f'copy.putAll(Values.object({read(child, expr)}));')
                elif child.endswith('_presence_v1'):
                    inner = target.fields[child][1][1]
                    expression = read(inner, member + 'Slot.asSlice(' + str(offset(child, 'value')) + ')')
                    if normalized(source['properties'][member]).get('type') == 'boolean': expression = f'{member}Slot.get(ValueLayout.JAVA_INT, {offset(child, "value")}) != 0'
                    lines += [f'var {member}Slot = {expr};', f'int {member}State = {member}Slot.get(ValueLayout.JAVA_INT, {offset(child,"presence")});', f'if ({member}State == 1) copy.put({json.dumps(member)}, null); else if ({member}State == 2) copy.put({json.dumps(member)}, {expression});']
                else:
                    expression = read(child, expr)
                    spec = source['properties'][member]
                    if normalized(spec).get('type') == 'boolean': expression = f'{expr}.get(ValueLayout.JAVA_INT, 0) != 0'
                    lines.append(f'copy.put({json.dumps(member)}, {expression});')
            lines.append('return Collections.unmodifiableMap(copy);')
        elif len(members) == 1 and members[0][0] == 'value':
            expression = read(members[0][1], f'p.asSlice({offset(typ,"value")})')
            if source.get('type') == 'boolean': expression = f'p.get(ValueLayout.JAVA_INT, {offset(typ,"value")}) != 0'
            lines.append(f'return {expression};')
        else:
            # Entry layouts are traversed by their owning mapping.
            lines.append('throw new IllegalStateException("Unexpected native standalone entry");')
        lines.append('}')
    return lines + ['}']
