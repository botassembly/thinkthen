#!/usr/bin/env python3
"""Validate the conformance file: schema, grammar, wire contract, offline replay.

The cases file is the first argument when one is given, and
`<repo>/conformance/conformance.json` otherwise, resolved from this
script's own place so the checker runs from any directory, the repository
root included. No network. Every expected answer is recomputed from the
recorded exchange under the rules of
specification/{decide,choose,tag,score,question-file}.md, so the file is
self-checking evidence, not prose.
"""
import hashlib
import pathlib
import re
import json
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
DEFAULT_CASES = ROOT / "conformance" / "conformance.json"
REPLAY_TABLE = ROOT / "standin" / "data" / "recognize-replay.json"

KINDS = {"usage", "backend", "local", "cancelled", "deadline", "defect"}
CAUSES = {"missing_answer", "wrong_kind", "missing_probability", "invalid_probability",
          "invalid_distribution", "unexpected_probability"}
VERB_KEYS = {"decide", "choose", "tag", "score"}
CASE_KEYS = {"id", "source", "verb", "question", "question_file", "evidence", "records", "jobs", "set",
             "calls", "cancel_after_replies", "budget_ms", "none", "exchanges", "expect",
             "text", "requests", "pairs", "form", "note", "failed_questions", "rows"}
GRANT = {"decide": {"decide", "true", "false", "threshold", "on", "model"},
         "choose": {"choose", "options", "threshold", "on", "model"},
         "tag": {"tag", "labels", "threshold", "on", "model"},
         "score": {"score", "levels", "on", "model"}}

def canon(question):
    verb = [k for k in VERB_KEYS if k in question][0]
    obj = {"verb": verb, "text": question[verb]}
    if verb == "decide":
        obj.update({k: question[k] for k in ("true", "false") if k in question})
        if "threshold" in question:
            t = question["threshold"]
            obj["threshold"] = t if isinstance(t, str) else float(t)
    elif verb in ("choose", "tag"):
        key = "options" if verb == "choose" else "labels"
        items = question[key]
        obj[key] = {o: None for o in items} if isinstance(items, list) else items
        if "threshold" in question:
            t = question["threshold"]
            obj["threshold"] = t if isinstance(t, str) else float(t)
    else:
        obj["levels"] = question["levels"]
    line = json.dumps(obj, separators=(",", ":"), ensure_ascii=False)
    return line, hashlib.sha256(line.encode("utf-8")).hexdigest()

def find_digest(text, none):
    """The find canonical form thinkthen-core pins: verb, text, none."""
    line = json.dumps({"verb": "find", "text": text, "none": none},
                      separators=(",", ":"), ensure_ascii=False)
    return hashlib.sha256(line.encode("utf-8")).hexdigest()

def unit_id(place):
    return f"u{place + 1:03d}"

def band(threshold):
    low, high = (float(x) for x in threshold.split(":"))
    return low, high

def decide_answer(p, threshold):
    if isinstance(threshold, str):
        low, high = band(threshold)
        if p >= high:
            return True
        if p <= low:
            return False
        return None
    return p >= threshold

errors = []
def check(cond, msg):
    if not cond:
        errors.append(msg)

cases_path = pathlib.Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else DEFAULT_CASES
try:
    data = json.loads(cases_path.read_text())
except OSError as error:
    print(f"cannot read the cases file: {error}")
    sys.exit(1)
check(data["schema"] == "thinkthen.conformance/1", "schema")
check(set(data["error_kinds"]) == KINDS, "error_kinds must be the six")
check(data["case_count"] == len(data["cases"]), "case_count")
ids = [c["id"] for c in data["cases"]]
check(len(set(ids)) == len(ids), "case ids unique")

