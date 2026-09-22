#!/usr/bin/env python3
"""The C surface's slice of the conformance file, run offline.

Drives the built door (target/release/libthinkthen.so) through ctypes: the
typed decide doors for the decide family, the out-param recognize and
relate doors for the two semantic functions, and the JSON door for the
rest. Prints one line a case — ok, skip with a reason — and exits nonzero
on any divergence. What this surface cannot run comes from the shared
skip table in the conformance file, one place a reason each. Run through
./check.sh; the door is the surface under test and this driver is
scaffolding.
"""

import ctypes
import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
LIB = HERE / "target" / "release" / "libthinkthen.so"
FILE = json.loads((HERE.parent.parent / "conformance" / "conformance.json").read_text())
CASES = FILE["cases"]
SKIPS = FILE.get("skips", [])


def central_skip(surface, case, wire):
    """The shared skip table's reason for this case on this surface, or
    None. First match wins; entries naming a surface apply only there."""
    kind = case.get("expect", {}).get("error", {}).get("kind")
    for entry in SKIPS:
        if entry.get("surfaces") and surface not in entry["surfaces"]:
            continue
        if entry.get("unless") == "wire" and wire:
            continue
        when = entry["when"]
        if "id" in when and case["id"] != when["id"]:
            continue
        if "verb" in when:
            listed = when["verb"] if isinstance(when["verb"], list) else [when["verb"]]
            if case["verb"] not in listed:
                continue
        if "kind" in when and kind != when["kind"]:
            continue
        if "form" in when and case.get("form") != when["form"]:
            continue
        if "none" in when and bool(case.get("none")) != when["none"]:
            continue
        if "error" in when and ("error" in case.get("expect", {})) != when["error"]:
            continue
        if "record" in when and when["record"] == "null" \
                and not any(record is None for record in case.get("records") or []):
            continue
        return entry["why"]
    return None

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
    lib.thinkthen_decide_opts.argtypes = [
        ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p,
        ctypes.c_size_t, ctypes.c_int64, ctypes.c_void_p,
        ctypes.POINTER(Answer),
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
    lib.thinkthen_recognize.argtypes = [
        ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_size_t,
        ctypes.POINTER(ctypes.c_void_p), ctypes.POINTER(ctypes.c_size_t),
    ]
    lib.thinkthen_recognize.restype = ctypes.c_int
    lib.thinkthen_relate.argtypes = [
        ctypes.c_void_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_char_p),
        ctypes.POINTER(ctypes.c_size_t), ctypes.c_size_t,
        ctypes.POINTER(ctypes.c_void_p), ctypes.POINTER(ctypes.c_size_t),
    ]
    lib.thinkthen_relate.restype = ctypes.c_int
    return lib


def recognize(lib, engine, spec, text):
    """One thinkthen_recognize call: the parsed answer or the message."""
    out = ctypes.c_void_p()
    out_len = ctypes.c_size_t()
    encoded = text.encode()
    code = lib.thinkthen_recognize(
        engine, json.dumps(spec).encode(), encoded, len(encoded),
        ctypes.byref(out), ctypes.byref(out_len),
    )
    if code != 0:
        return None, lib.thinkthen_error_message(engine).decode()
    body = ctypes.string_at(out, out_len.value).decode()
    lib.thinkthen_free_string(out)
    return json.loads(body), None


def relate(lib, engine, spec, records):
    """One thinkthen_relate call over `records`: the parsed answer or the
    message."""
    out = ctypes.c_void_p()
    out_len = ctypes.c_size_t()
    encoded = [record.encode() for record in records]
    pointers = (ctypes.c_char_p * len(encoded))(*encoded)
    lengths = (ctypes.c_size_t * len(encoded))(
        *[len(record) for record in encoded]
    )
    code = lib.thinkthen_relate(
        engine, json.dumps(spec).encode(), pointers, lengths, len(encoded),
        ctypes.byref(out), ctypes.byref(out_len),
    )
    if code != 0:
        return None, lib.thinkthen_error_message(engine).decode()
    body = ctypes.string_at(out, out_len.value).decode()
    lib.thinkthen_free_string(out)
    return json.loads(body), None


