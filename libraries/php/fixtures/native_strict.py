"""Engine-level C ABI proof; Python threads are NOT PHP in-flight cancellation capability."""
import ctypes as c
import hashlib
import json
import os
from pathlib import Path
import threading
import time

ROOT=Path(__file__).resolve().parent
libpath=Path(os.environ['TT_LIBRARY'])
lib=c.CDLL(str(libpath))
class Answer(c.Structure):
    _fields_=[('outcome',c.c_int),('probability',c.c_double)]
lib.thinkthen_engine_new.restype=c.c_void_p
lib.thinkthen_engine_free.argtypes=[c.c_void_p]
lib.thinkthen_cancel_token_new.restype=c.c_void_p
lib.thinkthen_cancel.argtypes=[c.c_void_p]
lib.thinkthen_cancel_token_free.argtypes=[c.c_void_p]
lib.thinkthen_decide_opts.argtypes=[c.c_void_p,c.c_char_p,c.c_char_p,c.c_size_t,c.c_int64,c.c_void_p,c.POINTER(Answer)]
lib.thinkthen_decide_opts.restype=c.c_int
lib.thinkthen_decide_many_opts.argtypes=[c.c_void_p,c.c_char_p,c.POINTER(c.c_char_p),c.POINTER(c.c_size_t),c.c_size_t,c.c_int64,c.c_void_p,c.POINTER(Answer)]
lib.thinkthen_decide_many_opts.restype=c.c_int
barrier=Path(os.environ['TT_BARRIER_DIR'])
engine=lib.thinkthen_engine_new()
token=lib.thinkthen_cancel_token_new()
assert engine and token
result={}
def call():
    answer=Answer(123,-1.0)
    text=b'hold-native'
    result['code']=lib.thinkthen_decide_opts(engine,b'Is it?',text,len(text),-1,token,c.byref(answer))
    result['outcome']=answer.outcome
    result['probability']=answer.probability
    result['returned_ns']=time.monotonic_ns()
thread=threading.Thread(target=call)
thread.start()
try:
    limit=time.monotonic()+5
    while not (barrier/'arrived-hold-native').exists():
        if time.monotonic()>limit: raise RuntimeError('strict request never arrived')
        time.sleep(.005)
    lib.thinkthen_cancel(token)
    lib.thinkthen_cancel(token)
    result['cancel_ns']=time.monotonic_ns()
    time.sleep(.25)
    assert thread.is_alive(), 'cancelled request did not drain'
finally:
    result['release_ns']=time.monotonic_ns()
    (barrier/'release-hold-native').touch()
    thread.join(timeout=6)
if thread.is_alive():
    print('live call: cannot safely free token or engine',flush=True)
    os._exit(2)
try:
    assert (result['code'],result['outcome'],result['probability']) == (5,123,-1.0),result
    assert result['cancel_ns']<result['release_ns']<result['returned_ns']
    spent=Answer(123,-1.0)
    code=lib.thinkthen_decide_opts(engine,b'Is it?',b'never-sent',10,-1,token,c.byref(spent))
    assert code==5 and (spent.outcome,spent.probability)==(123,-1.0)
    lib.thinkthen_cancel_token_free(token)
    token=0
    fresh=lib.thinkthen_cancel_token_new()
    assert fresh
    try:
        answer=Answer(123,-1.0)
        text=b'native-recovery'
        code=lib.thinkthen_decide_opts(engine,b'Is it?',text,len(text),-1,fresh,c.byref(answer))
        assert code==0 and (answer.outcome,answer.probability)==(1,0.9)
    finally:
        lib.thinkthen_cancel_token_free(fresh)
    print(json.dumps({'proof':'Python ctypes, engine C ABI only, not PHP in-flight cancellation',
                      'library_sha256':hashlib.sha256(libpath.read_bytes()).hexdigest(),
                      'strict':result,'spent_token_code':5,'fresh_token_code':0,
                      'held_arrival':'hold-native','recovery_arrival':'native-recovery'}),flush=True)
    bulk_token=lib.thinkthen_cancel_token_new()
    assert bulk_token
    bulk={}
    texts=[f'hold-bulk-{i}'.encode() for i in range(1,7)]
    ptrs=(c.c_char_p*6)(*texts)
    lengths=(c.c_size_t*6)(*(len(s) for s in texts))
    outputs=(Answer*6)(*(Answer(123,-1.0) for _ in texts))
    def bulk_call():
        bulk['code']=lib.thinkthen_decide_many_opts(engine,b'Is it?',ptrs,lengths,6,-1,bulk_token,outputs)
        bulk['returned_ns']=time.monotonic_ns()
    worker=threading.Thread(target=bulk_call)
    worker.start()
    try:
        limit=time.monotonic()+5
        while not (barrier/'arrived-hold-bulk-1').exists():
            if time.monotonic()>limit: raise RuntimeError('bulk request never arrived')
            time.sleep(.005)
        lib.thinkthen_cancel(bulk_token)
        lib.thinkthen_cancel(bulk_token)
        bulk['cancel_ns']=time.monotonic_ns()
        time.sleep(.1)
        assert worker.is_alive(), 'bulk did not drain accepted work'
    finally:
        bulk['release_ns']=time.monotonic_ns()
        for i in range(1,7): (barrier/f'release-hold-bulk-{i}').touch()
        worker.join(timeout=6)
    if worker.is_alive():
        print('live bulk call: cannot safely free token or engine',flush=True)
        os._exit(2)
    try:
        assert bulk['code']==5,bulk
        assert all((a.outcome,a.probability)==(123,-1.0) for a in outputs),'bulk wrote partial rows'
        assert bulk['cancel_ns']<bulk['release_ns']<bulk['returned_ns']
        print(json.dumps({'bulk_engine_proof':bulk,'sentinels_intact':True}),flush=True)
    finally:
        lib.thinkthen_cancel_token_free(bulk_token)
    print('STRICT_C_CANCEL_PASS',flush=True)
finally:
    if token: lib.thinkthen_cancel_token_free(token)
    lib.thinkthen_engine_free(engine)
