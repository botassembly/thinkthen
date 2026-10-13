"""Shared fixture projections and assertions, plus installed C session parity.

Missing projections fail instead of producing successful or skipped cells.
"""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "conformance/children"))
ERRORS = {'usage': 1, 'backend': 2, 'local': 4, 'cancelled': 5, 'deadline': 3, 'defect': 6}


def compact(value):
    return json.dumps(value, ensure_ascii=False, separators=(',', ':'))


def document(row, cases, named):
    kind = row['kind']
    if kind == 'image-admission':
        from c_images import project
        return project(row)
    if kind in ('images', 'image-location', 'refusal'):
        base = document({**row, 'kind': 'behavior'}, cases, named)
        base.update(arm='arm/full/capture/v1', metadata_only=True,
                    settings={'backend': 'liquid', 'model': 'd1'},
                    image_paths=row['input'].get('images', []))
        base['items'] = base['items'][:1]
        if kind == 'image-location':
            base.update(paths=row['input']['paths'], source_unit=4)
        if kind == 'refusal':
            base['expect'] = {'error': 'usage', 'requests_sent': 0}
        return base
    if row['id'] == 'files-relate':
        return {'verb':'relate','question':row['input']['question'],'items':[], 'text':True,
                'paths':row['input']['paths'],'source_unit':3,'expect':row['expect'],'arm':'case/files-relate/v1'}
    if kind == 'boundary':
        base = {'verb': row['verb'], 'question': {'decide': 'Does this need attention?'},
                'items': ['evidence'], 'text': True, 'arm': 'arm/full/capture/v1',
                'expect': {'error': row['expect'].get('kind'), 'requests_sent': row['expect'].get('requests', 0)}}
        if row['id'].startswith('storage-'):
            base['question']={'decide':'Refund?'}
            base['items']=['Refund me.']
            base['settings']={'model':'literal'}
            if row['id'] == 'storage-no-store-record-refusal':
                return {**base,'arm':'arm/no-store/v1','settings':{'model':'literal','record':'$FOLDER'},'stored_answers':0}
            if row['id'] == 'storage-observation-identity':
                return {'arm':base['arm'],'identity_steps':True,'steps':[
                    {**base,'settings':{'model':'literal','record':'$FOLDER'},'expect':{'value':True,'requests_sent':1}},
                    {**base,'settings':{'model':'literal','cache':'$FOLDER'},'expect':{'value':True,'requests_sent':0},'count_delta':True},
                    {**base,'settings':{'model':'literal','replay':'$FOLDER'},'expect':{'value':True,'requests_sent':0},'count_delta':True},
                    {**base,'settings':{'model':'literal','cache':'$REFRESH','refresh_cache':True},'expect':{'value':True,'requests_sent':1},'count_delta':True,'copy_store':True},
                    {**base,'settings':{'model':'literal','cache':'$REFRESH'},'expect':{'value':True,'requests_sent':0},'count_delta':True},
                    {**base,'settings':{'model':'literal','replay':'$FOLDER'},'expect':{'value':True,'requests_sent':0},'count_delta':True}]}
            seed={**base,'settings':{'model':'literal','record':'$FOLDER'},'expect':{'value':True,'requests_sent':1}}
            replay={**base,'settings':{'model':'literal','replay':'$FOLDER'},'expect':{'error':'local','requests_sent':0},'count_delta':True}
            if row['id'] == 'storage-damaged-replay':
                replay['damage_store']=True
                return {'arm':base['arm'],'steps':[seed,replay]}
            if row['id'] == 'storage-ambiguous-replay':
                return {'arm':'arm/reported/v1','steps':[seed,{**seed,'expect':{'value':True,'requests_sent':2}},replay]}
        if row['id'] == 'cancellation-held-call':
            return {**base,'arm':'arm/held/v1','held_cancel':True,'expect':{'error':'cancelled','requests_sent':1}}
        if row['id'] == 'image-dropping-route':
            return {**base, 'image_paths': row['input']['images'], 'settings': {'backend': 'openrouter', 'model': 'arcee-ai/clef'}}
        if row['id'] in ('image-line-refusal', 'image-window-refusal'):
            return {**base, 'paths': ['missing-image-for-admission-order'], 'image_reader': True,
                    'source_unit': 1 if row['input']['unit'] == 'line' else 2, 'window': row['input'].get('window', 0)}
        if row['id'] == 'proxy-reservation-refusal':
            return {**base, 'settings': {'proxy': row['input']['proxy']}}
        if row['id'] == 'zero-observation-relate':
            return {**base, 'question': {'version': 1, 'relate': {'relations': [{'name': 'linked', **row['input']['relation']}]}},
                    'items': row['input']['entities'], 'text': False, 'zero_observations': True, 'expect': {}}
        raise ValueError('shared boundary projection not yet implemented')
    if kind == 'settings':
        fixture = next(c for c in json.loads((ROOT / 'conformance/settings.json').read_text())['cases'] if c['id'] == row['input']['case_ref'])
        steps = []
        for step in fixture['steps']:
            original_verb = step.get('verb', 'decide')
            verb = 'decide' if original_verb == 'decide_many' else original_verb
            question = {'decide': json.loads((ROOT / 'conformance/settings.json').read_text())['question']}
            items = step.get('records', [step.get('text', 'refund now')])
            if verb == 'relate':
                name, kinds = fixture['relation'].split('=')
                source, target = kinds.split(':')
                question = {'version': 1, 'relate': {'relations': [{'name': name, 'source': source, 'target': target}]}}
                items = fixture['entities']
            steps.append({'verb': verb, 'question': question, 'items': items, 'text': verb != 'relate', 'expect': step, 'settings': step['settings'], 'incremental': original_verb == 'decide_many'})
        return {'steps': steps, 'arm': fixture['arm'], 'profile': fixture.get('profile')}
    if kind == 'named-input':
        case = named[row['input']['case_ref']]
        given = case['input']
        if 'source_case' in given:
            base = document({**row, 'kind': 'behavior', 'input': {'case_ref': given['source_case']}}, cases, named)
            if row['verb'] == 'annotate':
                base['question'] = {**base['question'], 'questions': {
                    name: {**question, **given['metadata']}
                    for name, question in base['question']['questions'].items()}}
            else:
                base['question'] = {**base['question'], **given['metadata']}
            base['metadata'] = given['metadata']
            if 'records' in given:
                base.update(items=given['records'], contexts=given.get('contexts'),
                            shared_context=given.get('shared_context'), expect=case['expect'],
                            arm='case/'+row['id']+'/v1' if row.get('exchanges') else 'arm/full/capture/v1')
            if given.get('steps'):
                return {'arm':base['arm'], 'steps':[{**base, **step} for step in given['steps']]}
            return base
        if given.get('mode') == 'incremental':
            return {'verb': row['verb'], 'question': given['question'], 'items': given['records'],
                    'text': False, 'expect': case['expect'], 'settings': {'batch': given['batch']},
                    'owned_jsonl': True, 'paths': ['records.jsonl'], 'source_unit': 5,
                    'incremental': True, 'arm': 'arm/full/capture/v1'}
        if given.get('mode') == 'finite':
            return {'verb': row['verb'], 'question': given['question'], 'items': given['records'],
                    'text': False, 'expect': case['expect'],
                    'settings': {'batch': given['batch'], **({'model': given['model']} if 'model' in given else {})},
                    'shared_context': given.get('shared_context'),
                    'arm': 'case/'+row['id']+'/capture/v1' if row.get('exchanges') else 'arm/full/capture/v1'}
        if 'store' in given:
            store = given['store']
            return {'arm': 'arm/full/capture/v1', 'steps': [
                {'verb': row['verb'], 'question': store['seed_question'], 'items': [store['seed_item']], 'text': False,
                 'settings': {'record' if store['mode'] == 'replay' else 'cache': '$FOLDER'},
                 'expect': {'value': True, 'requests_sent': 1}},
                {'verb': row['verb'], 'question': given['question'], 'items': [given['item']], 'text': False,
                 'settings': {store['mode']: '$FOLDER'}, 'expect': case['expect'], 'count_delta': True}]}
        if given.get('reader') == 'csv':
            import csv, io
            return {'verb': row['verb'], 'question': given['question'], 'items': list(csv.DictReader(io.StringIO(given['data']))),
                    'text': False, 'expect': case['expect'], 'arm': 'arm/full/capture/v1'}
        if any(key in given for key in ('reader', 'store', 'mode', 'arguments')):
            raise ValueError('shared input projection not yet implemented: ' + ','.join(given))
        question = given.get('question', given.get('question_set'))
        raw = given.get('question_json')
        if question is None and raw is None and 'loader' not in given:
            raise ValueError('fixture supplies no C question grammar')
        question = question or {}
        if 'field' in given:
            question = {**question, 'on': given['field']}
        item = given.get('item', given.get('record', given.get('document')))
        return {'verb': row['verb'], 'question': question, 'raw': raw,
                'items': [item], 'text': isinstance(item, str), 'image_only': item is None and bool(given.get('images')), 'image_paths': given.get('images', []), 'settings': {'backend': 'liquid', 'model': 'd1'} if given.get('images') else {},
                'context': given.get('per_item_context'), 'context_present': 'per_item_context' in given,
                'shared_context': given.get('shared_context'), 'loader': given.get('loader'),
                'reference': given.get('reference', given.get('path')), 'file_body': given.get('question'), 'setup': given.get('setup'),
                'expect': case['expect'], 'arm': 'arm/full/capture/v1'}
    if kind not in ('behavior', 'typed-result', 'result2', 'type-fixture', 'located', 'error', 'packing'):
        raise ValueError('shared ' + kind + ' projection not yet implemented')
    reference = row['input'].get('case_ref')
    if kind == 'type-fixture':
        corpus = json.loads((ROOT / 'specification/fixtures/types/corpus.json').read_text())['cases']
        typed = next(value for value in corpus if value['name'] == reference)
        reference = typed.get('case_id')
        if typed['name'] == 'described-choice':
            request = dict(typed['request'])
            evidence = request.pop('evidence')
            return {'verb': 'choose', 'question': request, 'items': [evidence],
                    'text': True, 'expect': {'value': typed['response'], 'requests_sent': 1},
                    'arm': 'arm/full/capture/v1'}
        if reference is None:
            raise ValueError('type fixture lacks a public call case')
    case = cases[reference]
    question = case.get('question', case.get('question_set'))
    exchanges = case.get('exchanges', [])
    if question is None:
        raise ValueError('case has no public question grammar')
    if row['verb'] == 'find':
        items = question.get('units', [])
        question = {key: value for key, value in question.items() if key != 'units'}
    elif row['verb'] == 'recognize':
        items = [case['text']]
    elif row['verb'] == 'relate':
        items = case['entities']
    elif 'record' in case:
        items = [case['record']]
    else:
        items = [exchange['evidence'] for exchange in exchanges]
    operation = case.get('operation', {}).get('injection')
    if operation == 'internal_invariant_failure':
        raise ValueError('private invariant injection has no admitted public input')
    if operation:
        items = ['' if operation == 'invalid_arguments' else 'public refusal probe']
    return {'verb': row['verb'], 'question': question, 'items': items,
            'text': row['verb'] != 'relate', 'expect': case['expect'],
            'paths': row['input'].get('paths'), 'source_unit': {'file': 3, 'line': 1, 'window': 2}.get(row['input'].get('unit'), 3), 'arm': ('arm/full/capture/v1' if kind == 'located' else 'arm/refuse/v1' if operation == 'response_refusal' else 'generic/v1' if operation else 'case/' + reference + '/v1'),
            'operation': case.get('operation'), 'question_form': case.get('question_form'),
            'metadata_only': kind in ('typed-result', 'result2', 'located')}


