"""Serialize the installed owned records only for shared fixture assertions."""
import ctypes as ct
import json
import tempfile
from pathlib import Path
from session_records import ROOT, template, generator
from templates.c import Target, ident, normalized, nullable, container_key, variant_member


def record_types(package, header):
    with tempfile.TemporaryDirectory(prefix='cobol-owned-abi-') as temporary:
        combined = Path(temporary) / 'owned.h'
        combined.write_text('#include ' + json.dumps(str(header)) + '\n#include ' + json.dumps(str(package / 'src/tt_requests_generated.h')) + '\n')
        facts = template.abi_module(ROOT).header_abi(combined)
    records, types = facts['records'], {}
    def record(name):
        if name in types: return types[name]
        cls = type(name, (ct.Structure,), {})
        types[name] = cls
        fields = []
        for member, field in records[name]['fields'].items():
            if '.' in member: continue
            kind = field['type']; nested = kind.removeprefix('struct ')
            if '*' in kind: typ = ct.c_void_p
            elif kind.startswith('char['): typ = ct.c_char * field['width']
            elif nested in records: typ = record(nested)
            else: typ = {'uint32_t': ct.c_uint32, 'uint64_t': ct.c_uint64, 'int64_t': ct.c_int64, 'size_t': ct.c_size_t, 'double': ct.c_double}[kind]
            fields.append((member, typ))
        cls._fields_ = fields
        assert ct.sizeof(cls) == records[name]['size'], name
        for member, field in records[name]['fields'].items():
            if '.' not in member: assert getattr(cls, member).offset == field['offset'], (name, member)
        return cls
    return record