def check_question(q, verb_of_case, where=""):
    keys = set(q)
    present = keys & VERB_KEYS
    check(len(present) == 1, f"{where}question must hold exactly one verb key, has {sorted(present)}")
    verb = present.pop() if present else None
    # The specification's rule: filter and rank ask a yes/no question, the bulk
    # spelling is decide's, and details, usage, and cancel run over a question.
    decide_cases = {"filter", "rank", "decide_many", "details", "usage", "cancel"}
    if verb_of_case == "annotate":
        pass  # the set's members are validated one by one
    elif verb_of_case in decide_cases:
        check(verb == "decide", f"{where}case verb {verb_of_case} reads a decide question, got {verb}")
    else:
        check(verb == verb_of_case, f"{where}question verb {verb} != case verb {verb_of_case}")
    if verb not in GRANT:
        return None
    check(keys <= GRANT[verb], f"key refused for {verb}: {sorted(keys - GRANT[verb])}")
    text = q.get(verb, "")
    check(isinstance(text, str) and text.strip() != "", f"{where}{verb} text blank or not text")
    if verb == "choose":
        opts = q.get("options")
        check(isinstance(opts, list) and 2 <= len(opts) <= 255, f"{where}choose options 2..255")
        check(len(set(opts)) == len(opts), f"{where}choose options repeated")
    if verb == "tag":
        labels = q.get("labels")
        check(isinstance(labels, list) and 1 <= len(labels) <= 20, f"{where}tag labels 1..20")
    if verb == "score":
        levels = q.get("levels")
        check(isinstance(levels, list) and 2 <= len(levels) <= 10, f"{where}score levels 2..10")
        check("threshold" not in q, f"{where}score takes no threshold")
    if "threshold" in q:
        t = q["threshold"]
        if verb_of_case == "rank":
            check(False, f"{where}any threshold on rank is refused")
        elif isinstance(t, str):
            low, high = band(t)
            check(0 < low < high <= 1, f"{where}band out of range: {t}")
            check(verb == "decide" and verb_of_case not in ("filter",),
                  f"{where}band refused on {verb_of_case or verb}")
        else:
            check(0 < t <= 1, f"{where}cut out of range: {t}")
    return verb

def check_reply_shape(request, reply, status=None, allow_missing=()):
    if status is not None:
        check(status == 422, f"non-200 exchange must be 422, is {status}")
        return
    qs = request["questions"]
    answers = reply["answers"]
    check(bool(reply.get("model")), "reply model present")
    for name, q in qs.items():
        a = answers.get(name)
        if a is None:
            # 0054: a logical question the reply never answered carries the
            # failed marker, and the case's expectation names it.
            check(name in allow_missing, f"reply missing answer {name}")
            continue
        t = q["type"]
        if t == "noul":
            check(set(a) == {"type", "noul"} and 0 <= a["noul"] <= 1, f"{name} noul shape")
        elif t == "choice":
            ps = a.get("probabilities", {})
            check(set(q["criteria"]) == set(ps), f"{name} choice covers options")
            check(abs(sum(ps.values()) - 1) <= 0.01 + 1e-9, f"{name} choice sums to 1")
        elif t == "score":
            ps = a.get("probabilities", {})
            want = {str(i) for i in range(len(q["criteria"]))}
            check(set(ps) == want, f"{name} score keyed by level numbers")
            check(abs(sum(ps.values()) - 1) <= 0.01 + 1e-9, f"{name} score sums to 1")

def case_text(c):
    """The text a recognize case judges or a relate case's records join."""
    if c["verb"] == "recognize":
        return c["text"]
    return "\n".join(c["records"])


def replay_row(data, c):
    """The replay-table row a case answers from; per-subject relate rows
    give way to a pairs row on the same text, matching the stand-in."""
    key = case_text(c).rstrip("\n")
    rows = [
        row
        for row in data["relate"] if row["text"].rstrip("\n") == key or "\n".join(row["records"]).rstrip("\n") == key
    ] if c["verb"] == "relate" else [row for row in data["recognize"] if row["text"] == c["text"]]
    if c["verb"] == "relate":
        rows.sort(key=lambda row: row.get("form") == "per-subject")
    check(bool(rows), f"{c['id']} has no replay-table row")
    return rows[0] if rows else None


def pick_rule(pick):
    return re.sub(r"_(AB|BA)$", "", pick)


def pick_direction(pick, first, second):
    if pick.endswith("_AB"):
        return first, second
    if pick.endswith("_BA"):
        return second, first
    return min(first, second), max(first, second)


