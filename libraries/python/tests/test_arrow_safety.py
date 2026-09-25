"""The Arrow door's memory rules, each in a child that must exit 0.

A malformed column is refused with a pinned sentence, never read past its
buffers. What the door hands out releases cleanly and keeps a moved child
alive. The shapes the Rust unit tests pin are not repeated here; these
cross the whole door from Python.
"""

import pathlib

from conftest import child_env, run

TESTS = str(pathlib.Path(__file__).resolve().parent)
UNREADABLE = "the column's buffers declare bytes this process cannot read"
SETUP = f"""
    import ctypes, sys
    sys.path.insert(0, {TESTS!r})
    import polars as pl, thinkthen as tt
    from arrow_c import Column, address, guarded, pull, take, utf8
    engine = tt.Engine(cache=False)
    late = tt.question(decide="Is it late?")
    def said(call):
        try:
            call()
            print("answered")
        except tt.ThinkThenError as error:
            print(type(error).__name__, error)
"""


def test_extents_past_readable_memory_are_refused(backend, tmp_path):
    """R3-4 and R7-2: offsets ``60000000, 60000001`` over a three-byte
    buffer, a view column whose buffer count is one short against a guard
    page, and a view whose sizes overrun its data are refused. Regression:
    ``Readable`` answering yes reads unmapped memory and kills the child."""
    printed = run(SETUP + """
    keep, offsets = address((60000000).to_bytes(4, "little") + (60000001).to_bytes(4, "little"))
    region, values = guarded(b"abc")
    said(lambda: engine.decide(late, Column("u", [None, offsets, values], 1)))
    view = (40).to_bytes(4, "little") + b"head" + bytes(4) + bytes(4)
    held, views = address(view)
    data, data_at = address(b"x" * 100)
    table, table_at = guarded(bytes(8) + views.to_bytes(8, "little") + data_at.to_bytes(8, "little"))
    said(lambda: engine.decide(late, Column("vu", [], 1, n_buffers=4, table=table_at)))
    tail, tail_at = guarded(b"y" * 20)
    view = (13).to_bytes(4, "little") + b"head" + bytes(4) + (10).to_bytes(4, "little")
    held2, views2 = address(view)
    sizes, sizes_at = address((200).to_bytes(8, "little"))
    said(lambda: engine.decide(late, Column("vu", [None, views2, tail_at, sizes_at], 1)))
    keep2, offsets = address((0).to_bytes(4, "little") + (3).to_bytes(4, "little"))
    print(engine.decide(late, Column("u", [None, offsets, values], 1)) == [engine.decide(late, "abc")])
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == 3 * [f"UsageError {UNREADABLE}"] + ["True"]
    assert backend.count() == 2


def test_refused_inputs_release_their_batches(backend, tmp_path):
    """R2-17: 200 refused 8 MB number columns in a row each release their
    stream, so peak resident memory grows under 8 MiB. Regression: a
    refusal path that skips the release keeps every column."""
    printed = run(SETUP + """
    import resource
    def once(n):
        said(lambda: engine.decide(late, pl.int_range(n, n + 1_000_000, eager=True)))
    for n in range(5):
        once(n)
    before = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    for n in range(200):
        once(n)
    grown = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss - before
    print(grown < 8 * 1024, grown)
    """, child_env(backend, tmp_path))
    lines = printed.splitlines()
    assert set(lines[:-1]) == {"UsageError the column's Arrow format is 'l', not text"}
    assert lines[-1].startswith("True"), lines[-1]
    assert backend.count() == 0


def test_what_the_door_hands_out_releases_and_keeps_moved_children(backend, tmp_path):
    """R1-12 and R4-15 (a): each schema and array the door hands out
    clears its release pointer, pyarrow imports and frees the output, and a
    child moved out of a batch stays readable after the batch's release.
    Freed memory reads as 0xa5 under ``MALLOC_PERTURB_``."""
    printed = run(SETUP + """
    import pyarrow as pa
    import json
    spec = {"version": 1, "questions": {"team": {"choose": "Which team?", "options": ["a", "b"]}}}
    form = tt._thinkthen._QuestionSet._from_json(json.dumps(spec))
    rows = [f"note {n}" for n in range(50)]
    frame = pl.DataFrame({"body": rows})
    wanted = [one["team"] for one in engine.annotate(spec, rows)]
    out = tt._thinkthen._annotate_frame(engine._engine, form, frame, "body", None, None)
    stream, schema, [batch] = pull(out.__arrow_c_stream__())
    child = batch.children[1].contents
    moved, moved_schema = type(child)(), type(schema.children[1].contents)()
    ctypes.memmove(ctypes.byref(moved), ctypes.byref(child), ctypes.sizeof(moved))
    ctypes.memmove(ctypes.byref(moved_schema), schema.children[1], ctypes.sizeof(moved_schema))
    child.release = type(child.release)()
    schema.children[1].contents.release = type(schema.release)()
    batch.release(ctypes.byref(batch))
    schema.release(ctypes.byref(schema))
    stream.release(ctypes.byref(stream))
    print(bool(batch.release), bool(schema.release), bool(stream.release))
    reread = pa.Array._import_from_c(ctypes.addressof(moved), ctypes.addressof(moved_schema))
    print(reread.to_pylist() == wanted)
    column = tt._thinkthen._annotate_frame(engine._engine, form, frame, "body", None, None)
    table = pa.table(column)
    print(table.column("team").to_pylist() == wanted)
    del table
    answers = engine.choose("Which team?", pl.Series(rows), options=["a", "b"])
    print(answers.to_list() == wanted)
    """, child_env(backend, tmp_path, MALLOC_PERTURB_="165"))
    assert printed.splitlines() == ["False False False", "True", "True", "True"]


def test_the_worker_reads_the_producers_own_buffers(backend, tmp_path):
    """Change 3: ``_arrow_probe`` reads the buffer addresses on the worker,
    from the batches it holds, and they equal the ones Polars hands out.
    Regression: a copy of the texts before the hand-off moves them."""
    printed = run(SETUP + """
    series = pl.Series([f"a text long enough to leave the view {n}" for n in range(1000)])
    _, _, [batch] = pull(series.__arrow_c_stream__())
    polars_sees = (batch.buffers[2], batch.buffers[1], batch.length)
    batch.release(ctypes.byref(batch))
    print(tt._thinkthen._arrow_probe(series) == polars_sees)
    """, child_env(backend, tmp_path))
    assert printed.strip() == "True"
