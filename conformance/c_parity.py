"""Run shared fixture inputs through the installed typed C calls and readers.

Saved grammar is imported through a native role. Calls, controls, input records
and known result fields use counted C descriptors, never the JSON call door.
Missing projections are reported as failures, never as successful/skipped cells.
"""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
FUNCTIONS = ['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate']
ERRORS = {'usage': 1, 'backend': 2, 'local': 4, 'cancelled': 5, 'deadline': 3, 'defect': 6}


def compact(value):
    return json.dumps(value, ensure_ascii=False, separators=(',', ':'))


def literal(value):
    # ASCII octal strings preserve exact UTF-8 bytes without C escape ambiguity.
    return '"' + ''.join('\\%03o' % byte for byte in value.encode()) + '"'


def counted(value):
    return '(thinkthen_string_v1){%s,%d}' % (literal(value), len(value.encode()))


def content(value, text=False):
    return '(thinkthen_content_v1){%d,%s}' % (1 if text else 2, counted(value if text else compact(value)))


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
            return base
        if given.get('mode') == 'incremental':
            return {'verb': row['verb'], 'question': given['question'], 'items': given['records'],
                    'text': False, 'expect': case['expect'], 'settings': {'batch': given['batch']},
                    'owned_jsonl': True, 'paths': ['records.jsonl'], 'source_unit': 5,
                    'incremental': True, 'arm': 'arm/full/capture/v1'}
        if given.get('mode') == 'finite':
            return {'verb': row['verb'], 'question': given['question'], 'items': given['records'],
                    'text': False, 'expect': case['expect'], 'settings': {'batch': given['batch']},
                    'arm': 'arm/full/capture/v1'}
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


def role(verb, question):
    if verb == 'rank':
        return 7 if 'questions' in question else 6
    return {'annotate': 2, 'find': 8, 'recognize': 4, 'relate': 5}.get(verb, 1)


