#!/usr/bin/env python3
"""Pinned B9 tag/score comparison. The live wrapper reserves; this job stops between arms."""

import contextlib
import csv
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

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from probes.speed import measure as speed

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
BENCH_SHA = 'ed8ffe49'
COHORT_SHA = '8077e944f9bfe1a3ac0ea8596ebb9a019dff4893379aad7423aacfc9f0ad8bf3'
IDS_SHA = '878597095e6e74dfc7487f9ec837fa858071837f1118113caf6eff9c99a4127a'
INPUT_SHA = 'ba2c4f0e36d7666864499c9596bfe67b82a1b3027f408a0650cb10702239e0c3'
ARGS_SHA = '72c4be97bea7930285bd65f02efe1adb77dd60010d993e3715f7d175d7ae19cb'
MODEL = 'jev-latest'
STOP_TOKENS = 700_000
MAX_ATTEMPTS = 219
ARMS = (('tag', '1'), ('tag', 'max'), ('score', '1'), ('score', 'max'))
ATTEMPT_LIMIT = {'tag/1': 219, 'tag/max': 9, 'score/1': 219, 'score/max': 3}
NEXT_ARM_BOUND = {'tag/1': 310_000, 'tag/max': 330_000,
                  'score/1': 80_000, 'score/max': 80_000}
LOCAL_PLAN = {'tag/1': (219, 255_964, 876), 'tag/max': (3, 277_526, 876),
              'score/1': (219, 68_281, 219), 'score/max': (1, 66_432, 219)}
LEADS = {'Lennon': 'john', 'McCartney': 'paul', 'Harrison': 'george', 'Starr': 'ringo'}


def refuse(message):
    print(f'tag-score measure: {message}', file=sys.stderr)
    raise SystemExit(2)


def digest(value):
    return hashlib.sha256(json.dumps(value, ensure_ascii=False, separators=(',', ':')).encode()).hexdigest()


def sync_dir(path):
    directory = os.open(path, os.O_RDONLY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)


def durable_bytes(path, payload):
    """Create one named artifact and sync it before another paid action starts."""
    with path.open('xb') as stream:
        stream.write(payload)
        stream.flush()
        os.fsync(stream.fileno())
    sync_dir(path.parent)


def durable_json(path, value):
    durable_bytes(path, (json.dumps(value, ensure_ascii=False, separators=(',', ':')) + '\n').encode())


def append_json(path, value):
    with path.open('a', encoding='utf-8') as stream:
        stream.write(json.dumps(value, ensure_ascii=False, separators=(',', ':')) + '\n')
        stream.flush()
        os.fsync(stream.fileno())


def sample(bench):
    if subprocess.check_output(['git', '-C', str(bench), 'rev-parse', '--short=8', 'HEAD'], text=True,
                               env=speed.plain()).strip() != BENCH_SHA:
        refuse('the benchmark revision changed')
    if subprocess.check_output(['git', '-C', str(bench), 'status', '--porcelain'], text=True,
                               env=speed.plain()).strip():
        refuse('the benchmark checkout is dirty')
    sys.path.insert(0, str(bench / 'scripts/generate'))
    from generate import settled_lead
    with (bench / 'data/songs.tsv').open(encoding='utf-8', newline='') as stream:
        rows = sorted((row for row in csv.DictReader(stream, delimiter='\t')
                       if settled_lead(row) and row['views_2024']), key=lambda row: row['title'])
    cohort = [{'id': f'b9-song-{place:03d}', 'title': row['title'],
               'tag': sorted(LEADS[name] for name in row['lead_vocals'].split('+')),
               'views': int(row['views_2024'])} for place, row in enumerate(rows, 1)]
    if (len(cohort) != 219 or len({item['title'] for item in cohort}) != 219
            or digest(cohort) != COHORT_SHA or digest([item['id'] for item in cohort]) != IDS_SHA):
        refuse('the pinned 219-item IDs or truth changed')
    data = ''.join(json.dumps({'input': item['title']}, ensure_ascii=False, separators=(',', ':')) + '\n'
                   for item in cohort).encode()
    if hashlib.sha256(data).hexdigest() != INPUT_SHA:
        refuse('the pinned input bytes changed')
    args = []
    for verb in ('tag', 'score'):
        with (bench / f'questions/suite/{verb}.jsonl').open(encoding='utf-8') as stream:
            first = json.loads(stream.readline())
        if first['function'] != verb:
            refuse('the suite verb changed')
        args.append(first['args'])
    if digest(args) != ARGS_SHA:
        refuse('the fixed suite questions, descriptions or levels changed')
    return cohort, data, dict(zip(('tag', 'score'), args))


