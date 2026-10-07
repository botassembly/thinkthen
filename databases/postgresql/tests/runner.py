#!/usr/bin/env python3
"""The shared conformance cases, run as SQL through psql (ticket 0111).

`runner.py plan` prints one line per case of `conformance/cases.json`: the
id, the backend arm, and any server setting. check.sh restarts the local
server on that arm with a new backend and cache folder, then runs
`runner.py SOCKET ID`. That prints one line: `pass ID`, `fail ID: why`, or
`not run ID: reason`. The runner has no skip list and never excuses a
wrong answer by a note. `--cases FILE` reads another file, for the
runner's self-test. The backend's port comes from BPORT, a scratch
folder from SCRATCH, and the server's cache folder from STORE. A passing
case must leave one answer row per good question in STORE's
`thinkthen.sqlite` (ticket 0348).
"""

import hashlib
import json
import os
import pathlib
import sqlite3
import subprocess
import sys
import tempfile

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance" / "children"))
from children import child_env  # noqa: E402  the shared helper, ticket 0127
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[2] / "sqlite/tests"))
from question_keys import question_keys  # noqa: E402  the ADR 0111 question key

CASES = pathlib.Path(__file__).resolve().parents[3] / "conformance" / "cases.json"
CANONICAL = "https://api.typesafe.ai/v1/systemone"
NOT_RUN = {
    "23-cancelled-fault": "not run: SQL has no token; the R1-22 and R2-24 tests cover cancel",
    "25-defect-fault": "not run: no outside boundary reaches a defect; the panic-probe test covers XX000",
    "18-annotate-two-groups": "not run: one record's groups were recorded as separate requests, and ADR 0111 section 5 packs them into one",
}
# The arm and server setting of each fault case; every other case runs on its own case arm.
FAULTS = {
    "20-usage-fault": ("generic", ""),
    "21-backend-fault": ("arm/refuse", ""),
    "22-local-fault": ("generic", "thinkthen.cache = '@SCRATCH@/not-a-folder'"),
    "24-deadline-fault": ("generic", "thinkthen.deadline_ms = 0"),
    "29-usage-json-text": ("generic", ""),
    "30-local-question-file": ("generic", ""),
}
SQLSTATE = {"usage": "22023", "backend": "38000", "local": "58030", "deadline": "57014"}


class Failed(Exception):
    pass


def load(path):
    return json.loads(pathlib.Path(path).read_text())


def selected_ids(cases):
    available = [case["id"] for case in cases]
    if len(available) != len(set(available)):
        raise ValueError("duplicate shared case ID")
    path = os.environ.get("THINKTHEN_CONFORMANCE_IDS")
    if path is None:
        return set(available)
    source = pathlib.Path(path)
    if not source.is_absolute():
        raise ValueError("THINKTHEN_CONFORMANCE_IDS takes an absolute path")
    chosen = [line.strip() for line in source.read_text().splitlines()]
    chosen = [one for one in chosen if one and not one.startswith("#")]
    if not chosen:
        raise ValueError("the selected case list is empty")
    if len(chosen) != len(set(chosen)):
        raise ValueError("duplicate selected case ID")
    absent = set(chosen) - set(available)
    if absent:
        raise ValueError(f"selected case is absent from the shared corpus: {sorted(absent)[0]}")
    return set(chosen)


def skipped(case):
    if case["id"] in NOT_RUN:
        return NOT_RUN[case["id"]]
    return None


def plan(cases):
    for case in cases:
        arm, setting = FAULTS.get(case["id"], (f"case/{case['id']}", ""))
        if case["id"] in ("18-find-second", "19-find-none"):
            arm += "/capture"
        print(f"{case['id']}\t{'-' if skipped(case) else arm}\t{setting}")


def lit(value):
    return "'" + str(value).replace("'", "''") + "'"


def texts(values):
    return "ARRAY[" + ", ".join(lit(value) for value in values) + "]::text[]"


def keyed(values):
    return lit(json.dumps({str(index): value for index, value in enumerate(values)})) + "::jsonb"


def psql(*statements):
    command = ["psql", "-X", "-q", "-At", "-v", "ON_ERROR_STOP=1", "-v", "VERBOSITY=verbose"]
    command += ["-h", SOCKET, "-U", "postgres", "-d", "postgres"]
    for statement in ("SET statement_timeout='30s'", *statements):
        command += ["-c", statement]
    done = subprocess.run(command, capture_output=True, text=True, timeout=30, check=False,
                          env=child_env())
    if done.returncode:
        raise Failed(done.stderr.strip() or f"psql exited {done.returncode}")
    return done.stdout.rstrip("\n")


