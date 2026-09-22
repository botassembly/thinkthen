"""A frame stream whose struct root carries the slice offset.

The Arrow columnar format lets a producer slice a struct array by setting
the offset and length on the root and leaving the children whole; a reader
must apply the parent's offset when it indexes the children. pyarrow and
polars propagate slices into the children instead, so this layout is
assembled by hand: pyarrow mints the schema and the child arrays, and this
class re-cuts the root array so its offset and length name the caller's
rows while the children still carry every backing row.

The review of 2026-09-22 found the shim answering about the backing rows
instead of the caller's slice; a test over this stream pins the fix. The
array and schema release pointers are C-level no-ops (a libc function that
ignores its argument), so nothing here holds a Python closure that the
interpreter's shutdown could free while the shim still calls it.
"""

import ctypes

class ArrowSchema(ctypes.Structure):
    pass


class ArrowArray(ctypes.Structure):
    pass


class ArrowArrayStream(ctypes.Structure):
    pass


PFN_SCHEMA_RELEASE = ctypes.CFUNCTYPE(None, ctypes.POINTER(ArrowSchema))
PFN_ARRAY_RELEASE = ctypes.CFUNCTYPE(None, ctypes.POINTER(ArrowArray))
PFN_GET_SCHEMA = ctypes.CFUNCTYPE(
    ctypes.c_int, ctypes.POINTER(ArrowArrayStream), ctypes.POINTER(ArrowSchema)
)
PFN_GET_NEXT = ctypes.CFUNCTYPE(
    ctypes.c_int, ctypes.POINTER(ArrowArrayStream), ctypes.POINTER(ArrowArray)
)
PFN_GET_LAST_ERROR = ctypes.CFUNCTYPE(ctypes.c_char_p, ctypes.POINTER(ArrowArrayStream))
PFN_STREAM_RELEASE = ctypes.CFUNCTYPE(None, ctypes.POINTER(ArrowArrayStream))

ArrowSchema._fields_ = [
    ("format", ctypes.c_char_p),
    ("name", ctypes.c_char_p),
    ("metadata", ctypes.c_char_p),
    ("flags", ctypes.c_int64),
    ("n_children", ctypes.c_int64),
    ("children", ctypes.POINTER(ctypes.POINTER(ArrowSchema))),
    ("dictionary", ctypes.POINTER(ArrowSchema)),
    ("release", PFN_SCHEMA_RELEASE),
    ("private_data", ctypes.c_void_p),
]
ArrowArray._fields_ = [
    ("length", ctypes.c_int64),
    ("null_count", ctypes.c_int64),
    ("offset", ctypes.c_int64),
    ("n_buffers", ctypes.c_int64),
    ("n_children", ctypes.c_int64),
    ("buffers", ctypes.POINTER(ctypes.c_void_p)),
    ("children", ctypes.POINTER(ctypes.POINTER(ArrowArray))),
    ("dictionary", ctypes.POINTER(ArrowArray)),
    ("release", PFN_ARRAY_RELEASE),
    ("private_data", ctypes.c_void_p),
]
ArrowArrayStream._fields_ = [
    ("get_schema", PFN_GET_SCHEMA),
    ("get_next", PFN_GET_NEXT),
    ("get_last_error", PFN_GET_LAST_ERROR),
    ("release", PFN_STREAM_RELEASE),
    ("private_data", ctypes.c_void_p),
]

# A libc function that ignores its argument, cast to the release shapes:
# the pointer stays valid for the whole process.
_LIBC = ctypes.CDLL(None)
_LIBC.getpid.argtypes = [ctypes.c_void_p]
_LIBC.getpid.restype = ctypes.c_int
_ARRAY_RELEASE = ctypes.cast(ctypes.cast(_LIBC.getpid, ctypes.c_void_p), PFN_ARRAY_RELEASE)
_SCHEMA_RELEASE = ctypes.cast(ctypes.cast(_LIBC.getpid, ctypes.c_void_p), PFN_SCHEMA_RELEASE)

