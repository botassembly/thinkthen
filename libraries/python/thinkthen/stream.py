"""The single-reader Python face of the native bounded stream."""

import os
import threading

from . import UsageError


class Stream:
    __slots__ = ("_start", "_native", "_lock", "_pid", "_probability", "_closed")

    def __init__(self, start, *, probability=False):
        self._start = start
        self._native = None
        self._lock = threading.Lock()
        self._pid = os.getpid()
        self._probability = probability
        self._closed = False

    def __iter__(self):
        return self

    def __next__(self):
        if os.getpid() != self._pid:
            raise UsageError("an open stream cannot be read after fork")
        if not self._lock.acquire(blocking=False):
            raise UsageError("a stream has one reader")
        try:
            if self._closed:
                raise StopIteration
            if self._native is None:
                self._native = self._start()
                self._start = None
            try:
                result = next(self._native)
            except StopIteration:
                self._closed = True
                raise
            value, probability = result if isinstance(result, tuple) else (result, None)
            return (value, probability) if self._probability else value
        finally:
            self._lock.release()

    def close(self):
        if os.getpid() != self._pid:
            raise UsageError("an open stream cannot be closed after fork")
        if not self._lock.acquire(blocking=False):
            raise UsageError("a stream has one reader")
        try:
            if self._native is not None and not self._closed:
                self._native.close()
            self._closed = True
            self._start = None
        finally:
            self._lock.release()

    def __enter__(self):
        return self

    def __exit__(self, *ignored):
        self.close()

    @property
    def facts(self):
        return None if self._native is None else self._native.facts

    @property
    def value(self):
        raise AttributeError("thinkthen: a stream has no value; iterate it, or pass a list")

    def __reduce__(self):
        raise TypeError("a ThinkThen Stream cannot be pickled")

    def __del__(self):
        if getattr(self, "_pid", None) == os.getpid() and getattr(self, "_native", None) is not None:
            self._native = None  # the native destructor cancels without touching Python.
