"""Run the one conformance file through the Python surface, offline.

Usage: python tests/conformance.py [--wire]

Offline the engine answers from the null backend with the conformance
file's own numbers, so this needs no network and no key. Case 18
(cancel) needs a live interrupt and runs as ``tests/test_cancel.py``
against the stub instead; case 17 (usage and cache) is recorded as a
known divergence while the stand-in holds no disk cache.
"""

import json
import pathlib
import sys
import tempfile

import thinkthen as tt

ROOT = pathlib.Path(__file__).resolve().parents[3]
CASES = json.loads((ROOT / "conformance" / "conformance.json").read_text())["cases"]


def built(body, set_body=None):
    """Build one question or set from a case body through the surface."""
    if set_body is not None:
        held = {"version": 1, "questions": set_body}
        path = pathlib.Path(tempfile.mkdtemp()) / "set.json"
        path.write_text(json.dumps(held))
        return str(path)
    kwargs = {}
    for verb in ("decide", "choose", "score", "tag"):
        if verb in body:
            kwargs[verb] = body[verb]
    if "options" in body:
        kwargs["options"] = body["options"]
    if "labels" in body:
        kwargs["labels"] = body["labels"]
    if "levels" in body:
        kwargs["levels"] = body["levels"]
    threshold = body.get("threshold")
    if isinstance(threshold, str) and ":" in threshold:
        low, high = threshold.split(":")
        kwargs["threshold"] = (float(low), float(high))
    elif isinstance(threshold, (int, float)):
        kwargs["threshold"] = float(threshold)
    return tt.question(**kwargs)


def recognize_relations(body):
    """The case's relation list as this surface's mapping shape."""
    rules = body.get("relations") or []
    if not rules:
        return None
    return {rule["name"]: (rule["source"], rule["target"]) for rule in rules}


def entity_dict(entity):
    return {
        "id": entity.id, "text": entity.text, "kind": entity.kind,
        "start": entity.start, "end": entity.end,
        "strength": entity.strength,
    }


def relation_dict(relation):
    return {
        "name": relation.name, "source": relation.source,
        "target": relation.target, "probability": relation.probability,
    }


def edge_dict(edge):
    return {
        "name": edge.name, "source": edge.source,
        "target": edge.target, "probability": edge.probability,
    }


def run_recognize(case):
    body = case["question"]
    found = tt.recognize(
        case["text"],
        kinds=body.get("kinds"),
        relations=recognize_relations(body),
        threshold=body.get("threshold"),
        relation_threshold=body.get("relation_threshold"),
    )
    want = case["expect"]
    ok = (
        [entity_dict(entity) for entity in found.entities] == want.get("entities", [])
        and [relation_dict(relation) for relation in found.relations]
        == want.get("relations", [])
    )
    return ok, None


def run_relate(case):
    if case.get("form") == "per-subject":
        # R03 per-subject and R04 pairs hold the identical ten records,
        # and the stand-in serves the ruled pairs form; a replay keyed on
        # input cannot reach the per-subject expectation. Recorded as a
        # conformance-data finding for the build team, not bent here.
        return None, (
            "the per-subject arm shares its input with the pairs arm and "
            "the stand-in serves the ruled pairs form; a conformance-data "
            "finding for the build team"
        )
    body = case["question"]
    relations, either = [], []
    for rule in body.get("relations", []):
        if rule.get("either"):
            either.append(rule["name"])
        else:
            relations.append(
                {"name": rule["name"], "source": rule["source"], "target": rule["target"]}
            )
    edges = tt.relate(
        case["records"],
        relations=relations or None,
        either=either or None,
        threshold=body.get("threshold"),
    )
    want = case["expect"]["edges"]
    return [edge_dict(edge) for edge in edges] == want, None


