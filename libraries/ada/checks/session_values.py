"""Translate selected shared case values to ordinary generated Ada records."""
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
spec = importlib.util.spec_from_file_location('ada_result_generator', ROOT / 'sdlc/generators/results/generate.py')
generator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(generator)
spec = importlib.util.spec_from_file_location('ada_template', ROOT / 'sdlc/generators/results/templates/ada.py')
template = importlib.util.module_from_spec(spec)
spec.loader.exec_module(template)
schema = json.loads((ROOT / 'specification/request.schema.json').read_text())
schema['$defs']['Request'] = {k: v for k, v in schema.items() if k != '$defs'}
definitions = generator.prepare(generator.graph(schema, ('Request', 'RequestSessionDescriptor', 'RequestReaderFailure')))
name = template.identifier


def matches(source, value):
    if '$ref' in source:
        return matches(definitions[source['$ref'].removeprefix('#/$defs/')], value)
    if 'const' in source:
        return value == source['const']
    if 'properties' in source:
        return isinstance(value, dict) and set(source.get('required', [])) <= value.keys() and all(matches(source['properties'][k], v) for k, v in value.items() if k in source['properties']) and value.keys() <= source['properties'].keys()
    if 'variants' in source:
        return any(matches(definitions[n], value) for n, _ in source['variants'])
    alternatives = source.get('anyOf', source.get('oneOf', []))
    if alternatives:
        return any(matches(item, value) for item in alternatives)
    kind = source.get('type')
    if isinstance(kind, list):
        return any(matches({**source, 'type': k}, value) for k in kind)
    return {'string': isinstance(value, str), 'integer': type(value) is int,
            'number': type(value) in (int, float), 'boolean': type(value) is bool,
            'array': isinstance(value, list), 'object': isinstance(value, dict),
            'null': value is None}.get(kind, True)


def ada_text(value):
    chunks = []
    for byte in value.encode():
        if 32 <= byte <= 126:
            chunks.append(template.ada_string(chr(byte)))
        else:
            chunks.append(f"Character'Val ({byte})")
    return 'Ada.Strings.Unbounded.To_Unbounded_String (' + (' & '.join(chunks) if chunks else '""') + ')'


def construct(key, source, value):
    typ = name(key)
    if not isinstance(source, dict):
        source = {}
    if '$ref' in source:
        original = source['$ref'].removeprefix('#/$defs/')
        return construct(original, definitions[original], value)
    variants = source.get('variants')
    alternatives = source.get('anyOf', source.get('oneOf'))
    if isinstance(source.get('type'), list):
        alternatives = [{**source, 'type': k} for k in source['type']]
    if variants:
        index, (branch, _) = next((i, pair) for i, pair in enumerate(variants) if matches(definitions[pair[0]], value))
        return f"{typ}'(Kind => {name(key + '_arm_' + branch)}, V_{index} => " + construct(branch, definitions[branch], value) + ')'
    if alternatives and len(alternatives) > 1:
        index, branch = next((i, branch) for i, branch in enumerate(alternatives) if matches(branch, value))
        return f"{typ}'(Kind => {name(key + '_arm_' + str(index))}, V_{index} => " + construct(key + '_value_' + str(index), branch, value) + ')'
    if alternatives:
        source = alternatives[0]
    if 'properties' in source:
        assert value.keys() <= source['properties'].keys(), (key, value.keys() - source['properties'].keys())
        fields = []
        for member, child in source['properties'].items():
            field = name(member)
            required = member in source.get('required', [])
            if member not in value and not required:
                expression = '(Present => False)'
            else:
                expression = construct(key + '_field_' + member, child, value.get(member, child.get('const')))
                if not required:
                    expression = '(Present => True, Value => ' + expression + ')'
            fields.append(field + ' => ' + expression)
        return f"{typ}'(" + (', '.join(fields) if fields else 'null record') + ')'
    if source.get('type') == 'array' or source.get('type') == 'object' and isinstance(source.get('additionalProperties'), dict):
        mapping = source.get('type') == 'object'
        values = ['(Key => ' + ada_text(k) + ', Value => ' + construct(key + '_element', source['additionalProperties'], v) + ')' for k, v in value.items()] if mapping else [construct(key + '_element', source.get('items', {}), item) for item in value]
        return f"{typ}'[" + ', '.join(values) + ']'
    if 'const' in source or source.get('type') == 'null':
        return f"{typ}'(null record)"
    if source.get('type') == 'string':
        return f'{typ} (' + ada_text(value) + ')'
    if source.get('type') == 'boolean':
        return f"{typ}'(" + str(value) + ')'
    if source.get('type') in ('number', 'integer'):
        return f"{typ}'(" + str(value) + ')'
    return f'{typ} (' + ada_text(json.dumps(value, ensure_ascii=False)) + ')'


def cases():
    corpus = json.loads((ROOT / 'conformance/cases.json').read_text())['cases']
    result = []
    for verb in (arm['properties']['function']['const'] for arm in schema['$defs']['RequestCall']['oneOf']):
        case = next(case for case in corpus if case['verb'] == verb)
        question = dict(case.get('question', case.get('question_set', {})))
        question.pop('relation_threshold', None)
        question.pop('units', None)
        none = question.pop('none', None)
        if verb in ('filter', 'rank', 'annotate'):
            original = {'kind': 'records', 'items': [{'original': {'kind': 'text', 'text': e['evidence']}} for e in case['exchanges']]}
        elif verb == 'find':
            original = {'kind': 'units', 'items': [{'original': {'kind': 'text', 'text': unit}} for unit in case['question']['units']]}
        elif verb == 'relate':
            original = {'kind': 'entities', 'items': [{'original': {'kind': 'json', 'value': entity}} for entity in case['entities']]}
        else:
            original = {'kind': 'text', 'text': case.get('text', case.get('exchanges', [{}])[0].get('evidence', ''))}
        value = {'function': verb, 'question': {'kind': 'definition', 'value': question}, 'input': original,
                 'options': {'batch': 1}}
        if verb in ('decide', 'choose', 'tag', 'score'):
            value['options']['details'] = True
        if none is not None:
            value['options']['none'] = none
        key = 'RequestCall_' + verb
        result.append((verb, case['id'], construct(key, definitions[key], value)))
    return result
