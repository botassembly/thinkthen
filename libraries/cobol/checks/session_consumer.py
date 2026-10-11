"""Test-only transport through compiled COBOL named calls and owned native packets."""
import ctypes as ct
import ctypes.util
import json
import os
from pathlib import Path
import sys
import time
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
native.thinkthen_session_try_read.argtypes = [ct.c_void_p,ct.POINTER(ct.c_uint32),ct.POINTER(ct.c_void_p)]
native.thinkthen_session_finish.argtypes = [ct.c_void_p,ct.c_void_p,ct.c_size_t]
native.thinkthen_session_result_json.argtypes = [ct.c_void_p,ct.POINTER(ct.c_void_p),ct.POINTER(ct.c_size_t)]
native.thinkthen_session_error_message.restype = ct.c_char_p
bridge.TT_SESSION_NEXT.argtypes = [ct.c_void_p,ct.POINTER(ct.c_uint32),ct.POINTER(ct.c_void_p)]
engine = native.thinkthen_engine_new_with(sys.argv[2].encode())
if not engine:
    raise RuntimeError('fixture engine constructor refused')
value = json.loads(Path(sys.argv[1]).read_text())
records = Records(Path(os.environ['TT_SESSION_PACKAGE']))
verb = value['verb']
request = {'function':verb, 'question':value['question'], 'input':value['input'], 'options':value['options']}
address = ct.c_void_p(records.construct('RequestCall_' + verb, definitions['RequestCall_' + verb], request))
session = ct.c_void_p()
start = getattr(bridge, 'COBOL_' + verb.upper())
start.argtypes = [ct.POINTER(ct.c_void_p)] * 3
owner = ct.c_void_p(engine)
code = start(ct.byref(owner),ct.byref(address),ct.byref(session))
packets, owners = [], []
try:
    if code:
        print(json.dumps({'admission':{'code':code,'message':native.thinkthen_session_error_message().decode()}}))
    else:
        # Request records and caller buffers no longer own the admitted session.
        records.owners.clear()
        if value['cancel']: native.thinkthen_session_cancel(session)
        for item in value.get('feed_items', []):
            address = ct.c_void_p(records.construct('RequestSessionDescriptor', definitions['RequestSessionDescriptor'], {'item':item}))
            while True:
                state = ct.c_uint32()
                assert bridge.COBOL_PUSH(ct.byref(session),ct.byref(address),ct.byref(state)) == 0
                if state.value != 1: break
                status, packet = ct.c_uint32(), ct.c_void_p()
                assert native.thinkthen_session_try_read(session,ct.byref(status),ct.byref(packet)) == 0
                if packet.value: owners.append(packet.value)
                else: time.sleep(0.001)
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
            packets.append(json.loads(ct.string_at(data,length.value)))
        print(json.dumps({'packets':packets},ensure_ascii=False))
finally:
    for packet in owners: native.thinkthen_session_result_free(packet)
    if session: native.thinkthen_session_free(session)
    if engine: native.thinkthen_engine_free(engine)
