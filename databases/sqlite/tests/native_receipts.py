"""Read complete receipts exclusively through named typed SQL projections."""
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'sdlc/generators/results/templates'))
from postgresql import Composite, ROOTS, sql_schema

SCHEMA = json.loads((ROOT / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())
SCHEMA = sql_schema(SCHEMA)
MODEL = Composite(SCHEMA['$defs'])
for name in (*ROOTS, 'completesessionQuestionDetail'):
    MODEL.shape({'$ref': '#/$defs/' + name}, name)
TYPES = {}


def shape(descriptor, owner, field):
    if isinstance(descriptor, str):
        return descriptor
    mode, child = descriptor[:2]
    if mode == 'object':
        return mode, child.removeprefix('thinkthen_')
    if mode == 'mapping':
        return mode, child[1].removeprefix('thinkthen_')
    if isinstance(child, tuple) and child[0] == 'object':
        return mode, child[1].removeprefix('thinkthen_')
    name = owner + '_' + field
    TYPES[name] = ({'value': shape(child, name, 'value')}, True)
    return mode, name


for name, (fields, union, _) in MODEL.types.items():
    name = name.removeprefix('thinkthen_')
    TYPES[name] = ({field: shape(held[1], name, field) for field, held in fields.items()}, union)
TYPES['record_order'] = ({'ordinal': 'integer', 'result_index': 'integer', 'completed': 'boolean'}, False)
TYPES['find_selection'] = ({'selected': 'integer'}, False)
ROOT_KIND = dict(zip(('decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate'),
                     ('decision', 'choice', 'tags', 'score', 'filtered', 'ranked', 'found', 'annotation', 'recognized', 'related')))


class Read:
    def __init__(self, db, verb, question, inputs, settings):
        if verb in ('details', 'details_images'):
            parsed = json.loads(question) if question.lstrip().startswith('{') else {'decide':question}
            verb = next(name for name in ROOT_KIND if name in parsed)
        self.db, self.verb = db, verb + '_details'
        self.fields = {'request_question': question, 'settings': settings or '{}'}
        if isinstance(inputs, bytes):
            self.fields['text'] = inputs
            inputs = {}
        for name in ('records', 'incremental', 'attempts', 'cancelled'):
            if name in inputs:
                self.fields['request_records' if name == 'records' else name] = json.dumps(inputs[name], ensure_ascii=False, separators=(',', ':')) if name == 'records' else None if inputs[name] is None else int(inputs[name])
        if 'files' in inputs:
            file = inputs['files']
            self.fields['request_paths'] = json.dumps(file['paths'])
            if 'format' in file:
                self.fields['request_format'] = json.dumps(file['format']) if isinstance(file['format'],(dict,list)) else file['format']
            options = file.get('options', {})
            if 'media' in options:
                self.fields['request_media'] = options['media']
            for name, value in options.get('reading', {}).items():
                self.fields['request_' + name] = value
        for name, value in inputs.get('reading', {}).items():
            key = {'fields': 'record_fields', 'context': 'record_context_field', 'options': 'record_options_field',
                   'context_schema': 'record_context_schema', 'seed_spans': 'record_seed_spans_field', 'examples': 'record_examples_field'}[name]
            self.fields[key] = json.dumps(value) if name in ('fields','context_schema') else value
        self.cache = {}

    def rows(self, kind):
        if kind in self.cache:
            return self.cache[kind]
        columns = [kind + '_receipt_index']
        for name, descriptor in TYPES[kind][0].items():
            base = 'question_definition_name' if kind == 'question' and name == 'name' else kind + '_' + name
            columns.append(base + '_present')
            if isinstance(descriptor, str):
                if descriptor == 'json':
                    columns.append(base + '_type')
                columns.append(base)
            elif descriptor[0] == 'object':
                columns.append(base + '_index')
            else:
                columns += [base + '_first_index', base + '_count']
        sql = 'SELECT ' + ','.join('"' + column + '"' for column in columns)
        sql += ' FROM thinkthen_' + self.verb + ' WHERE '
        sql += ' AND '.join(name + '=?' for name in self.fields) + ' AND projection=?'
        rows = self.db.execute(sql, [*self.fields.values(), kind]).fetchall()
        self.cache[kind] = [dict(zip(columns, row)) for row in rows]
        return self.cache[kind]

    def object(self, kind, index):
        rows = self.rows(kind)
        row = next(row for row in rows if row[kind + '_receipt_index'] == index)
        fields, union = TYPES[kind]
        output = {}
        for name, descriptor in fields.items():
            base = 'question_definition_name' if kind == 'question' and name == 'name' else kind + '_' + name
            assert row[base + '_present'] in (0, 1)
            if not row[base + '_present']:
                continue
            if isinstance(descriptor, str):
                value = row[base]
                if descriptor == 'json':
                    data_type = row[base + '_type']
                    if data_type in ('array', 'object'):
                        value = json.loads(value)
                    elif data_type == 'boolean':
                        value = bool(value)
                    elif data_type == 'number' and isinstance(value,str):
                        value = json.loads(value)
                elif value is not None and descriptor in ('integer', 'unsigned'):
                    value = int(value)
                elif value is not None and descriptor == 'boolean':
                    value = bool(value)
            elif descriptor[0] == 'object':
                held = row[base + '_index']
                value = None if held is None else self.object(descriptor[1], held)
            else:
                first, count = row[base + '_first_index'], row[base + '_count']
                value = None if first is None else [self.object(descriptor[1], at) for at in range(first, first + count)]
                if value is not None and descriptor[0] == 'mapping':
                    value = {row['name']: row['value'] for row in value}
            if not union or value is not None:
                output[name] = value
        return next(iter(output.values()), None) if union else output


def call(db, verb, question, inputs, settings=None):
    if isinstance(inputs, str):
        inputs = json.loads(inputs)
    reader = Read(db, verb, question, inputs, settings)
    facts = reader.rows('facts')
    errors = reader.rows('error')
    root = ROOT_KIND[verb]
    values = [reader.object(root, row[root + '_receipt_index']) for row in reader.rows(root)]
    if verb == 'relate' and values:
        aggregate={**values[0],'value':[]}
        for value in values:
            assert {k:v for k,v in value.items() if k!='value'} == {k:v for k,v in aggregate.items() if k!='value'},value
            aggregate['value'].extend(value['value'])
        values=[aggregate]
    native = {}
    if facts:
        native['facts'] = reader.object('facts', facts[0]['facts_receipt_index'])
    if errors:
        native['error'] = reader.object('error', errors[0]['error_receipt_index'])
    else:
        native['value'] = values[0] if verb in ('find', 'relate') and values else values
    result = {'native': native,
              'observations': [reader.object('event', row['event_receipt_index']) for row in reader.rows('event')]}
    order = reader.rows('record_order')
    if order:
        result['ordinals'] = [reader.object('record_order', row['record_order_receipt_index'])['ordinal'] for row in order]
    if errors and values:
        result['completed'] = values
    if verb == 'find':
        selection = reader.rows('find_selection')
        if selection:
            result['selection'] = reader.object('find_selection', selection[0]['find_selection_receipt_index'])['selected']
    return result


def ordinary(db, question, text, settings=None):
    """An ordinary judgment retains its complete scalar metadata in native columns."""
    return ordinary_read(db, 'details', question, {'text': text}, settings)


def ordinary_images(db, question, images, text=None, settings=None):
    return ordinary_read(db, 'details_images', question, {'text': images, 'image_text': text}, settings)


def ordinary_read(db, verb, question, inputs, settings):
    reader = Read(db, verb, question, {}, settings)
    reader.fields = {'question': question, **inputs, 'settings': settings or '{}'}
    sql = 'SELECT value FROM thinkthen_' + reader.verb + ' WHERE ' + ' AND '.join(key + '=?' for key in reader.fields)
    db.execute(sql, list(reader.fields.values())).fetchall()
    details = reader.rows('question_detail')
    if not details:
        errors = reader.rows('error')
        if errors:
            return reader.object('error', errors[0]['error_receipt_index'])
        return None
    detail = reader.object('question_detail', details[0]['question_detail_receipt_index'])
    question = detail['question']
    kind = ROOT_KIND.get(question.get('verb'), 'decision')
    rows = reader.rows(kind)
    result = reader.object(kind, rows[0][kind + '_receipt_index'])
    observed = [reader.object('event', row['event_receipt_index']) for row in reader.rows('event')]
    bare = next((event['value'] for event in observed if event['kind'] == 'row'), result['value'])
    result['value'] = bare
    return result