def replay_recognize(c, row):
    """Recompute the expected object from the recorded row and the case's ask."""
    q = c["question"]
    entities = [
        e for e in row["entities"]
        if e["kind"] in q["kinds"] and e["confidence"] >= q["threshold"]
    ]
    asked = {rule["name"]: rule for rule in q["relations"]}
    ids = {e["id"] for e in entities}
    relations = []
    for entry in row["pairs"]:
        pick = entry["pick"]
        if pick in ("NO_RELATION", "NONE_OF_THESE"):
            continue
        name = pick_rule(pick)
        if name not in asked:
            continue
        if entry["options"][pick] < q["relation_threshold"]:
            continue
        if entry["pair"][0] not in ids or entry["pair"][1] not in ids:
            continue
        source, target = pick_direction(pick, entry["pair"][0], entry["pair"][1])
        if asked[name].get("either"):
            source, target = min(source, target), max(source, target)
        relations.append(
            {"name": name, "source": source, "target": target,
             "probability": entry["options"][pick]}
        )
    relations.sort(key=lambda edge: (edge["source"], edge["target"], edge["name"]))
    return entities, relations


def replay_relate(c, row):
    """Recompute the expected edges from the recorded row and the ask."""
    q = c["question"]
    asked = {rule["name"]: rule for rule in q["relations"]}
    edges = []
    for entry in row["entries"]:
        pick = entry["pick"]
        if pick in ("NO_RELATION", "NONE_OF_THESE"):
            continue
        name = entry.get("rule") if "pair" not in entry else pick_rule(pick)
        if name not in asked:
            continue
        if entry["options"][pick] < q["threshold"]:
            continue
        if "pair" in entry:
            source, target = pick_direction(pick, entry["pair"][0], entry["pair"][1])
        else:
            source, target = entry["subject"], int(pick[2:])
        edges.append(
            {"name": name, "source": source, "target": target, "probability": entry["options"][pick]}
        )
    edges.sort(key=lambda edge: (edge["source"], edge["target"], edge["name"]))
    return edges


def check_recognize_case(c):
    q = c["question"]
    check(set(q) == {"kinds", "relations", "threshold", "relation_threshold"},
          f"{c['id']} question keys")
    check(isinstance(c["text"], str) and c["text"].strip() != "", f"{c['id']} text")
    check(isinstance(q["kinds"], list) and 1 <= len(q["kinds"]) <= 20, f"{c['id']} kinds 1..20")
    check(len(set(q["kinds"])) == len(q["kinds"]), f"{c['id']} kinds repeat")
    for rule in q["relations"]:
        check(set(rule) <= {"name", "source", "target", "either"},
              f"{c['id']} rule keys: source and target, never from/to")
        check(bool(rule.get("name")), f"{c['id']} rule name")
        for end in (rule.get("source", ""), rule.get("target", "")):
            check(isinstance(end, str) and end != "", f"{c['id']} rule end written")
            check(end == "*" or end in q["kinds"], f"{c['id']} rule end among the kinds")
    for key in ("threshold", "relation_threshold"):
        check(0 <= q[key] <= 1, f"{c['id']} {key} range")
    expect = c["expect"]
    check(set(expect) == {"entities", "relations"}, f"{c['id']} expect keys")
    ids = []
    for entity in expect["entities"]:
        check(set(entity) == {"id", "text", "kind", "start", "end", "strength"},
              f"{c['id']} entity keys carry the ruled `strength`, never confidence")
        check(entity["kind"] in q["kinds"], f"{c['id']} entity kind asked")
        check(entity["start"] < entity["end"] <= len(c["text"]), f"{c['id']} offsets in bounds")
        check(c["text"][entity["start"]:entity["end"]] == entity["text"],
              f"{c['id']} offsets slice the name out")
        check(0 <= entity["strength"] <= 1 and entity["strength"] >= q["threshold"],
              f"{c['id']} strength above the bar")
        ids.append(entity["id"])
    check(ids == list(range(1, len(ids) + 1)), f"{c['id']} entity ids count from 1 in order")
    names = {rule["name"] for rule in q["relations"]}
    last = None
    for relation in expect["relations"]:
        check(set(relation) == {"name", "source", "target", "probability"},
              f"{c['id']} relation keys: source and target, probability")
        check(relation["name"] in names, f"{c['id']} relation name asked")
        check(relation["source"] in ids and relation["target"] in ids, f"{c['id']} relation ends are ids")
        check(relation["probability"] >= q["relation_threshold"], f"{c['id']} relation above the bar")
        order = (relation["source"], relation["target"], relation["name"])
        check(last is None or last <= order, f"{c['id']} relations sorted")
        last = order
    check(isinstance(c["requests"], list), f"{c['id']} requests list")
    for digest in c["requests"]:
        check(isinstance(digest, str) and re.fullmatch(r"[0-9a-f]{64}", digest), f"{c['id']} digest shape")
    check(isinstance(c["pairs"], int) and c["pairs"] >= 0, f"{c['id']} pairs pinned")


