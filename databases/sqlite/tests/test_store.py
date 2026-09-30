#!/usr/bin/env python3
"""The question store's edge rows through the SQLite extension on its pinned
host (ticket 0348): replay from a read-only folder, and a store another
connection holds."""

from __future__ import annotations

import os
import pathlib
import sqlite3
import sys
import time

from helper import Backend, Child, child, environment, expect, main
from test_settings import recording_entries

ASK = """
db = connect()
{configure}
say(answer=run(db, "SELECT thinkthen_decide('Is this a complaint?', ?)", ({text!r},)))
"""


def seeded(backend: Backend, env: dict[str, str], text: str) -> None:
    """One live answer stored in the child's cache folder."""
    held = child(ASK.format(configure="", text=text), env)
    expect(held["answer"], [[1]], f"seeded answer for {text!r}")


def shape(folder: pathlib.Path) -> dict[str, tuple]:
    """Each entry's metadata a write would change; the access time is left out."""
    return {path.name: (status.st_mode, status.st_size, status.st_mtime_ns, status.st_ctime_ns, status.st_ino)
            for path in [folder, *sorted(folder.iterdir())] for status in [path.stat()]}


def test_replay_from_a_read_only_folder_answers_and_changes_nothing() -> None:
    """specification/recording.md: replay of an existing read-only directory
    changes no file or directory metadata."""
    backend = Backend()
    env = environment(backend)
    seeded(backend, env, "i want a refund")
    folder = pathlib.Path(env.pop("THINKTHEN_CACHE"))
    store = folder / "thinkthen.sqlite"
    store.chmod(0o400)
    folder.chmod(0o500)
    try:
        before = shape(folder)
        configure = f"db.execute('SELECT thinkthen_configure(?)', ('{{\"replay\":\"{folder}\"}}',))"
        held = child(ASK.format(configure=configure, text="i want a refund"), env)
        expect(held["answer"], [[1]], "the replayed answer")
        expect(shape(folder), before, "the folder and its files after replay")
    finally:
        folder.chmod(0o700)
    expect(backend.close(), 1, "only the seeding call sends")


def test_a_store_another_connection_holds_waits_then_answers() -> None:
    """A lookup waits while another connection holds the store in a write,
    then answers and stores its answer once the holder lets go."""
    backend = Backend()
    env = environment(backend)
    seeded(backend, env, "i want a refund")
    holder = sqlite3.connect(pathlib.Path(env["THINKTHEN_CACHE"]) / "thinkthen.sqlite", isolation_level=None)
    holder.execute("BEGIN EXCLUSIVE")
    waiting = Child(ASK.format(configure="say(ready=1)", text="the box came broken"), env)
    expect(waiting.read(), {"ready": 1}, "the child is at its call while the store is held")
    time.sleep(1.0)
    expect(backend.count(), 1, "no send while the store is held")
    expect([line for line in waiting.said if line.startswith("{")][1:], [], "no answer while the store is held")
    holder.execute("ROLLBACK")
    holder.close()
    expect(waiting.result()["answer"], [[1]], "the answer once the holder lets go")
    expect(backend.close(), 2, "one send after the wait")
    expect(recording_entries(env["THINKTHEN_CACHE"]), 2, "the waited answer is stored")


if __name__ == "__main__":
    os.chdir(pathlib.Path(__file__).resolve().parent)
    sys.exit(main(globals()))
