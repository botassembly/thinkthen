"""Construct generated COBOL test records from the canonical Request graph."""
import ctypes as ct
import importlib.util
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[3]

def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result

generator = module('cobol_generator', ROOT / 'sdlc/generators/results/generate.py')
template = module('cobol_template', ROOT / 'sdlc/generators/results/templates/cobol.py')
schema = json.loads((ROOT / 'specification/request.schema.json').read_text())
schema['$defs']['Request'] = {k: v for k, v in schema.items() if k != '$defs'}
definitions = generator.prepare(generator.graph(schema, ('Request', 'RequestSessionDescriptor')))

class Records:
    def __init__(self, package):
        self.owners = []
        self.nodes = dict(definitions)
        self.types = {}
        header = (package / 'src/tt_requests_generated.h').read_text()
        for name, body in re.findall(r'typedef struct (thinkthen_cobol_\w+) \{([^}]+)\}', header):
            fields = []
            for kind, field in re.findall(r'([^;]+?)\b(\w+)\s*;', body):
                kind = kind.strip()
                typ = ct.c_void_p if '*' in kind else {'uint64_t': ct.c_uint64, 'int64_t': ct.c_int64, 'double': ct.c_double}[kind]
                fields.append((field, typ))
            self.types[name.removeprefix('thinkthen_cobol_')] = type(name, (ct.Structure,), {'_fields_': fields})
    def keep(self, value):
        self.owners.append(value)
        return ct.addressof(value)
    def branch(self, alternatives, value):
        for index, (key, tag) in enumerate(alternatives, 1):
            mode, member, expected = tag
            if mode == 'literal': matched = isinstance(value, dict) and value.get(member) == expected
            elif mode == 'literals': matched = isinstance(value, dict) and all(value.get(k) == v for k,v in expected.items())
            elif mode == 'member': matched = isinstance(value, dict) and member in value
            elif mode == 'structure': matched = isinstance(value, dict) and set(expected['required']) <= value.keys() and not set(expected['excluded']) & value.keys()
            else: raise ValueError(mode)
            if matched: return index, key
        raise ValueError(('no generated request variant', value))
    def construct(self, key, source, value):
        source = source if isinstance(source, dict) else {}
        if '$ref' in source:
            key = source['$ref'].removeprefix('#/$defs/')
            source = self.nodes[key]
        if key not in self.types:
            raise ValueError('missing generated request declaration ' + key)
        row = self.types[key]()
        if len(source.get('oneOf', [])) == 1 and 'const' in source['oneOf'][0]: source = source['oneOf'][0]
        if source.get('variants'):
            row.kind, branch = self.branch(source['variants'], value)
            row.value = self.construct(branch, self.nodes[branch], value)
        elif 'properties' in source:
            assert value.keys() <= source['properties'].keys(), (key, value)
            for member, child in source['properties'].items():
                if member in value and 'const' not in child:
                    setattr(row, 'm_' + member, self.construct(key + '_member_' + member, child, value[member]))
        elif source.get('type') == 'array':
            array = (ct.c_void_p * len(value))(*(self.construct(key + '_item', source.get('items', {}), v) for v in value))
            row.data, row.len = self.keep(array), len(value)
        elif source.get('type') == 'object' and isinstance(source.get('additionalProperties'), dict):
            keys = (self.types['text'] * len(value))()
            values = (ct.c_void_p * len(value))()
            for index, (k, v) in enumerate(value.items()):
                data = k.encode(); keys[index].data = self.keep(ct.create_string_buffer(data)); keys[index].len = len(data)
                values[index] = self.construct(key + '_entry', source['additionalProperties'], v)
            row.keys, row.values, row.len = self.keep(keys), self.keep(values), len(value)
        elif 'const' in source or source.get('type') == 'null': pass
        elif source.get('primitive_variants') or source.get('oneOf') or source.get('anyOf') or isinstance(source.get('type'), list):
            alternatives = source.get('oneOf', source.get('anyOf', []))
            if 'primitive_variants' in source: alternatives = [source['primitive_schemas'][k] for k in source['primitive_variants']]
            if isinstance(source.get('type'), list): alternatives = [{**source, 'type': k} for k in source['type']]
            kinds = {'string': isinstance(value, str), 'integer': type(value) is int, 'number': type(value) in (float,int), 'boolean': type(value) is bool, 'null': value is None, 'object': isinstance(value, dict), 'array': isinstance(value, list)}
            index, branch = next((i, a) for i,a in enumerate(alternatives,1) if kinds.get(a.get('type'), True) and ('const' not in a or a['const'] == value))
            row.kind, row.value = index, self.construct(key + '_arm_' + str(index), branch, value)
        elif source.get('type') in ('integer','number','boolean'): row.value = value
        else:
            data = (value if source.get('type') == 'string' else json.dumps(value,ensure_ascii=False,separators=(',',':'))).encode()
            row.data, row.len = self.keep(ct.create_string_buffer(data)), len(data)
        return self.keep(row)
