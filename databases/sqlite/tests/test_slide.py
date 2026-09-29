#!/usr/bin/env python3
"""The slide runs as drawn in the CLI built from the amalgamation.

The CLI runs in a folder holding the extension under its load name. The
keyed call judges five rows once; the second query reuses connection rows.
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
    expect((done.stdout, done.stderr), ("".join(f"{at}|{body}\n" for at, body in enumerate(rows, 1)) + "5\n", ""), "the slide")
    expect(backend.close(), 1, "one packed send and one connection-store reuse")


if __name__ == "__main__":
    sys.exit(main(globals()))
