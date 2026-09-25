"""Every shared case in conformance/cases.json through ``import thinkthen``.

Run as ``python tests/conformance.py PORT`` with a conformance backend on
that loopback port. Each success case runs on its own case arm, with each
expected request digest recomputed for the URL the backend served. Every
case ends as pass, fail, or not run with its reason, and the three counts
sum to the file's count. A case is not run only by a rule the library
cannot meet, never by its id.
"""

import hashlib
import json
import math
import os
import pathlib
import sys
import tempfile

import thinkthen as tt

CASES = pathlib.Path(__file__).resolve().parents[3] / "conformance" / "cases.json"
CANONICAL = "https://api.typesafe.ai/v1/systemone"
PARTS = {"true": "true_", "false": "false_"}


def not_run(case):
    """Why the library cannot run this case, or ``None``."""
    if case.get("question", {}).get("none"):
        return "the library's find has no switch for the none option"
    questions = (case.get("question_set") or {}).get("questions", {}).values()
    if any(question.get("on") for question in questions):
        return "a library call reads each record whole and takes no on"
    if case["expect"].get("error", {}).get("kind") == "defect":
        return "no outside boundary reaches an internal invariant failure"
    return None


def digest(url, request):
    return hashlib.sha256(b"systemone\n" + url.encode() + b"\n" + request.encode()).hexdigest()


def swap(value, renamed):
    if isinstance(value, str):
        return renamed.get(value, value)
    if isinstance(value, list):
        return [swap(item, renamed) for item in value]
    if isinstance(value, dict):
        return {name: swap(field, renamed) for name, field in value.items()}
    return value


def close(one, other):
    if isinstance(one, bool) or isinstance(other, bool):
        return one is other
    if isinstance(one, (int, float)) and isinstance(other, (int, float)):
        return math.isclose(one, other, abs_tol=1e-9)
    if isinstance(one, list) and isinstance(other, list):
        return len(one) == len(other) and all(map(close, one, other))
    if isinstance(one, dict) and isinstance(other, dict):
        return one.keys() == other.keys() and all(close(one[k], other[k]) for k in one)
    return one == other


def same(what, actual, expected):
    if not close(actual, expected):
        raise AssertionError(f"{what}: got {actual!r}, expected {expected!r}")


def asked(case):
    return tt.question(**{PARTS.get(name, name): value for name, value in case["question"].items()})


def detailed(details, expected):
    wanted = expected["details"]
    for field in ("probability", "probabilities", "level"):
        if field in wanted["answer"]:
            same(field, details["answer"].get(field), wanted["answer"][field])
    for field in ("model", "question_sha256", "requests"):
        same(field, details["meta"][field], wanted[field])


def single(engine, question, text, success, base):
    expected = success["answers"][0]
    details = engine.details(question, text)
    same("bare", details["value"], expected["bare"])
    detailed(details, expected)
    typed = getattr(engine, question.kind)(question, text)
    same("typed", typed, expected["bare"])
    counters = success.get("counters")
    if counters:
        cached = tt.Engine(base_url=base, cache=tempfile.mkdtemp())
        before = cached.usage()
        for _ in range(counters["calls"]):
            cached.details(question, text)
        after = cached.usage()
        moved = {"calls": counters["calls"],
                 "requests": after["requests_sent"] - before["requests_sent"],
                 "cache_answers": after["cache_answers"] - before["cache_answers"]}
        same("counters", moved, counters)


def annotated(engine, case, texts, success):
    records = engine.annotate(case["question_set"], texts)
    failed = 0
    for expected in success["answers"]:
        value = records[expected["exchange"]][expected["name"]]
        failed += isinstance(value, dict) and "failed" in value
        same("bare", value, expected["bare"])
    same("failed", failed, success.get("failed_questions", 0))


def entity(one, place=True):
    found = {"name": one.name, "kind": one.kind}
    if place:
        found.update(start=one.start, end=one.end, strength=one.strength)
    return found


