"""DuckDB receipt types derived from the canonical native result schema."""
import json
import re

ROOTS = ('completeDecide', 'completeChoose', 'completeTag', 'completeScore',
         'completeFilter', 'completeRank', 'completeAnnotation', 'completeRecognition',
         'completeFind', 'completeRelation', 'completeFacts', 'completeplan', 'completeError', 'completesessionObservation')
NAMES = {'duckdbObservedInput': 'observed_input', 'completeplan': 'plan', 'completesessionObservation': 'event',
         'completesessionQuestionDetail': 'question_detail',
         'completesessionObservedRow': 'observed_row',
         'completeDecide': 'decision', 'completeChoose': 'choice',
         'completeTag': 'tags', 'completeScore': 'score', 'completeFilter': 'filtered',
         'completeRank': 'ranked', 'completeAnnotation': 'annotation',
         'completeRecognition': 'recognized', 'completeFind': 'found',
         'completeRelation': 'related', 'completeFacts': 'facts',
         'completeError': 'error', 'completeMeta': 'metadata'}


def name(key):
    key = NAMES.get(key, key.removeprefix('complete'))
    return 'Receipt_' + re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', key).lower()


class Composite:
    def __init__(self, definitions):
        self.definitions = definitions
        self.types = {}
        self.active = set()
        self.required = {}

    def alternatives(self, schema):
        if not isinstance(schema, dict):
            return [schema]
        if '$ref' in schema:
            return self.alternatives(self.definitions[schema['$ref'].split('/')[-1]])
        kinds = schema.get('type')
        if isinstance(kinds, list) and len(kinds) > 1:
            return [{**schema, 'type': kind} for kind in kinds]
        choices = schema.get('oneOf', schema.get('anyOf'))
        if choices and not all(set(item) <= {'required', 'not'} for item in choices):
            common = schema.get('properties', {})
            choices = [part for child in choices for part in self.alternatives(child)]
            return [{**part, 'properties': {**common, **part.get('properties', {})}}
                    if common and isinstance(part, dict) else part for part in choices]
        return [schema]

    def shape(self, schema, key):
        if not isinstance(schema, dict) or not schema:
            return ('JSON', 'json')
        if '$ref' in schema:
            ref = schema['$ref'].split('/')[-1]
            return self.shape(self.definitions[ref], ref)
        choices = self.alternatives(schema)
        choices = [choice for choice in choices if not isinstance(choice, dict)
                   or choice.get('type') != 'null']
        if not choices:
            return ('JSON', 'json')
        if len(choices) > 1:
            if any(not isinstance(choice, dict) or not choice for choice in choices):
                return ('JSON', 'json')
            objects = [choice for choice in choices if 'properties' in choice]
            if len(objects) == len(choices):
                fields = {}
                for choice in choices:
                    for member, child in choice['properties'].items():
                        if member in fields and fields[member] != child:
                            fields[member] = {'anyOf': [fields[member], child]}
                        else:
                            fields[member] = child
                required = set.intersection(*(set(choice.get('required', [])) for choice in choices))
                return self.object(key, fields, required=required)
            groups = {}
            for child in choices:
                kinds = child.get('type', [])
                kinds = [kinds] if isinstance(kinds, str) else kinds
                for kind in kinds:
                    if kind != 'null':
                        groups.setdefault(kind, []).append(child)
            if len(groups) == 1:
                kind, children = next(iter(groups.items()))
                if kind == 'array':
                    items = [child.get('items', {}) for child in children]
                    return self.shape({'type': 'array', 'items': {'anyOf': items}}, key)
                return self.shape(max(children, key=lambda child: child.get('maximum', 0)), key)
            fields = {kind + '_value': {'anyOf': children} for kind, children in groups.items()}
            return self.object(key, fields, union=True)
        schema = choices[0]
        kinds = schema.get('type', [])
        kinds = [kinds] if isinstance(kinds, str) else kinds
        kinds = [kind for kind in kinds if kind != 'null']
        if 'properties' in schema:
            return self.object(key, schema['properties'], required=schema.get('required', []))
        if 'object' in kinds and isinstance(schema.get('additionalProperties'), dict):
            fields = {'name': {'type': 'string'}, 'value': schema['additionalProperties']}
            sql, descriptor = self.object(key + '_entry', fields, mapping=True)
            return (sql + '[]', ('mapping', descriptor))
        if 'array' in kinds:
            sql, descriptor = self.shape(schema.get('items', {}), key + '_item')
            return (sql + '[]', ('array', descriptor))
        if 'integer' in kinds and schema.get('maximum', 0) > 9223372036854775807:
            return ('UBIGINT', 'unsigned')
        for kind, sql in [('string', 'VARCHAR'), ('boolean', 'BOOLEAN'),
                          ('integer', 'BIGINT'), ('number', 'DOUBLE')]:
            if kind in kinds or (kind == 'string' and isinstance(schema.get('const'), str)):
                return (sql, kind)
        return ('JSON', 'json')

    def object(self, key, fields, union=False, mapping=False, required=()):
        sql = name(key)
        if sql in self.types:
            return (sql, ('object', sql))
        if sql in self.active:
            raise ValueError('recursive SQL composite: ' + sql)
        self.active.add(sql)
        shaped = {member: self.shape(child, key + '_' + member)
                  for member, child in sorted(fields.items())}
        self.active.remove(sql)
        self.types[sql] = (shaped, union, mapping)
        self.required[sql] = set(required)
        return (sql, ('object', sql))


