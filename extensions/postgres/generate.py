"""Derive installed PostgreSQL enums from the engine registry."""
import json
from pathlib import Path
import sys
registry = json.loads(Path(sys.argv[1]).read_text())
path = Path(sys.argv[2])
sql = path.read_text()
for token, key in [('STATUSES', 'statuses'), ('CODES', 'codes')]:
    sql = sql.replace('@' + token + '@', ', '.join("'" + row['name'] + "'" for row in registry[key]))
values = [row['value'] for row in registry['codes']]
assert len(values) == len(set(values)) and all(0 < value < 36**3 for value in values)
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
(path.parent / 'registry.h').write_text('\n'.join(header) + '\n')