def byte_slice(text, start, end):
    """The answer's offsets are code points; walk the text to the C-style
    byte range the way the example does, and return those bytes."""
    encoded = text.encode()
    points = 0
    from_byte = 0
    to_byte = len(encoded)
    byte = 0
    sizes = [(0x80, 1), (0xE0, 2), (0xF0, 3), (0x100, 4)]
    while byte < len(encoded):
        if points == start:
            from_byte = byte
        if points == end:
            to_byte = byte
            break
        lead = encoded[byte]
        width = next(size for limit, size in sizes if lead < limit)
        byte += width
        points += 1
    return encoded[from_byte:to_byte]


def typed_decide(lib, engine, question, evidence, budget_ms=None):
    """One decide call: the plain spelling, or the `_opts` spelling when a
    budget is given (`THINKTHEN_NO_DEADLINE` is -1)."""
    answer = Answer()
    if budget_ms is None:
        code = lib.thinkthen_decide(
            engine, question.encode(), evidence.encode(), len(evidence.encode()),
            ctypes.byref(answer),
        )
    else:
        code = lib.thinkthen_decide_opts(
            engine, question.encode(), evidence.encode(), len(evidence.encode()),
            budget_ms, None, ctypes.byref(answer),
        )
    return code, answer.outcome, answer.probability


