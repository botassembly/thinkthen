"""Read typed answers through ten compiled COBOL blocking calls."""
import ctypes as ct
import ctypes.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from session_records import Records, ROOT, definitions, template, JsonValue, JsonEntry
sys.path[:0] = [str(ROOT / 'conformance/children'), str(ROOT / 'conformance')]
from children import child_env
from backend import Backend
from c_parity import Backend as FixtureBackend

PACKAGE = Path(os.environ.get('THINKTHEN_SESSION_PACKAGE', ROOT / 'libraries/cobol')).resolve()
HEADER = Path(os.environ.get('THINKTHEN_C_HEADER', ROOT / 'libraries/c/include/thinkthen.h')).resolve()
NATIVE = Path(os.environ['THINKTHEN_C_LIBRARY']).resolve()
BACKEND_BINARY = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target')) / 'debug/conformance-backend'

class Text(ct.Structure):
    _fields_ = [('length', ct.c_uint64), ('text', ct.c_char * 8192)]
def status_type():
    with tempfile.TemporaryDirectory(prefix='cobol-status-abi-') as temporary:
        header = Path(temporary) / 'status.h'
        header.write_text('#include ' + json.dumps(str(HEADER)) + '\n#include ' + json.dumps(str(PACKAGE / 'src/tt_requests_generated.h')) + '\n')
        facts = template.abi_module(ROOT).header_abi(header)
    records, types = facts['records'], {}
    def record(name):
        if name in types: return types[name]
        cls = type(name, (ct.Structure,), {})
        types[name] = cls
        fields = []
        for member, field in records[name]['fields'].items():
            if '.' in member: continue
            kind = field['type']
            nested = kind.removeprefix('struct ')
            if '*' in kind: typ = ct.c_void_p
            elif kind.startswith('char['): typ = ct.c_char * field['width']
            elif nested in records: typ = record(nested)
            else: typ = {'uint32_t': ct.c_uint32, 'uint64_t': ct.c_uint64, 'int64_t': ct.c_int64, 'size_t': ct.c_size_t, 'double': ct.c_double}[kind]
            fields.append((member, typ))
        cls._fields_ = fields
        assert ct.sizeof(cls) == records[name]['size'], name
        return cls
    return record
record_type = status_type()
Status = record_type('thinkthen_cobol_AnswerStatus')
class Table(ct.Structure):
    _fields_ = [('owner', ct.c_void_p), ('rows', ct.c_void_p), ('count', ct.c_uint64)]
class Decision(ct.Structure):
    _fields_ = [('present', ct.c_uint64), ('value', ct.c_uint64)]
class Choice(ct.Structure):
    _fields_ = [('present', ct.c_uint64), ('value', Text)]
class Find(ct.Structure):
    _fields_ = [('present', ct.c_uint64), ('index', ct.c_uint64), ('original', ct.c_void_p)]
class Annotation(ct.Structure):
    _fields_ = [('record', ct.c_uint64), ('name', Text), ('kind', ct.c_uint64), ('boolean', ct.c_uint64), ('number', ct.c_double), ('text', Text), ('tags', ct.c_void_p), ('tag_count', ct.c_uint64), ('failure_kind', ct.c_uint64), ('failure_cause', ct.c_uint64)]
class Recognition(ct.Structure):
    _fields_ = [('record', ct.c_uint64), ('proposal', ct.c_uint64), ('kind', Text), ('text', Text), ('start', ct.c_uint64), ('end', ct.c_uint64), ('length', ct.c_uint64), ('probability', ct.c_double)]
class Relation(ct.Structure):
    _fields_ = [('record', ct.c_uint64), ('relation', Text), ('source', Text), ('source_kind', Text), ('target', Text), ('target_kind', Text), ('probability', ct.c_double), ('either_present', ct.c_uint64), ('either', ct.c_uint64)]

Table = record_type('thinkthen_cobol_AnswerTable')
Recognition = record_type('thinkthen_cobol_AnswerRecognition')
RecognitionEdge = record_type('thinkthen_cobol_AnswerRecognitionEdge')
Plan = record_type('thinkthen_cobol_AnswerPlan')


