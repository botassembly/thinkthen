#!/usr/bin/env python3
"""Pinned 182-row annotate comparison. The live wrapper reserves before this job starts."""

import contextlib
import hashlib
import io
import json
import os
import signal
import subprocess
import sys
import tempfile
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from probes import paid_evidence as evidence
from probes.speed import measure as speed

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
BENCH_SHA = 'ed8ffe498a21f9f60d5b5905008e1b3c249d87bb'
SUITE_SHA = '0c85011cdfb2c938636a7472d23831db7ca9e001f2d7eab74c20714fce5da13a'
IDS_SHA = 'a02b28178b2f20eb729eebb9098e3a178197bdc7b289c7fb6b00f2a926f8595d'
SELECTED_SHA = '76bd1db0ec0efd27287309502b0e8fe35fa2c64346db134576b5442fbd22ea70'
INPUT_SHA = 'e0f171ecd0932817c6090c682f3599f6541a5c84166608f47cac9f6838775270'
TRUTH_SHA = '3f98ca8148c4f26c83d963d3e3a5c47d3e44feaee9599bf9d0104bce00d04450'
SET_SHA = '53f5431e651d0ce6c45e361a7112103f66454000ccd702676c06af39b9c160d9'
MODEL = 'jev-latest'
RESERVATION = 750_000
STOP_TOKENS = 700_000
ARMS = ('batch1', 'default')
PRICE_INPUT_MILLION = 0.042  # Pinned historical rate; output was free in the admitted evidence.
EXPECTED_BINARY_SHA = 'decf1d5ff825a340cbf229fe1f0e1b24fe05784dc77482e4dc2b75fbc0886264'
EXPECTED_PLAN = {
    'batch1': (364, (182, 182), 180434, 364, 716, 202955,
               'e88baa6676da6352c16e5e9c767e51450c89dba34df978f41c574fdfd97651ae'),
    'default': (3, (1, 2), 188788, 364, 95459, 210540,
                '9752dd2be863cdfb37640114b48711d84c41e7da6f7fa60863ce06a907d3f628'),
}


def refuse(message):
    print(f'annotate batching: {message}', file=sys.stderr)
    raise SystemExit(2)


def digest(value):
    return hashlib.sha256(json.dumps(value, ensure_ascii=False, separators=(',', ':')).encode()).hexdigest()


def sha(payload):
    return hashlib.sha256(payload).hexdigest()


def sample(bench):
    if speed.git('rev-parse', 'HEAD', cwd=bench) != BENCH_SHA or speed.git('status', '--porcelain', cwd=bench):
        refuse('the benchmark revision or clean state changed')
    suite = bench / 'questions/suite/annotate.jsonl'
    if sha(suite.read_bytes()) != SUITE_SHA:
        refuse('the benchmark suite bytes changed')
    rows = [json.loads(line) for line in suite.read_text(encoding='utf-8').splitlines()]
    if len(rows) != 182 or [row['id'] for row in rows] != [f'annotate-card-{n:03d}' for n in range(1, 183)]:
        refuse('the ordered 182-case IDs changed')
    selected = [{'id': row['id'], 'input': row['records'][0]['input']} for row in rows]
    truth = [{'id': row['id'], 'before_1965': int(row['truth']['year']) < 1965,
              'album': row['truth']['album']} for row in rows]
    data = ''.join(json.dumps(row, ensure_ascii=False, separators=(',', ':')) + '\n'
                   for row in selected).encode()
    questions = json.loads((HERE / 'questions.json').read_text(encoding='utf-8'))
    source_options = json.loads((bench / 'questions/suite/annotate-card.json').read_text())['questions']['album']
    if (digest([row['id'] for row in rows]) != IDS_SHA or digest(selected) != SELECTED_SHA
            or sha(data) != INPUT_SHA or digest(truth) != TRUTH_SHA or digest(questions) != SET_SHA
            or questions['questions']['album']['choose'] != source_options['choose']
            or questions['questions']['album']['options'] != source_options['options']
            or any(item['album'] not in source_options['options'] for item in truth)
            or sum(item['before_1965'] for item in truth) != 55):
        refuse('the selected records, questions or truth changed')
    return selected, truth, data