def check_relate_case(c):
    q = c["question"]
    check(set(q) == {"relations", "threshold"}, f"{c['id']} question keys")
    check(isinstance(c["records"], list) and 1 <= len(c["records"]) <= 255, f"{c['id']} records 1..255")
    names = set()
    for rule in q["relations"]:
        check(set(rule) <= {"name", "source", "target", "either"},
              f"{c['id']} rule keys: source and target, never from/to")
        check(bool(rule.get("name")), f"{c['id']} rule name")
        for end in (rule.get("source", ""), rule.get("target", "")):
            check(end == "*", f"{c['id']} relate rules are any-kind against these recordings")
        names.add(rule["name"])
    check(0 <= q["threshold"] <= 1, f"{c['id']} threshold range")
    expect = c["expect"]
    check(set(expect) == {"edges"}, f"{c['id']} expect keys")
    last = None
    for edge in expect["edges"]:
        check(set(edge) <= {"name", "source", "target", "probability", "source_kind", "target_kind"},
              f"{c['id']} edge keys: source and target, probability")
        check("from" not in edge and "to" not in edge, f"{c['id']} never spells from and to")
        check(edge["name"] in names, f"{c['id']} edge name asked")
        check(1 <= edge["source"] <= len(c["records"]), f"{c['id']} edge source in range")
        check(1 <= edge["target"] <= len(c["records"]), f"{c['id']} edge target in range")
        check(edge["source"] != edge["target"], f"{c['id']} a record never pairs with itself")
        check(edge["probability"] >= q["threshold"], f"{c['id']} edge above the bar")
        order = (edge["source"], edge["target"], edge["name"])
        check(last is None or last <= order, f"{c['id']} edges sorted")
        last = order
    check(isinstance(c["requests"], list), f"{c['id']} requests list")
    check(isinstance(c["pairs"], int) and c["pairs"] >= 0, f"{c['id']} pairs pinned")
    if len({"\n".join(c["records"])}) and c.get("form") == "per-subject":
        check("note" in c and "per-subject" in c["note"], f"{c['id']} the per-subject skip is noted")


def replay_recognize_and_relate(c):
    """Prove the case against the replay table the stand-in serves."""
    table = json.loads(REPLAY_TABLE.read_text())
    row = replay_row(table, c)
    if row is None:
        return
    if c["verb"] == "recognize":
        entities, relations = replay_recognize(c, row)
        want_entities = [
            {"id": e["id"], "text": e["text"], "kind": e["kind"], "start": e["start"],
             "end": e["end"], "strength": e["confidence"]}
            for e in entities
        ]
        check(want_entities == c["expect"]["entities"], f"{c['id']} entities replay exactly")
        check(relations == c["expect"]["relations"], f"{c['id']} relations replay exactly")
    else:
        if c.get("form") == "per-subject":
            # The pairs recording on the same text is what the stand-in
            # serves; the per-subject case is pinned for the record.
            return
        edges = replay_relate(c, row)
        check(edges == c["expect"]["edges"], f"{c['id']} edges replay exactly")


