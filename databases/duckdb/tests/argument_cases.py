"""Keep shared fixture questions positional before DuckDB named options."""
import json
from pathlib import Path

from cases import arguments, invocation, quote


def argument_order():
    root = Path(__file__).resolve().parents[3]
    paths = sorted((root / 'conformance').rglob('*.json'))
    paths += sorted((root / 'construction').glob('*.json'))
    checked = 0
    for path in paths:
        value = json.loads(path.read_text())
        for case in value if isinstance(value, list) else [value]:
            if not isinstance(case, dict):
                continue
            request = case.get('request', {})
            question = request.get('question')
            if not isinstance(question, dict) or 'function' not in question:
                continue
            # Move the question to both ends, including after every named option.
            options = {key: value for key, value in question.items() if key != 'question'}
            variants = ([{'question': question['question'], **options},
                         {**options, 'question': question['question']}]
                        if 'question' in question else [question])
            for variant in variants:
                sql = invocation({**request, 'question': variant})
                values = arguments(variant)
                if 'name' in request:
                    values.append('name := ' + quote(request['name']))
                if 'question' in variant:
                    assert values[0] == quote(variant['question']), (path, sql)
                    values = values[1:]
                assert all(' := ' in value for value in values), (path, sql)
                if 'question' in variant:
                    suffix = ', ' + quote(variant['question']) + ''.join(', ' + value for value in values) + ')'
                    assert sql.endswith(suffix), (path, sql)
            checked += 1
    assert checked > 0
    return checked


if __name__ == '__main__':
    print('Shared fixture SQL argument order passes:', argument_order())
