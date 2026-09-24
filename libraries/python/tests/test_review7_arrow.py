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


# Review 7, fourth pass: a file mapping is listed readable to its end, but
# a page past the end of its file raises SIGBUS when touched. This child
# maps a pyarrow IPC file, truncates the file to half, and reads a 10-row
# slice from each end: the head still lies inside the file and answers,
# and the tail now lies past its end and is refused. A second mapping of
# two pages over a one-page memfd holds a format string past its end.
TRUNCATED = r"""
import ctypes as C, os, sys
import pyarrow as pa, pyarrow.ipc as ipc
import thinkthen as tt

path = sys.argv[1]
rows = ["please refund %06d " % i + "x" * 200 for i in range(20000)]
with ipc.new_file(path, pa.schema([("t", pa.string())])) as out:
    out.write_table(pa.table({"t": rows}))
column = ipc.open_file(pa.memory_map(path, "r")).read_all().column("t").chunk(0)
os.truncate(path, os.path.getsize(path) // 2)
for name, at in (("head", 0), ("tail", 19990)):
    try:
        print(name, "answered", len(tt.decide_many("Is this a refund?", pa.chunked_array([column.slice(at, 10)]))))
    except tt.UsageError as refused:
        print(name, "refused:", refused)

libc = C.CDLL(None, use_errno=True)
libc.mmap.restype = C.c_void_p
libc.mmap.argtypes = [C.c_void_p, C.c_size_t, C.c_int, C.c_int, C.c_int, C.c_long]
fd = libc.memfd_create(b"past-end", 0)
os.ftruncate(fd, 4096)
past = libc.mmap(None, 8192, 1, 1, fd, 0) + 4096

class Schema(C.Structure):
    _fields_ = [("format", C.c_void_p), ("name", C.c_char_p), ("metadata", C.c_void_p),
        ("flags", C.c_int64), ("n_children", C.c_int64), ("children", C.c_void_p),
        ("dictionary", C.c_void_p), ("release", C.c_void_p), ("private_data", C.c_void_p)]

class PastEnd:
    def __arrow_c_stream__(self, requested_schema=None):
        stream = pa.chunked_array([pa.array(["please refund"])])
        capsule = stream.__arrow_c_stream__()
        get = C.pythonapi.PyCapsule_GetPointer
        get.restype, get.argtypes = C.c_void_p, [C.py_object, C.c_char_p]
        pointer = get(capsule, b"arrow_array_stream")
        GET_SCHEMA = C.CFUNCTYPE(C.c_int, C.c_void_p, C.POINTER(Schema))
        inner = GET_SCHEMA(C.c_void_p.from_address(pointer).value)
        def get_schema(stream_pointer, out):
            got = inner(stream_pointer, out)
            out.contents.format = past
            return got
        self.keep = (stream, GET_SCHEMA(get_schema))
        C.c_void_p.from_address(pointer).value = C.cast(self.keep[1], C.c_void_p).value
        return capsule

try:
    tt.decide_many("Is this a refund?", PastEnd())
    print("format answered")
except tt.UsageError as refused:
    print("format refused:", refused)
"""


def test_a_mapping_past_the_end_of_its_file_is_refused(tmp_path):
    if sys.platform != "linux":
        import pytest

        pytest.skip("memfd and the truncated-mapping signal are Linux behavior")
    env = dict(os.environ, ENGINE_NULL="1", THINKTHEN_NULL="1")
    done = subprocess.run(
        [sys.executable, "-c", TRUNCATED, str(tmp_path / "column.arrow")],
        env=env, stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=120,
    )
    assert done.returncode == 0, (done.returncode, done.stderr[-2000:])
    assert done.stdout.splitlines() == [
        "head answered 10",
        "tail refused: the column's buffers declare bytes this process cannot read",
        "format refused: an Arrow format, name, or error string has no end within 64 KiB of readable memory",
    ]


# Review 7, fourth pass: the reader read /proc/self/maps once a batch, so a
# call cost chunks times map lines. A 100-chunk column in a process with
# 30,000 mappings took 1.3 s a call. This child adds 20,000 mappings and
# compares a 100-chunk call with a 1-chunk call, the best of five each.
# One snapshot a call keeps the two close; one a batch makes the 100-chunk
# call dozens of times slower.
SNAPSHOTS = r"""
import ctypes as C, time
import pyarrow as pa
import thinkthen as tt

libc = C.CDLL(None)
libc.mmap.restype = C.c_void_p
libc.mmap.argtypes = [C.c_void_p, C.c_size_t, C.c_int, C.c_int, C.c_int, C.c_long]
# Alternate the protection so the kernel cannot merge neighbours.
held = [libc.mmap(None, 4096, 1 if place % 2 else 3, 0x22, -1, 0) for place in range(20000)]

def best(column):
    tt.decide_many("Is this a refund?", column)
    times = []
    for _ in range(5):
        start = time.perf_counter()
        tt.decide_many("Is this a refund?", column)
        times.append(time.perf_counter() - start)
    return min(times)

one = best(pa.chunked_array([pa.array(["please refund %d" % place for place in range(100)])]))
many = best(pa.chunked_array([pa.array(["please refund %d" % place]) for place in range(100)]))
print(f"{many / one:.1f} {one * 1000:.2f} {many * 1000:.2f}")
"""


def test_a_many_chunk_column_reads_the_memory_map_once():
    if sys.platform != "linux":
        import pytest

        pytest.skip("the memory-map snapshot is the Linux check")
    env = dict(os.environ, ENGINE_NULL="1", THINKTHEN_NULL="1")
    done = subprocess.run(
        [sys.executable, "-c", SNAPSHOTS],
        env=env, stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=300,
    )
    assert done.returncode == 0, (done.returncode, done.stderr[-2000:])
    ratio, one, many = done.stdout.split()
    assert float(ratio) < 10, f"100 chunks took {many} ms, one chunk {one} ms"