def generated(at, value):
    verb = value['verb']
    kind = FUNCTIONS.index(verb) + 1
    question = value['raw'] if value.get('raw') is not None else compact(value['question'])
    image_paths = value.get('image_paths', [])
    lines = ['static void case_%d(thinkthen_engine *e) {' % at,
             'thinkthen_question *q=NULL; thinkthen_source *source=NULL; thinkthen_result *result=NULL;',
             'thinkthen_controls_v1 controls={0}; controls.deadline_ms=-1; controls.attempts=1;',
             'int code=0;', 'thinkthen_cancel_token *cancel=NULL;', 'thrd_t cancellation_thread; int cancellation_started=0;',
             'thinkthen_image *images[%d]={0};' % max(1, len(image_paths)), 'char *caption_allocations[%d]={0};' % max(1, len(value['items']))]
    if verb == 'find' and value['question'].get('none'):
        text = value['question']['find']
        lines.extend(['thinkthen_question_spec_v1 question_spec={0}; question_spec.kind=7; question_spec.none=1;', 'question_spec.text=%s;' % content(text, isinstance(text, str)),  ])
        if value.get('metadata'):
            metadata = value['metadata']
            lines.extend(['thinkthen_question_author_v1 author={0};',
                          'author.name=(thinkthen_optional_string_v1){1,%s};' % counted(metadata['name']),
                          'author.wording_version=(thinkthen_optional_u64_v1){1,%d};' % metadata['wording_version'],
                          'code=thinkthen_question_new_authored(e,&question_spec,&author,&q);'])
        else:
            lines.append('code=thinkthen_question_new(e,&question_spec,&q);')
    elif value.get('loader'):
        loader = value['loader']
        method = {'load_named': 'load_named', 'load_reference': 'load_reference', 'load': 'load', 'named': 'load_named', 'reference': 'load_reference', 'file': 'load'}.get(loader)
        if method is None:
            raise ValueError('unknown native loader ' + str(loader))
        args = ('e,%d,%s,&q' % (role(verb, value['question']), counted(value['reference']))) if method != 'load' else ('e,%s,&q' % counted(value['reference']))
        if method == 'load':
            args = 'e,%s,&q' % counted(value['reference'])
        lines.append('code=thinkthen_question_%s(%s);' % (method, args))
    elif value.get('question_form') == 'file':
        lines.append('code=thinkthen_question_load(e,%s,&q);' % counted('fixture-question.json'))
    else:
        lines.append('code=thinkthen_question_parse(e,%d,%s,&q);' % (role(verb, value['question']), counted(question)))
    if (value.get('operation') or {}).get('injection') == 'cancel_token':
        lines.extend(['cancel=thinkthen_cancel_token_new(); if(!cancel) abort(); thinkthen_cancel(cancel); controls.cancel=cancel;'])
    if (value.get('operation') or {}).get('injection') == 'expired_deadline':
        lines.append('controls.deadline_ms=0;')
    lines.append('if(code) goto finished;')
    for index, path in enumerate(image_paths if not value.get('paths') else []):
        raw = (ROOT / path).read_bytes()
        data = '"' + ''.join('\\%03o' % byte for byte in raw) + '"'
        lines.extend(['code=thinkthen_image_clone(e,(const uint8_t *)%s,%d,%d,(thinkthen_optional_string_v1){0},&images[%d]);' % (data, len(raw), 1 if value.get('media') == 'image/jpeg' else 2, index), 'if(code) goto finished;'])
    items = value['items']
    if value.get('paths') or (value.get('operation') or {}).get('injection') == 'recording_read_failure':
        paths = [path if value.get('owned_jsonl') else str(ROOT / path) for path in (value.get('paths') or ['target/c-parity-missing-input'])]
        lines.extend(['thinkthen_string_v1 paths[]={%s};' % ','.join(map(counted, paths)),
                      'thinkthen_source_spec_v1 spec={{paths,%d},%d,%d};' % (len(paths), value.get('source_unit', 3), value.get('window', 0)),
                      'code=thinkthen_source_%s(e,&spec,&source);' % ('image_files' if value.get('image_reader') else 'files')])
    else:
        lines.append('thinkthen_record_v1 records[%d]={0};' % max(1, len(items)))
        for index, item in enumerate(items):
            if value.get('image_only'):
                continue
            if value.get('caption_files'):
                lines.append('caption_allocations[%d]=read_caption("caption-%d.txt",&records[%d].original);' % (index,index,index))
            else:
                lines.append('records[%d].original=(thinkthen_optional_content_v1){1,%s};' % (index, content(item, value.get('text', False) and isinstance(item, str))))
            if value.get('candidate_orders'):
                order=value['candidate_orders'][index]
                lines.append('thinkthen_choice_v1 options_%d[]={%s};' % (index, ','.join('{.name=%s}' % counted(option) for option in order)))
                lines.append('records[%d].options=(thinkthen_choices_v1){options_%d,%d};' % (index,index,len(order)))
            if image_paths:
                lines.append('records[%d].images=(thinkthen_images_v1){(const thinkthen_image *const *)images,%d};' % (index, len(image_paths)))
            if value.get('context_present'):
                context = value['context']
                lines.append('records[%d].context=(thinkthen_optional_content_v1){1,%s};' % (index, content(context, isinstance(context, str))))
        if value.get('image_only'):
            lines.append('records[0].images=(thinkthen_images_v1){(const thinkthen_image *const *)images,%d};' % len(image_paths))
        lines.append('code=thinkthen_source_records(e,records,%d,&source);' % len(items))
    if value.get('shared_context') is not None:
        context = value['shared_context']
        lines.append('controls.context=(thinkthen_optional_content_v1){1,%s};' % content(context, isinstance(context, str)))
    if value.get('held_cancel'):
        lines.extend(['if(code) goto finished;', 'cancel=thinkthen_cancel_token_new(); if(!cancel) abort(); controls.cancel=cancel;', 'if(thrd_create(&cancellation_thread,cancel_on_input,cancel)!=thrd_success) { abort(); } cancellation_started=1;'])
    if value.get('incremental'):
        lines.extend(['if(code) goto finished;', 'thinkthen_batch *batch=NULL;',
                      'code=thinkthen_%s_batch_start(e,q,source,&controls,&batch); if(code) goto finished;' % verb,
                      'while((code=thinkthen_batch_next(batch,&result))==0 && result) { output(e,%d,0,result); thinkthen_result_free(result); result=NULL; }' % kind,
                      'if(thinkthen_batch_facts(batch,&result)) abort();', 'thinkthen_batch_free(batch);'])
    else:
        lines.extend(['if(code) goto finished;', 'code=thinkthen_%s_complete(e,q,source,&controls,&result);' % verb])
    lines.extend(['finished: if(cancellation_started && thrd_join(cancellation_thread,NULL)!=thrd_success) abort(); output(e,%d,code,result); thinkthen_result_free(result); thinkthen_source_free(source); thinkthen_question_free(q); thinkthen_cancel_token_free(cancel); for(size_t i=0;i<%d;++i) thinkthen_image_free(images[i]); for(size_t i=0;i<%d;++i) free(caption_allocations[i]);' % (kind, max(1, len(image_paths)), max(1,len(items))), '}'])
    return '\n'.join(lines)


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
    if 'edges' in expect:
        assert len(got['rows'][0]['value']) == expect['edges'], got
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
    questions = home / 'config/thinkthen/questions'
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


