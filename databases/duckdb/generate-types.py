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
        return 'LogicalType::JSON()'
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
        properties = value['properties']
        order = ([key for key in ('kind', 'text', 'start', 'end', 'probability') if key in properties]
                 if 'text' in properties and 'start' in properties else
                 [key for key in ('source', 'target', 'relation', 'probability', 'either') if key in properties]
                 if 'relation' in properties and 'source' in properties else list(properties))
        order += [key for key in properties if key not in order]
        return 'LogicalType::STRUCT({' + ','.join('{' + quote({'start':'start_offset','end':'end_offset'}.get(key,key)) + ',' + native(properties[key]) + '}' for key in order) + '})'
    if kind == 'array':
        return 'LogicalType::LIST(' + native(value['items']) + ')'
    return {'null':'LogicalType::JSON()', 'string':'LogicalTypeId::VARCHAR','boolean':'LogicalTypeId::BOOLEAN','number':'LogicalTypeId::DOUBLE','integer':'LogicalTypeId::BIGINT'}[kind]
descriptions = {
    'decide': 'Judge whether each record answers the question. Not sure has a NULL answer; inspect status.',
    'filter': "Return kept records. Record failures raise after input ends.",
    'choose': 'Choose one of the named choices for each record.',
    'tag': 'Return the labels that apply to each record.',
    'score': 'Score each record on the supplied ordered levels.',
    'rank': 'Rank the complete input on ordered levels. top applies to the whole input, including UNION branches.',
    'find': 'Find matching text and its Unicode code point offsets in each record.',
    'recognize': 'Recognize named kinds and return their text and Unicode code point offsets.',
    'relate': 'Find named relations between recognized entities in each record.',
    'annotate': 'Answer named questions. answer contains bare values; detail contains their complete outcomes.',
}
examples = {}
for statement in (root / 'databases/duckdb/examples/all-ten.sql').read_text().splitlines():
    for function in descriptions:
        if 'thinkthen_' + function + '(' in statement:
            examples[function] = statement
lines = ['// Generated at build time from the engine contract.',
         'const char *FunctionName(uint32_t id) { switch (id) {' + ''.join('case ' + str(v['value']) + ': return ' + quote(v['name']) + ';' for v in contract['functions']) + 'default: throw InvalidInputException("usage.abi: unknown function"); } }',
         'LogicalType StatusType() { return ' + enum([v['name'] for v in contract['statuses']]) + '.WithAlias("thinkthen_status"); }',
         'LogicalType CodeType() { return ' + enum([v['name'] for v in contract['codes']]) + '.WithAlias("thinkthen_error_code"); }',
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
lines += ['string FunctionHelp(const string &function) {', *['if (function == ' + quote(key) + ') return ' + quote(value) + ';' for key, value in descriptions.items()], 'return \"\";', '}', 'string FunctionExample(const string &function) {', *['if (function == ' + quote(key) + ') return ' + quote(value) + ';' for key, value in examples.items()], 'return \"\";', '}']
Path(sys.argv[1]).write_text('\n'.join(lines) + '\n')

sys.path.insert(0, str(root / "extensions"))
from rules import header, SETTINGS
with Path(sys.argv[1]).open("a") as output:
    output.write(header())
    output.write("bool HasArgument(const string &function, const char *name) { return tt_argument(function.c_str(), name); }\n")
    keys = list(SETTINGS)
    output.write('const char *SettingKey(int index) { static const char *keys[] = {' + ','.join(map(quote, keys)) + '}; return keys[index]; }\n')
    help_text = {
        'backend': 'Backend that answers: systemone, openai or stub. Credentials come from the environment. Session only.',
        'model': 'Model selected for this session; NULL uses the backend default.',
        'batch': 'Maximum questions sent in one request. Session only.',
        'max_retries': 'Retries after a failed backend request. Session only.',
        'initial_backoff_ms': 'Initial retry delay in milliseconds. Session only.',
        'attempt_timeout_ms': 'Time limit for one backend attempt in milliseconds. Session only.',
    }
    registrations = []
    for index, key in enumerate(keys):
        kind = 'BackendType()' if key == 'backend' else 'LogicalType(LogicalTypeId::VARCHAR)' if key == 'model' else 'LogicalType(LogicalTypeId::UBIGINT)'
        default = 'Value(' + kind + ')'
        registrations.append('config.AddExtensionOption("thinkthen_' + key + '", ' + quote(help_text[key]) + ', ' + kind + ', ' + default + ', Configure<' + str(index) + '>, SetScope::SESSION);')
    output.write('#define TT_REGISTER_SETTINGS ' + ' '.join(registrations) + '\n')
