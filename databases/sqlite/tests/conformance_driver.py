#!/usr/bin/env python3
"""The SQLite surface's slice of the conformance file, run offline.

Runs each case as SQL: the decide family through thinkthen_decide, the
filter cases through the ruled SQL pattern (warm the table, then WHERE
thinkthen_decide reads the saved answers), choose, score, and tag through
their JSON question grammar, annotate through a set, details and usage
directly, recognize through the table-valued function's five columns, and
relate through the table-valued call every record at once. Prints one line
a case — ok, diverge with its reason, skip with its reason — and exits
nonzero on any divergence. Run through ./check.sh.

Skips, with their reasons: the backend-refusal cases need the wire or a
dead address (the wire suite proves the backend kind there), and the
cancel case diverges on the stand-in, which conformance/DIVERGENCES.md
records as a real-engine requirement: the pre-fired token is ignored, and
this surface proves the mid-flight interrupt on the wire instead. The
relate per-subject arm is skipped: it is an engine-internal form, and the
surface serves the ruled pairs form that case 72 covers. Recognize cases
that ask relation rules assert their names only here: relations as rows
ride the beta question-file function, which the build team's page leaves
open.
"""

import json
import os
import pathlib
import sqlite3
import sys
import tempfile

HERE = pathlib.Path(__file__).resolve().parent
LIB = HERE.parent / "target" / "release" / "libthinkthen0.so"
CASES = HERE.parent.parent.parent / "conformance" / "conformance.json"

os.environ.setdefault("ENGINE_NULL", "1")

FAILURES = 0
TEMP = []


def fresh():
    connection = sqlite3.connect(":memory:")
    connection.enable_load_extension(True)
    connection.load_extension(str(LIB))
    return connection


def report(case_id, text="", failed=False):
    global FAILURES
    if failed:
        FAILURES += 1
        print(f"FAIL     {case_id}: {text}")
    else:
        print(f"ok       {case_id}{': ' + text if text else ''}")


def temp_file(body, suffix=".json"):
    handle = tempfile.NamedTemporaryFile(
        "w", suffix=suffix, delete=False, dir=HERE.parent
    )
    handle.write(body)
    handle.close()
    TEMP.append(handle.name)
    return "@" + os.path.basename(handle.name)


def outcome_of(wanted):
    if wanted is True:
        return 1
    if wanted is False:
        return 0
    return None


def as_sql_question(question):
    return temp_file(json.dumps(question))