def close(one, other):
    if isinstance(one, bool) or isinstance(other, bool):
        return one is other
    if isinstance(one, (int, float)) and isinstance(other, (int, float)):
        return abs(one - other) < 1e-9
    if isinstance(one, list) and isinstance(other, list):
        return len(one) == len(other) and all(close(a, b) for a, b in zip(one, other))
    if isinstance(one, dict) and isinstance(other, dict):
        return one.keys() == other.keys() and all(close(one[k], other[k]) for k in one)
    return one == other


def same(what, got, want):
    if not close(got, want):
        raise Failed(f"{what}: got {json.dumps(got)}, expected {json.dumps(want)}")


def digest(address, request):
    return hashlib.sha256(b"systemone\n" + address.encode() + b"\n" + request.encode()).hexdigest()


def url(case):
    arm = f"case/{case['id']}"
    if case["id"] in ("18-find-second", "19-find-none"):
        arm += "/capture"
    return f"http://127.0.0.1:{os.environ['BPORT']}/{arm}/v1/systemone"


def served(case):
    """Each recorded request digest, renamed for the served URL. Every row
    lists its question keys (ADR 0111 section 2), so a digest maps to its
    request's keys."""
    return {digest(CANONICAL, exchange["request"]): question_keys(url(case), exchange["request"], exchange["response"]["model"])
            for exchange in case.get("exchanges", [])}


def swap(value, renamed):
    if isinstance(value, str):
        return renamed.get(value, value)
    if isinstance(value, list):
        return [swap(item, renamed) for item in value]
    if isinstance(value, dict):
        return {key: swap(item, renamed) for key, item in value.items()}
    return value


def detailed(got, want):
    answer, wanted = got.get("answer", {}), want["details"]["answer"]
    key = "probability" if "probability" in wanted else "probabilities"
    same(key, answer.get(key), wanted[key])
    if "level" in wanted:
        same("level", answer.get("level"), wanted["level"])
    same("confidence", answer.get("confidence", "absent"), wanted.get("confidence", "absent"))
    for key in ("model", "question_sha256", "usage", "requests_sent", "cached"):
        same(key, got["meta"].get(key, "absent"), want["details"].get(key, "absent"))
    # A request stands for the list of its keys, so a row reads the flattened list.
    wanted = want["details"].get("requests")
    same("requests", got["meta"].get("requests", "absent"),
         "absent" if wanted is None else [key for held in wanted for key in held])


TYPED = {
    "decide": "thinkthen_decide({q}, {e})::text",
    "choose": "thinkthen_choose({q}, {e}, NULL)",
    "score": "thinkthen_score({q}, {e}, NULL)::text",
    "tag": "array_to_json(thinkthen_tag({q}, {e}, NULL))::text",
}


def typed(verb, text):
    if text == "":
        return None
    if verb in ("choose",):
        return text
    return json.loads({"t": "true", "f": "false"}.get(text, text))


def single(case, success):
    want = success["answers"][0]
    question, evidence = lit(json.dumps(case["question"])), lit(case["exchanges"][0]["evidence"])
    got = json.loads(psql(f"SELECT thinkthen_details({question}, {evidence})"))
    same("bare", got["value"], want["bare"])
    detailed(got, want)
    same("url", got["meta"]["url"], url(case))
    bare = psql("SELECT " + TYPED[case["verb"]].format(q=question, e=evidence))
    same("typed", typed(case["verb"], bare), want["bare"])
    counters = success.get("counters")
    if counters:  # last, on an empty cache folder of their own
        usage = "SELECT requests_sent || ' ' || cache_answers FROM thinkthen_usage()"
        calls = [f"SELECT 1 FROM thinkthen_details({question}, {evidence})"] * counters["calls"]
        with tempfile.TemporaryDirectory() as folder:
            lines = psql(f"SET thinkthen.cache = {lit(folder)}", usage, *calls, usage).splitlines()
        (sent0, cached0), (sent1, cached1) = map(int, lines[0].split()), map(int, lines[-1].split())
        moved = {"calls": counters["calls"], "requests": sent1 - sent0, "cache_answers": cached1 - cached0}
        same("counters", moved, counters)


def many(case, success):
    records = [exchange["evidence"] for exchange in case["exchanges"]]
    rows = psql(f"SELECT coalesce(value::text, 'null') FROM thinkthen_decide_many({lit(json.dumps(case['question']))}, {keyed(records)}) ORDER BY key::int")
    got = [json.loads({"t": "true", "f": "false"}.get(row, row)) for row in rows.splitlines()]
    same("decided", got, [answer["bare"] for answer in success["answers"]])


