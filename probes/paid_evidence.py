"""Durable, count-only evidence primitives for named paid probe arms."""

import json
import os
import subprocess


def sync_dir(path):
    directory = os.open(path, os.O_RDONLY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)


def durable_bytes(path, payload):
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


def totals(binary, env, refuse, evidence=None):
    command = [str(binary), 'status', '--json']
    done = subprocess.run(command, capture_output=True, env=env)
    if evidence is not None:
        durable_json(evidence.parent / f'{evidence.name}-command.json',
                     {'command': command, 'returncode': done.returncode})
        durable_bytes(evidence.parent / f'{evidence.name}-stdout.json', done.stdout)
        durable_bytes(evidence.parent / f'{evidence.name}-stderr.txt', done.stderr)
    if done.returncode:
        refuse('status failed; retain the named raw status evidence')
    try:
        total = json.loads(done.stdout)['usage']['total']
    except (ValueError, KeyError, TypeError):
        refuse('status has incomplete usage')
    if any(type(total.get(key)) is not int or total[key] < 0
           for key in ('requests_sent', 'input_tokens', 'output_tokens')):
        refuse('status has incomplete usage')
    return total


def checked_usage(delta, rows, refuse):
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


def checked_facts(facts, delta, records, refuse):
    if not isinstance(facts, dict) or facts.get('schema') != 'thinkthen.run/1' or facts.get('records') != records:
        refuse('the final facts are missing or incomplete')
    for key in ('requests_sent', 'input_tokens', 'output_tokens'):
        if type(facts.get(key)) is not int or facts[key] != delta[key]:
            refuse(f'final facts lack matching {key}; usage is unknown')
