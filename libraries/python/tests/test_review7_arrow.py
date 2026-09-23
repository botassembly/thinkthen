"""Review 7: the frame annotate returns follows the C data interface.

R4-15: a consumer may move a child array out of a batch and release the
parent; the moved child must stay valid until its own release. Every
child shared the parent's single owner, so the parent's release freed
the moved child's buffers. The same row found the output schema dropped
every metadata pointer, so a Polars Enum came back Categorical and a
field's own keys were lost.

Run: ENGINE_NULL=1 python -m pytest tests/test_review7_arrow.py -q
"""

import os
import subprocess
import sys
from pathlib import Path

import polars as pl
import pyarrow as pa

import thinkthen as tt
from thinkthen import _thinkthen as native

FORM = str(Path(__file__).parent / "fixture" / "form.json")

# The mover runs in its own process so glibc fills freed memory with 0xa5
# (MALLOC_PERTURB_), which makes a read of freed memory visible.
MOVER = r"""
import ctypes as C, sys
import polars as pl
from thinkthen import _thinkthen as native

class Array(C.Structure):
    pass
Array._fields_ = [("length", C.c_int64), ("null_count", C.c_int64), ("offset", C.c_int64),
    ("n_buffers", C.c_int64), ("n_children", C.c_int64), ("buffers", C.POINTER(C.c_void_p)),
    ("children", C.POINTER(C.POINTER(Array))), ("dictionary", C.c_void_p),
    ("release", C.c_void_p), ("private_data", C.c_void_p)]
class Stream(C.Structure):
    _fields_ = [("get_schema", C.c_void_p), ("get_next", C.c_void_p),
        ("get_last_error", C.c_void_p), ("release", C.c_void_p), ("private_data", C.c_void_p)]
NEXT = C.CFUNCTYPE(C.c_int, C.POINTER(Stream), C.POINTER(Array))
RELEASE = C.CFUNCTYPE(None, C.POINTER(Array))

df = pl.DataFrame({"body": [f"order {i}: please refund" for i in range(2000)],
                   "n": list(range(2000))})
capsule = native.annotate_stream(sys.argv[1], df, "body").__arrow_c_stream__()
C.pythonapi.PyCapsule_GetPointer.restype = C.c_void_p
C.pythonapi.PyCapsule_GetPointer.argtypes = [C.py_object, C.c_char_p]
stream = C.cast(C.pythonapi.PyCapsule_GetPointer(capsule, b"arrow_array_stream"), C.POINTER(Stream))
batch = Array()
assert NEXT(stream.contents.get_next)(stream, C.byref(batch)) == 0 and batch.release
moved = []
for place in range(batch.n_children):
    slot = batch.children[place].contents
    moved.append(Array.from_buffer_copy(slot))
    slot.release = None  # the move: the parent's slot now reads released
before = [[child.buffers[i] for i in range(child.n_buffers)] for child in moved]
numbers = C.string_at(moved[1].buffers[1], 16)
RELEASE(batch.release)(C.byref(batch))
junk = [C.create_string_buffer(4096) for _ in range(256)]
after = [[child.buffers[i] for i in range(child.n_buffers)] for child in moved]
assert after == before, "a moved child's buffer table changed after the parent's release"
assert C.string_at(moved[1].buffers[1], 16) == numbers, "a moved child's values changed"
for child in moved:
    assert child.release and child.private_data, "a moved child has no owner of its own"
    RELEASE(child.release)(C.byref(child))
    assert not child.release, "a released child still names its release"
print("moved", len(moved))
"""


def test_a_moved_child_outlives_its_parents_release():
    env = dict(os.environ, MALLOC_PERTURB_="165", ENGINE_NULL="1", THINKTHEN_NULL="1")
    done = subprocess.run(
        [sys.executable, "-c", MOVER, FORM],
        env=env, stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=120,
    )
    assert done.returncode == 0, (done.returncode, done.stderr[-2000:])
    assert done.stdout.startswith("moved "), done.stdout


def test_an_enum_column_comes_back_an_enum():
    df = pl.DataFrame({
        "body": ["please refund order 1", "hello there"],
        "enum": pl.Series(["x", "y"], dtype=pl.Enum(["x", "y", "z"])),
    })
    out = tt.annotate(FORM, df, on="body")
    assert out.schema["enum"] == pl.Enum(["x", "y", "z"])
    assert out["enum"].to_list() == ["x", "y"]


def test_field_and_schema_metadata_ride_through():
    src = pa.table(
        {"body": pa.array(["refund", "hi"]), "tag": pa.array([1, 2])},
        schema=pa.schema(
            [pa.field("body", pa.string()), pa.field("tag", pa.int64(), metadata={b"unit": b"cm"})],
            metadata={b"origin": b"probe"},
        ),
    )
    back = pa.RecordBatchReader.from_stream(native.annotate_stream(FORM, src, "body")).read_all()
    assert back.schema.field("tag").metadata == {b"unit": b"cm"}
    assert back.schema.metadata == {b"origin": b"probe"}