def totals(binary, env, evidence=None):
    command = [str(binary), 'status', '--json']
    done = subprocess.run(command, capture_output=True, env=env)
    if evidence is not None:
        durable_json(evidence.parent / f'{evidence.name}-command.json',
                     {'command': command, 'returncode': done.returncode})
        durable_bytes(evidence.parent / f'{evidence.name}-stdout.json', done.stdout)
        durable_bytes(evidence.parent / f'{evidence.name}-stderr.txt', done.stderr)
    if done.returncode:
        refuse('status failed; retain the named raw status evidence')
    total = json.loads(done.stdout)['usage']['total']
    if any(type(total.get(key)) is not int or total[key] < 0
           for key in ('requests_sent', 'input_tokens', 'output_tokens')):
        refuse('status has incomplete usage')
    return total


def checked_usage(delta, rows):
    """Require reported input and output token totals and exact row shares."""
    for key in ('input_tokens', 'output_tokens'):
        shares = []
        for row in rows:
            meta = row.get('meta') if isinstance(row, dict) else None
            usage = meta.get('usage') if isinstance(meta, dict) else None
            shares.append(usage.get(key) if isinstance(usage, dict) else None)
        if type(delta.get(key)) is not int or delta[key] < 0 or any(type(x) is not int or x < 0 for x in shares):
            refuse('a sent arm has missing usage')
        if sum(shares) != delta[key]:
            refuse('row shares disagree with process usage')
    if delta['input_tokens'] + delta['output_tokens'] == 0:
        refuse('a sent arm reported zero combined tokens')
    return delta['input_tokens'] + delta['output_tokens']


def checked_facts(facts, delta):
    """A status delta alone cannot prove a command's complete reported usage."""
    if not isinstance(facts, dict) or facts.get('schema') != 'thinkthen.run/1' or facts.get('records') != 219:
        refuse('the final facts are missing or incomplete')
    for key in ('requests_sent', 'input_tokens', 'output_tokens'):
        if type(facts.get(key)) is not int or facts[key] != delta[key]:
            refuse(f'final facts lack matching {key}; usage is unknown')


def ranks(values):
    """Average ranks preserve the score proxy's ties."""
    order = sorted(range(len(values)), key=values.__getitem__)
    result = [0.0] * len(values)
    at = 0
    while at < len(order):
        end = at + 1
        while end < len(order) and values[order[end]] == values[order[at]]:
            end += 1
        for place in order[at:end]:
            result[place] = (at + end - 1) / 2
        at = end
    return result


def spearman(left, right):
    x, y = ranks(left), ranks(right)
    xbar, ybar = sum(x) / len(x), sum(y) / len(y)
    xx = sum((v - xbar) ** 2 for v in x)
    yy = sum((v - ybar) ** 2 for v in y)
    if xx == 0 or yy == 0:
        return None
    return sum((a - xbar) * (b - ybar) for a, b in zip(x, y)) / (xx * yy) ** 0.5


