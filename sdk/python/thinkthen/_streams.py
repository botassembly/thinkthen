"""Owned sync and async iteration over one native operation."""

from __future__ import annotations

import asyncio
import logging
from collections.abc import AsyncIterator, Callable, Iterator
from dataclasses import replace
from types import TracebackType
from typing import TYPE_CHECKING, Any, Generic, TypeVar, cast

if TYPE_CHECKING:
    import pandas as pd
    import polars as pl

from . import _native
from ._decode import row_from_json
from ._runtime import binding
from ._types import FilterRow

R_co = TypeVar("R_co", covariant=True)
T = TypeVar("T")
_END = object()


class RowStream(Iterator[R_co], Generic[R_co]):
    def __init__(
        self,
        native: _native.NativeStream,
        failures: dict[int, str],
        remaining: int = 0,
        *,
        function: str,
        question: object,
    ) -> None:
        self._native = native
        self._failures = failures
        self._function = function
        self._question = question
        self._remaining = remaining
        self._closed = False

    def __iter__(self) -> RowStream[R_co]:
        return self

    def __next__(self) -> R_co:
        if self._closed:
            raise StopIteration
        try:
            raw = binding(self._native.receive)
            if raw is None:
                self.close()
                raise StopIteration
            self._remaining = max(0, self._remaining - 1)
            row = row_from_json(raw[0], raw[1], raw[3])
            cause = self._failures.pop(row.position, None)
            if cause is not None and row.error is not None:
                row = replace(
                    row,
                    error=replace(row.error, message=f"{row.error.message}: {cause}"),
                )
            if row.error is not None:
                logging.getLogger("thinkthen").debug(
                    "%s %s", row.function.value, row.error.code.value
                )
            return cast(R_co, row)
        except BaseException:
            self.close()
            raise

    def __length_hint__(self) -> int:
        return 0 if self._closed else self._remaining

    def cancel(self) -> None:
        self._native.cancel()

    def close(self) -> None:
        if not self._closed:
            self._closed = True
            self._native.close()
            self._failures.clear()

    def __enter__(self) -> RowStream[R_co]:
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc_value: BaseException | None,
        traceback: TracebackType | None,
    ) -> None:
        self.close()

    def __del__(self) -> None:
        if hasattr(self, "_native"):
            self._native.cancel()

    def kept_items(self: RowStream[FilterRow[T]]) -> Iterator[T]:
        try:
            for row in self:
                if row.kept:
                    yield row.item
        finally:
            self.close()

    def to_list(self) -> list[R_co]:
        try:
            return list(self)
        finally:
            self.close()

    def to_pandas(self) -> pd.DataFrame:
        try:
            from .pandas import frame

            rows = self.to_list()
            return frame(rows, function=self._function)
        finally:
            self.close()

    def to_polars(self) -> pl.DataFrame:
        try:
            from .polars import series

            return series(self.to_list(), self._function, self._question).to_frame()
        finally:
            self.close()


def _next(stream: RowStream[R_co]) -> R_co | object:
    return next(stream, _END)


class AsyncRowStream(Generic[R_co]):
    def __init__(
        self,
        create: Callable[[], RowStream[R_co]],
        cancel_pending: Callable[[], None],
        *,
        function: str,
        question: object,
    ) -> None:
        self._create = create
        self._function = function
        self._question = question
        self._cancel_pending = cancel_pending
        self._stream: RowStream[R_co] | None = None
        self._closed = False
        self._pending: asyncio.Task[Any] | None = None

    def __aiter__(self) -> AsyncRowStream[R_co]:
        return self

    async def __anext__(self) -> R_co:
        if self._closed:
            raise StopAsyncIteration
        try:
            if self._stream is None:
                self._pending = asyncio.create_task(asyncio.to_thread(self._create))
                self._stream = await asyncio.shield(self._pending)
            stream = self._stream
            assert stream is not None
            self._pending = asyncio.create_task(asyncio.to_thread(_next, stream))
            value = await asyncio.shield(self._pending)
            if value is _END:
                await self.aclose()
                raise StopAsyncIteration
            return cast(R_co, value)
        except BaseException:
            await self.aclose()
            raise

    def cancel(self) -> None:
        self._cancel_pending()
        if self._stream is not None:
            self._stream.cancel()

    async def aclose(self) -> None:
        if self._closed:
            return
        self._closed = True
        self.cancel()
        if self._pending is not None:
            try:
                result = await asyncio.shield(self._pending)
                if self._stream is None and isinstance(result, RowStream):
                    self._stream = result
            except BaseException:  # noqa: BLE001, S110 - preserve caller failure during cleanup.
                pass
        if self._stream is not None:
            await asyncio.to_thread(self._stream.close)

    async def __aenter__(self) -> AsyncRowStream[R_co]:
        return self

    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_value: BaseException | None,
        traceback: TracebackType | None,
    ) -> None:
        await self.aclose()

    def __del__(self) -> None:
        if hasattr(self, "_cancel_pending"):
            self.cancel()

    async def kept_items(self: AsyncRowStream[FilterRow[T]]) -> AsyncIterator[T]:
        try:
            async for row in self:
                if row.kept:
                    yield row.item
        finally:
            await self.aclose()

    async def to_list(self) -> list[R_co]:
        try:
            return [row async for row in self]
        finally:
            await self.aclose()

    async def to_pandas(self) -> pd.DataFrame:
        try:
            from .pandas import frame

            return frame(await self.to_list(), function=self._function)
        finally:
            await self.aclose()

    async def to_polars(self) -> pl.DataFrame:
        try:
            from .polars import series

            return series(
                await self.to_list(), self._function, self._question
            ).to_frame()
        finally:
            await self.aclose()


__all__ = ["AsyncRowStream", "RowStream"]
