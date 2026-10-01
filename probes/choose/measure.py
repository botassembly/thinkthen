#!/usr/bin/env python3
"""Measure the exact experiment-262 choose questions in grouped command streams.

No key, question, option, answer, or recording bytes are printed or saved in the
count record. The ignored target folder retains progress and the context recording.
The live wrapper precharges a reservation; this job stops between commands.
Set CHOOSE_SAMPLE_DIR to the experiment-262 folder that holds labels/ids.txt.
"""

import collections
import contextlib
import hashlib
import json
import os
import io
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from probes.speed import measure as speed

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
SAMPLE_DIR_VARIABLE = 'CHOOSE_SAMPLE_DIR'
IDS_SHA256 = '30ece78217239f69ca428dc029d2257a5129a9a33ff765c6b3f16e84e24e801e'
SAMPLE_SHA256 = 'c8095321f3f464712f7ba1dc3506714c16d7afc5c31617f5bd111d653aebdc02'
CATALOG_SHA256 = '982e6da743c57ea8b53eb8dbe1c1721b0059ea8e06ce416f2a944caca777151f'
STOP_TOKENS = 180_000
MODEL = 'jev-latest'


def refuse(message):
    print(f'choose measure: {message}', file=sys.stderr)
    raise SystemExit(2)


def usage(total, rows):
    """Count both reported token fields, with exact per-row share agreement."""
    if not isinstance(total, dict) or not rows:
        refuse('missing process usage or result rows')
    result = {}
    for key in ('input_tokens', 'output_tokens'):
        value = total.get(key)
        shares = []
        for row in rows:
            meta = row.get('meta') if isinstance(row, dict) else None
            reported = meta.get('usage') if isinstance(meta, dict) else None
            shares.append(reported.get(key) if isinstance(reported, dict) else None)
        if type(value) is not int or value < 0 or any(type(x) is not int or x < 0 for x in shares):
            refuse('a sent request has missing token usage')
        if sum(shares) != value:
            refuse('row shares disagree with process usage')
        result[key] = value
    if result['input_tokens'] + result['output_tokens'] == 0:
        refuse('a sent request reported zero combined tokens')
    return result


def sample_dir():
    folder = os.environ.get(SAMPLE_DIR_VARIABLE)
    if not folder:
        refuse(f'set {SAMPLE_DIR_VARIABLE} to the experiment-262 folder that holds labels/ids.txt')
    return Path(folder)


def sample(bench):
    ids_path = sample_dir() / 'labels/ids.txt'
    if hashlib.sha256(ids_path.read_bytes()).hexdigest() != IDS_SHA256:
        refuse('experiment 262 IDs changed')
    ids = ids_path.read_text().splitlines()
    if len(ids) != 200 or len(set(ids)) != 200:
        refuse('the fixed sample needs 200 unique IDs')
    questions = {}
    for path in sorted((bench / 'questions').glob('*.jsonl')):
        for line in path.read_text(encoding='utf-8').splitlines():
            item = json.loads(line)
            questions[item['id']] = item
    if any(item not in questions for item in ids):
        refuse('the bench lacks a sampled question')
    selected = [{key: questions[ident][key] for key in ('id', 'question', 'options', 'input', 'truth')}
                for ident in ids]
    payload = json.dumps(selected, ensure_ascii=False, separators=(',', ':')).encode()
    if hashlib.sha256(payload).hexdigest() != SAMPLE_SHA256:
        refuse('the sampled question, option, input, or truth bytes changed')
    groups = collections.OrderedDict()
    for ident in ids:
        item = questions[ident]
        text, options, evidence = item['question'], item['options'], item['input']
        if (item['function'] != 'choose' or not isinstance(text, str) or not isinstance(options, dict)
                or not isinstance(evidence, str) or not evidence.strip()
                or '\n' in evidence or '\r' in evidence or len(options) < 2):
            refuse('a sampled item cannot use an exact choose record stream')
        if any(not isinstance(label, str) or not isinstance(description, str)
               for label, description in options.items()):
            refuse('a sampled option cannot keep its exact description')
        key = (text, tuple(options.items()))
        groups.setdefault(key, []).append(item)
    if len(groups) != 153 or sum(len(g) == 1 for g in groups.values()) != 147:
        refuse('experiment 262 grouping changed')
    return list(groups.items())


def status(binary, env):
    done = subprocess.run([str(binary), 'status', '--json'], capture_output=True, env=env, check=True)
    total = json.loads(done.stdout)['usage']['total']
    if any(type(total.get(k)) is not int or total[k] < 0
           for k in ('requests_sent', 'input_tokens', 'output_tokens')):
        refuse('status has incomplete process usage')
    return total