def arm(binary, cohort, data, args, verb, setting, folder):
    """Keep raw output and status even when an arm fails before it can be scored."""
    arm_name = f'{verb}/{setting}'
    folder.mkdir()
    sync_dir(folder.parent)
    home = folder / 'home'
    home.mkdir()
    sync_dir(folder)
    env = {**speed.plain(), 'HOME': str(home), 'THINKTHEN_API_KEY': os.environ['THINKTHEN_API_KEY']}
    before = totals(binary, env, folder / 'before-status')
    durable_json(folder / 'before.json', before)
    command = [str(binary), verb, *args[verb], '--details', '--facts', '--batch', setting,
               '--max-retries', '0', '--no-cache', '--model', MODEL]
    durable_bytes(folder / 'input.jsonl', data)
    durable_json(folder / 'started.json', {'arm': arm_name, 'command': command,
                                         'cohort_sha256': COHORT_SHA,
                                         'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest()})
    stdout_path, stderr_path = folder / 'stdout.jsonl', folder / 'stderr.txt'
    interrupted = False
    with (folder / 'input.jsonl').open('rb') as input_stream, stdout_path.open('xb') as stdout, stderr_path.open('xb') as stderr:
        sync_dir(folder)
        started = time.monotonic()
        child = subprocess.Popen(command, stdin=input_stream, stdout=stdout, stderr=stderr, env=env)
        try:
            returncode = child.wait()
        except (KeyboardInterrupt, InterruptedError):
            interrupted = True
            child.terminate()
            try:
                returncode = child.wait(timeout=10)
            except subprocess.TimeoutExpired:
                child.kill()
                returncode = child.wait()
        elapsed = time.monotonic() - started
        stdout.flush()
        stderr.flush()
        os.fsync(stdout.fileno())
        os.fsync(stderr.fileno())
    facts_raw = b'\n'.join(line for line in stderr_path.read_bytes().splitlines() if line.startswith(b'{'))
    durable_bytes(folder / 'final-facts-raw.jsonl', facts_raw + (b'\n' if facts_raw else b''))
    try:
        after = totals(binary, env, folder / 'after-status')
        status_error = None
    except (Exception, SystemExit) as error:
        after = None
        status_error = type(error).__name__
    delta = ({key: after[key] - before[key] for key in ('requests_sent', 'input_tokens', 'output_tokens')}
             if after is not None else None)
    unknown_refusal_cost = (delta is None or returncode != 0 or interrupted
                            or delta['requests_sent'] > LOCAL_PLAN[arm_name][0])
    durable_json(folder / 'completion.json', {'arm': arm_name, 'returncode': returncode,
                 'interrupted': interrupted, 'seconds': round(elapsed, 3), 'after_status': after,
                 'status_error': status_error, 'reported_delta': delta,
                 'attempts_observed': None if delta is None else delta['requests_sent'],
                 'usage_validated': False, 'usage_scope': 'reported_tokens_only',
                 'unknown_refusal_cost': unknown_refusal_cost})
    if interrupted or returncode != 0 or delta is None:
        refuse(f'{arm_name} did not complete; retain the named arm directory and do not retry')
    try:
        rows = [json.loads(line) for line in stdout_path.read_bytes().splitlines()]
        facts = [json.loads(line) for line in facts_raw.splitlines()]
    except (ValueError, TypeError):
        refuse(f'{arm_name} has malformed output; retain the named arm directory')
    durable_json(folder / 'facts.json', facts)
    if len(rows) != len(cohort) or any(not isinstance(row, dict) or row.get('input') != {'input': item['title']}
                                       for row, item in zip(rows, cohort)):
        refuse(f'{arm_name} lost or reordered a result; retain the named arm directory')
    if len(facts) != 1:
        refuse(f'{arm_name} has no complete final facts; retain the named arm directory')
    checked_facts(facts[0], delta)
    if not 1 <= delta['requests_sent'] <= ATTEMPT_LIMIT[arm_name]:
        refuse(f'{arm_name} sent an unexpected number of requests; retain the named arm directory')
    combined = checked_usage(delta, rows)
    answers = [row['value'] for row in rows]
    if verb == 'tag':
        if any(not isinstance(value, list) or any(not isinstance(label, str) for label in value)
               for value in answers):
            refuse('tag did not return label arrays')
        metric = {'right': sum(sorted(value) == item['tag'] for value, item in zip(answers, cohort))}
    else:
        if any(type(value) not in (float, int) for value in answers):
            refuse('score did not return numbers')
        metric = {'spearman_views_proxy': spearman(answers, [item['views'] for item in cohort])}
    result = {'arm': arm_name, 'records': 219, 'requests_sent': delta['requests_sent'],
            'reported_input_tokens': delta['input_tokens'],
            'reported_output_tokens': delta['output_tokens'],
            'reported_combined_tokens': combined, 'unknown_refusal_cost': unknown_refusal_cost,
            'usage_scope': 'reported_tokens_only',
            'seconds': round(elapsed, 3), **metric,
            'answers_sha256': digest([[item['id'], value] for item, value in zip(cohort, answers)]),
            'request_ids_sha256': digest([row['meta']['requests'] for row in rows]),
            'cohort_sha256': COHORT_SHA, 'input_sha256': INPUT_SHA,
            'build': speed.git('rev-parse', 'HEAD'),
            'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(), 'bench': BENCH_SHA}
    durable_json(folder / 'validated.json', result)
    return result


def self_test(bench):
    cohort, data, args = sample(bench)
    assert len(cohort) == 219 and len(data) > 219
    assert len(args['tag']) == 12 and len(args['score']) == 9
    rows = [{'meta': {'usage': {'input_tokens': 3, 'output_tokens': 2}}},
            {'meta': {'usage': {'input_tokens': 4, 'output_tokens': 1}}}]
    assert checked_usage({'input_tokens': 7, 'output_tokens': 3}, rows) == 10
    checked_facts({'schema': 'thinkthen.run/1', 'records': 219, 'requests_sent': 2, 'input_tokens': 7,
                   'output_tokens': 3}, {'requests_sent': 2, 'input_tokens': 7, 'output_tokens': 3})
    for bad in ({'input_tokens': 7}, {'input_tokens': 7, 'output_tokens': 4}):
        try:
            with contextlib.redirect_stderr(io.StringIO()):
                checked_usage(bad, rows)
        except SystemExit as error:
            assert error.code == 2
        else:
            raise AssertionError('missing or mismatched usage passed')
    assert spearman([1, 2, 3], [10, 20, 30]) == 1.0
    for bad in ({'schema': 'thinkthen.run/1', 'records': 219, 'requests_sent': 2, 'input_tokens': 7},
                {'schema': 'thinkthen.run/1', 'records': 219, 'requests_sent': 2,
                 'input_tokens': 7, 'output_tokens': 4}):
        try:
            with contextlib.redirect_stderr(io.StringIO()):
                checked_facts(bad, {'requests_sent': 2, 'input_tokens': 7, 'output_tokens': 3})
        except SystemExit as error:
            assert error.code == 2
        else:
            raise AssertionError('missing or mismatched facts passed')
    with tempfile.TemporaryDirectory(prefix='thinkthen-b9-helper-') as temporary:
        folder = Path(temporary)
        fake = folder / 'fake.py'
        fake.write_text('''#!/usr/bin/env python3
import json, os, pathlib, sys
sent = pathlib.Path(os.environ['HOME']) / 'sent'
if sys.argv[1] == 'status':
    n = int(sent.exists())
    print(json.dumps({'usage': {'total': {'requests_sent': n, 'input_tokens': 0, 'output_tokens': 0}}}))
else:
    sent.write_text('attempted')
    print('{"partial":true}')
    print('synthetic refusal', file=sys.stderr)
    raise SystemExit(4)
''')
        fake.chmod(0o700)
        former_key = os.environ.get('THINKTHEN_API_KEY')
        os.environ['THINKTHEN_API_KEY'] = 'sk-synthetic-only'
        try:
            with contextlib.redirect_stderr(io.StringIO()):
                try:
                    arm(fake, cohort, data, args, 'tag', '1', folder / 'failed-arm')
                except SystemExit as error:
                    assert error.code == 2
                else:
                    raise AssertionError('a failed arm passed')
        finally:
            if former_key is None:
                del os.environ['THINKTHEN_API_KEY']
            else:
                os.environ['THINKTHEN_API_KEY'] = former_key
        saved = folder / 'failed-arm'
        assert (saved / 'started.json').exists() and (saved / 'stdout.jsonl').read_text().strip() == '{"partial":true}'
        assert 'synthetic refusal' in (saved / 'stderr.txt').read_text()
        assert json.loads((saved / 'completion.json').read_text())['attempts_observed'] == 1
        assert (saved / 'before-status-stdout.json').exists()
        assert (saved / 'after-status-stdout.json').exists()
        assert (saved / 'final-facts-raw.jsonl').exists()
        assert (saved / 'home/sent').exists()
        try:
            saved.mkdir()
        except FileExistsError:
            pass
        else:
            raise AssertionError('a started arm allowed a blind rerun')
    print(json.dumps({'records': len(cohort), 'ids_sha256': IDS_SHA, 'truth_sha256': COHORT_SHA,
                      'input_sha256': INPUT_SHA, 'suite_args_sha256': ARGS_SHA}))


def local_plan(bench):
    """Exercise all four groupings against a loopback reply and count real encoded bytes."""
    cohort, data, args = sample(bench)
    binary = ROOT / 'target/debug/thinkthen'
    if not binary.exists():
        refuse('build the compiled command before the local plan')
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
                    answers[name] = {'type': 'noul', 'noul': 0.1}
                else:
                    count = len(question['criteria'])
                    answers[name] = {'type': 'score', 'score': 0, 'confidence': 1,
                                     'legend': {str(i): str(i) for i in range(count)},
                                     'probabilities': {str(i): int(i == 0) for i in range(count)}}
            payload = json.dumps({'model': MODEL, 'answers': answers}, separators=(',', ':')).encode()
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Content-Length', str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)

        def log_message(self, *unused):
            pass

    server = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        for verb, setting in ARMS:
            before = len(bodies)
            command = [str(binary), verb, *args[verb], '--batch', setting, '--jobs', '1', '--facts', '--max-retries', '0',
                       '--no-cache', '--model', MODEL, '--url', f'http://127.0.0.1:{server.server_port}']
            env = {**speed.plain(), 'THINKTHEN_API_KEY': 'sk-loopback-only'}
            done = subprocess.run(command, input=data, capture_output=True, env=env, timeout=180)
            if done.returncode or len(done.stdout.splitlines()) != len(cohort):
                refuse(f'local {verb}/{setting} plan failed with exit {done.returncode}: '
                       f'{done.stderr.decode(errors="replace")[:400]}')
            captured = bodies[before:]
            if not 1 <= len(captured) <= MAX_ATTEMPTS:
                refuse(f'local {verb}/{setting} grouped unexpectedly')
            arm_name = f'{verb}/{setting}'
            shape = (len(captured), sum(map(len, captured)),
                     sum(len(json.loads(body)['questions']) for body in captured))
            if shape != LOCAL_PLAN[arm_name]:
                refuse(f'local {arm_name} request grouping or encoded bytes changed: {shape}')
            print(json.dumps({'arm': arm_name, 'requests': shape[0],
                              'body_bytes': shape[1], 'wire_questions': shape[2],
                              'max_body_bytes': max(map(len, captured))}))
    finally:
        server.shutdown()
        server.server_close()
        thread.join()


def main(bench, name):
    if not name or any(char not in 'abcdefghijklmnopqrstuvwxyz0123456789-' for char in name):
        refuse('NAME needs lowercase letters, digits, or hyphens')
    binary = ROOT / 'target/debug/thinkthen'
    if os.environ.get('THINKTHEN_BASE_URL') or not os.environ.get('THINKTHEN_API_KEY'):
        refuse('the built-in address and a key are required')
    if speed.git('status', '--porcelain', '--untracked-files=no'):
        refuse('the checkout has uncommitted changes')
    if not binary.exists():
        refuse('build the named binary before the charged job')
    source_head = int(speed.git('log', '-1', '--format=%ct', '--', *speed.SOURCES))
    if binary.stat().st_mtime < source_head:
        refuse('the compiled binary predates its source commit')
    output = HERE / 'runs' / f'{name}.jsonl'
    run_state = ROOT / 'target/tag-score' / name
    legacy_progress = ROOT / 'target/tag-score' / f'{name}-progress.jsonl'
    if output.exists() or run_state.exists() or legacy_progress.exists():
        refuse('the named result or started run already exists; inspect its saved state before any repeat')
    local_plan(bench)
    cohort, data, args = sample(bench)
    output.parent.mkdir(parents=True, exist_ok=True)
    run_state.parent.mkdir(parents=True, exist_ok=True)
    try:
        run_state.mkdir()
    except FileExistsError:
        refuse('the named run started elsewhere; inspect its saved state before any repeat')
    sync_dir(run_state.parent)
    durable_json(run_state / 'started.json', {'name': name, 'arms': [f'{verb}/{setting}' for verb, setting in ARMS],
                 'cohort_sha256': COHORT_SHA, 'build': speed.git('rev-parse', 'HEAD'),
                 'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
                 'stop_reported_tokens': STOP_TOKENS})

    def interrupt(signum, unused):
        raise InterruptedError(f'signal {signum}')

    signal.signal(signal.SIGINT, interrupt)
    signal.signal(signal.SIGTERM, interrupt)
    spent = 0
    for verb, setting in ARMS:
        arm_name = f'{verb}/{setting}'
        if spent + NEXT_ARM_BOUND[arm_name] > STOP_TOKENS:
            durable_json(run_state / 'stopped.json', {'before_arm': arm_name,
                         'reported_tokens_spent': spent, 'next_arm_estimate': NEXT_ARM_BOUND[arm_name]})
            refuse(f'stopped at {spent} reported combined tokens before {arm_name}; '
                   f'its conservative next-arm estimate is {NEXT_ARM_BOUND[arm_name]}')
        result = arm(binary, cohort, data, args, verb, setting, run_state / f'{verb}-{setting}')
        spent += result['reported_combined_tokens']
        append_json(run_state / 'progress.jsonl', {'arm': arm_name,
                    'reported_combined_tokens': result['reported_combined_tokens'],
                    'reported_spent': spent, 'requests_sent': result['requests_sent'],
                    'unknown_refusal_cost': result['unknown_refusal_cost']})
        append_json(output, result)
        print(json.dumps(result))
    durable_json(run_state / 'complete.json', {'arms': len(ARMS), 'reported_combined_tokens': spent})


if __name__ == '__main__':
    if len(sys.argv) == 3 and sys.argv[1] == '--self-test':
        self_test(Path(sys.argv[2]))
    elif len(sys.argv) == 3 and sys.argv[1] == '--local-plan':
        local_plan(Path(sys.argv[2]))
    elif len(sys.argv) == 3:
        main(Path(sys.argv[1]), sys.argv[2])
    else:
        refuse('usage: measure.py [--self-test] BENCH [NAME]')