def logical(shape):
    sql, descriptor = shape
    if isinstance(descriptor, tuple):
        mode, child = descriptor
        if mode == 'object':
            return child + 'Type()'
        return 'LogicalType::LIST(' + logical_descriptor(child) + ')'
    return 'LogicalType::JSON()' if sql == 'JSON' else 'LogicalType::' + sql


def logical_descriptor(descriptor):
    if isinstance(descriptor, tuple):
        mode, child = descriptor
        return child + 'Type()' if mode == 'object' else 'LogicalType::LIST(' + logical_descriptor(child) + ')'
    sql = {'json': 'JSON()', 'unsigned': 'UBIGINT', 'integer': 'BIGINT',
           'number': 'DOUBLE', 'string': 'VARCHAR', 'boolean': 'BOOLEAN'}[descriptor]
    return 'LogicalType::' + sql


def convert(descriptor, value):
    if isinstance(descriptor, tuple):
        mode, child = descriptor
        if mode == 'object':
            return child + 'Value(' + value + ')'
        if mode == 'mapping':
            fields = MODEL.types[child[1]][0]
            result = convert(fields['value'][1], 'item')
            values = 'Value(key), ' + result
            if fields['value'][1] == 'json':
                values += ', Value::BOOLEAN(true)'
            return 'ReceiptMapping(' + value + ', ' + child[1] + 'Type(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(' + child[1] + 'Type(), {' + values + '}); })'
        child_type = logical_descriptor(child)
        converted = convert(child, 'item')
        return 'ReceiptList(' + value + ', ' + child_type + ', [](ReceiptJSON *item) { return ' + converted + '; })'
    return 'ReceiptLeaf(' + value + ', ' + logical_descriptor(descriptor) + ')'


def render(schema):
    global MODEL
    model = model_for(schema)
    MODEL = model
    for root in ROOTS:
        model.shape({'$ref': '#/$defs/' + root}, root)
    header = ['// Generated from the canonical native result schema; do not edit.', '#pragma once']
    lines = list(header)
    chunks = []
    for kind, (fields, union, mapping) in model.types.items():
        members = []
        values = []
        for field, shape in fields.items():
            value = 'value' if union else 'ReceiptMember(value, ' + json.dumps(field) + ')'
            members.append('{' + json.dumps(field) + ', ' + logical(shape) + '}')
            values.append(convert(shape[1], value))
            if shape[1] == 'json':
                members.append('{' + json.dumps(field + '_present') + ', LogicalType::BOOLEAN}')
                values.append('Value::BOOLEAN(' + value + ' != nullptr)')
        lines += ['inline LogicalType ' + kind + 'Type() {',
                  'return LogicalType::STRUCT({' + ', '.join(members) + '});', '}',
                  'inline Value ' + kind + 'Value(ReceiptJSON *value) {',
                  'if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(' + kind + 'Type()); }']
        if not union:
            lines += ['if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(' + kind + 'Type()); }']
        if mapping:
            lines += ['return Value::STRUCT(' + kind + 'Type(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ' + values[1] + '});']
        else:
            lines += ['return Value::STRUCT(' + kind + 'Type(), {' + ', '.join(values) + '});']
        lines += ['}']
        if len(lines) >= 350:
            chunks.append('\n'.join(lines) + '\n')
            lines = list(header)
    if len(lines) > len(header):
        chunks.append('\n'.join(lines) + '\n')
    return chunks


def model_for(schema):
    import copy
    definitions = copy.deepcopy(schema['$defs'])
    # Native observation snapshots retain ancillary source/image inputs beside
    # the canonical original-only serialization. These fields belong to this
    # SQL projection, not to the canonical native result schema.
    definitions['duckdbObservedInput'] = {
        'type': 'object', 'required': ['input', 'images'], 'properties': {
            'input': True, 'source': {'$ref': '#/$defs/completePhysicalSource'},
            'images': {'type': 'array', 'items': {'$ref': '#/$defs/completeImage'}}}}
    detail = definitions['completesessionQuestionDetail']['properties']
    question = definitions['completesessionObservation']['oneOf'][0]['properties']
    for field in ('answer_id', 'failure_id', 'question_sources', 'observations', 'raw_pick'):
        question[field] = detail[field]
    question['inputs'] = {'type': 'array', 'items': {'$ref': '#/$defs/duckdbObservedInput'}}
    question['author'] = {'type': 'object', 'properties': {
        'name': {'type': 'string'}, 'wording_version': {'type': 'string'}}}
    return Composite(definitions)


if __name__ == '__main__':
    from pathlib import Path
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[3]
    schema = json.loads((root / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())
    output = root / 'databases/duckdb/cpp/src/receipt_types_generated.hpp'
    chunks = render(schema)
    generated = {output.with_name(f'receipt_types_generated_{index+1}.hpp'): chunk
                 for index, chunk in enumerate(chunks)}
    generated[output] = '// Generated canonical receipt fields; do not edit.\n#pragma once\n' + ''.join(
        f'#include "{path.name}"\n' for path in generated)
    for path, contents in generated.items():
        if args.check:
            if path.read_text() != contents:
                raise SystemExit('DuckDB native receipt types need generation')
        else:
            path.write_text(contents)

