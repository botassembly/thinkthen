"""Run packaged C and C++ consumers against canonical generated construction rows."""
import json
from pathlib import Path
import subprocess
import sys


def selected(record, pointer):
    value = record
    for field in pointer.removeprefix('/').split('/') if pointer else []:
        field = field.replace('~1', '/').replace('~0', '~')
        value = value[int(field)] if isinstance(value, list) else value[field]
    return value if isinstance(value, str) else json.dumps(value, ensure_ascii=False, separators=(',', ':'), sort_keys=True)


def utf8(value, evidence):
    if isinstance(value, dict):
        if 'start' in value and 'end' in value:
            value['start'] = len(evidence[:value['start']].encode('utf-8'))
            value['end'] = len(evidence[:value['end']].encode('utf-8'))
        for child in value.values():
            utf8(child, evidence)
    elif isinstance(value, list):
        for child in value:
            utf8(child, evidence)


def expected_rows(case, offsets):
    rows = json.loads(json.dumps(case['expected']))
    if offsets == 'utf8':
        question = case['request']['question']
        for row in rows:
            record = case['request']['records'][row['position'] - 1]
            if question['function'] == 'annotate' and row['status'] == 'answered':
                for name, child in row['answer'].items():
                    entry = question['questions'][name]
                    if entry['function'] in ('find', 'recognize', 'relate'):
                        utf8(child, selected(record, entry.get('evidence', question.get('evidence', ''))))
            elif question['function'] in ('find', 'recognize', 'relate'):
                utf8(row, selected(record, question.get('evidence', '')))
    return rows


def compare(output, case, registry, offsets='codepoints'):
    result = json.loads(output)
    assert 'error' not in result, result
    rows = result['rows']
    functions = {item['value']: item['name'] for item in registry['functions']}
    for row in rows:
        if isinstance(row['function'], int):
            row['function'] = functions[row['function']]
    assert rows == expected_rows(case, offsets), (case['request']['question']['function'], rows, expected_rows(case, offsets))


if __name__ == '__main__':
    binary, fixtures, registry, *offsets = sys.argv[1:]
    registry = json.loads(Path(registry).read_text())
    for case in json.loads(Path(fixtures).read_text()):
        output = subprocess.check_output([binary, json.dumps(case['request']), json.dumps(case['client'])], text=True)
        compare(output, case, registry, offsets[0] if offsets else 'codepoints')
    print('candidate native consumer: canonical construction rows pass')
