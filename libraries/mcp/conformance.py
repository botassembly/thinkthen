"""Execute every canonical required cell through installed named MCP tools."""
import base64
import json
from pathlib import Path
import re
import sqlite3
import sys
import tempfile
import threading

from client import Client, CancelledError, ProtocolError, ToolError

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'conformance'))
from c_parity import Backend, ERRORS, assertions, compact, document, prepare
from c_images import assert_images
import parity


def fixture(row, cases, named):
    if row['id'].startswith('mcp-question-'):
        case = named[row['input']['case_ref']]
        return {'verb': row['verb'], 'arguments': case['input']['arguments'],
                'setup': case['input'].get('setup'), 'question': {},
                'items': [], 'expect': case['expect'], 'arm': 'arm/full/capture/v1'}
    value = document(row, cases, named)
    if row['kind'] == 'settings':
        for step in value['steps']:
            if step.get('incremental'):
                # Reuse the canonical owned JSONL recipe and the native lazy reader.
                step.update(owned_jsonl=True, paths=['records.jsonl'], source_unit=5, jsonl_root=True)
    if row['kind'] == 'named-input' and named[row['input']['case_ref']]['input'].get('source_case'):
        value['author_expect'] = named[row['input']['case_ref']]['expect']
    if row['kind'] == 'refusal':
        value['image_only'] = True
    return value


def arguments(step, home, settings):
    if 'arguments' in step:
        return step['arguments']
    question = step['question']
    args = {'question': question, 'options': {'attempts': True}}
    if step.get('raw'):
        args['question'] = '__RAW_QUESTION__'
    if step['verb'] == 'find' and question.get('none'):
        args['question'] = {k: v for k, v in question.items() if k != 'none'}
        args['options']['none'] = True
    if step.get('question_form') == 'file':
        args = {'question_file': str(home / 'fixture-question.json'), 'options': args['options']}
    if step.get('loader'):
        loader = step['loader']
        reference = step['reference']
        key = 'question_name' if loader in ('named', 'load_named') else 'question_file'
        if loader in ('reference', 'load_reference'):
            key = 'question_reference'
        args = {key: reference if key in ('question_name', 'question_reference') else str(home / reference),
                'options': args['options']}
    if step.get('shared_context') is not None:
        args['options']['context'] = step['shared_context']
    for key in ('batch', 'threshold'):
        if key in settings:
            args['options'][key] = settings[key]
    if step.get('jsonl_root'):
        args['options']['field'] = ''
    operation = step.get('operation') or {}
    if operation.get('injection') == 'expired_deadline':
        args['options']['deadline_ms'] = 0
    if operation.get('injection') == 'cancel_token':
        args['options']['cancelled'] = True
    paths = step.get('paths')
    if operation.get('injection') == 'recording_read_failure':
        paths = [str(home / 'missing-input')]
    if paths:
        unit = step.get('source_unit', 3)
        args['source'] = {'paths': [str(home / p) if step.get('owned_jsonl') else str(ROOT / p) for p in paths],
                          'unit': {1: 'line', 2: 'window', 3: 'file', 4: 'file', 5: 'line'}[unit],
                          'media': 'image' if unit == 4 or step.get('image_reader') else 'text'}
        if unit == 2:
            args['source']['window'] = step['window']
    elif step.get('image_paths'):
        descriptors = []
        for at, item in enumerate(step['items']):
            entry = {'images': [{'path': str(ROOT / p), 'media': step.get('media', 'image/png')} for p in step['image_paths']]}
            if step.get('caption_files'):
                entry['source'] = {'paths': [str(home / ('caption-%d.txt' % at))], 'unit': 'file', 'media': 'text'}
            elif not step.get('image_only'):
                entry['text' if isinstance(item, str) else 'json'] = item
            if step.get('candidate_orders'):
                entry['options'] = step['candidate_orders'][at]
            descriptors.append(entry)
        args['inputs'] = descriptors
    elif step.get('text') and len(step['items']) == 1 and isinstance(step['items'][0], str):
        args['evidence'] = step['items'][0]
    else:
        args['records'] = step['items']
    if step.get('context_present'):
        entries = args.get('inputs')
        if entries is None:
            args.pop('evidence', None)
            args.pop('records', None)
            entries = [{'text' if step.get('text') else 'json': item} for item in step['items']]
            args['inputs'] = entries
        for entry in entries:
            entry['context'] = step['context']
    return args


