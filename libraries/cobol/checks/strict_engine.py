"""Engine-level strict in-flight proof, not a COBOL concurrency claim."""
import ctypes as C
import json
import os
from pathlib import Path
import threading
import time

class Answer(C.Structure):
    _fields_ = [("outcome", C.c_int), ("probability", C.c_double)]

lib = C.CDLL(str(Path(__file__).resolve().parent / "target/libthinkthen.so.0"))
p = C.c_void_p
lib.thinkthen_engine_new.restype = p
lib.thinkthen_engine_free.argtypes = [p]
lib.thinkthen_cancel_token_new.restype = p
lib.thinkthen_cancel_token_free.argtypes = [p]
lib.thinkthen_cancel.argtypes = [p]
lib.thinkthen_decide_opts.argtypes = [p, C.c_char_p, C.c_char_p, C.c_size_t, C.c_int64, p, C.POINTER(Answer)]
lib.thinkthen_decide_opts.restype = C.c_int
lib.thinkthen_decide_many_opts.argtypes = [p, C.c_char_p, C.POINTER(C.c_char_p), C.POINTER(C.c_size_t), C.c_size_t, C.c_int64, p, C.POINTER(Answer)]
lib.thinkthen_decide_many_opts.restype = C.c_int
lib.thinkthen_error_code.argtypes = [p]
lib.thinkthen_error_code.restype = C.c_int
barrier = Path(os.environ["TT_BARRIER_DIR"])
engine = lib.thinkthen_engine_new()
assert engine
proof = []

def wait(state):
    marker = barrier / ("arrived-" + state)
    end = time.monotonic() + 5
    while not marker.exists() and time.monotonic() < end:
        time.sleep(.005)
    assert marker.exists(), f"no counted arrival for {state}"

def held(state, bulk=False):
    token = lib.thinkthen_cancel_token_new()
    assert token
    answer = Answer(71, -0.125)
    result = {}
    def call():
        if bulk:
            texts = (C.c_char_p * 1)(state.encode())
            sizes = (C.c_size_t * 1)(len(state))
            code = lib.thinkthen_decide_many_opts(engine, b"Is it?", texts, sizes, 1, -1, token, C.byref(answer))
        else:
            code = lib.thinkthen_decide_opts(engine, b"Is it?", state.encode(), len(state), -1, token, C.byref(answer))
        result.update(code=code, error=lib.thinkthen_error_code(engine))
    thread = threading.Thread(target=call)
    thread.start()
    try:
        wait(state)
        lib.thinkthen_cancel(token)
        lib.thinkthen_cancel(token)
        (barrier / ("release-" + state)).touch()
        thread.join(timeout=6)
        assert not thread.is_alive(), f"held {state} did not finish"
        assert result == {"code": 5, "error": 5}, (state, result)
        assert (answer.outcome, answer.probability) == (71, -0.125), (state, answer.outcome, answer.probability)
        proof.append({"state": state, "code": 5, "out_unchanged": True, "double_fire": True})
    finally:
        if thread.is_alive():
            (barrier / ("release-" + state)).touch()
            thread.join(timeout=8)
        assert not thread.is_alive()
        lib.thinkthen_cancel_token_free(token)

try:
    held("hold-contract")
    fresh = lib.thinkthen_cancel_token_new()
    assert fresh
    out = Answer(71, -0.125)
    code = lib.thinkthen_decide_opts(engine, b"Is it?", b"recovery-contract", len(b"recovery-contract"), -1, fresh, C.byref(out))
    assert code == 0 and (out.outcome, out.probability) == (1, .9), (code, out.outcome, out.probability)
    lib.thinkthen_cancel_token_free(fresh)
    proof.append({"state": "recovery-contract", "code": 0, "fresh_token": True})
    held("hold-bulk-contract", bulk=True)
    print("STRICT_ENGINE_CANCEL_PASS " + json.dumps(proof))
finally:
    lib.thinkthen_engine_free(engine)
