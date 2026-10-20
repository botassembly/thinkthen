"""One packet oracle, shared request preparation and decimal-safe comparison.

Dumpers return {"packets": [native packet JSON, ...]} and optional host
conversions. Only this comparator normalizes runtime identities and timing.
"""
import difflib
import json
import os
from pathlib import Path
import re
import sys
from decimal import Decimal

ROOT = Path(__file__).resolve().parents[2]
ORACLE = ROOT / 'conformance/fixtures/packets.json'
PRIVATE_CASE = '25-defect-fault'
IDENTITIES = {'call_id', 'observation_id', 'failure_id', 'answer_id',
              'sdk_request_id', 'request_sha256'}


def decimal_json(text):
    """Keep JSON numbers as exact decimal strings, including large integers."""
    def number(raw):
        value = Decimal(raw)
        text = format(value, 'f')
        return (text.rstrip('0').rstrip('.') if '.' in text else text) if value else '0'
    return json.loads(text, parse_int=number, parse_float=number)


def canonical(document):
    """Keep packet structure, presence and identity equality; remove run entropy."""
    identities = {}

    def visit(value, key=''):
        if isinstance(value, dict):
            return {name: visit(child, name) for name, child in sorted(value.items())}
        if isinstance(value, list):
            return [visit(child, 'request_sha256' if key == 'requests' else key)
                    for child in value]
        if key in IDENTITIES and isinstance(value, str) and re.fullmatch('[0-9a-f]{64}', value):
            group = 'request_sha256' if key == 'requests' else key
            table = identities.setdefault(group, {})
            return table.setdefault(value, f'{group}:{len(table) + 1}')
        if key in ('seconds', 'wall_ms') and isinstance(value, str):
            assert Decimal(value) >= 0, (key, value)
            return '0'
        if key == 'url' and isinstance(value, str):
            return re.sub(r'http://127\.0\.0\.1:\d+/', 'http://127.0.0.1:PORT/', value)
        return value

    return visit(document)


def difference(want, got):
    """Report the changed packet fields as a readable unified JSON diff."""
    left = json.dumps(want, ensure_ascii=False, sort_keys=True, indent=2).splitlines(True)
    right = json.dumps(got, ensure_ascii=False, sort_keys=True, indent=2).splitlines(True)
    return ''.join(difflib.unified_diff(left, right, fromfile='oracle', tofile='actual'))


def folded(document):
    """Reuse existing shared assertions over native packets, without a host serializer."""
    if 'host' in document:
        return document['host']
    packets = document['packets']
    terminal = next((packet for packet in packets if packet['kind'] == 'terminal'), {})
    failure = document.get('error', terminal.get('failure'))
    values = [packet['value'] for packet in packets if packet['kind'] == 'row']
    aggregate = next((packet['value'] for packet in packets if packet['kind'] == 'aggregate'), None)
    results = aggregate if aggregate is not None else values
    packet = {'native': True, 'results': results, 'facts': terminal.get('facts'),
              'answer': terminal.get('answer')}
    if failure is not None:
        prefix = packet
        packet = {'error': failure['error'], 'facts': failure.get('facts', terminal.get('facts')),
                  'resolution': failure['resolution']}
        if results:
            packet['completed'] = prefix
    return packet


def run(consumer, command, root, *, update=False):
    """Keep every required Rust cell, its probes and assertions, then compare packets."""
    from fixture_suite import run as shared_run
    actual = {}

    def record(stdout, row, step, home):
        # File locations use this run's owned home or this checkout. Keep their
        # relative names while replacing only the machine-specific prefixes.
        raw = stdout.replace(str(home), '$FOLDER').replace(str(root), '$ROOT')
        actual.setdefault(row['id'], []).append(decimal_json(raw))
        return folded(json.loads(stdout))

    failures = shared_run(consumer, command, root, packet_transform=record)
    if failures:
        return failures
    document = {'schema': 'thinkthen.packet-oracle/1', 'cases': [
        {'id': case, 'dumps': canonical(dumps)} for case, dumps in actual.items()],
        'private_cases': [{'id': PRIVATE_CASE, 'reason':
            'Private invariant injection has no admitted public input and is outside parity.required_cases.'}]}
    if update:
        ORACLE.write_text(json.dumps(document, ensure_ascii=False, sort_keys=True, indent=2) + '\n')
        return 0
    oracle = json.loads(ORACLE.read_text())
    expected = {case['id']: case['dumps'] for case in oracle['cases']}
    failed = False
    for case in document['cases']:
        want = expected.get(case['id'])
        if want != case['dumps']:
            failed = True
            print(case['id'] + ':\n' + difference(want, case['dumps']), file=sys.stderr)
    if '--check' in sys.argv and document != oracle:
        failed = True
        print(difference(oracle, document), file=sys.stderr)
    return int(failed)


def main():
    if '--check' not in sys.argv or '--worker' not in sys.argv:
        raise SystemExit('use --check --worker TEST_BINARY [--update]')
    worker = sys.argv[sys.argv.index('--worker') + 1]
    os.environ.pop('THINKTHEN_CONFORMANCE_IDS', None)
    command = [worker, '--ignored', '--exact', 'packet_dumper_worker', '--nocapture']
    return run('rust', command, ROOT, update='--update' in sys.argv)


if __name__ == '__main__':
    sys.exit(main())