_PyCapsule_GetPointer = ctypes.pythonapi.PyCapsule_GetPointer
_PyCapsule_GetPointer.restype = ctypes.c_void_p
_PyCapsule_GetPointer.argtypes = [ctypes.py_object, ctypes.c_char_p]
_PyCapsule_New = ctypes.pythonapi.PyCapsule_New
_PyCapsule_New.restype = ctypes.py_object
_PyCapsule_New.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_void_p]


class SlicedStructStream:
    """The rows ``offset`` to ``offset + length`` of a whole frame, the
    slice carried on the struct root and the children left whole.

    With ``record_releases=True`` the array and schema release pointers
    set flags a test can read, so a refusal can be proven to release the
    producer's batch and schema instead of leaking them.
    """

    def __init__(self, offset=2, length=3, texts=None, ids=None, record_releases=False):
        import pyarrow as pa

        self.array_released = False
        self.schema_released = False
        if record_releases:
            self._array_release = PFN_ARRAY_RELEASE(self._record_array)
            self._schema_release = PFN_SCHEMA_RELEASE(self._record_schema)
        else:
            self._array_release = _ARRAY_RELEASE
            self._schema_release = _SCHEMA_RELEASE
        texts = list(texts if texts is not None else self.all_texts)
        ids = list(ids if ids is not None else range(10, 10 + len(texts)))
        batch = pa.record_batch({"body": pa.array(texts), "id": pa.array(ids)})
        schema_capsule, array_capsule = batch.__arrow_c_array__()
        self._keeps = [batch, schema_capsule, array_capsule]
        schema_ptr = _PyCapsule_GetPointer(schema_capsule, b"arrow_schema")
        array_ptr = _PyCapsule_GetPointer(array_capsule, b"arrow_array")
        self._schema = ctypes.cast(schema_ptr, ctypes.POINTER(ArrowSchema)).contents
        whole = ctypes.cast(array_ptr, ctypes.POINTER(ArrowArray)).contents
        self._root = ArrowArray(
            length=length,
            null_count=0,
            offset=offset,
            n_buffers=whole.n_buffers,
            n_children=whole.n_children,
            buffers=whole.buffers,
            children=whole.children,
            dictionary=whole.dictionary,
            release=self._array_release,
            private_data=None,
        )
        self._emitted = False
        self._stream = None

    def _record_array(self, array):
        self.array_released = True

    def _record_schema(self, schema):
        self.schema_released = True

    all_texts = [
        "please refund order 1",
        "short note",
        "please refund order 2",
        "another line",
        "please refund order 3",
    ]
    ids = [10, 11, 12, 13, 14]

    def get_schema(self, stream, out):
        dest = out.contents
        source = self._schema
        dest.format = source.format
        dest.name = source.name
        dest.metadata = source.metadata
        dest.flags = source.flags
        dest.n_children = source.n_children
        dest.children = source.children
        dest.dictionary = source.dictionary
        dest.release = self._schema_release
        dest.private_data = None
        return 0

    def get_next(self, stream, out):
        if not self._emitted:
            self._emitted = True
            dest = out.contents
            source = self._root
            dest.length = source.length
            dest.null_count = source.null_count
            dest.offset = source.offset
            dest.n_buffers = source.n_buffers
            dest.n_children = source.n_children
            dest.buffers = source.buffers
            dest.children = source.children
            dest.dictionary = source.dictionary
            dest.release = self._array_release
            dest.private_data = None
            return 0
        ctypes.memset(out, 0, ctypes.sizeof(ArrowArray))
        return 0

    def get_last_error(self, stream):
        return None

    def stream_release(self, stream):
        stream.contents.release = PFN_STREAM_RELEASE()

    def __arrow_c_stream__(self, requested_schema=None):
        self._stream = ArrowArrayStream(
            get_schema=PFN_GET_SCHEMA(self.get_schema),
            get_next=PFN_GET_NEXT(self.get_next),
            get_last_error=PFN_GET_LAST_ERROR(self.get_last_error),
            release=PFN_STREAM_RELEASE(self.stream_release),
            private_data=None,
        )
        pointer = ctypes.cast(ctypes.pointer(self._stream), ctypes.c_void_p)
        return _PyCapsule_New(pointer, b"arrow_array_stream", None)