def command(binary, arm, local_url=None, record=None):
    args = [str(binary), 'annotate', str(HERE / 'questions.json'), '--jsonl', '--details', '--facts',
            '--jobs', '1', '--max-retries', '0', '--no-cache', '--model', MODEL]
    if arm == 'batch1':
        args += ['--batch', '1']
    if local_url:
        args += ['--url', local_url]
    if record:
        args += ['--record', str(record)]
    return args


def checked_rows(rows, selected):
    if len(rows) != 182:
        refuse('an arm lost result rows')
    for actual, expected in zip(rows, selected):
        if not isinstance(actual, dict) or actual.get('input') != expected:
            refuse('an arm lost or reordered selected records')
        answers = actual.get('answers')
        value = actual.get('value')
        if (not isinstance(answers, dict) or set(answers) != {'before_1965', 'album'}
                or not isinstance(value, dict) or set(value) != set(answers)
                or any('failure' in entry or entry.get('value') != value[name]
                       for name, entry in answers.items() if isinstance(entry, dict))
                or any(not isinstance(entry, dict) for entry in answers.values())):
            refuse('an arm has missing or failed question answers')
        if type(value['before_1965']) not in (bool, type(None)) or type(value['album']) not in (str, type(None)):
            refuse('an arm has an unexpected answer type')
        meta = actual.get('meta')
        if (not isinstance(meta, dict) or not isinstance(meta.get('requests'), list)
                or len(meta['requests']) != 2 or any(not isinstance(item, str) for item in meta['requests'])
                or not isinstance(meta.get('questions_sha256'), str)):
            refuse('an arm lacks two ordered group request identities')