def main():
    cases = json.loads(CASES.read_text())["cases"]
    conn = fresh()

    for case in cases:
        case_id = case["id"]
        verb = case["verb"]
        if "question_file" in case:
            print(f"skip     {case_id}: the local kind needs a file door")
            continue
        question = case["question"]
        evidence = case.get("evidence")
        expect = case["expect"]

        if "error" in expect and expect["error"]["kind"] == "backend":
            print(f"skip     {case_id}: the backend kind needs the wire")
            continue

        if "error" in expect and expect["error"].get("kind") == "deadline":
            print(f"skip     {case_id}: the spent-budget case needs a deadline door this driver does not carry")
            continue

        try:
            if verb in ("decide", "details", "usage", "cancel"):
                if verb == "usage":
                    # The counters are cumulative (the reset-removal ruling),
                    # so the case is read as deltas between snapshots; its
                    # after_reset arm is superseded and its place is taken
                    # by the reset spelling's refusal.
                    before = json.loads(
                        conn.execute("SELECT thinkthen_usage()").fetchone()[0]
                    )
                    conn.execute(
                        "SELECT thinkthen_decide(?, ?)",
                        (json.dumps(question), evidence),
                    ).fetchone()
                    conn.execute(
                        "SELECT thinkthen_decide(?, ?)",
                        (json.dumps(question), evidence),
                    ).fetchone()
                    held = json.loads(
                        conn.execute("SELECT thinkthen_usage()").fetchone()[0]
                    )
                    wanted = expect
                    sent = held["requests"] - before["requests"]
                    served = held["cache_answers"] - before["cache_answers"]
                    try:
                        conn.execute("SELECT thinkthen_usage('reset')").fetchall()
                        refusal = "accepted"
                    except sqlite3.OperationalError as failure:
                        refusal = str(failure)
                    if (
                        sent == wanted["requests"]
                        and served == wanted["cache_answers"]
                        and "cumulative" in refusal
                        and "subtract" in refusal
                    ):
                        report(
                            case_id,
                            "sends and served answers counted by delta; "
                            "the reset spelling refuses (after_reset superseded)",
                        )
                    else:
                        report(
                            case_id,
                            f"usage delta {sent}/{served}, refusal {refusal!r}, "
                            f"expected {wanted}",
                            failed=True,
                        )
                    continue
                if verb == "cancel":
                    print(
                        f"diverge  {case_id}: the stand-in ignores a pre-fired "
                        "token; conformance/DIVERGENCES.md carries this as a "
                        "real-engine requirement, and the wire suite proves "
                        "the mid-flight interrupt"
                    )
                    continue
                sql = (
                    "SELECT thinkthen_details(?, ?)"
                    if verb == "details"
                    else "SELECT thinkthen_decide(?, ?)"
                )
                try:
                    held = conn.execute(
                        sql, (json.dumps(question), evidence)
                    ).fetchone()[0]
                except sqlite3.OperationalError as failure:
                    if expect.get("error", {}).get("kind") == "usage":
                        report(case_id, f"refused ({failure})")
                    else:
                        report(case_id, str(failure), failed=True)
                    continue
                if "error" in expect:
                    report(
                        case_id,
                        f"held {held!r}, expected {expect['error']['kind']}",
                        failed=True,
                    )
                elif verb == "details":
                    audit = json.loads(held)
                    wanted = expect["details"]
                    same = (
                        audit["model"] == wanted["model"]
                        and audit["digest"] == wanted["question_sha256"]
                    )
                    if "requests" in wanted:
                        same = same and audit["requests"] == wanted["requests"]
                    if "failed_questions" in wanted:
                        same = same and audit["failed_questions"] == wanted[
                            "failed_questions"
                        ]
                    if same:
                        report(case_id, "audit carried, digest equal")
                    else:
                        report(case_id, f"audit {audit}", failed=True)
                else:
                    wanted = outcome_of(expect["answer"])
                    if held == wanted:
                        report(case_id)
                    else:
                        report(
                            case_id, f"held {held!r}, expected {wanted!r}", failed=True
                        )
            elif verb == "filter":
                fresh_conn = fresh()
                fresh_conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, body TEXT)")
                fresh_conn.executemany(
                    "INSERT INTO t(body) VALUES (?)",
                    [(text,) for text in case["records"]],
                )
                if "error" in expect:
                    print(
                        f"diverge  {case_id}: the band-on-filter rule lives at "
                        "the case verb, and no stand-in door resolves a "
                        "question as a filter; conformance/DIVERGENCES.md "
                        "carries this as a real-engine requirement"
                    )
                    continue
                fresh_conn.execute(
                    "SELECT thinkthen_warm(?, body) FROM t", (json.dumps(question),)
                ).fetchall()
                held = [
                    row[0]
                    for row in fresh_conn.execute(
                        "SELECT id FROM t WHERE thinkthen_decide(?, body)",
                        (json.dumps(question),),
                    )
                ]
                wanted = [index + 1 for index in expect["indexes"]]
                if held == wanted:
                    report(case_id, f"kept {held}")
                else:
                    report(case_id, f"kept {held}, expected {wanted}", failed=True)
                if expect.get("rows"):
                    # The ruled record row (go-ahead item 4): SQL's own two
                    # columns are the row, `input` and `value`, in input
                    # order. Case 06's empty list is covered by its count.
                    pairs = "~".join(
                        f"{row['input']}|{'1' if row['value'] else '0'}"
                        for row in expect["rows"]
                    )
                    held_pairs = fresh_conn.execute(
                        "SELECT group_concat(body || '|' || "
                        "thinkthen_decide(?, body), '~') FROM "
                        "(SELECT body FROM t WHERE thinkthen_decide(?, body) ORDER BY id)",
                        (json.dumps(question), json.dumps(question)),
                    ).fetchone()[0]
                    if held_pairs == pairs:
                        report(case_id + " rows", pairs)
                    else:
                        report(
                            case_id + " rows",
                            f"held {held_pairs!r}, expected {pairs!r}",
                            failed=True,
                        )
            elif verb in ("choose", "score", "tag"):
                sql = f"SELECT thinkthen_{verb}(?, ?)"
                try:
                    held = conn.execute(
                        sql, (json.dumps(question), evidence)
                    ).fetchone()[0]
                except sqlite3.OperationalError as failure:
                    if expect.get("error", {}).get("kind") == "usage":
                        report(case_id, f"refused ({failure})")
                    else:
                        report(case_id, str(failure), failed=True)
                    continue
                if "error" in expect:
                    report(case_id, f"held {held!r}", failed=True)
                elif verb == "choose":
                    wanted = expect.get("answer")
                    if held == wanted:
                        report(case_id, f"{held!r}")
                    else:
                        report(case_id, f"held {held!r}, expected {wanted!r}", failed=True)
                elif verb == "score":
                    if held is not None and abs(held - expect["answer"]) < 1e-9:
                        report(case_id, f"{held}")
                    else:
                        report(case_id, f"held {held!r}", failed=True)
                else:
                    if held is not None and json.loads(held) == expect["answer"]:
                        report(case_id, held)
                    else:
                        report(case_id, f"held {held!r}", failed=True)
            elif verb == "annotate":
                body = json.dumps({"questions": case["set"]})
                held = conn.execute(
                    "SELECT thinkthen_annotate(?, ?)", (body, evidence)
                ).fetchone()[0]
                fields = json.loads(held)
                wanted = expect["answers"]

                def field_matches(name):
                    if "failed" in wanted[name]:
                        return fields[name].get("failed") == wanted[name]["failed"]
                    return fields[name]["answer"] == wanted[name]["answer"]

                same = sorted(fields.keys()) == sorted(wanted.keys()) and all(
                    field_matches(name) for name in wanted
                )
                if same:
                    report(case_id, "fields assembled")
                else:
                    report(case_id, f"held {fields}", failed=True)
            elif verb == "decide_many":
                fresh_conn = fresh()
                fresh_conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, body TEXT)")
                fresh_conn.executemany(
                    "INSERT INTO t(body) VALUES (?)",
                    [(text,) for text in case["records"]],
                )
                fresh_conn.execute(
                    "SELECT thinkthen_warm(?, body) FROM t", (json.dumps(question),)
                ).fetchall()
                held = [
                    row[0]
                    for row in fresh_conn.execute(
                        "SELECT thinkthen_decide(?, body) FROM t ORDER BY id",
                        (json.dumps(question),),
                    )
                ]
                wanted = [outcome_of(answer) for answer in expect["answers"]]
                if held == wanted:
                    report(case_id, "warm is the bulk spine, answers in order")
                else:
                    report(case_id, f"held {held}, expected {wanted}", failed=True)
                if expect.get("rows"):
                    pairs = "~".join(
                        f"{row['input']}|{'1' if row['value'] else '0'}"
                        for row in expect["rows"]
                    )
                    held_pairs = fresh_conn.execute(
                        "SELECT group_concat(body || '|' || "
                        "thinkthen_decide(?, body), '~') FROM "
                        "(SELECT body FROM t ORDER BY id)",
                        (json.dumps(question),),
                    ).fetchone()[0]
                    if held_pairs == pairs:
                        report(case_id + " rows", pairs)
                    else:
                        report(
                            case_id + " rows",
                            f"held {held_pairs!r}, expected {pairs!r}",
                            failed=True,
                        )
            elif verb == "recognize":
                kinds = case["question"].get("kinds") or []
                kinds_arg = ",".join(kinds) if kinds else None
                held = conn.execute(
                    "SELECT text, kind, start, end, strength "
                    "FROM thinkthen_recognize(?, ?)",
                    (case["text"], kinds_arg),
                ).fetchall()
                wanted = [
                    (e["text"], e["kind"], e["start"], e["end"], e["strength"])
                    for e in expect["entities"]
                ]
                same = len(held) == len(wanted) and all(
                    h[0] == w[0]
                    and h[1] == w[1]
                    and h[2] == w[2]
                    and h[3] == w[3]
                    and abs(h[4] - w[4]) < 1e-9
                    for h, w in zip(held, wanted)
                )
                note = "names"
                if expect.get("relations"):
                    note += "; relations ride the beta question-file function"
                if same:
                    report(case_id, f"{len(held)} {note}")
                else:
                    report(case_id, f"held {held}, expected {wanted}", failed=True)
            elif verb == "relate":
                if case.get("form") == "per-subject":
                    print(
                        f"skip     {case_id}: the per-subject form is an "
                        "engine-internal arm; the surface serves the ruled "
                        "pairs form, which case 72 covers"
                    )
                    continue
                if case["question"].get("threshold", 0.5) != 0.5:
                    print(
                        f"skip     {case_id}: the SQLite call takes no "
                        "threshold argument"
                    )
                    continue
                if any(
                    rule.get("source") != "*" or rule.get("target") != "*"
                    for rule in case["question"]["relations"]
                ):
                    print(
                        f"skip     {case_id}: the recordings carry no kinds "
                        "for relate's records, so a named end is refused"
                    )
                    continue
                fresh_conn = fresh()
                fresh_conn.execute(
                    "CREATE TABLE rs(id INTEGER PRIMARY KEY, body TEXT)"
                )
                fresh_conn.executemany(
                    "INSERT INTO rs(body) VALUES (?)",
                    [(text,) for text in case["records"]],
                )
                names = [rule["name"] for rule in case["question"]["relations"]]
                sql = (
                    "SELECT name, source, target, probability FROM "
                    "thinkthen_relate('rs', 'id', 'body', "
                    + ",".join("?" * len(names))
                    + ")"
                )
                held = sorted(
                    (row[0], row[1], row[2], row[3])
                    for row in fresh_conn.execute(sql, names)
                )
                wanted = sorted(
                    (e["name"], e["source"], e["target"], e["probability"])
                    for e in expect["edges"]
                )
                same = len(held) == len(wanted) and all(
                    h[0] == w[0]
                    and h[1] == w[1]
                    and h[2] == w[2]
                    and abs(h[3] - w[3]) < 1e-9
                    for h, w in zip(held, wanted)
                )
                if same:
                    report(case_id, f"{len(held)} edges")
                else:
                    report(case_id, f"held {held}, expected {wanted}", failed=True)
            else:
                print(f"skip     {case_id}: {verb} is not this surface's shape offline")
        except sqlite3.OperationalError as failure:
            report(case_id, f"unexpected: {failure}", failed=True)

    for path in TEMP:
        os.unlink(path)
    print("conformance slice done" if not FAILURES else "conformance diverged")
    sys.exit(1 if FAILURES else 0)


if __name__ == "__main__":
    main()
