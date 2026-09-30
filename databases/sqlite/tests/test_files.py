#!/usr/bin/env python3
"""The `'@name'` file door and the parse cache over it (ticket 0109 decision 4)."""

from __future__ import annotations

import json
import os
import pathlib
import sys
import tempfile

from helper import Backend, child, environment, expect, main

REFUSED = "thinkthen local: the question file '{}' did not read: it must be a regular file at most 1048576 bytes (retryable: no)"
QUESTION = json.dumps({"decide": "Is it red?"})


def asked(argument: str, timeout: float = 5) -> tuple[object, int]:
    backend = Backend()
    held = child(f"""
db = connect()
say(answer=run(db, "SELECT thinkthen_decide(?, 'a red door')", ({argument!r},)))
""", environment(backend), timeout)
    return held["answer"], backend.close()


def test_the_door_refuses_what_is_not_a_bounded_regular_file() -> None:
    """R3-2b and R4-6: a fifo, a link to one, a device, and an over-cap file."""
    folder = pathlib.Path(tempfile.mkdtemp(prefix="thinkthen-door-"))
    os.mkfifo(folder / "fifo")
    (folder / "fifo-link").symlink_to(folder / "fifo")
    (folder / "big.json").write_text(" " * 1_048_577)
    for name in (folder / "fifo", folder / "fifo-link", "/dev/zero", folder / "big.json"):
        argument = f"@{name}"
        expect(asked(argument, timeout=5), (REFUSED.format(argument), 0), f"{name}")
    backend = Backend()
    held = child("""
db = connect()
db.execute("CREATE TABLE e(id INTEGER, name TEXT, kind TEXT)")
say(recognize=run(db, "SELECT * FROM thinkthen_recognize('Ada', '@/dev/zero')"),
    relate=run(db, "SELECT * FROM thinkthen_relate('SELECT id, name, kind FROM e', '@/dev/zero')"),
    questions=run(db, "SELECT thinkthen_annotate('@/dev/zero', 'x')"))
""", environment(backend), 5)
    sentence = "thinkthen local: the {} file '@/dev/zero' did not read: it must be a regular file at most 1048576 bytes (retryable: no)"
    expect((held, backend.close()), ({"recognize": sentence.format("recognize spec"), "relate": sentence.format("relate spec"),
                                      "questions": sentence.format("question set")}, 0), "each door names its file's kind")


def test_a_link_to_a_question_file_reads_its_target() -> None:
    """R5-20 and R4-6: the door follows symlinks and confines nothing."""
    folder = pathlib.Path(tempfile.mkdtemp(prefix="thinkthen-door-"))
    (folder / "question.json").write_text(QUESTION)
    (folder / "link.json").symlink_to(folder / "question.json")
    expect(asked(f"@{folder / 'link.json'}"), ([[1]], 1), "the linked question")


def test_invalid_question_file_with_settings_stays_local() -> None:
    """The settings merge must not turn a broken named question into usage."""
    folder = pathlib.Path(tempfile.mkdtemp(prefix="thinkthen-door-"))
    source = folder / "bad.json"
    source.write_text('{"decide":""}')
    backend = Backend()
    held = child(f"""
db = connect()
say(answer=run(db, "SELECT thinkthen_decide(?, 'a red door', ?)", ('@{source}', '{{"context":"scene"}}')))
""", environment(backend))
    expect(held["answer"].startswith("thinkthen local:"), True, "file parser kind")
    expect(backend.close(), 0, "invalid named question sends nothing")


def test_a_rewritten_file_is_read_again() -> None:
    """R6-14: a rewrite that moves the modified time is re-read, for questions and sets."""
    backend = Backend()
    folder = pathlib.Path(tempfile.mkdtemp(prefix="thinkthen-door-"))
    question, questions = folder / "q.json", folder / "qs.json"
    question.write_text(json.dumps({"decide": "Is it red?", "model": "judge-a"}))
    questions.write_text(json.dumps({"version": 1, "questions": {"kind": {"decide": "Is it red?"}}}))
    held = child(f"""
import os
def moved(path, text):
    before = os.stat(path).st_mtime
    open(path, "w").write(text)
    os.utime(path, (before + 2, before + 2))
db = connect()
packed = run(db, "SELECT count(*) FROM thinkthen_decide_many(?, ?)", ('@{question}', '{{"7":"a red door"}}'))
moved({str(question)!r}, {json.dumps({"decide": "Is it red?", "model": "judge-b"})!r})
decided = run(db, "SELECT thinkthen_decide('@{question}', 'a red door')")
first = run(db, "SELECT thinkthen_annotate('@{questions}', 'a red door')")
moved({str(questions)!r}, {json.dumps({"version": 1, "questions": {"topic": {"decide": "Is it red?"}}})!r})
second = run(db, "SELECT thinkthen_annotate('@{questions}', 'a red door')")
say(packed=packed, decided=decided, first=first, second=second)
""", environment(backend))
    expect(held, {
        "packed": [[1]],
        "decided": [[1]],
        "first": [['{"kind":true}']],
        "second": [['{"topic":true}']],
    }, "the answers")
    expect(backend.close(), 3, "sends: model B asked again, and the renamed set read its cached answer")


def test_keyed_many_takes_the_banded_file_decide_uses() -> None:
    """The keyed form takes decide's banded file and later scalars read the cache."""
    backend = Backend()
    folder = pathlib.Path(tempfile.mkdtemp(prefix="thinkthen-door-"))
    banded = {"decide": "Is it red?", "true": "Red paint.", "false": "Any other colour.", "model": "judge-b", "threshold": "0.85:0.95"}
    (folder / "banded.json").write_text(json.dumps(banded))
    held = child(f"""
db = connect()
db.execute("SELECT thinkthen_configure(?)", ('{{"batch":1}}',))
db.execute("CREATE TABLE t(body TEXT)")
db.executemany("INSERT INTO t VALUES (?)", [("a red door",), ("a blue door",), ("a red door",)])
packed = run(db, "SELECT count(*) FROM thinkthen_decide_many(?, ?)", ('@{folder / 'banded.json'}', '{{"1":"a red door","2":"a blue door","3":"a red door"}}'))
decided = run(db, "SELECT thinkthen_decide('@{folder / 'banded.json'}', body) FROM t")
inline = run(db, "SELECT count(*) FROM thinkthen_decide_many(?, ?)", ({json.dumps(banded)!r}, '{{"1":"a red door","2":"a blue door","3":"a red door"}}'))
say(packed=packed, decided=decided, inline=inline)
""", environment(backend))
    expect(held, {"packed": [[3]], "decided": [[None], [None], [None]], "inline": [[3]]}, "the answers")
    expect(backend.close(), 2, "sends: two distinct singleton bodies; later identical calls read cache")


if __name__ == "__main__":
    sys.exit(main(globals()))