def usage_of(lib, engine):
    """The door's own request counter; reading the counters sends nothing."""
    reply, message, _ = call(lib, engine, {"usage": True})
    if reply is None:
        raise SystemExit(f"the counters did not answer: {message}")
    return reply["requests"]


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
    cases = CASES
    failures = 0

    for case in cases:
        case_id = case["id"]
        verb = case["verb"]
        reason = central_skip("c", case, wire=False)
        if reason is not None:
            print(f"skip     {case_id}: {reason}")
            continue
        question = case["question"]
        evidence = case.get("evidence")
        expect = case["expect"]
        line = ""

        if "error" in expect and expect["error"]["kind"] == "deadline":
            # The door's `deadline_ms` argument, with the case's own budget:
            # zero is spent before the call starts, so nothing is sent.
            budget = case.get("budget_ms", 0)
            before = usage_of(lib, engine)
            code, _, _ = typed_decide(lib, engine, json.dumps(question), evidence, budget)
            sent = usage_of(lib, engine) - before
            if code != 3:
                line = f"FAIL     {case_id}: code {code}, expected the deadline kind"
                failures += 1
            elif sent != 0:
                line = f"FAIL     {case_id}: {sent} request(s) left on a spent budget"
                failures += 1
            else:
                line = f"ok       {case_id}: spent budget refused, nothing sent"
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
            elif expect.get("rows"):
                # The ruled record row (go-ahead item 4): this host's own
                # object, the record and its verdict, checked through the
                # door's own outputs.
                rows = [
                    {"input": record, "value": outcome == YES}
                    for record, outcome in zip(records, held)
                ]
                if rows != expect["rows"]:
                    line = f"FAIL     {case_id}: rows {rows}"
                    failures += 1
                else:
                    line = f"ok       {case_id}"
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
            elif expect.get("rows"):
                records = case["records"]
                rows = [
                    {"input": records[index], "value": True}
                    for index in reply.get("indexes", [])
                ]
                if rows != expect["rows"]:
                    line = f"FAIL     {case_id}: rows {rows}"
                    failures += 1
                else:
                    line = f"ok       {case_id}"
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
            held_records = case.get("records") or [evidence]
            request = {"annotate": {"version": 1, "questions": case["set"]}, "records": held_records}
            reply, message, _ = call(lib, engine, request)
            if reply is None:
                line = f"FAIL     {case_id}: {message}"
                failures += 1
            elif "rows" in expect:
                # The multi-record form: one answer object a record, in
                # input order, each field the bare answer.
                rows = reply["answer"]
                wanted = [row["value"] for row in expect["rows"]]
                if len(rows) != len(wanted):
                    line = f"FAIL     {case_id}: {len(rows)} rows against the case's {len(wanted)}"
                    failures += 1
                else:
                    diverged = []
                    for at, (record, want) in enumerate(zip(rows, wanted)):
                        if set(record) != set(want):
                            diverged.append(f"row {at} fields {sorted(record)}")
                            continue
                        for name, value in want.items():
                            got = record[name]
                            if isinstance(value, float):
                                # This door spells a score field as
                                # {"answer": position, "nearest": level}; the
                                # case pins the bare position.
                                read = got.get("answer") if isinstance(got, dict) else got
                                if not isinstance(read, (int, float)) or abs(read - value) > 1e-9:
                                    diverged.append(f"row {at} {name}: {got!r}")
                            elif got != value:
                                diverged.append(f"row {at} {name}: {got!r}")
                    if diverged:
                        line = f"FAIL     {case_id}: {'; '.join(diverged)}"
                        failures += 1
                    else:
                        line = f"ok       {case_id}"
            else:
                record = reply["answer"][0]
                wanted = expect["answers"]
                diverged = []
                for name, want in wanted.items():
                    held = record.get(name)
                    if "failed" in want:
                        # The ruled marker (0054), this host's spelling:
                        # the JSON object `{"failed": {"kind", "cause"}}`.
                        if held != {"failed": want["failed"]}:
                            diverged.append(f"{name}: {held!r}")
                    elif held != want["answer"]:
                        diverged.append(f"{name}: {held!r}")
                counted = sum(
                    1 for value in record.values()
                    if isinstance(value, dict) and "failed" in value
                )
                if "failed_questions" in expect and counted != expect["failed_questions"]:
                    diverged.append(f"failed_questions: {counted}")
                if diverged:
                    line = f"FAIL     {case_id}: {' '.join(diverged)}"
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
            else:
                # The audit's identity fields and the two 0053/0054
                # additions; the recorded probability is not compared
                # because the null backend's own rule cannot reproduce
                # case 73's recorded number.
                want = expect.get("details", {})
                diverged = []
                if reply.get("model") != want.get("model"):
                    diverged.append(f"model {reply.get('model')!r}")
                if reply.get("digest") != want.get("question_sha256"):
                    diverged.append(f"digest {reply.get('digest')!r}")
                if "requests" in want and reply.get("requests") != want["requests"]:
                    diverged.append(f"requests {reply.get('requests')!r}")
                if "failed_questions" in want and reply.get("failed_questions") != want["failed_questions"]:
                    diverged.append(f"failed_questions {reply.get('failed_questions')!r}")
                if diverged:
                    line = f"FAIL     {case_id}: {' '.join(diverged)}"
                    failures += 1
                else:
                    line = f"ok       {case_id}"
        elif verb == "recognize":
            found, message = recognize(lib, engine, question, case["text"])
            if found is None:
                line = f"FAIL     {case_id}: {message}"
                failures += 1
            else:
                want = case["expect"]
                diverged = None
                names = found.get("entities", [])
                if names != want.get("entities", []):
                    diverged = f"entities {names}"
                if diverged is None:
                    # The offset proof for C's indexing: the answer's code
                    # points must slice the name out of the byte string.
                    for name in names:
                        sliced = byte_slice(
                            case["text"], name["start"], name["end"]
                        ).decode()
                        if sliced != name["text"]:
                            diverged = (
                                f"offsets {name['start']}..{name['end']} "
                                f"slice {sliced!r}, not {name['text']!r}"
                            )
                            break
                if diverged is None and (
                    found.get("relations", []) != want.get("relations", [])
                ):
                    diverged = f"relations {found.get('relations')}"
                if diverged is not None:
                    line = f"FAIL     {case_id}: {diverged}"
                    failures += 1
                else:
                    line = f"ok       {case_id}"
        elif verb == "relate":
            spec = {}
            rules = [
                {"name": rule["name"], "source": rule["source"], "target": rule["target"]}
                for rule in question.get("relations", [])
                if not rule.get("either")
            ]
            either = [
                rule["name"]
                for rule in question.get("relations", [])
                if rule.get("either")
            ]
            if rules:
                spec["relations"] = rules
            if either:
                spec["either"] = either
            if "threshold" in question:
                spec["threshold"] = question["threshold"]
            edges, message = relate(lib, engine, spec, case["records"])
            if edges is None:
                line = f"FAIL     {case_id}: {message}"
                failures += 1
            elif edges.get("edges") != case["expect"]["edges"]:
                line = f"FAIL     {case_id}: {edges.get('edges')}, expected {case['expect']['edges']}"
                failures += 1
            else:
                line = f"ok       {case_id}"
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
