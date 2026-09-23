"""Run the one conformance file through the Python surface, offline.

Usage: python tests/conformance.py

Offline the engine answers from the null backend with the conformance
file's own numbers, so this needs no network and no key. What this
surface cannot run comes from the shared skip table in the conformance
file, one place a reason each; the live interrupt runs as
tests/test_cancel.py against the stub instead.
"""

import json
import pathlib
import sys
import tempfile

import thinkthen as tt

ROOT = pathlib.Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "conformance"))
from skiptable import lookup_reason  # noqa: E402 - the one skip reader

FILE = json.loads((ROOT / "conformance" / "conformance.json").read_text())
CASES = FILE["cases"]


def central_skip(surface, case, wire):
    """The one reader's decision for this case on this surface (surfaces-
    review-4: this surface used to carry its own copy; the copy and the
    table drifted). `None` runs the case; otherwise (why, as)."""
    expect = case.get("expect", {})
    asked = {
        "kind": expect.get("error", {}).get("kind"),
        "record": "null" if any(r is None for r in case.get("records") or []) else None,
        "none": bool(case.get("none")),
        "error": "error" in expect,
    }
    return lookup_reason(surface, case["id"], asked, wire=wire)


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


def ruled_rows(records, values, expect):
    """The ruled `{"input","value"}` record row (go-ahead item 4) on this
    host's own types: a Python dict a record, checked where the case
    carries one. Case 06's empty list has no rows to check and passes
    through the count its filter assertion already made."""
    if "rows" not in expect:
        return True
    built_rows = [{"input": record, "value": value}
                  for record, value in zip(records, values)]
    return built_rows == expect["rows"]


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
        if got != expect.get("answers"):
            return False, None
        return ruled_rows(case["records"], got, expect), None
    if verb == "filter":
        kept = tt.filter(built(body), case["records"])
        wanted = [case["records"][i] for i in expect.get("indexes", [])]
        if kept != wanted:
            return False, None
        return ruled_rows(kept, [True] * len(kept), expect), None
    if verb == "choose":
        return tt.choose(built(body), evidence) == expect.get("answer"), None
    if verb == "score":
        return tt.score(built(body), evidence) == expect.get("answer"), None
    if verb == "tag":
        return tt.tag(built(body), evidence) == expect.get("answer"), None
    if verb == "rank":
        ranked = tt.rank(built(body), case["records"])
        order = [one["index"] for one in ranked]
        if order != expect.get("ranking"):
            return False, None
        wanted = expect.get("probabilities")
        if wanted is not None:
            by_index = {one["index"]: one["probability"] for one in ranked}
            got = [by_index[place] for place in range(len(case["records"]))]
            if got != wanted:
                return False, f"probabilities {got} against the case's {wanted}"
        return True, None
    if verb == "find":
        found = tt.find(case["question"], case["records"])
        if found is None:
            return False, "find answered None where the case expects a unit"
        return found["index"] == expect.get("answer"), None
    if verb == "annotate":
        path = built(body, set_body=case.get("set"))
        held = case.get("records") or [evidence]
        rows = tt.annotate(path, held)
        if "rows" in expect:
            # The multi-record form: one answer object a record, in input
            # order, each field the bare answer (a score is its position).
            wanted = [row["value"] for row in expect["rows"]]
            if len(rows) != len(wanted):
                return False, f"{len(rows)} rows against the case's {len(wanted)}"
            for at, (got_row, want_row) in enumerate(zip(rows, wanted)):
                if set(got_row) != set(want_row):
                    return False, f"row {at} fields {sorted(got_row)} against {sorted(want_row)}"
                for name, value in want_row.items():
                    got = got_row[name]
                    if isinstance(value, float):
                        if not isinstance(got, (int, float)) or abs(got - value) > 1e-9:
                            return False, f"row {at} {name}: {got!r} against {value!r}"
                    elif got != value:
                        return False, f"row {at} {name}: {got!r} against {value!r}"
            return True, None
        wanted = {}
        for name, field in expect.get("answers", {}).items():
            if "failed" in field:
                # The ruled marker (0054), in this host's own spelling.
                wanted[name] = {"failed": field["failed"]}
            else:
                wanted[name] = field.get("answer")
        ok = rows[0] == wanted
        failed = sum(1 for value in rows[0].values()
                     if isinstance(value, dict) and "failed" in value)
        if failed != expect.get("failed_questions", failed):
            return False, f"{failed} failed fields, the case counts " \
                          f"{expect.get('failed_questions')}"
        return ok, "bare answers only; the probabilities are the details form"
    if verb == "details":
        got = tt.details(built(body), evidence)
        held = expect.get("details", {})
        # The audit's identity fields and the two 0053/0054 additions; the
        # recorded probability is not compared because the null backend's
        # own rule cannot reproduce case 73's recorded number.
        checks = (
            got.get("model") == held.get("model")
            and got.get("digest") == held.get("question_sha256")
        )
        if "requests" in held:
            checks = checks and got.get("requests") == held["requests"]
        if "failed_questions" in held:
            checks = checks and got.get("failed_questions") == held["failed_questions"]
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
        held = central_skip("python", case, wire=False)
        if held is not None:
            why, _as = held
            skipped.append((name, why))
            continue
        kind = case.get("expect", {}).get("error", {}).get("kind")
        try:
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
    print(f"{len(passed)} passed, {len(failed)} failed, {len(skipped)} skipped")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