def replay(c):
    """Recompute the expected answer from question plus recorded exchange."""
    if c["verb"] in ("recognize", "relate"):
        replay_recognize_and_relate(c)
        return
    q = c.get("question")
    if q is None:
        # A question_file case: the named file failed before anything was sent.
        check("question_file" in c, f"{c['id']} has neither question nor question_file")
        check(c["expect"].get("error", {}).get("kind") == "local",
              f"{c['id']} is the local shape")
        return
    verb = "find" if isinstance(q, str) else [k for k in VERB_KEYS if k in q][0]
    exp = c["expect"]
    if "error" in exp:
        check(exp["error"]["kind"] in KINDS, f"{c['id']} kind in six")
        if exp["error"]["kind"] == "usage":
            global errors
            saved = errors
            errors = []
            if isinstance(q, str):
                check(q.strip() == "", f"{c['id']} expected a usage refusal but the text is legal")
            else:
                check_question(q, c["verb"], where=f"{c['id']}: ")
            captured = errors
            errors = saved
            check(len(captured) > 0,
                  f"{c['id']} expected a usage refusal but the question is legal")
        if exp["error"]["kind"] == "backend":
            ex = c["exchanges"][0]
            check(ex.get("status") == 422, f"{c['id']} backend refusal needs the 422 exchange")
        if exp["error"]["kind"] == "deadline":
            check("budget_ms" in c and c["budget_ms"] >= 0, f"{c['id']} deadline states its budget")
            check(len(c["exchanges"]) >= 1,
                  f"{c['id']} deadline case records the exchange the budget cuts off")
        return
    ex = c["exchanges"][0] if c["exchanges"] else None
    if c["verb"] == "usage" and "requests" in exp:
        check(len(c["exchanges"]) == exp["requests"], f"{c['id']} requests counted")
        check(c.get("calls") == exp["requests"] + exp["cache_answers"], f"{c['id']} cache answers")
        return
    if ex is None:
        check(c["verb"] == "filter" and exp.get("indexes") == [] and c["records"] == [],
              f"{c['id']} needs an exchange")
        return
    check("status" not in ex, f"{c['id']} success case carries a refusal exchange")
    if c["verb"] == "find":
        check(len(c["exchanges"]) == 1, f"{c['id']} find is one aggregate request")
        units = c["records"]
        none = bool(c.get("none", False))
        ids = [unit_id(i) for i in range(len(units))]
        want_state = json.dumps([{"id": i, "evidence": e} for i, e in zip(ids, units)],
                                separators=(",", ":"), ensure_ascii=False)
        check(ex["request"]["state"] == want_state, f"{c['id']} state is the units with ids")
        crit = list(ex["request"]["questions"]["q1"]["criteria"])
        check(crit == ids + (["none"] if none else []), f"{c['id']} options are the unit ids")
        ps = ex["reply"]["answers"]["q1"]["probabilities"]
        ordered = [(k, ps[k]) for k in crit]
        highest = max(p for _, p in ordered)
        leaders = [k for k, p in ordered if p == highest]
        det = exp.get("details", {})
        check(det.get("pick") == leaders[0], f"{c['id']} pick is the first leader")
        if "none" in leaders:
            check(exp.get("answer") is None and exp.get("none") is True,
                  f"{c['id']} none leads: the answer is null and none is true")
        else:
            check(exp.get("answer") == ids.index(leaders[0]),
                  f"{c['id']} answer is the winner's input index")
            check(exp.get("none", False) is False, f"{c['id']} a winner means no none flag")
        check(det.get("probabilities") == {k: ps[k] for k in crit},
              f"{c['id']} probabilities in input order")
        if "question_sha256" in det:
            check(det["question_sha256"] == find_digest(q, none), f"{c['id']} find digest")
        check(det.get("model") == ex["reply"]["model"], f"{c['id']} model")
        return
    if c["verb"] == "rank":
        ps = [e["reply"]["answers"]["q1"]["noul"] for e in c["exchanges"]]
        check(len(c["exchanges"]) == len(c["records"]), f"{c['id']} one exchange a record")
        order = sorted(range(len(ps)), key=lambda i: (-ps[i], i))
        check(exp.get("ranking") == order, f"{c['id']} ranking, best first, ties in input order")
        check(exp.get("probabilities") == ps, f"{c['id']} probabilities in input order")
        return
    reply = ex["reply"]
    det = exp.get("details", {})
    _, dig = canon(q)
    if "question_sha256" in det:
        check(det["question_sha256"] == dig, f"{c['id']} digest")
    if "requests" in det:
        # 0053: an ordered array of lowercase recording digests, and no
        # singular form anywhere.
        check(isinstance(det["requests"], list), f"{c['id']} requests is an array")
        check("request" not in det, f"{c['id']} no singular request beside requests")
        for digest in det["requests"]:
            check(isinstance(digest, str) and re.fullmatch("[0-9a-f]{64}", digest),
                  f"{c['id']} request digest is 64 lowercase hex")
        if c["verb"] in ("decide", "details", "choose", "tag", "score", "find"):
            check(len(det["requests"]) == 1, f"{c['id']} one logical request, one element")
    if "failed_questions" in det:
        check(det["failed_questions"] == 0,
              f"{c['id']} a details result carries zero failed questions")
    if verb == "decide" and c["verb"] in ("decide", "details"):
        p = reply["answers"]["q1"]["noul"]
        check(exp["answer"] == decide_answer(p, q.get("threshold", 0.5)), f"{c['id']} decide answer")
        if "probability" in det:
            check(det["probability"] == p, f"{c['id']} probability")
        check(det.get("model") == reply["model"], f"{c['id']} model")
    elif verb == "choose":
        ps = reply["answers"]["q1"]["probabilities"]
        opts = q["options"] if isinstance(q["options"], list) else list(q["options"])
        pick = max(opts, key=lambda o: ps[o])
        value = pick if ps[pick] >= q.get("threshold", 0.5) else None
        check(exp["answer"] == value, f"{c['id']} choose answer")
        check(exp.get("unsure", value is None) == (value is None), f"{c['id']} unsure flag")
        check(det.get("pick") == pick, f"{c['id']} pick")
        check(det.get("probabilities") == {o: ps[o] for o in opts}, f"{c['id']} probabilities order")
    elif verb == "tag":
        labels = q["labels"]
        ps = {labels[i]: reply["answers"][f"q{i+1}"]["noul"] for i in range(len(labels))}
        held = [l for l in labels if ps[l] >= q.get("threshold", 0.5)]
        check(exp["answer"] == held, f"{c['id']} tag labels")
        check(det.get("probabilities") == ps, f"{c['id']} tag probabilities")
    elif verb == "score":
        ps = reply["answers"]["q1"]["probabilities"]
        levels = q["levels"]
        total = sum(ps.values())
        value = round(sum(i * ps[str(i)] for i in range(len(levels))) / total, 12)
        nearest = max(range(len(levels)), key=lambda i: ps[str(i)])
        check(exp["answer"] == value, f"{c['id']} score number {exp['answer']} vs {value}")
        check("nearest_level" in det, f"{c['id']} score details carry the nearest level")
        check(det.get("nearest_level") == levels[nearest], f"{c['id']} nearest level")
        check(isinstance(det.get("nearest_level"), str) and det["nearest_level"] in levels,
              f"{c['id']} nearest level is one of the question's own levels")
        check(det.get("probabilities") == {l: ps[str(i)] for i, l in enumerate(levels)},
              f"{c['id']} score probabilities by level name")
    if c["verb"] == "filter":
        ps = [e["reply"]["answers"]["q1"]["noul"] for e in c["exchanges"]]
        t = q.get("threshold", 0.5)
        kept = [i for i, p in enumerate(ps) if decide_answer(p, t)]
        check(exp["indexes"] == kept, f"{c['id']} filter indexes")
        check(len(c["exchanges"]) == len(c["records"]), f"{c['id']} one exchange a record")
    if c["verb"] == "decide_many":
        ps = [e["reply"]["answers"]["q1"]["noul"] for e in c["exchanges"]]
        t = q.get("threshold", 0.5)
        check(exp["answers"] == [decide_answer(p, t) for p in ps], f"{c['id']} bulk answers")
        check(exp["probabilities"] == ps, f"{c['id']} bulk probabilities")
    if c["verb"] == "annotate":
        for name, sub in c["set"].items():
            want = exp["answers"][name]
            if "failed" in want:
                # The ruled failed marker (0054): the shape is exact, the
                # cause is closed, and no answer may derive for it.
                check(set(want) == {"failed"}, f"{c['id']} annotate {name} marker shape")
                check(set(want["failed"]) == {"kind", "cause"}, f"{c['id']} annotate {name} keys")
                check(want["failed"]["kind"] == "backend", f"{c['id']} annotate {name} kind")
                check(want["failed"]["cause"] in CAUSES, f"{c['id']} annotate {name} cause closed")
                continue
            p = reply["answers"][f"q{list(c['set']).index(name) + 1}"]["noul"]
            got = decide_answer(p, sub["threshold"])
            check(want["answer"] == got, f"{c['id']} annotate {name}")
            check(want["probability"] == p, f"{c['id']} annotate {name} p")
        counted = sum(1 for name in c["set"] if "failed" in exp["answers"][name])
        check(exp.get("failed_questions") == counted, f"{c['id']} failed_questions counted")
    # The ruled record rows ({"input","value"}) on the bulk cases.
    if "rows" in exp:
        rows = exp["rows"]
        check(isinstance(rows, list), f"{c['id']} rows list")
        for row in rows:
            check(set(row) == {"input", "value"}, f"{c['id']} row keys input and value only")
        if c["verb"] == "filter":
            ps = [e["reply"]["answers"]["q1"]["noul"] for e in c["exchanges"]]
            t = q.get("threshold", 0.5)
            kept = [i for i, p in enumerate(ps) if decide_answer(p, t)]
            check([r["input"] for r in rows] == [c["records"][i] for i in kept],
                  f"{c['id']} row inputs are the kept records in input order")
            check(all(r["value"] is True for r in rows), f"{c['id']} row values are the kept verdict")
        if c["verb"] == "decide_many":
            ps = [e["reply"]["answers"]["q1"]["noul"] for e in c["exchanges"]]
            t = q.get("threshold", 0.5)
            check([r["input"] for r in rows] == c["records"], f"{c['id']} row inputs in input order")
            check([r["value"] for r in rows] == [decide_answer(p, t) for p in ps],
                  f"{c['id']} row values are the answers")

