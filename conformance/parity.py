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


def all_consumers(parity):
    return parity["consumers"] + parity.get("pending_consumers", [])


def required_cases(parity, consumer):
    return {case["id"]: case for case in parity["required_cases"]
            if consumer in case.get("consumers", [consumer])}

ROOT = Path(__file__).resolve().parents[1]
DOCUMENT = ROOT / 'conformance/cases.json'
PREFIX = 'parity: '


def inventory():
    document = json.loads(DOCUMENT.read_text())
    parity = document['parity']
    if parity['schema'] != 'thinkthen.parity-cases/1':
        raise ValueError('unknown parity case schema')
    cases = parity['required_cases']
    consumers = parity['consumers']
    for label, rows in [('case', cases), ('consumer', all_consumers(parity))]:
        ids = [row['id'] for row in rows]
        if len(ids) != len(set(ids)) or not ids:
            raise ValueError(f'duplicate or empty parity {label} IDs')
    result_contract = (ROOT / 'specification/result.md').read_text()
    tokens = result_contract.split('The closed surface tokens are ', 1)[1].split('. TypeScript', 1)[0]
    expected_consumers = set(re.findall(r'`([^`]+)`', tokens)) | {'typescript'}
    pending = parity.get('pending_consumers', [])
    pending_ids = {row['id'] for row in pending}
    if pending_ids - {'mcp'} or pending_ids & {row['id'] for row in consumers}:
        raise ValueError('unknown or duplicate pending public consumer')
    if {row['id'] for row in all_consumers(parity)} != expected_consumers | {'mcp'}:
        raise ValueError('public variants differ from the settled surface contract')
    for kind in ('typed-result', 'result2', 'located'):
        if {case['verb'] for case in cases if case['kind'] == kind} != set(parity['functions']):
            raise ValueError(f'{kind}: a required function is missing')
    canonical = {case['id']: case for case in document['cases']}
    types = {case['name'] for case in json.loads((ROOT / 'specification/fixtures/types/corpus.json').read_text())['cases']}
    named = json.loads((ROOT / 'conformance/named-inputs.json').read_text())['cases']
    named_ids = [row['id'] for row in named]
    if not named_ids or len(named_ids) != len(set(named_ids)):
        raise ValueError('duplicate or empty named-input fixture IDs')
    settings = {case['id'] for case in json.loads((ROOT / 'conformance/settings.json').read_text())['cases']}
    if set(parity['functions']) != {case['verb'] for case in document['cases']}:
        raise ValueError('parity functions differ from the canonical ten')
    if len(parity['functions']) != len(set(parity['functions'])):
        raise ValueError('duplicate parity function')
    if {case['id'] for case in cases if case['kind'] == 'behavior'} != set(canonical) - {'25-defect-fault'}:
        raise ValueError('required behavior cases differ from canonical cases')
    registered = {line.split()[0] for line in (ROOT / 'sdlc/surfaces.txt').read_text().splitlines()
                  if line.strip() and not line.startswith('#')}
    if {row['surface'] for row in consumers} - {'.'} != registered:
        raise ValueError('public consumer inventory does not cover the surface registry')
    for consumer in consumers:
        if consumer['surface'] != '.' and not (ROOT / consumer['surface'] / 'check.sh').is_file():
            raise ValueError(f"{consumer['id']}: missing baseline runner")
    named_by_id = {row['id']: row for row in named}
    for case in cases:
        if case['verb'] not in parity['functions'] or not case['expect']:
            raise ValueError(f"{case['id']}: unknown verb or empty expectation")
        if 'consumers' in case and (not case['id'].startswith('mcp-question-') or case['consumers'] != ['mcp']):
            raise ValueError(f"{case['id']}: shared cases cannot exclude public variants")
        if set(case.get('consumers', [])) - {row['id'] for row in all_consumers(parity)}:
            raise ValueError(f"{case['id']}: unknown public variant")
        checks = case.get('checks', ['named', 'runtime'])
        if len(checks) != len(set(checks)) or set(checks) - {'named', 'compile', 'runtime'} or not {'named', 'runtime'} <= set(checks):
            raise ValueError(f"{case['id']}: invalid required checks")
        given = case['input']
        if 'scenarios_ref' in given:
            scenarios = parity['image_admission_scenarios'].get(given['scenarios_ref'])
            if not scenarios or any(row['profile_ref'] not in parity['image_profiles'] for row in scenarios):
                raise ValueError(f"{case['id']}: unknown image admission scenarios or profile")
        if 'case_ref' in given:
            source = {'conformance/settings.json': settings,
                      'specification/fixtures/types/corpus.json': types,
                      'conformance/named-inputs.json': set(named_ids)}.get(given.get('fixture'), canonical)
            if given['case_ref'] not in source:
                raise ValueError(f"{case['id']}: unknown case reference")
        if given.get('fixture') == 'conformance/named-inputs.json':
            fixture = named_by_id[given['case_ref']]
            if fixture['verb'] != case['verb'] or case['expect'] != {'fixture_ref': fixture['id']}:
                raise ValueError(f"{case['id']}: named-input expectation differs from independent fixture")
            source_case = fixture['input'].get('source_case')
            if source_case is not None and source_case not in canonical:
                raise ValueError(f"{case['id']}: unknown metadata baseline")
            for image in fixture['input'].get('images', []):
                if not (ROOT / image).is_file():
                    raise ValueError(f"{case['id']}: missing shared image fixture")
        for path in given.get('paths', []) + given.get('images', []):
            if not (ROOT / path).exists():
                raise ValueError(f"{case['id']}: missing input {path}")
        for owner in case['preconditions']:
            if not list((ROOT / 'sdlc/tickets').glob(owner + '-*.md')):
                raise ValueError(f"{case['id']}: unknown precondition {owner}")
    private_ids = {case['id'] for case in parity.get('private_cases', [])}
    if private_ids != {'25-defect-fault', 'boundary-defect'}:
        raise ValueError('private invariant coverage inventory differs from its safety boundary')
    schema_cases = parity.get('schema_cases', [])
    schema_ids = [case['id'] for case in schema_cases]
    if len(schema_ids) != len(set(schema_ids)) or set(schema_ids) & {case['id'] for case in cases}:
        raise ValueError('duplicate schema boundary case')
    corpus = {case['name']: case for case in json.loads(
        (ROOT / 'specification/fixtures/types/corpus.json').read_text())['cases']}
    for case in schema_cases:
        fixture = corpus.get(case['input']['case_ref'])
        if fixture is None or fixture.get('case_id') or fixture['name'] == 'described-choice':
            raise ValueError('schema boundary contains a real public call case')
    parity['unavailable_dependencies'] = {row['id']: row['dependency'] for row in pending
                                          if row['id'] not in expected_consumers
                                          or not (ROOT / row['surface'] / 'check.sh').is_file()}
    return parity


