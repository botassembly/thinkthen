"""Name host union alternatives from their schema meaning, retaining their order."""
import re


def alternative_name(schema):
    if '$ref' in schema:
        value = schema['$ref'].removeprefix('#/$defs/')
    elif 'title' in schema:
        value = schema['title']
    elif 'const' in schema:
        value = str(schema['const']).lower()
    else:
        value = {'string': 'Text', 'object': 'Object', 'array': 'Array',
                 'integer': 'Integer', 'number': 'Number', 'boolean': 'Boolean',
                 'null': 'Null'}.get(schema.get('type'))
    if value is None:
        raise ValueError('host alternative needs a semantic schema name')
    return re.sub(r'[^A-Za-z0-9]+', '_', value).strip('_')


def alternative_names(alternatives):
    names = [alternative_name(schema) for schema in alternatives]
    if len({name.lower() for name in names}) != len(names):
        raise ValueError('colliding semantic host alternative names')
    return names
