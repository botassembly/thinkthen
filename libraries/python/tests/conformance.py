"""Every shared case in conformance/cases.json through ``import thinkthen``.

Run as ``python tests/conformance.py PORT`` with a conformance backend on
that loopback port. Each success case runs on its own case arm, with each
expected request digest recomputed for the URL the backend served. Every
selected case ends as pass, fail, or not run with its reason, and the three
counts sum to the selected count. With no ID file the runner selects all 55.
A case is not run only by a rule the library
cannot meet, never by its id. Each typed, ``decide_many``, and ``annotate``
case also runs over a Polars column or frame, and must give the list form's
values.
"""

import hashlib
import json
import math
import os
import pathlib
import sys
import tempfile

import polars as pl

import thinkthen as tt

from keys import question_keys

CASES = pathlib.Path(__file__).resolve().parents[3] / "conformance" / "cases.json"
CANONICAL = "https://api.typesafe.ai/v1/systemone"
PARTS = {}


def not_run(case):
    """Why the library cannot run this case, or ``None``."""
    if case["expect"].get("error", {}).get("kind") == "defect":
        return "no outside boundary reaches an internal invariant failure"
    if case["verb"] == "annotate" and "record" in case and len(case.get("exchanges", [])) > 1:
        return "one record's groups were recorded as separate requests, and ADR 0111 section 5 packs them into one"
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


def detailed(details, expected, base):
    wanted = expected["details"]
    for field in ("probability", "probabilities", "level"):
        if field in wanted["answer"]:
            same(field, details["answer"].get(field), wanted["answer"][field])
    same("confidence", details["answer"].get("confidence", "absent"), wanted["answer"].get("confidence", "absent"))
    for field in ("model", "question_sha256", "usage", "requests_sent", "cached"):
        same(field, details["meta"].get(field, "absent"), wanted.get(field, "absent"))
    # A keyed request stands for the list of its keys, so a row reads the flattened list.
    requests = [key for held in wanted.get("requests", []) for key in (held if isinstance(held, list) else [held])]
    same("requests", details["meta"].get("requests", "absent"), requests if "requests" in wanted else "absent")
    same("url", details["meta"]["url"], base + "/systemone")


def single(engine, question, text, success, base):
    expected = success["answers"][0]
    details = engine.details(question, text).value
    same("bare", details["value"], expected["bare"])
    detailed(details, expected, base)
    typed = getattr(engine, question.kind)(question, text).value
    same("typed", typed, expected["bare"])
    same("column", getattr(engine, question.kind)(question, pl.Series([text])).value.to_list(),
         [expected["bare"]])
    counters = success.get("counters")
    if counters:
        cached = tt.Engine(base_url=base, cache=tempfile.mkdtemp())
        before = cached.usage()
        for _ in range(counters["calls"]):
            cached.details(question, text).value
        after = cached.usage()
        moved = {"calls": counters["calls"],
                 "requests": after["requests_sent"] - before["requests_sent"],
                 "cache_answers": after["cache_answers"] - before["cache_answers"]}
        same("counters", moved, counters)


def annotated(engine, case, texts, success):
    records = engine.annotate(case["question_set"], texts, batch=1).value
    failed = 0
    for expected in success["answers"]:
        value = records[0 if "record" in case else expected["exchange"]][expected["name"]]
        failed += isinstance(value, dict) and "failed" in value
        same("bare", value, expected["bare"])
    same("failed", failed, success.get("failed_questions", 0))
    frame = engine.annotate(case["question_set"], pl.DataFrame({"record": texts}), on="record", batch=1).value
    for got, listed in zip(frame.drop("record").to_dicts(), records):
        markers = {name: value if isinstance(value, dict) and "failed" in value else None
                   for name, value in listed.items()}
        same("frame failed", got["failed"], markers if any(markers.values()) else None)
        for name, value in listed.items():
            same("frame", got[name], None if markers[name] else value)


def entity(one):
    return {"name": one.name, "kind": one.kind}


def recognized(one):
    return {"text": one.text, "start": one.start, "end": one.end, "length": one.length,
            "kind": one.kind, "strength": one.strength}


