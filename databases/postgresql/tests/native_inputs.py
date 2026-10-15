"""Project fixture descriptors to actual named SQL records and readings."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

class AdmissionError(Exception):
    def __init__(self, error):
        self.error = error

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "conformance/children"))
from children import child_env

def literal(value):
    if value is None:
        return 'NULL'
    if isinstance(value, bool):
        return 'true' if value else 'false'
    if isinstance(value, (int, float)):
        return str(value)
    return "'" + str(value).replace("'", "''") + "'"

def array(values, kind, render=literal):
    return 'ARRAY[' + ','.join(render(value) for value in values) + ']::' + kind + '[]'

def composite(value, kind, fields):
    return 'ROW(' + ','.join(literal(value.get(key)) for key in fields) + ')::' + kind

def options(values):
    if isinstance(values, dict):
        values = [{'name': name, 'description': description} for name, description in values.items()]
    return array(values, 'thinkthen_option', lambda value: 'ROW(' + literal(value if isinstance(value, str) else value['name']) + ',' +
                 ('NULL' if isinstance(value, str) or 'description' not in value else literal(json.dumps(value['description'])) + '::json') + ')::thinkthen_option')

def seeds(values):
    return array(values, 'thinkthen_seed', lambda value: composite(value, 'thinkthen_seed', ('start', 'end', 'kind')))

def examples(values):
    def one(value):
        if isinstance(value, str):
            value = {'brackets': value}
        entities = array(value['entities'], 'thinkthen_example_entity', lambda entity: composite(entity, 'thinkthen_example_entity', ('start', 'end', 'kind'))) if 'entities' in value else 'NULL'
        kinds = array(value['kinds'], 'text') if 'kinds' in value else 'NULL'
        return 'ROW(' + ','.join((literal(value.get('brackets')), literal(value.get('text')), entities, kinds)) + ')::thinkthen_example'
    return array(values, 'thinkthen_example', one)

def record(value):
    arguments = []
    for key, child in value.items():
        name = {'document': 'text', 'json': 'value'}.get(key, key)
        if key in ('json_text', 'document'):
            name = 'value'
            try:
                json.loads(child)
                rendered = literal(child) + '::json'
            except json.JSONDecodeError:
                name = 'text'
                rendered = literal(child)
        elif key in ('json',):
            rendered = literal(json.dumps(child, ensure_ascii=False, separators=(',', ':'))) + '::json'
        elif key == 'context' and not isinstance(child, str):
            name = 'context_value'
            rendered = literal(json.dumps(child, ensure_ascii=False, separators=(',', ':'))) + '::json'
        elif key == 'options':
            rendered = options(child)
        elif key == 'seed_spans':
            rendered = seeds(child)
        elif key == 'examples':
            rendered = examples(child)
        elif key == 'source':
            rendered = composite(child, 'thinkthen_source', ('file', 'first_line', 'last_line'))
        elif key == 'read_error':
            rendered = composite(child.get('error', child), 'thinkthen_reader_failure', ('kind', 'message'))
        elif key == 'images':
            rendered = array(child, 'thinkthen_image_value', lambda image: 'ROW(' + literal(image['media']) + ",decode('" + bytes(image['bytes']).hex() + "','hex'),NULL)::thinkthen_image_value")
        else:
            rendered = literal(child)
        arguments.append(name + '=>' + rendered)
    return 'thinkthen_record(' + ','.join(arguments) + ')'

def query(payload):
    target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'databases/postgresql/target'))
    profile = 'release' if os.environ.get('THINKTHEN_TEST_PROFILE') == 'full' else 'debug'
    with tempfile.TemporaryDirectory(prefix='thinkthen-postgresql-inputs-') as home:
        reader = subprocess.run([str(target / profile / 'thinkthen_read_inputs')],
                                input=json.dumps(payload['inputs']), text=True,
                                capture_output=True, check=True, env=child_env(home=home))
    inputs = json.loads(reader.stdout)
    if 'admission_error' in inputs:
        raise AdmissionError(inputs['admission_error']['error'])
    reading = {**payload.get('controls', {})}
    pointers = {'fields': 'field', 'context': 'context_field', 'options': 'options_field', 'examples': 'examples_field', 'seed_spans': 'seed_spans_field'}
    reading.update({pointers.get(key, key): value for key, value in inputs.get('reading', {}).items()})
    reading.update({key: inputs[key] for key in ('incremental', 'attempts', 'cancelled') if key in inputs})
    arguments = []
    for key, child in reading.items():
        if key in ('options', 'levels', 'labels'):
            rendered = options(child)
        elif key == 'context_schema':
            properties = array([{'name': name, 'kind': 'string_list' if value.get('type') == 'array' else value['type']} for name, value in child.get('properties', {}).items()], 'thinkthen_context_property', lambda value: composite(value, 'thinkthen_context_property', ('name', 'kind'))) if 'properties' in child else 'NULL'
            rendered = 'ROW(' + literal(child['type']) + ',' + properties + ',' + (array(child['required'], 'text') if 'required' in child else 'NULL') + ')::thinkthen_context_schema'
        elif key == 'field':
            rendered = array(child, 'text')
        elif key == 'seed_spans':
            rendered = seeds(child)
        elif key == 'examples':
            rendered = examples(child)
        else:
            rendered = literal(child)
        if key in ('batch', 'threshold', 'relation_threshold'):
            rendered = literal(str(child))
        arguments.append('\"' + key + '\"=>' + rendered)
    records = array(inputs['records'], 'thinkthen_record', record)
    return 'thinkthen_' + payload['verb'] + '_details(' + literal(payload['question']) + ',' + records + ',thinkthen_reading(' + ','.join(arguments) + '))'
