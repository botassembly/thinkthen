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


# Review 7, second pass: the reader's check must not lean on a system call a
# sandbox can refuse. This child process forbids process_vm_readv (EPERM
# through seccomp), then hands the reader a Utf8 column whose offsets
# declare 200 bytes over three that end at a PROT_NONE page. A reader whose
# fallback cannot see a protected page reads into it and dies.
GUARDED = r"""
import ctypes as C, mmap, struct
import pyarrow as pa
import thinkthen as tt

libc = C.CDLL(None, use_errno=True)
class Filter(C.Structure):
    _fields_ = [("code", C.c_uint16), ("jt", C.c_uint8), ("jf", C.c_uint8), ("k", C.c_uint32)]
class Program(C.Structure):
    _fields_ = [("len", C.c_uint16), ("filter", C.POINTER(Filter))]
NR_PROCESS_VM_READV, AUDIT_ARCH_X86_64 = 310, 0xC000003E
# Allow every other architecture and call; answer process_vm_readv with EPERM.
rules = (Filter * 7)(
    Filter(0x20, 0, 0, 4), Filter(0x15, 1, 0, AUDIT_ARCH_X86_64), Filter(0x06, 0, 0, 0x7FFF0000),
    Filter(0x20, 0, 0, 0), Filter(0x15, 0, 1, NR_PROCESS_VM_READV), Filter(0x06, 0, 0, 0x00050000 | 1),
    Filter(0x06, 0, 0, 0x7FFF0000))
program = Program(7, rules)
assert libc.prctl(38, 1, 0, 0, 0) == 0 and libc.prctl(22, 2, C.byref(program), 0, 0) == 0
# The filter holds: the call now fails with EPERM.
assert libc.syscall(NR_PROCESS_VM_READV, 0, 0, 0, 0, 0, 0) == -1 and C.get_errno() == 1

page = mmap.PAGESIZE
region = mmap.mmap(-1, 2 * page)
base = C.addressof(C.c_char.from_buffer(region))
assert libc.mprotect(C.c_void_p(base + page), page, 0) == 0
C.memmove(base + page - 3, b"abc", 3)
# pyarrow checks offsets against the size it is told; it never reads the bytes.
values = pa.foreign_buffer(base + page - 3, 200, base=region)
column = pa.StringArray.from_buffers(1, pa.py_buffer(struct.pack("<2i", 0, 200)), values)
try:
    tt.decide_many("Is this a refund?", pa.chunked_array([column]))
    print("answered")
except tt.UsageError as refused:
    print("refused:", refused)
"""


def test_a_guard_page_is_refused_without_process_vm_readv():
    import platform

    import pytest

    if sys.platform != "linux" or platform.machine() != "x86_64":
        pytest.skip("the seccomp filter here is written for Linux on x86_64")
    env = dict(os.environ, ENGINE_NULL="1", THINKTHEN_NULL="1")
    done = subprocess.run(
        [sys.executable, "-c", GUARDED],
        env=env, stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=120,
    )
    assert done.returncode == 0, (done.returncode, done.stderr[-2000:])
    assert done.stdout.strip() == "refused: the column's buffers declare bytes this process cannot read"


# Review 7, third pass: a frame this surface builds whole reads no
# producer memory, so it must not need the memory map. This child process
# forbids every file open (EACCES through seccomp) after its imports. The
# frame door still builds, and a column read gets the map refusal instead
# of an unchecked read.
NO_OPEN = r"""
import ctypes as C
import polars as pl
import pyarrow as pa
import thinkthen as tt
from thinkthen import _thinkthen as native

# One call first, so every module the door imports lazily is loaded
# before the filter blocks file opens.
tt.decide_many("Is this a refund?", pa.chunked_array([pa.array(["hi"])]))
native._probe_frame_rebuild(pl.DataFrame())
libc = C.CDLL(None, use_errno=True)
class Filter(C.Structure):
    _fields_ = [("code", C.c_uint16), ("jt", C.c_uint8), ("jf", C.c_uint8), ("k", C.c_uint32)]
class Program(C.Structure):
    _fields_ = [("len", C.c_uint16), ("filter", C.POINTER(Filter))]
ALLOW, EACCES = 0x7FFF0000, 0x00050000 | 13
# open (2), openat (257), and openat2 (437) answer EACCES on x86_64.
rules = (Filter * 9)(
    Filter(0x20, 0, 0, 4), Filter(0x15, 1, 0, 0xC000003E), Filter(0x06, 0, 0, ALLOW),
    Filter(0x20, 0, 0, 0), Filter(0x15, 2, 0, 2), Filter(0x15, 1, 0, 257), Filter(0x15, 0, 1, 437),
    Filter(0x06, 0, 0, EACCES), Filter(0x06, 0, 0, ALLOW))
assert libc.prctl(38, 1, 0, 0, 0) == 0 and libc.prctl(22, 2, C.byref(Program(9, rules)), 0, 0) == 0
try:
    open("/proc/self/maps").close()
    raise SystemExit("the filter did not hold")
except PermissionError:
    pass
native._probe_frame_rebuild(pl.DataFrame())
print("table built")
try:
    tt.decide_many("Is this a refund?", pa.chunked_array([pa.array(["hi"])]))
    print("answered")
except tt.UsageError as refused:
    print("refused:", refused)
"""


def test_a_table_builds_without_the_memory_map():
    import platform

    import pytest

    if sys.platform != "linux" or platform.machine() != "x86_64":
        pytest.skip("the seccomp filter here is written for Linux on x86_64")
    env = dict(os.environ, ENGINE_NULL="1", THINKTHEN_NULL="1")
    done = subprocess.run(
        [sys.executable, "-c", NO_OPEN],
        env=env, stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=120,
    )
    assert done.returncode == 0, (done.returncode, done.stderr[-2000:])
    assert done.stdout.splitlines() == [
        "table built",
        "refused: this process cannot read its memory map (/proc/self/maps), "
        "so the Arrow door cannot check a column before it reads it",
    ]
