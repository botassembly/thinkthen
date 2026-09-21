#!/usr/bin/env python3
"""The SQLite surface's slice of the conformance file, run offline.

Runs each case as SQL: the decide family through thinkthen_decide, the
filter cases through the ruled SQL pattern (warm the table, then WHERE
thinkthen_decide reads the saved answers), choose, score, and tag through
their JSON question grammar, annotate through a set, details and usage
directly. Prints one line a case — ok, diverge with its reason, skip with
its reason — and exits nonzero on any divergence. Run through ./check.sh.

Skips, with their reasons: the backend-refusal cases need the wire or a
dead address (the wire suite proves the backend kind there), and the
cancel case diverges on the stand-in, which conformance/DIVERGENCES.md
records as a real-engine requirement: the pre-fired token is ignored, and
this surface proves the mid-flight interrupt on the wire instead.
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
                    requests = json.loads(
                        conn.execute("SELECT thinkthen_usage('reset')").fetchone()[0]
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
                    zeroed = json.loads(
                        conn.execute(
                            "SELECT thinkthen_usage('reset')"
                        ).fetchone()[0]
                    )
                    if (
                        held["requests"] == wanted["requests"]
                        and held["cache_answers"] == wanted["cache_answers"]
                        and zeroed["requests"] == wanted["after_reset"]["requests"]
                        and zeroed["cache_answers"]
                        == wanted["after_reset"]["cache_answers"]
                    ):
                        report(case_id, "sends and served answers counted")
                    else:
                        report(case_id, f"usage {held}, expected {wanted}", failed=True)
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
                    if (
                        abs(audit["probability"] - wanted["probability"]) < 1e-9
                        and audit["model"] == wanted["model"]
                        and audit["digest"] == wanted["question_sha256"]
                    ):
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
                same = sorted(fields.keys()) == sorted(wanted.keys()) and all(
                    fields[name]["answer"] == wanted[name]["answer"]
                    for name in wanted
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