def local_plan(bench, binary):
    selected, unused_truth, data = sample(bench)
    if not binary.exists():
        refuse('build the exact compiled source before the offline plan')
    if sha(binary.read_bytes()) != EXPECTED_BINARY_SHA:
        refuse('the compiled binary differs from the reviewed offline source')
    bodies = []
    guard = threading.Lock()

    class Handler(BaseHTTPRequestHandler):
        def do_POST(self):
            body = self.rfile.read(int(self.headers['Content-Length']))
            request = json.loads(body)
            with guard:
                bodies.append(body)
            answers = {}
            for name, question in request['questions'].items():
                if question['type'] == 'noul':
                    answers[name] = {'type': 'noul', 'noul': 0.9}
                elif question['type'] == 'choice':
                    choices = list(question['criteria'])
                    answers[name] = {'type': 'choice', 'probabilities': {item: float(item == choices[0])
                                                                      for item in choices}}
                else:
                    refuse('the loopback saw an unexpected question type')
            reply = json.dumps({'model': MODEL, 'answers': answers,
                                'usage': {'input_tokens': 3, 'output_tokens': 2}}, separators=(',', ':')).encode()
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Content-Length', str(len(reply)))
            self.end_headers()
            self.wfile.write(reply)

        def log_message(self, *unused):
            pass

    server = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    plans = {}
    try:
        for arm in ARMS:
            first = len(bodies)
            args = command(binary, arm, f'http://127.0.0.1:{server.server_port}')
            done = subprocess.run(args, input=data, capture_output=True,
                                  env={**speed.plain(), 'THINKTHEN_API_KEY': 'sk-loopback-only'}, timeout=180)
            if done.returncode:
                refuse(f'offline {arm} exited {done.returncode}: {done.stderr.decode(errors="replace")[:350]}')
            rows = [json.loads(line) for line in done.stdout.splitlines()]
            checked_rows(rows, selected)
            resolved = {row['meta']['questions_sha256'] for row in rows}
            if len(resolved) != 1:
                refuse('the offline rows disagree on the resolved question set')
            captured = bodies[first:]
            groups = {'before_1965': [], 'album': []}
            for body in captured:
                request = json.loads(body)
                kinds = {question['type'] for question in request['questions'].values()}
                if kinds == {'noul'}:
                    groups['before_1965'].append(body)
                elif kinds == {'choice'}:
                    groups['album'].append(body)
                else:
                    refuse('an offline request mixed independent on groups')
            if arm == 'batch1' and [len(groups[name]) for name in groups] != [182, 182]:
                refuse('batch-one changed its one-request-per-row group shape')
            if not all(groups.values()):
                refuse('an offline arm omitted a selected on group')
            if any(len(body) > 96_000 for body in captured):
                refuse('an offline body exceeded the normal request ceiling')
            # The B9 estimate is a scheduling margin over measured wire bytes, not a runtime cap.
            size = sum(map(len, captured))
            wires = sum(len(json.loads(body)['questions']) for body in captured)
            bound = int(0.908 * size + 80 * wires + 0.999999) + 10_000
            plans[arm] = {'requests': len(captured), 'groups': {name: len(part) for name, part in groups.items()},
                          'body_bytes': size, 'wire_questions': wires,
                          'max_body_bytes': max(map(len, captured)), 'combined_token_bound': bound,
                          'resolved_questions_sha256': resolved.pop(),
                          'body_sha256': sha(b''.join(captured)),
                          'request_sha256': digest([sha(body) for body in captured])}
            observed = (plans[arm]['requests'], tuple(plans[arm]['groups'].values()),
                        size, wires, plans[arm]['max_body_bytes'], bound, plans[arm]['body_sha256'])
            if observed != EXPECTED_PLAN[arm]:
                refuse(f'offline {arm} changed its reviewed request shape or exact body bytes')
        if plans['batch1']['resolved_questions_sha256'] != plans['default']['resolved_questions_sha256']:
            refuse('batch selection changed the resolved question set')
        if sum(plan['combined_token_bound'] for plan in plans.values()) > STOP_TOKENS:
            refuse('the measured full-cohort bounds exceed the between-arm budget')
    finally:
        server.shutdown()
        server.server_close()
        thread.join()
    return plans