questions = {
 'decide': {'decide': 'Refund?'}, 'choose': {'choose': 'Which?', 'options': ['a', 'b']},
 'tag': {'tag': 'Which?', 'labels': ['a', 'b']}, 'score': {'score': 'How much?', 'levels': ['low', 'high']},
 'filter': {'decide': 'Refund?'}, 'rank': {'decide': 'Relevant?'}, 'find': {'find': 'Which?'},
 'annotate': {'version': 1, 'questions': {'refund': {'decide': 'Refund?'}}},
 'recognize': {'version': 1, 'recognize': {'kinds': {'person': 'A name.'}}},
 'relate': {'version': 1, 'relate': {'relations': [{'name': 'linked', 'source': 'person', 'target': 'person'}]}},
}
OUTPUT_TYPES = {function: record_type('thinkthen_cobol_Answer' + function.title() + 'Result') for function in questions}

with tempfile.TemporaryDirectory(prefix='cobol-blocking-') as temporary:
    work = Path(temporary)
    home = work / 'home'; home.mkdir()
    programs = []
    for function in questions:
        programs.append(f'''identification division.
program-id. BLOCK_{function.upper()}.
data division.
working-storage section.
copy "tt-native-generated.cpy".
linkage section.
01 engine usage pointer.
01 question usage pointer.
01 input-record usage pointer.
01 option-record usage pointer.
01 answer usage pointer.
01 status-record usage pointer.
procedure division using engine question input-record option-record answer status-record.
 set address of tt-n-cobol-Answer{function.title()}Result to answer
 set address of tt-n-cobol-AnswerStatus to status-record
 call "TT_{function.upper()}" using by value engine question input-record option-record
   by reference tt-n-cobol-Answer{function.title()}Result tt-n-cobol-AnswerStatus returning return-code
 goback.
end program BLOCK_{function.upper()}.
''')
    programs.append("""identification division.
program-id. BLOCK_PLAN.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 function-length usage binary-double unsigned based.
linkage section.
01 engine usage pointer.
01 function-text usage pointer.
01 length-address usage pointer.
01 question usage pointer.
01 input-record usage pointer.
01 option-record usage pointer.
01 answer usage pointer.
01 status-record usage pointer.
procedure division using engine function-text length-address question input-record option-record answer status-record.
 set address of function-length to length-address
 set address of tt-n-cobol-AnswerPlan to answer
 set address of tt-n-cobol-AnswerStatus to status-record
 call "TT_PREVIEW" using by value engine function-text by reference function-length
   by value question input-record option-record
   by reference tt-n-cobol-AnswerPlan tt-n-cobol-AnswerStatus returning return-code
 goback.
end program BLOCK_PLAN.
""")
    source = work / 'blocking.cob'; source.write_text('\n'.join(programs))
    bridge_path = work / 'blocking.so'
    subprocess.run(['cobc', '-b', '-free', '-fstatic-call', '-fno-gen-c-decl-static-call', '-I', str(PACKAGE / 'copybooks'), '-A', '-include ' + str(PACKAGE / 'src/tt_session.h') + ' -I' + str(HEADER.parent) + ' -Wno-incompatible-pointer-types', '-o', str(bridge_path), str(source), str(PACKAGE / 'src/tt_session.c'), str(PACKAGE / 'src/tt_requests_generated.c'), '-Q', str(NATIVE) + ' -Wl,-rpath,' + str(NATIVE.parent)], env=child_env(LC_ALL='C.UTF-8'), check=True)
    runtime = ct.CDLL(ctypes.util.find_library('cob'), mode=ct.RTLD_GLOBAL); runtime.cob_init(0, None)
    native = ct.CDLL(str(NATIVE), mode=ct.RTLD_GLOBAL)
    native.thinkthen_engine_new_with.argtypes = [ct.c_char_p]; native.thinkthen_engine_new_with.restype = ct.c_void_p
    native.thinkthen_engine_free.argtypes = [ct.c_void_p]
    bridge = ct.CDLL(str(bridge_path)); bridge.TT_ANSWER_FREE.argtypes = [ct.c_void_p]
    bridge.TT_VALUE_TEXT.argtypes = [ct.c_void_p, ct.c_void_p, ct.c_void_p]
    backend = Backend(key='tt-cobol-blocking')
    try:
        os.environ.clear(); os.environ.update(child_env(home=home, THINKTHEN_API_KEY='tt-cobol-blocking'))
        settings = {'base_url': f'http://127.0.0.1:{backend.server_port}/generic/v1', 'cache': str(work / 'store'), 'model': 'jev-1.13.0', 'batch': 1, 'max_retries': 0}
        records = Records(PACKAGE)
        settings_address = records.construct('EngineSettings', definitions['EngineSettings'], settings)
        engine_slot = ct.c_void_p()
        bridge.TT_ENGINE_NEW.argtypes = [ct.c_void_p, ct.c_void_p]
        assert bridge.TT_ENGINE_NEW(settings_address, ct.byref(engine_slot)) == 0
        engine = engine_slot.value; assert engine
        records.owners.clear()
        def invoke(function, originals=None, options=None, preview=False, preview_function=None, question_override=None, input_override=None):
            records = Records(PACKAGE)
            question = {'kind': 'definition', 'value': question_override or questions[function]}
            kind = {'find': 'units', 'relate': 'entities'}.get(function, 'records')
            if originals is None: originals = ['John Smith'] if function == 'recognize' else ['yes']
            if function == 'find': originals = ['first', 'second']
            if function == 'relate': originals = [{'kind': 'person', 'name': 'Sam'}, {'kind': 'person', 'name': 'Lee'}]
            items = [{'original': {'kind': 'text', 'text': value} if isinstance(value, str) else {'kind': 'json', 'value': value}} for value in originals]
            values = [engine, records.construct('RequestQuestion', definitions['RequestQuestion'], question), records.construct('RequestInput', definitions['RequestInput'], input_override or {'kind': kind, 'items': items}), records.construct('RequestOptions', definitions['RequestOptions'], options or {})]
            pointers = [ct.c_void_p(v) for v in values]
            answer, status = (Plan() if preview else OUTPUT_TYPES[function]()), Status()
            pointers.extend([ct.c_void_p(ct.addressof(answer)), ct.c_void_p(ct.addressof(status))])
            if preview:
                function_bytes = ct.create_string_buffer((preview_function or function).encode())
                length = ct.c_uint64(len(function_bytes.value))
                pointers = [pointers[0], ct.c_void_p(ct.addressof(function_bytes)), ct.c_void_p(ct.addressof(length)), *pointers[1:]]
            call = getattr(bridge, 'BLOCK_PLAN' if preview else 'BLOCK_' + function.upper()); call.argtypes = [ct.POINTER(ct.c_void_p)] * len(pointers)
            code = call(*(ct.byref(v) for v in pointers))
            records.owners.clear()
            return code, answer, status
        def native_plan(function):
            request = {'schema': 'thinkthen.request/1', 'call': {'function': function, 'question': {'kind': 'definition', 'value': questions[function]}, 'input': {'kind': 'records', 'items': [{'original': {'kind': 'text', 'text': 'yes'}}]}, 'options': {}}}
            data = json.dumps(request).encode()
            native.thinkthen_request_plan_value.argtypes = [ct.c_void_p, ct.c_char_p, ct.c_size_t, ct.c_void_p]
            native.thinkthen_value_view.argtypes = [ct.c_void_p, ct.c_void_p]
            native.thinkthen_value_free.argtypes = [ct.c_void_p]
            owner, view = ct.c_void_p(), ct.c_void_p()
            assert native.thinkthen_request_plan_value(engine, data, len(data), ct.byref(owner)) == 0
            assert native.thinkthen_value_view(owner, ct.byref(view)) == 0
            constants = Records(PACKAGE).constants
            def decode(address):
                value = JsonValue.from_address(address)
                if value.kind == constants['THINKTHEN_COMPLETE_JSON_NULL_V1']: return None
                if value.kind == constants['THINKTHEN_COMPLETE_JSON_BOOLEAN_V1']: return bool(value.data.boolean)
                if value.kind in (constants['THINKTHEN_COMPLETE_JSON_STRING_V1'], constants['THINKTHEN_COMPLETE_JSON_NUMBER_V1']):
                    text = value.data.string if value.kind == constants['THINKTHEN_COMPLETE_JSON_STRING_V1'] else value.data.number
                    text = ct.string_at(text.data, text.len).decode()
                    return text if value.kind == constants['THINKTHEN_COMPLETE_JSON_STRING_V1'] else json.loads(text)
                assert value.kind == constants['THINKTHEN_COMPLETE_JSON_OBJECT_V1']
                entries = (JsonEntry * value.data.object.len).from_address(value.data.object.data)
                return {ct.string_at(e.name.data, e.name.len).decode(): decode(e.value) for e in entries}
            result = decode(view.value); native.thinkthen_value_free(owner)
            return result
        for function in ('decide', 'choose', 'tag', 'score', 'filter', 'rank'):
            before = backend.attempts
            code, plan, status = invoke(function, preview=True)
            assert code == 0 and backend.attempts == before, (function, code)
            expected = native_plan(function)
            actual = {name: getattr(plan, name) for name in ('records', 'requests', 'estimated_bytes', 'largest_request_bytes', 'largest_request_estimated_input_tokens', 'upper_bound')}
            actual['token_estimate_method'] = bytes(plan.token_estimate_method.text[:plan.token_estimate_method.length]).decode()
            actual['estimated_input_tokens'] = {'lower': plan.estimated_input_tokens.lower, 'upper': plan.estimated_input_tokens.upper}
            actual['first_body_utf8'] = bytes(plan.first_body_utf8.text[:plan.first_body_utf8.length]).decode() if plan.first_body_utf8_presence == 2 else None
            assert actual == expected, (function, actual, expected)
            assert set(expected) == set(json.loads((ROOT / 'specification/result.schema.json').read_text(encoding='utf-8'))['$defs']['plan']['properties'])
            assert not status.owner
        before = backend.attempts
        code, plan, status = invoke('decide', preview=True, preview_function='other')
        assert code == 1 and backend.attempts == before
        if status.owner: bridge.TT_ANSWER_FREE(status.owner)
        for function in ('find', 'annotate', 'recognize', 'relate'):
            before = backend.attempts
            code, plan, status = invoke(function, preview=True)
            assert code == 1 and backend.attempts == before, (function, code)
            if status.owner: bridge.TT_ANSWER_FREE(status.owner)
        before = backend.attempts
        code, answer, status = invoke('rank', ['second', 'first', 'third', 'first'])
        try:
            assert code == 0 and answer.owner and answer.count == 4
            indexes = list((ct.c_uint64 * answer.count).from_address(answer.rows))
            assert indexes == [1, 3, 2, 0], indexes
            assert status.facts_present and status.facts.records == 4
            assert backend.attempts - before == status.facts.requests_sent > 0
        finally:
            if answer.owner: bridge.TT_ANSWER_FREE(answer.owner)
            if status.owner and status.owner != answer.owner: bridge.TT_ANSWER_FREE(status.owner)
        for function in questions:
            code, answer, status = invoke(function, options={'attempts': True})
            assert code == 0, (function, code, bytes(status.message.text[:status.message.length]))
            assert answer.owner and answer.count, (function, answer.count)
            assert status.facts_present and (status.facts.requests_sent + status.facts.cache_answers) > 0, function
            if function == 'decide':
                assert Decision.from_address(answer.rows).value == 1
                assert status.facts.attempts.presence == 2 and status.facts.attempts.value.len == 1
                attempt = record_type('thinkthen_cobol_AnswerAttempt').from_address(status.facts.attempts.value.data)
                assert attempt.request_sha256.length == 64 and attempt.sdk_request_id
                assert status.facts.call_id
                assert record_type('thinkthen_cobol_AnswerCallId').from_address(status.facts.call_id).value.length == 64
            if function == 'choose': assert Choice.from_address(answer.rows).value.text[:1] == b'a'
            if function == 'tag':
                tags = Table.from_address(answer.rows); assert tags.count == 2
                assert Text.from_address(tags.rows).text[:1] == b'a'
                assert Text.from_address(tags.rows + ct.sizeof(Text)).text[:1] == b'b'
            if function == 'score': assert abs(ct.c_double.from_address(answer.rows).value - .1) < 1e-12
            if function == 'rank': assert ct.c_uint64.from_address(answer.rows).value == 0
            if function == 'find': assert Find.from_address(answer.rows).present == 1
            if function == 'annotate': assert Annotation.from_address(answer.rows).boolean == 1
            if function == 'recognize': assert Recognition.from_address(answer.rows).text.text[:10] == b'John Smith'
            if function == 'relate': assert Relation.from_address(answer.rows).source.text[:3] == b'Sam'
            if function == 'filter':
                value = ct.c_void_p.from_address(answer.rows).value; output = ct.create_string_buffer(8192); length = ct.c_uint64()
                assert bridge.TT_VALUE_TEXT(value, output, ct.byref(length)) == 0
                assert output.raw[:length.value] == b'yes'
            bridge.TT_ANSWER_FREE(answer.owner)
        for original, present, value in [('no', 1, 0), ('unsure', 0, 0)]:
            code, answer, status = invoke('decide', [original], {'threshold': '0.4:0.6'} if original == 'unsure' else {}); assert code == 0
            row = Decision.from_address(answer.rows); assert (row.present, row.value) == (present, value)
            bridge.TT_ANSWER_FREE(answer.owner)
        before = backend.attempts
        code, answer, status = invoke('decide', ['yes']); assert code == 0
        assert status.facts.cache_answers == 1 and backend.attempts == before
        bridge.TT_ANSWER_FREE(answer.owner)
        for evidence in ['x' * 8193, 'backend-failure']:
            before = backend.attempts
            code, answer, status = invoke('decide', [evidence])
            assert code != 0 and not answer.owner, (code, answer.owner)
            if len(evidence) > 8192: assert code == 1002 and backend.attempts == before
            else: assert status.failure_kind == 2 and status.facts_present
            if status.owner: bridge.TT_ANSWER_FREE(status.owner)
        before = backend.attempts
        code, answer, status = invoke('decide', question_override=questions['choose'])
        assert code == 1 and backend.attempts == before
        if status.owner: bridge.TT_ANSWER_FREE(status.owner)
        before = backend.attempts
        code, answer, status = invoke('decide', input_override={'kind': 'source', 'source': {'paths': [str(work / 'missing.txt')]}})
        assert code != 0 and backend.attempts == before
        if status.owner: bridge.TT_ANSWER_FREE(status.owner)
        saved_engine = engine
        settings = {'cache': False, 'batch': 1, 'model': 'jev-1.13.0', 'max_retries': 0, 'base_url': f'http://127.0.0.1:{backend.server_port}/generic/v1'}
        engine = native.thinkthen_engine_new_with(json.dumps(settings).encode()); assert engine
        code, answer, status = invoke('decide', ['backend-failure'], options={'attempts': True})
        native.thinkthen_engine_free(engine); engine = saved_engine
        assert code == 2 and status.facts_present and status.facts.requests_sent == 1 and status.details
        detail = record_type('thinkthen_cobol_AnswerError').from_address(status.details)
        assert detail.retryable == 0 and detail.message.length > 0
        assert record_type('thinkthen_cobol_AnswerFailureKind').from_address(detail.kind).kind == 2
        if detail.stopped:
            stopped = record_type('thinkthen_cobol_AnswerStopped').from_address(detail.stopped)
            if stopped.status.presence == 2: assert stopped.status.value == 401
        assert status.facts.attempts.presence == 2 and status.facts.attempts.value.len == 1
        assert record_type('thinkthen_cobol_AnswerAttempt').from_address(status.facts.attempts.value.data).request_sha256.length == 64
        bridge.TT_ANSWER_FREE(status.owner)
        fixture = next(case for case in json.loads((ROOT / 'conformance/cases.json').read_text(encoding='utf-8'))['cases'] if case['id'] == '42-recognize-C01-relations')
        fixture_backend = FixtureBackend(BACKEND_BINARY, child_env(home=work / 'recognition-home'))
        try:
            os.environ['THINKTHEN_API_KEY'] = 'sk-conformance-loopback'
            settings = {'cache': False, 'batch': 1, 'model': 'jev-1.13.0', 'max_retries': 0, 'base_url': f'http://127.0.0.1:{fixture_backend.port}/case/42-recognize-C01-relations/v1'}
            engine = native.thinkthen_engine_new_with(json.dumps(settings).encode()); assert engine
            code, answer, status = invoke('recognize', [fixture['text']], question_override=fixture['question'])
            assert code == 0, (code, status.message.text[:status.message.length])
            native.thinkthen_engine_free(engine); engine = saved_engine
            expected = fixture['expect']['success']['answers'][0]['bare']
            assert answer.count == len(expected['entities']) and answer.edge_count == len(expected['relations'])
            def entity(row):
                return {'text': bytes(row.text.text[:row.text.length]).decode(), 'kind': bytes(row.kind.text[:row.kind.length]).decode(), 'start': row.start, 'end': row.end, 'length': row.length, 'strength': row.probability}
            for index, value in enumerate(expected['entities']):
                assert entity(Recognition.from_address(answer.rows + index * ct.sizeof(Recognition))) == value
            for index, value in enumerate(expected['relations']):
                row = RecognitionEdge.from_address(answer.edges + index * ct.sizeof(RecognitionEdge))
                assert entity(row.source) == value['source'] and entity(row.target) == value['target']
                assert row.relation.text[:row.relation.length].decode() == value['relation'] and abs(row.probability - value['probability']) < 1e-12
            bridge.TT_ANSWER_FREE(answer.owner)
            engine = native.thinkthen_engine_new_with(json.dumps(settings).encode()); assert engine
            boundary_question = {key: value for key, value in fixture['question'].items() if key != 'relation_threshold'}
            boundary_question['recognize'] = {key: value for key, value in fixture['question']['recognize'].items() if key != 'relations'}
            code, answer, status = invoke('recognize', [fixture['text']], options={'mode': 'boundary_only'}, question_override=boundary_question)
            assert code == 0 and answer.count > 0 and answer.edge_count == 0, (code, answer.count, answer.edge_count, bytes(status.message.text[:status.message.length]))
            native.thinkthen_engine_free(engine); engine = saved_engine
            for index in range(answer.count):
                row = Recognition.from_address(answer.rows + index * ct.sizeof(Recognition))
                assert row.proposal == 1 and row.mode == 1 and row.length == row.end - row.start
                assert row.text.text[:row.text.length].decode() == fixture['text'][row.start:row.end]
                assert 0 <= row.probability <= 1 and row.extensions
            bridge.TT_ANSWER_FREE(answer.owner)
            for name in ('17-annotate-mixed', '17-annotate-partial'):
                case = next(case for case in json.loads((ROOT / 'conformance/cases.json').read_text(encoding='utf-8'))['cases'] if case['id'] == name)
                settings['base_url'] = f'http://127.0.0.1:{fixture_backend.port}/case/{name}/v1'
                engine = native.thinkthen_engine_new_with(json.dumps(settings).encode()); assert engine
                code, answer, status = invoke('annotate', [case['exchanges'][0]['evidence']], question_override=case.get('question', case.get('question_set')))
                assert code == 0
                native.thinkthen_engine_free(engine); engine = saved_engine
                expected = {value['name']: value['bare'] for value in case['expect']['success']['answers']}
                assert answer.count == len(expected)
                constants = Records(PACKAGE).constants
                for index in range(answer.count):
                    row = Annotation.from_address(answer.rows + index * ct.sizeof(Annotation))
                    want = expected[row.name.text[:row.name.length].decode()]
                    if want is None: assert row.kind == constants['THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL_V1']
                    elif type(want) is bool: assert row.boolean == want
                    elif isinstance(want, str): assert row.text.text[:row.text.length].decode() == want
                    elif isinstance(want, (float, int)): assert abs(row.number - want) < 1e-12
                    elif isinstance(want, list):
                        assert row.tag_count == len(want)
                        assert [Text.from_address(row.tags + i * ct.sizeof(Text)).text[:Text.from_address(row.tags + i * ct.sizeof(Text)).length].decode() for i in range(row.tag_count)] == want
                    else:
                        failure = want['failed']
                        assert row.failure_kind == constants['THINKTHEN_COMPLETE_FAILURE_FIELD_KIND_' + failure['kind'].upper() + '_V1']
                        assert row.failure_cause == constants['THINKTHEN_COMPLETE_FAILURE_CAUSE_' + failure['cause'].upper() + '_V1']
                bridge.TT_ANSWER_FREE(answer.owner)
        finally:
            fixture_backend.close()
            os.environ['THINKTHEN_API_KEY'] = 'tt-cobol-blocking'
            engine = saved_engine
        page = (PACKAGE / 'README.md').read_text(encoding='utf-8')
        assert '`examples/blocking.cob`' in page, 'installed guide must identify the blocking caller'
        example = (PACKAGE / 'examples/blocking.cob').read_bytes()
        assert example == (ROOT / 'libraries/cobol/examples/blocking.cob').read_bytes(), 'installed blocking example differs from its owned source'
        executable = work / 'public-example'
        subprocess.run(['cobc', '-x', '-free', '-fstatic-call', '-fno-gen-c-decl-static-call', '-I', str(PACKAGE / 'copybooks'), '-A', '-include ' + str(PACKAGE / 'src/tt_session.h') + ' -I' + str(HEADER.parent) + ' -Wno-incompatible-pointer-types', '-o', str(executable), str(PACKAGE / 'examples/blocking.cob'), str(PACKAGE / 'src/tt_session.c'), str(PACKAGE / 'src/tt_requests_generated.c'), '-Q', str(NATIVE) + ' -Wl,-rpath,' + str(NATIVE.parent)], env=child_env(LC_ALL='C.UTF-8'), check=True)
        decide_file, choose_file = work / 'decide.json', work / 'choose.json'
        decide_file.write_text(json.dumps(questions['decide'])); choose_file.write_text(json.dumps(questions['choose']))
        env = child_env(home=work / 'public-home', THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.server_port}/generic/v1', THINKTHEN_API_KEY='tt-cobol-blocking', TT_SESSION_QUESTION_FILE=str(decide_file), TT_PUBLIC_CHOOSE_FILE=str(choose_file), TT_SESSION_EVIDENCE='yes', TT_SESSION_EXPECT_FAILURE='N')
        result = subprocess.run([str(executable)], env=env, capture_output=True, text=True, timeout=5)
        assert result.returncode == 0 and result.stdout.splitlines() == ['00000000000000000001', 'a'], (result.returncode, result.stdout, result.stderr)
        decide_file.write_text(json.dumps({'decide': 'Refund?', 'threshold': '0.4:0.6'}))
        result = subprocess.run([str(executable)], env=dict(env, TT_SESSION_EVIDENCE='unsure'), capture_output=True, text=True, timeout=5)
        assert result.returncode == 0 and result.stdout.splitlines() == ['a'], (result.returncode, result.stdout, result.stderr)
        decide_file.write_text(json.dumps(questions['decide']))
        choose_file.write_text(json.dumps({**questions['choose'], 'threshold': .95}))
        result = subprocess.run([str(executable)], env=env, capture_output=True, text=True, timeout=5)
        assert result.returncode == 0 and result.stdout.splitlines() == ['00000000000000000001'], (result.returncode, result.stdout, result.stderr)
        native.thinkthen_engine_free(engine)
    finally:
        backend.close()
print('COBOL ten compiled blocking calls, typed answers, owned tables, false/null, cache, overflow zero sends and failure facts passed')