def project(packet, verb):
    """Read only actual complete serializer fields; never substitute fixture data."""
    if 'error' in packet:
        error = packet['error']
        out = {'code': ERRORS[error['kind']], 'message': error['message']}
        if packet.get('facts'):
            out.update({k: packet['facts'][k] for k in ('requests_sent', 'records')})
        if error.get('stopped', {}).get('at') is not None:
            out['stopped_at'] = error['stopped']['at']
        if packet.get('completed'):
            out['completed'] = project({'value': packet['completed'], 'facts': packet['facts']}, verb)['rows']
        return out
    facts = packet['facts']
    results = packet['value'] if isinstance(packet['value'], list) else [packet['value']]
    out = {'code': 0, 'schema': results[0]['schema'] if results else 'thinkthen.result/2',
           'call_id': facts['call_id'], 'requests_sent': facts['requests_sent'],
           'cache_answers': facts['cache_answers'], 'records': facts['records'], 'rows': []}
    for at, result in enumerate(results):
        assert result['schema'] == 'thinkthen.result/2', result
        meta = result['meta']
        answer = result.get('answer', {})
        row = {'value': result['value'], 'index': result.get('index'),
               'answer_id': result['answer_id'], 'input': result.get('input'),
               'origin': {'live': 1, 'cache': 2, 'replay': 3, None: None}[meta['origin']],
               'answered_by': meta.get('answered_by'), 'observations': len(meta['observations']),
               'sources': len(meta['question_sources']),
               'observation_ids': [o.get('observation_id', o.get('failure_id')) for o in meta['observations']],
               'detail_inputs': [dict(input=candidate['input'], **candidate.get('source', {})) for candidate in result.get('candidates', []) if candidate['index'] is not None]}
        row['question'] = result.get('question', {})
        row['answer'] = answer
        row['question_digest'] = meta.get('question_sha256', meta.get('questions_sha256'))
        assert re.fullmatch('[0-9a-f]{64}', row['answer_id']), 'invalid native answer ID'
        assert meta['cached'] is (meta['origin'] in ('cache', 'replay')), 'native cache fact differs from origin'
        assert all(re.fullmatch('[0-9a-f]{64}', identity) for identity in meta['requests']), 'invalid request identity'
        assert len(meta['requests']) == row['observations'] == row['sources'], meta
        if verb in ('rank', 'filter'):
            assert 'index' in result, 'native complete rows do not expose original ordinals'
        if verb == 'find':
            assert 'index' in result, 'native complete find does not expose its selected ordinal'
        if verb == 'decide' and isinstance(row['value'], bool):
            row['value'] = result['question'].get('true' if row['value'] else 'false', row['value'])
        for key in ('probability', 'probabilities'):
            if key in answer:
                row[key] = answer[key]
        row.update({k: result['question'][k] for k in ('name', 'wording_version') if k in result.get('question', {})})
        row.update(result.get('source', {}))
        for key in ('file', 'first_line', 'last_line'):
            if key in result:
                row[key] = result[key]
        row['member_authors'] = [{k: a['question'][k] for k in ('name', 'wording_version') if k in a['question']}
                                 for a in result.get('answers', {}).values()]
        for member in result.get('answers', {}).values():
            identity = member.get('answer_id', member.get('failure_id'))
            assert re.fullmatch('[0-9a-f]{64}', identity), 'annotation member identity missing'
        row['answers'] = result.get('answers', {})
        if verb == 'relate':
            for edge in result['value']:
                for endpoint in (edge['source'], edge['target']):
                    if 'file' in endpoint and all(i['file'] != endpoint['file'] for i in row['detail_inputs']):
                        row['detail_inputs'].append({'input': endpoint['record'], **{key: endpoint[key] for key in ('file', 'first_line', 'last_line')}})
        if isinstance(result.get('input'), dict) and 'images' in result['input']:
            images = result['input']['images']
            row['input'] = result['input'].get('text')
            row.update(result['input'].get('location', {}))
            row['images'] = [base64.b64decode(i['base64']).hex() for i in images]
            row['image_properties'] = [[1 if i['media'] == 'image/jpeg' else 2, i['width'], i['height']]
                                       for i in images]
        if result.get('images'):
            images = result['images']
            row['images'] = [base64.b64decode(i['base64']).hex() for i in images]
            row['image_properties'] = [[1 if i['media'] == 'image/jpeg' else 2, i['width'], i['height']] for i in images]
        out['rows'].append(row)
    return out