def arm(binary, groups, setting, context, record, home, progress, spent):
    env = {**speed.plain(), 'HOME': str(home), 'THINKTHEN_API_KEY': os.environ['THINKTHEN_API_KEY']}
    before = status(binary, env)
    right = requests = input_tokens = output_tokens = seen = 0
    max_attempts = 3 * sum(len(items) for _, items in groups)
    answers_sha256 = hashlib.sha256()
    requests_sha256 = hashlib.sha256()
    context_sha256 = hashlib.sha256(context.read_bytes()).hexdigest() if context is not None else None
    for number, ((question, options), items) in enumerate(groups, 1):
        if spent >= STOP_TOKENS:
            refuse(f'stopped at {spent} combined reported tokens before the next group')
        args = [str(binary), 'choose', question]
        for label, description in options:
            args += ['--option', f'{label}={description}']
        args += ['--lines', '--details', '--batch', setting, '--max-retries', '0',
                 '--no-cache', '--model', MODEL]
        if context is not None:
            args += ['--context', str(context)]
        if record is not None:
            args += ['--record', str(record)]
        data = ''.join(item['input'] + '\n' for item in items).encode()
        done = subprocess.run(args, input=data, capture_output=True, env=env)
        if done.returncode:
            refuse(f'{setting} group {number} exited {done.returncode}')
        rows = [json.loads(line) for line in done.stdout.splitlines()]
        if len(rows) != len(items) or any(row.get('input') != item['input']
                                           for row, item in zip(rows, items)):
            refuse(f'{setting} group {number} lost or reordered results')
        if context_sha256 is not None and any(row.get('meta', {}).get('context_sha256') != context_sha256
                                              for row in rows):
            refuse(f'{setting} group {number} reported a wrong context identity')
        answers_sha256.update(json.dumps([[item['id'], row['value']] for row, item in zip(rows, items)],
                                         ensure_ascii=False, separators=(',', ':')).encode())
        requests_sha256.update(json.dumps([row['meta']['requests'] for row in rows],
                                          separators=(',', ':')).encode())
        after = status(binary, env)
        delta = {key: after[key] - before[key]
                 for key in ('requests_sent', 'input_tokens', 'output_tokens')}
        if delta['requests_sent'] < 1 or delta['requests_sent'] > 3 * len(items):
            refuse(f'{setting} group {number} sent an unexpected number of attempts')
        counted = usage(delta, rows)
        if counted['input_tokens'] != delta['input_tokens'] or counted['output_tokens'] != delta['output_tokens']:
            refuse('process and row token counts differ')
        right += sum(row['value'] == item['truth'] for row, item in zip(rows, items))
        seen += len(items)
        requests += delta['requests_sent']
        input_tokens += delta['input_tokens']
        output_tokens += delta['output_tokens']
        spent += delta['input_tokens'] + delta['output_tokens']
        with progress.open('a', encoding='utf-8') as stream:
            stream.write(json.dumps({'arm': setting if context is None else 'context',
                                     'group': number, 'records': seen, 'requests': requests,
                                     'input_tokens': input_tokens, 'output_tokens': output_tokens}) + '\n')
        if requests > max_attempts:
            refuse(f'{setting} exceeded {max_attempts} attempts')
        before = after
    return {'records': seen, 'groups': len(groups), 'right': right,
            'bar_135': right >= 135 if seen == 200 else None,
            'requests_sent': requests, 'input_tokens': input_tokens,
            'output_tokens': output_tokens, 'combined_tokens': input_tokens + output_tokens,
            'sample_sha256': SAMPLE_SHA256, 'answers_sha256': answers_sha256.hexdigest(),
            'requests_sha256': requests_sha256.hexdigest(), 'context_sha256': context_sha256}, spent


def main(bench, name):
    if not name or any(ch not in 'abcdefghijklmnopqrstuvwxyz0123456789-' for ch in name):
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
    progress = ROOT / 'target' / 'choose' / f'{name}-progress.jsonl'
    if output.exists() or progress.exists():
        refuse('the named count or progress record already exists')
    groups = sample(Path(bench))
    progress.parent.mkdir(parents=True, exist_ok=True)
    output.parent.mkdir(parents=True, exist_ok=True)
    spent = 0
    with tempfile.TemporaryDirectory(prefix='thinkthen-choose-') as scratch:
        scratch = Path(scratch)
        for setting in ('max', '10'):
            home = scratch / setting
            home.mkdir()
            result, spent = arm(binary, groups, setting, None, None, home, progress, spent)
            result.update(arm=setting, build=speed.git('rev-parse', 'HEAD'))
            with output.open('a', encoding='utf-8') as stream:
                stream.write(json.dumps(result) + '\n')
            print(json.dumps(result))
        largest = max(groups, key=lambda pair: len(pair[1]))
        catalog = scratch / 'catalog.txt'
        catalog_bytes = speed.catalog_text(Path(bench)).encode()
        if hashlib.sha256(catalog_bytes).hexdigest() != CATALOG_SHA256:
            refuse('the fixed context catalog changed')
        catalog.write_bytes(catalog_bytes)
        home = scratch / 'context'
        home.mkdir()
        recording = ROOT / 'target' / 'choose' / f'{name}-context-record'
        if recording.exists():
            refuse('the named context recording already exists')
        result, spent = arm(binary, [largest], 'max', catalog, recording, home, progress, spent)
        result.update(arm='context', build=speed.git('rev-parse', 'HEAD'))
        with output.open('a', encoding='utf-8') as stream:
            stream.write(json.dumps(result) + '\n')
        print(json.dumps(result))


def self_test(bench):
    groups = sample(bench)
    assert sum(len(items) for _, items in groups) == 200
    assert sum((len(items) + 9) // 10 for _, items in groups) == 156
    assert hashlib.sha256(speed.catalog_text(bench).encode()).hexdigest() == CATALOG_SHA256
    rows = [{'meta': {'usage': {'input_tokens': 3, 'output_tokens': 2}}},
            {'meta': {'usage': {'input_tokens': 4, 'output_tokens': 1}}}]
    assert usage({'input_tokens': 7, 'output_tokens': 3}, rows) == {'input_tokens': 7, 'output_tokens': 3}
    for bad in ({'input_tokens': 7}, {'input_tokens': 7, 'output_tokens': 4}):
        try:
            with contextlib.redirect_stderr(io.StringIO()):
                usage(bad, rows)
        except SystemExit as error:
            assert error.code == 2
        else:
            raise AssertionError('missing or mismatched usage passed')


if __name__ == '__main__':
    if len(sys.argv) == 3 and sys.argv[1] == '--self-test':
        self_test(Path(sys.argv[2]))
    elif len(sys.argv) == 3:
        main(*sys.argv[1:])
    else:
        refuse('usage: measure.py BENCH NAME')
