"""Derive native DuckDB result types from the admitted row schema."""
import json
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[2]
schema = json.loads((root / 'schemas/row.schema.json').read_text())
contract = json.loads((root / 'schemas/contract.json').read_text())
def quote(value):
    return json.dumps(value)
def enum(values):
    return 'EnumType({' + ','.join(map(quote, values)) + '})'
def native(value):
    if value is False:
        return 'LogicalTypeId::SQLNULL'
    if '$ref' in value:
        return native(schema['$defs'][value['$ref'].split('/')[-1]])
    if 'anyOf' in value:
        members = [item for item in value['anyOf'] if item.get('type') != 'null']
        return native(members[0]) if len(members) == 1 else 'LogicalType::JSON()'
    if 'oneOf' in value:
        return enum([item['const'] for item in value['oneOf']])
    kind = value.get('type')
    if isinstance(kind, list):
        kind = next(item for item in kind if item != 'null')
    if kind == 'object':
        if 'additionalProperties' in value and isinstance(value['additionalProperties'], dict):
            return 'LogicalType::MAP(LogicalTypeId::VARCHAR,' + native(value['additionalProperties']) + ')'
        return 'LogicalType::STRUCT({' + ','.join('{' + quote(key) + ',' + native(item) + '}' for key,item in value['properties'].items()) + '})'
    if kind == 'array':
        return 'LogicalType::LIST(' + native(value['items']) + ')'
    return {'null':'LogicalTypeId::SQLNULL', 'string':'LogicalTypeId::VARCHAR','boolean':'LogicalTypeId::BOOLEAN','number':'LogicalTypeId::DOUBLE','integer':'LogicalTypeId::BIGINT'}[kind]
lines = ['// Generated at build time from the engine contract.',
         'const char *FunctionName(uint32_t id) { switch (id) {' + ''.join('case ' + str(v['value']) + ': return ' + quote(v['name']) + ';' for v in contract['functions']) + 'default: throw InvalidInputException("usage.abi: unknown function"); } }',
         'LogicalType StatusType() { return ' + enum([v['name'] for v in contract['statuses']]) + '; }',
         'LogicalType CodeType() { return ' + enum([v['name'] for v in contract['codes']]) + '; }',
         'LogicalType BackendType() { return ' + enum([v['name'] for v in contract['backends']]) + '; }',
         'LogicalType SourceType() { return ' + native(schema['$defs']['Source']) + '; }',
         'LogicalType AnswerType(const string &function) {']
rows = []
for ref in schema['oneOf']:
    row = schema['$defs'][ref['$ref'].split('/')[-1]]
    function = schema['$defs'][row['properties']['function']['$ref'].split('/')[-1]]['const']
    rows.append((function,row))
    if function != 'annotate':
        lines.append('if (function == ' + quote(function) + ') return ' + native(row['oneOf'][0]['properties']['answer']) + ';')
lines.extend(['throw InternalException("Unknown answer type");', '}', 'LogicalType DetailType(const string &function) {'])
for function,row in rows:
    if function != 'annotate':
        lines.append('if (function == ' + quote(function) + ') return ' + native(row['oneOf'][0]['properties']['detail']) + ';')
lines.extend(['return LogicalTypeId::SQLNULL;', '}'])
Path(sys.argv[1]).write_text('\n'.join(lines) + '\n')