def known_fields(step, got, bodies):
    """Compare native metadata and selected wire evidence with independent inputs."""
    expected = step.get('author_expect', step['expect'])
    if got['code']:
        return
    for row in got['rows']:
        if 'resolved_wording' in expected:
            assert row['question']['text'] == expected['resolved_wording'], row['question']
        metadata = expected.get('resolved_metadata', {})
        questions = [a['question'] for a in row['answers'].values()] if step['verb'] == 'annotate' else [row['question']]
        for question in questions:
            for key, value in metadata.items():
                assert question[key] == value, question
            if 'no_inferred_metadata' in expected:
                assert all(key in metadata or key not in question for key in ('name', 'wording_version')), question
        for key in ('ordered_properties', 'required_properties'):
            if key in expected:
                declaration = row['question']['item_schema']
                actual = list(declaration['properties']) if key == 'ordered_properties' else declaration['required']
                assert actual == expected[key], declaration
    for body in map(json.loads, bodies):
        if 'selected_item' in expected:
            selected = expected['selected_item']
            if step.get('image_paths'):
                assert body['state'] == selected, body
            else:
                wire = compact(selected)
                if step['verb'] == 'annotate' and step.get('text') and isinstance(selected, (dict, list)):
                    # Native document annotation validates JSON but keeps literal text evidence.
                    wire = compact(wire)
                    assert all(row['input'] == compact(selected) for row in got['rows'])
                for question in body['questions'].values():
                    assert question['instructions'].startswith('The text is ' + wire + '. '), question
        if 'per_item_context' in expected:
            assert body['state'] == expected['per_item_context'], body
            assert 'context' not in body, body
        elif step.get('shared_context') is not None:
            assert body['state'] == step['shared_context'], body
    if step.get('paths') and step.get('source_unit') == 3 and step.get('arm') == 'arm/full/capture/v1':
        for row in got['rows']:
            if step['verb'] not in ('find', 'relate'):
                assert row['input'] == Path(row['file']).read_text(), row
                assert row['first_line'] == 1 and row['last_line'] == len(row['input'].splitlines()), row
            if step['verb'] == 'annotate':
                for member in row['answers'].values():
                    answer = member['answer']
                    if answer['kind'] == 'yes_no':
                        assert answer['probability'] == .9 and member['value'] is True, member
                    elif answer['kind'] == 'tag':
                        assert all(p == .9 for p in answer['probabilities'].values()), member
                        assert member['value'] == list(member['question']['labels']), member
                    else:
                        choices = member['question'].get('options', member['question'].get('levels'))
                        assert list(answer['probabilities']) == choices, member
                        assert answer['probabilities'][choices[0]] == .9 and answer['confidence'] == .9, member
                        assert all(abs(p - .1 / (len(choices) - 1)) < 1e-10
                                   for p in list(answer['probabilities'].values())[1:]), member
            elif step['verb'] == 'recognize':
                assert row['answer']['pieces'] and row['answer']['names'], row
                for piece in row['answer']['pieces']:
                    assert 0 <= piece['start'] < piece['end'] <= len(row['input']), piece
                    assert sorted(piece['tags'].values()) == [.025, .025, .025, .025, .9], piece
                for entity in row['value']['entities']:
                    assert entity['text'] == row['input'][entity['start']:entity['end']], entity
                    assert entity['kind'] == 'person', entity
    elif step['verb'] == 'annotate' and 'success' in step['expect']:
        wanted = step['expect']['success']['answers']
        for actual in got['rows']:
            for member in wanted:
                if member['name'] in actual['answers'] and (len(got['rows']) == 1 or member['exchange'] == actual['index']):
                    for key in ('answer', 'failure'):
                        if key in member['details']:
                            assert actual['answers'][member['name']][key] == member['details'][key], actual['answers']