class Backend:
    def __init__(self, binary, env):
        self.process = subprocess.Popen([str(binary)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=subprocess.PIPE, text=True, env=env)
        self.port = int(self.process.stdout.readline())
    def read(self, command):
        self.process.stdin.write(command + '\n')
        self.process.stdin.flush()
        return self.process.stdout.readline().strip()
    def close(self):
        self.process.stdin.close()
        code = self.process.wait(timeout=10)
        stderr = self.process.stderr.read()
        if code or stderr:
            raise AssertionError('owned fixture backend failed: ' + stderr)


def equivalent(got, want):
    if isinstance(want, (int, float)) and not isinstance(want, bool):
        return isinstance(got, (int, float)) and abs(got - want) <= 1e-10
    if isinstance(want, dict):
        return isinstance(got, dict) and set(got) == set(want) and all(equivalent(got[key], value) for key, value in want.items())
    if isinstance(want, list):
        return isinstance(got, list) and len(got) == len(want) and all(equivalent(left, right) for left, right in zip(got, want))
    return type(got) is type(want) and got == want


def assertions(row, value, got, count):
    expect = value['expect']
    error = expect.get('error')
    if error is not None:
        error = error.get('kind') if isinstance(error, dict) else error
        assert got['code'] == ERRORS[error], got
        assert got['message'], got
        if 'message' in expect:
            assert got['message'] == expect['message'], got
        if 'completed_prefix' in expect:
            assert [answer['index'] for answer in got['completed']] == expect['completed_prefix'], got
            assert all(len(answer['answer_id']) == 64 and answer['observations'] == answer['sources'] > 0 for answer in got['completed']), got
            assert got['stopped_at'] == expect['stopped_at'] and got['message'] == expect['message'], got
            assert got['requests_sent'] == expect['requests_sent'] and got['records'] == len(expect['completed_prefix']), got
        if 'requests_sent' in expect or 'count' in expect:
            assert count == expect.get('requests_sent', expect.get('count')), (count, expect)
        return
    assert got['code'] == 0, got
    assert got['schema'] == 'thinkthen.result/2' and len(got['call_id']) == 64, got
    if value.get('zero_observations'):
        answer = got['rows'][0]
        assert count == 0 and got['requests_sent'] == 0 and answer['value'] == [], got
        assert answer['origin'] is None and answer['answered_by'] is None and answer['observations'] == answer['sources'] == 0, got
        return
    for answer in got['rows']:
        assert len(answer['answer_id']) == 64 and answer['observations'] == answer['sources'] > 0, answer
        assert answer['origin'] in (1, 2, 3) and answer['answered_by'], answer
        if value.get('metadata'):
            for name, field in value['metadata'].items():
                if name in ('name', 'wording_version'):
                    if value['verb'] == 'annotate':
                        assert all(author.get(name) == field for author in answer['member_authors']), answer
                    else:
                        assert answer.get(name) == field, answer
    if row['kind'] in ('images', 'image-location'):
        expected = value.get('image_paths') or value['paths']
        assert [bytes.fromhex(data) for data in got['rows'][0]['images']] == [(ROOT / path).read_bytes() for path in expected], got
        if row['kind'] == 'image-location':
            assert len(got['rows']) == 1 and 'first_line' not in got['rows'][0] and 'last_line' not in got['rows'][0], got
        return
    if row['kind'] == 'located':
        assert got['rows'], got
        if value['verb'] not in ('find', 'relate'):
            assert {Path(answer['file']).name for answer in got['rows']} == set(row['expect']['locations']), got
        else:
            inputs = got['rows'][0]['detail_inputs']
            assert {Path(answer['file']).name for answer in inputs} == set(row['expect']['locations']), got
            for original in inputs:
                assert original['input'] == Path(original['file']).read_text(), original
                assert original['first_line'] == 1 and original['last_line'] == len(Path(original['file']).read_text().splitlines()), original
        return
    if value.get('metadata_only'):
        assert got['rows'], got
        return
    if 'rank_members' in expect:
        assert count == expect['requests_sent'] and got['requests_sent'] == expect['requests_sent'], got
        assert got['records'] == expect['records'] and got['input_tokens'] == expect['input_tokens'], got
        assert 'output_tokens' not in got, got
        assert len(got['rows']) == len(expect['rank_members']), got
        for actual, wanted in zip(got['rows'], expect['rank_members'], strict=True):
            for key in ('index', 'value', 'question_name', 'probability'):
                assert equivalent(actual[key], wanted[key]), (actual, wanted)
            assert actual['usage'] == {'input_tokens': 4} and actual['model'] == 'fixed', actual
            assert len(actual['context_digest']) == 64 and actual['source_batch_sizes'] == [6,6], actual
            members = actual['members']
            assert [m['name'] for m in members] == expect['member_names'], actual
            assert [m['author'] for m in members] == expect['member_authors'], actual
            assert members[0]['wording_version'] == 4 and 'wording_version' not in members[1], actual
            assert [m['value'] for m in members] == wanted['member_positions'], actual
            assert equivalent([m['probability'] for m in members], wanted['member_probabilities']), actual
            assert len({m['answer_id'] for m in members}) == len(members), actual
            for m in members:
                assert len(m['answer_id']) == 64 and m['answer_id'] != actual['answer_id'], m
                assert m['usage'] == {'input_tokens': 2} and m['model'] == 'fixed', m
                assert m['source_batch_sizes'] == [6] and m['observations'] == m['sources'] == 1, m
                assert m['context_digest'] == actual['context_digest'], m
        return
    if 'edges' in expect:
        assert len(got['rows'][0]['value']) == expect['edges'], got
    if 'values' in expect:
        assert equivalent([r['value'] for r in got['rows']], expect['values']), (got, expect)
    if 'success' not in expect:
        for key in ('value', 'probability'):
            if key in expect:
                assert equivalent(got['rows'][0][key], expect[key]), (got, expect)
        if 'requests_sent' in expect or 'count' in expect:
            assert count == expect.get('requests_sent', expect.get('count')), (count, expect)
        return
    if 'count' in expect:
        assert count == expect['count'], (count, expect)
    success = expect['success']
    answers = success.get('answers', [])
    actual = [answer['value'] for answer in got['rows']]
    verb = value['verb']
    if verb == 'filter':
        indexes = [answer['index'] for answer in got['rows'] if answer['value']]
        assert indexes == success['operation']['indexes'], (got, expect)
    elif verb == 'rank':
        wanted = success['operation']['ranking']
        assert [answer['index'] for answer in got['rows']] == [answer['index'] for answer in wanted], (got, expect)
        assert all(equivalent(left['probability'], right['probability']) for left, right in zip(got['rows'], wanted)), (got, expect)
    elif verb == 'find':
        assert got['rows'][0]['index'] == success['operation']['selected'], (got, expect)
    elif verb == 'annotate':
        if len(got['rows']) == 1:
            wanted = {answer['name']: answer['bare'] for answer in answers}
            assert equivalent(actual[0], wanted), (got, wanted)
        else:
            grouped = [{answer['name']: answer['bare'] for answer in answers if answer['exchange'] == at} for at in range(len(got['rows']))]
            assert equivalent(actual, grouped), (got, grouped)
    else:
        wanted = [answer['bare'] for answer in answers]
        if verb == 'decide':
            wanted = [value['question'].get('true' if answer is True else 'false', answer) if isinstance(answer, bool) else answer for answer in wanted]
        assert equivalent(actual, wanted), (got, wanted)
        for answer, wanted in zip(got['rows'], answers):
            original = wanted.get('details', {}).get('answer', {})
            for key in ('probability', 'probabilities'):
                if key in original:
                    assert equivalent(answer[key], original[key]), (answer, wanted)


def prepare(home, value):
    if value.get('caption_files'):
        for at, caption in enumerate(value['items']):
            (home / ('caption-%d.txt' % at)).write_text(caption)
    if value.get('owned_jsonl'):
        (home / 'records.jsonl').write_text(''.join(compact(item) + '\n' for item in value['items']))
    if value.get('profile'):
        (home / 'profile.json').write_text(compact(value['profile']))
    if value.get('question_form') == 'file':
        (home / 'fixture-question.json').write_text(compact(value['question']))
    setup = value.get('setup') or {}
    questions = home / ('Library/Application Support/thinkthen/questions' if sys.platform == 'darwin' else 'config/thinkthen/questions')
    questions.mkdir(parents=True, exist_ok=True)
    if setup.get('questions_symlink_outside'):
        questions.rmdir()
        outside = home / 'outside'
        outside.mkdir()
        (outside / 'refund.json').write_text(compact(value['question']))
        questions.symlink_to(outside, target_is_directory=True)
    if 'named_file' in setup or 'named_file_json' in setup:
        (questions / 'refund.json').write_text(setup.get('named_file_json', compact(setup.get('named_file'))))
    if setup.get('file_symlink_outside'):
        outside = home / 'outside.json'
        outside.write_text(compact(value['question']))
        (questions / 'refund.json').symlink_to(outside)
    for key, path in [('cwd_refund', 'refund'), ('cwd_at_refund', '@refund')]:
        if key in setup:
            (home / path).write_text(compact(setup[key]))
    if 'parent_refund_json' in setup:
        (home.parent / 'refund.json').write_text(compact(setup['parent_refund_json']))
    if setup.get('cwd_refund_directory'):
        (home / 'refund').mkdir()
    if value.get('loader') == 'load' and value.get('file_body') is not None:
        (home / value['reference']).write_text(compact(value['file_body']))


def main(*, header=None, library=None):
    header = header or os.environ.get('THINKTHEN_C_HEADER')
    library = library or os.environ.get('THINKTHEN_C_LIBRARY')
    if bool(header) != bool(library) or (os.environ.get('THINKTHEN_ARTIFACT') and not header):
        raise ValueError('installed C parity requires its explicit header and library')
    if header:
        header, library = Path(header).resolve(strict=True), Path(library).resolve(strict=True)
        if not header.is_file() or not library.is_file():
            raise ValueError('installed C parity header or library is missing')
    from children import child_env
    env = child_env(keep=('HOME', 'CARGO_HOME', 'RUSTUP_HOME', 'RUSTUP_TOOLCHAIN', 'CARGO_TARGET_DIR'),
                    CARGO_BUILD_JOBS='2', CARGO_NET_OFFLINE='true')
    target = Path(os.environ.get('CARGO_TARGET_DIR', str(ROOT / 'target'))).resolve()
    env['CARGO_TARGET_DIR'] = str(target)
    if not header:
        subprocess.run(['cargo', 'build', '--locked', '--offline', '--lib'],
                       cwd=ROOT / 'libraries/c', env=env, check=True)
        header, library = ROOT / 'libraries/c/include/thinkthen.h', target / 'debug/libthinkthen_c.so'
    subprocess.run(['cargo', 'build', '--locked', '--offline', '-p', 'conformance-backend'],
                   cwd=ROOT, env=env, check=True)
    sys.path.insert(0, str(ROOT / 'libraries/c/tests'))
    from installed_cases import run
    with tempfile.TemporaryDirectory(prefix='thinkthen-c-parity-') as folder:
        run(None, Path(folder), header=header, library=library)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
