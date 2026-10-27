"""Read the published schemas for database registration and settings."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CLIENT = json.loads((ROOT / 'schemas/client.schema.json').read_text())
REQUEST = json.loads((ROOT / 'schemas/request.schema.json').read_text())
FUNCTIONS = {row['properties']['function']['const']: row
             for row in REQUEST['$defs']['RawQuestion']['oneOf']}
SETTINGS = {key: (False, value) for key, value in CLIENT['properties'].items()
            if key not in ('api_key', 'url', 'retry')}
SETTINGS.update({key: (True, value) for key, value in CLIENT['$defs']['Retry']['properties'].items()})


def arguments(function):
    row = FUNCTIONS[function]
    names = set(row['properties']) - {'function'}
    if 'questions' in names:
        names.remove('questions')
        names.add('toolbox')
    threshold = row['properties'].get('threshold', {})
    if 'anyOf' in threshold or threshold.get('$ref', '').endswith('/Threshold'):
        names.update(('low', 'high'))
    return names


def header():
    lines = ['/* Generated from published schemas during packaging. */']
    lines.append('#define TT_DEFAULT_BACKEND_NAME ' + json.dumps(CLIENT['properties']['backend']['default']))
    lines.append('static int tt_setting(const char *key) {')
    for key, (retry, definition) in SETTINGS.items():
        kind = 1 if key in ('backend', 'model') else 2
        lines.append('if (!strcmp(key, ' + json.dumps(key) + ')) return ' + str(kind + 2 * retry) + ';')
    lines.extend(['return 0;', '}', 'static int tt_argument(const char *function, const char *name) {'])
    for function in FUNCTIONS:
        names = sorted(arguments(function))
        lines.append('if (!strcmp(function, ' + json.dumps(function) + ')) return ' + ' || '.join('!strcmp(name, ' + json.dumps(name) + ')' for name in names) + ';')
    lines.extend(['return 0;', '}'])
    return '\n'.join(lines) + '\n'


if __name__ == '__main__':
    import sys
    Path(sys.argv[1]).write_text(header())
