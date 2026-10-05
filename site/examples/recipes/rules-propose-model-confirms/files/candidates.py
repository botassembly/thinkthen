"""Illustrative caller-owned amount discovery."""
import json
import re
import subprocess
import sys


class CandidateError(ValueError):
    pass


def closed_object(pairs):
    value = {}
    for k, v in pairs:
        if k in value:
            raise CandidateError('duplicate member')
        value[k] = v
    return value


def read_rows(text):
    try:
        return [
            json.loads(line,
                       object_pairs_hook=closed_object)
            for line in text.splitlines() if line.strip()
        ]
    except (ValueError, TypeError) as error:
        raise CandidateError('invalid JSON') from error


def validate(rows):
    ids = set()
    row_fields = {'id', 'text', 'candidates'}
    for row in rows:
        if (not isinstance(row, dict)
                or set(row) != row_fields):
            raise CandidateError('required fields')
        identity = row['id']
        if (not isinstance(identity, str) or not identity
                or identity in ids):
            raise CandidateError('duplicate or invalid id')
        ids.add(identity)
        text = row['text']
        if (not isinstance(text, str) or not text
                or len(text) > 4096):
            raise CandidateError('invalid evidence')
        pool = row['candidates']
        if not isinstance(pool, list) or len(pool) > 2:
            raise CandidateError('invalid pool')
        labels, values, last = set(), set(), -1
        for i, c in enumerate(pool):
            fields = {
                'label', 'value', 'description',
                'start', 'end'
            }
            if not isinstance(c, dict) or set(c) != fields:
                raise CandidateError('candidate fields')
            label = c['label']
            if (not isinstance(label, str)
                    or label != f'c{i+1:03d}'
                    or label in labels):
                raise CandidateError('invalid label/order')
            labels.add(label)
            value = c['value']
            if (not isinstance(value, str) or not value
                    or value in values):
                raise CandidateError('invalid value')
            values.add(value)
            start, end = c['start'], c['end']
            if (type(start) is not int
                    or type(end) is not int or start <= last
                    or end <= start or end > len(text)
                    or text[start:end] != value):
                raise CandidateError('wrong location/value')
            description = (
                f'Amount {value} at characters '
                f'{start}:{end}'
            )
            if c['description'] != description:
                raise CandidateError('wrong description')
            last = start
    return rows


def discover(cases):
    rows = []
    for case in cases:
        fields = {'id', 'text'}
        if (not isinstance(case, dict)
                or set(case) != fields):
            raise CandidateError('input fields')
        text = case['text']
        if not isinstance(text, str):
            raise CandidateError('input text')
        candidates, seen = [], set()
        for hit in re.finditer(r'\$\d+\.\d{2}\b', text):
            value = hit.group()
            if value in seen:
                continue
            seen.add(value)
            start, end = hit.start(), hit.end()
            candidates.append({
                'label': f'c{len(candidates)+1:03d}',
                'value': value,
                'description': (
                    f'Amount {value} at characters '
                    f'{start}:{end}'
                ),
                'start': start, 'end': end
            })
        rows.append({
            'id': case['id'], 'text': text,
            'candidates': candidates
        })
    return validate(rows)


def produce(command):
    done = subprocess.run(
        command, capture_output=True, text=True,
        timeout=10, check=False
    )
    if done.returncode != 0:
        raise CandidateError('producer exit')
    return validate(read_rows(done.stdout))


def adapt(row):
    options = [
        (c['label'], c['description'])
        for c in row['candidates']
    ] + [('none', 'No offered amount is the stated total')]
    return {
        'id': row['id'], 'text': row['text'],
        'options': dict(options),
        'candidates': row['candidates']
    }


if __name__ == '__main__':
    try:
        with open(sys.argv[1], encoding='utf8') as source:
            rows = discover(read_rows(source.read()))
        if '--adapt' in sys.argv[2:]:
            rows = [adapt(row) for row in rows
                    if row['candidates']
                    and row['id'].startswith('R')]
        print('\n'.join(
            json.dumps(row, separators=(',', ':'))
            for row in rows
        ))
    except (CandidateError, OSError, IndexError):
        sys.stderr.write('recipe producer: invalid input\n')
        sys.exit(1)