def selected_rows(case, success):
    records = [exchange["evidence"] for exchange in case["exchanges"]]
    if case["verb"] == "filter":
        question = lit(json.dumps(case["question"]))
        input_rows = f"SELECT key::int AS i, value AS decided, probability FROM thinkthen_decide_many({question}, {keyed(records)})"
        query = (f"WITH input AS MATERIALIZED ({input_rows}), "
                 "kept AS MATERIALIZED (SELECT i FROM input WHERE decided) "
                 "SELECT coalesce(json_agg(i ORDER BY i), '[]'::json) FROM kept")
        wanted = success["operation"]["indexes"]
    else:
        # Batch 1 keeps one recorded exchange per record.
        ranked = (f"SELECT key::int AS i, rank, probability FROM thinkthen_rank("
                  f"{lit(case['question']['decide'])}, {keyed(records)}, '{{\"batch\":1}}'::json)")
        query = (f"WITH ranked AS MATERIALIZED ({ranked}) "
                 "SELECT coalesce(json_agg(json_build_object('index', i, 'probability', probability) "
                 "ORDER BY rank), '[]'::json) FROM ranked")
        wanted = success["operation"]["ranking"]
    lines = psql("SELECT requests_sent FROM thinkthen_usage()", query,
                 "SELECT requests_sent FROM thinkthen_usage()").splitlines()
    same("SQL rows", json.loads(lines[1]), wanted)
    same("judgments sent", int(lines[2]) - int(lines[0]), len(records))


def found(case, success):
    question = case["question"]
    units = question["units"]
    none = "true" if question.get("none", False) else "false"
    settings = lit(json.dumps({"none": True})) + "::json" if none == "true" else "NULL::json"
    answer = json.loads(psql(f"SELECT thinkthen_find({lit(question['find'])}, {texts(units)}, {settings})"))
    operation = success["operation"]
    selected = operation["selected"]
    wanted = {
        "index": selected,
        "value": units[selected] if selected is not None else None,
        "probability": next(one["probability"] for one in operation["probabilities"]
                            if one["index"] == selected),
        "candidates": operation["probabilities"],
    }
    same("find result", answer, wanted)


def captured(case, path):
    observed = load(path)
    if "error" in observed:
        raise Failed(f"backend capture: {observed['error']}")
    bodies = observed.get("bodies", [])
    same("captured body count", len(bodies), 1)
    same("captured request", bodies[0], case["exchanges"][0]["request"])
    same("captured digest", digest(url(case), bodies[0]),
         digest(url(case), case["exchanges"][0]["request"]))


def annotated(case, success):
    question_set = lit(json.dumps(case["question_set"]))
    failed, records = 0, {}
    for want in success["answers"]:
        evidence = json.dumps(case["record"]) if "record" in case else case["exchanges"][want["exchange"]]["evidence"]
        # One call per record: a failed answer is never stored (ADR 0111
        # section 5), so a second call would ask the failed question alone.
        if evidence not in records:
            records[evidence] = json.loads(psql(f"SELECT thinkthen_annotate({question_set}, {lit(evidence)})"))
        got = records[evidence].get(want["name"])
        failed += isinstance(got, dict) and "failed" in got
        same(want["name"], got, want["bare"])
    same("failed", failed, success.get("failed_questions", 0))


def pair(entity):
    return {"text": entity["text"], "kind": entity["kind"]}


def recognized(case, success):
    spec, text = lit(json.dumps(case["question"])), lit(case["text"])
    want = success["answers"][0]["bare"]
    rows = psql(f"SELECT json_agg(json_build_object('text', text, 'start', start, 'end', \"end\", 'length', length, 'kind', kind, 'strength', strength)) FROM thinkthen_recognize({text}, {spec})")
    same("entities", json.loads(rows) or [], want["entities"])
    if "relations" in want:
        rows = psql(f"SELECT json_agg(json_build_object('relation', relation, 'source', json_build_object('text', source_text, 'kind', source_kind), 'target', json_build_object('text', target_text, 'kind', target_kind), 'probability', probability, 'either', either)) FROM thinkthen_relations({text}, {spec})")
        wanted = [dict(one, source=pair(one["source"]), target=pair(one["target"]), either=one.get("either", False)) for one in want["relations"]]
        same("relations", json.loads(rows) or [], wanted)