def main():
    import parity
    inventory = parity.inventory()
    rows = list(parity.required_cases(inventory, 'c').values())
    cases = {row['id']: row for row in json.loads((ROOT / 'conformance/cases.json').read_text())['cases']}
    named = {row['id']: row for row in json.loads((ROOT / 'conformance/named-inputs.json').read_text())['cases']}
    names = ('PATH', 'HOME', 'CARGO_HOME', 'RUSTUP_HOME', 'RUSTUP_TOOLCHAIN',
             'RUSTFLAGS', 'CFLAGS', 'TMPDIR', 'XDG_RUNTIME_DIR',
             'THINKTHEN_HEAVY_LOCK', 'THINKTHEN_HEAVY_LOCK_HELD')
    env = {name: value for name in names if (value := os.environ.get(name)) is not None}
    env.update(CARGO_BUILD_JOBS='1', CARGO_NET_OFFLINE='true')
    subprocess.run(['cargo', 'build', '--locked', '--offline', '--lib'], cwd=ROOT / 'libraries/c', env=env, check=True)
    subprocess.run(['cargo', 'build', '--locked', '--offline', '-p', 'conformance-backend'], cwd=ROOT, env=env, check=True)
    with tempfile.TemporaryDirectory(prefix='thinkthen-c-parity-') as folder:
        scratch = Path(folder)
        definitions, ready, errors = [], {}, {}
        for at, row in enumerate(rows):
            try:
                value = document(row, cases, named)
                if 'steps' in value:
                    for step_at, step in enumerate(value['steps']):
                        definitions.append(generated(10000 + at * 100 + step_at, step))
                else:
                    definitions.append(generated(at, value))
                ready[at] = value
            except (KeyError, ValueError, TypeError, AttributeError) as error:
                errors[at] = str(error)
        source = scratch / 'driver.c'
        numbers = [at for at, value in ready.items() if 'steps' not in value]
        numbers.extend(10000 + at * 100 + step for at, value in ready.items()
                       if 'steps' in value for step in range(len(value['steps'])))
        dispatch = '\n'.join('case %d: case_%d(e); break;' % (number, number)
                             for number in numbers)
        source.write_text('#include <threads.h>\n#include "%s"\n' % (ROOT / 'libraries/c/tests/c/parity_output.c')
                          + '\nint cancel_on_input(void *token) { if(getchar()!=\'!\') abort(); thinkthen_cancel(token); fputs("cancel-fired\\n",stdout); fflush(stdout); return 0; }\n'
                          + '\nchar *read_caption(const char *path,thinkthen_optional_content_v1 *out) { FILE *f=fopen(path,"rb"); if(!f || fseek(f,0,SEEK_END)) abort(); long n=ftell(f); if(n<0 || n>16777216 || fseek(f,0,SEEK_SET)) abort(); char *s=malloc((size_t)n+1); if(!s || fread(s,1,(size_t)n,f)!=(size_t)n || fclose(f)) abort(); s[n]=0; *out=(thinkthen_optional_content_v1){1,{1,{s,(size_t)n}}}; return s; }\n'
                          + '\n'.join(definitions)
                          + '\nint main(int argc,char **argv) { if(argc!=3) return 2; '
                          'if(!strstr(argv[2], \"\\\"base_url\\\":\\\"http://127.0.0.1:\")) return 3; '
                          'thinkthen_engine *e=thinkthen_engine_new_with(argv[2]); '
                          'if(!e) { output(NULL,1,thinkthen_error_code(NULL),NULL); return 0; } switch(atoi(argv[1])) {\n' + dispatch
                          + '\ndefault: return 2; } thinkthen_engine_free(e); return 0; }\n')
        binary = scratch / 'driver'
        library = ROOT / 'libraries/c/target/debug'
        # Match the shipped soname using an owned directory; do not alter build files.
        (scratch / 'libthinkthen.so.0').symlink_to(library / 'libthinkthen_c.so')
        subprocess.run(['cc', '-pthread', '-std=c11', '-Wall', '-Wextra', '-Werror', '-g', '-fsanitize=address', '-fno-omit-frame-pointer', '-I', str(ROOT / 'libraries/c/include'), str(source), '-L', str(library), '-lthinkthen_c', '-Wl,-rpath,' + str(scratch), '-o', str(binary)], env=env, check=True)
        failures = 0
        for at, row in enumerate(rows):
            error = errors.get(at)
            if error is None:
                try:
                    with tempfile.TemporaryDirectory(prefix='case-', dir=scratch) as owned:
                        home = Path(owned)
                        child = {'PATH': os.environ.get('PATH', '/usr/bin:/bin'), 'HOME': str(home), 'XDG_CONFIG_HOME': str(home / 'config'), 'XDG_CACHE_HOME': str(home / 'cache'), 'XDG_STATE_HOME': str(home / 'state'), 'ASAN_OPTIONS': 'detect_leaks=1'}
                        backend = Backend(ROOT / 'target/debug/conformance-backend', child)
                        try:
                            value = ready[at]
                            child.update(THINKTHEN_BASE_URL='http://127.0.0.1:%d/%s' % (backend.port, value['arm']), THINKTHEN_API_KEY='sk-conformance-loopback', LIQUIDAI_API_KEY='sk-conformance-loopback', OPENROUTER_API_KEY='sk-conformance-loopback')
                            prepare(home, value)
                            steps = value.get('steps', [value])
                            identities=[]
                            for step_at, step in enumerate(steps):
                                if step.get('copy_store'):
                                    import sqlite3
                                    (home / 'refreshed').mkdir()
                                    with sqlite3.connect(home / 'saved/thinkthen.sqlite') as original, sqlite3.connect(home / 'refreshed/thinkthen.sqlite') as target:
                                        original.backup(target)
                                if step.get('damage_store'):
                                    import sqlite3
                                    with sqlite3.connect(home / 'saved/thinkthen.sqlite') as db:
                                        db.execute("UPDATE answers SET answer='damaged fixture answer'")
                                if value.get('image_variants'):
                                    backend.close()
                                    backend = Backend(ROOT / 'target/debug/conformance-backend', child)
                                    child['THINKTHEN_BASE_URL']='http://127.0.0.1:%d/%s' % (backend.port, step['arm'])
                                    child['PERPLEXITY_API_KEY']='sk-conformance-loopback'
                                    prepare(home,step)
                                settings = {'cache': False, 'model': 'jev-latest' if 'steps' in value else 'jev-1.13.0', 'batch': 1, 'max_retries': 0}
                                settings.update(step.get('settings', {}))
                                settings['base_url'] = child['THINKTHEN_BASE_URL']
                                settings = {key: (str(home / 'saved') if entry == '$FOLDER' else str(home / 'refreshed') if entry == '$REFRESH' else str(home / 'profile.json') if entry == '$PROFILE' else entry) for key, entry in settings.items()}
                                if row['kind'] in ('images', 'image-location'):
                                    settings['record'] = str(home / 'recorded')
                                number = 10000 + at * 100 + step_at if 'steps' in value else at
                                before = int(backend.read('count'))
                                if step.get('held_cancel'):
                                    running=subprocess.Popen([str(binary),str(number),compact(settings)],env=child,cwd=home,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
                                    try:
                                        assert backend.read('wait 1') == 'wait 1'
                                        running.stdin.write('!')
                                        running.stdin.flush()
                                        assert running.stdout.readline() == 'cancel-fired\n'
                                        backend.process.stdin.write('release\n')
                                        backend.process.stdin.flush()
                                        stdout,stderr=running.communicate(timeout=60)
                                        output=subprocess.CompletedProcess(running.args,running.returncode,stdout,stderr)
                                    finally:
                                        backend.process.stdin.write('release\n')
                                        backend.process.stdin.flush()
                                        if running.poll() is None:
                                            running.kill()
                                            running.wait()
                                else:
                                    output = subprocess.run([str(binary), str(number), compact(settings)], env=child, cwd=home, capture_output=True, text=True, timeout=120 if value.get('image_variants') else 60)
                                assert output.returncode == 0 and not output.stderr, output.stderr
                                if step.get('incremental'):
                                    emitted = [json.loads(line) for line in output.stdout.splitlines()]
                                    got = {**emitted[-1], 'completed': [answer for partial in emitted[:-1] for answer in partial['rows']]}
                                    if step.get('owned_jsonl'):
                                        bodies = json.loads(backend.read('capture'))['bodies']
                                        request = json.loads(bodies[0])
                                        expected = {'q%d' % (index + 1): {'type': 'noul', 'instructions': 'The text is %s. %s' % (compact(item), step['question']['decide'])} for index, item in enumerate(step['items'][:2])}
                                        assert request['questions'] == expected, request
                                else:
                                    got = json.loads(output.stdout)
                                if value.get('identity_steps'):
                                    identities.append(got)
                                if step.get('stored_answers') == 0:
                                    import sqlite3
                                    path=home / 'saved/thinkthen.sqlite'
                                    if path.exists():
                                        with sqlite3.connect(path) as db:
                                            assert db.execute('SELECT count(*) FROM answers').fetchone()[0] == 0
                                if value.get('image_variants'):
                                    from c_images import assert_images
                                    assert_images(step,got,json.loads(backend.read('capture'))['bodies'])
                                assertions(row, step, got, int(backend.read('count')) - (before if step.get('count_delta') else 0))
                                if row['kind'] in ('images', 'image-location'):
                                    before = int(backend.read('count'))
                                    replay = {key: entry for key, entry in settings.items() if key != 'record'}
                                    replay['replay'] = str(home / 'recorded')
                                    repeated = subprocess.run([str(binary), str(number), compact(replay)], env=child, cwd=home, capture_output=True, text=True, timeout=60)
                                    assert repeated.returncode == 0 and not repeated.stderr, repeated.stderr
                                    saved = json.loads(repeated.stdout)
                                    assertions(row, step, saved, int(backend.read('count')))
                                    assert saved['requests_sent'] == 0 and int(backend.read('count')) == before, saved
                                    assert saved['rows'][0]['answer_id'] == got['rows'][0]['answer_id'], saved
                            if value.get('identity_steps'):
                                ids=[entry['rows'][0]['observation_ids'] for entry in identities]
                                assert ids[0] == ids[1] == ids[2] and ids[3] != ids[0] and ids[4] == ids[3] and ids[5] == ids[0], ids
                                assert len({entry['call_id'] for entry in identities}) == 6, identities
                                assert len({identities[i]['rows'][0]['answer_id'] for i in (0,1,2,5)}) == 1, identities
                                assert identities[3]['rows'][0]['answer_id'] == identities[4]['rows'][0]['answer_id'] != identities[0]['rows'][0]['answer_id'], identities
                        finally:
                            backend.close()
                except (AssertionError, ValueError, KeyError, subprocess.SubprocessError, OSError) as failure:
                    error = type(failure).__name__ + ': ' + str(failure)
            if error is not None:
                failures += 1
                print('C fixture %s failed: %s' % (row['id'], error), file=sys.stderr)
            print('parity: ' + json.dumps({'consumer': 'c', 'case': row['id'], 'checks': row.get('checks', ['named', 'runtime']), 'status': 'fail' if error is not None else 'pass'}), flush=True)
        print('C shared fixture results: %d passed, %d failed' % (len(rows) - failures, failures))
        return bool(failures)


if __name__ == '__main__':
    raise SystemExit(main())
