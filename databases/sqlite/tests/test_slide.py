#!/usr/bin/env python3
"""The slide runs as drawn in the CLI built from the amalgamation.

`tests/slide.sql` is the slide unchanged, with its `.load ./thinkthen`
line, so the CLI runs in a folder that holds the library under that name.
The warm judges five rows, the WHERE keeps the rows the generic arm says
yes to, and the queries after the warm send nothing new.
"""

from __future__ import annotations

import pathlib
import shutil
import subprocess
import sys

from helper import CLI, HERE, LIB, Backend, environment, expect, main


def test_the_slide_runs_as_drawn() -> None:
    backend = Backend()
    env = environment(backend)
    folder = pathlib.Path(env["SCRATCH"])
    shutil.copy(LIB, folder / "thinkthen.so")
    done = subprocess.run([CLI, ":memory:"], stdin=(HERE / "slide.sql").open(), cwd=folder, env=env,
                          capture_output=True, text=True, timeout=60, check=False)
    rows = ["i want a refund now", "good morning", "refund, please", "maybe later", "see you"]
    expect((done.stdout, done.stderr), ("5\n" + "".join(f"{at}|{body}\n" for at, body in enumerate(rows, 1)) + "5\n", ""), "the slide")
    expect(backend.close(), 5, "sends: the warm's five, and none after it")


if __name__ == "__main__":
    sys.exit(main(globals()))
