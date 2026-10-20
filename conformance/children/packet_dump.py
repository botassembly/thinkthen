"""One packet oracle, shared request preparation and decimal-safe comparison.

Dumpers return {"packets": [native packet JSON, ...]} and optional host
conversions. Only this comparator normalizes runtime identities and timing.
"""
import base64
import difflib
import gzip
import io
import json
import os
from pathlib import Path
import sys
from decimal import Decimal
from packet_normalize import canonical

ROOT = Path(__file__).resolve().parents[2]
ORACLE = ROOT / 'conformance/fixtures/packets.json'
PRIVATE_CASE = '25-defect-fault'


def decimal_json(text):
    """Keep JSON numbers as exact decimal strings, including large integers."""
    def number(raw):
        value = Decimal(raw)
        text = format(value, 'f')
        return (text.rstrip('0').rstrip('.') if '.' in text else text) if value else '0'
    return json.loads(text, parse_int=number, parse_float=number)


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


def stored(document):
    """Store large strings losslessly while leaving ordinary packet JSON readable."""
    # Image boundary cases repeat padded inputs into a 1.2 GB expanded oracle.
    # Paths belong to this storage envelope, never to authored packet objects.
    compressed = []

    def visit(value, path):
        if isinstance(value, dict):
            return {key: visit(child, [*path, key]) for key, child in value.items()}
        if isinstance(value, list):
            return [visit(child, [*path, index]) for index, child in enumerate(value)]
        if isinstance(value, str) and len(value) >= 16384:
            buffer = io.BytesIO()
            with gzip.GzipFile(filename='', mode='wb', fileobj=buffer, mtime=0) as writer:
                writer.write(value.encode('utf-8'))
            payload = base64.b64encode(buffer.getvalue()).decode('ascii')
            if len(payload) < len(value):
                compressed.append({'path': path, 'gzip_base64': payload})
                return None
        return value

    result = visit(document, [])
    if compressed:
        result['compressed_strings'] = compressed
    return result


def expanded(document):
    """Restore exact strings before comparing any native packet value or type."""
    for entry in document.pop('compressed_strings', []):
        parent = document
        for key in entry['path'][:-1]:
            parent = parent[key]
        key = entry['path'][-1]
        if parent[key] is not None:
            raise ValueError('compressed oracle path must contain its null placeholder')
        parent[key] = gzip.decompress(base64.b64decode(
            entry['gzip_base64'], validate=True)).decode('utf-8')
    return document


def run(consumer, command, root, *, update=False):
    """Keep every required Rust cell, its probes and assertions, then compare packets."""
    from fixture_suite import run as shared_run
    oracle = None if update else expanded(json.loads(ORACLE.read_text()))
    actual = {}
    folders = {}

    def record(stdout, row, step, home):
        # File locations use this run's owned home or this checkout. Keep their
        # relative names while replacing only the machine-specific prefixes.
        dump = decimal_json(stdout)
        actual.setdefault(row['id'], []).append(dump)
        folders[row['id']] = ((str(home), '$FOLDER'), (str(root), '$ROOT'))
        return folded(json.loads(stdout))

    failures = shared_run(consumer, command, root, packet_transform=record)
    if failures:
        return failures
    document = {'schema': 'thinkthen.packet-oracle/1', 'cases': [
        {'id': case, 'dumps': canonical(dumps, folders[case])} for case, dumps in actual.items()],
        'private_cases': [{'id': PRIVATE_CASE, 'reason':
            'Private invariant injection has no admitted public input and is outside parity.required_cases.'}]}
    if update:
        ORACLE.write_text(json.dumps(stored(document), ensure_ascii=False, sort_keys=True, indent=2) + '\n')
        return 0
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