for c in data["cases"]:
    check(set(c) <= CASE_KEYS, f"{c.get('id')} unknown keys {set(c) - CASE_KEYS}")
    is_usage_refusal = "error" in c["expect"] and c["expect"]["error"].get("kind") == "usage"
    if "question_file" in c:
        named = c["question_file"]
        check(isinstance(named, str) and named.startswith("@") and len(named) > 1,
              f"{c.get('id')} question_file is @ and a path")
        check(c["exchanges"] == [], f"{c['id']} a local failure sends nothing")
        check(c["expect"].get("error", {}).get("kind") == "local",
              f"{c['id']} a named file that fails is the local kind")
    elif c["verb"] == "find":
        check(isinstance(c["question"], str) and c["question"].strip() != "",
              f"{c['id']} find takes its question as text")
        check(2 <= len(c["records"]) <= (254 if c.get("none") else 255),
              f"{c['id']} find takes 2 to 255 units, 254 with none")
    elif not is_usage_refusal and c["verb"] not in ("recognize", "relate"):
        check_question(c["question"], c["verb"], where=f"{c['id']}: ")
    if c["verb"] == "annotate":
        for name, member in c.get("set", {}).items():
            check_question(member, member and [k for k in VERB_KEYS if k in member][0],
                           where=f"{c['id']}/{name}: ")
    if c["verb"] == "recognize":
        check_recognize_case(c)
    if c["verb"] == "relate":
        check_relate_case(c)
    failed_names = ()
    if c["verb"] == "annotate":
        # The failed logical names map to their wire names: set order is
        # name order, and q{i} names the i-th logical question.
        order = list(c.get("set", {}))
        failed_names = tuple(f"q{order.index(name) + 1}" for name in order
                             if "failed" in c["expect"]["answers"].get(name, {}))
    for ex in c["exchanges"]:
        check_reply_shape(ex["request"], ex.get("reply", {}), ex.get("status"), failed_names)
    replay(c)

if errors:
    print(f"FAILED: {len(errors)} problem(s)")
    for e in errors:
        print(" -", e)
    sys.exit(1)
print(f"OK: {len(data['cases'])} cases validated: schema, grammar, digests, wire contract, offline replay")
