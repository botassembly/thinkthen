from templates.host_alternatives import alternative_names
"""COBOL records and mechanical serialization from Rust schemas and measured C ABI."""
import importlib.util
import json
import re
from pathlib import Path


def identifier(value):
    text = value.removeprefix('thinkthen_').replace('_', '-')
    if len(text) > 55:
        raise ValueError('COBOL native name exceeds 60 characters: ' + value)
    return 'tt-n-' + text


def abi_module(root):
    spec = importlib.util.spec_from_file_location('cobol_abi', root / 'sdlc/scripts/check-c-exports.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def copybook(facts):
    lines = ['      *> Generated from measured canonical C declarations; do not edit.']
    def members(record, level, path=""):
        def label(key):
            text = "v-" + (path + key).replace("_", "-").rstrip("-")
            if len(text) > 60:
                raise ValueError("COBOL field name exceeds 60 characters: " + text)
            return text
        output, cursor = [], 0
        direct = [(key, field) for key, field in record['fields'].items() if '.' not in key]
        union = len(direct) > 1 and all(field['offset'] == 0 for _, field in direct)
        if union:
            output.append(f' {level:02} {label("union_storage")} pic x({record["size"]}).')
        for key, field in direct:
            offset, width, kind = field['offset'], field['width'], field['type']
            if not union and offset > cursor:
                output.append(f' {level:02} filler pic x({offset-cursor}).')
            field_label = label(key)
            if union:
                field_label += ' redefines ' + label('union_storage')
            nested = kind.removeprefix('struct ').removeprefix('union ')
            if kind == 'union':
                candidates = [name for name, owners in facts['union_owners'].items()
                              if any(facts['records'][parent] is record and fieldname == key
                                     for parent, fieldname in owners)]
                if len(candidates) != 1:
                    raise ValueError(f'unsupported anonymous union: {key}')
                nested = candidates[0]
            if '*' in kind:
                output.append(f' {level:02} {field_label} usage pointer.')
            elif nested in facts['records']:
                output.append(f' {level:02} {field_label}.')
                output.extend(members(facts['records'][nested], level + 1, path + key + '_'))
            elif re.fullmatch(r'char\s*\[\s*\d+\s*\]', kind):
                output.append(f' {level:02} {field_label} pic x({width}).')
            elif kind == 'double':
                output.append(f' {level:02} {field_label} usage float-long.')
            elif kind == 'float':
                output.append(f' {level:02} {field_label} usage float-short.')
            elif width in (1, 2, 4, 8):
                usage = {1:'binary-char', 2:'binary-short', 4:'binary-long', 8:'binary-double'}[width]
                signed = 'unsigned' if kind.startswith('uint') or kind in ('size_t', 'bool') else 'signed'
                output.append(f' {level:02} {field_label} usage {usage} {signed}.')
            else:
                raise ValueError(f'unrepresented field {key}: {field}')
            cursor = max(cursor, offset + width)
        if not union and cursor < record['size']:
            output.append(f' {level:02} filler pic x({record["size"]-cursor}).')
        return output
    for name, record in facts['records'].items():
        if not name.startswith(('thinkthen_complete_', 'thinkthen_cobol_')) or name.endswith('_data_v1'):
            # Union roots are represented in their owning structs.
            continue
        lines.append('01 ' + identifier(name) + ' based.')
        lines.extend(members(record, 2))
    for key, value in facts['constants'].items():
        if key.startswith('THINKTHEN_REQUEST_'):
            continue
        lines.append('78 ' + identifier(key.lower()) + '-k value ' + str(value) + '.')
    return '\n'.join(lines) + '\n'


def native_name(key):
    if key.startswith('EngineSettings'):
        key = key.replace('EngineSettings', 'Settings', 1).replace('max_estimated_input_tokens_total', 'input_token_budget')
    key = key.replace('RequestDefinition_', 'Definition_').replace('Authored_', '')
    key = key.replace('_member_', '_').replace('_properties_', '_')
    return ''.join(part[:1].upper() + part[1:] for part in key.split('_'))

def requests(definitions, native_facts, root):
    constants = native_facts["constants"]
    """Keep native COBOL storage and delegate canonical type conversion."""
    nodes = {key: (value["oneOf"][0] if isinstance(value, dict) and len(value.get("oneOf", [])) == 1 and "const" in value["oneOf"][0] else value) for key, value in definitions.items()}
    from templates.native_request import render as native_render
    metadata = {}
    native_render(definitions, metadata)
    layouts = {}
    pending = list(nodes)
    while pending:
        key = pending.pop(0)
        source = nodes[key] if isinstance(nodes[key], dict) else {}
        nodes[key] = source
        def child(value, path):
            if isinstance(value, dict) and '$ref' in value:
                return value['$ref'].removeprefix('#/$defs/')
            if path not in nodes:
                nodes[path] = value
                pending.append(path)
            return path
        if '$ref' in source:
            fields = [('const void *', 'value')]
        elif source.get('variants'):
            fields = [('uint64_t', 'kind'), ('const void *', 'value')]
        elif 'properties' in source:
            fields = [('uint64_t', 'reserved')]
            for member, value in source['properties'].items():
                child(value, key + '_member_' + member)
                fields.append(('const void *', 'm_' + member))
        elif source.get('type') == 'array':
            child(source.get('items', {}), key + '_item')
            fields = [('const void *const *', 'data'), ('uint64_t', 'len')]
        elif source.get('type') == 'object':
            child(source.get('additionalProperties') if isinstance(source.get('additionalProperties'), dict) else {}, key + '_entry')
            fields = [('const thinkthen_cobol_text *', 'keys'), ('const void *const *', 'values'), ('uint64_t', 'len')]
        elif 'const' in source or source.get('type') == 'null':
            fields = [('uint64_t', 'reserved')]
        elif source.get('primitive_variants') or source.get('oneOf') or source.get('anyOf') or isinstance(source.get('type'), list):
            alternatives = source.get('oneOf', source.get('anyOf', []))
            if source.get('primitive_variants'):
                alternatives = [source['primitive_schemas'][kind] for kind in source['primitive_variants']]
            elif isinstance(source.get('type'), list):
                alternatives = [{**source, 'type':kind} for kind in source['type']]
            for tag, value in zip(alternative_names(alternatives), alternatives):
                child(value, key + '_' + tag + '_Value')
            fields = [('uint64_t', 'kind'), ('const void *', 'value')]
        elif source.get('type') in ('integer', 'number', 'boolean'):
            native = 'double' if source['type'] == 'number' else 'uint64_t' if source['type'] == 'boolean' or source.get('format', '').startswith('u') else 'int64_t'
            fields = [(native, 'value')]
        elif source.get('type') == 'string':
            fields = [('const char *', 'data'), ('uint64_t', 'len')]
        else:
            if tuple(metadata['fields']['JsonValue']) != (('value', '*const thinkthen_complete_json_v1'),):
                raise ValueError('unsupported canonical arbitrary-value carrier')
            fields = [('const struct thinkthen_complete_json_v1 *', 'value')]
        layouts[key] = fields
    header = ['/* Generated from the canonical Request graph; do not edit. */', '#ifndef TT_COBOL_REQUESTS_GENERATED_H', '#define TT_COBOL_REQUESTS_GENERATED_H', '#include <stdint.h>', '#define THINKTHEN_COBOL_REPRESENTATION 1001', '#define THINKTHEN_COBOL_OVERFLOW 1002', '#define THINKTHEN_COBOL_TEXT_CAPACITY 8192', 'typedef struct thinkthen_cobol_text {const char *data; uint64_t len;} thinkthen_cobol_text;']
    for key, source in nodes.items():
        if source.get('variants'):
            for index, (target, _) in enumerate(source['variants'], 1):
                header.append(f'#define THINKTHEN_COBOL_{target.upper()}_KIND {index}')
        elif source.get('primitive_variants') or source.get('oneOf') or source.get('anyOf') or isinstance(source.get('type'), list):
            alternatives = source.get('oneOf', source.get('anyOf', []))
            if source.get('primitive_variants'):
                alternatives = [source['primitive_schemas'][kind] for kind in source['primitive_variants']]
            elif isinstance(source.get('type'), list):
                alternatives = [{**source, 'type': kind} for kind in source['type']]
            for index, tag in enumerate(alternative_names(alternatives), 1):
                header.append(f'#define THINKTHEN_COBOL_{key.upper()}_{tag.upper()}_KIND {index}')
    for key, fields in layouts.items():
        header += ['typedef struct thinkthen_cobol_' + key + ' {'] + [' '+kind+' '+member+';' for kind, member in fields] + ['} thinkthen_cobol_'+key+';']
    from templates.csharp import enum_values
    for function in enum_values(definitions['RequestFunction']):
        header.append(f'int TT_SESSION_{function.upper()}(const void *,const thinkthen_cobol_RequestCall *,void **);')
    header += [f'#define TT_VALUE_{kind}_KIND {constants["THINKTHEN_COMPLETE_JSON_" + kind + "_V1"]}' for kind in ('NULL','BOOLEAN','NUMBER','STRING','ARRAY','OBJECT')]
    header.append('typedef struct tt_value tt_value;')
    header.append('int TT_ENGINE_NEW(const thinkthen_cobol_EngineSettings *,void **);')
    header += ['int TT_PLAN(const void *,const char *,uint64_t,const thinkthen_cobol_RequestCall *,tt_value **);', 'int TT_SESSION_PUSH(const void *,const thinkthen_cobol_RequestSessionDescriptor *,uint32_t *);', 'int TT_SESSION_REQUEST(const void *engine,const thinkthen_cobol_Request *request,void **out);', '#endif']
    from templates.cobol_values import DECLARATIONS
    from templates.cobol_native import render, builders
    builder_header, builder_source = builders(definitions)
    from templates.cobol_blocking import render as blocking_render
    blocking_header, blocking_source = blocking_render(native_facts, root, definitions)
    header[-1:-1] = blocking_header.splitlines()
    header[-1:-1] = DECLARATIONS.splitlines()[1:] + builder_header
    code = render(definitions, nodes) + builder_source + '\n' + blocking_source
    names = {key: native_name(key) for key in nodes}
    if len(set(names.values())) != len(names):
        raise ValueError('colliding semantic COBOL native names')
    substitutions = {'thinkthen_cobol_' + key: 'thinkthen_cobol_' + value for key, value in names.items()}
    for key, value in names.items():
        for suffix in re.findall(r'\bTHINKTHEN_COBOL_' + re.escape(key.upper()) + r'(_[A-Z_]+_KIND|_KIND)\b', '\n'.join(header)):
            substitutions['THINKTHEN_COBOL_' + key.upper() + suffix] = 'THINKTHEN_COBOL_' + value.upper() + suffix
    def rename(text):
        return re.sub(r'\b[A-Za-z_][A-Za-z_0-9]*\b', lambda token: substitutions.get(token[0], token[0]), text)
    return rename('\n'.join(header) + '\n'), rename(code)


def outputs(root, definitions):
    from generate import graph, prepare
    schema = json.loads((root / 'specification/request.schema.json').read_text())
    schema['$defs']['Request'] = {key: value for key, value in schema.items() if key != '$defs'}
    definitions = prepare(graph(schema, ('Request', 'RequestSessionDescriptor', 'RequestReaderFailure', 'EngineSettings')))
    native_facts = abi_module(root).header_abi(root / 'libraries/c/include/thinkthen.h')
    header, serializer = requests(definitions, native_facts, root)
    package = root / 'libraries/cobol'
    # Measure generated Request declarations with the same compiler as native ABI.
    import tempfile
    with tempfile.TemporaryDirectory(prefix='cobol-layout-') as folder:
        path = Path(folder) / 'requests.h'
        # The ABI probe requires public functions and constants as well as records.
        path.write_text('#include ' + json.dumps(str(root / 'libraries/c/include/thinkthen.h')) + '\n' + header)
        facts = abi_module(root).header_abi(path)
    all_copybooks = copybook(facts)
    from templates.csharp import enum_values
    result_schema = json.loads((root / 'specification/result.schema.json').read_text())
    functions = enum_values(definitions['RequestFunction'])
    manifest = {'generated': True, 'blocking': ['TT_' + function.upper() for function in functions],
                'streaming': ['TT_SESSION_' + function.upper() for function in functions],
                'answer_records': {function: 'tt-n-cobol-Answer' + function.title() + 'Result' for function in functions},
                'settings': 'TT_ENGINE_NEW', 'preview': 'TT_PREVIEW',
                'preview_fields': list(result_schema['$defs']['plan']['properties']),
                'native_constructor': 'thinkthen_session_new_typed_with_surface',
                'native_wait': 'thinkthen_session_next_packet',
                'native_views': 'thinkthen_session_result_view'}
    return {'src/tt_requests_generated.h': header, 'src/tt_requests_generated.c': serializer,
            'copybooks/tt-native-generated.cpy': all_copybooks,
            'native-helpers.json': json.dumps(manifest, indent=2) + '\n'}
