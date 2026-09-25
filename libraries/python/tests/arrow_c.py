"""Hand-built Arrow producers and consumers for the door's tests.

``Column`` hands the door one array whose buffers the test lays out, so a
test can build the malformed shapes Polars and pyarrow never make.
``Stream`` is a producer written in Python with ``ctypes``: each batch's
release drops an object from ``HELD``, so a test can see every release
happen. ``guarded`` places bytes against unreadable memory. ``take`` and
``pull`` read the door's own output through the C structs.
"""

import ctypes
import mmap

PAGE = mmap.PAGESIZE
HELD = {}
# Every producer lives as long as the process. A consumer may call its
# release callbacks after the caller has dropped it, so the ctypes
# trampolines must never be freed.
KEEP = []


class Schema(ctypes.Structure):
    pass


class Array(ctypes.Structure):
    pass


class StreamC(ctypes.Structure):
    pass


SCHEMA_RELEASE = ctypes.CFUNCTYPE(None, ctypes.POINTER(Schema))
ARRAY_RELEASE = ctypes.CFUNCTYPE(None, ctypes.POINTER(Array))
GET_SCHEMA = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.POINTER(StreamC), ctypes.POINTER(Schema))
GET_NEXT = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.POINTER(StreamC), ctypes.POINTER(Array))
GET_ERROR = ctypes.CFUNCTYPE(ctypes.c_char_p, ctypes.POINTER(StreamC))
STREAM_RELEASE = ctypes.CFUNCTYPE(None, ctypes.POINTER(StreamC))
Schema._fields_ = [
    ("format", ctypes.c_char_p), ("name", ctypes.c_char_p), ("metadata", ctypes.c_char_p),
    ("flags", ctypes.c_int64), ("n_children", ctypes.c_int64),
    ("children", ctypes.POINTER(ctypes.POINTER(Schema))), ("dictionary", ctypes.POINTER(Schema)),
    ("release", SCHEMA_RELEASE), ("private_data", ctypes.c_void_p)]
Array._fields_ = [
    ("length", ctypes.c_int64), ("null_count", ctypes.c_int64), ("offset", ctypes.c_int64),
    ("n_buffers", ctypes.c_int64), ("n_children", ctypes.c_int64),
    ("buffers", ctypes.POINTER(ctypes.c_void_p)), ("children", ctypes.POINTER(ctypes.POINTER(Array))),
    ("dictionary", ctypes.POINTER(Array)), ("release", ARRAY_RELEASE), ("private_data", ctypes.c_void_p)]
StreamC._fields_ = [
    ("get_schema", GET_SCHEMA), ("get_next", GET_NEXT), ("get_last_error", GET_ERROR),
    ("release", STREAM_RELEASE), ("private_data", ctypes.c_void_p)]

_new = ctypes.pythonapi.PyCapsule_New
_new.restype, _new.argtypes = ctypes.py_object, [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_void_p]
_pointer = ctypes.pythonapi.PyCapsule_GetPointer
_pointer.restype, _pointer.argtypes = ctypes.c_void_p, [ctypes.py_object, ctypes.c_char_p]
_libc = ctypes.CDLL(None, use_errno=True)
_libc.mprotect.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_int]


def utf8(texts):
    """The offsets and values buffers of a Utf8 column."""
    ends, values = [0], b""
    for text in texts:
        values += text.encode()
        ends.append(len(values))
    return b"".join(end.to_bytes(4, "little", signed=True) for end in ends), values


def address(data):
    """A buffer that outlives the call, and its address."""
    held = ctypes.create_string_buffer(data, max(len(data), 1))
    return held, ctypes.addressof(held)


def guarded(data, reserve=64 << 20):
    """``data`` placed to end at the one readable page of a region whose
    other pages are unreadable. Returns the region and the data's address."""
    region = mmap.mmap(-1, 2 * PAGE + reserve)
    base = ctypes.addressof(ctypes.c_char.from_buffer(region))
    start = base + 2 * PAGE - len(data)
    ctypes.memmove(start, data, len(data))
    for first, length in ((base, PAGE), (base + 2 * PAGE, reserve)):
        assert _libc.mprotect(first, length, 0) == 0, ctypes.get_errno()
    return region, start


