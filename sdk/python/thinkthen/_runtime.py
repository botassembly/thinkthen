"""Resource ownership at the Python boundary."""

from __future__ import annotations

import json
from collections.abc import Callable, Iterable, Iterator
from dataclasses import dataclass
from types import TracebackType
from typing import Generic, Literal, TypeVar, cast

from . import _native
from ._types import (
    A_co,
    BackendError,
    JSONValue,
    LocalError,
    Record,
    Row,
    RowError,
    T_co,
    ThinkThenError,
    Threshold,
    UsageError,
)

T = TypeVar("T")
Raw = tuple[str, str | None, bool, object, int]


def binding(call: Callable[[], T]) -> T:
    try:
        return call()
    except _native.Error as error:
        family, code, message = cast(tuple[str, str, str], error.args)
        raise failure(family, RowError(code, message)) from None


def failure(family: str, error: RowError) -> ThinkThenError:
    kind = {"usage": UsageError, "backend": BackendError, "local": LocalError}.get(
        family, ThinkThenError
    )
    return kind(error.code, error.message)


def encode(value: object) -> str:
    return json.dumps(value, ensure_ascii=False, allow_nan=False)


def source(records: Iterable[Record]) -> Iterator[tuple[bool, str, object]]:
    for record in records:
        yield (
            isinstance(record, str),
            record if isinstance(record, str) else encode(record),
            record,
        )


def row_from_json(text: str, detail: str | None = None) -> Row[JSONValue]:
    value = cast(dict[str, JSONValue], json.loads(text))
    error = cast(dict[str, str] | None, value.get("error"))
    row = Row[JSONValue](
        id=value["id"],
        status=cast(Literal["answered", "unsure", "failed"], value["status"]),
        answer=value.get("answer"),
        p=cast(float | None, value.get("p")),
        error=RowError(error["code"], error["message"]) if error is not None else None,
    )
    object.__setattr__(row, "_detail_json", detail)
    return row


class Cancel:
    def __init__(self) -> None:
        self._native = _native.NativeCancel()

    def cancel(self) -> None:
        self._native.cancel()


class _Skip:
    pass


SKIP = _Skip()


class Stream(Iterator[T_co], Generic[T_co]):
    """An iterator whose context owns one engine operation."""

    def __init__(
        self,
        native: _native.NativeStream,
        convert: Callable[[Raw], T_co | _Skip],
    ) -> None:
        self._native = native
        self._convert = convert
        self._closed = False

    def __iter__(self) -> Stream[T_co]:
        return self

    def __next__(self) -> T_co:
        if self._closed:
            raise StopIteration
        try:
            while True:
                item = binding(self._native.receive)
                if item is None:
                    self.close()
                    raise StopIteration
                value = self._convert(item)
                if not isinstance(value, _Skip):
                    return value
        except BaseException:
            self.close()
            raise

    def close(self) -> None:
        if not self._closed:
            self._closed = True
            self._native.close()

    def __enter__(self) -> Stream[T_co]:
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc_value: BaseException | None,
        traceback: TracebackType | None,
    ) -> None:
        self.close()

    def __del__(self) -> None:
        self.close()


class Toolbox:
    def __init__(self, definition: dict[str, JSONValue]) -> None:
        self._native = binding(lambda: _native.NativeToolbox(encode(definition)))

    def question(self, name: str | None = None) -> Question[JSONValue]:
        return Question(binding(lambda: self._native.question(name)))


@dataclass(frozen=True)
class Question(Generic[A_co]):
    _native: _native.NativeQuestion

    def __call__(
        self,
        text: Record,
        *,
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Row[A_co]:
        from ._client import default_client

        return cast(
            Row[A_co],
            default_client()._single(
                self._native.function,
                None,
                text,
                threshold=threshold,
                on=on,
                cancel=cancel,
                loaded=self,
            ),
        )

    def stream(
        self,
        records: Iterable[Record],
        *,
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Stream[Row[A_co]]:
        from ._client import default_client

        return cast(
            Stream[Row[A_co]],
            default_client()._rows(
                self._native.function,
                None,
                records,
                threshold=threshold,
                on=on,
                cancel=cancel,
                loaded=self,
            ),
        )
