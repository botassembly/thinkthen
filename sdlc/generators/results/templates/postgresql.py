"""Native PostgreSQL composites and projections from the canonical result schema."""
import json
import re

ROOTS = ('completeDecide', 'completeChoose', 'completeTag', 'completeScore',
         'completeFilter', 'completeRank', 'completeAnnotation', 'completeRecognition',
         'completeFind', 'completeRelation', 'completeFacts', 'completeplan', 'completeError', 'completesessionObservation')
NAMES = {'completeplan': 'plan', 'completesessionObservation': 'event',
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
            return self.object(key, schema['properties'])
        if 'object' in kinds and isinstance(schema.get('additionalProperties'), dict):
            fields = {'name': {'type': 'string'}, 'value': schema['additionalProperties']}
            sql, descriptor = self.object(key + '_entry', fields, mapping=True)
            return (sql + '[]', ('mapping', descriptor))
        if 'array' in kinds:
            sql, descriptor = self.shape(schema.get('items', {}), key + '_item')
            return (sql + '[]', ('array', descriptor))
        if 'integer' in kinds and schema.get('maximum', 0) > 9223372036854775807:
            return ('numeric', 'unsigned')
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
                     'if let Some(converted) = converted {',
                     'tuple.set_by_name(' + json.dumps(field) + ', converted)',
                     '.map_err(|_| crate::call::defect("generated result field is unavailable"))?;', '}']
        arms += ['Ok(tuple)', '},']
    return '\n'.join([
        '// Generated from the canonical Rust result schema; do not edit.',
        'use pgrx::prelude::*;', 'use serde_json::Value;',
        'pgrx::extension_sql!(r#"', *sql, '"#, name = "native_generated_types");',
        '#[expect(clippy::too_many_lines, clippy::excessive_nesting, clippy::cognitive_complexity, reason = "generated schema projections")]',
        'pub(super) fn convert(kind: &str, value: &Value) -> Result<PgHeapTuple<\'static, AllocatedByRust>, thinkthen::Error> {',
        'match kind {', *arms,
        '_ => Err(crate::call::defect("unknown generated result type")),', '}', '}',
    ]) + '\n'


def conversion(descriptor, value, kind):
    if descriptor == 'json':
        return (value + '.map(|value| pgrx::datum::JsonB(value.clone()))', 'pgrx::datum::JsonB')
    if descriptor == 'unsigned':
        return (value + '.and_then(Value::as_u64).map(pgrx::AnyNumeric::from)', 'pgrx::AnyNumeric')
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
        elements = 'values.iter().map(|value| ' + result + ').collect::<Result<Vec<_>, thinkthen::Error>>()'
        if isinstance(child, tuple) and child[0] == 'object':
            array = 'crate::ffi::composite_array(' + json.dumps(child[1]) + ', ' + elements + '?)'
            return (value + '.and_then(Value::as_array).map(|values| ' + array + ').transpose()?',
                    "Array<'static, PgHeapTuple<'static, AllocatedByRust>>")
        return (value + '.and_then(Value::as_array).map(|values| ' + elements + ').transpose()?',
                'Vec<Option<' + rust + '>>')
    if mode == 'mapping':
        child_type = child[1]
        elements = ('values.iter().map(|(name, value)| {'
                    ' let entry = serde_json::json!({"name": name, "value": value}); '
                    'convert(' + json.dumps(child_type) + ', &entry).map(Some) })'
                    '.collect::<Result<Vec<_>, thinkthen::Error>>()?')
        return (value + '.and_then(Value::as_object).map(|values| '
                'crate::ffi::composite_array(' + json.dumps(child_type) + ', ' + elements + ')).transpose()?',
                "Array<'static, PgHeapTuple<'static, AllocatedByRust>>")
    raise ValueError(descriptor)