class RawClient(Client):
    """Preserve duplicate native grammar members in negative transport inputs."""
    raw_question = None

    def _send(self, message):
        if self.raw_question is None or message.get('method') != 'tools/call':
            return super()._send(message)
        encoded = compact(message).replace('"__RAW_QUESTION__"', self.raw_question).encode() + b'\n'
        assert len(encoded) <= 16 * 1024 * 1024, 'raw request exceeds MCP bound'
        with self._write_lock:
            self.writer.write(encoded)
            self.writer.flush()


def invoke(binary, backend, env, home, step, settings):
    command = [binary, 'mcp', '--url', settings['base_url']]
    for key, value in settings.items():
        if key in ('base_url', 'batch', 'threshold', 'proxy') or value is None:
            continue
        option = '--' + key.replace('_', '-')
        if key == 'cache' and value is False:
            command.append('--no-cache')
        elif isinstance(value, bool):
            if value:
                command.append(option)
        else:
            command.extend([option, str(value)])
    with RawClient.launch(command, env=env, cwd=home) as client:
        client.raw_question = step.get('raw')
        tools = client.tools()
        assert [t['name'] for t in tools] == list(parity.inventory()['functions'])
        assert all(isinstance(t['inputSchema'], dict) and isinstance(t['outputSchema'], dict) for t in tools)
        args = arguments(step, home, settings)
        if 'proxy' in settings:
            args['options']['proxy'] = settings['proxy']
        if step.get('held_cancel'):
            result = []
            def call():
                try:
                    result.append(getattr(client, step['verb'])(**args))
                except (ToolError, CancelledError) as e:
                    result.append(e)
            worker = threading.Thread(target=call)
            worker.start()
            assert backend.read('wait 1') == 'wait 1'
            client.cancel(client.pending_id)
            backend.process.stdin.write('release\n')
            backend.process.stdin.flush()
            worker.join(timeout=10)
            assert not worker.is_alive() and isinstance(result[0], CancelledError)
            client.ping()  # Native work retired; the same installed session remains usable.
            return {'code': ERRORS['cancelled'], 'message': 'MCP request cancelled'}
        try:
            packet = getattr(client, step['verb'])(**args)
        except ToolError as failure:
            packet = failure.result
        assert 'sk-conformance-loopback' not in compact(packet), 'credential disclosed'
        return project(packet, step['verb'])