def arm_run(binary, selected, truth, data, arm, folder, plan, key):
    folder.mkdir()
    evidence.sync_dir(folder.parent)
    home = folder / 'home'
    home.mkdir()
    evidence.sync_dir(folder)
    env = {**speed.plain(), 'HOME': str(home), 'THINKTHEN_API_KEY': key}
    before = evidence.totals(binary, env, refuse, folder / 'before-status')
    evidence.durable_json(folder / 'before.json', before)
    evidence.durable_bytes(folder / 'input.jsonl', data)
    args = command(binary, arm, record=folder / 'recording')
    evidence.durable_json(folder / 'started.json', {'arm': arm, 'command': args,
        'input_sha256': INPUT_SHA, 'set_sha256': SET_SHA, 'binary_sha256': sha(binary.read_bytes()),
        'planned_body_sha256': plan['body_sha256']})
    stdout_path, stderr_path = folder / 'stdout.jsonl', folder / 'stderr.txt'
    interrupted = False
    with (folder / 'input.jsonl').open('rb') as stream, stdout_path.open('xb') as out, stderr_path.open('xb') as err:
        evidence.sync_dir(folder)
        start = time.monotonic()
        child = subprocess.Popen(args, stdin=stream, stdout=out, stderr=err, env=env)
        try:
            code = child.wait()
        except (KeyboardInterrupt, InterruptedError):
            interrupted = True
            child.terminate()
            try:
                code = child.wait(timeout=10)
            except subprocess.TimeoutExpired:
                child.kill()
                code = child.wait()
        seconds = round(time.monotonic() - start, 3)
        out.flush(); err.flush()
        os.fsync(out.fileno()); os.fsync(err.fileno())
    raw = b'\n'.join(line for line in stderr_path.read_bytes().splitlines() if line.startswith(b'{'))
    evidence.durable_bytes(folder / 'final-facts-raw.jsonl', raw + (b'\n' if raw else b''))
    try:
        after = evidence.totals(binary, env, refuse, folder / 'after-status')
        status_error = None
    except (Exception, SystemExit) as error:
        after = None
        status_error = type(error).__name__
    delta = ({name: after[name] - before[name] for name in ('requests_sent', 'input_tokens', 'output_tokens')}
             if after is not None else None)
    unknown = delta is None or code != 0 or interrupted or delta['requests_sent'] > plan['requests']
    evidence.durable_json(folder / 'completion.json', {'arm': arm, 'returncode': code, 'interrupted': interrupted,
        'seconds': seconds, 'after_status': after, 'status_error': status_error, 'reported_delta': delta,
        'attempts_observed': None if delta is None else delta['requests_sent'], 'usage_validated': False,
        'unknown_refusal_cost': unknown})
    if interrupted or code != 0 or delta is None:
        refuse(f'{arm} is incomplete; retain its named raw arm and do not rerun blindly')
    try:
        rows = [json.loads(line) for line in stdout_path.read_bytes().splitlines()]
        facts = [json.loads(line) for line in raw.splitlines()]
    except (ValueError, TypeError):
        refuse(f'{arm} has malformed output; retain its named arm')
    evidence.durable_json(folder / 'facts.json', facts)
    checked_rows(rows, selected)
    if any(row['meta']['questions_sha256'] != plan['resolved_questions_sha256'] for row in rows):
        refuse(f'{arm} changed the resolved question set')
    if len(facts) != 1:
        refuse(f'{arm} lacks one final facts row')
    evidence.checked_facts(facts[0], delta, 182, refuse)
    attempt_limit = plan['requests'] if arm == 'batch1' else 3 * plan['requests']
    if not 1 <= delta['requests_sent'] <= attempt_limit:
        refuse(f'{arm} has unexpected attempts')
    combined = evidence.checked_usage(delta, rows, refuse)
    by_group = {name: len({row['meta']['requests'][place] for row in rows})
                for place, name in enumerate(('before_1965', 'album'))}
    if arm == 'batch1' and by_group != {'before_1965': 182, 'album': 182}:
        refuse('batch-one changed its per-group request identities')
    answer_rows = [[item['id'], row['value']] for item, row in zip(selected, rows)]
    result = {'arm': arm, 'records': 182, 'group_requests_planned': plan['groups'],
        'group_completed_requests': by_group,
        'requests_sent': delta['requests_sent'], 'reported_input_tokens': delta['input_tokens'],
        'reported_output_tokens': delta['output_tokens'], 'reported_combined_tokens': combined,
        'unknown_refusal_cost': unknown, 'usage_scope': 'reported_tokens_only', 'seconds': seconds,
        'correct': {name: sum(row['value'][name] == key[name] for row, key in zip(rows, truth))
                    for name in ('before_1965', 'album')},
        'answers_sha256': digest(answer_rows), 'request_ids_sha256': digest([row['meta']['requests'] for row in rows]),
        'ids_sha256': IDS_SHA, 'selected_sha256': SELECTED_SHA, 'input_sha256': INPUT_SHA,
        'set_sha256': SET_SHA, 'truth_sha256': TRUTH_SHA, 'bench': BENCH_SHA,
        'resolved_questions_sha256': plan['resolved_questions_sha256'],
        'build': speed.git('rev-parse', 'HEAD'), 'binary_sha256': sha(binary.read_bytes()),
        'historical_input_rate_per_million_usd': PRICE_INPUT_MILLION,
        'historical_reported_input_cost_usd': round(delta['input_tokens'] * PRICE_INPUT_MILLION / 1_000_000, 8)}
    evidence.durable_json(folder / 'validated.json', result)
    return result


