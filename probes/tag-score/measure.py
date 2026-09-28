#!/usr/bin/env python3
"""Pinned B9 tag/score comparison. The live wrapper reserves; this job stops between arms."""

import contextlib
import csv
import hashlib
import io
import json
import os
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


def sample(bench):
    if subprocess.check_output(['git', '-C', str(bench), 'rev-parse', '--short=8', 'HEAD'], text=True).strip() != BENCH_SHA:
        refuse('the benchmark revision changed')
    if subprocess.check_output(['git', '-C', str(bench), 'status', '--porcelain'], text=True).strip():
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


def totals(binary, env):
    done = subprocess.run([str(binary), 'status', '--json'], capture_output=True, env=env, check=True)
    total = json.loads(done.stdout)['usage']['total']
    if any(type(total.get(key)) is not int or total[key] < 0
           for key in ('requests_sent', 'input_tokens', 'output_tokens')):
        refuse('status has incomplete usage')
    return total


def checked_usage(delta, rows):
    """Require reported input and output token totals and exact row shares."""
    for key in ('input_tokens', 'output_tokens'):
        shares = [row.get('meta', {}).get('usage', {}).get(key) for row in rows]
        if type(delta.get(key)) is not int or delta[key] < 0 or any(type(x) is not int or x < 0 for x in shares):
            refuse('a sent arm has missing usage')
        if sum(shares) != delta[key]:
            refuse('row shares disagree with process usage')
    if delta['input_tokens'] + delta['output_tokens'] == 0:
        refuse('a sent arm reported zero combined tokens')
    return delta['input_tokens'] + delta['output_tokens']


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


def arm(binary, cohort, data, args, verb, setting, home):
    env = {**speed.plain(), 'HOME': str(home), 'THINKTHEN_API_KEY': os.environ['THINKTHEN_API_KEY']}
    before = totals(binary, env)
    command = [str(binary), verb, *args[verb], '--details', '--facts', '--batch', setting,
               '--max-retries', '0', '--no-cache', '--model', MODEL]
    started = time.monotonic()
    done = subprocess.run(command, input=data, capture_output=True, env=env)
    elapsed = time.monotonic() - started
    if done.returncode:
        refuse(f'{verb}/{setting} exited {done.returncode}; preserve local output for diagnosis')
    rows = [json.loads(line) for line in done.stdout.splitlines()]
    if len(rows) != len(cohort) or any(row.get('input') != {'input': item['title']}
                                       for row, item in zip(rows, cohort)):
        refuse(f'{verb}/{setting} lost or reordered a result')
    facts = [json.loads(line) for line in done.stderr.splitlines() if line.startswith(b'{')]
    if len(facts) != 1 or facts[0].get('records') != 219:
        refuse(f'{verb}/{setting} has no complete final facts')
    after = totals(binary, env)
    delta = {key: after[key] - before[key] for key in ('requests_sent', 'input_tokens', 'output_tokens')}
    if not 1 <= delta['requests_sent'] <= ATTEMPT_LIMIT[f'{verb}/{setting}'] or facts[0].get('requests_sent') != delta['requests_sent']:
        refuse(f'{verb}/{setting} sent an unexpected number of requests')
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
    return {'arm': f'{verb}/{setting}', 'records': 219, **delta, 'combined_tokens': combined,
            'seconds': round(elapsed, 3), **metric,
            'answers_sha256': digest([[item['id'], value] for item, value in zip(cohort, answers)]),
            'request_ids_sha256': digest([row['meta']['requests'] for row in rows]),
            'cohort_sha256': COHORT_SHA, 'input_sha256': INPUT_SHA,
            'build': speed.git('rev-parse', 'HEAD'),
            'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(), 'bench': BENCH_SHA}


def self_test(bench):
    cohort, data, args = sample(bench)
    assert len(cohort) == 219 and len(data) > 219
    assert len(args['tag']) == 12 and len(args['score']) == 9
    rows = [{'meta': {'usage': {'input_tokens': 3, 'output_tokens': 2}}},
            {'meta': {'usage': {'input_tokens': 4, 'output_tokens': 1}}}]
    assert checked_usage({'input_tokens': 7, 'output_tokens': 3}, rows) == 10
    for bad in ({'input_tokens': 7}, {'input_tokens': 7, 'output_tokens': 4}):
        try:
            with contextlib.redirect_stderr(io.StringIO()):
                checked_usage(bad, rows)
        except SystemExit as error:
            assert error.code == 2
        else:
            raise AssertionError('missing or mismatched usage passed')
    assert spearman([1, 2, 3], [10, 20, 30]) == 1.0
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
    progress = ROOT / 'target/tag-score' / f'{name}-progress.jsonl'
    if output.exists() or progress.exists():
        refuse('the named result or progress record already exists')
    local_plan(bench)
    cohort, data, args = sample(bench)
    output.parent.mkdir(parents=True, exist_ok=True)
    progress.parent.mkdir(parents=True, exist_ok=True)
    spent = 0
    with tempfile.TemporaryDirectory(prefix='thinkthen-tag-score-') as temporary:
        for verb, setting in ARMS:
            arm_name = f'{verb}/{setting}'
            if spent + NEXT_ARM_BOUND[arm_name] > STOP_TOKENS:
                refuse(f'stopped at {spent} combined reported tokens before {arm_name}; '
                       f'its conservative next-arm bound is {NEXT_ARM_BOUND[arm_name]}')
            home = Path(temporary) / f'{verb}-{setting}'
            home.mkdir()
            result = arm(binary, cohort, data, args, verb, setting, home)
            spent += result['combined_tokens']
            with progress.open('a', encoding='utf-8') as stream:
                stream.write(json.dumps({'arm': result['arm'], 'combined_tokens': result['combined_tokens'],
                                         'spent': spent, 'requests_sent': result['requests_sent']}) + '\n')
            with output.open('a', encoding='utf-8') as stream:
                stream.write(json.dumps(result) + '\n')
            print(json.dumps(result))


if __name__ == '__main__':
    if len(sys.argv) == 3 and sys.argv[1] == '--self-test':
        self_test(Path(sys.argv[2]))
    elif len(sys.argv) == 3 and sys.argv[1] == '--local-plan':
        local_plan(Path(sys.argv[2]))
    elif len(sys.argv) == 3:
        main(Path(sys.argv[1]), sys.argv[2])
    else:
        refuse('usage: measure.py [--self-test] BENCH [NAME]')