class Column:
    """One array of ``form`` over ``buffers``, each an address or None."""

    def __init__(self, form, buffers, length, n_buffers=None, table=None, offset=0):
        KEEP.append(self)
        self.pointers = (ctypes.c_void_p * max(len(buffers), 1))(*buffers)
        self.form, self.length, self.offset = form.encode(), length, offset
        self.count = len(buffers) if n_buffers is None else n_buffers
        self.table = table if table is not None else ctypes.addressof(self.pointers)
        self.schema_release = SCHEMA_RELEASE(lambda made: setattr(made.contents, "release", SCHEMA_RELEASE()))
        self.array_release = ARRAY_RELEASE(lambda made: setattr(made.contents, "release", ARRAY_RELEASE()))

    def __arrow_c_array__(self, requested_schema=None):
        self.structs = (Schema(format=self.form, name=b"", release=self.schema_release),
                        Array(length=self.length, offset=self.offset, n_buffers=self.count,
                              buffers=ctypes.cast(self.table, ctypes.POINTER(ctypes.c_void_p)),
                              release=self.array_release))
        schema, array = (ctypes.addressof(one) for one in self.structs)
        return _new(schema, b"arrow_schema", None), _new(array, b"arrow_array", None)


class Stream:
    """A Utf8 stream of ``batches`` one-row batches. Every batch and the
    stream hold a key in ``HELD``, and each release pops its key."""

    def __init__(self, batches, text="note"):
        KEEP.append(self)
        self.left, self.name = batches, id(self)
        self.offsets, self.values = utf8([text])
        self.table = (ctypes.c_void_p * 3)(None, ctypes.cast(self.offsets, ctypes.c_void_p),
                                           ctypes.cast(self.values, ctypes.c_void_p))
        HELD[(self.name, "stream")] = object()
        self.calls = (GET_SCHEMA(self.schema), GET_NEXT(self.next), GET_ERROR(lambda _: None),
                      STREAM_RELEASE(self.release), ARRAY_RELEASE(self.release_batch),
                      SCHEMA_RELEASE(lambda made: setattr(made.contents, "release", SCHEMA_RELEASE())))

    def schema(self, _stream, out):
        out.contents.format, out.contents.name, out.contents.release = b"u", b"", self.calls[5]
        return 0

    def next(self, _stream, out):
        batch = Array(release=ARRAY_RELEASE())
        if self.left:
            self.left -= 1
            HELD[(self.name, self.left)] = object()
            batch = Array(length=1, n_buffers=3, buffers=self.table, release=self.calls[4],
                          private_data=self.left)
        ctypes.memmove(out, ctypes.addressof(batch), ctypes.sizeof(Array))
        return 0

    def release_batch(self, batch):
        HELD.pop((self.name, batch.contents.private_data or 0))
        batch.contents.release = ARRAY_RELEASE()

    def release(self, stream):
        HELD.pop((self.name, "stream"))
        stream.contents.release = STREAM_RELEASE()

    def __arrow_c_stream__(self, requested_schema=None):
        self.struct = StreamC(*self.calls[:4])
        return _new(ctypes.addressof(self.struct), b"arrow_array_stream", None)


def pull(capsule):
    """The stream moved out of a capsule, its schema, and every batch, as
    ctypes structs the caller now owns."""
    inside = ctypes.cast(_pointer(capsule, b"arrow_array_stream"), ctypes.POINTER(StreamC)).contents
    stream = StreamC()
    ctypes.memmove(ctypes.byref(stream), ctypes.byref(inside), ctypes.sizeof(StreamC))
    inside.release = STREAM_RELEASE()
    schema, batches = Schema(), []
    assert stream.get_schema(ctypes.byref(stream), ctypes.byref(schema)) == 0
    while True:
        batch = Array()
        assert stream.get_next(ctypes.byref(stream), ctypes.byref(batch)) == 0
        if not batch.release:
            return stream, schema, batches
        batches.append(batch)


def take(capsule, name):
    """The struct inside an array or schema capsule."""
    kind = Array if name == b"arrow_array" else Schema
    return ctypes.cast(_pointer(capsule, name), ctypes.POINTER(kind)).contents