def self_test(bench):
    selected, truth, data = sample(bench)
    with tempfile.TemporaryDirectory(prefix='thinkthen-b10-helper-') as temporary:
        folder = Path(temporary)
        fake = folder / 'fake.py'
        fake.write_text('''#!/usr/bin/env python3
import json, os, pathlib, sys
home = pathlib.Path(os.environ['HOME'])
if sys.argv[1] == 'status':
    sent = (home / 'sent').exists()
    print(json.dumps({'usage': {'total': {'requests_sent': int(sent), 'input_tokens': 0, 'output_tokens': 0}}}))
else:
    (home / 'sent').write_text('attempted')
    print('{"partial":true}')
    print('synthetic refusal', file=sys.stderr)
    raise SystemExit(4)
''')
        fake.chmod(0o700)
        plan = {'requests': 364, 'body_sha256': 'synthetic'}
        with contextlib.redirect_stderr(io.StringIO()):
            try:
                arm_run(fake, selected, truth, data, 'batch1', folder / 'failed', plan, 'sk-synthetic-only')
            except SystemExit as error:
                assert error.code == 2
            else:
                raise AssertionError('the failed synthetic arm passed')
        saved = folder / 'failed'
        assert (saved / 'stdout.jsonl').read_text().strip() == '{"partial":true}'
        assert 'synthetic refusal' in (saved / 'stderr.txt').read_text()
        assert (saved / 'home/sent').exists()
        assert json.loads((saved / 'completion.json').read_text())['attempts_observed'] == 1
        assert (saved / 'before-status-stdout.json').exists() and (saved / 'after-status-stdout.json').exists()
        assert (saved / 'final-facts-raw.jsonl').exists()
        sleeper = folder / 'sleeper.py'
        sleeper.write_text('''#!/usr/bin/env python3
import json, sys, time
if sys.argv[1] == 'status':
    print(json.dumps({'usage': {'total': {'requests_sent': 0, 'input_tokens': 0, 'output_tokens': 0}}}))
else:
    print('{"partial":true}', flush=True)
    time.sleep(10)
''')
        sleeper.chmod(0o700)
        real_popen = subprocess.Popen

        class InterruptedChild:
            def __init__(self, child):
                self.child = child
                self.first = True

            def wait(self, *args, **kwargs):
                if self.first:
                    self.first = False
                    raise InterruptedError('synthetic interrupt')
                return self.child.wait(*args, **kwargs)

            def terminate(self):
                self.child.terminate()

            def kill(self):
                self.child.kill()

        def launch(args, *args_rest, **kwargs):
            child = real_popen(args, *args_rest, **kwargs)
            return InterruptedChild(child) if len(args) > 1 and args[1] == 'annotate' else child

        with mock.patch.object(subprocess, 'Popen', side_effect=launch):
            with contextlib.redirect_stderr(io.StringIO()):
                try:
                    arm_run(sleeper, selected, truth, data, 'batch1', folder / 'interrupted', plan,
                            'sk-synthetic-only')
                except SystemExit as error:
                    assert error.code == 2
                else:
                    raise AssertionError('the interrupted synthetic arm passed')
        interrupted = folder / 'interrupted'
        assert json.loads((interrupted / 'completion.json').read_text())['interrupted'] is True
        assert (interrupted / 'stdout.jsonl').exists() and (interrupted / 'stderr.txt').exists()
        assert (interrupted / 'before-status-stdout.json').exists()
        assert (interrupted / 'after-status-stdout.json').exists()
        assert (interrupted / 'final-facts-raw.jsonl').exists()
        for bad in ({'input_tokens': 3}, {'input_tokens': 3, 'output_tokens': 2}):
            with contextlib.redirect_stderr(io.StringIO()):
                try:
                    evidence.checked_usage(bad, [{'meta': {'usage': {'input_tokens': 3}}}], refuse)
                except SystemExit as error:
                    assert error.code == 2
                else:
                    raise AssertionError('missing usage passed')
        with contextlib.redirect_stderr(io.StringIO()):
            try:
                evidence.checked_facts({'schema': 'thinkthen.run/1', 'records': 182,
                    'requests_sent': 1, 'input_tokens': 3},
                    {'requests_sent': 1, 'input_tokens': 3, 'output_tokens': 1}, 182, refuse)
            except SystemExit as error:
                assert error.code == 2
            else:
                raise AssertionError('missing final usage passed')
    print(json.dumps({'records': len(selected), 'ids_sha256': IDS_SHA, 'input_sha256': INPUT_SHA,
                      'set_sha256': SET_SHA, 'truth_sha256': TRUTH_SHA}))