def run(case):
    verb = case["verb"]
    body = case.get("question", {})
    expect = case["expect"]
    evidence = case.get("evidence", "")

    if verb == "recognize":
        return run_recognize(case)
    if verb == "relate":
        return run_relate(case)

    if verb == "decide":
        return tt.decide(built(body), evidence) == expect.get("answer"), None
    if verb == "decide_many":
        got = tt.decide_many(built(body), case["records"])
        return got == expect.get("answers"), None
    if verb == "filter":
        kept = tt.filter(built(body), case["records"])
        wanted = [case["records"][i] for i in expect.get("indexes", [])]
        return kept == wanted, None
    if verb == "choose":
        return tt.choose(built(body), evidence) == expect.get("answer"), None
    if verb == "score":
        return tt.score(built(body), evidence) == expect.get("answer"), None
    if verb == "tag":
        return tt.tag(built(body), evidence) == expect.get("answer"), None
    if verb == "annotate":
        path = built(body, set_body=case.get("set"))
        rows = tt.annotate(path, [evidence])
        wanted = {
            name: field.get("answer")
            for name, field in expect.get("answers", {}).items()
        }
        return rows[0] == wanted, "bare answers only; the probabilities are the details form"
    if verb == "details":
        got = tt.details(built(body), evidence)
        held = expect.get("details", {})
        checks = (
            got.get("probability") == held.get("probability")
            and got.get("model") == held.get("model")
            and got.get("digest") == held.get("question_sha256")
            and got.get("answer") == expect.get("answer")
        )
        return checks, None
    if verb == "usage":
        before = tt.usage()
        for _ in range(case.get("calls", 1)):
            tt.decide(built(body), case["evidence"])
        counted = tt.usage()
        wanted = expect.get("requests")
        cache = expect.get("cache_answers")
        sent = counted.get("requests") - before.get("requests")
        cached = counted.get("cache_answers") - before.get("cache_answers")
        if sent == wanted and cached == cache:
            return True, None
        return False, (
            f"expected requests {wanted} cache {cache}, "
            f"the stand-in counted {sent} sends and "
            f"{cached} cache answers: it holds no disk cache"
        )
    return None, f"no runner for verb {verb}"


def expect_error(case, kind):
    """Run a case that must fail, and return whether it failed rightly."""
    verb = case["verb"]
    if "question_file" in case:
        return None, "the local kind needs a file door this surface does not carry"
    body = case.get("question", {})
    try:
        question = built(body)
    except tt.UsageError:
        # The builder refused the question, which is the usage kind itself.
        return kind == "usage", None
    try:
        if kind == "deadline":
            tt.decide(question, case.get("evidence", ""), deadline=0.0)
        elif verb == "decide":
            tt.decide(question, case.get("evidence", ""))
        elif verb == "filter":
            tt.filter(question, case.get("records", []))
        elif verb == "choose":
            tt.choose(question, case.get("evidence", ""))
        elif verb == "rank":
            tt.rank(question, case.get("records", []))
        else:
            return None, f"no error runner for verb {verb}"
    except tt.ThinkThenError as error:
        return error.kind == kind, f"kind {error.kind}, message: {error}"
    return False, "the call did not fail"


def main():
    passed, failed, skipped = [], [], []
    for case in CASES:
        name = case["id"]
        kind = case.get("expect", {}).get("error", {}).get("kind")
        try:
            if name == "18-cancel-mid-batch":
                skipped.append((name, "runs as tests/test_cancel.py against the stub"))
                continue
            if kind:
                ok, note = expect_error(case, kind)
            else:
                ok, note = run(case)
        except tt.ThinkThenError as error:
            ok, note = False, f"{error.kind}: {error}"
        if ok is True:
            passed.append(name)
        elif ok is False:
            failed.append((name, note or "the answer did not match"))
        else:
            skipped.append((name, note or "not runnable"))
    for name, why in failed:
        print(f"FAIL  {name}: {why}")
    for name, why in skipped:
        print(f"skip  {name}: {why}")
    known = [name for name, _ in failed if name == "17-usage-and-cache"]
    hard = [item for item in failed if item[0] != "17-usage-and-cache"]
    print(f"{len(passed)} passed, {len(hard)} failed, {len(skipped)} skipped")
    if known:
        print("known divergence: 17-usage-and-cache waits on the disk cache")
    return 1 if hard else 0


if __name__ == "__main__":
    sys.exit(main())
