#!/usr/bin/env python3
"""The `'@name'` file door and the parse cache over it (ticket 0109 decision 4)."""

from __future__ import annotations

import json
import os
import pathlib
import sys
import tempfile

from helper import Backend, child, environment, expect, main

REFUSED = "thinkthen local: the question file '{}' did not read: it must be a regular file at most 1048576 bytes"
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


def test_a_link_to_a_question_file_reads_its_target() -> None:
    """R5-20 and R4-6: the door follows symlinks and confines nothing."""
    folder = pathlib.Path(tempfile.mkdtemp(prefix="thinkthen-door-"))
    (folder / "question.json").write_text(QUESTION)
    (folder / "link.json").symlink_to(folder / "question.json")
    expect(asked(f"@{folder / 'link.json'}"), ([[1]], 1), "the linked question")


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
warm = run(db, "SELECT thinkthen_warm('@{question}', 'a red door')")
moved({str(question)!r}, {json.dumps({"decide": "Is it red?", "model": "judge-b"})!r})
decided = run(db, "SELECT thinkthen_decide('@{question}', 'a red door')")
first = run(db, "SELECT thinkthen_annotate('@{questions}', 'a red door')")
moved({str(questions)!r}, {json.dumps({"version": 1, "questions": {"topic": {"decide": "Is it red?"}}})!r})
second = run(db, "SELECT thinkthen_annotate('@{questions}', 'a red door')")
say(warm=warm, decided=decided, first=first, second=second)
""", environment(backend))
    expect(held, {
        "warm": [[1]],
        "decided": [[1]],
        "first": [['{"kind":true}']],
        "second": [['{"topic":true}']],
    }, "the answers")
    expect(backend.close(), 3, "sends: model B asked again, and the renamed set read its cached answer")


if __name__ == "__main__":
    sys.exit(main(globals()))
