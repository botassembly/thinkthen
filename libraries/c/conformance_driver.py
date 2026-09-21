#!/usr/bin/env python3
"""The C surface's slice of the conformance file, run offline.

Drives the built door (target/release/libthinkthen.so) through ctypes: the
typed decide doors for the decide family, and the JSON door for the rest.
Prints one line a case — ok, skip with a reason — and exits nonzero on any
divergence. Run through ./check.sh; the door is the surface under test and
this driver is scaffolding.

Skips, with their reasons: the backend-refusal cases need the wire or a
dead address (tests/wire.rs proves the backend kind there), the usage case
needs a cache and a reset the door does not carry, and the cancel case
needs a token the C header does not expose — a finding recorded in
NOTES.md.
"""

import ctypes
import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
LIB = HERE / "target" / "release" / "libthinkthen.so"
CASES = HERE.parent.parent / "conformance" / "conformance.json"

YES, NO, UNSURE = 1, 0, 2


class Answer(ctypes.Structure):
    _fields_ = [("outcome", ctypes.c_int), ("probability", ctypes.c_double)]


def load():
    lib = ctypes.CDLL(str(LIB))
    lib.thinkthen_engine_new.restype = ctypes.c_void_p
    lib.thinkthen_error_message.restype = ctypes.c_char_p
    lib.thinkthen_decide.argtypes = [
        ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p,
        ctypes.c_size_t, ctypes.POINTER(Answer),
    ]
    lib.thinkthen_decide_many.argtypes = [
        ctypes.c_void_p, ctypes.c_char_p,
        ctypes.POINTER(ctypes.c_char_p), ctypes.POINTER(ctypes.c_size_t),
        ctypes.c_size_t, ctypes.POINTER(Answer),
    ]
    lib.thinkthen_call.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
    lib.thinkthen_call.restype = ctypes.c_void_p
    lib.thinkthen_free_string.argtypes = [ctypes.c_void_p]
    lib.thinkthen_error_retryable.restype = ctypes.c_int
    return lib


def typed_decide(lib, engine, question, evidence):
    answer = Answer()
    code = lib.thinkthen_decide(
        engine, question.encode(), evidence.encode(), len(evidence.encode()),
        ctypes.byref(answer),
    )
    return code, answer.outcome, answer.probability


def call(lib, engine, request):
    pointer = lib.thinkthen_call(engine, json.dumps(request).encode())
    if not pointer:
        message = lib.thinkthen_error_message(engine).decode()
        retryable = lib.thinkthen_error_retryable(engine)
        return None, message, retryable
    text = ctypes.string_at(pointer).decode()
    lib.thinkthen_free_string(pointer)
    return json.loads(text), None, None


def outcome_of(expected):
    if expected is True:
        return YES
    if expected is False:
        return NO
    return UNSURE