def cells(output, consumers, cases):
    """Read asserted named cells, never infer typed support from generic tests."""
    found = {}
    for line in output.splitlines():
        # Existing consumers already report these full-case counters.
        if re.search(r'\b(?:not_run|unselected)=[1-9][0-9]*\b', line):
            raise ValueError('required cases skipped or unselected')
        if 'not run' in line.lower() and any(case in line for case in cases):
            raise ValueError('required case reported not run')
        if not line.startswith(PREFIX):
            continue
        row = json.loads(line[len(PREFIX):])
        if not isinstance(row, dict) or set(row) != {'consumer', 'case', 'checks', 'status'}:
            raise ValueError('unexpected parity cell fields')
        key = (row['consumer'], row['case'])
        if row['consumer'] not in consumers or row['case'] not in cases:
            raise ValueError(f'unknown parity cell {key}')
        if row['consumer'] not in cases[row['case']].get('consumers', [row['consumer']]):
            raise ValueError(f'parity cell at wrong public variant {key}')
        if key in found:
            raise ValueError(f'duplicate parity cell {key}')
        required = cases[row['case']].get('checks', ['named', 'runtime'])
        if row['status'] not in ('pass', 'fail'):
            raise ValueError(f'{key}: skipped or unknown status')
        if not isinstance(row['checks'], list) or not all(isinstance(check, str) for check in row['checks']):
            raise ValueError(f'{key}: invalid checks')
        if set(row['checks']) != set(required) or len(row['checks']) != len(required):
            raise ValueError(f'{key}: missing named/compiler/runtime check')
        found[key] = row['status']
    return found


