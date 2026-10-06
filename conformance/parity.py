"""Shared required cells for the existing surfaces rung (0432 phase A).

Baseline command success is deliberately separate from adopted public cases.
Families print a cell only after assertions at their named public boundary.
"""
import json
import os
import re
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
DOCUMENT = ROOT / 'conformance/cases.json'
PREFIX = 'parity: '


def inventory():
    document = json.loads(DOCUMENT.read_text())
    parity = document['parity']
    cases = parity['required_cases']
    consumers = parity['consumers']
    for label, rows in [('case', cases), ('consumer', consumers)]:
        ids = [row['id'] for row in rows]
        if len(ids) != len(set(ids)) or not ids:
            raise ValueError(f'duplicate or empty parity {label} IDs')
    result_contract = (ROOT / 'specification/result.md').read_text()
    tokens = result_contract.split('The closed surface tokens are ', 1)[1].split('. TypeScript', 1)[0]
    expected_consumers = set(re.findall(r'`([^`]+)`', tokens)) | {'typescript'}
    if {row['id'] for row in consumers} != expected_consumers:
        raise ValueError('public variants differ from the settled surface contract')
    for kind in ('typed-result', 'result2', 'located'):
        if {case['verb'] for case in cases if case['kind'] == kind} != set(parity['functions']):
            raise ValueError(f'{kind}: a required function is missing')
    canonical = {case['id']: case for case in document['cases']}
    types = {case['name'] for case in json.loads((ROOT / 'specification/fixtures/types/corpus.json').read_text())['cases']}
    settings = {case['id'] for case in json.loads((ROOT / 'conformance/settings.json').read_text())['cases']}
    if set(parity['functions']) != {case['verb'] for case in document['cases']}:
        raise ValueError('parity functions differ from the canonical ten')
    if len(parity['functions']) != len(set(parity['functions'])):
        raise ValueError('duplicate parity function')
    if {case['id'] for case in cases if case['kind'] == 'behavior'} != set(canonical):
        raise ValueError('required behavior cases differ from canonical cases')
    registered = {line.split()[0] for line in (ROOT / 'sdlc/surfaces.txt').read_text().splitlines()
                  if line.strip() and not line.startswith('#')}
    if {row['surface'] for row in consumers} - {'.'} != registered:
        raise ValueError('public consumer inventory does not cover the surface registry')
    for consumer in consumers:
        if consumer['surface'] != '.' and not (ROOT / consumer['surface'] / 'check.sh').is_file():
            raise ValueError(f"{consumer['id']}: missing baseline runner")
    for case in cases:
        if case['verb'] not in parity['functions'] or not case['expect']:
            raise ValueError(f"{case['id']}: unknown verb or empty expectation")
        given = case['input']
        if 'case_ref' in given:
            source = {'conformance/settings.json': settings,
                      'specification/fixtures/types/corpus.json': types}.get(given.get('fixture'), canonical)
            if given['case_ref'] not in source:
                raise ValueError(f"{case['id']}: unknown case reference")
        for path in given.get('paths', []) + given.get('images', []):
            if not (ROOT / path).exists():
                raise ValueError(f"{case['id']}: missing input {path}")
        for owner in case['preconditions']:
            if not list((ROOT / 'sdlc/tickets').glob(owner + '-*.md')):
                raise ValueError(f"{case['id']}: unknown precondition {owner}")
    return parity


def cells(output, consumers, cases):
    """Read asserted named cells, never infer typed support from generic tests."""
    found = {}
    for line in output.splitlines():
        if not line.startswith(PREFIX):
            continue
        row = json.loads(line[len(PREFIX):])
        key = (row['consumer'], row['case'])
        if row['consumer'] not in consumers or row['case'] not in cases:
            raise ValueError(f'unknown parity cell {key}')
        if key in found:
            raise ValueError(f'duplicate parity cell {key}')
        required = cases[row['case']].get('checks', ['named', 'runtime'])
        if set(row) != {'consumer', 'case', 'checks', 'status'}:
            raise ValueError(f'{key}: unexpected cell fields')
        if row['status'] not in ('pass', 'fail'):
            raise ValueError(f'{key}: skipped or unknown status')
        if set(row['checks']) != set(required) or len(row['checks']) != len(required):
            raise ValueError(f'{key}: missing named/compiler/runtime check')
        found[key] = row['status']
    return found


def run(port):
    parity = inventory()
    if os.environ.get('THINKTHEN_CONFORMANCE_IDS') is not None:
        raise ValueError('strict parity refuses a case selector')
    cases = {case['id']: case for case in parity['required_cases']}
    consumers = {row['id']: row for row in parity['consumers']}
    output_dir = ROOT / 'target/parity'
    output_dir.mkdir(parents=True, exist_ok=True)
    commands = {}
    for row in consumers.values():
        commands.setdefault(tuple(row['baseline']), []).append(row['id'])
    matrix = []
    for command, ids in commands.items():
        label = '+'.join(ids)
        log = output_dir / (label + '.log')
        print(f'parity baseline: {label}', flush=True)
        args = list(command) + ([] if command[0] == 'cargo' else [port])
        # Existing runners own scratch configuration and loopback backends.
        # The surfaces rung has already removed unrelated environment values.
        with tempfile.TemporaryDirectory(prefix='thinkthen-parity-config-') as config, log.open('w') as stream:
            env = dict(os.environ, XDG_CONFIG_HOME=config)
            process = subprocess.run(['sh', 'sdlc/scripts/time-limit', '1800', *args],
                                     cwd=ROOT, env=env, stdout=stream, stderr=subprocess.STDOUT, check=False)
        code = process.returncode
        seen = {}
        error = None
        try:
            seen = cells(log.read_text(), ids, cases)
        except (ValueError, KeyError, TypeError) as failure:
            error = str(failure)
        for consumer in ids:
            states = {}
            for case in cases:
                # A failing process, including 77, invalidates all its cells.
                states[case] = ('fail' if code or error else seen.get((consumer, case), 'missing'))
            named = [verb for verb in parity['functions']
                     if states['typed-' + verb] == 'pass']
            matrix.append({'consumer': consumer, 'baseline_exit': code, 'baseline_error': error,
                           'named_typed_functions': named if seen and not code and not error else None,
                           'cells': states, 'log': str(log.relative_to(ROOT))})
        print(f'parity baseline: {label} exit={code}; adopted cells={len(seen)}', flush=True)
    (output_dir / 'matrix.json').write_text(json.dumps(matrix, indent=2) + '\n')
    lines = ['| Consumer | Baseline exit | Named typed functions | Required gaps |',
             '| --- | --- | --- | --- |']
    for row in matrix:
        count = 'not checked' if row['named_typed_functions'] is None else str(len(row['named_typed_functions']))
        gaps = sum(state != 'pass' for state in row['cells'].values())
        lines.append(f"| {row['consumer']} | {row['baseline_exit']} | {count} | {gaps} |")
    (output_dir / 'matrix.md').write_text('\n'.join(lines) + '\n')
    print('\n'.join(lines))
    return int(any(state != 'pass' for row in matrix for state in row['cells'].values()))


def main():
    if sys.argv[1:] == ['--validate']:
        parity = inventory()
        print(f"parity: {len(parity['consumers'])} public consumers; "
              f"{len(parity['required_cases'])} required cases; declarations valid")
        return 0
    if len(sys.argv) == 2:
        return run(sys.argv[1])
    raise ValueError('usage: parity.py --validate|PORT (called by surfaces --parity)')


if __name__ == '__main__':
    sys.exit(main())