def render_functions():
    calls = (
        ('decide', 'bool', 'Decisions', 'decision', 'crate::answer_value(row.result().value())'),
        ('choose', 'String', 'Choices', 'choice', 'row.result().value().map(str::to_owned)'),
        ('tag', 'Vec<String>', 'Tags', 'tags', 'Some(row.result().value().to_vec())'),
        ('score', 'f64', 'Scores', 'score', 'Some(row.result().value())'),
        ('filter', 'JsonB', 'Filtered', 'filtered', 'row.result().value().then(|| output::original(row.original()))'),
        ('rank', 'i64', 'Ranked', 'ranked', 'Some(output::index(row.ordinal()))'),
        ('annotate', 'JsonB', 'Annotations', 'annotation', 'Some(crate::jsonb(&row.result().value_json().or_raise()))'),
        ('recognize', 'Vec<pgrx::composite_type!(\'static, "thinkthen_name")>', 'Recognized', 'recognized', 'Some(output::names(row.result().value()))'),
    )
    result = ["// Generated native PostgreSQL declarations; do not edit.",
              "use super::output;", "use crate::call::{self, OrRaise as _};",
              "use pgrx::prelude::*;", "use pgrx::datum::JsonB;",
              "use thinkthen::RequestValue;"]
    for verb, kind, variant, reading, expression in calls:
        extra = ""
        if verb == 'rank':
            extra = """Some(RequestValue::SetRanked(rows)) => rows.into_iter().map(|row| {
                let reading = output::reading("thinkthen_ranked", &serde_json::to_value(&row)
                    .map_err(|_| call::defect("native result could not be projected")).or_raise());
                (Some(output::index(row.ordinal())), Some(output::index(row.ordinal())), Some(reading),
                    facts.as_ref().map(output::facts), None, output::events(&observations))
            }).collect::<Vec<_>>(),"""
        result.append(f'''crate::descriptions::describe! {{
"Read ordered native SQL records with typed controls; return typed answers, full result readings, final facts, failures and observations. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_{verb}", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn {verb}_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (
    name!(ordinal, Option<i64>), name!(value, Option<{kind}>),
    name!(result, Option<pgrx::composite_type!('static, "thinkthen_{reading}")>),
    name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
    name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
    name!(observations, Vec<pgrx::composite_type!('static, "thinkthen_event")>),
)> {{
    call::guarded(|| {{
        let (Some(question), Some(records)) = (question, records) else {{
            return TableIterator::new(Vec::new());
        }};
        let output::Projection {{ value, facts, failure, observations }} = output::outcome(
            super::execute("{verb}", question, records, reading),
        );
        let mut rows = match value {{
            Some(RequestValue::{variant}(rows)) => rows.into_iter().map(|row| {{
                let reading = output::reading("thinkthen_{reading}", &serde_json::to_value(&row)
                    .map_err(|_| call::defect("native result could not be projected")).or_raise());
                (Some(output::index(row.ordinal())), {expression}, Some(reading),
                    facts.as_ref().map(output::facts), None, output::events(&observations))
            }}).collect::<Vec<_>>(),
            {extra}
            None => Vec::new(),
            _ => call::raise(crate::request::wrong_result()),
        }};
        if failure.is_some() || rows.is_empty() {{
            rows.push((None, None, None, facts.as_ref().map(output::facts),
                failure.as_ref().map(output::failure), output::events(&observations)));
        }}
        TableIterator::new(rows)
    }})
}}
}}
''')
    return '\n'.join(result)

def main():
    import argparse
    from pathlib import Path
    import subprocess
    parser = argparse.ArgumentParser()
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[4]
    schema = json.loads((root / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())
    outputs = {'results_generated.rs': render(schema), 'functions_generated.rs': render_functions()}
    for name, source in outputs.items():
        target = root / 'databases/postgresql/src/native' / name
        result = subprocess.run(['rustfmt', '--edition', '2024', '--emit', 'stdout'], input=source,
                                text=True, capture_output=True, check=True).stdout
        if args.check:
            if target.read_text() != result:
                raise SystemExit('PostgreSQL result generation is stale: ' + name)
        else:
            target.write_text(result)



if __name__ == '__main__':
    main()
