"""Derive installed PostgreSQL enums from the engine registry."""
import json
from pathlib import Path
import sys
import re
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from rules import FUNCTIONS, arguments, SETTINGS
registry = json.loads(Path(sys.argv[1]).read_text())
path = Path(sys.argv[2])
sql = path.read_text()
for token, key in [('STATUSES', 'statuses'), ('CODES', 'codes')]:
    sql = sql.replace('@' + token + '@', ', '.join("'" + row['name'] + "'" for row in registry[key]))
values = [row['value'] for row in registry['codes']]
assert len(values) == len(set(values)) and all(0 < value < 36**3 for value in values)
# Host transport arguments stay local; question arguments come from the schema.
for function, row in FUNCTIONS.items():
    names = arguments(function)
    required = set(row['required']) - {'function'}
    required = {'toolbox' if name == 'questions' else name for name in required}
    types = {'question': 'text', 'choices': 'text[]', 'labels': 'text[]', 'levels': 'text[]',
             'kinds': 'text[]', 'rules': 'thinkthen.rule[]', 'toolbox': 'jsonb',
             'threshold': 'float8', 'low': 'float8', 'high': 'float8', 'top': 'bigint', 'evidence': 'text'}
    order = ['question', 'choices', 'labels', 'levels', 'kinds', 'rules', 'toolbox', 'threshold', 'low', 'high', 'top', 'evidence']
    parameters = ['source_sql text'] + [name + ' ' + types[name] + ('' if name in required else ' DEFAULT NULL') for name in order if name in names]
    parameters += ["name text DEFAULT NULL", "params jsonb DEFAULT '[]'::jsonb", "record_column name DEFAULT 'record'", "id_column name DEFAULT NULL"]
    sql = re.sub(r'(CREATE FUNCTION thinkthen\.' + function + r')\([^\n]*\)', lambda match: match[1] + '(' + ', '.join(parameters) + ')', sql)
    signature = ['text'] + [types[name] for name in order if name in names] + ['text', 'jsonb', 'name', 'name']
    sql = re.sub(r'(REVOKE EXECUTE ON FUNCTION thinkthen\.' + function + r')\([^\n]*\)', lambda match: match[1] + '(' + ', '.join(signature) + ')', sql)
path.write_text(sql)

header = ['/* Generated from engine declarations during packaging. */', 'static const struct config_enum_entry backend_options[] = {']
for row in registry['backends']:
    header.append('    {"' + row['name'] + '", THINKTHEN_BACKEND_' + row['name'].upper() + ', false},')
header.append('    {NULL, 0, false}\n};')
schema = json.loads((Path(sys.argv[1]).parent / 'client.schema.json').read_text())
header.append('#define TT_DEFAULT_BACKEND THINKTHEN_BACKEND_' + schema['properties']['backend']['default'].upper())
retry = schema['$defs']['Retry']['properties']
for name, definition in retry.items():
    header.append('#define TT_DEFAULT_' + name.upper() + ' ' + str(definition['default']))
    header.append('#define TT_MIN_' + name.upper() + ' ' + str(definition['minimum']))
(path.parent / 'registry.h').write_text('\n'.join(header) + '\n')

variables = {'backend': 'backend', 'model': 'model', 'batch': 'batch', 'max_retries': 'retries', 'initial_backoff_ms': 'backoff', 'attempt_timeout_ms': 'timeout'}
registrations = []
for key, (nested, definition) in SETTINGS.items():
    name = '"thinkthen.' + key + '", "ThinkThen ' + key + '.", NULL, &' + variables[key]
    if key == 'backend':
        call = 'DefineCustomEnumVariable(' + name + ', TT_DEFAULT_BACKEND, backend_options'
    elif key == 'model':
        call = 'DefineCustomStringVariable(' + name + ', ""'
    else:
        default = str(definition.get('default', 0))
        minimum = str(definition.get('minimum', 0)) if nested else '0'
        call = 'DefineCustomIntVariable(' + name + ', ' + default + ', ' + minimum + ', ' + str(definition['default'] if key == 'max_retries' else definition.get('maximum', 'INT_MAX'))
    hook = 'tt_check_batch' if key == 'batch' else 'NULL'
    registrations.append(call + ', PGC_USERSET, 0, ' + hook + ', NULL, NULL);')
with (path.parent / 'registry.h').open('a') as output:
    output.write('#define TT_REGISTER_SETTINGS ' + ' '.join(registrations) + '\n')
