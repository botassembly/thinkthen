"""Run the current J1 corpus structural verdicts with Draft 2020-12."""
import json
from pathlib import Path
from jsonschema import Draft202012Validator
root = Path(__file__).resolve().parent
source = root.parents[2]
schema = json.loads((source / 'specification/result.schema.json').read_text())
requests = json.loads((source / 'specification/question-file.schema.json').read_text())
cases = json.loads((source / 'specification/fixtures/types/corpus.json').read_text())['cases']
Draft202012Validator.check_schema(schema)
count = 0
positive = negative = 0
for case in cases:
    checks = []
    if 'request' in case:
        checks.append(('doorRequest', case['request'], case.get('request_valid', True)))
    if 'response' in case:
        checks.append((case['definition'], case['response'], case.get('response_valid', True)))
    for definition, value, wanted in checks:
        defs = requests['$defs'] if definition == 'doorRequest' else schema['$defs']
        validator = Draft202012Validator({'$ref': f'#/$defs/{definition}', '$defs': defs})
        actual = validator.is_valid(value)
        if actual != wanted: raise AssertionError(f"{case['name']} {definition} expected {wanted} got {actual}")
        positive += wanted
        negative += not wanted
        count += 1
print(f'DRAFT202012_RESULT_PARITY_PASS {len(cases)} cases {count} checks {positive} positive {negative} negative')