def main(bench, name):
    if not name or any(char not in 'abcdefghijklmnopqrstuvwxyz0123456789-' for char in name):
        refuse('NAME needs lowercase letters, digits or hyphens')
    if os.environ.get('THINKTHEN_BASE_URL') or not os.environ.get('THINKTHEN_API_KEY'):
        refuse('the built-in address and a key are required')
    if speed.git('status', '--porcelain', '--untracked-files=no'):
        refuse('commit the exact source before a paid job')
    binary = ROOT / 'target/debug/thinkthen'
    if not binary.exists() or binary.stat().st_mtime < int(speed.git('log', '-1', '--format=%ct', '--', *speed.SOURCES)):
        refuse('build the current compiled source before a paid job')
    selected, truth, data = sample(bench)
    plans = local_plan(bench, binary)
    output = ROOT / 'target/annotate-batching' / name
    if output.exists():
        refuse('the named job already started; inspect its raw state before any repeat')
    output.parent.mkdir(parents=True, exist_ok=True)
    output.mkdir()
    evidence.sync_dir(output.parent)
    evidence.durable_json(output / 'started.json', {'name': name, 'arms': ARMS,
        'build': speed.git('rev-parse', 'HEAD'), 'binary_sha256': sha(binary.read_bytes()),
        'bench': BENCH_SHA, 'input_sha256': INPUT_SHA, 'set_sha256': SET_SHA,
        'truth_sha256': TRUTH_SHA, 'plans': plans, 'reservation_tokens': RESERVATION,
        'stop_reported_tokens': STOP_TOKENS})

    def interrupt(signum, unused):
        raise InterruptedError(f'signal {signum}')

    signal.signal(signal.SIGINT, interrupt)
    signal.signal(signal.SIGTERM, interrupt)
    spent = 0
    for arm in ARMS:
        bound = plans[arm]['combined_token_bound']
        if spent + bound > STOP_TOKENS:
            evidence.durable_json(output / 'stopped.json', {'before_arm': arm,
                'reported_tokens_spent': spent, 'next_arm_bound': bound})
            refuse(f'stopped before {arm} on the conservative combined-token bound')
        result = arm_run(binary, selected, truth, data, arm, output / arm, plans[arm],
                         os.environ['THINKTHEN_API_KEY'])
        spent += result['reported_combined_tokens']
        evidence.append_json(output / 'progress.jsonl', {'arm': arm, 'reported_spent': spent,
            'attempts': result['requests_sent'], 'unknown_refusal_cost': result['unknown_refusal_cost']})
        print(json.dumps(result), flush=True)
    evidence.durable_json(output / 'complete.json', {'arms': len(ARMS), 'reported_combined_tokens': spent})


if __name__ == '__main__':
    if len(sys.argv) == 3 and sys.argv[1] == '--self-test':
        self_test(Path(sys.argv[2]))
    elif len(sys.argv) == 3 and sys.argv[1] == '--local-plan':
        print(json.dumps(local_plan(Path(sys.argv[2]), ROOT / 'target/debug/thinkthen'), indent=2))
    elif len(sys.argv) == 3:
        main(Path(sys.argv[1]), sys.argv[2])
    else:
        refuse('usage: measure.py [--self-test|--local-plan] BENCH [NAME]')
