"""Derive installed PostgreSQL enums, signatures and settings from the engine registry."""
import json
from pathlib import Path
import sys
import re
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from rules import FUNCTIONS, arguments, SETTINGS
registry = json.loads(Path(sys.argv[1]).read_text())
path = Path(sys.argv[2])
upgrades = [Path(name) for name in sys.argv[3:]]
sql = path.read_text()
for native_type, key in [('status', 'statuses'), ('error_code', 'codes')]:
    labels = ', '.join("'" + row['name'] + "'" for row in registry[key])
    sql, count = re.subn(r'(CREATE TYPE thinkthen\.' + native_type + r' AS ENUM \()[^;]*(\);)',
                         lambda match: match[1] + labels + match[2], sql)
    assert count == 1, native_type
values = [row['value'] for row in registry['codes']]
assert len(values) == len(set(values)) and all(0 < value < 36**3 for value in values)
# Host transport arguments stay local; question arguments come from the schema.
TYPES = {'question': 'text', 'choices': 'text[]', 'labels': 'text[]', 'levels': 'text[]',
         'kinds': 'text[]', 'rules': 'thinkthen.rule[]', 'questions': 'jsonb',
         'threshold': 'float8', 'low': 'float8', 'high': 'float8', 'top': 'bigint', 'field': 'text'}
ORDER = ['question', 'choices', 'labels', 'levels', 'kinds', 'rules', 'questions', 'threshold', 'low', 'high', 'top', 'field']
# The shared SQL call always raises filter failures; other functions opt in.
LOCAL = {function: [('yes_means', 'text', 'NULL'), ('no_means', 'text', 'NULL')]
         for function in ('decide', 'filter')}

LOCAL.update({function: [('descriptions', 'jsonb', 'NULL')]
              for function in ('choose', 'tag', 'score', 'rank')})
TAIL = [('name', 'text', 'NULL'), ('params', 'jsonb', "'[]'::jsonb"),
        ('record_column', 'name', "'record'"), ('id_column', 'name', 'NULL')]
for function, row in FUNCTIONS.items():
    names = arguments(function)
    required = set(row['required']) - {'function'}
    own = [(name, TYPES[name], None if name in required else 'NULL') for name in ORDER if name in names]
    raising = [] if function == 'filter' else [('raise_if_failed', 'boolean', 'false')]
    parameters = [('source_sql', 'text', None)] + own + LOCAL.get(function, []) + raising + TAIL
    declared = ', '.join(name + ' ' + kind + ('' if default is None else ' DEFAULT ' + default)
                         for name, kind, default in parameters)
    signature = ', '.join(kind for _, kind, _ in parameters)
    sql, count = re.subn(r'(CREATE FUNCTION thinkthen\.' + function + r')\([^()]*\)',
                         lambda match: match[1] + '(' + declared + ')', sql)
    assert count == 1, function
    sql, count = re.subn(r'((?:REVOKE EXECUTE ON|COMMENT ON) FUNCTION thinkthen\.' + function + r')\([^()]*\)',
                         lambda match: match[1] + '(' + signature + ')', sql)
    assert count == 2, function
path.write_text(sql)
# Each upgrade script ends by installing the current release's objects.
install = sql.split('\n', 1)[1]
for upgrade in upgrades:
    text = upgrade.read_text()
    assert text.count('@INSTALL@') == 1, upgrade
    upgrade.write_text(text.replace('@INSTALL@', install))

header = ['/* Generated from engine declarations during packaging. */', 'static const struct config_enum_entry backend_options[] = {']
header.append('    {"", 0, true}, /* Omitted selection is resolved by the engine. */')
for row in registry['backends']:
    header.append('    {"' + row['name'] + '", THINKTHEN_BACKEND_' + row['name'].upper() + ', false},')
header.append('    {NULL, 0, false}\n};')
schema = json.loads((Path(sys.argv[1]).parent / 'client.schema.json').read_text())
retry = schema['$defs']['Retry']['properties']
for name, definition in retry.items():
    header.append('#define TT_DEFAULT_' + name.upper() + ' ' + str(definition['default']))
    header.append('#define TT_MIN_' + name.upper() + ' ' + str(definition['minimum']))
(path.parent / 'registry.h').write_text('\n'.join(header) + '\n')

# Engine keys ending in _ms become PostgreSQL time settings, so '2min' works.
variables = {'backend': 'backend', 'model': 'model', 'batch': 'batch', 'max_retries': 'retries', 'initial_backoff_ms': 'backoff', 'attempt_timeout_ms': 'timeout'}
descriptions = {
    'backend': 'Backend that answers questions; an administrator limits choices with thinkthen.allowed_backends.',
    'model': 'Model the backend uses; empty selects the backend default.',
    'batch': 'Records sent in one backend request; 0 lets the engine choose.',
    'max_retries': 'Retries after a failed backend request; SQL may lower but not raise the engine default.',
    'initial_backoff_ms': 'Wait before the first retry of a failed backend request.',
    'attempt_timeout_ms': 'Time limit for one backend request attempt.',
}
assert set(descriptions) == set(SETTINGS) == set(variables), SETTINGS
registrations = []
for key, (nested, definition) in SETTINGS.items():
    setting = key.removesuffix('_ms')
    name = '"thinkthen.' + setting + '", "' + descriptions[key] + '", NULL, &' + variables[key]
    if key == 'backend':
        call = 'DefineCustomEnumVariable(' + name + ', 0, backend_options'
    elif key == 'model':
        call = 'DefineCustomStringVariable(' + name + ', ""'
    else:
        default = str(definition.get('default', 0))
        minimum = str(definition.get('minimum', 0)) if nested else '0'
        call = 'DefineCustomIntVariable(' + name + ', ' + default + ', ' + minimum + ', ' + str(definition['default'] if key == 'max_retries' else definition.get('maximum', 'INT_MAX'))
    hook = 'tt_check_batch' if key == 'batch' else 'NULL'
    flags = 'GUC_UNIT_MS' if setting != key else '0'
    registrations.append(call + ', PGC_USERSET, ' + flags + ', ' + hook + ', NULL, NULL);')
with (path.parent / 'registry.h').open('a') as output:
    output.write('#define TT_REGISTER_SETTINGS ' + ' '.join(registrations) + '\n')