def run(port, binary):
    assert port.isdecimal() and 0 < int(port) <= 65535, 'loopback port required'
    # Each cell owns a backend so captured bodies and zero-send counts are isolated.
    rows = list(parity.required_cases(parity.inventory(), 'mcp').values())
    cases = {r['id']: r for r in json.loads((ROOT / 'conformance/cases.json').read_text())['cases']}
    named = {r['id']: r for r in json.loads((ROOT / 'conformance/named-inputs.json').read_text())['cases']}
    failures = []
    for row in rows:
        try:
            value = fixture(row, cases, named)
            with tempfile.TemporaryDirectory(prefix='thinkthen-mcp-parity-') as tmp:
                home = Path(tmp) / 'home'
                home.mkdir()
                tmp = str(home)
                env = {'PATH': '/usr/bin:/bin', 'LANG': 'C.UTF-8', 'LC_ALL': 'C.UTF-8', 'HOME': tmp,
                       'XDG_CONFIG_HOME': tmp + '/config', 'XDG_CACHE_HOME': tmp + '/cache',
                       'XDG_STATE_HOME': tmp + '/state', 'THINKTHEN_API_KEY': 'sk-conformance-loopback',
                       'LIQUIDAI_API_KEY': 'sk-conformance-loopback', 'PERPLEXITY_API_KEY': 'sk-conformance-loopback',
                       'OPENROUTER_API_KEY': 'sk-conformance-loopback'}
                backend = Backend(ROOT / 'target/debug/conformance-backend', env)
                try:
                    prepare(home, value)
                    identities = []
                    for step in value.get('steps', [value]):
                        if step.get('owned_jsonl'):
                            prepare(home, step)
                        if step.get('copy_store'):
                            (home / 'refreshed').mkdir()
                            with sqlite3.connect(home / 'saved/thinkthen.sqlite') as a, sqlite3.connect(home / 'refreshed/thinkthen.sqlite') as b:
                                a.backup(b)
                        if step.get('damage_store'):
                            with sqlite3.connect(home / 'saved/thinkthen.sqlite') as db:
                                db.execute("UPDATE answers SET answer='damaged fixture answer'")
                        if value.get('image_variants'):
                            backend.close()
                            backend = Backend(ROOT / 'target/debug/conformance-backend', env)
                            prepare(home, step)
                        settings = {'cache': False, 'model': 'jev-latest' if 'steps' in value else 'jev-1.13.0',
                                    'batch': 1, 'max_retries': 0, **step.get('settings', {})}
                        settings['base_url'] = f'http://127.0.0.1:{backend.port}/{step["arm"] if value.get("image_variants") else value["arm"]}'
                        settings = {k: str(home / {'$FOLDER': 'saved', '$REFRESH': 'refreshed', '$PROFILE': 'profile.json'}[v])
                                    if isinstance(v, str) and v.startswith('$') else v for k, v in settings.items()}
                        if row['kind'] in ('images', 'image-location'):
                            settings['record'] = str(home / 'recorded')
                        before = int(backend.read('count'))
                        got = invoke(binary, backend, env, home, step, settings)
                        count = int(backend.read('count'))
                        assert got.get('requests_sent', count - before) == count - before, (got, count - before)
                        assertions(row, step, got, count - before if step.get('count_delta') else count)
                        if row['kind'] in ('typed-result', 'result2'):
                            assertions({**row, 'kind': 'behavior'}, {**step, 'metadata_only': False}, got, count)
                        if value.get('identity_steps'):
                            identities.append(got)
                        if step.get('stored_answers') == 0:
                            path = home / 'saved/thinkthen.sqlite'
                            if path.exists():
                                with sqlite3.connect(path) as db:
                                    assert db.execute('SELECT count(*) FROM answers').fetchone()[0] == 0
                        bodies = json.loads(backend.read('capture'))['bodies']
                        known_fields(step, got, bodies)
                        if step.get('author_expect'):
                            unadorned = fixture({**row, 'kind': 'behavior', 'input': {
                                'case_ref': named[row['input']['case_ref']]['input']['source_case']}}, cases, named)
                            baseline = invoke(binary, backend, env, home, unadorned, settings)
                            for actual, original in zip(got['rows'], baseline['rows'], strict=True):
                                for key in ('question_digest', 'answer_id', 'observation_ids'):
                                    assert actual[key] == original[key], key
                                assert all(k not in original['question'] for k in ('name', 'wording_version')), original
                        if value.get('image_variants'):
                            assert_images(step, got, bodies)
                        if row['kind'] in ('images', 'image-location') and got['code'] == 0:
                            saved = invoke(binary, backend, env, home, step, {**settings, 'record': None, 'replay': str(home / 'recorded')})
                            assert int(backend.read('count')) == count and saved['requests_sent'] == 0
                            assert saved['rows'][0]['answer_id'] == got['rows'][0]['answer_id']
                    if identities:
                        ids = [g['rows'][0]['observation_ids'] for g in identities]
                        assert ids[0] == ids[1] == ids[2] and ids[3] != ids[0] and ids[4] == ids[3] and ids[5] == ids[0]
                        assert len({g['call_id'] for g in identities}) == 6
                finally:
                    backend.close()
        except (AssertionError, KeyError, ValueError, TypeError, OSError, ProtocolError, ToolError) as error:
            failures.append(row['id'])
            print(row['id'] + ': ' + str(error)[:1600], file=sys.stderr, flush=True)
        print('parity: ' + json.dumps({'consumer': 'mcp', 'case': row['id'],
              'checks': row.get('checks', ['named', 'runtime']), 'status': 'fail' if row['id'] in failures else 'pass'}), flush=True)
    print(f'MCP shared fixture results: {len(rows) - len(failures)} passed, {len(failures)} failed')
    return bool(failures)


if __name__ == '__main__':
    if len(sys.argv) != 3:
        raise SystemExit('usage: conformance.py LOOPBACK_PORT ABSOLUTE_THINKTHEN_BINARY')
    raise SystemExit(run(*sys.argv[1:]))
