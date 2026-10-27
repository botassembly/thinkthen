"""Native loading, conversion and cancellation at the Python boundary."""

from __future__ import annotations

import json
import math
from collections.abc import Callable, Iterable, Iterator
from datetime import date
from threading import Lock
from typing import NoReturn, TypeVar, cast
from weakref import WeakMethod

from . import _native
from ._contract import (
    ABI_CODE,
    ABI_EXPORTS,
    ABI_MESSAGE,
    ABI_VERSION,
    ABI_VERSION_EXPORT,
)
from ._types import (
    Cancelled,
    ErrorCode,
    InternalError,
    Record,
    SetupError,
    ThinkThenError,
    UsageError,
)

T = TypeVar("T")
Raw = tuple[str, str | None, bool, object, int]


def _export(path: str) -> object:
    value: object = _native
    for name in path.split("."):
        value = getattr(value, name, None)
        if value is None:
            raise SetupError(ErrorCode(ABI_CODE), ABI_MESSAGE)
    return value


_version = _export(ABI_VERSION_EXPORT)
if (
    type(_version) is not int
    or _version >> 16 != ABI_VERSION >> 16
    or (_version & 0xFFFF) < (ABI_VERSION & 0xFFFF)
):
    raise SetupError(ErrorCode(ABI_CODE), ABI_MESSAGE)
for _name in ABI_EXPORTS:
    _export(_name)


def binding(call: Callable[[], T]) -> T:
    try:
        return call()
    except KeyboardInterrupt:
        return binding(_native.reject_cancelled)
    except _native.Error as error:
        family, code, message = cast(tuple[str, str, str], error.args)
        try:
            typed = ErrorCode(code)
        except ValueError:
            raise SetupError(ErrorCode(ABI_CODE), ABI_MESSAGE) from None
        kind: type[ThinkThenError] = UsageError if family == "usage" else SetupError
        if typed is ErrorCode.USAGE_ABI:
            kind = SetupError
        if typed is ErrorCode.LOCAL_CANCELLED:
            kind = Cancelled
        elif family == "internal":
            kind = InternalError
        raise kind(typed, message) from None


def _converted(convert: Callable[[], str]) -> str | Exception:
    try:
        value = convert()
        value.encode("utf-8")
        return value
    except Exception as error:  # noqa: BLE001 - arbitrary caller conversion can raise.
        return error


def _json(
    value: object, convert: Callable[[object], object] | None = None
) -> str | Exception:
    return _converted(
        lambda: json.dumps(
            value if convert is None else convert(value),
            ensure_ascii=False,
            allow_nan=False,
        )
    )


def encode(value: object, reject: Callable[[], NoReturn]) -> str:
    text = _json(value)
    if isinstance(text, Exception):
        binding(reject)
    return text


def normalize(value: object, missing: Callable[[object], bool]) -> object:
    """Translate frame values into JSON values before shared serialization."""
    if missing(value) or isinstance(value, float) and math.isnan(value):
        return None
    if isinstance(value, date):
        return value.isoformat()
    if isinstance(value, dict):
        return {key: normalize(item, missing) for key, item in value.items()}
    if isinstance(value, (list, tuple)):
        return [normalize(item, missing) for item in value]
    return value


def source(
    records: Iterable[Record],
    failures: dict[int, str],
    convert: Callable[[object], object] | None = None,
) -> Iterator[tuple[bool, str | None, object]]:
    for position, record in enumerate(records, 1):
        text = (
            _converted(lambda: record)  # noqa: B023 - conversion runs immediately.
            if isinstance(record, str)
            else _json(record, convert)
        )
        if isinstance(text, Exception):
            failures[position] = f"{type(text).__name__}: {text}"
            yield False, None, record
        else:
            yield isinstance(record, str), text, record


class Cancel:
    __slots__ = ("__weakref__", "_callbacks", "_cancelled", "_lock", "_native")

    def __init__(self) -> None:
        self._native = _native.NativeCancel()
        self._callbacks: list[WeakMethod[Callable[[], None]]] = []
        self._lock = Lock()
        self._cancelled = False

    def _child(self) -> Cancel:
        child = Cancel()
        child._native = self._native.child()
        return child

    def _watch(self, callback: Callable[[], None]) -> None:
        with self._lock:
            cancelled = self._cancelled
            if not cancelled:
                self._callbacks = [ref for ref in self._callbacks if ref() is not None]
                self._callbacks.append(WeakMethod(callback))
        if cancelled:
            callback()

    def cancel(self) -> None:
        self._native.cancel()
        with self._lock:
            self._cancelled = True
            callbacks, self._callbacks = self._callbacks, []
        for ref in callbacks:
            callback = ref()
            if callback is not None:
                callback()


__all__ = ["Cancel"]
