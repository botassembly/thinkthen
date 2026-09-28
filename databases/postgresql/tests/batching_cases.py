#!/usr/bin/env python3
"""Inspect the installed extension's exact saved loopback requests."""

import hashlib
import json
import pathlib
import sys


QUESTION = "Is this a complaint?"
MODEL = "jev-1.13.0"


def entries(folder: str) -> list[tuple[dict, str]]:
    found = []
    for path in pathlib.Path(folder).rglob("*.json"):
        if path.name == ".thinkthen-backend.json":
            continue
        source = path.read_text()
        start = source.index('"request":') + len('"request":')
        start += len(source[start:]) - len(source[start:].lstrip())
        request, length = json.JSONDecoder().raw_decode(source[start:])
        raw = source[start:start + length]
        entry = json.loads(source)
        digest = hashlib.sha256(("systemone\n" + entry["url"] + "\n" + raw).encode()).hexdigest()
        assert path.stem == digest, (path.name, digest)
        assert entry["request"] == request
        found.append((request, raw))
    return found


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
    if mode == "packed":
        expected = {body(["b", "a"]), body(["c", "d"])}
        assert len(saved) == 2 and {raw for _, raw in saved} == expected, saved
    elif mode == "one_of_packed":
        expected = {body(["b", "a"]), body(["c", "d"])}
        assert len(saved) == 1 and saved[0][1] in expected, saved
    elif mode == "context":
        expected = {body(["b", "a"], "shared reference")}
        assert len(saved) == 1 and {raw for _, raw in saved} == expected, saved
    else:
        raise ValueError(f"unknown batching proof: {mode}")
    print(f"pass {mode}")


if __name__ == "__main__":
    main()