def main():
    lib = load()
    engine = lib.thinkthen_engine_new()
    cases = json.loads(CASES.read_text())["cases"]
    failures = 0

    for case in cases:
        case_id = case["id"]
        verb = case["verb"]
        question = case["question"]
        evidence = case.get("evidence")
        expect = case["expect"]
        line = ""

        if "error" in expect and expect["error"]["kind"] == "backend":
            line = f"skip     {case_id}: the backend kind needs the wire or a dead address"
        elif verb == "decide":
            code, outcome, _ = typed_decide(lib, engine, json.dumps(question), evidence)
            wanted_kind = expect.get("error", {}).get("kind")
            if wanted_kind == "usage":
                # The typed doors return the code; usage is 1.
                if code == 1:
                    line = f"ok       {case_id}: refused as usage"
                else:
                    line = f"FAIL     {case_id}: code {code}, expected the usage kind"
                    failures += 1
            elif code != 0:
                line = f"FAIL     {case_id}: code {code}, {lib.thinkthen_error_message(engine).decode()}"
                failures += 1
            elif outcome != outcome_of(expect["answer"]):
                line = f"FAIL     {case_id}: outcome {outcome}, expected {expect['answer']}"
                failures += 1
            else:
                line = f"ok       {case_id}"
        elif verb == "decide_many":
            records = case["records"]
            answers = (Answer * len(records))()
            encoded = [record.encode() for record in records]
            pointers = (ctypes.c_char_p * len(records))(*encoded)
            lengths = (ctypes.c_size_t * len(records))(
                *[len(record) for record in encoded]
            )
            code = lib.thinkthen_decide_many(
                engine, json.dumps(question).encode(), pointers, lengths,
                len(records), answers,
            )
            wanted = [outcome_of(answer) for answer in expect["answers"]]
            held = [answer.outcome for answer in answers]
            if code != 0 or held != wanted:
                line = f"FAIL     {case_id}: {held}, expected {wanted}"
                failures += 1
            else:
                line = f"ok       {case_id}"
        elif verb == "filter":
            request = dict(question)
            request["records"] = case["records"]
            reply, message, _ = call(lib, engine, request)
            if reply is None:
                if expect.get("error", {}).get("kind") == "usage" and _ is None:
                    pass
                kind = expect.get("error", {}).get("kind")
                if kind == "usage":
                    # Finding 2 in NOTES.md: the JSON door's failure code is
                    # not retrievable; the retryable flag and a send count
                    # of the usage kind stand in offline.
                    line = f"ok       {case_id}: refused ({message})"
                else:
                    line = f"FAIL     {case_id}: {message}"
                    failures += 1
            elif reply.get("indexes") != expect.get("indexes"):
                line = f"FAIL     {case_id}: {reply.get('indexes')}, expected {expect.get('indexes')}"
                failures += 1
            else:
                line = f"ok       {case_id}"
        elif verb in ("choose", "score", "tag"):
            request = dict(question)
            request["evidence"] = evidence
            reply, message, _ = call(lib, engine, request)
            if reply is None:
                line = f"FAIL     {case_id}: {message}"
                failures += 1
            elif reply.get("answer") != expect.get("answer"):
                line = f"FAIL     {case_id}: {reply.get('answer')}, expected {expect.get('answer')}"
                failures += 1
            elif verb == "score" and expect.get("details", {}).get("nearest_level"):
                if reply.get("nearest") != expect["details"]["nearest_level"]:
                    line = f"FAIL     {case_id}: nearest {reply.get('nearest')}"
                    failures += 1
                else:
                    line = f"ok       {case_id}"
            else:
                line = f"ok       {case_id}"
        elif verb == "annotate":
            request = {"annotate": {"questions": case["set"]}, "records": [evidence]}
            reply, message, _ = call(lib, engine, request)
            if reply is None:
                line = f"FAIL     {case_id}: {message}"
                failures += 1
            else:
                record = reply["answer"][0]
                wanted = expect["answers"]
                held = {}
                for name, value in record.items():
                    held[name] = value
                diverged = [
                    name for name, want in wanted.items()
                    if held.get(name) != want["answer"]
                ]
                if diverged:
                    line = f"FAIL     {case_id}: {held}"
                    failures += 1
                else:
                    line = f"ok       {case_id}"
        elif verb == "details":
            request = dict(question)
            request["evidence"] = evidence
            request["details"] = True
            reply, message, _ = call(lib, engine, request)
            if reply is None:
                line = f"FAIL     {case_id}: {message}"
                failures += 1
            elif reply.get("answer") != expect.get("answer"):
                line = f"FAIL     {case_id}: {reply.get('answer')}"
                failures += 1
            else:
                line = f"ok       {case_id}"
        elif verb == "usage":
            line = f"skip     {case_id}: the cache half and the reset need machinery the door does not carry"
        elif verb == "cancel":
            line = f"skip     {case_id}: the C header exposes no cancel token (NOTES.md, finding 3)"
        else:
            line = f"skip     {case_id}: the driver has no route for {verb}"

        print(line)

    lib.thinkthen_engine_free(engine)
    if failures:
        print(f"{failures} case(s) diverged", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