def consumer_environment(scratch, port):
    # Preserve tool locations before replacing HOME; never copy user config.
    names = ('PATH', 'LANG', 'LC_ALL', 'TMPDIR', 'XDG_RUNTIME_DIR',
             'RUSTUP_TOOLCHAIN', 'RUSTC_WRAPPER', 'SCCACHE_CONF',
             'THINKTHEN_HEAVY_LOCK', 'THINKTHEN_HEAVY_LOCK_HELD',
             'THINKTHEN_TOOLCHAINS', 'THINKTHEN_DUCKDB_CLI',
             'SQLITE_AMALGAMATION', 'THINKTHEN_PRIVATE_NAMES',
             'R_LIBS_USER', 'PUB_CACHE')
    env = {name: value for name in names if (value := os.environ.get(name)) is not None}
    for name, fallback in [('CARGO_HOME', '.cargo'), ('RUSTUP_HOME', '.rustup')]:
        env[name] = os.environ.get(name, str(Path.home() / fallback))
    for name, folder in [('HOME', 'home'), ('XDG_CONFIG_HOME', 'config'),
                         ('XDG_CACHE_HOME', 'cache'), ('XDG_STATE_HOME', 'state'),
                         ('APPDATA', 'appdata'), ('LOCALAPPDATA', 'localappdata')]:
        path = Path(scratch) / folder
        path.mkdir()
        env[name] = str(path)
    if toolchains := env.get('THINKTHEN_TOOLCHAINS'):
        cache = Path(env['HOME']) / '.cache'
        cache.mkdir()
        (cache / 'thinkthen-toolchains').symlink_to(Path(toolchains).expanduser().resolve(),
                                                   target_is_directory=True)
    env.update(CARGO_BUILD_JOBS=os.environ.get('CARGO_BUILD_JOBS', '1'), CARGO_NET_OFFLINE='true',
               THINKTHEN_TEST_PROFILE='full',
               THINKTHEN_BASE_URL=f'http://127.0.0.1:{port}/generic/v1',
               THINKTHEN_API_KEY='sk-conformance-loopback')
    return env


def summarize(consumer, cases, code, seen, error, log):
    invalid = bool(error) or code not in (0, 1)
    states = {case: ('fail' if invalid else seen.get((consumer, case), 'missing'))
              for case in cases}
    typed = [case['verb'] for case in cases.values()
             if case['kind'] == 'typed-result' and states[case['id']] == 'pass']
    checked = any((consumer, case) in seen for case in cases) and not invalid
    return {'consumer': consumer, 'baseline_exit': code, 'baseline_error': error,
            'named_typed_functions': typed if checked else None,
            'adopted_case_count': sum(state == 'pass' for state in states.values()),
            'cells': states, 'log': log}


def support_table(parity, matrix):
    cases = {case['id']: case for case in parity['required_cases']}
    consumers = {row['id']: row for row in all_consumers(parity)}
    lines = ['| Binding | Functions working out of ten | Files | Images | What is left |',
             '| --- | --- | --- | --- | --- |']
    for row in matrix:
        def capability(kinds):
            selected = [state for key, state in row['cells'].items() if cases[key]['kind'] in kinds]
            passed = sum(state == 'pass' for state in selected)
            if not passed:
                return 'not checked' if all(state == 'missing' for state in selected) else 'failed'
            return 'works' if passed == len(selected) else f'{passed}/{len(selected)} assertions; gaps'
        count = ('not checked' if row['named_typed_functions'] is None
                 else f"{len(row['named_typed_functions'])}/10")
        gaps = [key for key, state in row['cells'].items() if state != 'pass']
        owners = sorted({owner for key in gaps for owner in cases[key]['preconditions']})
        remaining = (f"{len(gaps)} required assertions; owner {consumers[row['consumer']]['owner']}"
                     + (f"; dependencies {', '.join(owners)}" if owners else '')) if gaps else 'none'
        if row['baseline_exit'] is not None and row['baseline_exit'] != 0:
            remaining += f"; consumer exit {row['baseline_exit']}"
        if row['baseline_error']:
            remaining += '; ' + row['baseline_error'].replace('|', '/').replace('\n', ' ')
        rulings = consumers[row['consumer']].get('rulings', [])
        files = capability({'located'})
        images = capability({'images', 'image-location', 'image-admission', 'refusal'})
        # Written limits describe the public contract, never pass an assertion.
        images += '; text-only: tag/filter/rank/annotate/find/recognize/relate'
        if rulings:
            remaining += '; written ruling: ' + '; '.join(rulings)
        lines.append(f"| {row['consumer']} | {count} | {files} | {images} | {remaining} |")
    return '\n'.join(lines) + '\n'


