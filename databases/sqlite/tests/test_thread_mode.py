#!/usr/bin/env python3
"""A single-thread SQLite host and the question store (ticket 0304 slice 3b).

The store opens its connection on the pipeline's thread, which a SQLite built
with SQLITE_THREADSAFE=0 forbids. The extension refuses a named cache, record
or replay folder on such a host with its own sentence, and a call that names
no folder still answers. check.sh builds the single-thread host on Linux.
"""

from __future__ import annotations

import os
import pathlib
import sys

from helper import Backend, NotRun, child, environment, expect, main

SINGLE = os.environ.get("THINKTHEN_SQLITE_SINGLE_THREAD", "")
REFUSED = ("thinkthen usage: a cache, record or replay folder needs a thread-safe SQLite, and this host's "
           "SQLite is single-threaded; name no folder, or load thinkthen into a thread-safe SQLite (retryable: no)")
ASK = "SELECT thinkthen_decide('Is this a refund?', 'Please refund me.')"


def single_host(backend: Backend, **extra: str) -> dict[str, str]:
    """A child on the single-thread library, with no folder unless named."""
    if not (pathlib.Path(SINGLE) / "libsqlite3.so.0").is_file():
        raise NotRun("no single-thread SQLite host; databases/sqlite/check.sh builds one on Linux")
    env = environment(backend, LD_LIBRARY_PATH=SINGLE)
    del env["THINKTHEN_CACHE"]
    env.update(extra)
    return env


def test_a_single_thread_host_answers_with_no_folder() -> None:
    backend = Backend()
    result = child(f"db = connect()\n"
                   f"say(single=run(db, \"SELECT sqlite_compileoption_used('THREADSAFE=0')\"), answer=run(db, {ASK!r}))\n",
                   single_host(backend))
    expect(result["single"], [[1]], "the child runs on the single-thread library")
    expect(isinstance(result["answer"], list), True, f"a call with no folder answers: {result['answer']!r}")
    expect(backend.close(), 1, "the call sends one request")


def test_a_single_thread_host_refuses_every_named_folder_and_sends_nothing() -> None:
    """Each named folder, from the settings or the environment, is refused."""
    for name in ("cache", "record", "replay"):
        backend = Backend()
        env = single_host(backend)
        folder = pathlib.Path(env["SCRATCH"]) / name
        result = child(f"db = connect()\n"
                       f"say(configured=run(db, \"SELECT thinkthen_configure(json_object('{name}', ?))\", ({str(folder)!r},)))\n",
                       env)
        expect(result["configured"], REFUSED, f"configure naming a {name} folder")
        expect(folder.exists(), False, f"the refused {name} folder is not created")
        expect(backend.close(), 0, f"a refused {name} folder sends nothing")
    backend = Backend()
    env = single_host(backend)
    env["THINKTHEN_CACHE"] = str(pathlib.Path(env["SCRATCH"]) / "cache")
    result = child(f"db = connect()\nsay(answer=run(db, {ASK!r}))\n", env)
    expect(result["answer"], REFUSED, "THINKTHEN_CACHE on a single-thread host")
    expect(pathlib.Path(env["THINKTHEN_CACHE"]).exists(), False, "the refused cache folder is not created")
    expect(backend.close(), 0, "a refused THINKTHEN_CACHE sends nothing")


if __name__ == "__main__":
    sys.exit(main(dict(globals())))