def succeeded(port, case):
    base = f"http://127.0.0.1:{port}/case/{case['id']}/v1"
    served = base + "/systemone"
    exchanges = case.get("exchanges", [])
    # Every row lists question keys, by ADR 0111.
    renamed = {digest(CANONICAL, e["request"]): question_keys(served, e["request"], e["response"]["model"]) for e in exchanges}
    success = swap(case["expect"]["success"], renamed)
    texts = [exchange["evidence"] for exchange in exchanges]
    engine = tt.Engine(base_url=base, cache=False)
    verb, kind = case["verb"], success["kind"]
    if verb == "recognize":
        found = engine.recognize(case["text"], case["question"]).value
        result = {"entities": [recognized(one) for one in found.entities]}
        if found.relations is not None:
            result["relations"] = [
                {"relation": one.relation, "source": recognized(one.source),
                 "target": recognized(one.target), "probability": one.probability,
                 **({"either": True} if one.either else {})}
                for one in found.relations]
        return same("result", result, success["answers"][0]["bare"])
    if verb == "relate":
        edges = engine.relate(case["entities"], case["question"]).value
        result = [{"relation": edge.relation, "source": entity(edge.source),
                   "target": entity(edge.target), "probability": edge.probability,
                   **({"either": True} if edge.either else {})}
                  for edge in edges]
        return same("result", result, success["answers"][0]["bare"])
    if verb == "annotate":
        records = [json.dumps(case["record"])] if "record" in case else texts
        return annotated(engine, case, records, success)
    if verb == "find":
        spec = case["question"]
        found = engine.find(spec["find"], spec["units"], none=spec["none"]).value
        operation = success["operation"]
        picked = [row for row in operation["probabilities"] if row["index"] == operation["selected"]]
        want = None if operation["selected"] is None else {
            "index": operation["selected"], "unit": spec["units"][operation["selected"]],
            "probability": picked[0]["probability"]}
        return same("found", found, want)
    if verb == "rank":
        ranked = engine.rank(case["question"]["decide"], texts, batch=1).value
        rows = [{"index": row["index"], "probability": row["probability"]} for row in ranked]
        return same("ranking", rows, success["operation"]["ranking"])
    question = asked(case)
    if kind == "filter":
        kept = engine.filter(question, texts, batch=1).value
        return same("indexes", [texts.index(text) for text in kept],
                    success["operation"]["indexes"])
    if kind == "decide_many":
        answers = engine.decide(question, texts, batch=1).value
        same("column", engine.decide(question, pl.Series(texts), batch=1).value.to_list(), answers)
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
        "usage": lambda: tt.Engine(base_url=generic).decide(asked(case), "   ").value,
        "backend": lambda: tt.Engine(base_url=f"http://127.0.0.1:{port}/arm/refuse/v1",
                                     cache=False).decide(asked(case), text).value,
        "cancelled": lambda: tt.Engine(base_url=generic).decide(asked(case), text, token=token).value,
        "deadline": lambda: tt.Engine(base_url=generic).decide(asked(case), text, deadline_ms=0).value,
    }
    if case["verb"] == "rank":
        calls["usage"] = lambda: tt.rank(text, ["one", "two"]).value
    elif "threshold" in case["question"]:
        calls["usage"] = lambda: asked(case)
    if case.get("question_form") == "file":
        (folder / "question.json").write_text(json.dumps(case["question"]))
        calls["local"] = lambda: tt.question(file=folder / "question.json")
    else:
        (folder / "cache").write_text("not a folder")
        calls["local"] = lambda: tt.Engine(base_url=generic, cache=folder / "cache").decide(
            asked(case), text).value
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
    cases = document["cases"]
    ids = [case["id"] for case in cases]
    if len(ids) != len(set(ids)) or len(ids) != document["case_count"]:
        raise ValueError("the canonical conformance cases need unique IDs and their declared count")
    selected = os.environ.get("THINKTHEN_CONFORMANCE_IDS")
    if selected is not None:
        path = pathlib.Path(selected)
        if not path.is_absolute():
            raise ValueError("THINKTHEN_CONFORMANCE_IDS must be an absolute path")
        wanted = [line.strip() for line in path.read_text().splitlines()
                  if line.strip() and not line.lstrip().startswith("#")]
        if len(wanted) != len(set(wanted)) or not wanted:
            raise ValueError("the routine conformance IDs must be nonempty and unique")
        missing = set(wanted) - set(ids)
        if missing:
            raise ValueError(f"unknown routine conformance IDs: {', '.join(sorted(missing))}")
        wanted_ids = set(wanted)
        cases = [case for case in cases if case["id"] in wanted_ids]
    passed, failed, skipped = [], [], []
    for case in cases:
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
    print(f"conformance: total={document['case_count']} selected={len(cases)} "
          f"pass={len(passed)} fail={len(failed)} not_run={len(skipped)} "
          f"unselected={document['case_count'] - len(cases)}")
    return 0 if not failed and total == len(cases) else 1


if __name__ == "__main__":
    os.environ.setdefault("THINKTHEN_BASE_URL", "http://127.0.0.1:9/unused/v1")
    sys.exit(main(sys.argv[1]))
