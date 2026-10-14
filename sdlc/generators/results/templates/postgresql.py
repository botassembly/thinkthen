"""Native PostgreSQL composites and projections from the canonical result schema."""
import json
import re

ROOTS = ('completeDecide', 'completeChoose', 'completeTag', 'completeScore',
         'completeFilter', 'completeRank', 'completeAnnotation', 'completeRecognition',
         'completeFind', 'completeRelation', 'completeFacts', 'completeError', 'completesessionObservation')
NAMES = {'completesessionObservation': 'event',
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
    return 'thinkthen_' + re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', key).lower()


class Composite:
    def __init__(self, definitions):
        self.definitions = definitions
        self.types = {}
        self.active = set()

    def alternatives(self, schema):
        if not isinstance(schema, dict):
            return [schema]
        if '$ref' in schema:
            return self.alternatives(self.definitions[schema['$ref'].split('/')[-1]])
        choices = schema.get('oneOf', schema.get('anyOf'))
        if choices and not all(set(item) <= {'required', 'not'} for item in choices):
            common = schema.get('properties', {})
            choices = [part for child in choices for part in self.alternatives(child)]
            return [{**part, 'properties': {**common, **part.get('properties', {})}}
                    if common and isinstance(part, dict) else part for part in choices]
        return [schema]

    def shape(self, schema, key):
        if not isinstance(schema, dict) or not schema:
            return ('jsonb', 'json')
        if '$ref' in schema:
            ref = schema['$ref'].split('/')[-1]
            return self.shape(self.definitions[ref], ref)
        choices = self.alternatives(schema)
        choices = [choice for choice in choices if not isinstance(choice, dict)
                   or choice.get('type') != 'null']
        if not choices:
            return ('jsonb', 'json')
        if len(choices) > 1:
            if any(not isinstance(choice, dict) or not choice for choice in choices):
                return ('jsonb', 'json')
            objects = [choice for choice in choices if 'properties' in choice]
            if len(objects) == len(choices):
                fields = {}
                for choice in choices:
                    for member, child in choice['properties'].items():
                        if member in fields and fields[member] != child:
                            fields[member] = {'anyOf': [fields[member], child]}
                        else:
                            fields[member] = child
                return self.object(key, fields)
            shapes = [self.shape(child, key + '_value') for child in choices]
            if all(shape == shapes[0] for shape in shapes):
                return shapes[0]
            # Different native values retain named columns, rather than numbered arms.
            fields = {}
            for child in choices:
                kinds = child.get('type', [])
                kinds = [kinds] if isinstance(kinds, str) else kinds
                for kind in kinds:
                    if kind != 'null':
                        fields[kind + '_value'] = child
            return self.object(key, fields, union=True)
        schema = choices[0]
        kinds = schema.get('type', [])
        kinds = [kinds] if isinstance(kinds, str) else kinds
        kinds = [kind for kind in kinds if kind != 'null']
        if 'properties' in schema:
            return self.object(key, schema['properties'])
        if 'object' in kinds and isinstance(schema.get('additionalProperties'), dict):
            fields = {'name': {'type': 'string'}, 'value': schema['additionalProperties']}
            sql, descriptor = self.object(key + '_entry', fields, mapping=True)
            return (sql + '[]', ('mapping', descriptor))
        if 'array' in kinds:
            sql, descriptor = self.shape(schema.get('items', {}), key + '_item')
            return (sql + '[]', ('array', descriptor))
        for kind, sql in [('string', 'text'), ('boolean', 'boolean'),
                          ('integer', 'bigint'), ('number', 'double precision')]:
            if kind in kinds or (kind == 'string' and isinstance(schema.get('const'), str)):
                return (sql, kind)
        return ('jsonb', 'json')

    def object(self, key, fields, union=False, mapping=False):
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
        return (sql, ('object', sql))


def render(schema):
    model = Composite(schema['$defs'])
    for root in ROOTS:
        model.shape({'$ref': '#/$defs/' + root}, root)
    sql = []
    arms = []
    for kind, (fields, union, mapping) in model.types.items():
        sql.append('CREATE TYPE ' + kind + ' AS (' + ', '.join(
            '"' + field + '" ' + shape[0] + (', "' + field + '_present" boolean'
                                                if shape[1] == 'json' else '')
            for field, shape in fields.items()) + ');')
        arms += [json.dumps(kind) + ' => {', 'let mut tuple = super::output::tuple(kind);']
        for field, (sql_type, descriptor) in fields.items():
            if union:
                value = 'Some(value)'
            else:
                value = 'value.get(' + json.dumps(field) + ')'
            arms.append('let held = ' + value + ';')
            if descriptor == 'json':
                arms += ['tuple.set_by_name(' + json.dumps(field + '_present') + ', held.is_some())',
                         '.map_err(|_| crate::call::defect("generated result presence is unavailable"))?;']
            expression, rust_type = conversion(descriptor, 'held', kind)
            arms += ['let converted: Option<' + rust_type + '> = ' + expression + ';',
                     'tuple.set_by_name(' + json.dumps(field) + ', converted)',
                     '.map_err(|_| crate::call::defect("generated result field is unavailable"))?;']
        arms += ['Ok(tuple)', '},']
    return '\n'.join([
        '// Generated from the canonical Rust result schema; do not edit.',
        'use pgrx::prelude::*;', 'use serde_json::Value;',
        'pgrx::extension_sql!(r#"', *sql, '"#, name = "native_generated_types");',
        '#[expect(clippy::too_many_lines, reason = "generated schema projections")]',
        'pub(super) fn convert(kind: &str, value: &Value) -> Result<PgHeapTuple<\'static, AllocatedByRust>, thinkthen::Error> {',
        'match kind {', *arms,
        '_ => Err(crate::call::defect("unknown generated result type")),', '}', '}',
    ]) + '\n'


def conversion(descriptor, value, kind):
    if descriptor == 'json':
        return (value + '.map(|value| pgrx::datum::JsonB(value.clone()))', 'pgrx::datum::JsonB')
    if descriptor in ('string', 'integer', 'number', 'boolean'):
        method, rust = {'string': ('as_str', 'String'), 'integer': ('as_i64', 'i64'),
                        'number': ('as_f64', 'f64'), 'boolean': ('as_bool', 'bool')}[descriptor]
        expr = value + '.and_then(Value::' + method + ')'
        return (expr + ('.map(str::to_owned)' if descriptor == 'string' else ''), rust)
    mode, child = descriptor
    if mode == 'object':
        return (value + '.filter(|value| !value.is_null()).map(|value| convert(' +
                json.dumps(child) + ', value)).transpose()?', "PgHeapTuple<'static, AllocatedByRust>")
    if mode == 'array':
        expression, rust = conversion(child, 'Some(value)', kind)
        result = expression[:-1] if expression.endswith('?') else 'Ok(' + expression + ')'
        return (value + '.and_then(Value::as_array).map(|values| values.iter().map(|value| '
                + result + ').collect::<Result<Vec<_>, thinkthen::Error>>()).transpose()?',
                'Vec<Option<' + rust + '>>')
    if mode == 'mapping':
        child_type = child[1]
        return (value + '.and_then(Value::as_object).map(|values| values.iter().map(|(name, value)| {'
                ' let entry = serde_json::json!({"name": name, "value": value}); '
                'convert(' + json.dumps(child_type) + ', &entry) }).collect::<Result<Vec<_>, thinkthen::Error>>()).transpose()?',
                "Vec<PgHeapTuple<'static, AllocatedByRust>>")
    raise ValueError(descriptor)


def main():
    import argparse
    from pathlib import Path
    import subprocess
    parser = argparse.ArgumentParser()
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[4]
    schema = json.loads((root / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())
    target = root / 'databases/postgresql/src/native/results_generated.rs'
    result = render(schema)
    result = subprocess.run(['rustfmt', '--edition', '2024', '--emit', 'stdout'], input=result,
                            text=True, capture_output=True, check=True).stdout
    if args.check:
        if target.read_text() != result:
            raise SystemExit('PostgreSQL result generation is stale')
    else:
        target.write_text(result)


if __name__ == '__main__':
    main()