def run(port, baseline=False):
    if not re.fullmatch(r'[0-9]+', port) or not 0 < int(port) < 65536:
        raise ValueError('parity needs an owned loopback port')
    parity = inventory()
    if os.environ.get('THINKTHEN_CONFORMANCE_IDS') is not None:
        raise ValueError('strict parity refuses a case selector')
    cases = {case['id']: case for case in parity['required_cases']}
    consumers = {row['id']: row for row in all_consumers(parity)}
    output_dir = ROOT / 'target/parity'
    output_dir.mkdir(parents=True, exist_ok=True)
    commands = {}
    for row in consumers.values():
        command = ['python3', 'conformance/c_parity.py'] if row['id'] == 'c' else row['baseline']
        commands.setdefault(tuple(command), []).append(row['id'])
    matrix = []
    for command, ids in commands.items():
        label = '+'.join(ids)
        log = output_dir / (label + '.log')
        code, seen, error = None, {}, None
        execute = not baseline or set(ids) <= {'cli', 'rust', 'c'}
        if execute:
            print(f'parity consumer: {label}', flush=True)
            args = list(command) + ([] if command[0] == 'cargo' else [port])
            with tempfile.TemporaryDirectory(prefix='thinkthen-parity-') as scratch, log.open('w') as stream:
                env = consumer_environment(scratch, port)
                try:
                    process = subprocess.run(['sh', 'sdlc/scripts/time-limit', '1800', *args],
                                             cwd=ROOT, env=env, stdout=stream,
                                             stderr=subprocess.STDOUT, check=False)
                    code = process.returncode
                except OSError:
                    error = 'required runner or toolchain unavailable'
            try:
                seen = cells(log.read_text(), ids, cases)
            except (ValueError, KeyError, TypeError) as failure:
                error = str(failure)
            print(f'parity consumer: {label} exit={code}; asserted cells={len(seen)}', flush=True)
        for consumer in ids:
            dependency = parity.get('unavailable_dependencies', {}).get(consumer)
            row = summarize(consumer, required_cases(parity, consumer), code, seen, error or dependency,
                            str(log.relative_to(ROOT)) if execute else None)
            matrix.append(row)
    (output_dir / 'matrix.json').write_text(json.dumps(matrix, indent=2) + '\n')
    table = support_table(parity, matrix)
    (output_dir / 'matrix.md').write_text(table)
    print(table, end='')
    return int(any(row['baseline_exit'] != 0 or row['baseline_error'] or any(state != 'pass' for state in row['cells'].values()) for row in matrix))


def main():
    if sys.argv[1:] == ['--validate']:
        parity = inventory()
        print(f"parity inventory: {len(all_consumers(parity))} public consumers; "
              f"{len(parity['required_cases'])} required cases; declarations valid")
        for consumer, dependency in parity['unavailable_dependencies'].items():
            print(f'parity dependency: {consumer}: {dependency}')
        return 0
    if len(sys.argv) == 3 and sys.argv[2] == '--baseline':
        return run(sys.argv[1], baseline=True)
    if len(sys.argv) == 2:
        return run(sys.argv[1])
    raise ValueError('usage: parity.py --validate|PORT [--baseline] (called by surfaces)')


if __name__ == '__main__':
    sys.exit(main())
