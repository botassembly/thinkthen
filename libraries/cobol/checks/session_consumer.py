"""Test-only transport through compiled COBOL named calls and owned native packets."""
import ctypes as ct
import ctypes.util
import json
import os
from pathlib import Path
import sys
from session_records import Records, definitions

native = ct.CDLL(os.environ['TT_SESSION_NATIVE'], mode=ct.RTLD_GLOBAL)
runtime = ct.CDLL(ctypes.util.find_library('cob'), mode=ct.RTLD_GLOBAL)
runtime.cob_init(0, None)
bridge = ct.CDLL(os.environ['TT_SESSION_BRIDGE'])
native.thinkthen_engine_new_with.argtypes = [ct.c_char_p]
native.thinkthen_engine_new_with.restype = ct.c_void_p
for name in ('thinkthen_session_cancel','thinkthen_session_free','thinkthen_engine_free','thinkthen_session_result_free'):
    function = getattr(native, name); function.argtypes = [ct.c_void_p]; function.restype = None
bridge.COBOL_PUSH.argtypes = [ct.POINTER(ct.c_void_p),ct.POINTER(ct.c_void_p),ct.POINTER(ct.c_uint32)]
native.thinkthen_session_finish.argtypes = [ct.c_void_p,ct.c_void_p,ct.c_size_t]
native.thinkthen_session_result_json.argtypes = [ct.c_void_p,ct.POINTER(ct.c_void_p),ct.POINTER(ct.c_size_t)]
native.thinkthen_session_error_message.restype = ct.c_char_p
bridge.TT_SESSION_NEXT.argtypes = [ct.c_void_p,ct.POINTER(ct.c_uint32),ct.POINTER(ct.c_void_p)]
engine = native.thinkthen_engine_new_with(sys.argv[2].encode())
if not engine:
    native.thinkthen_error_message.restype = ct.c_char_p
    print(json.dumps({'admission': {'code': native.thinkthen_error_code(None),
        'message': native.thinkthen_error_message(None).decode()}}))
    raise SystemExit(0)
value = json.loads(Path(sys.argv[1]).read_text())
records = Records(Path(os.environ['TT_SESSION_PACKAGE']))
records.builder_bridge = bridge
verb = value['verb']
if value.get('call_result_contract'):
    from owned_reader import record_types, Reader
    from templates.host_answers import call_results
    package = Path(os.environ['TT_SESSION_PACKAGE'])
    record = record_types(package, package / 'src/thinkthen.h')
    reader = Reader(bridge, record)
    answer = record('thinkthen_cobol_Answer' + verb.title() + 'Result')()
    status = record('thinkthen_cobol_AnswerStatus')()
    pointers = [ct.c_void_p(engine)]
    for name, schema in [('question', 'RequestQuestion'), ('input', 'RequestInput'), ('options', 'RequestOptions')]:
        pointers.append(ct.c_void_p(records.construct(schema, definitions[schema], value[name])))
    pointers += [ct.c_void_p(ct.addressof(answer)), ct.c_void_p(ct.addressof(status))]
    call = getattr(bridge, 'COBOL_BLOCK_' + verb.upper()); call.argtypes = [ct.POINTER(ct.c_void_p)] * len(pointers)
    bridge.TT_ANSWER_FREE.argtypes = [ct.c_void_p]
    try:
        code = call(*(ct.byref(pointer) for pointer in pointers))
        assert code == 0, ('public-call-result', code, reader.text(status.message))
        contracts = call_results(reader.definitions)
        receipts = reader.decode(contracts[verb]['receipt_payload'], answer.results, 'receipt')
        bare = reader.json_value(answer.value.first.value) if verb == 'decide' else reader.text(answer.value.first.value)
        probability = answer.probability.first.value if answer.probability.first.present else None
        reader.prefix = 'Answer'
        facts = reader.definition('completeFacts', answer.facts)
        payload = {'call_result': {'value': bare, 'probability': probability, 'results': receipts, 'facts': facts},
                   'packets': [{'kind': 'aggregate', 'function': verb, 'value': receipts}, {'kind': 'terminal', 'facts': facts}]}
        print(json.dumps(payload, ensure_ascii=False))
    finally:
        if status.owner: bridge.TT_ANSWER_FREE(status.owner)
        native.thinkthen_engine_free(engine)
    raise SystemExit(0)
request = {'function':verb, 'question':value['question'], 'input':value['input'], 'options':value['options']}
address = ct.c_void_p(records.construct('RequestCall', definitions['RequestCall'], request))
session = ct.c_void_p()
start = getattr(bridge, 'COBOL_' + verb.upper())
start.argtypes = [ct.POINTER(ct.c_void_p)] * 3
owner = ct.c_void_p(engine)
code = start(ct.byref(owner),ct.byref(address),ct.byref(session))
packets, owners = [], []
try:
    if code:
        assert not session, 'refused request created a session'
        print(json.dumps({'admission':{'code':code,'message':native.thinkthen_session_error_message().decode()}}))
    else:
        # Request records and caller buffers no longer own the admitted session.
        records.owners.clear()
        if value['cancel']: native.thinkthen_session_cancel(session)
        # Complete intake before reading any packet.
        for item in value.get('feed_items', []):
            address = ct.c_void_p(records.construct('RequestSessionDescriptor', definitions['RequestSessionDescriptor'], {'item':item}))
            state = ct.c_uint32()
            assert bridge.COBOL_PUSH(ct.byref(session),ct.byref(address),ct.byref(state)) == 0
            if state.value == 2: break
            assert state.value == 0
        assert native.thinkthen_session_finish(session,None,0) == 0
        if value['held_cancel']:
            assert sys.stdin.read(1) == '!'
            native.thinkthen_session_cancel(session)
            print('cancel-fired',flush=True)
        while True:
            status, packet = ct.c_uint32(), ct.c_void_p()
            assert bridge.TT_SESSION_NEXT(session,ct.byref(status),ct.byref(packet)) == 0
            if status.value == 2: break
            assert status.value == 0 and packet.value
            owners.append(packet.value)
        native.thinkthen_session_free(session); session = ct.c_void_p()
        native.thinkthen_engine_free(engine); engine = None
        # Private fixture projection uses the retained canonical packet transport.
        # Ordinary COBOL callers read the generated typed graph demonstrated by
        # examples/session.cob and the installed ABI/retention checks.
        for packet in owners:
            data, length = ct.c_void_p(), ct.c_size_t()
            assert native.thinkthen_session_result_json(packet,ct.byref(data),ct.byref(length)) == 0
            packet = json.loads(ct.string_at(data,length.value))
            if packet['kind'] == 'aggregate' and packet['function'] == 'rank':
                rows = packet['value']
                assert [row['value'] for row in rows] == list(range(1, len(rows) + 1))
                assert len({row['index'] for row in rows}) == len(rows)
            packets.append(packet)
        print(json.dumps({'packets':packets},ensure_ascii=False))
finally:
    for packet in owners: native.thinkthen_session_result_free(packet)
    if session: native.thinkthen_session_free(session)
    if engine: native.thinkthen_engine_free(engine)
