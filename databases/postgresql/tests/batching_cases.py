#!/usr/bin/env python3
"""Inspect the installed extension's saved loopback answers.

A recording keeps one store row per question (ADR 0111 section 3), so each
proof compares the stored question keys with the keys of the bodies the
packing would send. The send counts in check.sh pin the packing itself.
"""

import json
import pathlib
import sqlite3
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance" / "children"))
from portable import question_keys  # noqa: E402  the ADR 0111 question key


QUESTION = "Is this a complaint?"
MODEL = "jev-1.13.0"


def entries(folder: str) -> list[tuple[str, str]]:
    """Each stored answer's URL and hex question key."""
    store = pathlib.Path(folder) / "thinkthen.sqlite"
    if not store.is_file():
        return []
    with sqlite3.connect(store) as connection:
        found = connection.execute("SELECT url, lower(hex(key)) FROM answers").fetchall()
    connection.close()
    return found


def keys(saved: list[tuple[str, str]], bodies: list[str]) -> set[str]:
    """The question keys of these bodies at the URL the store names."""
    urls = {url for url, _ in saved}
    assert len(urls) <= 1, urls
    url = next(iter(urls), "")
    return {key for body in bodies for key in question_keys(url, body)}


def body(records: list[str], shared: str | None = None) -> str:
    state = "Each question quotes the text it asks about." if shared is None else shared
    questions = {f"q{index}": {"type": "noul", "instructions":
                 f'The text is "{record}". {QUESTION}'}
                 for index, record in enumerate(records, 1)}
    return json.dumps({"state": state, "model": MODEL, "questions": questions},
                      separators=(",", ":"), ensure_ascii=False)


def main() -> None:
    mode, folder = sys.argv[1:]
    saved = entries(folder)
    stored = {key for _, key in saved}
    assert len(stored) == len(saved), saved
    if mode in ("packed", "max", "singleton"):
        # Every packing asks the same four questions, so it stores the same keys.
        assert len(saved) == 4 and stored == keys(saved, [body(["b", "a", "c", "d"])]), saved
    elif mode == "one_of_packed":
        options = [keys(saved, [body(["b", "a"])]), keys(saved, [body(["c", "d"])])]
        assert len(saved) == 2 and stored in options, saved
    elif mode == "context":
        assert len(saved) == 2 and stored == keys(saved, [body(["b", "a"], "shared reference")]), saved
    elif mode == "warm":
        wanted = keys(saved, [body(["b", "a"], "first"), body(["c", "d"], "second")])
        assert len(saved) == 4 and stored == wanted, saved
    else:
        raise ValueError(f"unknown batching proof: {mode}")
    print(f"pass {mode}")


if __name__ == "__main__":
    main()