def succeeded(port, case):
    base = f"http://127.0.0.1:{port}/case/{case['id']}/v1"
    served = base + "/systemone"
    exchanges = case.get("exchanges", [])
    renamed = {digest(CANONICAL, e["request"]): digest(served, e["request"]) for e in exchanges}
    success = swap(case["expect"]["success"], renamed)
    texts = [exchange["evidence"] for exchange in exchanges]
    engine = tt.Engine(base_url=base, cache=False)
    verb, kind = case["verb"], success["kind"]
    if verb == "recognize":
        found = engine.recognize(case["text"], case["question"])
        result = {"entities": [entity(one) for one in found.entities]}
        if found.relations is not None:
            result["relations"] = [
                {"relation": one.relation, "source": entity(one.source),
                 "target": entity(one.target), "probability": one.probability}
                for one in found.relations]
        return same("result", result, success["answers"][0]["bare"])
    if verb == "relate":
        edges = engine.relate(case["entities"], case["question"])
        result = [{"relation": edge.relation, "source": entity(edge.source, False),
                   "target": entity(edge.target, False), "probability": edge.probability}
                  for edge in edges]
        return same("result", result, success["answers"][0]["bare"])
    if verb == "annotate":
        return annotated(engine, case, texts, success)
    if verb == "rank":
        ranked = engine.rank(case["question"]["decide"], texts)
        rows = [{"index": row["index"], "probability": row["probability"]} for row in ranked]
        return same("ranking", rows, success["operation"]["ranking"])
    question = asked(case)
    if kind == "filter":
        kept = engine.filter(question, texts)
        return same("indexes", [texts.index(text) for text in kept],
                    success["operation"]["indexes"])
    if kind == "decide_many":
        answers = engine.decide_many(question, texts)
        return same("bare", answers, [answer["bare"] for answer in success["answers"]])
    return single(engine, question, texts[0], success, base)


def refused(port, case):
    """Each error case through its Python boundary. Only the refusal arm sees a request."""
    generic = f"http://127.0.0.1:{port}/generic/v1"
    text = case["question"].get("decide", "")
    folder = pathlib.Path(tempfile.mkdtemp())
    token = tt.CancelToken()
    token.cancel()
    calls = {
        "usage": lambda: tt.Engine(base_url=generic).decide(asked(case), "   "),
        "backend": lambda: tt.Engine(base_url=f"http://127.0.0.1:{port}/arm/refuse/v1",
                                     cache=False).decide(asked(case), text),
        "cancelled": lambda: tt.Engine(base_url=generic).decide(asked(case), text, token=token),
        "deadline": lambda: tt.Engine(base_url=generic).decide(asked(case), text, deadline=0),
    }
    if case["verb"] == "rank":
        calls["usage"] = lambda: tt.rank(text, ["one", "two"])
    elif "threshold" in case["question"]:
        calls["usage"] = lambda: asked(case)
    if case.get("question_form") == "file":
        (folder / "question.json").write_text(json.dumps(case["question"]))
        calls["local"] = lambda: tt.question(file=folder / "question.json")
    else:
        (folder / "cache").write_text("not a folder")
        calls["local"] = lambda: tt.Engine(base_url=generic, cache=folder / "cache").decide(
            asked(case), text)
    kind = case["expect"]["error"]["kind"]
    try:
        calls[kind]()
    except tt.ThinkThenError as error:
        same("kind", error.kind, kind)
        same("retryable", error.retryable, False)
        if not str(error).strip():
            raise AssertionError("the error names nothing") from error
        return
    raise AssertionError("the case succeeded")


def main(port):
    document = json.loads(CASES.read_text())
    passed, failed, skipped = [], [], []
    for case in document["cases"]:
        reason = not_run(case)
        if reason:
            skipped.append(f"{case['id']}: {reason}")
            continue
        try:
            (refused if "error" in case["expect"] else succeeded)(port, case)
            passed.append(case["id"])
        except Exception as error:
            failed.append(f"{case['id']}: {type(error).__name__}: {error}")
    for line in failed:
        print("FAIL", line)
    for line in skipped:
        print("NOT RUN", line)
    total = len(passed) + len(failed) + len(skipped)
    print(f"conformance: {len(passed)} passed, {len(failed)} failed, {len(skipped)} not run, "
          f"of {document['case_count']}")
    return 0 if not failed and total == document["case_count"] == len(document["cases"]) else 1


if __name__ == "__main__":
    os.environ.setdefault("THINKTHEN_BASE_URL", "http://127.0.0.1:9/unused/v1")
    sys.exit(main(sys.argv[1]))