class Reader:
    def __init__(self, bridge, record):
        self.bridge, self.record = bridge, record
        self.prefix = 'Receipt'
        self.constants = template.abi_module(ROOT).header_abi(ROOT / 'libraries/c/include/thinkthen.h')['constants']
        schema = json.loads((ROOT / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())
        self.definitions = generator.prepare(generator.graph(schema, ('completesessionPacket',)))
        self.target = Target(self.definitions)
        for key, source in self.definitions.items(): self.target.definition(key, source)
        for name, args in {'TT_VALUE_KIND': [ct.c_void_p, ct.c_void_p], 'TT_VALUE_BOOLEAN': [ct.c_void_p, ct.c_void_p],
                           'TT_VALUE_LENGTH': [ct.c_void_p, ct.c_void_p], 'TT_VALUE_TEXT': [ct.c_void_p, ct.c_void_p, ct.c_void_p],
                           'TT_VALUE_NAME': [ct.c_void_p, ct.c_uint64, ct.c_void_p, ct.c_void_p],
                           'TT_VALUE_ELEMENT': [ct.c_void_p, ct.c_uint64, ct.c_void_p], 'TT_VALUE_FREE': [ct.c_void_p]}.items():
            getattr(bridge, name).argtypes = args

    def text(self, value):
        return value.text[:value.length].decode()

    def json_value(self, address):
        kind = ct.c_uint32(); assert self.bridge.TT_VALUE_KIND(address, ct.byref(kind)) == 0
        constants = self.constants
        if kind.value == constants['THINKTHEN_COMPLETE_JSON_NULL_V1']: return None
        if kind.value == constants['THINKTHEN_COMPLETE_JSON_BOOLEAN_V1']:
            boolean = ct.c_uint32(); assert self.bridge.TT_VALUE_BOOLEAN(address, ct.byref(boolean)) == 0
            return bool(boolean.value)
        output, length = ct.create_string_buffer(8192), ct.c_uint64()
        if kind.value in (constants['THINKTHEN_COMPLETE_JSON_STRING_V1'], constants['THINKTHEN_COMPLETE_JSON_NUMBER_V1']):
            assert self.bridge.TT_VALUE_TEXT(address, output, ct.byref(length)) == 0
            text = output.raw[:length.value].decode()
            return text if kind.value == constants['THINKTHEN_COMPLETE_JSON_STRING_V1'] else json.loads(text)
        assert self.bridge.TT_VALUE_LENGTH(address, ct.byref(length)) == 0
        result = [] if kind.value == constants['THINKTHEN_COMPLETE_JSON_ARRAY_V1'] else {}
        for index in range(1, length.value + 1):
            child = ct.c_void_p(); assert self.bridge.TT_VALUE_ELEMENT(address, index, ct.byref(child)) == 0
            projected = self.json_value(child.value); self.bridge.TT_VALUE_FREE(child)
            if isinstance(result, list): result.append(projected)
            else:
                size = ct.c_uint64(); assert self.bridge.TT_VALUE_NAME(address, index, output, ct.byref(size)) == 0
                result[output.raw[:size.value].decode()] = projected
        return result

    def name(self, native):
        if native == 'thinkthen_complete_utf8_v1': return 'thinkthen_cobol_AnswerText'
        if native == 'thinkthen_complete_json_v1': return None
        return 'thinkthen_cobol_' + self.prefix + ''.join(word.title() for word in native.removeprefix('thinkthen_complete_').removesuffix('_v1').split('_'))

    def shape(self, schema, path):
        schema = normalized(schema)
        if '$ref' in schema: return self.name(ident(schema['$ref'].removeprefix('#/$defs/')))
        if not schema or schema.get('type') == 'null': return None
        if any(key in schema for key in ('enum', 'variants', 'primitive_variants', 'properties')): return self.name(ident(path))
        if schema.get('type') == 'string' or isinstance(schema.get('const'), str): return 'thinkthen_cobol_AnswerText'
        if schema.get('type') in ('integer', 'number', 'boolean'): return schema['type']
        return self.name(ident(container_key(schema, path)))

    def decode(self, schema, value, path):
        schema = normalized(schema)
        if '$ref' in schema:
            key = schema['$ref'].removeprefix('#/$defs/')
            return self.definition(key, value)
        if not schema or schema.get('type') == 'null': return self.json_value(value)
        if any(key in schema for key in ('enum', 'variants', 'primitive_variants', 'properties')): return self.definition(path, value)
        kind = schema.get('type')
        if kind == 'string' or isinstance(schema.get('const'), str): return self.text(value)
        if kind == 'boolean': return bool(value)
        if kind in ('integer', 'number'): return value
        mapping = kind == 'object'; element = schema.get('additionalProperties', {}) if mapping else schema['items']
        element_path = container_key(schema, path) + ('_entry_value' if mapping else '_item')
        item_shape = self.shape(element, element_path)
        typ = self.record(self.name(ident(container_key(schema, path) + '_entry'))) if mapping else {'integer': ct.c_uint64, 'number': ct.c_double, 'boolean': ct.c_uint32, None: ct.c_void_p}.get(item_shape) or self.record(item_shape)
        rows = [typ.from_address(value.data + index * ct.sizeof(typ)) for index in range(value.len)]
        return {self.text(row.name): self.decode(element, row.value, element_path) for row in rows} if mapping else [self.decode(element, row.value if item_shape in (None, 'integer', 'number', 'boolean') else row, element_path) for row in rows]

    def definition(self, key, value):
        source = self.target.schemas[ident(key)]
        if isinstance(value, int): value = self.record(self.name(ident(key))).from_address(value)
        if 'enum' in source: return source['enum'][value.kind - 1]
        if 'variants' in source:
            child = source['variants'][value.kind - 1][0]
            return self.definition(child, getattr(value.data, variant_member(key, child)))
        if 'primitive_variants' in source:
            kind = source['primitive_variants'][value.kind - 1]
            return self.decode(source['primitive_schemas'][kind], getattr(value.data, kind), key + '_' + kind)
        if 'properties' not in source: return self.decode(source, value.value, key + '_value')
        result = {}
        for member, schema in source['properties'].items():
            field = getattr(value, member)
            if member not in source.get('required', []) or nullable(schema, self.definitions):
                if field.presence == 0: continue
                if field.presence == 1: result[member] = None; continue
                field = field.value
            result[member] = self.decode(schema, field, key + '_field_' + member)
        extension = self.record('thinkthen_cobol_' + self.prefix + 'Extension')
        for index in range(value.extensions.len):
            row = extension.from_address(value.extensions.data + index * ct.sizeof(extension))
            result[self.text(row.name)] = self.json_value(row.value)
        return result
