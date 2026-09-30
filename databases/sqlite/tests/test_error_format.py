#!/usr/bin/env python3
"""The installed host renders typed failures and explicit removal guidance."""

from __future__ import annotations

import json
import sys

from helper import Backend, child, environment, expect, main


def test_sql_errors_keep_typed_retryability_and_plain_removal() -> None:
    """One bounded 503 send distinguishes the suffix from a zero-send refusal."""
    backend = Backend()
    held = child("""
db = connect()
db.execute('SELECT thinkthen_configure(?)', ('{"max_retries":0}',))
usage = run(db, "SELECT thinkthen_decide('Is it red?', 'a red door', ?)", (json.dumps({"context":""}),))
removed = run(db, "SELECT thinkthen_warm('q', 'e')")
structured = run(db, "SELECT thinkthen_try_details('  ', 'private evidence')")
backend_error = run(db, "SELECT thinkthen_decide('Is it red?', 'a red door')")
say(usage=usage, removed=removed, structured=structured, backend_error=backend_error)
""", environment(backend, "arm/503"))
    expect(held["usage"], "thinkthen usage: `context` is text that is not blank (retryable: no)",
           "ordinary usage")
    expect(held["removed"],
           "thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many",
           "plain removal")
    failed = json.loads(held["structured"][0][0])
    expect(failed["error"], {
        "kind": "usage",
        "message": "check the row's question and arguments, or raise the process request total when it is spent",
        "retryable": False,
    }, "structured try-details remains safe")
    expect("private evidence" in json.dumps(failed), False, "private evidence stays out")
    expect(held["backend_error"],
           "thinkthen backend: the backend answered with status 503 (retryable: yes)",
           "retryable backend")
    expect(backend.close(), 1, "only the backend failure sent")


if __name__ == "__main__":
    sys.exit(main(globals()))