def related(case, success):
    entities = case["entities"]
    values = ", ".join(f"({n}, {lit(one['name'])}, {lit(one['kind'])})" for n, one in enumerate(entities))
    query = lit(f"SELECT * FROM (VALUES {values}) v(id, body, kind)")
    rows = psql(f"SELECT json_agg(json_build_object('relation', relation, 's', source, 't', target, 'probability', probability, 'either', either)) FROM thinkthen_relate({query}, {lit(json.dumps(case['question']))})")
    got = [{"relation": one["relation"], "source": entities[one["s"]], "target": entities[one["t"]], "probability": one["probability"], **({"either": True} if one["either"] else {})} for one in json.loads(rows) or []]
    order = lambda edge: (edge["relation"], edge["source"]["name"], edge["target"]["name"])  # noqa: E731
    same("result", sorted(got, key=order), sorted(success["answers"][0]["bare"], key=order))


def refused(case, kind):
    question = lit(json.dumps(case["question"]))
    evidence = {"20-usage-fault": "   "}.get(case["id"], "Is this urgent?")
    if case.get("question_form") == "file":
        path = pathlib.Path(os.environ["SCRATCH"]) / f"{case['id']}.json"
        path.write_text(json.dumps(case["question"]))
        question = lit(f"@{path}")
    try:
        if case["id"] == "31-usage-rank-blank-question":
            got = psql(f"SELECT count(*) FROM thinkthen_rank({lit(case['question']['decide'])}, {keyed([evidence])})")
        else:
            got = psql(f"SELECT thinkthen_decide({question}, {lit(evidence)})")
    except Failed as error:
        said = str(error)
        if f"ERROR:  {SQLSTATE[kind]}: thinkthen {kind}: " not in said:
            raise Failed(f"expected {kind} ({SQLSTATE[kind]}), got {said}") from None
        return
    raise Failed(f"expected {kind}, got the answer {got!r}")


def run(case):
    expect = case["expect"]
    if "error" in expect:
        return refused(case, expect["error"]["kind"])
    success = swap(expect["success"], served(case))
    verb, kind = case["verb"], success["kind"]
    if kind == "decide_many":
        return many(case, success)
    if kind == "find":
        return found(case, success)
    if verb in ("filter", "rank"):
        return selected_rows(case, success)
    handler = {"annotate": annotated, "recognize": recognized, "relate": related}.get(verb, single)
    return handler(case, success)


def stored(case, success):
    """ADR 0111: the case ran on the question store, one row per good answer."""
    keys = {key for exchange in case.get("exchanges", []) for key in question_keys(url(case), exchange["request"], exchange["response"]["model"])}
    if not os.environ.get("STORE"):
        raise Failed("stored answers: no STORE folder to count")
    store = pathlib.Path(os.environ["STORE"]) / "thinkthen.sqlite"
    held = 0
    if store.is_file():
        connection = sqlite3.connect(f"file:{store}?mode=ro", uri=True)
        try:
            held = connection.execute("SELECT count(*) FROM answers").fetchone()[0]
        finally:
            connection.close()
    same("stored answers", held, len(keys) - success.get("failed_questions", 0))


def main():
    args = sys.argv[1:]
    path = CASES
    if "--cases" in args:
        at = args.index("--cases")
        path = args[at + 1]
        del args[at : at + 2]
    cases = load(path)["cases"]
    try:
        selected = selected_ids(cases)
    except (OSError, ValueError) as error:
        print(f"fail selector: {error}", file=sys.stderr)
        return 1
    chosen = [case for case in cases if case["id"] in selected]
    if args == ["plan"]:
        plan(chosen)
        not_run = sum(skipped(case) is not None for case in chosen)
        print(f"postgresql plan: total={len(cases)} selected={len(chosen)} supported={len(chosen) - not_run} not_run={not_run} unselected={len(cases) - len(chosen)}", file=sys.stderr)
        return 0
    if len(args) == 3 and args[0] == "capture":
        _, wanted, path = args
        case = next((one for one in chosen if one["id"] == wanted), None)
        if case is None or wanted not in ("18-find-second", "19-find-none"):
            print(f"fail {wanted}: no selected capture case")
            return 0
        try:
            captured(case, path)
        except (Failed, OSError, ValueError, KeyError) as error:
            print(f"fail {wanted}: {error}")
            return 0
        print(f"pass {wanted}")
        return 0
    global SOCKET
    SOCKET, wanted = args
    if wanted not in selected:
        print(f"fail {wanted}: not selected")
        return 0
    case = next((one for one in cases if one["id"] == wanted), None)
    if case is None:
        print(f"fail {wanted}: absent from the shared corpus")
        return 0
    if skipped(case):
        print(f"not run {wanted}: {skipped(case).removeprefix('not run: ')}")
        return 0
    try:
        run(case)
        if "success" in case["expect"]:
            stored(case, case["expect"]["success"])
    except (Failed, subprocess.TimeoutExpired, KeyError, ValueError, sqlite3.Error) as error:
        print(f"fail {wanted}: {error}")
        return 0
    print(f"pass {wanted}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
